use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, FieldDiff, FinancialClassification,
    FourColorArchetype,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::spot_conditional_order::SpotOrderTriggerRole;
use super::spot_order_primitives::{
    SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTimeInForce, option_status_reason_value, option_u64_value, push_change,
    stable_order_entity_id,
};
use super::spot_trade::SpotTrade;
use crate::entity::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error, BalanceLedgerReason, Reservation,
    ReservationCloseReason, ReservationError, ReservationKind, ReservationMarketKind,
    ReservationStatus,
};

#[cfg(test)]
mod spot_order_v2_bdd_behavior_methods;
#[cfg(test)]
mod spot_order_v2_bdd_factory_scenario;
#[cfg(test)]
mod spot_order_v2_bdd_happy_path;
#[cfg(test)]
mod spot_order_v2_bdd_lifecycle;

const SPOT_ORDER_V2_ENTITY_TYPE: u8 = 3;

/// `SpotOrderV2` 的冻结资产角色。
///
/// 它只表达订单视角的 `Base` / `Quote` 业务角色，不绑定余额 ID、asset_id
/// 或 reservation_id。具体映射由 use case 在聚合外完成。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotOrderHoldAsset {
    Base,
    Quote,
}

impl SpotOrderHoldAsset {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Quote => "quote",
        }
    }
}

/// 订单建立时声明的冻结需求。
///
/// 这是订单聚合内部业务事实，不代表 authoritative 冻结记录本身。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderHoldRequirement {
    /// 订单需要冻结的资产角色。
    pub asset: SpotOrderHoldAsset,
    /// 订单视角下的冻结数量。
    pub amount: u64,
}

/// 订单手续费预冻结时使用的成交角色。
///
/// 下单阶段不知道最终会以 maker 还是 taker 成交，因此订单侧会先按最坏角色
/// 费率声明 fee hold requirement；实际成交后再按真实角色 consume。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotTradeFeeRole {
    Maker,
    Taker,
}

impl SpotTradeFeeRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Maker => "maker",
            Self::Taker => "taker",
        }
    }
}

/// 订单建立时声明的手续费冻结需求。
///
/// 当前 spot v2 统一按 quote 计价手续费上界冻结，冻结数量按
/// `ceil(order_quote_notional * max_fee_bps / 10_000)` 计算。
/// 它只表达订单侧 requirement，不直接持有 authoritative reservation remaining。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderFeeHoldRequirement {
    /// fee 预冻结资产角色。spot 现阶段统一为 quote。
    pub asset: SpotOrderHoldAsset,
    /// 订单视角下的 fee 预冻结数量上界。
    pub amount: u64,
    /// 触发该 hold requirement 的最坏角色。
    pub worst_case_role: SpotTradeFeeRole,
    /// 对应的最坏费率，单位为 bps，分母固定 10_000。
    pub fee_bps: u64,
}

/// 单笔成交在订单侧应消耗的手续费需求。
///
/// 该结构只表达“如果这笔 trade 由当前订单承担 fee，应 consume 多少 fee reservation”。
/// authoritative consume/release 仍由 reservation / settlement use case 驱动。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderFeeConsumeRequirement {
    /// fee consume 资产角色。spot 现阶段统一为 quote。
    pub asset: SpotOrderHoldAsset,
    /// 本次 trade 对应的实际 fee consume 数量。
    pub amount: u64,
    /// 当前成交的真实角色。
    pub role: SpotTradeFeeRole,
    /// 本次 consume 采用的真实费率，单位为 bps。
    pub fee_bps: u64,
}

/// 订单侧 principal / fee 释放需求组合。
///
/// use case 只读取该组合语义，再映射到 authoritative reservation / balance 变化。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderReleaseRequirements {
    /// 订单 principal 冻结释放需求。
    pub principal: Option<SpotOrderReleaseRequirement>,
    /// 订单 fee 冻结释放需求。
    pub fee: Option<SpotOrderReleaseRequirement>,
}

/// 订单终态允许释放冻结的业务原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotOrderReleaseReason {
    /// 用户主动撤单导致释放剩余冻结。
    Canceled,
    /// IOC 在本轮结束后未全部成交，剩余冻结应释放。
    IocUnfilled,
    /// 下单阶段被拒绝，没有进入持续撮合生命周期。
    Rejected,
    /// 订单已全部成交，残余冻结应做清理式释放。
    FilledCleanup,
}

impl SpotOrderReleaseReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::IocUnfilled => "ioc_unfilled",
            Self::Rejected => "rejected",
            Self::FilledCleanup => "filled_cleanup",
        }
    }
}

/// 订单在终态下允许释放的冻结需求。
///
/// `amount` 只表达订单侧允许释放的上界事实；外部 use case 仍需和 authoritative
/// hold / balance 状态做 `min(...)` 或等价校验后再执行实际释放。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderReleaseRequirement {
    /// 允许释放的资产角色。
    pub asset: SpotOrderHoldAsset,
    /// 订单侧允许释放的数量上界。
    pub amount: u64,
    /// 允许释放的业务原因。
    pub reason: SpotOrderReleaseReason,
}

/// `SpotOrderV2` 单聚合撮合/生命周期错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SpotOrderV2MatchError {
    /// 订单状态和已成交数量不一致。
    #[error("order execution state is inconsistent")]
    InconsistentExecutionState,
    /// 订单当前生命周期状态不允许继续撮合。
    #[error("order is not matchable")]
    OrderNotMatchable,
    /// 订单当前生命周期状态不允许用户撤单。
    #[error("order is not cancelable")]
    OrderNotCancelable,
    /// maker 和 taker 不能是同一张订单。
    #[error("maker order must not be the taker order")]
    MakerIsTaker,
    /// maker 和 taker 必须方向相反。
    #[error("maker order has the same side as taker")]
    SameSideMaker,
    /// maker 必须是限价单，成交价取 maker 限价。
    #[error("maker order must be a limit order")]
    MakerMustBeLimit,
    /// maker 和 taker 必须交易同一现货 asset。
    #[error("maker order trades a different asset")]
    AssetMismatch,
    /// maker 和 taker 必须交易同一展示交易对。
    #[error("maker order trades a different symbol")]
    SymbolMismatch,
    /// 按当前 maker 顺序没有任何可成交结果。
    #[error("no spot trades were matched")]
    NoTradesMatched,
    /// 生成撮合结果时发生整数溢出。
    #[error("arithmetic overflow while deriving match result")]
    ArithmeticOverflow,
    /// 订单没有稳定 quote 名义价值，无法计算 quote 计价手续费。
    #[error("spot order quote notional is unavailable")]
    QuoteNotionalUnavailable,
}

/// 下单行为输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceSpotOrderV2Input {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// Hyperliquid 现货资产编号。
    pub asset: u32,
    /// 拥有该订单的交易账户 ID。
    pub account_id: String,
    /// 交易对展示名。
    pub symbol: String,
    /// 买卖方向。
    pub side: SpotOrderSide,
    /// 执行意图。
    pub execution: SpotOrderExecution,
    /// 订单有效方式。
    pub time_in_force: SpotOrderTimeInForce,
    /// base 计价下单数量。
    pub qty: u64,
    /// base 资产 ID。
    pub base_asset_id: String,
    /// quote 资产 ID。
    pub quote_asset_id: String,
    /// base 余额实体 ID。
    pub base_balance_entity_id: String,
    /// quote 余额实体 ID。
    pub quote_balance_entity_id: String,
    /// maker 手续费 bps，用于订单内 fee reservation 最坏情况预冻结。
    pub maker_fee_bps: u64,
    /// taker 手续费 bps，用于订单内 fee reservation 最坏情况预冻结。
    pub taker_fee_bps: u64,
    /// 客户端自定义订单 ID。
    pub client_order_id: Option<String>,
}

/// 下单行为结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceSpotOrderV2Outcome {
    /// 新创建的订单聚合。
    pub order: SpotOrderV2,
    /// 本次下单直接派生的冻结流水。
    pub freeze_ledger_entry: BalanceLedgerEntryV2,
}

/// 撮合行为输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchSpotOrderV2Input {
    /// 一次撮合批次 ID。
    pub match_id: String,
    /// maker 手续费 bps。
    pub maker_fee_bps: u64,
    /// taker 手续费 bps。
    pub taker_fee_bps: u64,
}

/// 撮合行为结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchSpotOrderV2Outcome {
    /// 本次撮合直接派生的成交事实。
    pub trades: Vec<SpotTrade>,
}

/// 撤单行为输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderV2Input {
    /// 被释放冻结余额对应的余额实体 ID。
    pub balance_entity_id: String,
}

/// 撤单行为结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderV2Outcome {
    /// 本次撤单直接派生的解冻流水；未触发条件单撤销时没有冻结可释放。
    pub unfreeze_ledger_entry: Option<BalanceLedgerEntryV2>,
}

/// `SpotOrderV2` 三个聚合行为入口的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotOrderV2BehaviorError {
    /// 下单数量必须大于零。
    #[error("spot order quantity must be greater than zero")]
    InvalidQuantity,
    /// 下单价格必须大于零。
    #[error("spot order price must be greater than zero")]
    InvalidPrice,
    /// 订单当前生命周期状态不允许继续撮合。
    #[error("order is not matchable")]
    OrderNotMatchable,
    /// 订单当前生命周期状态不允许用户撤单。
    #[error("order is not cancelable")]
    OrderNotCancelable,
    /// maker 和 taker 不能是同一张订单。
    #[error("maker order must not be the taker order")]
    MakerIsTaker,
    /// maker 和 taker 必须方向相反。
    #[error("maker order has the same side as taker")]
    SameSideMaker,
    /// maker 必须是限价单。
    #[error("maker order must be a limit order")]
    MakerMustBeLimit,
    /// maker 和 taker 必须交易同一现货 asset。
    #[error("maker order trades a different asset")]
    AssetMismatch,
    /// maker 和 taker 必须交易同一展示交易对。
    #[error("maker order trades a different symbol")]
    SymbolMismatch,
    /// 订单行为推导发生整数溢出。
    #[error("arithmetic overflow while deriving spot order behavior")]
    ArithmeticOverflow,
    /// principal reservation 行为失败。
    #[error("reservation behavior failed: {0}")]
    Reservation(#[from] ReservationError),
    /// 余额流水派生失败。
    #[error("balance ledger entry derivation failed: {0}")]
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
}

