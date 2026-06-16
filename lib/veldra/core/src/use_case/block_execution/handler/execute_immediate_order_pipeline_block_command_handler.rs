use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};
use example_core::{
    Balance, ExecuteImmediateSpotOrderPipelineChanges, ExecuteImmediateSpotOrderPipelineCmd,
    MatchSpotOrderChanges, MatchSpotOrderCmd, MatchSpotOrderState, MatchSpotOrderUseCase,
    PlaceImmediateOrderChanges, PlaceImmediateOrderState, PlaceImmediateOrderUseCase,
    SettleSpotTradeCmd, SettleSpotTradeState, SettleSpotTradeUseCase, SpotOrder, SpotOrderSide,
    SpotOrderTimeInForce,
};

use crate::entity::{
    AccountAssetKey, CommandEnvelope, CommandExecutionResult, ExchangeState, ProductCommand,
    ProductCommandResult, SpotCommandResult, SpotPipelineExecution, SpotState,
};
use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::block_command_handler::{
    BlockCommandHandler, normalize_local_events, rebase_events,
};

pub(in crate::use_case::block_execution) static EXECUTE_IMMEDIATE_ORDER_PIPELINE_BLOCK_COMMAND_HANDLER:
    ExecuteImmediateOrderPipelineBlockCommandHandler =
    ExecuteImmediateOrderPipelineBlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct ExecuteImmediateOrderPipelineBlockCommandHandler;

impl BlockCommandHandler for ExecuteImmediateOrderPipelineBlockCommandHandler {
    type Command = ExecuteImmediateSpotOrderPipelineCmd;
    type Execution = SpotPipelineExecution;

    fn validate(
        &self,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<(), BuildBlockError> {
        let _ = build_place_state(command, &exchange_state.spot)?;
        Ok(())
    }

    fn execute(
        &self,
        envelope: &CommandEnvelope<ProductCommand>,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<CommandExecutionResult, BuildBlockError> {
        let execution = execute_immediate_spot_pipeline(command, &exchange_state.spot)?;
        Ok(CommandExecutionResult {
            command_id: envelope.command_id.clone(),
            command_kind: "spot".to_string(),
            command_commitment: envelope.commitment(),
            result: ProductCommandResult::Spot(SpotCommandResult::ExecuteImmediateOrderPipeline(
                execution,
            )),
        })
    }

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution) {
        let account_id = execution.pipeline_output.place_output.order.account_id.as_str();
        exchange_state
            .spot
            .next_order_sequence_by_account
            .insert(account_id.to_string(), execution.next_order_sequence);
        for balance in &execution.balances_after {
            exchange_state.spot.balances.insert(
                AccountAssetKey::new(balance.account_id.as_str(), balance.asset_id.as_str()),
                balance.clone(),
            );
        }
        for order in &execution.orders_after {
            exchange_state.spot.orders.insert(order.order_id.clone(), order.clone());
        }
        for trade_id in &execution.settled_trade_ids_appended {
            exchange_state.spot.settled_trade_ids.insert(trade_id.clone());
        }
    }

    fn events<'a>(&self, execution: &'a Self::Execution) -> &'a [EntityReplayableEvent] {
        execution.events.as_slice()
    }

    fn rebase_events(&self, execution: &mut Self::Execution, base_sequence: u64) {
        execution.events = rebase_events(&execution.events, base_sequence);
    }
}

