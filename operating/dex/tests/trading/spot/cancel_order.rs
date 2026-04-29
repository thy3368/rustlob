use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, ProductType, SpotCancelOrderCmd, SpotCommand,
    SpotPlaceOrderCmd, SpotSide, SubmitTradingCommandHandler, TradingCommand,
};

#[test]
fn spot_cancel_order_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    handler
        .cmd_handle(
            ExchangeCommandEnvelope {
                command_id: 100,
                trader_id: 42,
                nonce: 1,
                timestamp_ns: 10_000,
                product_type: ProductType::Spot,
                command: ExchangeCommand::TradingCommand(TradingCommand::Spot(
                    SpotCommand::PlaceOrder(SpotPlaceOrderCmd {
                        trader_id: 42,
                        market: "BTC-USDT".into(),
                        side: SpotSide::Buy,
                        price: 100_000,
                        quantity: 2,
                    }),
                )),
            },
            |writes, _| writes.clone(),
        )
        .unwrap();

    let cmd = ExchangeCommandEnvelope {
        command_id: 101,
        trader_id: 42,
        nonce: 2,
        timestamp_ns: 10_001,
        product_type: ProductType::Spot,
        command: ExchangeCommand::TradingCommand(TradingCommand::Spot(SpotCommand::CancelOrder(
            SpotCancelOrderCmd { trader_id: 42, order_id: 100 },
        ))),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 2);
    assert_eq!(handler.pending_len(), 2);
}
