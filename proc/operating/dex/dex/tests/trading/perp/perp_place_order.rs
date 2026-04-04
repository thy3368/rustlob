use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, PerpCommand, PerpPlaceOrderCmd, PerpSide,
    SubmitTradingCommandHandler, TradingCommand,
};

#[test]
fn perp_place_order_command_is_accepted_into_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = ExchangeCommandEnvelope {
        command_id: 200,
        trader_id: 7,
        nonce: 1,
        timestamp_ns: 20_000,
        command: ExchangeCommand::TradingCommand(TradingCommand::Perp(PerpCommand::PlaceOrder(
            PerpPlaceOrderCmd {
                trader_id: 7,
                market: "BTC-PERP".into(),
                side: PerpSide::Sell,
                price: 101_000,
                quantity: 3,
                leverage: 5,
                reduce_only: false,
            },
        ))),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
