use common_entity::{Entity, EntityError, EntityFieldChange};

const HYPERLIQUID_PERP_ADL_BATCH_ENTITY_TYPE: u8 = 18;

/// 一次 ADL 覆盖批次的业务状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpAdlBatchStatus {
    Open,
    PartiallyCovered,
    Covered,
    Exhausted,
}

impl HyperliquidPerpAdlBatchStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::PartiallyCovered => "partially_covered",
            Self::Covered => "covered",
            Self::Exhausted => "exhausted",
        }
    }
}

/// 缺口进入 ADL 覆盖后的批次级业务实体。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpAdlBatch {
    pub adl_batch_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub asset: u32,
    pub symbol: String,
    pub target_cover_quote: u64,
    pub covered_quote: u64,
    pub remaining_quote: u64,
    pub entry_count: u64,
    pub status: HyperliquidPerpAdlBatchStatus,
    pub version: u64,
}

impl HyperliquidPerpAdlBatch {
    pub fn new(
        adl_batch_id: String,
        liquidation_id: String,
        shortfall_id: String,
        asset: u32,
        symbol: String,
        target_cover_quote: u64,
    ) -> Self {
        Self {
            adl_batch_id,
            liquidation_id,
            shortfall_id,
            asset,
            symbol,
            target_cover_quote,
            covered_quote: 0,
            remaining_quote: target_cover_quote,
            entry_count: 0,
            status: HyperliquidPerpAdlBatchStatus::Open,
            version: 1,
        }
    }

    pub fn has_remaining_quote(&self) -> bool {
        self.remaining_quote > 0
    }

    pub fn apply_entry(&mut self, covered_quote: u64, version: u64) -> Option<()> {
        if covered_quote == 0
            || !matches!(
                self.status,
                HyperliquidPerpAdlBatchStatus::Open
                    | HyperliquidPerpAdlBatchStatus::PartiallyCovered
            )
            || covered_quote > self.remaining_quote
        {
            return None;
        }

        self.covered_quote = self.covered_quote.checked_add(covered_quote)?;
        self.remaining_quote = self.remaining_quote.checked_sub(covered_quote)?;
        self.entry_count = self.entry_count.checked_add(1)?;
        self.status = if self.remaining_quote == 0 {
            HyperliquidPerpAdlBatchStatus::Covered
        } else {
            HyperliquidPerpAdlBatchStatus::PartiallyCovered
        };
        self.version = version;
        Some(())
    }

    pub fn apply_exhausted(&mut self, version: u64) -> Option<()> {
        if self.remaining_quote == 0
            || !matches!(
                self.status,
                HyperliquidPerpAdlBatchStatus::Open
                    | HyperliquidPerpAdlBatchStatus::PartiallyCovered
            )
        {
            return None;
        }

        self.status = HyperliquidPerpAdlBatchStatus::Exhausted;
        self.version = version;
        Some(())
    }
}

impl Entity for HyperliquidPerpAdlBatch {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.adl_batch_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_ADL_BATCH_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("adl_batch_id", "", self.adl_batch_id.clone()),
            EntityFieldChange::new("liquidation_id", "", self.liquidation_id.clone()),
            EntityFieldChange::new("shortfall_id", "", self.shortfall_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("target_cover_quote", "", self.target_cover_quote.to_string()),
            EntityFieldChange::new("covered_quote", "", self.covered_quote.to_string()),
            EntityFieldChange::new("remaining_quote", "", self.remaining_quote.to_string()),
            EntityFieldChange::new("entry_count", "", self.entry_count.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        push_change(&mut changes, "covered_quote", self.covered_quote, other.covered_quote);
        push_change(&mut changes, "remaining_quote", self.remaining_quote, other.remaining_quote);
        push_change(&mut changes, "entry_count", self.entry_count, other.entry_count);
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "adl_batch_id" | "liquidation_id" | "shortfall_id" | "symbol" | "status" => 0,
            "asset" | "target_cover_quote" | "covered_quote" | "remaining_quote"
            | "entry_count" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.adl_batch_id))
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

    fn batch() -> HyperliquidPerpAdlBatch {
        HyperliquidPerpAdlBatch::new(
            "adl-batch-1".to_string(),
            "liq-1".to_string(),
            "shortfall-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            30_000,
        )
    }

    #[test]
    fn create_event_contains_batch_fields() {
        let event = batch().track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("target_cover_quote")
                && change.new_value_bytes() == b"30000"
        }));
    }

    #[test]
    fn diff_contains_only_mutable_business_fields() {
        let before = batch();
        let mut after = before.clone();
        after.apply_entry(10_000, 2).unwrap();

        let diff = before.diff(&after);
        assert_eq!(diff.len(), 4);
        assert!(diff.iter().any(|change| change.field_name == "covered_quote"));
        assert!(diff.iter().any(|change| change.field_name == "remaining_quote"));
        assert!(diff.iter().any(|change| change.field_name == "entry_count"));
        assert!(diff.iter().any(|change| change.field_name == "status"));
    }

    #[test]
    fn apply_exhausted_rejects_after_being_fully_covered() {
        let mut batch = batch();
        batch.apply_entry(30_000, 2).unwrap();

        assert_eq!(batch.apply_exhausted(3), None);
    }
}
