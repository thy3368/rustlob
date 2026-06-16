use cmd_handler::command_use_case_def2::CommandUseCase4;
use example_core::{
    CancelSpotOrderChanges, CancelSpotOrderCmd, CancelSpotOrderState, CancelSpotOrderUseCase,
};

use crate::entity::{AccountAssetKey, CommandEnvelope, ExchangeState, ProductCommand, SpotState};
use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::BlockCommandHandler;

pub(in crate::use_case::block_execution) static CANCEL_ORDER_BLOCK_COMMAND_HANDLER:
    CancelOrderBlockCommandHandler = CancelOrderBlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct CancelOrderBlockCommandHandler;

impl BlockCommandHandler for CancelOrderBlockCommandHandler {
    type Command = CancelSpotOrderCmd;
    type Execution = CancelSpotOrderChanges;

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
        _envelope: &CommandEnvelope<ProductCommand>,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<Self::Execution, BuildBlockError> {
        execute_cancel_spot_order(command, &exchange_state.spot)
    }

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution) {
        for balance in &execution.released_balances {
            exchange_state.spot.balances.insert(
                AccountAssetKey::new(
                    balance.after.account_id.as_str(),
                    balance.after.asset_id.as_str(),
                ),
                balance.after.clone(),
            );
        }
        exchange_state.spot.orders.insert(
            execution.canceled_order.after.order_id.clone(),
            execution.canceled_order.after.clone(),
        );
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
) -> Result<CancelSpotOrderChanges, BuildBlockError> {
    CommandUseCase4::pre_check_command(&CancelSpotOrderUseCase, command)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let state = build_cancel_state(command, spot_state)?;
    CommandUseCase4::validate_against_state(&CancelSpotOrderUseCase, command, &state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let changes = CommandUseCase4::compute_changes(&CancelSpotOrderUseCase, command, state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;

    Ok(changes)
}
