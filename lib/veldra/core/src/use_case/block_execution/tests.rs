use std::collections::BTreeMap;

use cmd_handler::command_use_case_def2::CommandUseCase3;
use example_core::{
    Balance, DepositQuoteCmd, ExecuteImmediateSpotOrderPipelineCmd, MarketRules,
    PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceOrderTimeInForce,
};

use super::*;
use crate::entity::{
    AccountAssetKey, CommandEnvelope, ExchangeState, ProductCommand, SpotAssetPair, SpotCommand,
    TreasuryCommand,
};

fn sample_command() -> BuildBlockFromCommandsCommand {
    BuildBlockFromCommandsCommand { block_height: 2 }
}

fn sample_envelope() -> CommandEnvelope<ProductCommand> {
    CommandEnvelope {
        command_id: "cmd-1".to_string(),
        account_id: "trader-1".to_string(),
        nonce: 1,
        timestamp_ns: 1_000,
        command: ProductCommand::Spot(SpotCommand::ExecuteImmediateOrderPipeline(
            ExecuteImmediateSpotOrderPipelineCmd {
                place: PlaceImmediateOrderCmd {
                    party_id: "trader-1".to_string(),
                    asset: 10_001,
                    symbol: "BTCUSDT".to_string(),
                    is_buy: true,
                    size: 2,
                    reduce_only: false,
                    execution: PlaceImmediateOrderExecution::Limit {
                        price: 100,
                        time_in_force: PlaceOrderTimeInForce::Gtc,
                    },
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
    market_rules_by_symbol.insert(
        "BTCUSDT".to_string(),
        MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
    );

    let mut asset_pairs_by_symbol = BTreeMap::new();
    asset_pairs_by_symbol.insert(
        "BTCUSDT".to_string(),
        SpotAssetPair::new("BTC", "USDT"),
    );

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
                settled_trade_ids: Default::default(),
                next_order_sequence_by_account,
            },
            ..ExchangeState::default()
        },
        commands: vec![sample_envelope()],
    }
}

fn treasury_envelope() -> CommandEnvelope<ProductCommand> {
    CommandEnvelope {
        command_id: "cmd-2".to_string(),
        account_id: "trader-1".to_string(),
        nonce: 2,
        timestamp_ns: 1_001,
        command: ProductCommand::Treasury(TreasuryCommand::DepositQuote(DepositQuoteCmd {
            party_id: "trader-1".to_string(),
            amount: 500,
        })),
    }
}

#[test]
fn role_is_block_builder() {
    assert_eq!(CommandUseCase3::role(&BuildBlockFromCommandsUseCase), "BlockBuilder");
}

#[test]
fn pre_check_rejects_zero_block_height() {
    let cmd = BuildBlockFromCommandsCommand { block_height: 0 };
    let result = CommandUseCase3::pre_check_command(&BuildBlockFromCommandsUseCase, &cmd);
    assert_eq!(result, Err(BuildBlockError::BlockHeightMustBePositive));
}

#[test]
fn validate_rejects_empty_batch() {
    let mut state = sample_state();
    state.commands.clear();
    let result =
        CommandUseCase3::validate_against_state(&BuildBlockFromCommandsUseCase, &sample_command(), &state);
    assert_eq!(result, Err(BuildBlockError::EmptyCommands));
}

#[test]
fn single_spot_command_builds_block() -> Result<(), BuildBlockError> {
    let result =
        CommandUseCase3::compute_output_and_events(&BuildBlockFromCommandsUseCase, &sample_command(), sample_state())?;

    assert_eq!(result.output.command_results.len(), 1);
    assert_eq!(result.output.new_block.block_height, 2);
    assert_eq!(result.output.new_block.parent_block_hash, "parent-1");
    assert!(!result.output.new_block.commands_root.is_empty());
    assert!(!result.output.new_block.events_root.is_empty());
    assert!(!result.output.new_block.post_state_root.is_empty());
    assert_eq!(result.events.len(), 2);

    let next_usdt = result
        .output
        .exchange_state
        .spot
        .balances
        .get(&AccountAssetKey::new("trader-1", "USDT"))
        .unwrap();
    assert_eq!((next_usdt.available, next_usdt.frozen), (9_800, 200));
    assert_eq!(
        result.output.exchange_state.spot.next_order_sequence_by_account["trader-1"],
        8
    );

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

    let result =
        CommandUseCase3::compute_output_and_events(&BuildBlockFromCommandsUseCase, &sample_command(), state)?;

    let next_usdt = result
        .output
        .exchange_state
        .treasury
        .balances
        .get(&AccountAssetKey::new("trader-1", "USDT"))
        .unwrap();
    assert_eq!((next_usdt.available, next_usdt.frozen, next_usdt.version), (1_500, 0, 2));
    assert_eq!(result.events.len(), 1);

    Ok(())
}