impl From<SpotOrderV2MatchError> for SpotOrderV2BehaviorError {
    fn from(error: SpotOrderV2MatchError) -> Self {
        match error {
            SpotOrderV2MatchError::InconsistentExecutionState
            | SpotOrderV2MatchError::OrderNotMatchable
            | SpotOrderV2MatchError::NoTradesMatched
            | SpotOrderV2MatchError::QuoteNotionalUnavailable => Self::OrderNotMatchable,
            SpotOrderV2MatchError::OrderNotCancelable => Self::OrderNotCancelable,
            SpotOrderV2MatchError::MakerIsTaker => Self::MakerIsTaker,
            SpotOrderV2MatchError::SameSideMaker => Self::SameSideMaker,
            SpotOrderV2MatchError::MakerMustBeLimit => Self::MakerMustBeLimit,
            SpotOrderV2MatchError::AssetMismatch => Self::AssetMismatch,
            SpotOrderV2MatchError::SymbolMismatch => Self::SymbolMismatch,
            SpotOrderV2MatchError::ArithmeticOverflow => Self::ArithmeticOverflow,
        }
    }
}

const FEE_BPS_DENOMINATOR: u64 = 10_000;

fn quote_notional(qty: u64, price: u64) -> Option<u64> {
    qty.checked_mul(price)
}

fn fee_amount_round_up(notional: u64, fee_bps: u64) -> Option<u64> {
    let scaled = notional.checked_mul(fee_bps)?;
    let numerator = scaled.checked_add(FEE_BPS_DENOMINATOR.checked_sub(1)?)?;
    Some(numerator / FEE_BPS_DENOMINATOR)
}

#[derive(Debug, Clone)]
struct PlacePrincipalHoldPlan {
    reserved_base: u64,
    reserved_quote: u64,
    freeze_asset_id: String,
    freeze_balance_entity_id: String,
    amount: u64,
}

/// `SpotOrderV2` 的稳定身份事实。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderIdentity {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// Hyperliquid 现货资产编号，现货通常为 `10000 + spot index`。
    pub asset: u32,
    /// Hyperliquid 返回的 numeric `oid`；订单尚未被交易所确认时可以为空。
    pub exchange_oid: Option<u64>,
    /// 拥有该订单的交易账户 ID。
    pub account_id: String,
    /// 交易对展示名，例如 `BTCUSDT`。业务身份以 `asset` 为准。
    pub symbol: String,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
}

/// `SpotOrderV2` 跨生命周期共享的业务事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderCommonFacts {
    /// 买卖方向。
    pub side: SpotOrderSide,
    /// 以 base asset 计价的下单数量。
    pub qty: u64,
}

/// 未触发条件单状态，只保存触发规则和触发后的执行意图。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotTriggerPendingState {
    /// 触发价格，使用 core fixed-point 整数价格。
    pub trigger_price: u64,
    /// 触发单业务角色。
    pub trigger_role: SpotOrderTriggerRole,
    /// 触发后的执行意图。
    pub trigger_execution: SpotOrderExecution,
    /// 触发后 active 订单使用的 TIF。
    pub triggered_time_in_force: SpotOrderTimeInForce,
}

/// 已进入撮合生命周期的订单状态。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotActiveOrderState {
    /// 执行意图。
    pub execution: SpotOrderExecution,
    /// 订单有效方式。
    pub time_in_force: SpotOrderTimeInForce,
    /// 已成交数量。
    pub filled_qty: u64,
    /// 订单建立时记录的 base 冻结快照，主要用于卖单。
    pub reserved_base: u64,
    /// 订单建立时记录的 quote 冻结快照，主要用于买单。
    pub reserved_quote: u64,
    /// 订单 principal 冻结凭证。
    pub reservation: Reservation,
    /// 订单 fee 冻结凭证。
    pub fee_reservation: Reservation,
}

/// 订单终态状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotTerminalOrderState {
    /// 本地生命周期终态。
    pub status: SpotOrderStatus,
    /// Hyperliquid 细分状态原因。
    pub status_reason: Option<SpotOrderStatusReason>,
    /// 终态时已经成交的数量。
    pub filled_qty: u64,
}

/// `SpotOrderV2` 的统一生命周期。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotOrderLifecycle {
    /// 条件单已接受，等待触发；不冻结、不撮合。
    TriggerPending(SpotTriggerPendingState),
    /// 普通订单已进入执行流程，尚未成交。
    Active(SpotActiveOrderState),
    /// 普通订单已部分成交，剩余数量仍在业务上可撤。
    PartiallyFilled(SpotActiveOrderState),
    /// 订单已完全成交。
    Filled(SpotTerminalOrderState),
    /// 订单已取消。
    Canceled(SpotTerminalOrderState),
    /// 订单提交或触发时被拒绝。
    Rejected(SpotTerminalOrderState),
    /// 条件单或订单已过期。
    Expired(SpotTerminalOrderState),
}

/// 条件单触发输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerSpotOrderV2Input {
    /// base 资产 ID。
    pub base_asset_id: String,
    /// quote 资产 ID。
    pub quote_asset_id: String,
    /// maker 手续费 bps，用于订单内 fee reservation 最坏情况预冻结。
    pub maker_fee_bps: u64,
    /// taker 手续费 bps，用于订单内 fee reservation 最坏情况预冻结。
    pub taker_fee_bps: u64,
}

/// `SpotOrderV2 v2` 的目标态订单聚合。
///
/// 这是一个 `MomentInterval + AggregateRoot`，只表达订单自身的业务真相：
/// 订单身份、生命周期推进、撮合语义，以及订单侧的 hold / release requirement。
///
/// 它不再承担完整 `Reservation` 子聚合职责，也不直接表达余额冻结记录的
/// before / after。authoritative 冻结状态留在余额或独立 hold 聚合，由 use case
/// 在 `Command + GivenState -> Changes` 编排层驱动。
///
/// 与 `SpotTrade` 的边界也在这里固定：
/// - `SpotOrderV2` 负责订单是否可撮合、如何成交、以及订单状态如何推进
/// - `SpotTrade` 负责成交已经成立后的事实与清结算角色映射
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrderV2 {
    /// 订单身份事实。
    #[serde(skip, default = "SpotOrderV2::serde_default_identity")]
    pub identity: SpotOrderIdentity,
    /// 跨条件单/普通单共享的业务事实。
    #[serde(skip, default = "SpotOrderV2::serde_default_common")]
    pub common: SpotOrderCommonFacts,
    /// 订单生命周期权威状态。
    #[serde(skip, default = "SpotOrderV2::serde_default_lifecycle")]
    pub lifecycle: SpotOrderLifecycle,
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// Hyperliquid 现货资产编号，现货通常为 `10000 + spot index`。
    pub asset: u32,
    /// Hyperliquid 返回的 numeric `oid`；订单尚未被交易所确认时可以为空。
    pub exchange_oid: Option<u64>,
    /// 拥有该订单的交易账户 ID。
    pub account_id: String,
    /// 交易对展示名，例如 `BTCUSDT`。业务身份以 `asset` 为准。
    pub symbol: String,
    /// 买卖方向。
    pub side: SpotOrderSide,
    /// 进入执行流程后的执行意图。
    pub execution: SpotOrderExecution,
    /// Hyperliquid `t.limit.tif`。市价意图通常映射为 `Ioc`。
    pub time_in_force: SpotOrderTimeInForce,
    /// 以 base asset 计价的下单数量。
    pub qty: u64,
    /// 已成交数量。
    pub filled_qty: u64,
    /// 本地生命周期状态。
    pub status: SpotOrderStatus,
    /// Hyperliquid 细分状态原因。
    pub status_reason: Option<SpotOrderStatusReason>,
    /// 订单建立时记录的 base 冻结快照，主要用于卖单。
    ///
    /// 该字段只保留订单内初始冻结事实或回放兼容语义，不代表余额侧 authoritative
    /// remaining hold。
    pub reserved_base: u64,
    /// 订单建立时记录的 quote 冻结快照，主要用于买单。
    ///
    /// 该字段只保留订单内初始冻结事实或回放兼容语义，不代表余额侧 authoritative
    /// remaining hold。
    pub reserved_quote: u64,
    /// 订单 principal 冻结凭证。
    ///
    /// `reserved_base` / `reserved_quote` 仍作为历史快照字段保留，但订单生命周期中的
    /// authoritative principal 冻结状态从这里读取和回放。
    pub reservation: Reservation,
    /// 订单 fee 冻结凭证。
    ///
    /// v3 下单/撤单用例以订单聚合内的 fee reservation 为权威状态；余额流水仍负责
    /// 表达冻结资金的审计变化。
    pub fee_reservation: Reservation,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
    /// 当前订单实体版本，用于生成可重放更新事件。
    pub version: u64,
}

impl SpotOrderV2 {
    fn serde_default_identity() -> SpotOrderIdentity {
        SpotOrderIdentity {
            order_id: String::new(),
            asset: 0,
            exchange_oid: None,
            account_id: String::new(),
            symbol: String::new(),
            client_order_id: None,
        }
    }

    fn serde_default_common() -> SpotOrderCommonFacts {
        SpotOrderCommonFacts { side: SpotOrderSide::Buy, qty: 0 }
    }

    fn serde_default_lifecycle() -> SpotOrderLifecycle {
        let reservation = Self::empty_trigger_reservation(
            "serde-default",
            "",
            ReservationKind::SpotBuyQuote,
            "UNRESERVED",
        );
        let fee_reservation = Self::empty_trigger_reservation(
            "serde-default:fee",
            "",
            ReservationKind::SpotBuyFeeQuote,
            "UNRESERVED",
        );
        SpotOrderLifecycle::Active(SpotActiveOrderState {
            execution: SpotOrderExecution::Limit { price: 0 },
            time_in_force: SpotOrderTimeInForce::Gtc,
            filled_qty: 0,
            reserved_base: 0,
            reserved_quote: 0,
            reservation,
            fee_reservation,
        })
    }

