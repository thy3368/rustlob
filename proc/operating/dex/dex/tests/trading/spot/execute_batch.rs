use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    AmendOrderCmd, CancelOrderCmd, ExchangeCommand, ExchangeCommandEnvelope,
    ExecuteTradingBatchHandler, OrderSide, PlaceOrderCmd, TradingCommand,
};

#[test]
fn execute_trading_batch_counts_each_command_kind() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(
            vec![
                ExchangeCommandEnvelope {
                    command_id: 1,
                    trader_id: 1,
                    nonce: 1,
                    timestamp_ns: 1_000,
                    command: ExchangeCommand::TradingCommand(TradingCommand::PlaceOrder(
                        PlaceOrderCmd {
                            trader_id: 1,
                            market: "BTC-PERP".into(),
                            side: OrderSide::Buy,
                            price: 100_000,
                            quantity: 1,
                        },
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 2,
                    trader_id: 1,
                    nonce: 2,
                    timestamp_ns: 1_001,
                    command: ExchangeCommand::TradingCommand(TradingCommand::CancelOrder(
                        CancelOrderCmd {
                            trader_id: 1,
                            order_id: 88,
                        },
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 3,
                    trader_id: 1,
                    nonce: 3,
                    timestamp_ns: 1_002,
                    command: ExchangeCommand::TradingCommand(TradingCommand::AmendOrder(
                        AmendOrderCmd {
                            trader_id: 1,
                            order_id: 88,
                            new_price: Some(100_100),
                            new_quantity: Some(2),
                        },
                    )),
                },
            ],
            |writes, _| writes.clone(),
        )
        .unwrap();

    assert_eq!(result.total_commands, 3);
    assert_eq!(result.place_order_commands, 1);
    assert_eq!(result.cancel_order_commands, 1);
    assert_eq!(result.amend_order_commands, 1);
}
