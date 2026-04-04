use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, SubmitTradingCommandHandler, TransferCmd,
    TreasuryCommand,
};

#[test]
fn treasury_transfer_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = ExchangeCommandEnvelope {
        command_id: 402,
        trader_id: 21,
        nonce: 3,
        timestamp_ns: 40_002,
        command: ExchangeCommand::TreasuryCommand(TreasuryCommand::Transfer(TransferCmd {
            trader_id: 21,
            asset: "USDC".into(),
            amount: 2_500,
            from_account: "spot".into(),
            to_account: "perp".into(),
        })),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
