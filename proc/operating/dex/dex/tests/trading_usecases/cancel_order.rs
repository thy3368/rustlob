use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    CancelOrderCmd, SubmitTradingCommandHandler, TradingCommand, TradingCommandEnvelope,
};

#[test]
fn cancel_order_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = TradingCommandEnvelope {
        command_id: 10,
        trader_id: 42,
        nonce: 2,
        timestamp_ns: 2_000,
        command: TradingCommand::CancelOrder(CancelOrderCmd {
            trader_id: 42,
            order_id: 99,
        }),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
