use common_entity::EntityFieldChange;
use serde::{Deserialize, Serialize};

/// 现货订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// 买入订单。
    Buy,
    /// 卖出订单。
    Sell,
}

impl OrderSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// Hyperliquid 现货限价单的有效方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderTif {
    Gtc,
    Ioc,
    Alo,
}

impl OrderTif {
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
pub enum OrderTriggerRole {
    /// 止盈触发单。
    TakeProfit,
    /// 止损触发单。
    StopLoss,
}

/// 现货订单在 `normalTpsl` 中承担的关系角色。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OrderGroupRelation {
    /// 不属于父子订单组。
    #[default]
    Standalone,
    /// `normalTpsl` entry 父单。
    NormalTpslParent,
    /// `normalTpsl` TP/SL 子单及其父单 Hyperliquid spot `oid`。
    NormalTpslChild { parent_order_id: u64 },
}

/// 现货成交中的手续费角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeFeeRole {
    Maker,
    Taker,
}

impl TradeFeeRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Maker => "maker",
            Self::Taker => "taker",
        }
    }
}

/// Hyperliquid 现货订单类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// 普通限价单；所谓市价意图由 IOC 加激进限价表达。
    Limit { tif: OrderTif },
    /// 条件单；触发后进入 GTC 限价或 IOC 激进限价生命周期。
    Trigger { is_market: bool, trigger_price: u64, tpsl: OrderTriggerRole },
}

impl OrderType {
    pub const fn effective_tif(self) -> OrderTif {
        match self {
            Self::Limit { tif } => tif,
            Self::Trigger { is_market: true, .. } => OrderTif::Ioc,
            Self::Trigger { is_market: false, .. } => OrderTif::Gtc,
        }
    }

    pub const fn is_trigger(self) -> bool {
        matches!(self, Self::Trigger { .. })
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Limit { .. } => "limit",
            Self::Trigger { is_market: true, .. } => "trigger_market",
            Self::Trigger { is_market: false, .. } => "trigger_limit",
        }
    }
}

impl OrderTriggerRole {
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
    /// 条件单已接受，等待触发；尚未进入执行流程。
    Pending,
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
            Self::Pending => "pending",
            Self::Open => "open",
            Self::PartiallyFilled => "partially_filled",
            Self::Filled => "filled",
            Self::Canceled => "canceled",
            Self::Rejected => "rejected",
        }
    }

    pub const fn is_cancelable(self) -> bool {
        matches!(self, Self::Pending | Self::Open | Self::PartiallyFilled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_status_uses_public_pending_label_and_remains_cancelable() {
        assert_eq!(SpotOrderStatus::Pending.as_str(), "pending");
        assert!(SpotOrderStatus::Pending.is_cancelable());
    }
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

