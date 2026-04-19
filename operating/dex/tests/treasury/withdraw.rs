use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, ProductType, SubmitTradingCommandHandler,
    TreasuryCommand, WithdrawCmd,
};

#[test]
fn treasury_withdraw_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = ExchangeCommandEnvelope {
        command_id: 401,
        trader_id: 21,
        nonce: 2,
        timestamp_ns: 40_001,
        product_type: ProductType::Treasury,
        command: ExchangeCommand::TreasuryCommand(TreasuryCommand::Withdraw(WithdrawCmd {
            trader_id: 21,
            asset: "USDC".into(),
            amount: 5_000,
        })),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
