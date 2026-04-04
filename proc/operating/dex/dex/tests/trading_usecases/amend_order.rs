use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    AmendOrderCmd, SubmitTradingCommandHandler, TradingCommand, TradingCommandEnvelope,
};

#[test]
fn amend_order_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = TradingCommandEnvelope {
        command_id: 20,
        trader_id: 9,
        nonce: 1,
        timestamp_ns: 3_000,
        command: TradingCommand::AmendOrder(AmendOrderCmd {
            trader_id: 9,
            order_id: 123,
            new_price: Some(101_000),
            new_quantity: Some(3),
        }),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
