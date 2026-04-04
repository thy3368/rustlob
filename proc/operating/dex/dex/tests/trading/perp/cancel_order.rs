use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, PerpCancelOrderCmd, PerpCommand,
    PerpPlaceOrderCmd, PerpSide, SubmitTradingCommandHandler, TradingCommand,
};

#[test]
fn perp_cancel_order_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    handler
        .cmd_handle(
            ExchangeCommandEnvelope {
                command_id: 200,
                trader_id: 7,
                nonce: 1,
                timestamp_ns: 20_000,
                command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                    PerpCommand::PlaceOrder(PerpPlaceOrderCmd {
                        trader_id: 7,
                        market: "BTC-PERP".into(),
                        side: PerpSide::Sell,
                        price: 101_000,
                        quantity: 3,
                        leverage: 5,
                        reduce_only: false,
                    }),
                )),
            },
            |writes, _| writes.clone(),
        )
        .unwrap();

    let cmd = ExchangeCommandEnvelope {
        command_id: 201,
        trader_id: 7,
        nonce: 2,
        timestamp_ns: 20_001,
        command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
            PerpCommand::CancelOrder(PerpCancelOrderCmd {
                trader_id: 7,
                order_id: 200,
            }),
        )),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 2);
    assert_eq!(handler.pending_len(), 2);
}
