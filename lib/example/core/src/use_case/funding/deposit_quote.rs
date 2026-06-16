use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::Balance;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositQuoteCmd {
    pub party_id: String,
    pub amount: u64,
}

impl IssuedByParty for DepositQuoteCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositQuoteState {
    pub quote_balance: Balance,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DepositQuoteError {
    #[error("deposit amount must be greater than zero")]
    InvalidAmount,
    #[error("arithmetic overflow while deriving business result")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositQuoteChanges {
    pub updated_quote_balance: UpdatedEntityPair<Balance>,
}

impl ReplayableChanges for DepositQuoteChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        Ok(vec![
            self.updated_quote_balance
                .after
                .track_update_event_from(&self.updated_quote_balance.before)?,
        ])
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DepositQuoteUseCase;

impl CommandUseCase4 for DepositQuoteUseCase {
    type Command = DepositQuoteCmd;
    type GivenState = DepositQuoteState;
    type Error = DepositQuoteError;
    type Changes = DepositQuoteChanges;

    fn role(&self) -> &'static str {
        "Treasury"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.amount == 0 {
            return Err(DepositQuoteError::InvalidAmount);
        }

        Ok(())
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let mut balance = state.quote_balance;
        let quote_balance_before = balance.clone();
        let next_available = balance
            .available
            .checked_add(cmd.amount)
            .ok_or(DepositQuoteError::ArithmeticOverflow)?;
        let next_version =
            balance.version.checked_add(1).ok_or(DepositQuoteError::ArithmeticOverflow)?;
        let next_frozen = balance.frozen;

        balance.apply_after(next_available, next_frozen, next_version);

        Ok(DepositQuoteChanges {
            updated_quote_balance: UpdatedEntityPair {
                before: quote_balance_before,
                after: balance,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_case::support::field_as_u64;

    fn event_field<'a>(
        event: &'a common_entity::EntityReplayableEvent,
        field_name: &str,
    ) -> Option<&'a str> {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }
            std::str::from_utf8(change.new_value_bytes()).ok()
        })
    }

    fn sample_state() -> DepositQuoteState {
        DepositQuoteState {
            quote_balance: Balance {
                account_id: "trader-1".to_string(),
                asset_id: "USDT".to_string(),
                available: 1_000,
                frozen: 50,
                version: 3,
            },
        }
    }

    #[test]
    fn role_is_treasury() {
        let use_case = DepositQuoteUseCase;
        assert_eq!(use_case.role(), "Treasury");
    }

    #[test]
    fn pre_check_rejects_zero_amount() {
        let use_case = DepositQuoteUseCase;
        let result = use_case
            .pre_check_command(&DepositQuoteCmd { party_id: "trader-1".to_string(), amount: 0 });

        assert_eq!(result, Err(DepositQuoteError::InvalidAmount));
    }

    #[test]
    fn compute_replayable_events_updates_available_quote() -> Result<(), DepositQuoteError> {
        let use_case = DepositQuoteUseCase;
        let changes = use_case.compute_changes(
            &DepositQuoteCmd { party_id: "trader-1".to_string(), amount: 200 },
            sample_state(),
        )?;
        let events =
            changes.to_replayable_events().map_err(|_| DepositQuoteError::ArithmeticOverflow)?;

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert_eq!(event_field(&events[0], "account_id"), Some("trader-1"));
        assert_eq!(event_field(&events[0], "asset_id"), Some("USDT"));
        assert_eq!(field_as_u64(&events[0], "available"), Some(1_200));
        assert_eq!(field_as_u64(&events[0], "frozen"), None);
        assert_eq!(changes.updated_quote_balance.after.available, 1_200);

        Ok(())
    }
}
