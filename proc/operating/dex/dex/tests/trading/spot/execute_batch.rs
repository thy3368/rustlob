use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, ExecuteTradingBatchHandler, OrderStatus,
    SpotCommand, SpotPlaceOrderCmd, SpotSide, TradingCommand,
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

    assert_eq!(result.orders[0].status, OrderStatus::Open);
    assert_eq!(result.orders[0].remaining_quantity, 1);
    assert_eq!(result.orders[1].status, OrderStatus::Open);
    assert_eq!(result.orders[1].remaining_quantity, 2);
    assert_eq!(result.orders[2].status, OrderStatus::Open);
    assert_eq!(result.orders[2].remaining_quantity, 3);
    assert_eq!(result.orders[3].status, OrderStatus::Open);
    assert_eq!(result.orders[3].remaining_quantity, 4);
    assert_eq!(result.orders[4].status, OrderStatus::Open);
    assert_eq!(result.orders[4].remaining_quantity, 5);
}

//todo 增加场景2 5条spot order, 都有对手单
