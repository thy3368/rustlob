use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    OrderSide, PlaceOrderCmd, SubmitTradingCommandHandler, TradingCommand,
    TradingCommandEnvelope,
};

#[test]
fn place_order_command_is_accepted_into_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = TradingCommandEnvelope {
        command_id: 1,
        trader_id: 42,
        nonce: 1,
        timestamp_ns: 1_000,
        command: TradingCommand::PlaceOrder(PlaceOrderCmd {
            trader_id: 42,
            market: "BTC-PERP".into(),
            side: OrderSide::Buy,
            price: 100_000,
            quantity: 2,
        }),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
