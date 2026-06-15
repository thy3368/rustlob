use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase2, CommandUseCase3, UseCaseOutput};
use example_core::{
    Balance, CancelSpotOrderCmd, CancelSpotOrderState, CancelSpotOrderUseCase, DepositQuoteCmd,
    DepositQuoteState, DepositQuoteUseCase, ExecuteImmediateSpotOrderPipelineCmd,
    ExecuteImmediateSpotOrderPipelineOutput, MatchSpotOrderCmd, MatchSpotOrderOutput,
    MatchSpotOrderState, MatchSpotOrderUseCase, PlaceImmediateOrderOutput,
    PlaceImmediateOrderState, PlaceImmediateOrderUseCase, SettleSpotTradeCmd,
    SettleSpotTradeOutput, SettleSpotTradeState, SettleSpotTradeUseCase, SpotOrder, SpotOrderSide,
    SpotOrderTimeInForce, WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteUseCase,
};

use super::{
    BuildBlockError, BuildBlockFromCommandsCommand, BuildBlockFromCommandsOutput,
    BuildBlockFromCommandsState,
};
use crate::entity::{
    AccountAssetKey, CommandEnvelope, CommandExecutionResult, ExchangeState, ProductCommand,
    ProductCommandResult, SpotCancelExecution, SpotCommand, SpotCommandResult,
    SpotPipelineExecution, SpotState, TreasuryBalanceUpdate, TreasuryCommand,
    TreasuryCommandResult, TreasuryState, build_new_block,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct BuildBlockFromCommandsUseCase;

impl CommandUseCase3 for BuildBlockFromCommandsUseCase {
    type Command = BuildBlockFromCommandsCommand;
    type GivenState = BuildBlockFromCommandsState;
    type Error = BuildBlockError;
    type Output = BuildBlockFromCommandsOutput;

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
        validate_batch_commands(&state.commands, &state.exchange_state.spot)
    }

    fn compute_output_and_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {
        let BuildBlockFromCommandsState { parent_block_hash, mut exchange_state, commands, .. } =
            state;
        let (command_results, events) = execute_batch_commands(&commands, &mut exchange_state)?;
        let output = build_block_output(
            cmd.block_height,
            parent_block_hash,
            commands,
            events.clone(),
            exchange_state,
            command_results,
        );

        Ok(UseCaseOutput { output, events })
    }
}

fn validate_batch_commands(
    commands: &[CommandEnvelope<ProductCommand>],
    spot_state: &SpotState,
) -> Result<(), BuildBlockError> {
    for envelope in commands {
        validate_command_against_exchange_state(&envelope.command, spot_state)?;
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
        // 先基于当前 exchange state 执行单条命令，拿到局部结果和局部事件。
        let mut result = execute_product_command(envelope, exchange_state)?;
        // 再把局部事件 rebasing 到整个 batch 的全局 sequence 空间。
        rebase_command_result_events(&mut result, next_sequence);
        next_sequence += command_result_events_len(&result) as u64;
        // 汇总事件后，再把命令结果 apply 回 exchange state，供后续命令串行依赖。
        events.extend(command_result_events(&result).iter().cloned());
        apply_command_result(exchange_state, &result);
        command_results.push(result);
    }

    Ok((command_results, events))
}

fn build_block_output(
    block_height: u64,
    parent_block_hash: String,
    commands: Vec<CommandEnvelope<ProductCommand>>,
    events: Vec<EntityReplayableEvent>,
    exchange_state: ExchangeState,
    command_results: Vec<CommandExecutionResult>,
) -> BuildBlockFromCommandsOutput {
    let new_block =
        build_new_block(block_height, parent_block_hash, &commands, &events, &exchange_state);

    BuildBlockFromCommandsOutput { new_block, command_results, exchange_state }
}

fn validate_command_against_exchange_state(
    command: &ProductCommand,
    spot_state: &SpotState,
) -> Result<(), BuildBlockError> {
    match command {
        ProductCommand::Spot(command) => validate_spot_command(command, spot_state),
        ProductCommand::Perp(_) => Err(BuildBlockError::UnsupportedPerpCommand),
        ProductCommand::Treasury(command) => validate_treasury_command(command, spot_state),
    }
}

