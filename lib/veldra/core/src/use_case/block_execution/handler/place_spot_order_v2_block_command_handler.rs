use common_entity::{MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2Unchecked};
use example_core::{
    PlaceSpotOrderV2ChangesV3, PlaceSpotOrderV2CmdV3, PlaceSpotOrderV2TakerTemplateContextV3,
    SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3, SpotOrderV2GivenStateV3,
    SpotOrderV2UseCaseFamilyV3, build_place_spot_order_v2_taker_template_v3,
};

use crate::entity::{AccountAssetKey, CommandEnvelope, ExchangeState, ProductCommand, SpotState};
use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::BlockCommandHandler;

pub(in crate::use_case::block_execution) static PLACE_SPOT_ORDER_V2_BLOCK_COMMAND_HANDLER:
    PlaceSpotOrderV2BlockCommandHandler = PlaceSpotOrderV2BlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct PlaceSpotOrderV2BlockCommandHandler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::use_case::block_execution) struct PlaceSpotOrderV2ExecutionBundle {
    pub changes: PlaceSpotOrderV2ChangesV3,
    pub next_order_sequence: u64,
}

impl BlockCommandHandler for PlaceSpotOrderV2BlockCommandHandler {
    type Command = PlaceSpotOrderV2CmdV3;
    type Execution = PlaceSpotOrderV2ExecutionBundle;

