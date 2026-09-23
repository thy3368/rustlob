use common_entity::{ExecutionContext, StateMachineOwnedV2Diff, StateMachineV2Unchecked};
use example_core_use_case::{
    PlaceMatchSpotOrderV2Changes, PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd, SpotOrderV2,
};
use veldra_core_entity::{
    AccountAssetKey, CommandEnvelope, ExchangeState, ProductCommand, SpotState,
};

use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::BlockCommandHandler;

pub(in crate::use_case::block_execution) static PLACE_SPOT_ORDER_V2_BLOCK_COMMAND_HANDLER:
    PlaceSpotOrderV2BlockCommandHandler = PlaceSpotOrderV2BlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct PlaceSpotOrderV2BlockCommandHandler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::use_case::block_execution) struct PlaceSpotOrderV2ExecutionBundle {
    pub changes: PlaceMatchSpotOrderV2Changes,
    pub next_order_sequence: u64,
}

impl BlockCommandHandler for PlaceSpotOrderV2BlockCommandHandler {
    type Command = PlaceOnlySpotOrderV2Cmd;
    type Execution = PlaceSpotOrderV2ExecutionBundle;

    fn validate(
        &self,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<(), BuildBlockError> {
        let state = build_place_state(command, &exchange_state.spot)?;
        PlaceMatchSpotOrderV2UseCase
            .check_command(command)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
        PlaceMatchSpotOrderV2UseCase
            .validate_state_given(command, &state)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))
    }

    fn execute(
        &self,
        _envelope: &CommandEnvelope<ProductCommand>,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<Self::Execution, BuildBlockError> {
        execute_place_spot_order_v2(command, &exchange_state.spot)
    }

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution) {
        apply_place_changes(&mut exchange_state.spot, &execution.changes);
        let account_id = command_account_id_from_changes(&execution.changes);
        exchange_state
            .spot
            .next_order_sequence_by_account
            .insert(account_id.to_string(), execution.next_order_sequence);
    }
}

fn execute_place_spot_order_v2(
    command: &PlaceOnlySpotOrderV2Cmd,
    spot_state: &SpotState,
) -> Result<PlaceSpotOrderV2ExecutionBundle, BuildBlockError> {
    let state = build_place_state(command, spot_state)?;
    PlaceMatchSpotOrderV2UseCase
        .check_command(command)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    PlaceMatchSpotOrderV2UseCase
        .validate_state_given(command, &state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let changes = PlaceMatchSpotOrderV2UseCase
        .compute_state_diff_with_context(command, state, &ExecutionContext::now())
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;

    let account_id = parent_order(command).party_id.clone();
    let next_order_sequence = spot_state
        .next_order_sequence_by_account
        .get(account_id.as_str())
        .copied()
        .ok_or_else(|| BuildBlockError::MissingSpotOrderSequence { account_id })?
        .checked_add(1)
        .ok_or_else(|| {
            BuildBlockError::SpotExecution("spot order sequence overflow".to_string())
        })?;

    Ok(PlaceSpotOrderV2ExecutionBundle { changes, next_order_sequence })
}

fn build_place_state(
    command: &PlaceOnlySpotOrderV2Cmd,
    spot_state: &SpotState,
) -> Result<PlaceMatchSpotOrderV2State, BuildBlockError> {
    let order = parent_order(command);
    let symbol = spot_state
        .symbol_by_asset
        .get(&order.asset)
        .ok_or(BuildBlockError::MissingSpotAssetSymbol { asset: order.asset })?;
    let market_rules = spot_state
        .market_rules_by_symbol
        .get(symbol)
        .ok_or_else(|| BuildBlockError::MissingSpotMarketRules { symbol: symbol.clone() })?;
    if !spot_state
        .trading_enabled_by_symbol
        .get(symbol)
        .copied()
        .ok_or_else(|| BuildBlockError::MissingSpotTradingRuntime { symbol: symbol.clone() })?
    {
        return Err(BuildBlockError::SpotExecution("trading is disabled".to_string()));
    }
    if market_rules.symbol != order.symbol {
        return Err(BuildBlockError::SpotExecution(
            "spot symbol does not match market rules".to_string(),
        ));
    }

    let asset_pair = spot_state
        .asset_pairs_by_symbol
        .get(symbol)
        .ok_or_else(|| BuildBlockError::MissingSpotAssetPair { symbol: symbol.clone() })?;
    for asset_id in [&asset_pair.base_asset_id, &asset_pair.quote_asset_id] {
        spot_state
            .balances
            .get(&AccountAssetKey::new(order.party_id.as_str(), asset_id.as_str()))
            .ok_or_else(|| BuildBlockError::MissingSpotBalance {
                account_id: order.party_id.clone(),
                asset_id: asset_id.to_string(),
            })?;
    }

    let fee_account_id = "fee".to_string();
    let mut settlement_balances = spot_state.balances.values().cloned().collect::<Vec<_>>();
    if !settlement_balances.iter().any(|balance| {
        balance.account_id == fee_account_id && balance.asset_id == asset_pair.quote_asset_id
    }) {
        settlement_balances.push(example_core_use_case::Balance::new(
            fee_account_id.clone(),
            asset_pair.quote_asset_id.clone(),
            0,
            0,
            1,
        ));
    }

    let taker_side = if order.is_buy {
        example_core_use_case::SpotOrderSide::Buy
    } else {
        example_core_use_case::SpotOrderSide::Sell
    };
    let maker_orders = spot_state
        .orders
        .values()
        .filter(|maker| {
            maker.trades_asset(order.asset)
                && maker.trades_symbol(symbol)
                && maker.side() != taker_side
                && matches!(
                    maker.status(),
                    example_core_use_case::SpotOrderStatus::Open
                        | example_core_use_case::SpotOrderStatus::PartiallyFilled
                )
        })
        .cloned()
        .collect();

    Ok(PlaceMatchSpotOrderV2State { maker_orders, settlement_balances, fee_account_id })
}

fn parent_order(command: &PlaceOnlySpotOrderV2Cmd) -> &PlaceOnlySpotOrderV2OrderCmd {
    match command {
        PlaceOnlySpotOrderV2Cmd::Single(order)
        | PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent: order, .. } => order,
    }
}

