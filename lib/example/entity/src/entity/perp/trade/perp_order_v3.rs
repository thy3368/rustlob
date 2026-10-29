use crate::entity::common::common_order::OrderBase;

/// 永续订单实体，共享订单价格、数量与触发价的语义单位。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpOrderV3 {
    pub comon_order: OrderBase,
    /// 是否仅允许减少既有敞口。实体保存意图，不在订单内模拟仓位。 perp特有
    pub reduce_only: bool,
    /// 是否是强平流程发出的风险订单。
    pub is_liquidation: bool,
}