fn validate_spot_command(
    command: &SpotCommand,
    spot_state: &SpotState,
) -> Result<(), BuildBlockError> {
    match command {
        SpotCommand::ExecuteImmediateOrderPipeline(command) => {
            let symbol = command.place.symbol.as_str();
            let account_id = command.place.party_id.as_str();
            let rules = spot_state.market_rules_by_symbol.get(symbol).ok_or_else(|| {
                BuildBlockError::MissingSpotMarketRules { symbol: symbol.to_string() }
            })?;
            let pair = spot_state.asset_pairs_by_symbol.get(symbol).ok_or_else(|| {
                BuildBlockError::MissingSpotAssetPair { symbol: symbol.to_string() }
            })?;
            let _ = spot_state.trading_enabled_by_symbol.get(symbol).ok_or_else(|| {
                BuildBlockError::MissingSpotTradingRuntime { symbol: symbol.to_string() }
            })?;
            let _ = spot_state.next_order_sequence_by_account.get(account_id).ok_or_else(|| {
                BuildBlockError::MissingSpotOrderSequence { account_id: account_id.to_string() }
            })?;
            let _ = spot_state
                .balances
                .get(&AccountAssetKey::new(account_id, pair.base_asset_id.as_str()))
                .ok_or_else(|| BuildBlockError::MissingSpotBalance {
                    account_id: account_id.to_string(),
                    asset_id: pair.base_asset_id.clone(),
                })?;
            let _ = spot_state
                .balances
                .get(&AccountAssetKey::new(account_id, pair.quote_asset_id.as_str()))
                .ok_or_else(|| BuildBlockError::MissingSpotBalance {
                    account_id: account_id.to_string(),
                    asset_id: pair.quote_asset_id.clone(),
                })?;
            let _ = rules;
            Ok(())
        }
        SpotCommand::CancelOrder(command) => {
            let _ = build_cancel_state(command, spot_state)?;
            Ok(())
        }
    }
}

fn validate_treasury_command(
    command: &TreasuryCommand,
    _spot_state: &SpotState,
) -> Result<(), BuildBlockError> {
    match command {
        TreasuryCommand::DepositQuote(_) => Ok(()),
        TreasuryCommand::WithdrawQuote(_) => Ok(()),
    }
}

fn execute_product_command(
    envelope: &CommandEnvelope<ProductCommand>,
    exchange_state: &ExchangeState,
) -> Result<CommandExecutionResult, BuildBlockError> {
    let (command_kind, result) = match &envelope.command {
        ProductCommand::Spot(command) => (
            "spot",
            ProductCommandResult::Spot(execute_spot_command(command, &exchange_state.spot)?),
        ),
        ProductCommand::Treasury(command) => (
            "treasury",
            ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(
                execute_treasury_command(command, &exchange_state.treasury)?,
            )),
        ),
        ProductCommand::Perp(_) => return Err(BuildBlockError::UnsupportedPerpCommand),
    };

    Ok(CommandExecutionResult {
        command_id: envelope.command_id.clone(),
        command_kind: command_kind.to_string(),
        command_commitment: envelope.commitment(),
        result,
    })
}

fn execute_spot_command(
    command: &SpotCommand,
    spot_state: &SpotState,
) -> Result<SpotCommandResult, BuildBlockError> {
    match command {
        SpotCommand::ExecuteImmediateOrderPipeline(command) => {
            execute_immediate_spot_pipeline(command, spot_state)
                .map(SpotCommandResult::ExecuteImmediateOrderPipeline)
        }
        SpotCommand::CancelOrder(command) => {
            execute_cancel_spot_order(command, spot_state).map(SpotCommandResult::CancelOrder)
        }
    }
}

