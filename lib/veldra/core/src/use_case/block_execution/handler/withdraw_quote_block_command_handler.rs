use cmd_handler::command_use_case_def2::CommandUseCase4;
use example_core::{
    WithdrawQuoteChanges, WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteUseCase,
};

use crate::entity::{CommandEnvelope, ExchangeState, ProductCommand, TreasuryState};
use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::{
    BlockCommandHandler, apply_withdraw_quote_changes, treasury_quote_balance,
};

pub(in crate::use_case::block_execution) static WITHDRAW_QUOTE_BLOCK_COMMAND_HANDLER:
    WithdrawQuoteBlockCommandHandler = WithdrawQuoteBlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct WithdrawQuoteBlockCommandHandler;

impl BlockCommandHandler for WithdrawQuoteBlockCommandHandler {
    type Command = WithdrawQuoteCmd;
    type Execution = WithdrawQuoteChanges;

    fn validate(
        &self,
        _command: &Self::Command,
        _exchange_state: &ExchangeState,
    ) -> Result<(), BuildBlockError> {
        Ok(())
    }

    fn execute(
        &self,
        _envelope: &CommandEnvelope<ProductCommand>,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<Self::Execution, BuildBlockError> {
        execute_withdraw_quote(command, &exchange_state.treasury)
    }

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution) {
        apply_withdraw_quote_changes(&mut exchange_state.treasury, execution);
    }
}

fn execute_withdraw_quote(
    command: &WithdrawQuoteCmd,
    treasury_state: &TreasuryState,
) -> Result<WithdrawQuoteChanges, BuildBlockError> {
    let balance = treasury_quote_balance(treasury_state, command.party_id.as_str())?;
    CommandUseCase4::pre_check_command(&WithdrawQuoteUseCase, command)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    let state = WithdrawQuoteState { quote_balance: balance.clone() };
    CommandUseCase4::validate_against_state(&WithdrawQuoteUseCase, command, &state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    CommandUseCase4::compute_changes(&WithdrawQuoteUseCase, command, state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))
}
