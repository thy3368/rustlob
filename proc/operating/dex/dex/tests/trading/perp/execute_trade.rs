use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExecuteTradingBatchHandler, ExchangeCommand, ExchangeCommandEnvelope, ExecuteTradeCmd,
    PerpCommand, PerpPlaceOrderCmd, PerpSide, TradeExecutionLog, TradeExecutionResult,
    TradingCommand,
};

#[test]
fn execute_trade_command_returns_single_trade_execution() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(
            vec![
                ExchangeCommandEnvelope {
                    command_id: 1,
                    trader_id: 11,
                    nonce: 1,
                    timestamp_ns: 50_000,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::PlaceOrder(PerpPlaceOrderCmd {
                            trader_id: 11,
                            market: "BTC-PERP".into(),
                            side: PerpSide::Buy,
                            price: 100_000,
                            quantity: 2,
                            leverage: 1,
                            reduce_only: false,
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 2,
                    trader_id: 12,
                    nonce: 1,
                    timestamp_ns: 50_001,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::ExecuteTrade(ExecuteTradeCmd {
                            market: "BTC-PERP".into(),
                            maker_order_id: 1,
                            taker_order_id: 2,
                            price: 100_000,
                            quantity: 2,
                        }),
                    )),
                },
            ],
            |writes, changelogs| (writes.clone(), changelogs.to_vec()),
        )
        .unwrap();

    let (writes, changelogs) = result;
    assert_eq!(writes.summary.total_commands, 2);
    assert_eq!(writes.summary.accepted_commands, 2);
    assert_eq!(writes.summary.orders_created, 0);
    assert_eq!(writes.summary.trades_executed, 1);
    assert_eq!(writes.trades.len(), 1);
    assert_eq!(
        writes.trades[0],
        TradeExecutionResult {
            market: "BTC-PERP".into(),
            maker_order_id: 1,
            taker_order_id: 2,
            price: 100_000,
            quantity: 2,
        }
    );
    assert_eq!(
        changelogs,
        vec![
            TradeExecutionLog::TradeExecuted {
                market: "BTC-PERP".into(),
                maker_order_id: 1,
                taker_order_id: 2,
                price: 100_000,
                quantity: 2,
            },
            TradeExecutionLog::BatchExecuted { batch_size: 2 },
        ]
    );
}
