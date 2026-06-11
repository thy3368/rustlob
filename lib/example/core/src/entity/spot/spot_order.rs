use common_entity::{Entity, EntityError, EntityFieldChange};

use crate::use_case::MatchSpotOrderError;

#[cfg(test)]
mod spot_order_fixed_scenarios;

#[cfg(test)]
mod spot_order_exhaustive_scenarios;

#[cfg(test)]
mod spot_order_match_semantics;

#[cfg(test)]
pub(crate) mod spot_order_scenarios;

const SPOT_ORDER_ENTITY_TYPE: u8 = 3;
const SPOT_CONDITIONAL_ORDER_ENTITY_TYPE: u8 = 4;

/// 现货订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderSide {
    /// 买入订单。
    Buy,
    /// 卖出订单。
    Sell,
}

impl SpotOrderSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// 现货订单执行方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderExecution {
    /// 市价意图。Hyperliquid adapter 可映射为 IOC + 激进限价。
    Market {
        /// 市价意图使用的激进价格，用于冻结上限和 Hyperliquid `p` 字段。
        aggressive_price: u64,
    },
    /// 限价意图，`price` 是 core fixed-point 整数价格。
    Limit {
        /// quote 计价价格。
        price: u64,
    },
}

impl SpotOrderExecution {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Market { .. } => "market",
            Self::Limit { .. } => "limit",
        }
    }

    /// 返回限价价格；市价意图没有稳定限价价格。
    pub const fn limit_price(self) -> Option<u64> {
        match self {
            Self::Market { .. } => None,
            Self::Limit { price } => Some(price),
        }
    }

    /// 返回需要提交给 Hyperliquid 的价格字段。
    ///
    /// 限价单返回限价价格；市价意图返回 adapter 使用的激进价格。
    pub const fn order_price(self) -> u64 {
        match self {
            Self::Market { aggressive_price } => aggressive_price,
            Self::Limit { price } => price,
        }
    }
}

/// 现货限价订单有效方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderTimeInForce {
    /// 一直有效，直到成交或取消。
    Gtc,
    /// 立即成交，剩余取消。
    Ioc,
    /// 只做 Maker，若会立即吃单则拒绝。
    Alo,
}

impl SpotOrderTimeInForce {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gtc => "gtc",
            Self::Ioc => "ioc",
            Self::Alo => "alo",
        }
    }
}

/// 条件单触发角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderTriggerRole {
    /// 止盈触发单。
    TakeProfit,
    /// 止损触发单。
    StopLoss,
}

impl SpotOrderTriggerRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TakeProfit => "take_profit",
            Self::StopLoss => "stop_loss",
        }
    }
}

/// 已进入执行流程的现货订单生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderStatus {
    /// 订单已进入执行流程，尚未成交。
    Open,
    /// 订单已部分成交，剩余数量仍在业务上可撤。
    PartiallyFilled,
    /// 订单已完全成交。
    Filled,
    /// 订单已取消。
    Canceled,
    /// 订单提交时被交易所拒绝。
    Rejected,
}

impl SpotOrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::PartiallyFilled => "partially_filled",
            Self::Filled => "filled",
            Self::Canceled => "canceled",
            Self::Rejected => "rejected",
        }
    }

    pub const fn is_cancelable(self) -> bool {
        matches!(self, Self::Open | Self::PartiallyFilled)
    }
}

/// 未触发条件单生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotConditionalOrderStatus {
    /// 条件单已接受，等待触发。
    Open,
    /// 条件单已触发，并转换为 active `SpotOrder`。
    Triggered,
    /// 条件单触发前被取消。
    Canceled,
    /// 条件单提交时被交易所拒绝。
    Rejected,
    /// 条件单过期。
    Expired,
}

impl SpotConditionalOrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Triggered => "triggered",
            Self::Canceled => "canceled",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub const fn is_cancelable(self) -> bool {
        matches!(self, Self::Open)
    }
}