fn execute_immediate_spot_pipeline(
    command: &ExecuteImmediateSpotOrderPipelineCmd,
    spot_state: &SpotState,
) -> Result<SpotPipelineExecution, BuildBlockError> {
    let place_state = build_place_state(command, spot_state)?;
    CommandUseCase4::pre_check_command(&PlaceImmediateOrderUseCase, &command.place)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    CommandUseCase4::validate_against_state(
        &PlaceImmediateOrderUseCase,
        &command.place,
        &place_state,
    )
    .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let place_changes =
        CommandUseCase4::compute_changes(&PlaceImmediateOrderUseCase, &command.place, place_state)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;

    let mut all_events = place_changes
        .to_replayable_events()
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let place_output = place_changes.clone();
    let taker_order = place_changes.order.clone();
    let affected_balance_after = place_changes.affected_balance_after.clone();
    let next_order_sequence = spot_state
        .next_order_sequence_by_account
        .get(command.place.party_id.as_str())
        .copied()
        .ok_or_else(|| BuildBlockError::MissingSpotOrderSequence {
            account_id: command.place.party_id.clone(),
        })?
        .checked_add(1)
        .ok_or_else(|| {
            BuildBlockError::SpotExecution("spot order sequence overflow".to_string())
        })?;

    let maker_orders = sorted_maker_orders(command, spot_state);
    if !should_enter_matching(&taker_order, &maker_orders) {
        return Ok(build_pipeline_without_match(
            place_output,
            affected_balance_after,
            taker_order,
            next_order_sequence,
            all_events,
        ));
    }

    let match_cmd = MatchSpotOrderCmd {
        party_id: command.place.party_id.clone(),
        taker_order_id: taker_order.order_id.clone(),
        match_id: command.match_id.clone(),
    };
    let match_state = MatchSpotOrderState { taker_order, maker_orders };
    CommandUseCase4::pre_check_command(&MatchSpotOrderUseCase, &match_cmd)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    CommandUseCase4::validate_against_state(&MatchSpotOrderUseCase, &match_cmd, &match_state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let match_changes =
        CommandUseCase4::compute_changes(&MatchSpotOrderUseCase, &match_cmd, match_state)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    all_events.extend(
        match_changes
            .to_replayable_events()
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?,
    );

    let match_output = match_changes.clone();
    let trades = match_changes.trades.clone();
    let taker_order_after = match_changes.taker_order_after.clone();
    let maker_orders_after = match_changes.maker_orders_after.clone();
    if trades.is_empty() {
        return Ok(build_pipeline_without_settlement(
            place_output,
            match_output,
            affected_balance_after,
            taker_order_after,
            maker_orders_after,
            next_order_sequence,
            all_events,
        ));
    }

    let asset_pair =
        spot_state.asset_pairs_by_symbol.get(command.place.symbol.as_str()).ok_or_else(|| {
            BuildBlockError::MissingSpotAssetPair { symbol: command.place.symbol.clone() }
        })?;
    let settle_cmd = SettleSpotTradeCmd {
        party_id: command.place.party_id.clone(),
        settlement_batch_id: command.settlement_batch_id.clone(),
        trade_ids: trades.iter().map(|trade| trade.trade_id.clone()).collect(),
    };
    let settle_state = SettleSpotTradeState {
        trades: trades.clone(),
        base_asset_id: asset_pair.base_asset_id.clone(),
        quote_asset_id: asset_pair.quote_asset_id.clone(),
        balances: settlement_balances_after_place(
            spot_state.balances.values().cloned().collect(),
            affected_balance_after.clone(),
        ),
        settled_trade_ids: spot_state.settled_trade_ids.iter().cloned().collect(),
    };
    CommandUseCase4::pre_check_command(&SettleSpotTradeUseCase, &settle_cmd)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    CommandUseCase4::validate_against_state(&SettleSpotTradeUseCase, &settle_cmd, &settle_state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let settle_changes =
        CommandUseCase4::compute_changes(&SettleSpotTradeUseCase, &settle_cmd, settle_state)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    all_events.extend(
        settle_changes
            .to_replayable_events()
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?,
    );

    let settlements = settle_changes.settlements.clone();
    let balances_after = settle_changes.balances_after.clone();
    let settled_trade_ids_appended =
        settlements.iter().map(|settlement| settlement.trade_id.clone()).collect::<Vec<_>>();

    Ok(build_pipeline_with_settlement(
        place_output,
        match_output,
        balances_after,
        taker_order_after,
        maker_orders_after,
        trades,
        settlements,
        settled_trade_ids_appended,
        next_order_sequence,
        all_events,
    ))
}

fn build_place_state(
    command: &ExecuteImmediateSpotOrderPipelineCmd,
    spot_state: &SpotState,
) -> Result<PlaceImmediateOrderState, BuildBlockError> {
    let symbol = command.place.symbol.as_str();
    let account_id = command.place.party_id.as_str();
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

    Ok(PlaceImmediateOrderState {
        trading_enabled,
        next_order_sequence,
        account_id: account_id.to_string(),
        base_balance,
        quote_balance,
        market_rules,
    })
}

fn sorted_maker_orders(
    command: &ExecuteImmediateSpotOrderPipelineCmd,
    spot_state: &SpotState,
) -> Vec<SpotOrder> {
    let mut maker_orders = spot_state
        .orders
        .values()
        .filter(|order| {
            order.trades_symbol(command.place.symbol.as_str()) && order.can_be_cancelled()
        })
        .cloned()
        .collect::<Vec<_>>();
    maker_orders.sort_by(|left, right| compare_maker_orders(left, right, command.place.is_buy));
    maker_orders
}

fn compare_maker_orders(
    left: &SpotOrder,
    right: &SpotOrder,
    taker_is_buy: bool,
) -> std::cmp::Ordering {
    let left_price = left.limit_price().unwrap_or(if taker_is_buy { u64::MAX } else { 0 });
    let right_price = right.limit_price().unwrap_or(if taker_is_buy { u64::MAX } else { 0 });
    let price_order =
        if taker_is_buy { left_price.cmp(&right_price) } else { right_price.cmp(&left_price) };
    price_order.then_with(|| left.order_id.cmp(&right.order_id))
}

fn should_enter_matching(taker_order: &SpotOrder, maker_orders: &[SpotOrder]) -> bool {
    if matches!(taker_order.time_in_force, SpotOrderTimeInForce::Ioc) {
        return true;
    }
    let Some(best_maker) = maker_orders.first() else {
        return false;
    };
    let Some(maker_price) = best_maker.limit_price() else {
        return true;
    };
    match taker_order.side {
        SpotOrderSide::Buy => taker_order.order_price() >= maker_price,
        SpotOrderSide::Sell => taker_order.order_price() <= maker_price,
    }
}

fn settlement_balances_after_place(
    mut balances: Vec<Balance>,
    affected_balance_after: Balance,
) -> Vec<Balance> {
    if let Some(balance) = balances.iter_mut().find(|balance| {
        balance.account_id == affected_balance_after.account_id
            && balance.asset_id == affected_balance_after.asset_id
    }) {
        *balance = affected_balance_after;
        return balances;
    }
    balances.push(affected_balance_after);
    balances
}

fn build_pipeline_without_match(
    place_output: PlaceImmediateOrderChanges,
    affected_balance_after: Balance,
    taker_order: SpotOrder,
    next_order_sequence: u64,
    events: Vec<EntityReplayableEvent>,
) -> SpotPipelineExecution {
    SpotPipelineExecution {
        pipeline_output: ExecuteImmediateSpotOrderPipelineChanges {
            place_output,
            match_output: None,
            settle_changes: None,
        },
        balances_after: vec![affected_balance_after],
        orders_after: vec![taker_order],
        trades: Vec::new(),
        settlements: Vec::new(),
        settled_trade_ids_appended: Vec::new(),
        next_order_sequence,
        events: normalize_local_events(events),
    }
}

fn build_pipeline_without_settlement(
    place_output: PlaceImmediateOrderChanges,
    match_output: MatchSpotOrderChanges,
    affected_balance_after: Balance,
    taker_order_after: SpotOrder,
    maker_orders_after: Vec<SpotOrder>,
    next_order_sequence: u64,
    events: Vec<EntityReplayableEvent>,
) -> SpotPipelineExecution {
    SpotPipelineExecution {
        pipeline_output: ExecuteImmediateSpotOrderPipelineChanges {
            place_output,
            match_output: Some(match_output),
            settle_changes: None,
        },
        balances_after: vec![affected_balance_after],
        orders_after: collect_orders_after(taker_order_after, maker_orders_after),
        trades: Vec::new(),
        settlements: Vec::new(),
        settled_trade_ids_appended: Vec::new(),
        next_order_sequence,
        events: normalize_local_events(events),
    }
}

fn build_pipeline_with_settlement(
    place_output: PlaceImmediateOrderChanges,
    match_output: MatchSpotOrderChanges,
    balances_after: Vec<Balance>,
    taker_order_after: SpotOrder,
    maker_orders_after: Vec<SpotOrder>,
    trades: Vec<example_core::SpotTrade>,
    settlements: Vec<example_core::SpotSettlement>,
    settled_trade_ids_appended: Vec<String>,
    next_order_sequence: u64,
    events: Vec<EntityReplayableEvent>,
) -> SpotPipelineExecution {
    SpotPipelineExecution {
        pipeline_output: ExecuteImmediateSpotOrderPipelineChanges {
            place_output,
            match_output: Some(match_output),
            settle_changes: None,
        },
        balances_after,
        orders_after: collect_orders_after(taker_order_after, maker_orders_after),
        trades,
        settlements,
        settled_trade_ids_appended,
        next_order_sequence,
        events: normalize_local_events(events),
    }
}

fn collect_orders_after(
    taker_order_after: SpotOrder,
    maker_orders_after: Vec<SpotOrder>,
) -> Vec<SpotOrder> {
    let mut orders = Vec::with_capacity(1 + maker_orders_after.len());
    orders.push(taker_order_after);
    orders.extend(maker_orders_after);
    orders
}
