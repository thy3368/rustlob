use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    SpotCommand, SpotPlaceOrderCmd, SpotSide, SubmitTradingCommandHandler, TradingCommand,
    TradingCommandEnvelope,
};

#[test]
fn spot_place_order_command_is_accepted_into_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = TradingCommandEnvelope {
        command_id: 100,
        trader_id: 42,
        nonce: 1,
        timestamp_ns: 10_000,
        command: TradingCommand::Spot(SpotCommand::PlaceOrder(SpotPlaceOrderCmd {
            trader_id: 42,
            market: "BTC-USDC".into(),
            side: SpotSide::Buy,
            price: 100_000,
            quantity: 2,
        })),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
