use std::collections::BTreeMap;

use cmd_handler::command_use_case_def2::CommandUseCase3;
use example_core::{
    Balance, ExecuteImmediateSpotOrderPipelineCmd, MarketRules, PlaceImmediateOrderCmd,
    PlaceImmediateOrderExecution, PlaceOrderTimeInForce,
};
use veldra_core::entity::{
    AccountAssetKey, CommandEnvelope, ExchangeState, ProductCommand, SpotAssetPair, SpotCommand,
    SpotState,
};
use veldra_core::use_case::{
    BuildBlockError, BuildBlockFromCommandsCommand, BuildBlockFromCommandsState,
    BuildBlockFromCommandsUseCase,
};

pub fn sample_exchange_state() -> ExchangeState {
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

    ExchangeState {
        spot: SpotState {
            market_rules_by_symbol,
            asset_pairs_by_symbol,
            trading_enabled_by_symbol,
            balances,
            orders: BTreeMap::new(),
            settled_trade_ids: Default::default(),
            next_order_sequence_by_account,
        },
        ..ExchangeState::default()
    }
}

pub fn sample_spot_command() -> CommandEnvelope<ProductCommand> {
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

pub fn execute_sample_block() -> Result<
    cmd_handler::command_use_case_def2::UseCaseOutput<
        veldra_core::use_case::BuildBlockFromCommandsOutput,
    >,
    BuildBlockError,
> {
    CommandUseCase3::compute_output_and_events(
        &BuildBlockFromCommandsUseCase,
        &BuildBlockFromCommandsCommand { block_height: 2 },
        BuildBlockFromCommandsState {
            parent_height: 1,
            parent_block_hash: "parent-1".to_string(),
            exchange_state: sample_exchange_state(),
            commands: vec![sample_spot_command()],
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_spot_flow_builds_block() -> Result<(), BuildBlockError> {
        let output = execute_sample_block()?;

        assert_eq!(output.output.new_block.block_height, 2);
        assert_eq!(output.output.command_results.len(), 1);
        assert_eq!(output.events.len(), 2);
        assert!(!output.output.new_block.commands_root.is_empty());
        assert!(!output.output.new_block.events_root.is_empty());
        assert!(!output.output.new_block.post_state_root.is_empty());

        let next_balance = output
            .output
            .exchange_state
            .spot
            .balances
            .get(&AccountAssetKey::new("trader-1", "USDT"))
            .unwrap();
        assert_eq!((next_balance.available, next_balance.frozen), (9_800, 200));

        Ok(())
    }
}
