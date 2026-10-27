use base_types::UserId;
use serde::{Deserialize, Serialize};

use crate::{OrderGroupRelation, OrderSide, OrderTif, SpotOrderStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommonOrderExecution {
    Limit,
    Market,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CommonOrderTrigger {
    #[default]
    None,
    TakeProfit,
    StopLoss,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderBase {
    pub owner: UserId,

    /// Hyperliquid 原生现货订单 `oid`。
    ///
    /// 字段名保留 `order_id` 以兼容本仓库内实体接口语义，但业务含义等同
    /// Hyperliquid numeric spot `oid`。
    pub order_id: u64,
    /// Hyperliquid 现货资产编号，现货通常为 `10000 + spot index`。
    pub asset: u32,

    /// 交易对展示名，例如 `BTCUSDT`。业务身份以 `asset` 为准。
    pub pair: String,
    /// 买卖方向。
    pub side: OrderSide,
    /// Hyperliquid `p` 字段；market 意图同样保存其激进限价。
    pub limit_px: u64,
    /// 以 base asset 计价的可撮合数量。
    pub size: u64,
    /// 下单数量。
    pub orig_size: u64,
    /// 订单如何执行。
    pub execution: CommonOrderExecution,
    /// 限价单有效方式；市价单固定按 IOC 执行。
    pub tif: OrderTif,
    /// 订单何时触发。
    pub trigger: CommonOrderTrigger,
    /// TP/SL 触发价格；普通订单必须为空。
    pub trigger_px: Option<u64>,
    /// `normalTpsl` 父子关系；历史订单反序列化时默认为独立订单。
    pub group_relation: OrderGroupRelation,
    /// 是否为仓单
    pub is_position_tpsl: bool,
    /// 是否仅允许减少既有敞口。实体保存意图，不在订单内模拟仓位。 perp特有
    pub reduce_only: bool,
    /// 本地生命周期状态。
    pub status: SpotOrderStatus,
    /// 冻结费 spot 特有
    pub taker_max_fee: u64,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub cloid: Option<String>,
    /// 订单创建时间，单位为 Unix 纳秒。
    pub created_at: u64,
    /// 订单最后更新时间，单位为 Unix 纳秒。
    pub updated_at: u64,
    /// 当前订单实体版本，用于生成可重放更新事件。
    pub version: u64,
}

impl OrderBase {
    pub fn effective_tif(&self) -> OrderTif {
        match self.execution {
            CommonOrderExecution::Market => OrderTif::Ioc,
            CommonOrderExecution::Limit => self.tif,
        }
    }
    pub fn is_trigger(&self) -> bool {
        self.trigger != CommonOrderTrigger::None
    }
    pub fn trigger_role(&self) -> Option<crate::OrderTriggerRole> {
        match self.trigger {
            CommonOrderTrigger::None => None,
            CommonOrderTrigger::TakeProfit => Some(crate::OrderTriggerRole::TakeProfit),
            CommonOrderTrigger::StopLoss => Some(crate::OrderTriggerRole::StopLoss),
        }
    }
    pub fn has_consistent_execution_facts(&self) -> bool {
        let trigger_is_valid = match self.trigger {
            CommonOrderTrigger::None => self.trigger_px.is_none(),
            CommonOrderTrigger::TakeProfit | CommonOrderTrigger::StopLoss => {
                self.trigger_px.is_some_and(|px| px > 0)
            }
        };
        let group_is_valid = match &self.group_relation {
            OrderGroupRelation::Standalone => {
                !self.is_position_tpsl || self.trigger != CommonOrderTrigger::None
            }
            OrderGroupRelation::NormalTpslParent => {
                !self.is_position_tpsl && self.trigger == CommonOrderTrigger::None
            }
            OrderGroupRelation::NormalTpslChild { .. } => {
                !self.is_position_tpsl && self.trigger != CommonOrderTrigger::None
            }
        };
        trigger_is_valid
            && group_is_valid
            && (self.execution != CommonOrderExecution::Market || self.limit_px > 0)
    }
}
