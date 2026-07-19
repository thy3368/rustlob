use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff};

const HYPERLIQUID_PERP_INSURANCE_FUND_ALLOCATION_ENTITY_TYPE: u8 = 16;

/// 一次保险基金对强平缺口进行覆盖的 append-only 审计事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpInsuranceFundAllocation {
    pub insurance_fund_allocation_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub asset: u32,
    pub allocated_quote: u64,
    pub remaining_shortfall_quote_after: u64,
    pub insurance_fund_account_id: String,
}

impl HyperliquidPerpInsuranceFundAllocation {
    pub fn new(
        insurance_fund_allocation_id: String,
        liquidation_id: String,
        shortfall_id: String,
        asset: u32,
        allocated_quote: u64,
        remaining_shortfall_quote_after: u64,
        insurance_fund_account_id: String,
    ) -> Self {
        Self {
            insurance_fund_allocation_id,
            liquidation_id,
            shortfall_id,
            asset,
            allocated_quote,
            remaining_shortfall_quote_after,
            insurance_fund_account_id,
        }
    }
}

impl FieldDiff for HyperliquidPerpInsuranceFundAllocation {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new(
                "insurance_fund_allocation_id",
                "",
                self.insurance_fund_allocation_id.clone(),
            ),
            EntityFieldChange::new("liquidation_id", "", self.liquidation_id.clone()),
            EntityFieldChange::new("shortfall_id", "", self.shortfall_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("allocated_quote", "", self.allocated_quote.to_string()),
            EntityFieldChange::new(
                "remaining_shortfall_quote_after",
                "",
                self.remaining_shortfall_quote_after.to_string(),
            ),
            EntityFieldChange::new(
                "insurance_fund_account_id",
                "",
                self.insurance_fund_account_id.clone(),
            ),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Entity for HyperliquidPerpInsuranceFundAllocation {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.insurance_fund_allocation_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_INSURANCE_FUND_ALLOCATION_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "insurance_fund_allocation_id"
            | "liquidation_id"
            | "shortfall_id"
            | "insurance_fund_account_id" => 0,
            "asset" | "allocated_quote" | "remaining_shortfall_quote_after" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.insurance_fund_allocation_id))
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

    #[test]
    fn create_event_contains_allocation_fields() {
        let allocation = HyperliquidPerpInsuranceFundAllocation::new(
            "ifa-1".to_string(),
            "liq-1".to_string(),
            "shortfall-1".to_string(),
            7,
            1_000,
            250,
            "insurance-fund".to_string(),
        );

        let event = allocation.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("insurance_fund_account_id")
                && change.new_value_bytes() == b"insurance-fund"
        }));
    }
}
