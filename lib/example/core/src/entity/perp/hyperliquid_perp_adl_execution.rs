use common_entity::{Entity, EntityError, EntityFieldChange};

const HYPERLIQUID_PERP_ADL_EXECUTION_ENTITY_TYPE: u8 = 20;

/// 单个 ADL 去杠杆执行会话的过程型业务事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpAdlExecutionStatus {
    Started,
    Completed,
    Exhausted,
}

impl HyperliquidPerpAdlExecutionStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Completed => "completed",
            Self::Exhausted => "exhausted",
        }
    }
}

/// 围绕一次 ADL 覆盖尝试的过程型 MI。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpAdlExecution {
    pub adl_execution_id: String,
    pub adl_batch_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub deleveraged_account_id: String,
    pub deleveraged_position_id: String,
    pub asset: u32,
    pub symbol: String,
    pub reference_bankruptcy_price: u64,
    pub execution_price: Option<u64>,
    pub qty: Option<u64>,
    pub covered_quote: Option<u64>,
    pub status: HyperliquidPerpAdlExecutionStatus,
    pub version: u64,
}

impl HyperliquidPerpAdlExecution {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adl_execution_id: String,
        adl_batch_id: String,
        liquidation_id: String,
        shortfall_id: String,
        deleveraged_account_id: String,
        deleveraged_position_id: String,
        asset: u32,
        symbol: String,
        reference_bankruptcy_price: u64,
    ) -> Self {
        Self {
            adl_execution_id,
            adl_batch_id,
            liquidation_id,
            shortfall_id,
            deleveraged_account_id,
            deleveraged_position_id,
            asset,
            symbol,
            reference_bankruptcy_price,
            execution_price: None,
            qty: None,
            covered_quote: None,
            status: HyperliquidPerpAdlExecutionStatus::Started,
            version: 1,
        }
    }

    pub fn is_started(&self) -> bool {
        self.status == HyperliquidPerpAdlExecutionStatus::Started
    }

    pub fn apply_completed(
        &mut self,
        execution_price: u64,
        qty: u64,
        covered_quote: u64,
        version: u64,
    ) -> Option<()> {
        if !self.is_started() || execution_price == 0 || qty == 0 || covered_quote == 0 {
            return None;
        }

        self.execution_price = Some(execution_price);
        self.qty = Some(qty);
        self.covered_quote = Some(covered_quote);
        self.status = HyperliquidPerpAdlExecutionStatus::Completed;
        self.version = version;
        Some(())
    }

    pub fn apply_exhausted(&mut self, version: u64) -> Option<()> {
        if !self.is_started() {
            return None;
        }

        self.status = HyperliquidPerpAdlExecutionStatus::Exhausted;
        self.version = version;
        Some(())
    }
}

impl Entity for HyperliquidPerpAdlExecution {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.adl_execution_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_ADL_EXECUTION_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
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
            EntityFieldChange::new(
                "reference_bankruptcy_price",
                "",
                self.reference_bankruptcy_price.to_string(),
            ),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        push_optional_change(
            &mut changes,
            "execution_price",
            self.execution_price,
            other.execution_price,
        );
        push_optional_change(&mut changes, "qty", self.qty, other.qty);
        push_optional_change(
            &mut changes,
            "covered_quote",
            self.covered_quote,
            other.covered_quote,
        );
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "adl_execution_id"
            | "adl_batch_id"
            | "liquidation_id"
            | "shortfall_id"
            | "deleveraged_account_id"
            | "deleveraged_position_id"
            | "symbol"
            | "status" => 0,
            "asset"
            | "reference_bankruptcy_price"
            | "execution_price"
            | "qty"
            | "covered_quote" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.adl_execution_id))
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

fn push_optional_change(
    changes: &mut Vec<EntityFieldChange>,
    field_name: &'static str,
    old_value: Option<u64>,
    new_value: Option<u64>,
) {
    if old_value != new_value {
        changes.push(EntityFieldChange::new(
            field_name,
            old_value.map(|value| value.to_string()).unwrap_or_default(),
            new_value.map(|value| value.to_string()).unwrap_or_default(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn execution() -> HyperliquidPerpAdlExecution {
        HyperliquidPerpAdlExecution::new(
            "adl-exec-1".to_string(),
            "adl-batch-1".to_string(),
            "liq-1".to_string(),
            "shortfall-1".to_string(),
            "winner-1".to_string(),
            "position-9".to_string(),
            7,
            "BTC-PERP".to_string(),
            49_000,
        )
    }

    #[test]
    fn new_execution_starts_in_started_status() {
        let execution = execution();

        assert_eq!(execution.status, HyperliquidPerpAdlExecutionStatus::Started);
        assert_eq!(execution.execution_price, None);
        assert_eq!(execution.qty, None);
        assert_eq!(execution.covered_quote, None);
    }

    #[test]
    fn apply_completed_only_allows_transition_from_started() {
        let mut execution = execution();
        execution.apply_completed(25_000, 1, 25_000, 2).unwrap();

        assert_eq!(execution.status, HyperliquidPerpAdlExecutionStatus::Completed);
        assert_eq!(execution.apply_completed(26_000, 1, 26_000, 3), None);
        assert_eq!(execution.apply_exhausted(3), None);
    }

    #[test]
    fn apply_exhausted_only_allows_transition_from_started() {
        let mut execution = execution();
        execution.apply_exhausted(2).unwrap();

        assert_eq!(execution.status, HyperliquidPerpAdlExecutionStatus::Exhausted);
        assert_eq!(execution.apply_exhausted(3), None);
        assert_eq!(execution.apply_completed(25_000, 1, 25_000, 3), None);
    }
}
