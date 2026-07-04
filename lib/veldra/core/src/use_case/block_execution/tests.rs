use std::collections::BTreeMap;

use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};
use example_core::{
    Balance, CancelSpotOrderCmd, DepositQuoteCmd, ExecuteImmediateSpotOrderPipelineCmd,
    MarketRules, PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceOrderTimeInForce,
    SpotOrder, SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTimeInForce, WithdrawQuoteCmd,
};

use crate::entity::stable_hash_hex;
use crate::use_case::block_execution::canonical_batch::{
    canonical_sort_commands, validate_and_clone_canonical_commands,
};
use crate::use_case::block_execution::handler::block_command_handler::{
    BlockCommandHandler, ResolvedBlockCommandHandler, resolve_block_command_handler,
};
use crate::use_case::block_execution::handler::cancel_order_block_command_handler::CANCEL_ORDER_BLOCK_COMMAND_HANDLER;
use crate::use_case::block_execution::handler::deposit_quote_block_command_handler::DEPOSIT_QUOTE_BLOCK_COMMAND_HANDLER;
use crate::use_case::block_execution::handler::execute_immediate_order_pipeline_block_command_handler::EXECUTE_IMMEDIATE_ORDER_PIPELINE_BLOCK_COMMAND_HANDLER;
use crate::use_case::block_execution::handler::withdraw_quote_block_command_handler::WITHDRAW_QUOTE_BLOCK_COMMAND_HANDLER;
use super::*;
use crate::entity::{
    AccountAssetKey, CommandEnvelope, ExchangeState, PerpCommand, ProductCommand, SpotAssetPair,
    SpotCommand, TreasuryCommand,
};

fn sample_command() -> BuildBlockFromCommandsCommand {
    BuildBlockFromCommandsCommand { block_height: 2 }
}

fn sample_envelope() -> CommandEnvelope<ProductCommand> {
    sample_spot_envelope_with("cmd-1", "trader-1", 1, 1_000, PlaceOrderTimeInForce::Gtc)
}

fn sample_spot_envelope_with(
    command_id: &str,
    account_id: &str,
    nonce: u64,
    timestamp_ns: u64,
    time_in_force: PlaceOrderTimeInForce,
) -> CommandEnvelope<ProductCommand> {
    CommandEnvelope {
        command_id: command_id.to_string(),
        account_id: account_id.to_string(),
        nonce,
        timestamp_ns,
        command: ProductCommand::Spot(SpotCommand::ExecuteImmediateOrderPipeline(
            ExecuteImmediateSpotOrderPipelineCmd {
                place: PlaceImmediateOrderCmd {
                    party_id: account_id.to_string(),
                    asset: 10_001,
                    symbol: "BTCUSDT".to_string(),
                    is_buy: true,
                    size: 2,
                    reduce_only: false,
                    execution: PlaceImmediateOrderExecution::Limit { price: 100, time_in_force },
                    cloid: Some("cl-1".to_string()),
                },
                match_id: "match-1".to_string(),
                settlement_batch_id: "settle-1".to_string(),
            },
        )),
    }
}

fn sample_state() -> BuildBlockFromCommandsState {
    let mut market_rules_by_symbol = BTreeMap::new();
    market_rules_by_symbol
        .insert("BTCUSDT".to_string(), MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 });

    let mut asset_pairs_by_symbol = BTreeMap::new();
    asset_pairs_by_symbol.insert("BTCUSDT".to_string(), SpotAssetPair::new("BTC", "USDT"));

    let mut trading_enabled_by_symbol = BTreeMap::new();
    trading_enabled_by_symbol.insert("BTCUSDT".to_string(), true);

    let mut balances = BTreeMap::new();
    balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 10_000, 0, 3),
    );
    balances.insert(
        AccountAssetKey::new("trader-1", "BTC"),
        Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 2),
    );

    let mut next_order_sequence_by_account = BTreeMap::new();
    next_order_sequence_by_account.insert("trader-1".to_string(), 7);

    BuildBlockFromCommandsState {
        parent_height: 1,
        parent_block_hash: "parent-1".to_string(),
        exchange_state: ExchangeState {
            spot: crate::entity::SpotState {
                market_rules_by_symbol,
                asset_pairs_by_symbol,
                trading_enabled_by_symbol,
                balances,
                orders: BTreeMap::new(),
                reservations: BTreeMap::new(),
                settled_trade_ids: Default::default(),
                next_order_sequence_by_account,
            },
            ..ExchangeState::default()
        },
        commands: vec![sample_envelope()],
    }
}

