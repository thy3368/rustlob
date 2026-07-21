use std::hash::{Hash, Hasher};

use common_entity::EntityFieldChange;
use serde::{Deserialize, Serialize};

/// 现货订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// 条件触发订单角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Hyperliquid 订单状态原因。
///
/// 该枚举对齐 `orderStatus` 返回的细分状态；其中部分状态只会出现在 perp，
/// 但保留枚举值可以让 adapter 无损回放交易所状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

pub(crate) fn stable_order_entity_id(value: &str) -> i64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

pub(crate) fn push_change(
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

pub(crate) fn option_u64_value(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

pub(crate) fn option_status_reason_value(value: Option<SpotOrderStatusReason>) -> &'static str {
    value.map(SpotOrderStatusReason::as_str).unwrap_or_default()
}
