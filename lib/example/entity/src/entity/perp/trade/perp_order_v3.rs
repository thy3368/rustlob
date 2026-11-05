use common_entity::{Entity, EntityFieldChange, FieldDiff, Shard};

use crate::entity::common::common_order::OrderBase;

const PERP_ORDER_V3_ENTITY_TYPE: u8 = 9;

/// 永续订单实体，共享订单价格、数量与触发价的语义单位。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpOrderV3 {
    pub comon_order: OrderBase,
    /// 是否仅允许减少既有敞口。实体保存意图，不在订单内模拟仓位。 perp特有
    pub reduce_only: bool,
    /// 是否是强平流程发出的风险订单。
    pub is_liquidation: bool,
}

impl FieldDiff for PerpOrderV3 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        fields(self, "")
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        values(self)
            .into_iter()
            .zip(values(other))
            .filter(|((_, before), (_, after))| before != after)
            .map(|((name, before), (_, after))| EntityFieldChange::new(name, before, after))
            .collect()
    }
}

fn fields(order: &PerpOrderV3, before: &str) -> Vec<EntityFieldChange> {
    values(order)
        .into_iter()
        .map(|(name, after)| EntityFieldChange::new(name, before, after))
        .collect()
}

fn values(order: &PerpOrderV3) -> Vec<(&'static str, String)> {
    let base = &order.comon_order;
    vec![
        ("owner", base.owner.0.to_string()),
        ("symbol", base.symbol.id().to_string()),
        ("side", format!("{:?}", base.side)),
        ("limit_px", base.limit_px.to_string()),
        ("size", base.size.to_string()),
        ("orig_size", base.orig_size.to_string()),
        ("status", format!("{:?}", base.status)),
        ("reduce_only", order.reduce_only.to_string()),
        ("is_liquidation", order.is_liquidation.to_string()),
        ("created_at", base.created_at.to_string()),
        ("updated_at", base.updated_at.to_string()),
    ]
}

impl Shard for PerpOrderV3 {
    fn partition(&self) -> u8 {
        (self.comon_order.owner.0 % 256) as u8
    }

    fn party_id(&self) -> u64 {
        self.comon_order.owner.0
    }
}

impl Entity for PerpOrderV3 {
    fn entity_id(&self) -> u64 {
        self.comon_order.order_id
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8 {
        PERP_ORDER_V3_ENTITY_TYPE
    }

    fn lsn(&self) -> u64 {
        self.comon_order.version
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "limit_px" | "size" | "orig_size" => 1,
            _ => 0,
        }
    }
}
