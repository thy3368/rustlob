use common_entity::{Entity, EntityError, EntityFieldChange};

const HYPERLIQUID_PERP_SHORTFALL_ENTITY_TYPE: u8 = 17;

/// 强平执行结束后仍然存在的缺口业务事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpShortfallStatus {
    Open,
    PartiallyCovered,
    Covered,
    Exhausted,
}

impl HyperliquidPerpShortfallStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::PartiallyCovered => "partially_covered",
            Self::Covered => "covered",
            Self::Exhausted => "exhausted",
        }
    }
}

/// 独立可审计的 Hyperliquid perp 强平缺口实体。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpShortfall {
    pub shortfall_id: String,
    pub liquidation_id: String,
    pub account_id: String,
    pub position_id: String,
    pub asset: u32,
    pub symbol: String,
    pub bankruptcy_price: u64,
    pub required_quote: u64,
    pub recovered_quote: u64,
    pub shortfall_quote: u64,
    pub covered_by_insurance_quote: u64,
    pub covered_by_adl_quote: u64,
    pub remaining_quote: u64,
    pub status: HyperliquidPerpShortfallStatus,
    pub version: u64,
}

impl HyperliquidPerpShortfall {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        shortfall_id: String,
        liquidation_id: String,
        account_id: String,
        position_id: String,
        asset: u32,
        symbol: String,
        bankruptcy_price: u64,
        required_quote: u64,
        recovered_quote: u64,
        shortfall_quote: u64,
    ) -> Self {
        Self {
            shortfall_id,
            liquidation_id,
            account_id,
            position_id,
            asset,
            symbol,
            bankruptcy_price,
            required_quote,
            recovered_quote,
            shortfall_quote,
            covered_by_insurance_quote: 0,
            covered_by_adl_quote: 0,
            remaining_quote: shortfall_quote,
            status: HyperliquidPerpShortfallStatus::Open,
            version: 1,
        }
    }

    pub fn has_remaining_quote(&self) -> bool {
        self.remaining_quote > 0
    }

    pub fn apply_insurance_coverage(&mut self, allocated_quote: u64, version: u64) -> Option<()> {
        if allocated_quote == 0
            || !matches!(
                self.status,
                HyperliquidPerpShortfallStatus::Open | HyperliquidPerpShortfallStatus::PartiallyCovered
            )
            || allocated_quote > self.remaining_quote
        {
            return None;
        }

        self.covered_by_insurance_quote =
            self.covered_by_insurance_quote.checked_add(allocated_quote)?;
        self.remaining_quote = self.remaining_quote.checked_sub(allocated_quote)?;
        self.status = if self.remaining_quote == 0 {
            HyperliquidPerpShortfallStatus::Covered
        } else {
            HyperliquidPerpShortfallStatus::PartiallyCovered
        };
        self.version = version;
        Some(())
    }

    pub fn apply_adl_coverage(&mut self, covered_quote: u64, version: u64) -> Option<()> {
        if covered_quote == 0
            || !matches!(
                self.status,
                HyperliquidPerpShortfallStatus::Open | HyperliquidPerpShortfallStatus::PartiallyCovered
            )
            || covered_quote > self.remaining_quote
        {
            return None;
        }

        self.covered_by_adl_quote = self.covered_by_adl_quote.checked_add(covered_quote)?;
        self.remaining_quote = self.remaining_quote.checked_sub(covered_quote)?;
        self.status = if self.remaining_quote == 0 {
            HyperliquidPerpShortfallStatus::Covered
        } else {
            HyperliquidPerpShortfallStatus::PartiallyCovered
        };
        self.version = version;
        Some(())
    }

    pub fn apply_exhausted(&mut self, version: u64) -> Option<()> {
        if self.remaining_quote == 0
            || !matches!(
                self.status,
                HyperliquidPerpShortfallStatus::Open | HyperliquidPerpShortfallStatus::PartiallyCovered
            )
        {
            return None;
        }

        self.status = HyperliquidPerpShortfallStatus::Exhausted;
        self.version = version;
        Some(())
    }
}

impl Entity for HyperliquidPerpShortfall {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.shortfall_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_SHORTFALL_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("shortfall_id", "", self.shortfall_id.clone()),
            EntityFieldChange::new("liquidation_id", "", self.liquidation_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("position_id", "", self.position_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("bankruptcy_price", "", self.bankruptcy_price.to_string()),
            EntityFieldChange::new("required_quote", "", self.required_quote.to_string()),
            EntityFieldChange::new("recovered_quote", "", self.recovered_quote.to_string()),
            EntityFieldChange::new("shortfall_quote", "", self.shortfall_quote.to_string()),
            EntityFieldChange::new(
                "covered_by_insurance_quote",
                "",
                self.covered_by_insurance_quote.to_string(),
            ),
            EntityFieldChange::new(
                "covered_by_adl_quote",
                "",
                self.covered_by_adl_quote.to_string(),
            ),
            EntityFieldChange::new("remaining_quote", "", self.remaining_quote.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        push_change(&mut changes, "covered_by_insurance_quote", self.covered_by_insurance_quote, other.covered_by_insurance_quote);
        push_change(&mut changes, "covered_by_adl_quote", self.covered_by_adl_quote, other.covered_by_adl_quote);
        push_change(&mut changes, "remaining_quote", self.remaining_quote, other.remaining_quote);
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "shortfall_id" | "liquidation_id" | "account_id" | "position_id" | "symbol"
            | "status" => 0,
            "asset"
            | "bankruptcy_price"
            | "required_quote"
            | "recovered_quote"
            | "shortfall_quote"
            | "covered_by_insurance_quote"
            | "covered_by_adl_quote"
            | "remaining_quote" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.shortfall_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn push_change(
    changes: &mut Vec<EntityFieldChange>,
    field_name: &'static str,
    old_value: impl ToString,
    new_value: impl ToString,
) {
    let old_value = old_value.to_string();
    let new_value = new_value.to_string();
    if old_value != new_value {
        changes.push(EntityFieldChange::new(field_name, old_value, new_value));
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn shortfall() -> HyperliquidPerpShortfall {
        HyperliquidPerpShortfall::new(
            "shortfall-1".to_string(),
            "liq-1".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            50_000,
            100_000,
            70_000,
            30_000,
        )
    }

    #[test]
    fn create_event_contains_shortfall_fields() {
        let event = shortfall().track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("shortfall_quote")
                && change.new_value_bytes() == b"30000"
        }));
    }

    #[test]
    fn diff_contains_only_mutable_business_fields() {
        let before = shortfall();
        let mut after = before.clone();
        after.apply_insurance_coverage(10_000, 2).unwrap();

        let diff = before.diff(&after);
        assert_eq!(diff.len(), 3);
        assert!(diff.iter().any(|change| change.field_name_as_str().ok() == Some("covered_by_insurance_quote")));
        assert!(diff.iter().any(|change| change.field_name_as_str().ok() == Some("remaining_quote")));
        assert!(diff.iter().any(|change| change.field_name_as_str().ok() == Some("status")));
    }

    #[test]
    fn apply_exhausted_rejects_after_being_fully_covered() {
        let mut shortfall = shortfall();
        shortfall.apply_insurance_coverage(30_000, 2).unwrap();

        assert_eq!(shortfall.apply_exhausted(3), None);
    }
}
