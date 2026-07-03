use std::hash::{Hash, Hasher};

use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, FinancialClassification,
    FourColorArchetype,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::spot_order::{
    SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce,
};

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

const FEE_BPS_DENOMINATOR: u64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpotOrderV2Finalization {
    pub(crate) next_filled_qty: u64,
    pub(crate) status: SpotOrderStatus,
    pub(crate) status_reason: Option<SpotOrderStatusReason>,
}

/// `SpotOrder v2` 的目标态订单聚合。
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
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
    /// 当前订单实体版本，用于生成可重放更新事件。
    pub version: u64,
}

impl SpotOrderV2 {
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
        client_order_id: Option<String>,
        version: u64,
    ) -> Self {
        Self {
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
            client_order_id,
            version,
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

    /// 返回订单限价价格。
    pub fn limit_price(&self) -> Option<u64> {
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

    /// 返回订单当前剩余可成交数量。
    pub fn remaining_qty(&self) -> Option<u64> {
        self.qty.checked_sub(self.filled_qty)
    }

    /// 返回订单建立时的 quote 冻结快照。
    fn initial_quote_hold_snapshot(&self) -> Option<u64> {
        self.qty.checked_mul(self.order_price())
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
        if fee_bps == 0 {
            return Some(0);
        }

        let notional = self.initial_quote_hold_snapshot()?;
        let scaled = notional.checked_mul(fee_bps)?;
        let numerator = scaled.checked_add(FEE_BPS_DENOMINATOR.checked_sub(1)?)?;
        Some(numerator / FEE_BPS_DENOMINATOR)
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
    pub fn cancel_release_requirement(&self) -> Option<SpotOrderReleaseRequirement> {
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
    pub fn terminal_release_requirement(&self) -> Option<SpotOrderReleaseRequirement> {
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
    pub fn fee_cancel_release_requirement(
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
    pub fn fee_terminal_release_requirement(
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

        let trade_notional =
            trade_qty.checked_mul(trade_price).ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        let scaled =
            trade_notional.checked_mul(fee_bps).ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        let numerator = scaled
            .checked_add(FEE_BPS_DENOMINATOR - 1)
            .ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        let amount = numerator / FEE_BPS_DENOMINATOR;

        Ok(SpotOrderFeeConsumeRequirement { asset: self.fee_hold_asset(), amount, role, fee_bps })
    }

    /// 返回订单在当前终态下是否存在订单侧释放需求。
    pub fn has_terminal_release(&self) -> bool {
        self.terminal_release_requirement().is_some()
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
            SpotOrderSide::Buy => self.initial_quote_hold_snapshot() == Some(self.reserved_quote),
            SpotOrderSide::Sell => self.reserved_quote == 0,
        }
    }

    /// 返回 `reserved_base` 是否仍符合买卖方向与订单快照语义。
    pub fn has_consistent_reserved_base(&self) -> bool {
        match self.side {
            SpotOrderSide::Buy => self.reserved_base == 0,
            SpotOrderSide::Sell => self.reserved_base == self.qty,
        }
    }

    /// 返回该订单是否应进入撮合。
    pub(crate) fn should_enter_matching(
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
    pub(crate) fn ensure_compatible_maker_for(
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
    pub(crate) fn crosses_maker_price(&self, maker_price: u64) -> bool {
        match self.side {
            SpotOrderSide::Buy => self.order_price() >= maker_price,
            SpotOrderSide::Sell => self.order_price() <= maker_price,
        }
    }

    /// 返回该订单是否会和给定 maker 订单成交。
    pub(crate) fn crosses_order(&self, maker: &SpotOrderV2) -> Result<bool, SpotOrderV2MatchError> {
        if self.side == maker.side {
            return Err(SpotOrderV2MatchError::SameSideMaker);
        }

        let maker_price = maker.limit_price().ok_or(SpotOrderV2MatchError::MakerMustBeLimit)?;
        Ok(self.crosses_maker_price(maker_price))
    }

    /// 返回该订单是否会因 ALO 语义在进入撮合前被拒绝。
    pub(crate) fn would_be_rejected_as_alo(
        &self,
        best_maker: Option<&SpotOrderV2>,
    ) -> Result<bool, SpotOrderV2MatchError> {
        if self.time_in_force != SpotOrderTimeInForce::Alo {
            return Ok(false);
        }

        best_maker.map_or(Ok(false), |maker| self.crosses_order(maker))
    }

    /// 返回给定成交后数量对应的撮合状态。
    pub(crate) fn matched_status_for(&self, next_filled_qty: u64) -> SpotOrderStatus {
        if next_filled_qty == self.qty {
            SpotOrderStatus::Filled
        } else {
            SpotOrderStatus::PartiallyFilled
        }
    }

    /// 应用一次 maker 成交推进。
    pub(crate) fn apply_fill(&mut self, added_fill_qty: u64) -> Result<(), SpotOrderV2MatchError> {
        let next_filled_qty = self
            .filled_qty
            .checked_add(added_fill_qty)
            .ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        let next_version =
            self.version.checked_add(1).ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        self.filled_qty = next_filled_qty;
        self.status = self.matched_status_for(next_filled_qty);
        self.version = next_version;
        Ok(())
    }

    /// 返回本轮撮合结束后 taker 订单应进入的生命周期状态。
    pub(crate) fn finalize_after_match(
        &self,
        added_fill_qty: u64,
    ) -> Result<SpotOrderV2Finalization, SpotOrderV2MatchError> {
        let next_filled_qty = self
            .filled_qty
            .checked_add(added_fill_qty)
            .ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;

        match self.time_in_force {
            SpotOrderTimeInForce::Gtc => {
                if added_fill_qty == 0 {
                    return Err(SpotOrderV2MatchError::NoTradesMatched);
                }

                Ok(SpotOrderV2Finalization {
                    next_filled_qty,
                    status: self.matched_status_for(next_filled_qty),
                    status_reason: None,
                })
            }
            SpotOrderTimeInForce::Ioc => {
                let (status, status_reason) = if added_fill_qty == 0 {
                    (SpotOrderStatus::Rejected, Some(self.no_liquidity_status_reason()))
                } else if next_filled_qty == self.qty {
                    (SpotOrderStatus::Filled, None)
                } else {
                    (SpotOrderStatus::Canceled, Some(SpotOrderStatusReason::IocCancelRejected))
                };

                Ok(SpotOrderV2Finalization { next_filled_qty, status, status_reason })
            }
            SpotOrderTimeInForce::Alo => {
                if added_fill_qty == 0 {
                    return Err(SpotOrderV2MatchError::NoTradesMatched);
                }

                Ok(SpotOrderV2Finalization {
                    next_filled_qty,
                    status: self.matched_status_for(next_filled_qty),
                    status_reason: None,
                })
            }
        }
    }

    /// 应用 taker 撮合结束后的目标状态。
    pub(crate) fn apply_finalization(
        &mut self,
        finalization: SpotOrderV2Finalization,
    ) -> Result<(), SpotOrderV2MatchError> {
        let next_version =
            self.version.checked_add(1).ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        self.filled_qty = finalization.next_filled_qty;
        self.status = finalization.status;
        self.status_reason = finalization.status_reason;
        self.version = next_version;
        Ok(())
    }

    /// 将 ALO 订单按“会立即吃单”语义拒绝。
    pub(crate) fn reject_as_bad_alo(&mut self) -> Result<(), SpotOrderV2MatchError> {
        let next_version =
            self.version.checked_add(1).ok_or(SpotOrderV2MatchError::ArithmeticOverflow)?;
        self.status = SpotOrderStatus::Rejected;
        self.status_reason = Some(SpotOrderStatusReason::BadAloPxRejected);
        self.version = next_version;
        Ok(())
    }

    pub(crate) fn no_liquidity_status_reason(&self) -> SpotOrderStatusReason {
        if self.limit_price().is_none() {
            SpotOrderStatusReason::MarketOrderNoLiquidityRejected
        } else {
            SpotOrderStatusReason::IocCancelRejected
        }
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
            "client_order_id",
            self.client_order_id.clone().unwrap_or_default(),
            other.client_order_id.clone().unwrap_or_default(),
        );
        push_change(&mut changes, "version", self.version.to_string(), other.version.to_string());

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "order_id" | "account_id" | "symbol" | "side" | "execution" | "time_in_force"
            | "status" | "status_reason" | "client_order_id" => 0,
            "asset" | "exchange_oid" | "qty" | "filled_qty" | "price" | "reserved_base"
            | "reserved_quote" | "version" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_order_entity_id(&self.order_id))
    }
}

fn stable_order_entity_id(value: &str) -> i64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn push_change(
    changes: &mut Vec<EntityFieldChange>,
    field_name: &'static str,
    old_value: impl Into<String>,
    new_value: impl Into<String>,
) {
    let old_value = old_value.into();
    let new_value = new_value.into();
    if old_value != new_value {
        changes.push(EntityFieldChange::new(field_name, old_value, new_value));
    }
}

fn option_u64_value(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn option_status_reason_value(value: Option<SpotOrderStatusReason>) -> &'static str {
    value.map(SpotOrderStatusReason::as_str).unwrap_or_default()
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
    fn cancel_release_requirement_only_exists_for_cancelable_orders() {
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

        assert_eq!(
            open.cancel_release_requirement(),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 200,
                reason: SpotOrderReleaseReason::Canceled,
            })
        );
        assert_eq!(
            partial.cancel_release_requirement(),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 200,
                reason: SpotOrderReleaseReason::Canceled,
            })
        );
        assert_eq!(canceled.cancel_release_requirement(), None);
        assert_eq!(filled.cancel_release_requirement(), None);
        assert_eq!(rejected.cancel_release_requirement(), None);
    }

    #[test]
    fn fee_cancel_release_requirement_only_exists_for_cancelable_orders() {
        let open = buy_order();
        let canceled = SpotOrderV2 {
            status: SpotOrderStatus::Canceled,
            status_reason: Some(SpotOrderStatusReason::CanceledByUser),
            ..buy_order()
        };

        assert_eq!(
            open.fee_cancel_release_requirement(5, 10),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                reason: SpotOrderReleaseReason::Canceled,
            })
        );
        assert_eq!(canceled.fee_cancel_release_requirement(5, 10), None);
    }

    #[test]
    fn terminal_release_requirement_follows_terminal_status_semantics() {
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

        assert_eq!(open.terminal_release_requirement(), None);
        assert_eq!(partial.terminal_release_requirement(), None);
        assert_eq!(
            filled.terminal_release_requirement(),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 200,
                reason: SpotOrderReleaseReason::FilledCleanup,
            })
        );
        assert_eq!(
            rejected.terminal_release_requirement(),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 200,
                reason: SpotOrderReleaseReason::Rejected,
            })
        );
        assert_eq!(
            ioc_canceled.terminal_release_requirement(),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 240,
                reason: SpotOrderReleaseReason::IocUnfilled,
            })
        );
        assert!(ioc_canceled.has_terminal_release());
    }

    #[test]
    fn fee_terminal_release_requirement_follows_terminal_status_semantics() {
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
            filled.fee_terminal_release_requirement(5, 10),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                reason: SpotOrderReleaseReason::FilledCleanup,
            })
        );
        assert_eq!(
            rejected.fee_terminal_release_requirement(5, 10),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                reason: SpotOrderReleaseReason::Rejected,
            })
        );
        assert_eq!(
            ioc_canceled.fee_terminal_release_requirement(5, 10),
            Some(SpotOrderReleaseRequirement {
                asset: SpotOrderHoldAsset::Quote,
                amount: 1,
                reason: SpotOrderReleaseReason::IocUnfilled,
            })
        );
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
    fn should_enter_matching_and_crossing_semantics_still_live_on_order() {
        let buy = buy_order();

        assert_eq!(buy.should_enter_matching(None), Ok(false));
        assert_eq!(buy.should_enter_matching(Some(&maker_sell(90))), Ok(true));
        assert_eq!(buy.should_enter_matching(Some(&maker_sell(120))), Ok(false));
        assert_eq!(buy.crosses_order(&maker_sell(90)), Ok(true));
        assert_eq!(buy.crosses_order(&maker_sell(120)), Ok(false));
    }

    #[test]
    fn finalize_after_match_keeps_ioc_terminal_behavior_on_order() {
        let order = market_buy_order();

        let partial = order.finalize_after_match(1).unwrap();
        assert_eq!(partial.next_filled_qty, 1);
        assert_eq!(partial.status, SpotOrderStatus::Canceled);
        assert_eq!(partial.status_reason, Some(SpotOrderStatusReason::IocCancelRejected));

        let none = order.finalize_after_match(0).unwrap();
        assert_eq!(none.status, SpotOrderStatus::Rejected);
    }
}
