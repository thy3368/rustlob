use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff};

const HYPERLIQUID_PERP_ADL_DELEVERAGING_RECORD_ENTITY_TYPE: u8 = 19;

/// 单个对手方仓位被 ADL 去杠杆后的 append-only 结果凭证。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpAdlDeleveragingRecord {
    pub adl_deleveraging_record_id: String,
    pub adl_execution_id: String,
    pub adl_batch_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub deleveraged_account_id: String,
    pub deleveraged_position_id: String,
    pub asset: u32,
    pub symbol: String,
    pub qty: u64,
    pub execution_price: u64,
    pub covered_quote: u64,
}

impl HyperliquidPerpAdlDeleveragingRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adl_deleveraging_record_id: String,
        adl_execution_id: String,
        adl_batch_id: String,
        liquidation_id: String,
        shortfall_id: String,
        deleveraged_account_id: String,
        deleveraged_position_id: String,
        asset: u32,
        symbol: String,
        qty: u64,
        execution_price: u64,
        covered_quote: u64,
    ) -> Self {
        Self {
            adl_deleveraging_record_id,
            adl_execution_id,
            adl_batch_id,
            liquidation_id,
            shortfall_id,
            deleveraged_account_id,
            deleveraged_position_id,
            asset,
            symbol,
            qty,
            execution_price,
            covered_quote,
        }
    }
}

impl FieldDiff for HyperliquidPerpAdlDeleveragingRecord {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new(
                "adl_deleveraging_record_id",
                "",
                self.adl_deleveraging_record_id.clone(),
            ),
            EntityFieldChange::new("adl_execution_id", "", self.adl_execution_id.clone()),
            EntityFieldChange::new("adl_batch_id", "", self.adl_batch_id.clone()),
            EntityFieldChange::new("liquidation_id", "", self.liquidation_id.clone()),
            EntityFieldChange::new("shortfall_id", "", self.shortfall_id.clone()),
            EntityFieldChange::new(
                "deleveraged_account_id",
                "",
                self.deleveraged_account_id.clone(),
            ),
            EntityFieldChange::new(
                "deleveraged_position_id",
                "",
                self.deleveraged_position_id.clone(),
            ),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("execution_price", "", self.execution_price.to_string()),
            EntityFieldChange::new("covered_quote", "", self.covered_quote.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Entity for HyperliquidPerpAdlDeleveragingRecord {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.adl_deleveraging_record_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_ADL_DELEVERAGING_RECORD_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "adl_deleveraging_record_id"
            | "adl_execution_id"
            | "adl_batch_id"
            | "liquidation_id"
            | "shortfall_id"
            | "deleveraged_account_id"
            | "deleveraged_position_id"
            | "symbol" => 0,
            "asset" | "qty" | "execution_price" | "covered_quote" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.adl_deleveraging_record_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn record() -> HyperliquidPerpAdlDeleveragingRecord {
        HyperliquidPerpAdlDeleveragingRecord::new(
            "adl-record-1".to_string(),
            "adl-exec-1".to_string(),
            "adl-batch-1".to_string(),
            "liq-1".to_string(),
            "shortfall-1".to_string(),
            "profitable-trader".to_string(),
            "position-9".to_string(),
            7,
            "BTC-PERP".to_string(),
            1,
            50_100,
            50_100,
        )
    }

    #[test]
    fn create_event_contains_adl_record_fields() {
        let event = record().track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("adl_deleveraging_record_id")
                && change.new_value_bytes() == b"adl-record-1"
        }));
    }

    #[test]
    fn record_has_no_update_diff() {
        let record = record();

        assert!(record.diff(&record).is_empty());
    }
}
