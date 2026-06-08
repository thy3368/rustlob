use common_entity::{Entity, EntityError, EntityFieldChange};

const SPOT_SETTLEMENT_ENTITY_TYPE: u8 = 6;

/// 一笔现货成交完成资产清结算后的事实记录。
///
/// `SpotSettlement` 表达 trade 已经完成 base/quote 资产交割，不包含手续费、
/// 返佣或税费。构造器假定输入已经由清结算 use case 校验。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotSettlement {
    /// 本系统稳定清算记录 ID。
    pub settlement_id: String,
    /// 被清结算的成交 ID。
    pub trade_id: String,
    /// 成交所属撮合批次 ID。
    pub match_id: String,
    /// 买方账户 ID。
    pub buyer_account_id: String,
    /// 卖方账户 ID。
    pub seller_account_id: String,
    /// 已交割 base 数量。
    pub base_qty: u64,
    /// 已交割 quote 数量。
    pub quote_qty: u64,
    /// 成交价格。
    pub price: u64,
}

impl SpotSettlement {
    /// 从已经校验过的清结算事实构造记录。
    pub fn new(
        settlement_id: String,
        trade_id: String,
        match_id: String,
        buyer_account_id: String,
        seller_account_id: String,
        base_qty: u64,
        quote_qty: u64,
        price: u64,
    ) -> Self {
        Self {
            settlement_id,
            trade_id,
            match_id,
            buyer_account_id,
            seller_account_id,
            base_qty,
            quote_qty,
            price,
        }
    }
}

impl Entity for SpotSettlement {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.settlement_id.clone()
    }

    fn entity_type() -> u8 {
        SPOT_SETTLEMENT_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("settlement_id", "", self.settlement_id.clone()),
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone()),
            EntityFieldChange::new("buyer_account_id", "", self.buyer_account_id.clone()),
            EntityFieldChange::new("seller_account_id", "", self.seller_account_id.clone()),
            EntityFieldChange::new("base_qty", "", self.base_qty.to_string()),
            EntityFieldChange::new("quote_qty", "", self.quote_qty.to_string()),
            EntityFieldChange::new("price", "", self.price.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "settlement_id" | "trade_id" | "match_id" | "buyer_account_id"
            | "seller_account_id" => 0,
            "base_qty" | "quote_qty" | "price" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_settlement_entity_id(&self.settlement_id))
    }
}

fn stable_settlement_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn settlement() -> SpotSettlement {
        SpotSettlement::new(
            "settle-1-1".to_string(),
            "trade-1".to_string(),
            "match-1".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            2,
            200,
            100,
        )
    }

    #[test]
    fn constructor_stores_settlement_facts() {
        let settlement = settlement();

        assert_eq!(settlement.settlement_id, "settle-1-1");
        assert_eq!(settlement.trade_id, "trade-1");
        assert_eq!(settlement.base_qty, 2);
        assert_eq!(settlement.quote_qty, 200);
    }

    #[test]
    fn create_event_contains_settlement_fields() {
        let event = settlement().track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("settlement_id")
                && change.new_value_bytes() == b"settle-1-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("quote_qty")
                && change.new_value_bytes() == b"200"
        }));
    }
}