fn treasury_envelope() -> CommandEnvelope<ProductCommand> {
    treasury_envelope_with("cmd-2", "trader-1", 2, 1_001, 500)
}

fn treasury_envelope_with(
    command_id: &str,
    account_id: &str,
    nonce: u64,
    timestamp_ns: u64,
    amount: u64,
) -> CommandEnvelope<ProductCommand> {
    CommandEnvelope {
        command_id: command_id.to_string(),
        account_id: account_id.to_string(),
        nonce,
        timestamp_ns,
        command: ProductCommand::Treasury(TreasuryCommand::DepositQuote(DepositQuoteCmd {
            party_id: account_id.to_string(),
            amount,
        })),
    }
}

fn cancel_envelope() -> CommandEnvelope<ProductCommand> {
    cancel_envelope_with("cmd-3", "trader-1", 3, 1_002)
}

fn cancel_envelope_with(
    command_id: &str,
    account_id: &str,
    nonce: u64,
    timestamp_ns: u64,
) -> CommandEnvelope<ProductCommand> {
    CommandEnvelope {
        command_id: command_id.to_string(),
        account_id: account_id.to_string(),
        nonce,
        timestamp_ns,
        command: ProductCommand::Spot(SpotCommand::CancelOrder(CancelSpotOrderCmd {
            party_id: account_id.to_string(),
            asset: 10_001,
            order_id: 42,
        })),
    }
}

fn withdraw_envelope() -> CommandEnvelope<ProductCommand> {
    CommandEnvelope {
        command_id: "cmd-4".to_string(),
        account_id: "trader-1".to_string(),
        nonce: 4,
        timestamp_ns: 1_003,
        command: ProductCommand::Treasury(TreasuryCommand::WithdrawQuote(WithdrawQuoteCmd {
            party_id: "trader-1".to_string(),
            amount: 250,
        })),
    }
}

fn open_buy_order() -> SpotOrder {
    SpotOrder::new(
        "order-42".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: 100 },
        SpotOrderTimeInForce::Gtc,
        2,
        0,
        200,
        None,
    )
}

fn open_sell_order() -> SpotOrder {
    SpotOrder::new(
        "order-42".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: 100 },
        SpotOrderTimeInForce::Gtc,
        2,
        2,
        0,
        None,
    )
}

fn state_with_open_buy_order() -> BuildBlockFromCommandsState {
    let mut state = sample_state();
    let order = open_buy_order();
    let reservation = order.to_reservation("BTC", "USDT").unwrap();
    state.exchange_state.spot.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 9_800, 200, 3),
    );
    state.exchange_state.spot.orders.insert("order-42".to_string(), order);
    state.exchange_state.spot.reservations.insert(reservation.reservation_id.clone(), reservation);
    state.commands = vec![cancel_envelope()];
    state
}

fn state_with_open_sell_order() -> BuildBlockFromCommandsState {
    let mut state = sample_state();
    let order = open_sell_order();
    let reservation = order.to_reservation("BTC", "USDT").unwrap();
    state.exchange_state.spot.balances.insert(
        AccountAssetKey::new("trader-1", "BTC"),
        Balance::new("trader-1".to_string(), "BTC".to_string(), 5, 2, 2),
    );
    state.exchange_state.spot.orders.insert("order-42".to_string(), order);
    state.exchange_state.spot.reservations.insert(reservation.reservation_id.clone(), reservation);
    state.commands = vec![cancel_envelope()];
    state
}

