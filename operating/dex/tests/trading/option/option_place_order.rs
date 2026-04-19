use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, OptionCommand, OptionKind, OptionPlaceOrderCmd,
    OptionSide, ProductType, SubmitTradingCommandHandler, TradingCommand,
};

#[test]
fn option_place_order_command_is_accepted_into_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = ExchangeCommandEnvelope {
        command_id: 300,
        trader_id: 99,
        nonce: 1,
        timestamp_ns: 30_000,
        product_type: ProductType::Option,
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
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
