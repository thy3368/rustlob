use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    DepositCmd, ExchangeCommand, ExchangeCommandEnvelope, SubmitTradingCommandHandler,
    TreasuryCommand,
};

#[test]
fn treasury_deposit_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = ExchangeCommandEnvelope {
        command_id: 400,
        trader_id: 21,
        nonce: 1,
        timestamp_ns: 40_000,
        command: ExchangeCommand::TreasuryCommand(TreasuryCommand::Deposit(DepositCmd {
            trader_id: 21,
            asset: "USDC".into(),
            amount: 10_000,
        })),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