fn block(changes: &BuildBlockFromCommandsChanges) -> &crate::entity::NewBlock {
    changes.new_block.as_ref().expect("block builder should always produce a block")
}

fn execution_body(changes: &BuildBlockFromCommandsChanges) -> &crate::entity::BlockExecutionBody {
    changes.execution_body.as_ref().expect("block builder should always produce an execution body")
}

fn spot_balance_after<'a>(
    changes: &'a BuildBlockFromCommandsChanges,
    account_id: &str,
    asset_id: &str,
) -> &'a Balance {
    changes
        .ordered_changes
        .iter()
        .rev()
        .find_map(|change| match change {
            BlockEntityChange::BalanceUpdated(balance)
                if balance.after.account_id == account_id && balance.after.asset_id == asset_id =>
            {
                Some(&balance.after)
            }
            _ => None,
        })
        .unwrap()
}

fn spot_order_after<'a>(
    changes: &'a BuildBlockFromCommandsChanges,
    order_id: &str,
) -> &'a SpotOrder {
    changes
        .ordered_changes
        .iter()
        .rev()
        .find_map(|change| match change {
            BlockEntityChange::SpotOrderCreated(order) if order.order_id == order_id => Some(order),
            BlockEntityChange::SpotOrderUpdated(order) if order.after.order_id == order_id => {
                Some(&order.after)
            }
            _ => None,
        })
        .unwrap()
}

#[test]
fn role_is_block_builder() {
    assert_eq!(CommandUseCase4::role(&BuildBlockFromCommandsUseCase), "BlockBuilder");
}

#[test]
fn pre_check_rejects_zero_block_height() {
    let cmd = BuildBlockFromCommandsCommand { block_height: 0 };
    let result = CommandUseCase4::pre_check_command(&BuildBlockFromCommandsUseCase, &cmd);
    assert_eq!(result, Err(BuildBlockError::BlockHeightMustBePositive));
}

#[test]
fn validate_rejects_empty_batch() {
    let mut state = sample_state();
    state.commands.clear();
    let result = CommandUseCase4::validate_against_state(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        &state,
    );
    assert_eq!(result, Err(BuildBlockError::EmptyCommands));
}

#[test]
fn single_spot_command_builds_block() -> Result<(), BuildBlockError> {
    let changes = CommandUseCase4::compute_changes(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        sample_state(),
    )?;
    let events = changes.to_replayable_events().expect("changes should project to events");
    let new_block = block(&changes);
    let body = execution_body(&changes);

    assert_eq!(changes.ordered_changes.len(), 2);
    assert!(matches!(changes.ordered_changes[0], BlockEntityChange::SpotOrderCreated(_)));
    assert!(matches!(changes.ordered_changes[1], BlockEntityChange::BalanceUpdated(_)));
    assert_eq!(new_block.block_height, 2);
    assert_eq!(new_block.parent_block_hash, "parent-1");
    assert!(!new_block.commands_root.is_empty());
    assert!(!new_block.events_root.is_empty());
    assert!(!new_block.post_state_root.is_empty());
    assert_eq!(body.block_height, new_block.block_height);
    assert_eq!(body.block_hash, new_block.block_hash);
    assert_eq!(body.commands.len(), 1);
    assert_eq!(events.len(), 2);
    assert_eq!(body.replayable_events, events);

    let next_usdt = spot_balance_after(&changes, "trader-1", "USDT");
    assert_eq!((next_usdt.available, next_usdt.frozen), (9_800, 200));

    Ok(())
}

