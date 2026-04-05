use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, ExecuteTradingBatchHandler, ExecutedOrder,
    ExecutedTrade, OrderStatus, SpotCommand, SpotPlaceOrderCmd, SpotSide, TradingCommand,
};

fn spot_place_order(
    command_id: u64,
    trader_id: u64,
    side: SpotSide,
    price: u64,
    quantity: u64,
) -> ExchangeCommandEnvelope {
    ExchangeCommandEnvelope {
        command_id,
        trader_id,
        nonce: command_id,
        timestamp_ns: 1_000 + command_id,
        command: ExchangeCommand::TradingCommand(TradingCommand::Spot(
            SpotCommand::PlaceOrder(SpotPlaceOrderCmd {
                trader_id,
                market: "BTC-USDC".into(),
                side,
                price,
                quantity,
            }),
        )),
    }
}

#[test]
fn five_spot_orders_without_counterparty_create_five_open_resting_orders() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(
            vec![
                spot_place_order(1, 11, SpotSide::Buy, 100_000, 1),
                spot_place_order(2, 12, SpotSide::Buy, 99_000, 2),
                spot_place_order(3, 13, SpotSide::Buy, 98_000, 3),
                spot_place_order(4, 14, SpotSide::Buy, 97_000, 4),
                spot_place_order(5, 15, SpotSide::Buy, 96_000, 5),
            ],
            |writes, _| writes.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.total_commands, 5);
    assert_eq!(result.summary.accepted_commands, 5);
    assert_eq!(result.summary.rejected_commands, 0);
    assert_eq!(result.summary.orders_created, 5);
    assert_eq!(result.summary.trades_executed, 0);
    assert_eq!(result.summary.balance_updates, 0);

    assert_eq!(result.orders.len(), 5);
    assert_eq!(result.trades.len(), 0);
    assert_eq!(result.balance_deltas.len(), 0);

    match &result.orders[0] {
        ExecutedOrder::SpotOrder(order) => {
            assert_eq!(order.state.status, OrderStatus::Pending);
            assert_eq!(order.unfilled_qty().to_f64(), 1.0);
        }
        ExecutedOrder::PrepOrder(_) => panic!("expected spot order result"),
    }
    match &result.orders[1] {
        ExecutedOrder::SpotOrder(order) => {
            assert_eq!(order.state.status, OrderStatus::Pending);
            assert_eq!(order.unfilled_qty().to_f64(), 2.0);
        }
        ExecutedOrder::PrepOrder(_) => panic!("expected spot order result"),
    }
    match &result.orders[2] {
        ExecutedOrder::SpotOrder(order) => {
            assert_eq!(order.state.status, OrderStatus::Pending);
            assert_eq!(order.unfilled_qty().to_f64(), 3.0);
        }
        ExecutedOrder::PrepOrder(_) => panic!("expected spot order result"),
    }
    match &result.orders[3] {
        ExecutedOrder::SpotOrder(order) => {
            assert_eq!(order.state.status, OrderStatus::Pending);
            assert_eq!(order.unfilled_qty().to_f64(), 4.0);
        }
        ExecutedOrder::PrepOrder(_) => panic!("expected spot order result"),
    }
    match &result.orders[4] {
        ExecutedOrder::SpotOrder(order) => {
            assert_eq!(order.state.status, OrderStatus::Pending);
            assert_eq!(order.unfilled_qty().to_f64(), 5.0);
        }
        ExecutedOrder::PrepOrder(_) => panic!("expected spot order result"),
    }
}


#[test]
fn five_spot_orders_with_counterparty_each_execute_a_trade() {
    let handler = ExecuteTradingBatchHandler::new();

    handler
        .cmd_handle(
            vec![
                spot_place_order(11, 101, SpotSide::Sell, 100_000, 1),
                spot_place_order(12, 102, SpotSide::Sell, 99_000, 1),
                spot_place_order(13, 103, SpotSide::Sell, 98_000, 1),
                spot_place_order(14, 104, SpotSide::Sell, 97_000, 1),
                spot_place_order(15, 105, SpotSide::Sell, 96_000, 1),
            ],
            |writes, _| writes.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![
                spot_place_order(21, 201, SpotSide::Buy, 100_000, 1),
                spot_place_order(22, 202, SpotSide::Buy, 99_000, 1),
                spot_place_order(23, 203, SpotSide::Buy, 98_000, 1),
                spot_place_order(24, 204, SpotSide::Buy, 97_000, 1),
                spot_place_order(25, 205, SpotSide::Buy, 96_000, 1),
            ],
            |writes, _| writes.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.total_commands, 5);
    assert_eq!(result.summary.accepted_commands, 5);
    assert_eq!(result.summary.rejected_commands, 0);
    assert_eq!(result.summary.orders_created, 5);
    assert_eq!(result.summary.trades_executed, 5);
    assert_eq!(result.summary.balance_updates, 20);

    assert_eq!(result.orders.len(), 5);
    assert_eq!(result.trades.len(), 5);
    assert_eq!(result.balance_deltas.len(), 20);

    for order in &result.orders {
        match order {
            ExecutedOrder::SpotOrder(order) => {
                assert_eq!(order.state.status, OrderStatus::Filled);
                assert_eq!(order.unfilled_qty().to_f64(), 0.0);
            }
            ExecutedOrder::PrepOrder(_) => panic!("expected spot order result"),
        }
    }

    match &result.trades[0] {
        ExecutedTrade::SpotTrade(trade) => assert_eq!(trade.price.to_f64(), 100_000.0),
        ExecutedTrade::PrepTrade(_) => panic!("expected spot trade result"),
    }
    match &result.trades[1] {
        ExecutedTrade::SpotTrade(trade) => assert_eq!(trade.price.to_f64(), 99_000.0),
        ExecutedTrade::PrepTrade(_) => panic!("expected spot trade result"),
    }
    match &result.trades[2] {
        ExecutedTrade::SpotTrade(trade) => assert_eq!(trade.price.to_f64(), 98_000.0),
        ExecutedTrade::PrepTrade(_) => panic!("expected spot trade result"),
    }
    match &result.trades[3] {
        ExecutedTrade::SpotTrade(trade) => assert_eq!(trade.price.to_f64(), 97_000.0),
        ExecutedTrade::PrepTrade(_) => panic!("expected spot trade result"),
    }
    match &result.trades[4] {
        ExecutedTrade::SpotTrade(trade) => assert_eq!(trade.price.to_f64(), 96_000.0),
        ExecutedTrade::PrepTrade(_) => panic!("expected spot trade result"),
    }
}

#[test]
#[should_panic(expected = "not yet implemented: spot command extraction")]
fn extracted_spot_module_dispatch_is_used_for_spot_commands() {
    let handler = ExecuteTradingBatchHandler::new();

    let _ = handler.cmd_handle(
        vec![spot_place_order(100, 999, SpotSide::Buy, 100_000, 1)],
        |writes, _| writes.clone(),
    );
}
