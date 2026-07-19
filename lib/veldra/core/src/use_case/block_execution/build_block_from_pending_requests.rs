use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::ReplayableChanges;
use common_entity::MiStateMachineV2Unchecked;
use example_core::{DepositQuoteChanges, PlaceSpotOrderV2ChangesV3, WithdrawQuoteChanges};

use super::{
    BlockEntityChange, BuildBlockError, BuildBlockFromCommandsChanges,
    BuildBlockFromCommandsCommand, BuildBlockFromCommandsState,
};
use crate::entity::{
    BlockExecutionBody, CommandEnvelope, ExchangeState, ProductCommand, build_new_block,
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

impl MiStateMachineV2Unchecked for BuildBlockFromCommandsUseCase {
    type Command = BuildBlockFromCommandsCommand;
    type GivenState = BuildBlockFromCommandsState;
    type Error = BuildBlockError;
    type AfterChanges = BuildBlockFromCommandsChanges;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.block_height == 0 {
            return Err(BuildBlockError::BlockHeightMustBePositive);
        }
        Ok(())
    }

    fn validate_against_given_state(
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

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        let parent_block_hash = state.parent_block_hash.clone();
        let mut exchange_state = state.exchange_state.clone();
        let commands = validate_and_clone_canonical_commands(&state.commands)?;
        let ordered_changes = execute_batch_commands(&commands, &mut exchange_state)?;
        let events = BuildBlockFromCommandsChanges {
            new_block: None,
            execution_body: None,
            ordered_changes: ordered_changes.clone(),
        }
        .to_replayable_events()
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
        Ok(build_block_changes(
            cmd.block_height,
            parent_block_hash,
            commands,
            events,
            exchange_state,
            ordered_changes,
        ))
    }
}

fn validate_batch_commands(
    commands: &[CommandEnvelope<ProductCommand>],
    exchange_state: &ExchangeState,
) -> Result<(), BuildBlockError> {
    for envelope in commands {
        match resolve_block_command_handler(&envelope.command) {
            ResolvedBlockCommandHandler::PlaceSpotOrderV2(handler, command) => {
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
) -> Result<Vec<BlockEntityChange>, BuildBlockError> {
    let mut ordered_changes = Vec::new();

    for envelope in commands {
        match resolve_block_command_handler(&envelope.command) {
            ResolvedBlockCommandHandler::PlaceSpotOrderV2(handler, command) => {
                let execution = handler.execute(envelope, command, exchange_state)?;
                ordered_changes.extend(extract_place_spot_order_v2_changes(&execution.changes));
                handler.apply(exchange_state, &execution);
            }
            ResolvedBlockCommandHandler::DepositQuote(handler, command) => {
                let execution = handler.execute(envelope, command, exchange_state)?;
                ordered_changes.push(extract_deposit_quote_change(&execution));
                handler.apply(exchange_state, &execution);
            }
            ResolvedBlockCommandHandler::WithdrawQuote(handler, command) => {
                let execution = handler.execute(envelope, command, exchange_state)?;
                ordered_changes.push(extract_withdraw_quote_change(&execution));
                handler.apply(exchange_state, &execution);
            }
            ResolvedBlockCommandHandler::PerpUnsupported(handler) => {
                execute_unsupported_perp(handler, envelope, exchange_state)?;
            }
        }
    }

    Ok(ordered_changes)
}

fn build_block_changes(
    block_height: u64,
    parent_block_hash: String,
    commands: Vec<CommandEnvelope<ProductCommand>>,
    events: Vec<EntityReplayableEvent>,
    exchange_state: ExchangeState,
    ordered_changes: Vec<BlockEntityChange>,
) -> BuildBlockFromCommandsChanges {
    let new_block =
        build_new_block(block_height, parent_block_hash, &commands, &events, &exchange_state);
    let execution_body = BlockExecutionBody {
        block_hash: new_block.block_hash.clone(),
        block_height: new_block.block_height,
        commands,
        replayable_events: events,
    };

    BuildBlockFromCommandsChanges {
        new_block: Some(new_block),
        execution_body: Some(execution_body),
        ordered_changes,
    }
}

fn extract_place_spot_order_v2_changes(
    execution: &PlaceSpotOrderV2ChangesV3,
) -> Vec<BlockEntityChange> {
    let mut changes = Vec::new();
    if execution.updated_taker_order.before == execution.updated_taker_order.after {
        changes.push(BlockEntityChange::SpotOrderCreated(
            execution.updated_taker_order.after.clone(),
        ));
    } else {
        changes.push(BlockEntityChange::SpotOrderUpdated(
            execution.updated_taker_order.clone(),
        ));
    }
    changes.extend(
        execution
            .updated_maker_orders
            .iter()
            .cloned()
            .map(BlockEntityChange::SpotOrderUpdated),
    );
    changes.extend(
        execution
            .updated_balances
            .iter()
            .filter(|pair| pair.before != pair.after)
            .cloned()
            .map(BlockEntityChange::BalanceUpdated),
    );
    changes.extend(
        execution
            .created_trades
            .iter()
            .cloned()
            .map(BlockEntityChange::SpotTradeCreated),
    );
    changes.extend(
        execution
            .created_vouchers
            .iter()
            .cloned()
            .map(BlockEntityChange::SettlementTransferVoucherCreated),
    );
    changes.extend(
        execution
            .created_balance_ledger_entries
            .iter()
            .cloned()
            .map(BlockEntityChange::BalanceLedgerEntryCreated),
    );
    changes
}

fn extract_deposit_quote_change(execution: &DepositQuoteChanges) -> BlockEntityChange {
    BlockEntityChange::BalanceUpdated(execution.updated_quote_balance.clone())
}

fn extract_withdraw_quote_change(execution: &WithdrawQuoteChanges) -> BlockEntityChange {
    BlockEntityChange::BalanceUpdated(execution.updated_quote_balance.clone())
}