#[test]
fn same_input_produces_same_block_commitment() -> Result<(), BuildBlockError> {
    let first = CommandUseCase4::compute_changes(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        sample_state(),
    )?;
    let second = CommandUseCase4::compute_changes(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        sample_state(),
    )?;

    assert_eq!(block(&first), block(&second));
    assert_eq!(execution_body(&first), execution_body(&second));

    Ok(())
}

#[test]
fn treasury_deposit_updates_exchange_state() -> Result<(), BuildBlockError> {
    let mut state = sample_state();
    state.exchange_state.treasury.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );
    state.commands = vec![treasury_envelope()];

    let changes =
        CommandUseCase4::compute_changes(&BuildBlockFromCommandsUseCase, &sample_command(), state)?;
    let events = changes.to_replayable_events().expect("changes should project to events");

    assert_eq!(changes.ordered_changes.len(), 1);
    assert!(matches!(changes.ordered_changes[0], BlockEntityChange::BalanceUpdated(_)));
    let next_usdt = spot_balance_after(&changes, "trader-1", "USDT");
    assert_eq!((next_usdt.available, next_usdt.frozen, next_usdt.version), (1_500, 0, 2));
    assert_eq!(events.len(), 1);

    Ok(())
}

#[test]
fn single_spot_cancel_command_builds_block() -> Result<(), BuildBlockError> {
    let changes = CommandUseCase4::compute_changes(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        state_with_open_buy_order(),
    )?;
    let events = changes.to_replayable_events().expect("changes should project to events");

    assert_eq!(changes.ordered_changes.len(), 2);
    assert!(matches!(changes.ordered_changes[0], BlockEntityChange::SpotOrderUpdated(_)));
    assert!(matches!(changes.ordered_changes[1], BlockEntityChange::BalanceUpdated(_)));
    assert_eq!(events.len(), 2);

    let next_order = spot_order_after(&changes, "order-42");
    assert_eq!(next_order.status, SpotOrderStatus::Canceled);
    assert_eq!(next_order.status_reason, Some(SpotOrderStatusReason::CanceledByUser));
    assert_eq!(next_order.version, 2);

    let next_usdt = spot_balance_after(&changes, "trader-1", "USDT");
    assert_eq!((next_usdt.available, next_usdt.frozen, next_usdt.version), (10_000, 0, 4));

    Ok(())
}

#[test]
fn spot_cancel_sell_order_releases_base_balance() -> Result<(), BuildBlockError> {
    let changes = CommandUseCase4::compute_changes(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        state_with_open_sell_order(),
    )?;

    let next_order = spot_order_after(&changes, "order-42");
    assert_eq!(next_order.status, SpotOrderStatus::Canceled);
    assert_eq!(next_order.status_reason, Some(SpotOrderStatusReason::CanceledByUser));

    let next_btc = spot_balance_after(&changes, "trader-1", "BTC");
    assert_eq!((next_btc.available, next_btc.frozen, next_btc.version), (7, 0, 3));

    Ok(())
}

#[test]
fn spot_cancel_missing_order_returns_spot_execution_error() {
    let mut state = sample_state();
    state.commands = vec![cancel_envelope()];

    let result =
        CommandUseCase4::compute_changes(&BuildBlockFromCommandsUseCase, &sample_command(), state);

    assert_eq!(result, Err(BuildBlockError::SpotExecution("open order was not found".to_string())));
}

