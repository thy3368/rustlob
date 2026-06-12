use example_core::{Balance, SpotOrder, SpotOrderExecution, SpotOrderSide, SpotOrderTimeInForce};
use example_inbound_adapter::{
    MatchSpotOrderEventRequest, SettleSpotTradeEventRequest, handle_place_order_http,
    handle_spot_order_placed_event, handle_spot_trade_matched_event,
};
use example_outbound_adapter::{
    InMemoryMatchSpotOrderOutbound, InMemoryPlaceOrderOutbound, InMemorySettleSpotTradeOutbound,
    InMemorySpotPipelineBroker, InMemoryStore, SpotPipelineBroker, SpotPipelineMessage,
};

fn seeded_store() -> Result<InMemoryStore, Box<dyn std::error::Error>> {
    let store = InMemoryStore::seed_balances(
        Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 2),
        Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 2),
        example_core::MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
    )?;
    store.seed_balance(Balance::new("maker-1".to_string(), "BTC".to_string(), 0, 1, 1))?;
    store.seed_balance(Balance::new("maker-1".to_string(), "USDT".to_string(), 0, 0, 1))?;
    store.seed_order(SpotOrder::new(
        "maker-order-1".to_string(),
        10_001,
        None,
        "maker-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: 100 },
        SpotOrderTimeInForce::Gtc,
        1,
        1,
        0,
        None,
    ))?;
    Ok(store)
}

#[test]
fn event_pipeline_matches_and_settles_after_http_place() -> Result<(), Box<dyn std::error::Error>> {
    let store = seeded_store()?;
    let broker = InMemorySpotPipelineBroker::default();
    let place_order_outbound =
        InMemoryPlaceOrderOutbound::from_store_with_broker(store.clone(), broker.clone());
    let match_outbound = InMemoryMatchSpotOrderOutbound::new(store.clone(), broker.clone());
    let settle_outbound = InMemorySettleSpotTradeOutbound::new(store.clone());

    let _ = handle_place_order_http(
        example_inbound_adapter::PlaceOrderHttpRequest {
            trace_id: Some("trace-1".to_string()),
            command_id: Some("cmd-1".to_string()),
            trader_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            qty: 1,
            price: 100,
        },
        &place_order_outbound,
    )?;

    let Some(SpotPipelineMessage::SpotOrderPlaced(placed)) = broker.pop()? else {
        panic!("expected SpotOrderPlaced message");
    };
    let match_result = handle_spot_order_placed_event(
        MatchSpotOrderEventRequest {
            trace_id: placed.trace_id.clone(),
            command_id: placed.command_id.clone(),
            party_id: placed.party_id.clone(),
            taker_order_id: placed.order_id.clone(),
            match_id: placed.match_id.clone(),
        },
        &match_outbound,
    )?;
    assert_eq!(match_result.output.trades.len(), 1);

    let Some(SpotPipelineMessage::SpotTradeMatched(matched)) = broker.pop()? else {
        panic!("expected SpotTradeMatched message");
    };
    let settle_result = handle_spot_trade_matched_event(
        SettleSpotTradeEventRequest {
            trace_id: matched.trace_id.clone(),
            command_id: matched.command_id.clone(),
            party_id: matched.party_id.clone(),
            settlement_batch_id: matched.settlement_batch_id.clone(),
            trade_ids: matched.trade_ids.clone(),
        },
        &settle_outbound,
    )?;
    assert_eq!(settle_result.output.settlements.len(), 1);

    let snapshot = store.snapshot_with_broker_depth(broker.len()?)?;
    assert_eq!(snapshot.trades.len(), 1);
    assert_eq!(snapshot.settlements.len(), 1);
    assert_eq!(snapshot.balances.get("trader-1:BTC").map(|balance| balance.available), Some(1));

    Ok(())
}
