use common_entity::{
    Entity, EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff, UpdatedEntityPair,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{Balance, BalanceError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositQuoteCmd {
    pub party_id: String,
    pub amount: u64,
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
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.updated_quote_balance
                .after
                .track_update_event_from(&self.updated_quote_balance.before)?,
        ])
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DepositQuoteUseCase;

impl StateMachineV2 for DepositQuoteUseCase {
    type Command = LogEntry<DepositQuoteCmd>;
    type StateGiven = DepositQuoteState;
    type Error = DepositQuoteError;
    type StateChanged = DepositQuoteChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.amount == 0 {
            return Err(DepositQuoteError::InvalidAmount);
        }

        Ok(())
    }

    fn validate_state_given(
        &self,
        _cmd: &Self::Command,
        _state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let quote_balance_before = state.quote_balance.clone();
        let mut quote_balance_after = quote_balance_before.clone();
        quote_balance_after.credit_available(cmd.amount).map_err(map_balance_error)?;

        Ok(DepositQuoteChanges {
            updated_quote_balance: UpdatedEntityPair {
                before: quote_balance_before,
                after: quote_balance_after,
            },
        })
    }
}

impl DepositQuoteUseCase {
    pub const fn role(&self) -> &'static str {
        "Treasury"
    }
}
impl StateMachineV2Diff for DepositQuoteUseCase {
    type StateDiff = DepositQuoteChanges;
    fn do_compute_state_diff(
        _: DepositQuoteState,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(changed)
    }
}

fn map_balance_error(error: BalanceError) -> DepositQuoteError {
    match error {
        BalanceError::InvalidAmount => DepositQuoteError::InvalidAmount,
        BalanceError::ArithmeticOverflow
        | BalanceError::InsufficientAvailableBalance
        | BalanceError::InsufficientFrozenBalance => DepositQuoteError::ArithmeticOverflow,
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
                party_id: common_entity::stable_string_entity_id("trader-1"),
                account_id: common_entity::stable_string_entity_id("trader-1"),
                asset_id: common_entity::stable_string_entity_id("USDT"),
                available: 1_000,
                frozen: 50,
                entry_notional: None,
                identifier: None,
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
        let result = use_case.check_command(&common_entity::LogEntrySequencer::from_metadata(
            1,
            1,
            1,
            1,
            DepositQuoteCmd { party_id: "trader-1".to_string(), amount: 0 },
        ));

        assert_eq!(result, Err(DepositQuoteError::InvalidAmount));
    }

    #[test]
    fn compute_replayable_events_updates_available_quote() -> Result<(), DepositQuoteError> {
        let use_case = DepositQuoteUseCase;
        let command = common_entity::LogEntrySequencer::from_metadata(
            1,
            1,
            1,
            1,
            DepositQuoteCmd { party_id: "trader-1".to_string(), amount: 200 },
        );
        let changes = use_case.compute_state_diff(&command, sample_state())?;
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