#[test]
fn mixed_spot_and_treasury_batch_builds_block() -> Result<(), BuildBlockError> {
    let mut state = sample_state();
    state.exchange_state.treasury.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );
    state.commands = vec![sample_envelope(), treasury_envelope()];

    let changes =
        CommandUseCase4::compute_changes(&BuildBlockFromCommandsUseCase, &sample_command(), state)?;
    let events = changes.to_replayable_events().expect("changes should project to events");

    assert_eq!(changes.ordered_changes.len(), 3);
    assert_eq!(events.len(), 3);

    let treasury_usdt = spot_balance_after(&changes, "trader-1", "USDT");
    assert_eq!(
        (treasury_usdt.available, treasury_usdt.frozen, treasury_usdt.version),
        (1_500, 0, 2)
    );

    let spot_usdt_change = changes
        .ordered_changes
        .iter()
        .find_map(|change| match change {
            BlockEntityChange::BalanceUpdated(balance)
                if balance.before.available == 10_000 && balance.after.available == 9_800 =>
            {
                Some(&balance.after)
            }
            _ => None,
        })
        .unwrap();
    assert_eq!((spot_usdt_change.available, spot_usdt_change.frozen), (9_800, 200));

    let sequences = events.iter().map(|event| event.sequence).collect::<Vec<_>>();
    assert_eq!(sequences, vec![0, 1, 2]);

    Ok(())
}

#[test]
fn batch_event_sequences_are_continuous_across_commands() -> Result<(), BuildBlockError> {
    let mut state = sample_state();
    state.exchange_state.treasury.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );
    state.commands = vec![sample_envelope(), treasury_envelope()];

    let changes =
        CommandUseCase4::compute_changes(&BuildBlockFromCommandsUseCase, &sample_command(), state)?;
    let events = changes.to_replayable_events().expect("changes should project to events");

    let sequences = events.iter().map(|event| event.sequence).collect::<Vec<_>>();
    assert_eq!(sequences, vec![0, 1, 2]);

    Ok(())
}

#[test]
fn resolve_block_command_handler_maps_all_five_command_families() {
    assert!(matches!(
        resolve_block_command_handler(&sample_envelope().command),
        ResolvedBlockCommandHandler::ExecuteImmediateOrderPipeline(_, _)
    ));
    assert!(matches!(
        resolve_block_command_handler(&cancel_envelope().command),
        ResolvedBlockCommandHandler::CancelOrder(_, _)
    ));
    assert!(matches!(
        resolve_block_command_handler(&treasury_envelope().command),
        ResolvedBlockCommandHandler::DepositQuote(_, _)
    ));
    assert!(matches!(
        resolve_block_command_handler(&withdraw_envelope().command),
        ResolvedBlockCommandHandler::WithdrawQuote(_, _)
    ));
    assert!(matches!(
        resolve_block_command_handler(&ProductCommand::Perp(PerpCommand::Unsupported)),
        ResolvedBlockCommandHandler::PerpUnsupported(_)
    ));
}

#[test]
fn execute_immediate_order_pipeline_handler_returns_changes_and_apply_patch()
-> Result<(), BuildBlockError> {
    let state = sample_state();
    let envelope = sample_envelope();
    let ProductCommand::Spot(SpotCommand::ExecuteImmediateOrderPipeline(command)) =
        &envelope.command
    else {
        unreachable!();
    };

    let result = EXECUTE_IMMEDIATE_ORDER_PIPELINE_BLOCK_COMMAND_HANDLER.execute(
        &envelope,
        command,
        &state.exchange_state,
    )?;
    assert_eq!(result.apply_patch.next_order_sequence, 8);
    assert!(result.apply_patch.settled_trade_ids_appended.is_empty());
    assert!(result.changes.match_output.is_none());
    assert!(result.changes.settle_changes.is_none());
    assert_eq!(result.changes.place_output.created_order.order_id, "trader-1-BTCUSDT-7");

    Ok(())
}

#[test]
fn cancel_order_handler_returns_example_core_changes() -> Result<(), BuildBlockError> {
    let state = state_with_open_buy_order();
    let envelope = cancel_envelope();
    let ProductCommand::Spot(SpotCommand::CancelOrder(command)) = &envelope.command else {
        unreachable!();
    };

    let result =
        CANCEL_ORDER_BLOCK_COMMAND_HANDLER.execute(&envelope, command, &state.exchange_state)?;
    assert_eq!(result.canceled_order.before.status, SpotOrderStatus::Open);
    assert_eq!(result.canceled_order.after.status, SpotOrderStatus::Canceled);
    assert_eq!(result.released_balances.len(), 1);
    assert_eq!(result.released_balances[0].before.frozen, 200);
    assert_eq!(result.released_balances[0].after.frozen, 0);

    Ok(())
}

