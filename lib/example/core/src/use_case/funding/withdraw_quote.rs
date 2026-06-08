use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{CommandUseCase2, IssuedByParty};
use thiserror::Error;

use super::super::support::{
    ACCOUNT_ENTITY_TYPE, stable_entity_id, string_field, updated_int_field,
};
use crate::entity::TradingAccount;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub account: TradingAccount,
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

#[derive(Debug, Clone, Copy, Default)]
pub struct WithdrawQuoteUseCase;

impl CommandUseCase2 for WithdrawQuoteUseCase {
    type Command = WithdrawQuoteCmd;
    type GivenState = WithdrawQuoteState;
    type Error = WithdrawQuoteError;

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
        if state.account.available_quote < cmd.amount {
            return Err(WithdrawQuoteError::InsufficientQuoteBalance);
        }

        Ok(())
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let next_available = state
            .account
            .available_quote
            .checked_sub(cmd.amount)
            .ok_or(WithdrawQuoteError::ArithmeticOverflow)?;
        let next_version =
            state.account.version.checked_add(1).ok_or(WithdrawQuoteError::ArithmeticOverflow)?;

        let mut account_event = EntityReplayableEvent::new_updated(
            0,
            0,
            state.account.version,
            next_version,
            stable_entity_id(&state.account.account_id),
            ACCOUNT_ENTITY_TYPE,
        );
        account_event.add_field_change(string_field("account_id", &state.account.account_id));
        account_event.add_field_change(updated_int_field(
            "available_quote",
            state.account.available_quote,
            next_available,
        ));
        account_event.add_field_change(updated_int_field(
            "frozen_quote",
            state.account.frozen_quote,
            state.account.frozen_quote,
        ));

        Ok(vec![account_event])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_case::support::field_as_u64;

    fn sample_state() -> WithdrawQuoteState {
        WithdrawQuoteState {
            account: TradingAccount {
                account_id: "trader-1".to_string(),
                available_base: 0,
                frozen_base: 0,
                available_quote: 1_000,
                frozen_quote: 50,
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
        let events = use_case.compute_replayable_events(
            &WithdrawQuoteCmd { party_id: "trader-1".to_string(), amount: 200 },
            sample_state(),
        )?;

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert_eq!(field_as_u64(&events[0], "available_quote"), Some(800));
        assert_eq!(field_as_u64(&events[0], "frozen_quote"), Some(50));

        Ok(())
    }
}