fn execute_treasury_command(
    command: &TreasuryCommand,
    treasury_state: &TreasuryState,
) -> Result<TreasuryBalanceUpdate, BuildBlockError> {
    match command {
        TreasuryCommand::DepositQuote(command) => execute_deposit_quote(command, treasury_state),
        TreasuryCommand::WithdrawQuote(command) => execute_withdraw_quote(command, treasury_state),
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

fn execute_withdraw_quote(
    command: &WithdrawQuoteCmd,
    treasury_state: &TreasuryState,
) -> Result<TreasuryBalanceUpdate, BuildBlockError> {
    let balance = treasury_quote_balance(treasury_state, command.party_id.as_str())?;
    CommandUseCase2::pre_check_command(&WithdrawQuoteUseCase, command)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    let state = WithdrawQuoteState { quote_balance: balance.clone() };
    CommandUseCase2::validate_against_state(&WithdrawQuoteUseCase, command, &state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;
    let events = CommandUseCase2::compute_replayable_events(&WithdrawQuoteUseCase, command, state)
        .map_err(|error| BuildBlockError::TreasuryExecution(error.to_string()))?;

    let mut balance_after = balance;
    balance_after.apply_after(
        balance_after.available.checked_sub(command.amount).ok_or_else(|| {
            BuildBlockError::TreasuryExecution("treasury balance underflow".to_string())
        })?,
        balance_after.frozen,
        balance_after.version + 1,
    );
    Ok(TreasuryBalanceUpdate { balance_after, events: normalize_local_events(events) })
}

fn treasury_quote_balance(
    treasury_state: &TreasuryState,
    account_id: &str,
) -> Result<Balance, BuildBlockError> {
    treasury_state.balances.get(&AccountAssetKey::new(account_id, "USDT")).cloned().ok_or_else(
        || BuildBlockError::MissingTreasuryBalance {
            account_id: account_id.to_string(),
            asset_id: "USDT".to_string(),
        },
    )
}

fn execute_immediate_spot_pipeline(
    command: &ExecuteImmediateSpotOrderPipelineCmd,
    spot_state: &SpotState,
) -> Result<SpotPipelineExecution, BuildBlockError> {
    let place_state = build_place_state(command, spot_state)?;
    CommandUseCase3::pre_check_command(&PlaceImmediateOrderUseCase, &command.place)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    CommandUseCase3::validate_against_state(
        &PlaceImmediateOrderUseCase,
        &command.place,
        &place_state,
    )
    .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let place_result = CommandUseCase3::compute_output_and_events(
        &PlaceImmediateOrderUseCase,
        &command.place,
        place_state,
    )
    .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;

    let mut all_events = place_result.events;
    let place_output = place_result.output.clone();
    let PlaceImmediateOrderOutput { order: taker_order, affected_balance_after } =
        place_result.output;
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
    CommandUseCase3::pre_check_command(&MatchSpotOrderUseCase, &match_cmd)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    CommandUseCase3::validate_against_state(&MatchSpotOrderUseCase, &match_cmd, &match_state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let match_result =
        CommandUseCase3::compute_output_and_events(&MatchSpotOrderUseCase, &match_cmd, match_state)
            .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    all_events.extend(match_result.events);

    let match_output = match_result.output.clone();
    let MatchSpotOrderOutput { trades, taker_order_after, maker_orders_after } =
        match_result.output;
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
    CommandUseCase3::pre_check_command(&SettleSpotTradeUseCase, &settle_cmd)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    CommandUseCase3::validate_against_state(&SettleSpotTradeUseCase, &settle_cmd, &settle_state)
        .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    let settle_result = CommandUseCase3::compute_output_and_events(
        &SettleSpotTradeUseCase,
        &settle_cmd,
        settle_state,
    )
    .map_err(|error| BuildBlockError::SpotExecution(error.to_string()))?;
    all_events.extend(settle_result.events);

    let SettleSpotTradeOutput { settlements, balances_after } = settle_result.output;
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
    order_after.status = example_core::SpotOrderStatus::Canceled;
    order_after.status_reason = Some(example_core::SpotOrderStatusReason::CanceledByUser);
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
    place_output: PlaceImmediateOrderOutput,
    affected_balance_after: Balance,
    taker_order: SpotOrder,
    next_order_sequence: u64,
    events: Vec<EntityReplayableEvent>,
) -> SpotPipelineExecution {
    SpotPipelineExecution {
        pipeline_output: ExecuteImmediateSpotOrderPipelineOutput {
            place_output,
            match_output: None,
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
    place_output: PlaceImmediateOrderOutput,
    match_output: MatchSpotOrderOutput,
    affected_balance_after: Balance,
    taker_order_after: SpotOrder,
    maker_orders_after: Vec<SpotOrder>,
    next_order_sequence: u64,
    events: Vec<EntityReplayableEvent>,
) -> SpotPipelineExecution {
    SpotPipelineExecution {
        pipeline_output: ExecuteImmediateSpotOrderPipelineOutput {
            place_output,
            match_output: Some(match_output),
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
    place_output: PlaceImmediateOrderOutput,
    match_output: MatchSpotOrderOutput,
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
        pipeline_output: ExecuteImmediateSpotOrderPipelineOutput {
            place_output,
            match_output: Some(match_output),
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

fn apply_command_result(exchange_state: &mut ExchangeState, result: &CommandExecutionResult) {
    match &result.result {
        ProductCommandResult::Spot(result) => {
            apply_spot_execution(&mut exchange_state.spot, result)
        }
        ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(result)) => {
            apply_treasury_execution(&mut exchange_state.treasury, result)
        }
        ProductCommandResult::Perp(_) => {}
    }
}

fn apply_spot_execution(spot_state: &mut SpotState, result: &SpotCommandResult) {
    match result {
        SpotCommandResult::ExecuteImmediateOrderPipeline(result) => {
            let account_id = result.pipeline_output.place_output.order.account_id.as_str();
            spot_state
                .next_order_sequence_by_account
                .insert(account_id.to_string(), result.next_order_sequence);
            for balance in &result.balances_after {
                spot_state.balances.insert(
                    AccountAssetKey::new(balance.account_id.as_str(), balance.asset_id.as_str()),
                    balance.clone(),
                );
            }
            for order in &result.orders_after {
                spot_state.orders.insert(order.order_id.clone(), order.clone());
            }
            for trade_id in &result.settled_trade_ids_appended {
                spot_state.settled_trade_ids.insert(trade_id.clone());
            }
        }
        SpotCommandResult::CancelOrder(result) => {
            for balance in &result.balances_after {
                spot_state.balances.insert(
                    AccountAssetKey::new(balance.account_id.as_str(), balance.asset_id.as_str()),
                    balance.clone(),
                );
            }
            spot_state
                .orders
                .insert(result.order_after.order_id.clone(), result.order_after.clone());
        }
    }
}

fn apply_treasury_execution(treasury_state: &mut TreasuryState, result: &TreasuryBalanceUpdate) {
    treasury_state.balances.insert(
        AccountAssetKey::new(
            result.balance_after.account_id.as_str(),
            result.balance_after.asset_id.as_str(),
        ),
        result.balance_after.clone(),
    );
}

fn command_result_events(result: &CommandExecutionResult) -> &[EntityReplayableEvent] {
    match &result.result {
        ProductCommandResult::Spot(SpotCommandResult::ExecuteImmediateOrderPipeline(result)) => {
            result.events.as_slice()
        }
        ProductCommandResult::Spot(SpotCommandResult::CancelOrder(result)) => {
            result.events.as_slice()
        }
        ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(result)) => {
            result.events.as_slice()
        }
        ProductCommandResult::Perp(_) => &[],
    }
}

fn command_result_events_len(result: &CommandExecutionResult) -> usize {
    command_result_events(result).len()
}

fn rebase_command_result_events(result: &mut CommandExecutionResult, base_sequence: u64) {
    match &mut result.result {
        ProductCommandResult::Spot(SpotCommandResult::ExecuteImmediateOrderPipeline(result)) => {
            result.events = rebase_events(&result.events, base_sequence);
        }
        ProductCommandResult::Spot(SpotCommandResult::CancelOrder(result)) => {
            result.events = rebase_events(&result.events, base_sequence);
        }
        ProductCommandResult::Treasury(TreasuryCommandResult::QuoteBalanceUpdated(result)) => {
            result.events = rebase_events(&result.events, base_sequence);
        }
        ProductCommandResult::Perp(_) => {}
    }
}

fn normalize_local_events(events: Vec<EntityReplayableEvent>) -> Vec<EntityReplayableEvent> {
    events
        .into_iter()
        .enumerate()
        .map(|(index, mut event)| {
            event.timestamp = 0;
            event.sequence = index as u64;
            event
        })
        .collect()
}

fn rebase_events(
    events: &[EntityReplayableEvent],
    base_sequence: u64,
) -> Vec<EntityReplayableEvent> {
    events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            let mut cloned = event.clone();
            cloned.timestamp = 0;
            cloned.sequence = base_sequence + index as u64;
            cloned
        })
        .collect()
}
