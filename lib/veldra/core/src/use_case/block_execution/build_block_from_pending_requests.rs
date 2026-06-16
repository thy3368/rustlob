use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges, UpdatedEntityPair};
use example_core::{
    CancelSpotOrderChanges, DepositQuoteChanges, ExecuteImmediateSpotOrderPipelineChanges,
    WithdrawQuoteChanges,
};

use super::{
    BlockEntityChange, BuildBlockError, BuildBlockFromCommandsChanges,
    BuildBlockFromCommandsCommand, BuildBlockFromCommandsState,
};
use crate::entity::{CommandEnvelope, ExchangeState, ProductCommand, build_new_block};
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
        let ordered_changes = execute_batch_commands(&commands, &mut exchange_state)?;
        let events = BuildBlockFromCommandsChanges {
            new_block: None,
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
) -> Result<Vec<BlockEntityChange>, BuildBlockError> {
    let mut ordered_changes = Vec::new();

    for envelope in commands {
        match resolve_block_command_handler(&envelope.command) {
            ResolvedBlockCommandHandler::ExecuteImmediateOrderPipeline(handler, command) => {
                let execution = handler.execute(envelope, command, exchange_state)?;
                ordered_changes.extend(extract_spot_pipeline_changes(&execution.changes));
                handler.apply(exchange_state, &execution);
            }
            ResolvedBlockCommandHandler::CancelOrder(handler, command) => {
                let execution = handler.execute(envelope, command, exchange_state)?;
                ordered_changes.extend(extract_spot_cancel_changes(&execution));
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

    BuildBlockFromCommandsChanges { new_block: Some(new_block), ordered_changes }
}

fn extract_spot_pipeline_changes(
    execution: &ExecuteImmediateSpotOrderPipelineChanges,
) -> Vec<BlockEntityChange> {
    let mut ordered_changes = Vec::new();
    let place_output = &execution.place_output;
    ordered_changes.push(BlockEntityChange::SpotOrderCreated(place_output.order.clone()));
    ordered_changes.push(BlockEntityChange::BalanceUpdated(UpdatedEntityPair {
        before: place_output.affected_balance_before.clone(),
        after: place_output.affected_balance_after.clone(),
    }));

    if let Some(match_output) = &execution.match_output {
        for (trade, maker_update) in
            match_output.trades.iter().zip(&match_output.maker_orders_updated)
        {
            ordered_changes.push(BlockEntityChange::SpotTradeCreated(trade.clone()));
            ordered_changes.push(BlockEntityChange::SpotOrderUpdated(maker_update.clone()));
        }
        ordered_changes.push(BlockEntityChange::SpotOrderUpdated(UpdatedEntityPair {
            before: match_output.taker_order_before.clone(),
            after: match_output.taker_order_after.clone(),
        }));
    }

    if let Some(settle_changes) = &execution.settle_changes {
        for settlement in &settle_changes.settlements {
            ordered_changes.push(BlockEntityChange::SpotSettlementCreated(settlement.clone()));
        }
        for balance in &settle_changes.balances_updated {
            ordered_changes.push(BlockEntityChange::BalanceUpdated(balance.clone()));
        }
    }

    ordered_changes
}

fn extract_spot_cancel_changes(execution: &CancelSpotOrderChanges) -> Vec<BlockEntityChange> {
    let mut ordered_changes = Vec::with_capacity(1 + execution.balances_updated.len());
    ordered_changes.push(BlockEntityChange::SpotOrderUpdated(UpdatedEntityPair {
        before: execution.order_before.clone(),
        after: execution.order_after.clone(),
    }));
    for balance in &execution.balances_updated {
        ordered_changes.push(BlockEntityChange::BalanceUpdated(balance.clone()));
    }
    ordered_changes
}

fn extract_deposit_quote_change(execution: &DepositQuoteChanges) -> BlockEntityChange {
    BlockEntityChange::BalanceUpdated(UpdatedEntityPair {
        before: execution.quote_balance_before.clone(),
        after: execution.quote_balance_after.clone(),
    })
}

fn extract_withdraw_quote_change(execution: &WithdrawQuoteChanges) -> BlockEntityChange {
    BlockEntityChange::BalanceUpdated(UpdatedEntityPair {
        before: execution.quote_balance_before.clone(),
        after: execution.quote_balance_after.clone(),
    })
}