    /// 从已校验业务事实或回放事件构造订单快照。
    ///
    /// 构造器只负责装配订单事实，不承担完整业务校验；一致性由查询方法暴露，
    /// 再由 use case 决定如何映射为命令错误。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        order_id: String,
        asset: u32,
        exchange_oid: Option<u64>,
        account_id: String,
        symbol: String,
        side: SpotOrderSide,
        execution: SpotOrderExecution,
        time_in_force: SpotOrderTimeInForce,
        qty: u64,
        filled_qty: u64,
        status: SpotOrderStatus,
        status_reason: Option<SpotOrderStatusReason>,
        reserved_base: u64,
        reserved_quote: u64,
        reservation: Reservation,
        client_order_id: Option<String>,
        version: u64,
    ) -> Self {
        let fee_reservation = Self::default_fee_reservation_from_order_parts(
            order_id.as_str(),
            account_id.as_str(),
            side,
            qty,
            match execution {
                SpotOrderExecution::Limit { price }
                | SpotOrderExecution::Market { aggressive_price: price } => price,
            },
            "USDT",
        );
        Self::new_with_fee_reservation(
            order_id,
            asset,
            exchange_oid,
            account_id,
            symbol,
            side,
            execution,
            time_in_force,
            qty,
            filled_qty,
            status,
            status_reason,
            reserved_base,
            reserved_quote,
            reservation,
            fee_reservation,
            client_order_id,
            version,
        )
    }

    /// 从已校验业务事实或回放事件构造带 fee reservation 的订单快照。
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_fee_reservation(
        order_id: String,
        asset: u32,
        exchange_oid: Option<u64>,
        account_id: String,
        symbol: String,
        side: SpotOrderSide,
        execution: SpotOrderExecution,
        time_in_force: SpotOrderTimeInForce,
        qty: u64,
        filled_qty: u64,
        status: SpotOrderStatus,
        status_reason: Option<SpotOrderStatusReason>,
        reserved_base: u64,
        reserved_quote: u64,
        reservation: Reservation,
        fee_reservation: Reservation,
        client_order_id: Option<String>,
        version: u64,
    ) -> Self {
        let identity = SpotOrderIdentity {
            order_id: order_id.clone(),
            asset,
            exchange_oid,
            account_id: account_id.clone(),
            symbol: symbol.clone(),
            client_order_id: client_order_id.clone(),
        };
        let common = SpotOrderCommonFacts { side, qty };
        let active = SpotActiveOrderState {
            execution,
            time_in_force,
            filled_qty,
            reserved_base,
            reserved_quote,
            reservation: reservation.clone(),
            fee_reservation: fee_reservation.clone(),
        };
        let terminal = SpotTerminalOrderState { status, status_reason, filled_qty };
        let lifecycle = match status {
            SpotOrderStatus::Open => SpotOrderLifecycle::Active(active),
            SpotOrderStatus::PartiallyFilled => SpotOrderLifecycle::PartiallyFilled(active),
            SpotOrderStatus::Filled => SpotOrderLifecycle::Filled(terminal),
            SpotOrderStatus::Canceled => SpotOrderLifecycle::Canceled(terminal),
            SpotOrderStatus::Rejected => SpotOrderLifecycle::Rejected(terminal),
        };
        Self {
            identity,
            common,
            lifecycle,
            order_id,
            asset,
            exchange_oid,
            account_id,
            symbol,
            side,
            execution,
            time_in_force,
            qty,
            filled_qty,
            status,
            status_reason,
            reserved_base,
            reserved_quote,
            reservation,
            fee_reservation,
            client_order_id,
            version,
        }
    }

    /// 从已校验业务事实创建普通 active order，并生成 principal / fee reservation。
    #[allow(clippy::too_many_arguments)]
    pub fn new_active(
        order_id: String,
        asset: u32,
        exchange_oid: Option<u64>,
        account_id: String,
        symbol: String,
        side: SpotOrderSide,
        execution: SpotOrderExecution,
        time_in_force: SpotOrderTimeInForce,
        qty: u64,
        base_asset_id: &str,
        quote_asset_id: &str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
        client_order_id: Option<String>,
    ) -> Result<Self, SpotOrderV2BehaviorError> {
        let order_price = execution.order_price();
        let reservation = Self::principal_reservation(
            order_id.as_str(),
            account_id.as_str(),
            side,
            qty,
            order_price,
            base_asset_id,
            quote_asset_id,
        )?;
        let fee_reservation = Self::fee_reservation(
            order_id.as_str(),
            account_id.as_str(),
            side,
            qty,
            order_price,
            quote_asset_id,
            maker_fee_bps,
            taker_fee_bps,
        )?;
        let reserved_base = if side == SpotOrderSide::Sell { qty } else { 0 };
        let reserved_quote = if side == SpotOrderSide::Buy {
            quote_notional(qty, order_price).ok_or(SpotOrderV2BehaviorError::ArithmeticOverflow)?
        } else {
            0
        };
        Ok(Self::new_with_fee_reservation(
            order_id,
            asset,
            exchange_oid,
            account_id,
            symbol,
            side,
            execution,
            time_in_force,
            qty,
            0,
            SpotOrderStatus::Open,
            None,
            reserved_base,
            reserved_quote,
            reservation,
            fee_reservation,
            client_order_id,
            1,
        ))
    }

    /// 创建未触发条件单；该状态不生成 principal / fee reservation。
    #[allow(clippy::too_many_arguments)]
    pub fn new_trigger_pending(
        order_id: String,
        asset: u32,
        exchange_oid: Option<u64>,
        account_id: String,
        symbol: String,
        side: SpotOrderSide,
        qty: u64,
        trigger_price: u64,
        trigger_role: SpotOrderTriggerRole,
        trigger_execution: SpotOrderExecution,
        triggered_time_in_force: SpotOrderTimeInForce,
        client_order_id: Option<String>,
        version: u64,
    ) -> Self {
        let identity = SpotOrderIdentity {
            order_id: order_id.clone(),
            asset,
            exchange_oid,
            account_id: account_id.clone(),
            symbol: symbol.clone(),
            client_order_id: client_order_id.clone(),
        };
        let common = SpotOrderCommonFacts { side, qty };
        let lifecycle = SpotOrderLifecycle::TriggerPending(SpotTriggerPendingState {
            trigger_price,
            trigger_role,
            trigger_execution,
            triggered_time_in_force,
        });
        let reservation = Self::empty_trigger_reservation(
            order_id.as_str(),
            account_id.as_str(),
            ReservationKind::SpotBuyQuote,
            "UNRESERVED",
        );
        let fee_reservation = Self::empty_trigger_reservation(
            format!("{order_id}:fee").as_str(),
            account_id.as_str(),
            ReservationKind::SpotBuyFeeQuote,
            "UNRESERVED",
        );
        Self {
            identity,
            common,
            lifecycle,
            order_id,
            asset,
            exchange_oid,
            account_id,
            symbol,
            side,
            execution: trigger_execution,
            time_in_force: triggered_time_in_force,
            qty,
            filled_qty: 0,
            status: SpotOrderStatus::Open,
            status_reason: None,
            reserved_base: 0,
            reserved_quote: 0,
            reservation,
            fee_reservation,
            client_order_id,
            version,
        }
    }

    /// 可 BDD 规格化的聚合根行为：创建现货订单并派生冻结流水。
    ///
    /// 该方法只创建订单聚合和下游余额流水单据，不执行余额落账。
    pub(crate) fn place(
        input: PlaceSpotOrderV2Input,
    ) -> Result<PlaceSpotOrderV2Outcome, SpotOrderV2BehaviorError> {
        if input.qty == 0 {
            return Err(SpotOrderV2BehaviorError::InvalidQuantity);
        }

        let order_price = input.execution.order_price();
        if order_price == 0 {
            return Err(SpotOrderV2BehaviorError::InvalidPrice);
        }

        let quote_notional = quote_notional(input.qty, order_price)
            .ok_or(SpotOrderV2BehaviorError::ArithmeticOverflow)?;
        let principal_hold = Self::place_principal_hold_plan(&input, quote_notional);

        let reservation = Self::principal_reservation(
            input.order_id.as_str(),
            input.account_id.as_str(),
            input.side,
            input.qty,
            order_price,
            input.base_asset_id.as_str(),
            input.quote_asset_id.as_str(),
        )?;
        let fee_reservation = Self::fee_reservation(
            input.order_id.as_str(),
            input.account_id.as_str(),
            input.side,
            input.qty,
            order_price,
            input.quote_asset_id.as_str(),
            input.maker_fee_bps,
            input.taker_fee_bps,
        )?;
        let freeze_ledger_entry = BalanceLedgerEntryV2::freeze(
            format!("balance-ledger:freeze:{}", input.order_id),
            input.account_id.clone(),
            principal_hold.freeze_asset_id,
            principal_hold.freeze_balance_entity_id,
            principal_hold.amount,
            BalanceLedgerReason::FreezeForOrder { order_id: input.order_id.clone() },
        )?;

        let order = Self::new_with_fee_reservation(
            input.order_id,
            input.asset,
            None,
            input.account_id,
            input.symbol,
            input.side,
            input.execution,
            input.time_in_force,
            input.qty,
            0,
            SpotOrderStatus::Open,
            None,
            principal_hold.reserved_base,
            principal_hold.reserved_quote,
            reservation,
            fee_reservation,
            input.client_order_id,
            1,
        );

        Ok(PlaceSpotOrderV2Outcome { order, freeze_ledger_entry })
    }

    /// 可 BDD 规格化的聚合根行为：未触发条件单进入 active 订单生命周期。
    ///
    /// 触发前不冻结、不撮合；触发时才生成 principal / fee reservation。
    pub fn trigger(
        &mut self,
        input: TriggerSpotOrderV2Input,
    ) -> Result<(), SpotOrderV2BehaviorError> {
        let pending = match &self.lifecycle {
            SpotOrderLifecycle::TriggerPending(pending) => *pending,
            _ => return Err(SpotOrderV2BehaviorError::OrderNotMatchable),
        };
        let order_price = pending.trigger_execution.order_price();
        let reservation = Self::principal_reservation(
            self.order_id.as_str(),
            self.account_id.as_str(),
            self.side,
            self.qty,
            order_price,
            input.base_asset_id.as_str(),
            input.quote_asset_id.as_str(),
        )?;
        let fee_reservation = Self::fee_reservation(
            self.order_id.as_str(),
            self.account_id.as_str(),
            self.side,
            self.qty,
            order_price,
            input.quote_asset_id.as_str(),
            input.maker_fee_bps,
            input.taker_fee_bps,
        )?;
        let reserved_base = if self.side == SpotOrderSide::Sell { self.qty } else { 0 };
        let reserved_quote = if self.side == SpotOrderSide::Buy {
            quote_notional(self.qty, order_price)
                .ok_or(SpotOrderV2BehaviorError::ArithmeticOverflow)?
        } else {
            0
        };
        let next_version = self.next_version()?;

        self.execution = pending.trigger_execution;
        self.time_in_force = pending.triggered_time_in_force;
        self.filled_qty = 0;
        self.status = SpotOrderStatus::Open;
        self.status_reason = Some(SpotOrderStatusReason::Triggered);
        self.reserved_base = reserved_base;
        self.reserved_quote = reserved_quote;
        self.reservation = reservation;
        self.fee_reservation = fee_reservation;
        self.version = next_version;
        self.lifecycle = SpotOrderLifecycle::Active(self.active_state_from_legacy());
        Ok(())
    }

    /// 为 spot 订单构造 principal reservation。
    #[allow(clippy::too_many_arguments)]
    pub fn principal_reservation(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        qty: u64,
        order_price: u64,
        base_asset_id: &str,
        quote_asset_id: &str,
    ) -> Result<Reservation, ReservationError> {
        let (kind, asset_id, amount) = match side {
            SpotOrderSide::Buy => (
                ReservationKind::SpotBuyQuote,
                quote_asset_id.to_string(),
                quote_notional(qty, order_price).ok_or(ReservationError::ArithmeticOverflow)?,
            ),
            SpotOrderSide::Sell => (ReservationKind::SpotSellBase, base_asset_id.to_string(), qty),
        };
        Reservation::new(
            format!("reservation:{order_id}"),
            account_id.to_string(),
            order_id.to_string(),
            ReservationMarketKind::Spot,
            kind,
            asset_id,
            amount,
        )
    }

    /// 为 spot 订单构造 fee reservation。
    #[allow(clippy::too_many_arguments)]
    pub fn fee_reservation(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        qty: u64,
        order_price: u64,
        quote_asset_id: &str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Result<Reservation, ReservationError> {
        let fee_bps = maker_fee_bps.max(taker_fee_bps);
        let amount = if fee_bps == 0 {
            1
        } else {
            let notional =
                quote_notional(qty, order_price).ok_or(ReservationError::ArithmeticOverflow)?;
            fee_amount_round_up(notional, fee_bps).ok_or(ReservationError::ArithmeticOverflow)?
        };
        let kind = match side {
            SpotOrderSide::Buy => ReservationKind::SpotBuyFeeQuote,
            SpotOrderSide::Sell => ReservationKind::SpotSellFeeQuote,
        };
        Reservation::new(
            format!("reservation:{order_id}:fee"),
            account_id.to_string(),
            order_id.to_string(),
            ReservationMarketKind::Spot,
            kind,
            quote_asset_id.to_string(),
            amount,
        )
    }

    fn default_fee_reservation_from_order_parts(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        qty: u64,
        order_price: u64,
        quote_asset_id: &str,
    ) -> Reservation {
        match Self::fee_reservation(
            order_id,
            account_id,
            side,
            qty,
            order_price,
            quote_asset_id,
            1,
            1,
        ) {
            Ok(reservation) => reservation,
            Err(_) => Reservation {
                reservation_id: format!("reservation:{order_id}:fee:fallback"),
                owner_account_id: account_id.to_string(),
                caused_by_order_id: order_id.to_string(),
                market_kind: ReservationMarketKind::Spot,
                reservation_kind: match side {
                    SpotOrderSide::Buy => ReservationKind::SpotBuyFeeQuote,
                    SpotOrderSide::Sell => ReservationKind::SpotSellFeeQuote,
                },
                asset_id: quote_asset_id.to_string(),
                original_amount: 1,
                consumed_amount: 0,
                released_amount: 0,
                remaining_amount: 1,
                status: ReservationStatus::Active,
                close_reason: None,
                version: 1,
            },
        }
    }

    fn empty_trigger_reservation(
        order_id: &str,
        account_id: &str,
        reservation_kind: ReservationKind,
        asset_id: &str,
    ) -> Reservation {
        Reservation {
            reservation_id: format!("reservation:{order_id}:trigger-pending"),
            owner_account_id: account_id.to_string(),
            caused_by_order_id: order_id.to_string(),
            market_kind: ReservationMarketKind::Spot,
            reservation_kind,
            asset_id: asset_id.to_string(),
            original_amount: 0,
            consumed_amount: 0,
            released_amount: 0,
            remaining_amount: 0,
            status: ReservationStatus::ClosedByRelease,
            close_reason: None,
            version: 1,
        }
    }

    fn place_principal_hold_plan(
        input: &PlaceSpotOrderV2Input,
        quote_notional: u64,
    ) -> PlacePrincipalHoldPlan {
        match input.side {
            SpotOrderSide::Buy => PlacePrincipalHoldPlan {
                reserved_base: 0,
                reserved_quote: quote_notional,
                freeze_asset_id: input.quote_asset_id.clone(),
                freeze_balance_entity_id: input.quote_balance_entity_id.clone(),
                amount: quote_notional,
            },
            SpotOrderSide::Sell => PlacePrincipalHoldPlan {
                reserved_base: input.qty,
                reserved_quote: 0,
                freeze_asset_id: input.base_asset_id.clone(),
                freeze_balance_entity_id: input.base_balance_entity_id.clone(),
                amount: input.qty,
            },
        }
    }

    /// 返回本系统生成的稳定订单 ID。
    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    /// 返回拥有该订单的交易账户 ID。
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// 返回 Hyperliquid 现货资产编号。
    pub fn asset(&self) -> u32 {
        self.asset
    }

    /// 返回交易对展示名。
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// 返回买卖方向。
    pub fn side(&self) -> SpotOrderSide {
        self.side
    }

    /// 返回以 base asset 计价的下单数量。
    pub fn qty(&self) -> u64 {
        self.qty
    }

    /// 返回本地生命周期状态。
    pub fn status(&self) -> SpotOrderStatus {
        self.status
    }

    /// 返回 Hyperliquid 细分状态原因。
    pub fn status_reason(&self) -> Option<SpotOrderStatusReason> {
        self.status_reason
    }

    /// 返回订单当前已经成交的数量。
    pub fn filled_qty(&self) -> u64 {
        self.filled_qty
    }

    /// 返回交易所确认后的 numeric `oid`。
    pub fn exchange_oid(&self) -> Option<u64> {
        self.exchange_oid
    }

    /// 返回订单是否仍是未触发条件单。
    pub fn is_trigger_pending(&self) -> bool {
        matches!(self.lifecycle, SpotOrderLifecycle::TriggerPending(_))
    }

    /// 返回 active lifecycle 的 principal reservation；未触发条件单返回 `None`。
    pub fn active_reservation(&self) -> Option<&Reservation> {
        match &self.lifecycle {
            SpotOrderLifecycle::Active(state) | SpotOrderLifecycle::PartiallyFilled(state) => {
                Some(&state.reservation)
            }
            _ => None,
        }
    }

    /// 返回 active lifecycle 的 fee reservation；未触发条件单返回 `None`。
    pub fn active_fee_reservation(&self) -> Option<&Reservation> {
        match &self.lifecycle {
            SpotOrderLifecycle::Active(state) | SpotOrderLifecycle::PartiallyFilled(state) => {
                Some(&state.fee_reservation)
            }
            _ => None,
        }
    }

    /// 返回订单是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回订单是否交易指定 Hyperliquid asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.asset == asset
    }

    /// 返回订单是否交易指定展示交易对。
    pub fn trades_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol
    }

    fn limit_price(&self) -> Option<u64> {
        self.execution.limit_price()
    }

    /// 返回需要提交给交易所的价格字段。
    pub fn order_price(&self) -> u64 {
        self.execution.order_price()
    }

    /// 返回订单 quote 名义价值。
    ///
    /// 市价意图没有稳定限价价格，或乘法溢出时返回 `None`。
    pub fn notional_quote(&self) -> Option<u64> {
        self.qty.checked_mul(self.limit_price()?)
    }

    /// 返回该订单内嵌的 principal reservation 快照。
    pub fn to_reservation(
        &self,
        _base_asset_id: &str,
        _quote_asset_id: &str,
    ) -> Result<Reservation, ReservationError> {
        Ok(self.reservation.clone())
    }

    /// 返回该订单内嵌的 fee reservation 快照。
    pub fn to_fee_reservation(&self) -> Reservation {
        self.fee_reservation.clone()
    }

    /// 返回订单当前剩余可成交数量。
    fn remaining_qty(&self) -> Option<u64> {
        self.qty.checked_sub(self.filled_qty)
    }

    /// 返回订单建立时的 quote 冻结快照。
    fn initial_quote_hold_snapshot(&self) -> Option<u64> {
        quote_notional(self.qty, self.order_price())
    }

    fn hold_asset(&self) -> SpotOrderHoldAsset {
        match self.side {
            SpotOrderSide::Buy => SpotOrderHoldAsset::Quote,
            SpotOrderSide::Sell => SpotOrderHoldAsset::Base,
        }
    }

    fn hold_snapshot_amount(&self) -> Option<u64> {
        let amount = match self.side {
            SpotOrderSide::Buy => self.initial_quote_hold_snapshot()?,
            SpotOrderSide::Sell => self.qty,
        };
        if amount == 0 { None } else { Some(amount) }
    }

    fn release_snapshot_amount(&self) -> Option<u64> {
        let amount = match self.side {
            SpotOrderSide::Buy => self.reserved_quote,
            SpotOrderSide::Sell => self.reserved_base,
        };
        if amount == 0 { None } else { Some(amount) }
    }

    fn fee_hold_asset(&self) -> SpotOrderHoldAsset {
        SpotOrderHoldAsset::Quote
    }

    fn max_fee_role_and_bps(maker_fee_bps: u64, taker_fee_bps: u64) -> (SpotTradeFeeRole, u64) {
        if maker_fee_bps > taker_fee_bps {
            (SpotTradeFeeRole::Maker, maker_fee_bps)
        } else {
            (SpotTradeFeeRole::Taker, taker_fee_bps)
        }
    }

    fn quote_fee_amount_with_bps_round_up(&self, fee_bps: u64) -> Option<u64> {
        let notional = self.initial_quote_hold_snapshot()?;
        fee_amount_round_up(notional, fee_bps)
    }

    /// 返回该订单建立时声明的冻结需求。
    ///
    /// 这只是订单侧业务事实，不是实际冻结记录，也不携带 reservation / balance / asset
    /// 聚合标识。
    pub fn hold_requirement(&self) -> Option<SpotOrderHoldRequirement> {
        Some(SpotOrderHoldRequirement {
            asset: self.hold_asset(),
            amount: self.hold_snapshot_amount()?,
        })
    }

    /// 返回用户撤单语义下的释放需求。
    ///
    /// 只有当前仍可撤单的订单会返回释放需求；数量是订单侧允许释放的上界，
    /// 外部用例仍需对照 authoritative hold 状态决定实际释放量。
    fn cancel_release_requirement(&self) -> Option<SpotOrderReleaseRequirement> {
        if !self.can_be_cancelled() {
            return None;
        }

        Some(SpotOrderReleaseRequirement {
            asset: self.hold_asset(),
            amount: self.release_snapshot_amount()?,
            reason: SpotOrderReleaseReason::Canceled,
        })
    }

    /// 返回订单在终态下允许释放的冻结需求。
    ///
    /// 该方法只表达订单侧语义，不直接读取 reservation / balance 当前 remaining hold。
    fn terminal_release_requirement(&self) -> Option<SpotOrderReleaseRequirement> {
        let reason = match self.status {
            SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled => return None,
            SpotOrderStatus::Filled => SpotOrderReleaseReason::FilledCleanup,
            SpotOrderStatus::Canceled => {
                if self.time_in_force == SpotOrderTimeInForce::Ioc {
                    SpotOrderReleaseReason::IocUnfilled
                } else {
                    SpotOrderReleaseReason::Canceled
                }
            }
            SpotOrderStatus::Rejected => SpotOrderReleaseReason::Rejected,
        };

        Some(SpotOrderReleaseRequirement {
            asset: self.hold_asset(),
            amount: self.release_snapshot_amount()?,
            reason,
        })
    }

    /// 返回下单阶段 fee 按最坏角色费率的预冻结需求。
    ///
    /// 当前 v2 语义固定为：
    /// - fee reservation 与 principal reservation 独立
    /// - fee 统一按 quote 计价
    /// - 下单阶段按 `max(maker_fee_bps, taker_fee_bps)` 的最坏角色费率冻结
    /// - 金额按向上取整计算，避免实际成交时预冻结不足
    pub fn fee_hold_requirement(
        &self,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Option<SpotOrderFeeHoldRequirement> {
        let (worst_case_role, fee_bps) = Self::max_fee_role_and_bps(maker_fee_bps, taker_fee_bps);
        Some(SpotOrderFeeHoldRequirement {
            asset: self.fee_hold_asset(),
            amount: self.quote_fee_amount_with_bps_round_up(fee_bps)?,
            worst_case_role,
            fee_bps,
        })
    }

    /// 返回撤单语义下允许释放 fee reservation remainder 的订单侧 requirement。
    fn fee_cancel_release_requirement(
        &self,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Option<SpotOrderReleaseRequirement> {
        if !self.can_be_cancelled() {
            return None;
        }

        let requirement = self.fee_hold_requirement(maker_fee_bps, taker_fee_bps)?;
        if requirement.amount == 0 {
            return None;
        }

        Some(SpotOrderReleaseRequirement {
            asset: requirement.asset,
            amount: requirement.amount,
            reason: SpotOrderReleaseReason::Canceled,
        })
    }

    /// 返回订单终态下允许释放 fee reservation remainder 的订单侧 requirement。
    fn fee_terminal_release_requirement(
        &self,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Option<SpotOrderReleaseRequirement> {
        let principal_release = self.terminal_release_requirement()?;
        let requirement = self.fee_hold_requirement(maker_fee_bps, taker_fee_bps)?;
        if requirement.amount == 0 {
            return None;
        }

        Some(SpotOrderReleaseRequirement {
            asset: requirement.asset,
            amount: requirement.amount,
            reason: principal_release.reason,
        })
    }

    /// 返回一笔真实成交在订单侧应 consume 的 fee requirement。
    ///
    /// 该方法只表达订单事实，不推进 reservation remaining。真实 consume 必须由
    /// trade settlement use case 在 authoritative reservation 上执行。
    pub fn fee_consume_requirement_for_trade(
        &self,
        trade_qty: u64,
        trade_price: u64,
        role: SpotTradeFeeRole,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Result<SpotOrderFeeConsumeRequirement, SpotOrderV2MatchError> {
        let fee_bps = match role {
            SpotTradeFeeRole::Maker => maker_fee_bps,
            SpotTradeFeeRole::Taker => taker_fee_bps,
        };

        let trade_notional = quote_notional(trade_qty, trade_price)
            .ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        let amount = fee_amount_round_up(trade_notional, fee_bps)
            .ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;

        Ok(SpotOrderFeeConsumeRequirement { asset: self.fee_hold_asset(), amount, role, fee_bps })
    }

    /// 返回订单在当前终态下是否存在订单侧释放需求。
    fn has_terminal_release(&self) -> bool {
        self.terminal_release_requirement().is_some()
    }

    /// 返回用户撤单时订单侧允许释放的 principal / fee requirement。
    pub fn cancel_release_requirements(
        &self,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> SpotOrderReleaseRequirements {
        SpotOrderReleaseRequirements {
            principal: self.cancel_release_requirement(),
            fee: self.fee_cancel_release_requirement(maker_fee_bps, taker_fee_bps),
        }
    }

    /// 返回订单终态下允许释放的 principal / fee requirement。
    pub fn terminal_release_requirements(
        &self,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> SpotOrderReleaseRequirements {
        SpotOrderReleaseRequirements {
            principal: self.terminal_release_requirement(),
            fee: self.fee_terminal_release_requirement(maker_fee_bps, taker_fee_bps),
        }
    }

    /// 返回该订单是否允许撤销。
    pub fn can_be_cancelled(&self) -> bool {
        self.status.is_cancelable()
    }

    /// 返回生命周期状态和成交数量是否一致。
    pub fn has_consistent_execution_state(&self) -> bool {
        match self.status {
            SpotOrderStatus::Open => self.filled_qty == 0,
            SpotOrderStatus::PartiallyFilled => 0 < self.filled_qty && self.filled_qty < self.qty,
            SpotOrderStatus::Filled => self.filled_qty == self.qty,
            SpotOrderStatus::Canceled => self.filled_qty <= self.qty,
            SpotOrderStatus::Rejected => self.filled_qty == 0,
        }
    }

    /// 返回 `reserved_quote` 是否仍符合买卖方向与订单快照语义。
    pub fn has_consistent_reserved_quote(&self) -> bool {
        match self.side {
            SpotOrderSide::Buy => {
                self.reservation.reservation_kind == ReservationKind::SpotBuyQuote
                    && self.reservation.original_amount == self.reserved_quote
                    && self.initial_quote_hold_snapshot() == Some(self.reserved_quote)
            }
            SpotOrderSide::Sell => self.reserved_quote == 0,
        }
    }

    /// 返回 `reserved_base` 是否仍符合买卖方向与订单快照语义。
    pub fn has_consistent_reserved_base(&self) -> bool {
        match self.side {
            SpotOrderSide::Buy => self.reserved_base == 0,
            SpotOrderSide::Sell => {
                self.reservation.reservation_kind == ReservationKind::SpotSellBase
                    && self.reservation.original_amount == self.reserved_base
                    && self.reserved_base == self.qty
            }
        }
    }

    /// 返回该订单是否应进入撮合。
    fn should_enter_matching(
        &self,
        best_maker: Option<&SpotOrderV2>,
    ) -> Result<bool, SpotOrderV2MatchError> {
        if matches!(self.time_in_force, SpotOrderTimeInForce::Ioc) {
            return Ok(true);
        }

        let Some(best_maker) = best_maker else {
            return Ok(false);
        };

        match self.crosses_order(best_maker) {
            Ok(crosses) => Ok(crosses),
            Err(_) => Ok(true),
        }
    }

    /// 校验订单当前是否仍然允许进入撮合。
    pub(crate) fn ensure_matchable(&self) -> Result<(), SpotOrderV2MatchError> {
        if !matches!(
            self.lifecycle,
            SpotOrderLifecycle::Active(_) | SpotOrderLifecycle::PartiallyFilled(_)
        ) {
            return Err(SpotOrderV2MatchError::OrderNotMatchable);
        }
        if !self.has_consistent_execution_state() {
            return Err(SpotOrderV2MatchError::InconsistentExecutionState);
        }
        if !matches!(self.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled) {
            return Err(SpotOrderV2MatchError::OrderNotMatchable);
        }
        if self.remaining_qty().ok_or(SpotOrderV2MatchError::InconsistentExecutionState)? == 0 {
            return Err(SpotOrderV2MatchError::OrderNotMatchable);
        }
        Ok(())
    }

    /// 校验该 maker 是否可以作为给定 taker 的撮合对手方。
    fn ensure_compatible_maker_for(
        &self,
        taker: &SpotOrderV2,
    ) -> Result<(), SpotOrderV2MatchError> {
        if self.order_id == taker.order_id {
            return Err(SpotOrderV2MatchError::MakerIsTaker);
        }
        if self.side == taker.side {
            return Err(SpotOrderV2MatchError::SameSideMaker);
        }
        if self.limit_price().is_none() {
            return Err(SpotOrderV2MatchError::MakerMustBeLimit);
        }
        if !self.trades_asset(taker.asset) {
            return Err(SpotOrderV2MatchError::AssetMismatch);
        }
        if !self.trades_symbol(taker.symbol.as_str()) {
            return Err(SpotOrderV2MatchError::SymbolMismatch);
        }
        Ok(())
    }

    /// 返回该订单是否会以当前价格和 maker 价格交叉成交。
    fn crosses_maker_price(&self, maker_price: u64) -> bool {
        match self.side {
            SpotOrderSide::Buy => self.order_price() >= maker_price,
            SpotOrderSide::Sell => self.order_price() <= maker_price,
        }
    }

    /// 返回该订单是否会和给定 maker 订单成交。
    fn crosses_order(&self, maker: &SpotOrderV2) -> Result<bool, SpotOrderV2MatchError> {
        if self.side == maker.side {
            return Err(SpotOrderV2MatchError::SameSideMaker);
        }

        let maker_price = maker.limit_price().ok_or(SpotOrderV2MatchError::MakerMustBeLimit)?;
        Ok(self.crosses_maker_price(maker_price))
    }

    /// 返回该订单是否会因 ALO 语义在进入撮合前被拒绝。
    fn would_be_rejected_as_alo(
        &self,
        best_maker: Option<&SpotOrderV2>,
    ) -> Result<bool, SpotOrderV2MatchError> {
        if self.time_in_force != SpotOrderTimeInForce::Alo {
            return Ok(false);
        }

        best_maker.map_or(Ok(false), |maker| self.crosses_order(maker))
    }

    /// 返回给定成交后数量对应的撮合状态。
    fn matched_status_for(&self, next_filled_qty: u64) -> SpotOrderStatus {
        if next_filled_qty == self.qty {
            SpotOrderStatus::Filled
        } else {
            SpotOrderStatus::PartiallyFilled
        }
    }

    fn active_state_from_legacy(&self) -> SpotActiveOrderState {
        SpotActiveOrderState {
            execution: self.execution,
            time_in_force: self.time_in_force,
            filled_qty: self.filled_qty,
            reserved_base: self.reserved_base,
            reserved_quote: self.reserved_quote,
            reservation: self.reservation.clone(),
            fee_reservation: self.fee_reservation.clone(),
        }
    }

    fn sync_lifecycle_from_legacy(&mut self) {
        let active = self.active_state_from_legacy();
        let terminal = SpotTerminalOrderState {
            status: self.status,
            status_reason: self.status_reason,
            filled_qty: self.filled_qty,
        };
        self.lifecycle = match self.status {
            SpotOrderStatus::Open => SpotOrderLifecycle::Active(active),
            SpotOrderStatus::PartiallyFilled => SpotOrderLifecycle::PartiallyFilled(active),
            SpotOrderStatus::Filled => SpotOrderLifecycle::Filled(terminal),
            SpotOrderStatus::Canceled => SpotOrderLifecycle::Canceled(terminal),
            SpotOrderStatus::Rejected => SpotOrderLifecycle::Rejected(terminal),
        };
    }

    fn transition_to(
        &mut self,
        next_version: u64,
        status: SpotOrderStatus,
        status_reason: Option<SpotOrderStatusReason>,
    ) {
        self.version = next_version;
        self.status = status;
        self.status_reason = status_reason;
        self.sync_lifecycle_from_legacy();
    }

    fn next_version(&self) -> Result<u64, SpotOrderV2MatchError> {
        self.version.checked_add(1).ok_or(SpotOrderV2MatchError::ArithmeticOverflow)
    }

    // 领域方法：订单生命周期推进
    //
    // use case 应通过这些聚合根方法推进订单生命周期，不得绕过它们直接修改
    // status、filled_qty、status_reason、version。

    /// 领域方法：按成交事实推进订单生命周期。
    ///
    /// 可 BDD 规格化的聚合根行为：成交后推进 filled quantity 与生命周期状态。
    pub(crate) fn fill(&mut self, added_fill_qty: u64) -> Result<(), SpotOrderV2MatchError> {
        let next_filled_qty = self
            .filled_qty
            .checked_add(added_fill_qty)
            .ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        if next_filled_qty > self.qty {
            return Err(SpotOrderV2MatchError::InconsistentExecutionState);
        }
        let next_version = self.next_version()?;
        self.filled_qty = next_filled_qty;
        self.transition_to(next_version, self.matched_status_for(next_filled_qty), None);
        Ok(())
    }

    /// 可 BDD 规格化的聚合根行为：taker 按 maker 优先级数组逐笔撮合。
    ///
    /// 该方法推进 taker / maker 订单状态，并返回本次撮合直接派生的成交事实。
    /// 没有 crossing maker 时返回空成交列表，由 use case 继续处理 TIF / ALO 规则。
    pub(crate) fn match_with_makers(
        &mut self,
        makers: &mut [SpotOrderV2],
        input: MatchSpotOrderV2Input,
    ) -> Result<MatchSpotOrderV2Outcome, SpotOrderV2BehaviorError> {
        self.ensure_matchable()?;

        let mut trades = Vec::new();
        for maker in makers.iter_mut() {
            let Some(terms) = spot_order_v2_next_trade_terms(self, maker)? else {
                break;
            };

            let taker_fee = self
                .fee_consume_requirement_for_trade(
                    terms.trade_qty,
                    terms.maker_price,
                    SpotTradeFeeRole::Taker,
                    input.maker_fee_bps,
                    input.taker_fee_bps,
                )?
                .amount;
            let maker_fee = maker
                .fee_consume_requirement_for_trade(
                    terms.trade_qty,
                    terms.maker_price,
                    SpotTradeFeeRole::Maker,
                    input.maker_fee_bps,
                    input.taker_fee_bps,
                )?
                .amount;
            let trade = SpotTrade::new(
                format!("{}-{}", input.match_id, trades.len() + 1),
                input.match_id.clone(),
                self.asset,
                self.symbol.clone(),
                self.order_id.clone(),
                maker.order_id.clone(),
                self.account_id.clone(),
                maker.account_id.clone(),
                self.side,
                terms.maker_price,
                terms.trade_qty,
                taker_fee,
                maker_fee,
            );

            maker.fill(terms.trade_qty)?;
            self.fill(terms.trade_qty)?;
            trades.push(trade);

            if self.remaining_qty().ok_or(SpotOrderV2BehaviorError::OrderNotMatchable)? == 0 {
                break;
            }
        }

        Ok(MatchSpotOrderV2Outcome { trades })
    }

    /// 可 BDD 规格化的聚合根行为：撤销现货订单并派生解冻流水。
    ///
    /// 该方法释放订单内 principal reservation，并返回下游余额流水单据；
    /// 实际余额落账仍由 use case 编排。
    pub(crate) fn cancel(
        &mut self,
        input: CancelSpotOrderV2Input,
    ) -> Result<CancelSpotOrderV2Outcome, SpotOrderV2BehaviorError> {
        if matches!(self.lifecycle, SpotOrderLifecycle::TriggerPending(_)) {
            let next_version = self.next_version()?;
            self.version = next_version;
            self.status = SpotOrderStatus::Canceled;
            self.status_reason = Some(SpotOrderStatusReason::CanceledByUser);
            self.lifecycle = SpotOrderLifecycle::Canceled(SpotTerminalOrderState {
                status: SpotOrderStatus::Canceled,
                status_reason: Some(SpotOrderStatusReason::CanceledByUser),
                filled_qty: 0,
            });
            return Ok(CancelSpotOrderV2Outcome { unfreeze_ledger_entry: None });
        }

        if !matches!(
            self.lifecycle,
            SpotOrderLifecycle::Active(_) | SpotOrderLifecycle::PartiallyFilled(_)
        ) {
            return Err(SpotOrderV2BehaviorError::OrderNotCancelable);
        }

        if !self.can_be_cancelled() {
            return Err(SpotOrderV2BehaviorError::OrderNotCancelable);
        }

        let release_amount = self.reservation.remaining_amount;
        if release_amount == 0 {
            return Err(SpotOrderV2BehaviorError::Reservation(ReservationError::InvalidAmount));
        }

        let released_reservation =
            self.reservation.release(release_amount, Some(ReservationCloseReason::Canceled))?;
        let unfreeze_ledger_entry = BalanceLedgerEntryV2::unfreeze(
            format!("balance-ledger:unfreeze:{}", self.order_id),
            self.account_id.clone(),
            self.reservation.asset_id.clone(),
            input.balance_entity_id,
            release_amount,
            BalanceLedgerReason::UnfreezeForCancel { order_id: self.order_id.clone() },
        )?;
        let next_version = self.next_version()?;

        self.reservation = released_reservation;
        self.transition_to(
            next_version,
            SpotOrderStatus::Canceled,
            Some(SpotOrderStatusReason::CanceledByUser),
        );

        Ok(CancelSpotOrderV2Outcome { unfreeze_ledger_entry: Some(unfreeze_ledger_entry) })
    }

    /// 按 IOC 部分成交后取消剩余数量语义关闭订单。
    fn cancel_ioc_unfilled(&mut self, next_filled_qty: u64) -> Result<(), SpotOrderV2MatchError> {
        let next_version = self.next_version()?;
        self.filled_qty = next_filled_qty;
        self.transition_to(
            next_version,
            SpotOrderStatus::Canceled,
            Some(SpotOrderStatusReason::IocCancelRejected),
        );
        Ok(())
    }

    /// 领域方法：按 IOC / 市价订单无流动性语义拒绝订单。
    ///
    /// 可 BDD 规格化的聚合根行为：IOC 或市价订单因无流动性被拒绝。
    pub(crate) fn reject_as_no_liquidity(&mut self) -> Result<(), SpotOrderV2MatchError> {
        let next_version = self.next_version()?;
        self.transition_to(
            next_version,
            SpotOrderStatus::Rejected,
            Some(self.no_liquidity_status_reason()),
        );
        Ok(())
    }

    /// 领域方法：应用 taker 本轮撮合结束后的业务结果。
    ///
    /// 可 BDD 规格化的聚合根行为：taker 本轮撮合结束后进入终态或成交态。
    pub(crate) fn finish_after_match(
        &mut self,
        added_fill_qty: u64,
    ) -> Result<(), SpotOrderV2MatchError> {
        let next_filled_qty = self
            .filled_qty
            .checked_add(added_fill_qty)
            .ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;

        match self.time_in_force {
            SpotOrderTimeInForce::Gtc | SpotOrderTimeInForce::Alo => {
                if added_fill_qty == 0 {
                    return Err(SpotOrderV2MatchError::NoTradesMatched);
                }
                self.fill(added_fill_qty)
            }
            SpotOrderTimeInForce::Ioc => {
                if added_fill_qty == 0 {
                    self.reject_as_no_liquidity()
                } else if next_filled_qty == self.qty {
                    self.fill(added_fill_qty)
                } else {
                    self.cancel_ioc_unfilled(next_filled_qty)
                }
            }
        }
    }

    /// 领域方法：将 ALO 订单按“会立即吃单”语义拒绝。
    ///
    /// 可 BDD 规格化的聚合根行为：ALO 会立即吃单时被拒绝。
    pub(crate) fn reject_as_bad_alo(&mut self) -> Result<(), SpotOrderV2MatchError> {
        let next_version = self.next_version()?;
        self.transition_to(
            next_version,
            SpotOrderStatus::Rejected,
            Some(SpotOrderStatusReason::BadAloPxRejected),
        );
        Ok(())
    }

    fn no_liquidity_status_reason(&self) -> SpotOrderStatusReason {
        if self.limit_price().is_none() {
            SpotOrderStatusReason::MarketOrderNoLiquidityRejected
        } else {
            SpotOrderStatusReason::IocCancelRejected
        }
    }
}

/// 下单后面对最优 maker 时的订单侧撮合结论。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpotOrderV2MatchingDecision {
    /// 非 IOC 且当前没有可成交 maker，订单保持挂单。
    Rest,
    /// ALO 订单会立即吃单，应按 bad ALO 拒绝。
    RejectAlo,
    /// 应进入逐笔撮合。
    Match,
}

/// 单笔 taker/maker 撮合条款。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpotOrderV2TradeTerms {
    /// 成交价格，取 maker 限价。
    pub maker_price: u64,
    /// 本笔成交数量。
    pub trade_qty: u64,
}

/// 推导订单面对当前最优 maker 时是否应挂单、拒绝或进入撮合。
pub(crate) fn spot_order_v2_matching_decision(
    taker: &SpotOrderV2,
    best_maker: Option<&SpotOrderV2>,
) -> Result<SpotOrderV2MatchingDecision, SpotOrderV2MatchError> {
    if taker.would_be_rejected_as_alo(best_maker)? {
        return Ok(SpotOrderV2MatchingDecision::RejectAlo);
    }
    if taker.should_enter_matching(best_maker)? {
        Ok(SpotOrderV2MatchingDecision::Match)
    } else {
        Ok(SpotOrderV2MatchingDecision::Rest)
    }
}

/// 推导 taker 与 maker 的下一笔成交条款。
///
/// 返回 `Ok(None)` 表示当前 maker 价格已经不再与 taker crossing。
pub(crate) fn spot_order_v2_next_trade_terms(
    taker: &SpotOrderV2,
    maker: &SpotOrderV2,
) -> Result<Option<SpotOrderV2TradeTerms>, SpotOrderV2MatchError> {
    maker.ensure_matchable()?;
    maker.ensure_compatible_maker_for(taker)?;
    if !taker.crosses_order(maker)? {
        return Ok(None);
    }

    let maker_price = maker.limit_price().ok_or(SpotOrderV2MatchError::MakerMustBeLimit)?;
    let taker_remaining =
        taker.remaining_qty().ok_or(SpotOrderV2MatchError::InconsistentExecutionState)?;
    let maker_remaining =
        maker.remaining_qty().ok_or(SpotOrderV2MatchError::InconsistentExecutionState)?;
    let trade_qty = taker_remaining.min(maker_remaining);
    if trade_qty == 0 {
        Ok(None)
    } else {
        Ok(Some(SpotOrderV2TradeTerms { maker_price, trade_qty }))
    }
}

impl FieldDiff for SpotOrderV2 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.order_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("exchange_oid", "", option_u64_value(self.exchange_oid)),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("execution", "", self.execution.as_str()),
            EntityFieldChange::new("time_in_force", "", self.time_in_force.as_str()),
            EntityFieldChange::new("price", "", self.order_price().to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("filled_qty", "", self.filled_qty.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            EntityFieldChange::new(
                "status_reason",
                "",
                option_status_reason_value(self.status_reason),
            ),
            EntityFieldChange::new("reserved_base", "", self.reserved_base.to_string()),
            EntityFieldChange::new("reserved_quote", "", self.reserved_quote.to_string()),
            EntityFieldChange::new("reservation_id", "", self.reservation.reservation_id.clone()),
            EntityFieldChange::new("reservation_asset_id", "", self.reservation.asset_id.clone()),
            EntityFieldChange::new(
                "reservation_kind",
                "",
                self.reservation.reservation_kind.as_str(),
            ),
            EntityFieldChange::new(
                "reservation_original_amount",
                "",
                self.reservation.original_amount.to_string(),
            ),
            EntityFieldChange::new(
                "reservation_consumed_amount",
                "",
                self.reservation.consumed_amount.to_string(),
            ),
            EntityFieldChange::new(
                "reservation_released_amount",
                "",
                self.reservation.released_amount.to_string(),
            ),
            EntityFieldChange::new(
                "reservation_remaining_amount",
                "",
                self.reservation.remaining_amount.to_string(),
            ),
            EntityFieldChange::new("reservation_status", "", self.reservation.status.as_str()),
            EntityFieldChange::new(
                "fee_reservation_id",
                "",
                self.fee_reservation.reservation_id.clone(),
            ),
            EntityFieldChange::new(
                "fee_reservation_asset_id",
                "",
                self.fee_reservation.asset_id.clone(),
            ),
            EntityFieldChange::new(
                "fee_reservation_kind",
                "",
                self.fee_reservation.reservation_kind.as_str(),
            ),
            EntityFieldChange::new(
                "fee_reservation_original_amount",
                "",
                self.fee_reservation.original_amount.to_string(),
            ),
            EntityFieldChange::new(
                "fee_reservation_consumed_amount",
                "",
                self.fee_reservation.consumed_amount.to_string(),
            ),
            EntityFieldChange::new(
                "fee_reservation_released_amount",
                "",
                self.fee_reservation.released_amount.to_string(),
            ),
            EntityFieldChange::new(
                "fee_reservation_remaining_amount",
                "",
                self.fee_reservation.remaining_amount.to_string(),
            ),
            EntityFieldChange::new(
                "fee_reservation_status",
                "",
                self.fee_reservation.status.as_str(),
            ),
            EntityFieldChange::new(
                "client_order_id",
                "",
                self.client_order_id.clone().unwrap_or_default(),
            ),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(&mut changes, "asset", self.asset.to_string(), other.asset.to_string());
        push_change(
            &mut changes,
            "exchange_oid",
            option_u64_value(self.exchange_oid),
            option_u64_value(other.exchange_oid),
        );
        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "symbol", &self.symbol, &other.symbol);
        push_change(&mut changes, "side", self.side.as_str(), other.side.as_str());
        push_change(&mut changes, "execution", self.execution.as_str(), other.execution.as_str());
        push_change(
            &mut changes,
            "time_in_force",
            self.time_in_force.as_str(),
            other.time_in_force.as_str(),
        );
        push_change(
            &mut changes,
            "price",
            self.order_price().to_string(),
            other.order_price().to_string(),
        );
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(
            &mut changes,
            "filled_qty",
            self.filled_qty.to_string(),
            other.filled_qty.to_string(),
        );
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        push_change(
            &mut changes,
            "status_reason",
            option_status_reason_value(self.status_reason),
            option_status_reason_value(other.status_reason),
        );
        push_change(
            &mut changes,
            "reserved_base",
            self.reserved_base.to_string(),
            other.reserved_base.to_string(),
        );
        push_change(
            &mut changes,
            "reserved_quote",
            self.reserved_quote.to_string(),
            other.reserved_quote.to_string(),
        );
        push_change(
            &mut changes,
            "reservation_id",
            &self.reservation.reservation_id,
            &other.reservation.reservation_id,
        );
        push_change(
            &mut changes,
            "reservation_asset_id",
            &self.reservation.asset_id,
            &other.reservation.asset_id,
        );
        push_change(
            &mut changes,
            "reservation_kind",
            self.reservation.reservation_kind.as_str(),
            other.reservation.reservation_kind.as_str(),
        );
        push_change(
            &mut changes,
            "reservation_original_amount",
            self.reservation.original_amount.to_string(),
            other.reservation.original_amount.to_string(),
        );
        push_change(
            &mut changes,
            "reservation_consumed_amount",
            self.reservation.consumed_amount.to_string(),
            other.reservation.consumed_amount.to_string(),
        );
        push_change(
            &mut changes,
            "reservation_released_amount",
            self.reservation.released_amount.to_string(),
            other.reservation.released_amount.to_string(),
        );
        push_change(
            &mut changes,
            "reservation_remaining_amount",
            self.reservation.remaining_amount.to_string(),
            other.reservation.remaining_amount.to_string(),
        );
        push_change(
            &mut changes,
            "reservation_status",
            self.reservation.status.as_str(),
            other.reservation.status.as_str(),
        );
        push_change(
            &mut changes,
            "fee_reservation_id",
            &self.fee_reservation.reservation_id,
            &other.fee_reservation.reservation_id,
        );
        push_change(
            &mut changes,
            "fee_reservation_asset_id",
            &self.fee_reservation.asset_id,
            &other.fee_reservation.asset_id,
        );
        push_change(
            &mut changes,
            "fee_reservation_kind",
            self.fee_reservation.reservation_kind.as_str(),
            other.fee_reservation.reservation_kind.as_str(),
        );
        push_change(
            &mut changes,
            "fee_reservation_original_amount",
            self.fee_reservation.original_amount.to_string(),
            other.fee_reservation.original_amount.to_string(),
        );
        push_change(
            &mut changes,
            "fee_reservation_consumed_amount",
            self.fee_reservation.consumed_amount.to_string(),
            other.fee_reservation.consumed_amount.to_string(),
        );
        push_change(
            &mut changes,
            "fee_reservation_released_amount",
            self.fee_reservation.released_amount.to_string(),
            other.fee_reservation.released_amount.to_string(),
        );
        push_change(
            &mut changes,
            "fee_reservation_remaining_amount",
            self.fee_reservation.remaining_amount.to_string(),
            other.fee_reservation.remaining_amount.to_string(),
        );
        push_change(
            &mut changes,
            "fee_reservation_status",
            self.fee_reservation.status.as_str(),
            other.fee_reservation.status.as_str(),
        );
        push_change(
            &mut changes,
            "client_order_id",
            self.client_order_id.clone().unwrap_or_default(),
            other.client_order_id.clone().unwrap_or_default(),
        );
        push_change(&mut changes, "version", self.version.to_string(), other.version.to_string());

        changes
    }
}

impl Entity for SpotOrderV2 {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.order_id.clone()
    }

    fn entity_type() -> u8 {
        SPOT_ORDER_V2_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::BusinessVoucher
    }

    fn is_mi_chain_root() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn entity_version(&self) -> u64 {
        self.version
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "order_id"
            | "account_id"
            | "symbol"
            | "side"
            | "execution"
            | "time_in_force"
            | "status"
            | "status_reason"
            | "client_order_id"
            | "reservation_id"
            | "reservation_asset_id"
            | "reservation_kind"
            | "reservation_status"
            | "fee_reservation_id"
            | "fee_reservation_asset_id"
            | "fee_reservation_kind"
            | "fee_reservation_status" => 0,
            "asset"
            | "exchange_oid"
            | "qty"
            | "filled_qty"
            | "price"
            | "reserved_base"
            | "reserved_quote"
            | "version"
            | "reservation_original_amount"
            | "reservation_consumed_amount"
            | "reservation_released_amount"
            | "reservation_remaining_amount"
            | "fee_reservation_original_amount"
            | "fee_reservation_consumed_amount"
            | "fee_reservation_released_amount"
            | "fee_reservation_remaining_amount" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_order_entity_id(&self.order_id))
    }
}