    fn validate(
        &self,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<(), BuildBlockError> {
        let family = SpotOrderV2UseCaseFamilyV3;
        let cmd = SpotOrderV2CommandV3::Place(command.clone());
        let state = build_place_state(command, &exchange_state.spot)?;
        family
            .pre_check_command(&cmd)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
        family
            .validate_against_given_state(&cmd, &state)
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
        let account_id = execution.changes.updated_taker_order.after.account_id.as_str();
        exchange_state
            .spot
            .next_order_sequence_by_account
            .insert(account_id.to_string(), execution.next_order_sequence);
        exchange_state.spot.orders.insert(
            execution.changes.updated_taker_order.after.order_id.clone(),
            execution.changes.updated_taker_order.after.clone(),
        );
        for pair in &execution.changes.updated_maker_orders {
            exchange_state.spot.orders.insert(pair.after.order_id.clone(), pair.after.clone());
        }
        for pair in &execution.changes.updated_balances {
            exchange_state.spot.balances.insert(
                AccountAssetKey::new(pair.after.account_id.as_str(), pair.after.asset_id.as_str()),
                pair.after.clone(),
            );
        }
        for order in std::iter::once(&execution.changes.updated_taker_order.after)
            .chain(execution.changes.updated_maker_orders.iter().map(|pair| &pair.after))
        {
            exchange_state
                .spot
                .reservations
                .insert(order.reservation.reservation_id.clone(), order.reservation.clone());
            exchange_state
                .spot
                .reservations
                .insert(order.fee_reservation.reservation_id.clone(), order.fee_reservation.clone());
        }
    }
}

fn execute_place_spot_order_v2(
    command: &PlaceSpotOrderV2CmdV3,
    spot_state: &SpotState,
) -> Result<PlaceSpotOrderV2ExecutionBundle, BuildBlockError> {
    let family = SpotOrderV2UseCaseFamilyV3;
    let cmd = SpotOrderV2CommandV3::Place(command.clone());
    let state = build_place_state(command, spot_state)?;
    family
        .pre_check_command(&cmd)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    family
        .validate_against_given_state(&cmd, &state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let after = family
        .compute_after_changes_unchecked(&cmd, &state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let SpotOrderV2CaseChangesV3::Place(changes) =
        SpotOrderV2UseCaseFamilyV3::merge_before_and_after(state, after)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?
    else {
        return Err(BuildBlockError::SpotExecution("unexpected spot order v2 branch".to_string()));
    };
    let next_order_sequence = spot_state
        .next_order_sequence_by_account
        .get(command.party_id.as_str())
        .copied()
        .ok_or_else(|| BuildBlockError::MissingSpotOrderSequence {
            account_id: command.party_id.clone(),
        })?
        .checked_add(1)
        .ok_or_else(|| {
            BuildBlockError::SpotExecution("spot order sequence overflow".to_string())
        })?;

    Ok(PlaceSpotOrderV2ExecutionBundle { changes, next_order_sequence })
}

fn build_place_state(
    command: &PlaceSpotOrderV2CmdV3,
    spot_state: &SpotState,
) -> Result<SpotOrderV2GivenStateV3, BuildBlockError> {
    let symbol = spot_state
        .symbol_by_asset
        .get(&command.asset)
        .ok_or_else(|| BuildBlockError::MissingSpotAssetSymbol { asset: command.asset })?;
    let account_id = command.party_id.as_str();
    let market_rules =
        spot_state.market_rules_by_symbol.get(symbol).cloned().ok_or_else(|| {
            BuildBlockError::MissingSpotMarketRules { symbol: symbol.to_string() }
        })?;
    let asset_pair = spot_state
        .asset_pairs_by_symbol
        .get(symbol)
        .ok_or_else(|| BuildBlockError::MissingSpotAssetPair { symbol: symbol.to_string() })?;
    let trading_enabled = *spot_state
        .trading_enabled_by_symbol
        .get(symbol)
        .ok_or_else(|| BuildBlockError::MissingSpotTradingRuntime { symbol: symbol.to_string() })?;
    if !trading_enabled {
        return Err(BuildBlockError::SpotExecution("trading is disabled".to_string()));
    }
    let base_balance = spot_state
        .balances
        .get(&AccountAssetKey::new(account_id, asset_pair.base_asset_id.as_str()))
        .cloned()
        .ok_or_else(|| BuildBlockError::MissingSpotBalance {
            account_id: account_id.to_string(),
            asset_id: asset_pair.base_asset_id.clone(),
        })?;
    let quote_balance = spot_state
        .balances
        .get(&AccountAssetKey::new(account_id, asset_pair.quote_asset_id.as_str()))
        .cloned()
        .ok_or_else(|| BuildBlockError::MissingSpotBalance {
            account_id: account_id.to_string(),
            asset_id: asset_pair.quote_asset_id.clone(),
        })?;
    let next_order_sequence =
        *spot_state.next_order_sequence_by_account.get(account_id).ok_or_else(|| {
            BuildBlockError::MissingSpotOrderSequence { account_id: account_id.to_string() }
        })?;

    let mut settlement_balances = spot_state.balances.values().cloned().collect::<Vec<_>>();
    let fee_account_id = "fee".to_string();
    if !settlement_balances
        .iter()
        .any(|balance| balance.account_id == fee_account_id && balance.asset_id == asset_pair.quote_asset_id)
    {
        settlement_balances.push(example_core::Balance::new(
            fee_account_id.clone(),
            asset_pair.quote_asset_id.clone(),
            0,
            0,
            1,
        ));
    }
    let taker_order = build_place_spot_order_v2_taker_template_v3(
        command,
        PlaceSpotOrderV2TakerTemplateContextV3 {
            order_id: format!("{}-{}-{}", command.party_id, symbol, next_order_sequence),
            symbol: market_rules.symbol.clone(),
            settlement_balances: &settlement_balances,
            base_asset_id: asset_pair.base_asset_id.clone(),
            quote_asset_id: asset_pair.quote_asset_id.clone(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        },
    )
    .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let maker_orders = spot_state
        .orders
        .values()
        .filter(|order| {
            order.trades_asset(command.asset)
                && order.trades_symbol(symbol)
                && order.side() != taker_order.side()
                && matches!(
                    order.status(),
                    example_core::SpotOrderStatus::Open
                        | example_core::SpotOrderStatus::PartiallyFilled
                )
        })
        .cloned()
        .collect();

    let _ = (base_balance, quote_balance);
    Ok(SpotOrderV2GivenStateV3::Place {
        taker_order,
        maker_orders,
        settlement_balances,
        base_asset_id: asset_pair.base_asset_id.clone(),
        quote_asset_id: asset_pair.quote_asset_id.clone(),
        fee_account_id,
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    })
}
