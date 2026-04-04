use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, ExecuteTradingBatchHandler, OptionAmendOrderCmd,
    OptionCancelOrderCmd, OptionCommand, OptionKind, OptionPlaceOrderCmd, OptionSide,
    PerpAmendOrderCmd, PerpCommand, PerpPlaceOrderCmd, PerpSide, SpotCancelOrderCmd,
    SpotCommand, SpotPlaceOrderCmd, SpotSide, TradingCommand,
};

#[test]
fn execute_trading_batch_counts_each_command_kind() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(
            vec![
                ExchangeCommandEnvelope {
                    command_id: 1,
                    trader_id: 1,
                    nonce: 1,
                    timestamp_ns: 1_000,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Spot(
                        SpotCommand::PlaceOrder(SpotPlaceOrderCmd {
                            trader_id: 1,
                            market: "BTC-PERP".into(),
                            side: SpotSide::Buy,
                            price: 100_000,
                            quantity: 1,
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 2,
                    trader_id: 1,
                    nonce: 2,
                    timestamp_ns: 1_001,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Spot(
                        SpotCommand::CancelOrder(SpotCancelOrderCmd {
                            trader_id: 1,
                            order_id: 88,
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 3,
                    trader_id: 1,
                    nonce: 3,
                    timestamp_ns: 1_002,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::AmendOrder(PerpAmendOrderCmd {
                            trader_id: 1,
                            order_id: 88,
                            new_price: Some(100_100),
                            new_quantity: Some(2),
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 4,
                    trader_id: 1,
                    nonce: 4,
                    timestamp_ns: 1_003,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Option(
                        OptionCommand::CancelOrder(OptionCancelOrderCmd {
                            trader_id: 1,
                            order_id: 77,
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 5,
                    trader_id: 1,
                    nonce: 5,
                    timestamp_ns: 1_004,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Option(
                        OptionCommand::AmendOrder(OptionAmendOrderCmd {
                            trader_id: 1,
                            order_id: 77,
                            new_price: Some(2_200),
                            new_quantity: Some(3),
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 6,
                    trader_id: 1,
                    nonce: 6,
                    timestamp_ns: 1_005,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::PlaceOrder(PerpPlaceOrderCmd {
                            trader_id: 1,
                            market: "BTC-PERP".into(),
                            side: PerpSide::Sell,
                            price: 101_000,
                            quantity: 3,
                            leverage: 5,
                            reduce_only: false,
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 7,
                    trader_id: 1,
                    nonce: 7,
                    timestamp_ns: 1_006,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Option(
                        OptionCommand::PlaceOrder(OptionPlaceOrderCmd {
                            trader_id: 1,
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
            ],
            |writes, _| writes.clone(),
        )
        .unwrap();

    //todo 怎么验证状态 会有一个现货委托单 和成交单？

    assert_eq!(result.total_commands, 7);
    assert_eq!(result.place_order_commands, 3);
    assert_eq!(result.cancel_order_commands, 2);
    assert_eq!(result.amend_order_commands, 2);
}