/// Hyperliquid 订单状态原因。
///
/// 该枚举对齐 `orderStatus` 返回的细分状态；其中部分状态只会出现在 perp，
/// 但保留枚举值可以让 adapter 无损回放交易所状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderStatusReason {
    /// 完全成交后终止。
    Filled,
    /// 用户主动取消。
    CanceledByUser,
    /// 条件订单已触发。
    Triggered,
    /// 下单时被拒绝。
    RejectedAtPlacement,
    /// 保证金不足导致取消。
    MarginCanceled,
    /// 金库提款导致取消。
    VaultWithdrawalCanceled,
    /// 开仓量上限导致取消。
    OpenInterestCapCanceled,
    /// 自成交保护导致取消。
    SelfTradeCanceled,
    /// 只减仓订单没有减少仓位导致取消。
    ReduceOnlyCanceled,
    /// TP/SL 兄弟订单已成交导致取消。
    SiblingFilledCanceled,
    /// 资产下架导致取消。
    DelistedCanceled,
    /// 清算导致取消。
    LiquidatedCanceled,
    /// 计划取消触发。
    ScheduledCancel,
    /// tick 价格无效导致拒绝。
    TickRejected,
    /// 订单名义价值低于最小值导致拒绝。
    MinTradeNtlRejected,
    /// perp 保证金不足导致拒绝。
    PerpMarginRejected,
    /// 只减仓语义导致拒绝。
    ReduceOnlyRejected,
    /// post-only 价格会立即成交导致拒绝。
    BadAloPxRejected,
    /// IOC 无法成交导致拒绝。
    IocCancelRejected,
    /// 触发价格无效导致拒绝。
    BadTriggerPxRejected,
    /// 市价单流动性不足导致拒绝。
    MarketOrderNoLiquidityRejected,
    /// 开仓量上限导致增仓拒绝。
    PositionIncreaseAtOpenInterestCapRejected,
    /// 开仓量上限导致翻仓拒绝。
    PositionFlipAtOpenInterestCapRejected,
    /// 价格过于激进且达到开仓量上限导致拒绝。
    TooAggressiveAtOpenInterestCapRejected,
    /// 开仓量增加被拒绝。
    OpenInterestIncreaseRejected,
    /// 现货余额不足导致拒绝。
    InsufficientSpotBalanceRejected,
    /// 价格偏离 oracle 太远导致拒绝。
    OracleRejected,
    /// 超过当前杠杆下最大仓位限制导致拒绝。
    PerpMaxPositionRejected,
}

impl SpotOrderStatusReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::CanceledByUser => "canceled",
            Self::Triggered => "triggered",
            Self::RejectedAtPlacement => "rejected",
            Self::MarginCanceled => "marginCanceled",
            Self::VaultWithdrawalCanceled => "vaultWithdrawalCanceled",
            Self::OpenInterestCapCanceled => "openInterestCapCanceled",
            Self::SelfTradeCanceled => "selfTradeCanceled",
            Self::ReduceOnlyCanceled => "reduceOnlyCanceled",
            Self::SiblingFilledCanceled => "siblingFilledCanceled",
            Self::DelistedCanceled => "delistedCanceled",
            Self::LiquidatedCanceled => "liquidatedCanceled",
            Self::ScheduledCancel => "scheduledCancel",
            Self::TickRejected => "tickRejected",
            Self::MinTradeNtlRejected => "minTradeNtlRejected",
            Self::PerpMarginRejected => "perpMarginRejected",
            Self::ReduceOnlyRejected => "reduceOnlyRejected",
            Self::BadAloPxRejected => "badAloPxRejected",
            Self::IocCancelRejected => "iocCancelRejected",
            Self::BadTriggerPxRejected => "badTriggerPxRejected",
            Self::MarketOrderNoLiquidityRejected => "marketOrderNoLiquidityRejected",
            Self::PositionIncreaseAtOpenInterestCapRejected => {
                "positionIncreaseAtOpenInterestCapRejected"
            }
            Self::PositionFlipAtOpenInterestCapRejected => "positionFlipAtOpenInterestCapRejected",
            Self::TooAggressiveAtOpenInterestCapRejected => {
                "tooAggressiveAtOpenInterestCapRejected"
            }
            Self::OpenInterestIncreaseRejected => "openInterestIncreaseRejected",
            Self::InsufficientSpotBalanceRejected => "insufficientSpotBalanceRejected",
            Self::OracleRejected => "oracleRejected",
            Self::PerpMaxPositionRejected => "perpMaxPositionRejected",
        }
    }
}

