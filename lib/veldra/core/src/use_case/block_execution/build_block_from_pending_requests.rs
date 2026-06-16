use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges, UpdatedEntityPair};
use example_core::{Balance, SpotOrder};

use super::{
    BlockEntityChange, BuildBlockError, BuildBlockFromCommandsChanges,
    BuildBlockFromCommandsCommand, BuildBlockFromCommandsState,
};
use crate::entity::{
    AccountAssetKey, CommandEnvelope, ExchangeState, ProductCommand, ProductCommandResult,
    SpotCancelExecution, SpotCommandResult, SpotPipelineExecution, TreasuryBalanceUpdate,
    TreasuryCommandResult, build_new_block,
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
        let ordered_changes = execute_batch_commands(&commands, &mut exchange_state)?;
        let events = project_rebased_events(&ordered_changes)?;
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
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Spot(SpotCommandResult::ExecuteImmediateOrderPipeline(
                    execution,
                )) = &mut result.result
                else {
                    unreachable!("spot pipeline handler returned unexpected result");
                };
                ordered_changes.extend(extract_spot_pipeline_changes(exchange_state, execution)?);
                handler.apply(exchange_state, execution);
            }
            ResolvedBlockCommandHandler::CancelOrder(handler, command) => {
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Spot(SpotCommandResult::CancelOrder(execution)) =
                    &mut result.result
                else {
                    unreachable!("spot cancel handler returned unexpected result");
                };
                ordered_changes.extend(extract_spot_cancel_changes(exchange_state, execution)?);
                handler.apply(exchange_state, execution);
            }
            ResolvedBlockCommandHandler::DepositQuote(handler, command) => {
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(
                    execution,
                )) = &mut result.result
                else {
                    unreachable!("treasury deposit handler returned unexpected result");
                };
                ordered_changes.push(extract_treasury_balance_change(exchange_state, execution)?);
                handler.apply(exchange_state, execution);
            }
            ResolvedBlockCommandHandler::WithdrawQuote(handler, command) => {
                let mut result = handler.execute(envelope, command, exchange_state)?;
                let ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(
                    execution,
                )) = &mut result.result
                else {
                    unreachable!("treasury withdraw handler returned unexpected result");
                };
                ordered_changes.push(extract_treasury_balance_change(exchange_state, execution)?);
                handler.apply(exchange_state, execution);
            }
            ResolvedBlockCommandHandler::PerpUnsupported(handler) => {
                let _ = execute_unsupported_perp(handler, envelope, exchange_state)?;
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

fn project_rebased_events(
    ordered_changes: &[BlockEntityChange],
) -> Result<Vec<EntityReplayableEvent>, BuildBlockError> {
    let changes = BuildBlockFromCommandsChanges {
        new_block: None,
        ordered_changes: ordered_changes.to_vec(),
    };
    let events = changes
        .to_replayable_events()
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;

    Ok(events
        .into_iter()
        .enumerate()
        .map(|(sequence, mut event)| {
            event.timestamp = 0;
            event.sequence = sequence as u64;
            event
        })
        .collect())
}

fn extract_spot_pipeline_changes(
    exchange_state: &ExchangeState,
    execution: &SpotPipelineExecution,
) -> Result<Vec<BlockEntityChange>, BuildBlockError> {
    let mut ordered_changes = Vec::new();
    let mut spot_balances = exchange_state.spot.balances.clone();
    let mut spot_orders = exchange_state.spot.orders.clone();

    let place_output = &execution.pipeline_output.place_output;
    ordered_changes.push(BlockEntityChange::SpotOrderCreated(place_output.order.clone()));
    spot_orders.insert(place_output.order.order_id.clone(), place_output.order.clone());

    ordered_changes.push(BlockEntityChange::BalanceUpdated(UpdatedEntityPair {
        before: place_output.affected_balance_before.clone(),
        after: place_output.affected_balance_after.clone(),
    }));
    upsert_spot_balance(&mut spot_balances, &place_output.affected_balance_after);

    if let Some(match_output) = &execution.pipeline_output.match_output {
        for (trade, maker_after) in match_output.trades.iter().zip(&match_output.maker_orders_after)
        {
            ordered_changes.push(BlockEntityChange::SpotTradeCreated(trade.clone()));
            ordered_changes.push(BlockEntityChange::SpotOrderUpdated(updated_spot_order_pair(
                &spot_orders,
                maker_after,
            )?));
            spot_orders.insert(maker_after.order_id.clone(), maker_after.clone());
        }

        ordered_changes.push(BlockEntityChange::SpotOrderUpdated(UpdatedEntityPair {
            before: match_output.taker_order_before.clone(),
            after: match_output.taker_order_after.clone(),
        }));
        spot_orders.insert(
            match_output.taker_order_after.order_id.clone(),
            match_output.taker_order_after.clone(),
        );
    }

    if !execution.settlements.is_empty() {
        for settlement in &execution.settlements {
            ordered_changes.push(BlockEntityChange::SpotSettlementCreated(settlement.clone()));
        }

        for balance_after in &execution.balances_after {
            ordered_changes.push(BlockEntityChange::BalanceUpdated(updated_spot_balance_pair(
                &spot_balances,
                balance_after,
            )?));
            upsert_spot_balance(&mut spot_balances, balance_after);
        }
    }

    Ok(ordered_changes)
}

fn extract_spot_cancel_changes(
    exchange_state: &ExchangeState,
    execution: &SpotCancelExecution,
) -> Result<Vec<BlockEntityChange>, BuildBlockError> {
    let mut ordered_changes = Vec::with_capacity(1 + execution.balances_after.len());
    ordered_changes.push(BlockEntityChange::SpotOrderUpdated(updated_spot_order_pair(
        &exchange_state.spot.orders,
        &execution.order_after,
    )?));

    for balance_after in &execution.balances_after {
        ordered_changes.push(BlockEntityChange::BalanceUpdated(updated_spot_balance_pair(
            &exchange_state.spot.balances,
            balance_after,
        )?));
    }

    Ok(ordered_changes)
}

fn extract_treasury_balance_change(
    exchange_state: &ExchangeState,
    execution: &TreasuryBalanceUpdate,
) -> Result<BlockEntityChange, BuildBlockError> {
    Ok(BlockEntityChange::BalanceUpdated(updated_treasury_balance_pair(
        &exchange_state.treasury.balances,
        &execution.balance_after,
    )?))
}

fn updated_spot_order_pair(
    orders: &std::collections::BTreeMap<String, SpotOrder>,
    after: &SpotOrder,
) -> Result<UpdatedEntityPair<SpotOrder>, BuildBlockError> {
    let before = orders.get(after.order_id.as_str()).cloned().ok_or_else(|| {
        BuildBlockError::SpotExecution(format!(
            "missing spot order before update: {}",
            after.order_id
        ))
    })?;
    Ok(UpdatedEntityPair { before, after: after.clone() })
}

fn updated_spot_balance_pair(
    balances: &std::collections::BTreeMap<AccountAssetKey, Balance>,
    after: &Balance,
) -> Result<UpdatedEntityPair<Balance>, BuildBlockError> {
    let key = AccountAssetKey::new(after.account_id.as_str(), after.asset_id.as_str());
    let before =
        balances.get(&key).cloned().ok_or_else(|| BuildBlockError::MissingSpotBalance {
            account_id: after.account_id.clone(),
            asset_id: after.asset_id.clone(),
        })?;
    Ok(UpdatedEntityPair { before, after: after.clone() })
}

fn updated_treasury_balance_pair(
    balances: &std::collections::BTreeMap<AccountAssetKey, Balance>,
    after: &Balance,
) -> Result<UpdatedEntityPair<Balance>, BuildBlockError> {
    let key = AccountAssetKey::new(after.account_id.as_str(), after.asset_id.as_str());
    let before =
        balances.get(&key).cloned().ok_or_else(|| BuildBlockError::MissingTreasuryBalance {
            account_id: after.account_id.clone(),
            asset_id: after.asset_id.clone(),
        })?;
    Ok(UpdatedEntityPair { before, after: after.clone() })
}

fn upsert_spot_balance(
    balances: &mut std::collections::BTreeMap<AccountAssetKey, Balance>,
    balance: &Balance,
) {
    balances.insert(
        AccountAssetKey::new(balance.account_id.as_str(), balance.asset_id.as_str()),
        balance.clone(),
    );
}
