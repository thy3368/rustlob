use cmd_handler::command_use_case_def2::CommandUseCase4;
use example_core::{DepositQuoteChanges, DepositQuoteCmd, DepositQuoteState, DepositQuoteUseCase};

use crate::entity::{CommandEnvelope, ExchangeState, ProductCommand, TreasuryState};
use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::{
    BlockCommandHandler, apply_deposit_quote_changes, treasury_quote_balance,
};

pub(in crate::use_case::block_execution) static DEPOSIT_QUOTE_BLOCK_COMMAND_HANDLER:
    DepositQuoteBlockCommandHandler = DepositQuoteBlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct DepositQuoteBlockCommandHandler;

impl BlockCommandHandler for DepositQuoteBlockCommandHandler {
    type Command = DepositQuoteCmd;
    type Execution = DepositQuoteChanges;

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
        execute_deposit_quote(command, &exchange_state.treasury)
    }

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution) {
        apply_deposit_quote_changes(&mut exchange_state.treasury, execution);
    }
}

fn execute_deposit_quote(
    command: &DepositQuoteCmd,
    treasury_state: &TreasuryState,
) -> Result<DepositQuoteChanges, BuildBlockError> {
    let balance = treasury_quote_balance(treasury_state, command.party_id.as_str())?;
    CommandUseCase4::pre_check_command(&DepositQuoteUseCase, command)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    let state = DepositQuoteState { quote_balance: balance.clone() };
    CommandUseCase4::validate_against_state(&DepositQuoteUseCase, command, &state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    CommandUseCase4::compute_changes(&DepositQuoteUseCase, command, state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))
}