#[test]
fn deposit_and_withdraw_handlers_return_quote_changes() -> Result<(), BuildBlockError> {
    let mut deposit_state = sample_state();
    deposit_state.exchange_state.treasury.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );
    let deposit_envelope = treasury_envelope();
    let ProductCommand::Treasury(TreasuryCommand::DepositQuote(deposit_command)) =
        &deposit_envelope.command
    else {
        unreachable!();
    };
    let deposit_result = DEPOSIT_QUOTE_BLOCK_COMMAND_HANDLER.execute(
        &deposit_envelope,
        deposit_command,
        &deposit_state.exchange_state,
    )?;
    assert_eq!(deposit_result.updated_quote_balance.before.available, 1_000);
    assert_eq!(deposit_result.updated_quote_balance.after.available, 1_500);

    let mut withdraw_state = sample_state();
    withdraw_state.exchange_state.treasury.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );
    let withdraw_envelope = withdraw_envelope();
    let ProductCommand::Treasury(TreasuryCommand::WithdrawQuote(withdraw_command)) =
        &withdraw_envelope.command
    else {
        unreachable!();
    };
    let withdraw_result = WITHDRAW_QUOTE_BLOCK_COMMAND_HANDLER.execute(
        &withdraw_envelope,
        withdraw_command,
        &withdraw_state.exchange_state,
    )?;
    assert_eq!(withdraw_result.updated_quote_balance.before.available, 1_000);
    assert_eq!(withdraw_result.updated_quote_balance.after.available, 750);

    Ok(())
}

#[test]
fn validate_rejects_duplicate_command_id_in_batch() {
    let mut state = sample_state();
    state.commands = vec![
        sample_spot_envelope_with("dup-cmd", "trader-1", 1, 1_000, PlaceOrderTimeInForce::Gtc),
        treasury_envelope_with("dup-cmd", "trader-2", 2, 1_001, 500),
    ];

    let result = CommandUseCase4::validate_against_state(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        &state,
    );

    assert_eq!(
        result,
        Err(BuildBlockError::DuplicateCommandId { command_id: "dup-cmd".to_string() })
    );
}

#[test]
fn validate_rejects_duplicate_account_nonce_in_batch() {
    let mut state = sample_state();
    state.exchange_state.treasury.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );
    state.commands = vec![
        sample_spot_envelope_with("cmd-a", "trader-1", 7, 1_000, PlaceOrderTimeInForce::Gtc),
        treasury_envelope_with("cmd-b", "trader-1", 7, 1_001, 500),
    ];

    let result = CommandUseCase4::validate_against_state(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        &state,
    );

    assert_eq!(
        result,
        Err(BuildBlockError::DuplicateAccountNonce {
            account_id: "trader-1".to_string(),
            nonce: 7,
        })
    );
}

#[test]
fn validate_rejects_zero_timestamp_command() {
    let mut state = sample_state();
    state.commands =
        vec![sample_spot_envelope_with("cmd-zero", "trader-1", 1, 0, PlaceOrderTimeInForce::Gtc)];

    let result = CommandUseCase4::validate_against_state(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        &state,
    );

    assert_eq!(
        result,
        Err(BuildBlockError::ZeroCommandTimestamp { command_id: "cmd-zero".to_string() })
    );
}

#[test]
fn validate_rejects_envelope_account_mismatch() {
    let mut state = sample_state();
    let mut mismatched = sample_envelope();
    mismatched.account_id = "operator-1".to_string();
    state.commands = vec![mismatched];

    let result = CommandUseCase4::validate_against_state(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        &state,
    );

    assert_eq!(
        result,
        Err(BuildBlockError::EnvelopeAccountMismatch {
            command_id: "cmd-1".to_string(),
            envelope_account_id: "operator-1".to_string(),
            command_party_id: "trader-1".to_string(),
        })
    );
}

