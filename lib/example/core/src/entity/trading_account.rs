use common_entity::{Entity, EntityError, EntityFieldChange};

const TRADING_ACCOUNT_ENTITY_TYPE: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradingAccount {
    pub account_id: String,
    pub available_quote: u64,
    pub frozen_quote: u64,
    pub version: u64,
}

impl TradingAccount {
    pub fn can_reserve_quote(&self, reserved_quote: u64) -> bool {
        self.available_quote >= reserved_quote
    }

    pub fn reserve_quote_after(&self, reserved_quote: u64) -> Option<(u64, u64)> {
        let next_available = self.available_quote.checked_sub(reserved_quote)?;
        let next_frozen = self.frozen_quote.checked_add(reserved_quote)?;
        Some((next_available, next_frozen))
    }
}

impl Entity for TradingAccount {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.account_id.clone()
    }

    fn entity_type() -> u8 {
        TRADING_ACCOUNT_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        if self.available_quote != other.available_quote {
            changes.push(EntityFieldChange::new(
                "available_quote",
                self.available_quote.to_string(),
                other.available_quote.to_string(),
            ));
        }

        if self.frozen_quote != other.frozen_quote {
            changes.push(EntityFieldChange::new(
                "frozen_quote",
                self.frozen_quote.to_string(),
                other.frozen_quote.to_string(),
            ));
        }

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "available_quote" | "frozen_quote" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_account_entity_id(&self.account_id))
    }
}

fn stable_account_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}