/// 已进入执行流程的 Hyperliquid 现货订单快照。
///
/// 立即单创建后直接成为 `SpotOrder`；条件单只有触发后才转换为 `SpotOrder`。
/// `qty`、`filled_qty`、`price`、`reserved_base`、`reserved_quote` 都使用 core
/// fixed-point 整数；adapter 负责和 Hyperliquid 字符串数字互转。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrder {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// Hyperliquid 现货资产编号，现货通常为 `10000 + spot index`。
    pub asset: u32,
    /// Hyperliquid 返回的 numeric `oid`；订单尚未被交易所确认时可以为空。
    pub exchange_oid: Option<u64>,
    /// 拥有该订单和冻结余额的交易账户 ID。
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
    /// 订单接受或触发后冻结的 base 余额，通常用于卖单。
    pub reserved_base: u64,
    /// 订单接受或触发后冻结的 quote 余额，通常用于买单。
    pub reserved_quote: u64,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
    /// 当前订单实体版本，用于生成可重放更新事件。
    pub version: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpotOrderFinalization {
    pub(crate) next_filled_qty: u64,
    pub(crate) status: SpotOrderStatus,
    pub(crate) status_reason: Option<SpotOrderStatusReason>,
}

impl SpotOrder {
    /// 从已经校验过的业务事实或回放事件构造 active 现货订单快照。
    ///
    /// 构造器不拒绝不一致字段，便于从历史事件恢复；业务一致性由实体查询方法暴露，
    /// use case 再映射为具体错误。
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
        reserved_base: u64,
        reserved_quote: u64,
        client_order_id: Option<String>,
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
            filled_qty: 0,
            status: SpotOrderStatus::Open,
            status_reason: None,
            reserved_base,
            reserved_quote,
            client_order_id,
            version: 1,
        }
    }

    /// 返回带交易所 `oid` 的订单快照。
    pub fn with_exchange_oid(mut self, exchange_oid: u64) -> Self {
        self.exchange_oid = Some(exchange_oid);
        self
    }

    /// 返回带指定执行状态的订单快照。
    pub fn with_execution_state(mut self, status: SpotOrderStatus, filled_qty: u64) -> Self {
        self.status = status;
        self.filled_qty = filled_qty;
        self
    }

    /// 返回带 Hyperliquid 细分状态原因的订单快照。
    pub fn with_status_reason(mut self, status_reason: SpotOrderStatusReason) -> Self {
        self.status_reason = Some(status_reason);
        self
    }

    /// 返回带指定实体版本的订单快照。
    pub fn with_version(mut self, version: u64) -> Self {
        self.version = version;
        self
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

    /// 返回需要提交给 Hyperliquid 的价格字段。
    pub fn order_price(&self) -> u64 {
        self.execution.order_price()
    }

    /// 返回订单 quote 名义价值。
    ///
    /// 市价意图没有稳定限价价格，或乘法溢出时返回 `None`。
    pub fn notional_quote(&self) -> Option<u64> {
        self.qty.checked_mul(self.limit_price()?)
    }

    /// 返回冻结 quote 所需使用的价格上限。
    pub fn reservation_quote(&self) -> Option<u64> {
        self.qty.checked_mul(self.order_price())
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

    /// 返回当前订单剩余可成交数量。
    pub(crate) fn remaining_qty(&self) -> Result<u64, MatchSpotOrderError> {
        self.qty.checked_sub(self.filled_qty).ok_or(MatchSpotOrderError::InconsistentOrderState)
    }

    /// 返回该订单是否会以当前价格和 maker 价格交叉成交。
    pub(crate) fn crosses_maker_price(&self, maker_price: u64) -> bool {
        match self.side {
            SpotOrderSide::Buy => self.order_price() >= maker_price,
            SpotOrderSide::Sell => self.order_price() <= maker_price,
        }
    }

    /// 返回该订单是否会和给定 maker 订单成交。
    pub(crate) fn crosses_order(&self, maker: &SpotOrder) -> Result<bool, MatchSpotOrderError> {
        if self.side == maker.side {
            return Err(MatchSpotOrderError::SameSideMaker);
        }

        let maker_price = maker.limit_price().ok_or(MatchSpotOrderError::MakerMustBeLimit)?;
        Ok(self.crosses_maker_price(maker_price))
    }

    /// 返回该订单是否会因 ALO 语义在进入撮合前被拒绝。
    pub(crate) fn would_be_rejected_as_alo(
        &self,
        best_maker: Option<&SpotOrder>,
    ) -> Result<bool, MatchSpotOrderError> {
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

    /// 返回本轮撮合结束后 taker 订单应进入的生命周期状态。
    pub(crate) fn finalize_after_match(
        &self,
        added_fill_qty: u64,
    ) -> Result<SpotOrderFinalization, MatchSpotOrderError> {
        let next_filled_qty = self
            .filled_qty
            .checked_add(added_fill_qty)
            .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;

        match self.time_in_force {
            SpotOrderTimeInForce::Gtc => {
                if added_fill_qty == 0 {
                    return Err(MatchSpotOrderError::NoTradesMatched);
                }

                Ok(SpotOrderFinalization {
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

                Ok(SpotOrderFinalization { next_filled_qty, status, status_reason })
            }
            SpotOrderTimeInForce::Alo => {
                if added_fill_qty == 0 {
                    return Err(MatchSpotOrderError::NoTradesMatched);
                }

                Ok(SpotOrderFinalization {
                    next_filled_qty,
                    status: self.matched_status_for(next_filled_qty),
                    status_reason: None,
                })
            }
        }
    }

    /// 返回冻结 quote 余额是否符合买卖方向。
    pub fn has_consistent_reserved_quote(&self) -> bool {
        match self.side {
            SpotOrderSide::Buy => self.reservation_quote() == Some(self.reserved_quote),
            SpotOrderSide::Sell => self.reserved_quote == 0,
        }
    }

    /// 返回冻结 base 余额是否符合买卖方向。
    pub fn has_consistent_reserved_base(&self) -> bool {
        match self.side {
            SpotOrderSide::Buy => self.reserved_base == 0,
            SpotOrderSide::Sell => self.reserved_base == self.qty,
        }
    }

    /// 返回撤单时应释放的 base 余额。
    pub fn base_to_release_on_cancel(&self) -> u64 {
        self.reserved_base
    }

    /// 返回撤单时应释放的 quote 余额。
    pub fn quote_to_release_on_cancel(&self) -> u64 {
        self.reserved_quote
    }

    fn no_liquidity_status_reason(&self) -> SpotOrderStatusReason {
        if self.limit_price().is_none() {
            SpotOrderStatusReason::MarketOrderNoLiquidityRejected
        } else {
            SpotOrderStatusReason::IocCancelRejected
        }
    }
}

/// 未触发的 Hyperliquid 条件现货订单。
///
/// 条件单只保存触发规则和触发后的执行意图；它不表示已进入订单簿的订单，也不冻结余额。
/// 条件满足后，用 `triggered_order` 转换成 active `SpotOrder`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotConditionalOrder {
    /// 本系统生成的稳定条件单 ID。
    pub trigger_order_id: String,
    /// Hyperliquid 现货资产编号，现货通常为 `10000 + spot index`。
    pub asset: u32,
    /// Hyperliquid 返回的 numeric `oid`；尚未确认时可以为空。
    pub exchange_oid: Option<u64>,
    /// 条件单所属账户。
    pub account_id: String,
    /// 交易对展示名，例如 `BTCUSDT`。
    pub symbol: String,
    /// 买卖方向。
    pub side: SpotOrderSide,
    /// 触发价格，使用 core fixed-point 整数价格。
    pub trigger_price: u64,
    /// 触发单业务角色。
    pub trigger_role: SpotOrderTriggerRole,
    /// 触发后的执行意图。
    pub execution: SpotOrderExecution,
    /// 触发后 active 订单使用的 TIF。市价意图通常为 `Ioc`。
    pub time_in_force: SpotOrderTimeInForce,
    /// 以 base asset 计价的下单数量。
    pub qty: u64,
    /// 条件单生命周期状态。
    pub status: SpotConditionalOrderStatus,
    /// Hyperliquid 细分状态原因。
    pub status_reason: Option<SpotOrderStatusReason>,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
}

impl SpotConditionalOrder {
    /// 从已经校验过的业务事实或回放事件构造条件单快照。
    pub fn new(
        trigger_order_id: String,
        asset: u32,
        exchange_oid: Option<u64>,
        account_id: String,
        symbol: String,
        side: SpotOrderSide,
        trigger_price: u64,
        trigger_role: SpotOrderTriggerRole,
        execution: SpotOrderExecution,
        time_in_force: SpotOrderTimeInForce,
        qty: u64,
        client_order_id: Option<String>,
    ) -> Self {
        Self {
            trigger_order_id,
            asset,
            exchange_oid,
            account_id,
            symbol,
            side,
            trigger_price,
            trigger_role,
            execution,
            time_in_force,
            qty,
            status: SpotConditionalOrderStatus::Open,
            status_reason: None,
            client_order_id,
        }
    }

    /// 返回带交易所 `oid` 的条件单快照。
    pub fn with_exchange_oid(mut self, exchange_oid: u64) -> Self {
        self.exchange_oid = Some(exchange_oid);
        self
    }

    /// 返回带指定条件单状态的快照。
    pub fn with_status(mut self, status: SpotConditionalOrderStatus) -> Self {
        self.status = status;
        self
    }

    /// 返回带 Hyperliquid 细分状态原因的条件单快照。
    pub fn with_status_reason(mut self, status_reason: SpotOrderStatusReason) -> Self {
        self.status_reason = Some(status_reason);
        self
    }

    /// 返回条件单是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回条件单是否交易指定 Hyperliquid asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.asset == asset
    }

    /// 返回条件单是否允许撤销。
    pub fn can_be_cancelled(&self) -> bool {
        self.status.is_cancelable()
    }

    /// 条件满足后生成 active 现货订单。
    pub fn triggered_order(
        &self,
        order_id: String,
        reserved_base: u64,
        reserved_quote: u64,
    ) -> SpotOrder {
        SpotOrder::new(
            order_id,
            self.asset,
            self.exchange_oid,
            self.account_id.clone(),
            self.symbol.clone(),
            self.side,
            self.execution,
            self.time_in_force,
            self.qty,
            reserved_base,
            reserved_quote,
            self.client_order_id.clone(),
        )
    }
}

impl Entity for SpotOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.order_id.clone()
    }

    fn entity_type() -> u8 {
        SPOT_ORDER_ENTITY_TYPE
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

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "order_id" | "account_id" | "symbol" | "side" | "execution" | "time_in_force"
            | "status" | "status_reason" | "client_order_id" => 0,
            "asset" | "exchange_oid" | "qty" | "filled_qty" | "price" | "reserved_base"
            | "reserved_quote" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_order_entity_id(&self.order_id))
    }
}

