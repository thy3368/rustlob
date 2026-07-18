use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{Balance, BalanceError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawQuoteCmd {
    pub party_id: String,
    pub amount: u64,
}

impl IssuedByParty for WithdrawQuoteCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawQuoteState {
    pub quote_balance: Balance,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum WithdrawQuoteError {
    #[error("withdraw amount must be greater than zero")]
    InvalidAmount,
    #[error("insufficient quote balance")]
    InsufficientQuoteBalance,
    #[error("arithmetic overflow while deriving business result")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawQuoteChanges {
    pub updated_quote_balance: UpdatedEntityPair<Balance>,
}

impl ReplayableChanges for WithdrawQuoteChanges {
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
pub struct WithdrawQuoteUseCase;

impl CommandUseCase4 for WithdrawQuoteUseCase {
    type Command = WithdrawQuoteCmd;
    type GivenState = WithdrawQuoteState;
    type Error = WithdrawQuoteError;
    type Changes = WithdrawQuoteChanges;

    fn role(&self) -> &'static str {
        "Treasury"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.amount == 0 {
            return Err(WithdrawQuoteError::InvalidAmount);
        }

        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.quote_balance.available < cmd.amount {
            return Err(WithdrawQuoteError::InsufficientQuoteBalance);
        }

        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let quote_balance_before = state.quote_balance;
        let mut quote_balance_after = quote_balance_before.clone();
        quote_balance_after.debit_available(cmd.amount).map_err(map_balance_error)?;

        Ok(WithdrawQuoteChanges {
            updated_quote_balance: UpdatedEntityPair {
                before: quote_balance_before,
                after: quote_balance_after,
            },
        })
    }
}

fn map_balance_error(error: BalanceError) -> WithdrawQuoteError {
    match error {
        BalanceError::InvalidAmount => WithdrawQuoteError::InvalidAmount,
        BalanceError::InsufficientAvailableBalance => WithdrawQuoteError::InsufficientQuoteBalance,
        BalanceError::ArithmeticOverflow | BalanceError::InsufficientFrozenBalance => {
            WithdrawQuoteError::ArithmeticOverflow
        }
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

    fn sample_state() -> WithdrawQuoteState {
        WithdrawQuoteState {
            quote_balance: Balance {
                account_id: "trader-1".to_string(),
                asset_id: "USDT".to_string(),
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
        let use_case = WithdrawQuoteUseCase;
        assert_eq!(use_case.role(), "Treasury");
    }

    #[test]
    fn pre_check_rejects_zero_amount() {
        let use_case = WithdrawQuoteUseCase;
        let result = use_case
            .pre_check_command(&WithdrawQuoteCmd { party_id: "trader-1".to_string(), amount: 0 });

        assert_eq!(result, Err(WithdrawQuoteError::InvalidAmount));
    }

    #[test]
    fn validate_against_state_rejects_insufficient_balance() {
        let use_case = WithdrawQuoteUseCase;
        let result = use_case.validate_against_state(
            &WithdrawQuoteCmd { party_id: "trader-1".to_string(), amount: 2_000 },
            &sample_state(),
        );

        assert_eq!(result, Err(WithdrawQuoteError::InsufficientQuoteBalance));
    }

    #[test]
    fn compute_replayable_events_updates_available_quote() -> Result<(), WithdrawQuoteError> {
        let use_case = WithdrawQuoteUseCase;
        let changes = use_case.compute_changes(
            &WithdrawQuoteCmd { party_id: "trader-1".to_string(), amount: 200 },
            sample_state(),
        )?;
        let events =
            changes.to_replayable_events().map_err(|_| WithdrawQuoteError::ArithmeticOverflow)?;

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert_eq!(event_field(&events[0], "account_id"), Some("trader-1"));
        assert_eq!(event_field(&events[0], "asset_id"), Some("USDT"));
        assert_eq!(field_as_u64(&events[0], "available"), Some(800));
        assert_eq!(field_as_u64(&events[0], "frozen"), None);
        assert_eq!(changes.updated_quote_balance.after.available, 800);

        Ok(())
    }
}