fn command_account_id_from_changes(changes: &PlaceMatchSpotOrderV2Changes) -> &str {
    match changes {
        PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order } => {
            created_order.account_id.as_str()
        }
        PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched { created_taker_order, .. } => {
            created_taker_order.account_id.as_str()
        }
        PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
            created_parent_order, ..
        } => created_parent_order.account_id.as_str(),
    }
}

fn apply_place_changes(spot_state: &mut SpotState, changes: &PlaceMatchSpotOrderV2Changes) {
    match changes {
        PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order } => {
            apply_order(spot_state, created_order);
        }
        PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
            created_taker_order,
            match_changes,
        } => {
            apply_order(spot_state, created_taker_order);
            apply_match_changes(spot_state, match_changes);
        }
        PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
            created_parent_order,
            created_child_orders,
            match_changes,
        } => {
            apply_order(spot_state, created_parent_order);
            for child in created_child_orders {
                apply_order(spot_state, child);
            }
            apply_match_changes(spot_state, match_changes);
        }
    }
}

fn apply_match_changes(
    spot_state: &mut SpotState,
    changes: &example_core_use_case::MatchSpotOrderV2Changes,
) {
    if let Some(pair) = &changes.updated_taker_order {
        apply_order(spot_state, &pair.after);
    }
    for pair in &changes.updated_maker_orders {
        apply_order(spot_state, &pair.after);
    }
    for pair in &changes.updated_balances {
        spot_state.balances.insert(
            AccountAssetKey::new(pair.after.account_id.as_str(), pair.after.asset_id.as_str()),
            pair.after.clone(),
        );
    }
}

fn apply_order(spot_state: &mut SpotState, order: &SpotOrderV2) {
    spot_state.orders.insert(order.order_id.clone(), order.clone());
    spot_state
        .reservations
        .insert(order.reservation.reservation_id.clone(), order.reservation.clone());
    spot_state
        .reservations
        .insert(order.fee_reservation.reservation_id.clone(), order.fee_reservation.clone());
}
