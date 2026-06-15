use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCase2;
use example_core::{
    Balance, CancelSpotOrderCmd, CancelSpotOrderState, CancelSpotOrderUseCase, SpotOrder,
    SpotOrderStatus, SpotOrderStatusReason,
};

use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::{BlockCommandHandler, normalize_local_events, rebase_events};
use crate::entity::{
    AccountAssetKey, CommandEnvelope, CommandExecutionResult, ExchangeState, ProductCommand,
    ProductCommandResult, SpotCancelExecution, SpotCommandResult, SpotState,
};

pub(in crate::use_case::block_execution) static CANCEL_ORDER_BLOCK_COMMAND_HANDLER: CancelOrderBlockCommandHandler =
    CancelOrderBlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct CancelOrderBlockCommandHandler;

impl BlockCommandHandler for CancelOrderBlockCommandHandler {
    type Command = CancelSpotOrderCmd;
    type Execution = SpotCancelExecution;

    fn validate(
        &self,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<(), BuildBlockError> {
        let _ = build_cancel_state(command, &exchange_state.spot)?;
        Ok(())
    }

    fn execute(
        &self,
        envelope: &CommandEnvelope<ProductCommand>,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<CommandExecutionResult, BuildBlockError> {
        let execution = execute_cancel_spot_order(command, &exchange_state.spot)?;
        Ok(CommandExecutionResult {
            command_id: envelope.command_id.clone(),
            command_kind: "spot".to_string(),
            command_commitment: envelope.commitment(),
            result: ProductCommandResult::Spot(SpotCommandResult::CancelOrder(execution)),
        })
    }

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution) {
        for balance in &execution.balances_after {
            exchange_state.spot.balances.insert(
                AccountAssetKey::new(balance.account_id.as_str(), balance.asset_id.as_str()),
                balance.clone(),
            );
        }
        exchange_state
            .spot
            .orders
            .insert(execution.order_after.order_id.clone(), execution.order_after.clone());
    }

    fn events<'a>(&self, execution: &'a Self::Execution) -> &'a [EntityReplayableEvent] {
        execution.events.as_slice()
    }

    fn rebase_events(&self, execution: &mut Self::Execution, base_sequence: u64) {
        execution.events = rebase_events(&execution.events, base_sequence);
    }
}

fn build_cancel_state(
    command: &CancelSpotOrderCmd,
    spot_state: &SpotState,
) -> Result<CancelSpotOrderState, BuildBlockError> {
    let order = spot_state
        .orders
        .values()
        .find(|order| {
            order.exchange_oid == Some(command.order_id)
                && order.asset == command.asset
                && order.can_be_cancelled()
        })
        .cloned()
        .ok_or_else(|| BuildBlockError::SpotExecution("open order was not found".to_string()))?;

    let asset_pair = spot_state
        .asset_pairs_by_symbol
        .get(order.symbol.as_str())
        .ok_or_else(|| BuildBlockError::MissingSpotAssetPair { symbol: order.symbol.clone() })?;
    let base_balance = spot_state
        .balances
        .get(&AccountAssetKey::new(order.account_id.as_str(), asset_pair.base_asset_id.as_str()))
        .cloned()
        .ok_or_else(|| BuildBlockError::MissingSpotBalance {
            account_id: order.account_id.clone(),
            asset_id: asset_pair.base_asset_id.clone(),
        })?;
    let quote_balance = spot_state
        .balances
        .get(&AccountAssetKey::new(order.account_id.as_str(), asset_pair.quote_asset_id.as_str()))
        .cloned()
        .ok_or_else(|| BuildBlockError::MissingSpotBalance {
            account_id: order.account_id.clone(),
            asset_id: asset_pair.quote_asset_id.clone(),
        })?;

    Ok(CancelSpotOrderState {
        account_id: order.account_id.clone(),
        open_order: Some(order),
        base_balance,
        quote_balance,
    })
}

fn execute_cancel_spot_order(
    command: &CancelSpotOrderCmd,
    spot_state: &SpotState,
) -> Result<SpotCancelExecution, BuildBlockError> {
    CommandUseCase2::pre_check_command(&CancelSpotOrderUseCase, command)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let state = build_cancel_state(command, spot_state)?;
    CommandUseCase2::validate_against_state(&CancelSpotOrderUseCase, command, &state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let events =
        CommandUseCase2::compute_replayable_events(&CancelSpotOrderUseCase, command, state.clone())
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;

    let mut order_after = state
        .open_order
        .clone()
        .ok_or_else(|| BuildBlockError::SpotExecution("open order was not found".to_string()))?;
    order_after.status = SpotOrderStatus::Canceled;
    order_after.status_reason = Some(SpotOrderStatusReason::CanceledByUser);
    order_after.version = order_after
        .version
        .checked_add(1)
        .ok_or_else(|| BuildBlockError::SpotExecution("spot order version overflow".to_string()))?;
    let balance_after = release_cancelled_order_balance(&state, &order_after)?;

    Ok(SpotCancelExecution {
        order_after,
        balances_after: vec![balance_after],
        events: normalize_local_events(events),
    })
}

fn release_cancelled_order_balance(
    state: &CancelSpotOrderState,
    order_after: &SpotOrder,
) -> Result<Balance, BuildBlockError> {
    if order_after.quote_to_release_on_cancel() > 0 {
        return release_balance(
            state.quote_balance.clone(),
            order_after.quote_to_release_on_cancel(),
            "spot quote release overflow",
            "spot quote balance version overflow",
        );
    }

    release_balance(
        state.base_balance.clone(),
        order_after.base_to_release_on_cancel(),
        "spot base release overflow",
        "spot base balance version overflow",
    )
}

fn release_balance(
    mut balance: Balance,
    release_amount: u64,
    release_error: &'static str,
    version_error: &'static str,
) -> Result<Balance, BuildBlockError> {
    let (next_available, next_frozen) = balance
        .release_after(release_amount)
        .ok_or_else(|| BuildBlockError::SpotExecution(release_error.to_string()))?;
    let next_version = balance
        .version
        .checked_add(1)
        .ok_or_else(|| BuildBlockError::SpotExecution(version_error.to_string()))?;
    balance.apply_after(next_available, next_frozen, next_version);
    Ok(balance)
}
