use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, LiquidatePositionCmd, PerpCommand, ProductType,
    SubmitTradingCommandHandler, TradingCommand,
};

#[test]
fn perp_liquidate_position_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = ExchangeCommandEnvelope {
        command_id: 300,
        trader_id: 9,
        nonce: 1,
        timestamp_ns: 30_000,
        product_type: ProductType::Perp,
        command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
            PerpCommand::LiquidatePosition(LiquidatePositionCmd {
                liquidator_trader_id: 9,
                liquidated_trader_id: 12,
                market: "ETH-PERP".into(),
                quantity: 4,
            }),
        )),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
