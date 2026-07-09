use std::collections::BTreeMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use common_entity::MiStateMachineV2Unchecked;
use example_core::{
    Balance, DepositQuoteCmd, ExecuteImmediateSpotOrderPipelineCmd, MarketRules,
    PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceOrderTimeInForce,
};
use veldra_core::entity::{
    AccountAssetKey, BlockExecutionBody, ExchangeState, NewBlock, ProductCommand, SpotAssetPair,
    SpotCommand, TreasuryCommand,
};
use veldra_core::use_case::{
    BlockEntityChange, BuildBlockFromCommandsChanges, BuildBlockFromCommandsCommand,
    BuildBlockFromCommandsState, BuildBlockFromCommandsUseCase,
};

pub struct TestDir {
    pub path: std::path::PathBuf,
}

impl TestDir {
    pub fn new() -> Self {
        let unique =
            SystemTime::now().duration_since(UNIX_EPOCH).expect("clock should advance").as_nanos();
        let path = std::env::temp_dir().join(format!("veldra-mdbx-test-{unique}"));
        fs::create_dir_all(&path).expect("temp dir should be created");
        Self { path }
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub fn sample_command() -> BuildBlockFromCommandsCommand {
    BuildBlockFromCommandsCommand { block_height: 2 }
}

pub fn sample_state() -> BuildBlockFromCommandsState {
    let mut market_rules_by_symbol = BTreeMap::new();
    market_rules_by_symbol
        .insert("BTCUSDT".to_string(), MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 });

    let mut asset_pairs_by_symbol = BTreeMap::new();
    asset_pairs_by_symbol.insert("BTCUSDT".to_string(), SpotAssetPair::new("BTC", "USDT"));

    let mut trading_enabled_by_symbol = BTreeMap::new();
    trading_enabled_by_symbol.insert("BTCUSDT".to_string(), true);

    let mut spot_balances = BTreeMap::new();
    spot_balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 10_000, 0, 3),
    );
    spot_balances.insert(
        AccountAssetKey::new("trader-1", "BTC"),
        Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 2),
    );

    let mut treasury_balances = BTreeMap::new();
    treasury_balances.insert(
        AccountAssetKey::new("trader-1", "USDT"),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
    );

    let mut next_order_sequence_by_account = BTreeMap::new();
    next_order_sequence_by_account.insert("trader-1".to_string(), 7);

    BuildBlockFromCommandsState {
        parent_height: 1,
        parent_block_hash: "parent-1".to_string(),
        exchange_state: ExchangeState {
            spot: veldra_core::entity::SpotState {
                market_rules_by_symbol,
                asset_pairs_by_symbol,
                trading_enabled_by_symbol,
                balances: spot_balances,
                orders: BTreeMap::new(),
                reservations: BTreeMap::new(),
                settled_trade_ids: Default::default(),
                next_order_sequence_by_account,
            },
            treasury: veldra_core::entity::TreasuryState {
                balances: treasury_balances,
                processed_transfer_ids: Default::default(),
                processed_withdraw_ids: Default::default(),
                withdraw_locks: Default::default(),
            },
            ..ExchangeState::default()
        },
        commands: vec![
            veldra_core::entity::CommandEnvelope {
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
            },
            veldra_core::entity::CommandEnvelope {
                command_id: "cmd-2".to_string(),
                account_id: "trader-1".to_string(),
                nonce: 2,
                timestamp_ns: 1_001,
                command: ProductCommand::Treasury(TreasuryCommand::DepositQuote(DepositQuoteCmd {
                    party_id: "trader-1".to_string(),
                    amount: 500,
                })),
            },
        ],
    }
}

pub fn built_block() -> BuildBlockFromCommandsChanges {
    BuildBlockFromCommandsUseCase
        .compute_after_changes_unchecked(&sample_command(), &sample_state())
        .expect("block should build")
}

pub fn header_body_changes(
    changes: &BuildBlockFromCommandsChanges,
) -> (NewBlock, BlockExecutionBody, Vec<BlockEntityChange>) {
    (
        changes.new_block.clone().expect("new block is required"),
        changes.execution_body.clone().expect("execution body is required"),
        changes.ordered_changes.clone(),
    )
}