#[test]
fn validate_rejects_non_canonical_command_order() {
    let mut state = sample_state();
    let alo =
        sample_spot_envelope_with("cmd-alo", "trader-1", 2, 2_000, PlaceOrderTimeInForce::Alo);
    let gtc =
        sample_spot_envelope_with("cmd-gtc", "trader-1", 1, 1_000, PlaceOrderTimeInForce::Gtc);
    state.commands = vec![gtc, alo];

    let result = CommandUseCase4::validate_against_state(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        &state,
    );

    assert_eq!(result, Err(BuildBlockError::NonCanonicalCommandOrder));
}

#[test]
fn canonical_sort_prioritizes_alo_before_other_commands() {
    let gtc =
        sample_spot_envelope_with("cmd-gtc", "trader-1", 1, 1_000, PlaceOrderTimeInForce::Gtc);
    let alo =
        sample_spot_envelope_with("cmd-alo", "trader-1", 2, 2_000, PlaceOrderTimeInForce::Alo);
    let cancel = cancel_envelope_with("cmd-cancel", "trader-1", 3, 500);

    let sorted = canonical_sort_commands(&[gtc, cancel, alo]);
    let sorted_ids = sorted.iter().map(|command| command.command_id.as_str()).collect::<Vec<_>>();

    assert_eq!(sorted_ids, vec!["cmd-alo", "cmd-cancel", "cmd-gtc"]);
}

#[test]
fn compute_changes_rejects_non_canonical_batch() {
    let mut state = sample_state();
    let alo =
        sample_spot_envelope_with("cmd-alo", "trader-1", 2, 2_000, PlaceOrderTimeInForce::Alo);
    let gtc =
        sample_spot_envelope_with("cmd-gtc", "trader-1", 1, 1_000, PlaceOrderTimeInForce::Gtc);
    state.commands = vec![gtc, alo];

    let result =
        CommandUseCase4::compute_changes(&BuildBlockFromCommandsUseCase, &sample_command(), state);

    assert_eq!(result, Err(BuildBlockError::NonCanonicalCommandOrder));
}

#[test]
fn compute_changes_uses_canonical_commands_for_block_root() -> Result<(), BuildBlockError> {
    let mut state = sample_state();
    state.exchange_state.treasury.balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );
    let alo = sample_spot_envelope_with("cmd-alo", "trader-1", 1, 10, PlaceOrderTimeInForce::Alo);
    let treasury = treasury_envelope_with("cmd-treasury", "trader-1", 2, 30, 500);
    state.commands = vec![alo.clone(), treasury.clone()];

    let canonical = validate_and_clone_canonical_commands(&state.commands)?;
    let expected_root =
        stable_hash_hex(&canonical.iter().map(CommandEnvelope::commitment).collect::<Vec<_>>());

    let changes =
        CommandUseCase4::compute_changes(&BuildBlockFromCommandsUseCase, &sample_command(), state)?;

    assert_eq!(block(&changes).commands_root, expected_root);

    Ok(())
}

#[test]
fn changes_are_the_single_business_truth_and_events_are_projected_from_them()
-> Result<(), BuildBlockError> {
    let changes = CommandUseCase4::compute_changes(
        &BuildBlockFromCommandsUseCase,
        &sample_command(),
        sample_state(),
    )?;
    let events = changes.to_replayable_events().expect("changes should project to events");

    assert_eq!(block(&changes).block_height, 2);
    assert_eq!(execution_body(&changes).block_height, 2);
    assert_eq!(changes.ordered_changes.len(), 2);
    assert_eq!(events.len(), 2);
    assert_eq!(events.len(), changes.ordered_changes.len());

    Ok(())
}
