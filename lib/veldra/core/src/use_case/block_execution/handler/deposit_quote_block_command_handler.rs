use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCase2;
use example_core::{DepositQuoteCmd, DepositQuoteState, DepositQuoteUseCase};

use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::{
    BlockCommandHandler, apply_treasury_execution, normalize_local_events, rebase_events,
    treasury_quote_balance,
};
use crate::entity::{
    CommandEnvelope, CommandExecutionResult, ExchangeState, ProductCommand, ProductCommandResult,
    TreasuryBalanceUpdate, TreasuryCommandResult, TreasuryState,
};

pub(in crate::use_case::block_execution) static DEPOSIT_QUOTE_BLOCK_COMMAND_HANDLER: DepositQuoteBlockCommandHandler =
    DepositQuoteBlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct DepositQuoteBlockCommandHandler;

impl BlockCommandHandler for DepositQuoteBlockCommandHandler {
    type Command = DepositQuoteCmd;
    type Execution = TreasuryBalanceUpdate;

    fn validate(
        &self,
        _command: &Self::Command,
        _exchange_state: &ExchangeState,
    ) -> Result<(), BuildBlockError> {
        Ok(())
    }

    fn execute(
        &self,
        envelope: &CommandEnvelope<ProductCommand>,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<CommandExecutionResult, BuildBlockError> {
        let execution = execute_deposit_quote(command, &exchange_state.treasury)?;
        Ok(CommandExecutionResult {
            command_id: envelope.command_id.clone(),
            command_kind: "treasury".to_string(),
            command_commitment: envelope.commitment(),
            result: ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(
                execution,
            )),
        })
    }

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution) {
        apply_treasury_execution(&mut exchange_state.treasury, execution);
    }

    fn events<'a>(&self, execution: &'a Self::Execution) -> &'a [EntityReplayableEvent] {
        execution.events.as_slice()
    }

    fn rebase_events(&self, execution: &mut Self::Execution, base_sequence: u64) {
        execution.events = rebase_events(&execution.events, base_sequence);
    }
}

fn execute_deposit_quote(
    command: &DepositQuoteCmd,
    treasury_state: &TreasuryState,
) -> Result<TreasuryBalanceUpdate, BuildBlockError> {
    let balance = treasury_quote_balance(treasury_state, command.party_id.as_str())?;
    CommandUseCase2::pre_check_command(&DepositQuoteUseCase, command)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    let state = DepositQuoteState { quote_balance: balance.clone() };
    CommandUseCase2::validate_against_state(&DepositQuoteUseCase, command, &state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    let events = CommandUseCase2::compute_replayable_events(&DepositQuoteUseCase, command, state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;

    let mut balance_after = balance;
    balance_after.apply_after(
        balance_after.available.checked_add(command.amount).ok_or_else(|| {
            BuildBlockError::TreasuryExecution("treasury balance overflow".to_string())
        })?,
        balance_after.frozen,
        balance_after.version + 1,
    );
    Ok(TreasuryBalanceUpdate { balance_after, events: normalize_local_events(events) })
}
