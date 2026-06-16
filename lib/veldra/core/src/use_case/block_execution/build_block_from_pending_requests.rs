use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCase4;

use super::{
    BuildBlockError, BuildBlockFromCommandsChanges, BuildBlockFromCommandsCommand,
    BuildBlockFromCommandsState,
};
use crate::entity::{
    CommandEnvelope, CommandExecutionResult, ExchangeState, ProductCommand, ProductCommandResult,
    SpotCommandResult, TreasuryCommandResult, build_new_block,
};
use crate::use_case::block_execution::canonical_batch::validate_and_clone_canonical_commands;
use crate::use_case::block_execution::handler::block_command_handler::{
    BlockCommandHandler, ResolvedBlockCommandHandler, resolve_block_command_handler,
};
use crate::use_case::block_execution::handler::perp_unsupported_block_command_handler::{
    execute_unsupported_perp, validate_unsupported_perp,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct BuildBlockFromCommandsUseCase;

impl CommandUseCase4 for BuildBlockFromCommandsUseCase {
    type Command = BuildBlockFromCommandsCommand;
    type GivenState = BuildBlockFromCommandsState;
    type Error = BuildBlockError;
    type Changes = BuildBlockFromCommandsChanges;

    fn role(&self) -> &'static str {
        "BlockBuilder"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.block_height == 0 {
            return Err(BuildBlockError::BlockHeightMustBePositive);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.commands.is_empty() {
            return Err(BuildBlockError::EmptyCommands);
        }

        let expected_block_height = state.parent_height + 1;
        if cmd.block_height != expected_block_height {
            return Err(BuildBlockError::NonContinuousBlockHeight {
                parent_height: state.parent_height,
                actual: cmd.block_height,
            });
        }
        let commands = validate_and_clone_canonical_commands(&state.commands)?;
        validate_batch_commands(&commands, &state.exchange_state)
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let BuildBlockFromCommandsState { parent_block_hash, mut exchange_state, commands, .. } =
            state;
        let commands = validate_and_clone_canonical_commands(&commands)?;
        let (command_results, events) = execute_batch_commands(&commands, &mut exchange_state)?;
        Ok(build_block_changes(
            cmd.block_height,
            parent_block_hash,
            commands,
            events,
            exchange_state,
            command_results,
        ))
    }
}

fn validate_batch_commands(
    commands: &[CommandEnvelope<ProductCommand>],
    exchange_state: &ExchangeState,
) -> Result<(), BuildBlockError> {
    for envelope in commands {
        match resolve_block_command_handler(&envelope.command) {
            ResolvedBlockCommandHandler::ExecuteImmediateOrderPipeline(handler, command) => {
                handler.validate(command, exchange_state)?;
            }
            ResolvedBlockCommandHandler::CancelOrder(handler, command) => {
                handler.validate(command, exchange_state)?;
            }
            ResolvedBlockCommandHandler::DepositQuote(handler, command) => {
                handler.validate(command, exchange_state)?;
            }
            ResolvedBlockCommandHandler::WithdrawQuote(handler, command) => {
                handler.validate(command, exchange_state)?;
            }
            ResolvedBlockCommandHandler::PerpUnsupported(_) => validate_unsupported_perp()?,
        }
    }
    Ok(())
}

fn execute_batch_commands(
    commands: &[CommandEnvelope<ProductCommand>],
    exchange_state: &mut ExchangeState,
) -> Result<(Vec<CommandExecutionResult>, Vec<EntityReplayableEvent>), BuildBlockError> {
    let mut command_results = Vec::with_capacity(commands.len());
    let mut events = Vec::new();
    let mut next_sequence = 0_u64;

    for envelope in commands {
        let result = match resolve_block_command_handler(&envelope.command) {
            ResolvedBlockCommandHandler::ExecuteImmediateOrderPipeline(handler, command) => {
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Spot(SpotCommandResult::ExecuteImmediateOrderPipeline(
                    execution,
                )) = &mut result.result
                else {
                    unreachable!("spot pipeline handler returned unexpected result");
                };
                handler.rebase_events(execution, next_sequence);
                next_sequence += handler.events(execution).len() as u64;
                events.extend(handler.events(execution).iter().cloned());
                handler.apply(exchange_state, execution);
                result
            }
            ResolvedBlockCommandHandler::CancelOrder(handler, command) => {
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Spot(SpotCommandResult::CancelOrder(execution)) =
                    &mut result.result
                else {
                    unreachable!("spot cancel handler returned unexpected result");
                };
                handler.rebase_events(execution, next_sequence);
                next_sequence += handler.events(execution).len() as u64;
                events.extend(handler.events(execution).iter().cloned());
                handler.apply(exchange_state, execution);
                result
            }
            ResolvedBlockCommandHandler::DepositQuote(handler, command) => {
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(
                    execution,
                )) = &mut result.result
                else {
                    unreachable!("treasury deposit handler returned unexpected result");
                };
                handler.rebase_events(execution, next_sequence);
                next_sequence += handler.events(execution).len() as u64;
                events.extend(handler.events(execution).iter().cloned());
                handler.apply(exchange_state, execution);
                result
            }
            ResolvedBlockCommandHandler::WithdrawQuote(handler, command) => {
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(
                    execution,
                )) = &mut result.result
                else {
                    unreachable!("treasury withdraw handler returned unexpected result");
                };
                handler.rebase_events(execution, next_sequence);
                next_sequence += handler.events(execution).len() as u64;
                events.extend(handler.events(execution).iter().cloned());
                handler.apply(exchange_state, execution);
                result
            }
            ResolvedBlockCommandHandler::PerpUnsupported(handler) => {
                execute_unsupported_perp(handler, envelope, exchange_state)?
            }
        };
        command_results.push(result);
    }

    Ok((command_results, events))
}

fn build_block_changes(
    block_height: u64,
    parent_block_hash: String,
    commands: Vec<CommandEnvelope<ProductCommand>>,
    events: Vec<EntityReplayableEvent>,
    exchange_state: ExchangeState,
    command_results: Vec<CommandExecutionResult>,
) -> BuildBlockFromCommandsChanges {
    let new_block =
        build_new_block(block_height, parent_block_hash, &commands, &events, &exchange_state);

    BuildBlockFromCommandsChanges {
        new_block,
        command_results,
        exchange_state,
        replayable_events: events,
    }
}