impl Entity for SpotConditionalOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.trigger_order_id.clone()
    }

    fn entity_type() -> u8 {
        SPOT_CONDITIONAL_ORDER_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("trigger_order_id", "", self.trigger_order_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("exchange_oid", "", option_u64_value(self.exchange_oid)),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("trigger_price", "", self.trigger_price.to_string()),
            EntityFieldChange::new("trigger_role", "", self.trigger_role.as_str()),
            EntityFieldChange::new("execution", "", self.execution.as_str()),
            EntityFieldChange::new("time_in_force", "", self.time_in_force.as_str()),
            EntityFieldChange::new("price", "", self.execution.order_price().to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            EntityFieldChange::new(
                "status_reason",
                "",
                option_status_reason_value(self.status_reason),
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
        push_change(
            &mut changes,
            "trigger_price",
            self.trigger_price.to_string(),
            other.trigger_price.to_string(),
        );
        push_change(
            &mut changes,
            "trigger_role",
            self.trigger_role.as_str(),
            other.trigger_role.as_str(),
        );
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
            self.execution.order_price().to_string(),
            other.execution.order_price().to_string(),
        );
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        push_change(
            &mut changes,
            "status_reason",
            option_status_reason_value(self.status_reason),
            option_status_reason_value(other.status_reason),
        );
        push_change(
            &mut changes,
            "client_order_id",
            self.client_order_id.clone().unwrap_or_default(),
            other.client_order_id.clone().unwrap_or_default(),
        );

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "trigger_order_id" | "account_id" | "symbol" | "side" | "trigger_role"
            | "execution" | "time_in_force" | "status" | "status_reason" | "client_order_id" => 0,
            "asset" | "exchange_oid" | "trigger_price" | "qty" | "price" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_order_entity_id(&self.trigger_order_id))
    }
}

