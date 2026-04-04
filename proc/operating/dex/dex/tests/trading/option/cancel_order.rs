use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    CancelOrderCmd, ExchangeCommand, ExchangeCommandEnvelope, OptionCommand, OptionKind,
    OptionPlaceOrderCmd, OptionSide, SubmitTradingCommandHandler, TradingCommand,
};

#[test]
fn option_cancel_order_command_can_enter_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    handler
        .cmd_handle(
            ExchangeCommandEnvelope {
                command_id: 300,
                trader_id: 99,
                nonce: 1,
                timestamp_ns: 30_000,
                command: ExchangeCommand::TradingCommand(TradingCommand::Option(
                    OptionCommand::PlaceOrder(OptionPlaceOrderCmd {
                        trader_id: 99,
                        underlying: "BTC".into(),
                        expiry_ts: 1_750_000_000,
                        strike_price: 110_000,
                        kind: OptionKind::Call,
                        side: OptionSide::Buy,
                        premium: 2_000,
                        quantity: 1,
                    }),
                )),
            },
            |writes, _| writes.clone(),
        )
        .unwrap();

    let cmd = ExchangeCommandEnvelope {
        command_id: 301,
        trader_id: 99,
        nonce: 2,
        timestamp_ns: 30_001,
        command: ExchangeCommand::TradingCommand(TradingCommand::CancelOrder(CancelOrderCmd {
            trader_id: 99,
            order_id: 300,
        })),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 2);
    assert_eq!(handler.pending_len(), 2);
}