#[cfg(test)]
pub(crate) fn test_principal_reservation(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    qty: u64,
    order_price: u64,
) -> Reservation {
    match SpotOrderV2::principal_reservation(
        order_id,
        account_id,
        side,
        qty,
        order_price,
        "BTC",
        "USDT",
    ) {
        Ok(reservation) => reservation,
        Err(error) => panic!("invalid test spot order reservation: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buy_order() -> SpotOrderV2 {
        SpotOrderV2::new(
            "order-buy".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            SpotOrderTimeInForce::Gtc,
            2,
            0,
            SpotOrderStatus::Open,
            None,
            0,
            200,
            test_principal_reservation("order-buy", "trader-1", SpotOrderSide::Buy, 2, 100),
            Some("cloid-1".to_string()),
            1,
        )
    }

    fn sell_order() -> SpotOrderV2 {
        SpotOrderV2::new(
            "order-sell".to_string(),
            10_001,
            Some(43),
            "trader-2".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price: 105 },
            SpotOrderTimeInForce::Gtc,
            3,
            0,
            SpotOrderStatus::Open,
            None,
            3,
            0,
            test_principal_reservation("order-sell", "trader-2", SpotOrderSide::Sell, 3, 105),
            None,
            1,
        )
    }

    fn market_buy_order() -> SpotOrderV2 {
        SpotOrderV2::new(
            "order-market-buy".to_string(),
            10_001,
            None,
            "trader-3".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Market { aggressive_price: 120 },
            SpotOrderTimeInForce::Ioc,
            2,
            0,
            SpotOrderStatus::Open,
            None,
            0,
            240,
            test_principal_reservation("order-market-buy", "trader-3", SpotOrderSide::Buy, 2, 120),
            None,
            1,
        )
    }

    fn maker_sell(price: u64) -> SpotOrderV2 {
        SpotOrderV2::new(
            format!("maker-{price}"),
            10_001,
            Some(price),
            "maker".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            1,
            0,
            SpotOrderStatus::Open,
            None,
            1,
            0,
            test_principal_reservation(
                format!("maker-{price}").as_str(),
                "maker",
                SpotOrderSide::Sell,
                1,
                price,
            ),
            None,
            1,
        )
    }

    #[test]
    fn hold_requirement_uses_quote_for_buy_and_base_for_sell() {
        assert_eq!(
            buy_order().hold_requirement(),
            Some(SpotOrderHoldRequirement { asset: SpotOrderHoldAsset::Quote, amount: 200 })
        );
        assert_eq!(
            sell_order().hold_requirement(),
            Some(SpotOrderHoldRequirement { asset: SpotOrderHoldAsset::Base, amount: 3 })
        );
    }

    #[test]
    fn hold_requirement_uses_aggressive_price_for_market_buy() {
        assert_eq!(
            market_buy_order().hold_requirement(),
            Some(SpotOrderHoldRequirement { asset: SpotOrderHoldAsset::Quote, amount: 240 })
        );
    }

    #[test]
    fn fee_hold_requirement_uses_worst_case_bps_and_quote_asset() {
        assert_eq!(
            buy_order().fee_hold_requirement(5, 10),
            Some(SpotOrderFeeHoldRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                worst_case_role: SpotTradeFeeRole::Taker,
                fee_bps: 10,
            })
        );
        assert_eq!(
            sell_order().fee_hold_requirement(12, 5),
            Some(SpotOrderFeeHoldRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                worst_case_role: SpotTradeFeeRole::Maker,
                fee_bps: 12,
            })
        );
    }

    #[test]
    fn fee_hold_requirement_rounds_up_and_handles_zero_bps() {
        let order = SpotOrderV2::new(
            "order-round-up".to_string(),
            10_001,
            Some(44),
            "trader-4".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 101 },
            SpotOrderTimeInForce::Gtc,
            3,
            0,
            SpotOrderStatus::Open,
            None,
            0,
            303,
            test_principal_reservation("order-round-up", "trader-4", SpotOrderSide::Buy, 3, 101),
            None,
            1,
        );

        assert_eq!(
            order.fee_hold_requirement(0, 0),
            Some(SpotOrderFeeHoldRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 0,
                worst_case_role: SpotTradeFeeRole::Taker,
                fee_bps: 0,
            })
        );
        assert_eq!(
            order.fee_hold_requirement(0, 1),
            Some(SpotOrderFeeHoldRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                worst_case_role: SpotTradeFeeRole::Taker,
                fee_bps: 1,
            })
        );
    }

    #[test]
    fn cancel_release_requirements_include_principal_and_fee_for_cancelable_orders() {
        let open = buy_order();
        let partial =
            SpotOrderV2 { filled_qty: 1, status: SpotOrderStatus::PartiallyFilled, ..buy_order() };
        let canceled = SpotOrderV2 {
            status: SpotOrderStatus::Canceled,
            status_reason: Some(SpotOrderStatusReason::CanceledByUser),
            ..buy_order()
        };
        let filled = SpotOrderV2 {
            filled_qty: 2,
            status: SpotOrderStatus::Filled,
            status_reason: Some(SpotOrderStatusReason::Filled),
            ..buy_order()
        };
        let rejected = SpotOrderV2 {
            status: SpotOrderStatus::Rejected,
            status_reason: Some(SpotOrderStatusReason::RejectedAtPlacement),
            ..buy_order()
        };

        let expected_principal = Some(SpotOrderReleaseRequirement {
            asset: SpotOrderHoldAsset::Quote,
            amount: 200,
            reason: SpotOrderReleaseReason::Canceled,
        });
        let expected_fee = Some(SpotOrderReleaseRequirement {
            asset: SpotOrderHoldAsset::Quote,
            amount: 1,
            reason: SpotOrderReleaseReason::Canceled,
        });
        assert_eq!(
            open.cancel_release_requirements(5, 10),
            SpotOrderReleaseRequirements { principal: expected_principal, fee: expected_fee }
        );
        assert_eq!(
            partial.cancel_release_requirements(5, 10),
            SpotOrderReleaseRequirements { principal: expected_principal, fee: expected_fee }
        );
        assert_eq!(
            canceled.cancel_release_requirements(5, 10),
            SpotOrderReleaseRequirements { principal: None, fee: None }
        );
        assert_eq!(
            filled.cancel_release_requirements(5, 10),
            SpotOrderReleaseRequirements { principal: None, fee: None }
        );
        assert_eq!(
            rejected.cancel_release_requirements(5, 10),
            SpotOrderReleaseRequirements { principal: None, fee: None }
        );
    }

    #[test]
    fn cancel_release_requirements_skip_zero_fee() {
        let open = buy_order();

        assert_eq!(
            open.cancel_release_requirements(0, 0),
            SpotOrderReleaseRequirements {
                principal: Some(SpotOrderReleaseRequirement {
                    asset: SpotOrderHoldAsset::Quote,
                    amount: 200,
                    reason: SpotOrderReleaseReason::Canceled,
                }),
                fee: None,
            }
        );
    }

    #[test]
    fn terminal_release_requirements_follow_terminal_status_semantics() {
        let open = buy_order();
        let partial =
            SpotOrderV2 { filled_qty: 1, status: SpotOrderStatus::PartiallyFilled, ..buy_order() };
        let filled = SpotOrderV2 {
            filled_qty: 2,
            status: SpotOrderStatus::Filled,
            status_reason: Some(SpotOrderStatusReason::Filled),
            ..buy_order()
        };
        let rejected = SpotOrderV2 {
            status: SpotOrderStatus::Rejected,
            status_reason: Some(SpotOrderStatusReason::RejectedAtPlacement),
            ..buy_order()
        };
        let ioc_canceled = SpotOrderV2 {
            time_in_force: SpotOrderTimeInForce::Ioc,
            filled_qty: 1,
            status: SpotOrderStatus::Canceled,
            status_reason: Some(SpotOrderStatusReason::IocCancelRejected),
            ..market_buy_order()
        };

        assert_eq!(
            open.terminal_release_requirements(5, 10),
            SpotOrderReleaseRequirements { principal: None, fee: None }
        );
        assert_eq!(
            partial.terminal_release_requirements(5, 10),
            SpotOrderReleaseRequirements { principal: None, fee: None }
        );
        assert_eq!(
            filled.terminal_release_requirements(5, 10),
            SpotOrderReleaseRequirements {
                principal: Some(SpotOrderReleaseRequirement {
                    asset: SpotOrderHoldAsset::Quote,
                    amount: 200,
                    reason: SpotOrderReleaseReason::FilledCleanup,
                }),
                fee: Some(SpotOrderReleaseRequirement {
                    asset: SpotOrderHoldAsset::Quote,
                    amount: 1,
                    reason: SpotOrderReleaseReason::FilledCleanup,
                }),
            }
        );
        assert_eq!(
            rejected.terminal_release_requirements(5, 10),
            SpotOrderReleaseRequirements {
                principal: Some(SpotOrderReleaseRequirement {
                    asset: SpotOrderHoldAsset::Quote,
                    amount: 200,
                    reason: SpotOrderReleaseReason::Rejected,
                }),
                fee: Some(SpotOrderReleaseRequirement {
                    asset: SpotOrderHoldAsset::Quote,
                    amount: 1,
                    reason: SpotOrderReleaseReason::Rejected,
                }),
            }
        );
        assert_eq!(
            ioc_canceled.terminal_release_requirements(5, 10),
            SpotOrderReleaseRequirements {
                principal: Some(SpotOrderReleaseRequirement {
                    asset: SpotOrderHoldAsset::Quote,
                    amount: 240,
                    reason: SpotOrderReleaseReason::IocUnfilled,
                }),
                fee: Some(SpotOrderReleaseRequirement {
                    asset: SpotOrderHoldAsset::Quote,
                    amount: 1,
                    reason: SpotOrderReleaseReason::IocUnfilled,
                }),
            }
        );
        assert!(ioc_canceled.has_terminal_release());
    }

    #[test]
    fn fee_consume_requirement_uses_actual_role_bps() {
        assert_eq!(
            buy_order().fee_consume_requirement_for_trade(2, 100, SpotTradeFeeRole::Maker, 5, 10),
            Ok(SpotOrderFeeConsumeRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                role: SpotTradeFeeRole::Maker,
                fee_bps: 5,
            })
        );
        assert_eq!(
            buy_order().fee_consume_requirement_for_trade(2, 100, SpotTradeFeeRole::Taker, 5, 10),
            Ok(SpotOrderFeeConsumeRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                role: SpotTradeFeeRole::Taker,
                fee_bps: 10,
            })
        );
    }

    #[test]
    fn reserved_snapshots_still_validate_against_order_side() {
        assert!(buy_order().has_consistent_reserved_base());
        assert!(buy_order().has_consistent_reserved_quote());
        assert!(sell_order().has_consistent_reserved_base());
        assert!(sell_order().has_consistent_reserved_quote());
    }

    #[test]
    fn matching_decision_and_trade_terms_hide_crossing_details() -> Result<(), SpotOrderV2MatchError>
    {
        let buy = buy_order();

        assert_eq!(spot_order_v2_matching_decision(&buy, None)?, SpotOrderV2MatchingDecision::Rest);
        assert_eq!(
            spot_order_v2_matching_decision(&buy, Some(&maker_sell(90)))?,
            SpotOrderV2MatchingDecision::Match
        );
        assert_eq!(
            spot_order_v2_matching_decision(&buy, Some(&maker_sell(120)))?,
            SpotOrderV2MatchingDecision::Rest
        );
        assert_eq!(
            spot_order_v2_next_trade_terms(&buy, &maker_sell(90))?,
            Some(SpotOrderV2TradeTerms { maker_price: 90, trade_qty: 1 })
        );
        assert_eq!(spot_order_v2_next_trade_terms(&buy, &maker_sell(120))?, None);
        Ok(())
    }

    #[test]
    fn new_assembles_entrusted_open_order_fact() {
        let order = buy_order();

        assert_eq!(order.status, SpotOrderStatus::Open);
        assert_eq!(order.status_reason, None);
        assert_eq!(order.filled_qty, 0);
        assert_eq!(order.version, 1);
    }

    #[test]
    fn cancel_rejects_non_cancelable_order_without_partial_mutation() {
        let mut filled = SpotOrderV2 {
            filled_qty: 2,
            status: SpotOrderStatus::Filled,
            status_reason: Some(SpotOrderStatusReason::Filled),
            ..buy_order()
        };

        assert_eq!(
            filled.cancel(CancelSpotOrderV2Input {
                balance_entity_id: "balance:trader-1:USDT".to_string(),
            }),
            Err(SpotOrderV2BehaviorError::OrderNotCancelable)
        );
        assert_eq!(filled.status, SpotOrderStatus::Filled);
        assert_eq!(filled.version, 1);
    }

    #[test]
    fn fill_advances_version_once_without_partial_mutation() {
        let mut order = buy_order();

        assert_eq!(order.fill(1), Ok(()));
        assert_eq!(order.filled_qty, 1);
        assert_eq!(order.status, SpotOrderStatus::PartiallyFilled);
        assert_eq!(order.version, 2);
    }

    #[test]
    fn lifecycle_transition_rejects_version_overflow_without_partial_mutation() {
        let mut order = SpotOrderV2 { version: u64::MAX, ..buy_order() };

        assert_eq!(order.fill(1), Err(SpotOrderV2MatchError::ArithmeticOverflow));
        assert_eq!(order.filled_qty, 0);
        assert_eq!(order.status, SpotOrderStatus::Open);
        assert_eq!(order.version, u64::MAX);
    }
}