fn stable_order_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

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

    fn active_buy_order() -> SpotOrder {
        SpotOrder::new(
            "order-1".to_string(),
            10_001,
            None,
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            SpotOrderTimeInForce::Gtc,
            2,
            0,
            200,
            Some("0123456789abcdef0123456789abcdef".to_string()),
        )
    }

    fn conditional_sell_order() -> SpotConditionalOrder {
        SpotConditionalOrder::new(
            "trigger-1".to_string(),
            10_001,
            None,
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            90,
            SpotOrderTriggerRole::StopLoss,
            SpotOrderExecution::Market { aggressive_price: 95 },
            SpotOrderTimeInForce::Ioc,
            2,
            None,
        )
    }

    #[test]
    fn active_order_stores_hyperliquid_spot_identity_fields() {
        let order = active_buy_order().with_exchange_oid(42);

        assert_eq!(order.asset, 10_001);
        assert_eq!(order.exchange_oid, Some(42));
        assert!(order.trades_asset(10_001));
        assert!(!order.trades_asset(10_002));
        assert_eq!(order.client_order_id.as_deref(), Some("0123456789abcdef0123456789abcdef"));
    }

    #[test]
    fn active_order_validates_reserved_buy_limit_order() {
        let order = active_buy_order();

        assert_eq!(order.notional_quote(), Some(200));
        assert_eq!(order.reservation_quote(), Some(200));
        assert!(order.has_consistent_reserved_quote());
        assert!(order.has_consistent_reserved_base());
        assert_eq!(order.quote_to_release_on_cancel(), 200);
    }

    #[test]
    fn conditional_order_does_not_freeze_before_trigger() {
        let order = conditional_sell_order();

        assert_eq!(order.status, SpotConditionalOrderStatus::Open);
        assert!(order.can_be_cancelled());
        assert_eq!(order.trigger_price, 90);
        assert_eq!(order.execution.limit_price(), None);
        assert_eq!(order.execution.order_price(), 95);
    }

    #[test]
    fn conditional_order_triggers_active_order() {
        let conditional = conditional_sell_order().with_exchange_oid(77);
        let active = conditional.triggered_order("order-2".to_string(), 2, 0);

        assert_eq!(active.order_id, "order-2");
        assert_eq!(active.asset, conditional.asset);
        assert_eq!(active.exchange_oid, Some(77));
        assert_eq!(active.side, SpotOrderSide::Sell);
        assert_eq!(active.execution, conditional.execution);
        assert_eq!(active.time_in_force, SpotOrderTimeInForce::Ioc);
        assert_eq!(active.reserved_base, 2);
        assert_eq!(active.reserved_quote, 0);
        assert!(active.has_consistent_reserved_base());
        assert!(active.has_consistent_reserved_quote());
    }

    #[test]
    fn active_order_checks_execution_state_consistency() {
        let open = active_buy_order();
        let partial = active_buy_order().with_execution_state(SpotOrderStatus::PartiallyFilled, 1);
        let filled = active_buy_order().with_execution_state(SpotOrderStatus::Filled, 2);
        let inconsistent_filled =
            active_buy_order().with_execution_state(SpotOrderStatus::Filled, 1);

        assert!(open.has_consistent_execution_state());
        assert!(partial.has_consistent_execution_state());
        assert!(filled.has_consistent_execution_state());
        assert!(!inconsistent_filled.has_consistent_execution_state());
    }

    #[test]
    fn created_events_contain_spot_order_fields() {
        let active = active_buy_order()
            .with_exchange_oid(42)
            .with_status_reason(SpotOrderStatusReason::CanceledByUser);
        let active_changes = active.created_field_changes();

        assert!(
            active_changes
                .iter()
                .any(|change| { change.field_name == "asset" && change.new_value == "10001" })
        );
        assert!(
            active_changes
                .iter()
                .any(|change| { change.field_name == "exchange_oid" && change.new_value == "42" })
        );
        assert!(active_changes.iter().any(|change| {
            change.field_name == "status_reason" && change.new_value == "canceled"
        }));

        let conditional = conditional_sell_order()
            .with_status(SpotConditionalOrderStatus::Triggered)
            .with_status_reason(SpotOrderStatusReason::Triggered);
        let conditional_changes = conditional.created_field_changes();

        assert!(
            conditional_changes
                .iter()
                .any(|change| { change.field_name == "trigger_price" && change.new_value == "90" })
        );
        assert!(
            conditional_changes
                .iter()
                .any(|change| { change.field_name == "status" && change.new_value == "triggered" })
        );
    }
}
