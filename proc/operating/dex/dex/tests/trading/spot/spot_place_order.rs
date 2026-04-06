use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, ProductType, SpotCommand, SpotPlaceOrderCmd,
    SpotSide, SubmitTradingCommandHandler, TradingCommand,
};

#[test]
fn spot_place_order_command_is_accepted_into_pending_queue() {
    let handler = SubmitTradingCommandHandler::new();

    let cmd = ExchangeCommandEnvelope {
        command_id: 100,
        trader_id: 42,
        nonce: 1,
        timestamp_ns: 10_000,
        product_type: ProductType::Spot,
        command: ExchangeCommand::TradingCommand(TradingCommand::Spot(SpotCommand::PlaceOrder(
            SpotPlaceOrderCmd {
                trader_id: 42,
                market: "BTC-USDT".into(),
                side: SpotSide::Buy,
                price: 100_000,
                quantity: 2,
            },
        ))),
    };

    let result = handler.cmd_handle(cmd, |writes, _| writes.clone()).unwrap();

    //todo 怎么验证状态 会有一个现货委托单 还是在命令执行后才会有委托单？

    assert!(result.accepted);
    assert_eq!(result.queue_len, 1);
    assert_eq!(handler.pending_len(), 1);
}
