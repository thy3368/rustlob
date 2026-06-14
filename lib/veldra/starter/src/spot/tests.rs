use std::collections::BTreeMap;

use cmd_handler::command_use_case_def2::CommandUseCase3;
use veldra_core::entity::PendingRequest;
use veldra_core::use_case::{
    BuildBlockFromPendingRequestsCommand, BuildBlockFromPendingRequestsState,
    BuildBlockFromPendingRequestsUseCase,
};

use super::*;

fn balance(
    account_id: &str,
    asset: &str,
    available: u64,
    reserved: u64,
    version: u64,
) -> SpotBalanceSnapshot {
    SpotBalanceSnapshot {
        account_id: account_id.to_string(),
        asset: asset.to_string(),
        available,
        reserved,
        version,
    }
}

fn sample_context() -> SpotProductContext {
    let mut balances = BTreeMap::new();
    for item in [balance("trader-1", "USDT", 10_000, 0, 3), balance("trader-1", "BTC", 0, 0, 2)] {
        balances.insert(item.key(), item);
    }

    SpotProductContext {
        account_id: "trader-1".to_string(),
        balances,
        market_rules: SpotMarketRules {
            symbol: "BTCUSDT".to_string(),
            base_asset: "BTC".to_string(),
            quote_asset: "USDT".to_string(),
            min_qty: 1,
        },
        next_order_sequence: 7,
        trading_enabled: true,
        maker_orders: Vec::new(),
        settled_trade_ids: Vec::new(),
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
    }
}

fn sample_payload() -> SpotPlaceOrderPayload {
    SpotPlaceOrderPayload {
        account_id: "trader-1".to_string(),
        asset: 10_001,
        symbol: "BTCUSDT".to_string(),
        is_buy: true,
        size: 2,
        reduce_only: false,
        order_type: SpotPlaceOrderType::Limit { limit_px: 100, tif: SpotTimeInForce::Gtc },
        cloid: Some("cl-1".to_string()),
    }
}

fn sample_request() -> PendingRequest {
    PendingRequest {
        request_id: "req-1".to_string(),
        product_id: SPOT_PRODUCT_ID.to_string(),
        action: "place_order".to_string(),
        payload: serde_json::to_string(&sample_payload()).unwrap(),
    }
}

#[test]
fn build_block_use_case_executes_real_spot_adapter()
-> Result<(), veldra_core::use_case::BuildBlockError> {
    let mut product_contexts = BTreeMap::new();
    product_contexts.insert(SPOT_PRODUCT_ID.to_string(), sample_context().into_core().unwrap());

    let output = CommandUseCase3::compute_output_and_events(
        &BuildBlockFromPendingRequestsUseCase,
        &BuildBlockFromPendingRequestsCommand { block_height: 2 },
        BuildBlockFromPendingRequestsState {
            parent_height: 1,
            parent_block_hash: "parent-1".to_string(),
            pending_requests: vec![sample_request()],
            product_plugins: build_default_product_registry(),
            product_contexts,
        },
    )?;

    assert_eq!(output.output.new_block.block_height, 2);
    assert!(!output.output.new_block.events_root.is_empty());
    assert!(!output.output.new_block.post_state_root.is_empty());
    assert_eq!(output.output.request_results.len(), 1);
    assert_eq!(output.events.len(), 2);
    assert_eq!(output.events[0].sequence, 0);
    assert_eq!(output.events[1].sequence, 1);

    let request_result = &output.output.request_results[0];
    assert_eq!(request_result.result_kind, "spot.place_order");
    assert_eq!(request_result.product_id, SPOT_PRODUCT_ID);
    assert_eq!(request_result.next_product_context.product_id, SPOT_PRODUCT_ID);
    assert_eq!(request_result.events.len(), 2);
    assert_eq!(request_result.events[0].sequence, 0);
    assert_eq!(request_result.events[1].sequence, 1);

    let result_payload: SpotPlaceOrderResult =
        serde_json::from_str(&request_result.result_payload).unwrap();
    assert!(result_payload.matched_trades.is_empty());
    assert!(result_payload.settlements.is_empty());
    assert_eq!(result_payload.placed_order.status, SpotOrderStatus::Open);

    let next_context: SpotProductContext =
        serde_json::from_str(&request_result.next_product_context.snapshot).unwrap();
    assert_eq!(next_context.next_order_sequence, 8);
    assert_eq!(
        next_context
            .balances
            .get("trader-1:USDT")
            .map(|balance| (balance.available, balance.reserved)),
        Some((9_800, 200))
    );

    Ok(())
}
