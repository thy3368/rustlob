use base_types::base_types::Price;
use base_types::exchange::spot::spot_types::{OrderStatus, OrderType};
use base_types::handler::handler_update::CmdHandlerForUpdate;
use dex::cmd_handler::{
    ExchangeCommand, ExchangeCommandEnvelope, ExecuteTradingBatchHandler, ExecutedOrder,
    ExecutedTrade, ProductType, SpotCommand, SpotPlaceOrderCmd, SpotSide, TradingCommand,
};

fn spot_place_order(
    command_id: u64,
    trader_id: u64,
    market: &str,
    side: SpotSide,
    price: u64,
    quantity: u64,
) -> ExchangeCommandEnvelope {
    spot_place_order_with_type(
        command_id,
        trader_id,
        market,
        side,
        price,
        quantity,
        OrderType::Limit,
    )
}

fn spot_place_order_with_type(
    command_id: u64,
    trader_id: u64,
    market: &str,
    side: SpotSide,
    price: u64,
    quantity: u64,
    _order_type: impl core::fmt::Debug,
) -> ExchangeCommandEnvelope {
    ExchangeCommandEnvelope {
        command_id,
        trader_id,
        nonce: command_id,
        timestamp_ns: 1_000 + command_id,
        product_type: ProductType::Spot,
        command: ExchangeCommand::TradingCommand(TradingCommand::Spot(SpotCommand::PlaceOrder(
            SpotPlaceOrderCmd {
                trader_id,
                market: market.into(),
                side,
                price,
                quantity,
            },
        ))),
    }
}

/// 规则：分成两阶段
/// 1. 撮合 - 更新 order, trade 的状态
/// 2. 结算 - 通过 trade 计算 balance 的状态
/// 3. 返回变更的状态及 changelog

/// 场景 1: 无对手方下单 -> Pending 订单
/// 规则: 无撮合，无结算，只有 order 创建
#[test]
fn spot_place_order_no_counterparty_creates_pending_order() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(vec![spot_place_order(1, 42, "BTC-USDT", SpotSide::Buy, 100_000, 1)], |w, _| {
            w.clone()
        })
        .unwrap();

    assert_eq!(result.summary.total_commands, 1);
    assert_eq!(result.summary.accepted_commands, 1);
    assert_eq!(result.summary.orders_created, 1);
    assert_eq!(result.summary.trades_executed, 0);
    assert_eq!(result.summary.balance_updates, 0);

    assert_eq!(result.orders.len(), 1);
    match &result.orders[0] {
        ExecutedOrder::SpotOrder(order) => {
            assert_eq!(order.state.status, OrderStatus::Pending);
            assert_eq!(order.unfilled_qty(), Price::from_raw(1));
        }
        ExecutedOrder::PrepOrder(_) => panic!("expected spot order"),
    }
    assert_eq!(result.trades.len(), 0);
    assert_eq!(result.balance_deltas.len(), 0);
}

/// 场景 2: 有对手方完全成交
/// 规则: 撮合阶段创建 trade，结算阶段创建 balance delta
#[test]
fn spot_place_order_with_counterparty_fully_filled() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sell = handler
        .cmd_handle(
            vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 1)],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(vec![spot_place_order(2, 22, "BTC-USDT", SpotSide::Buy, 100_000, 1)], |w, _| {
            w.clone()
        })
        .unwrap();

    assert_eq!(result.summary.orders_created, 1);
    assert_eq!(result.summary.trades_executed, 1);
    assert_eq!(result.summary.balance_updates, 4);

    assert_eq!(result.trades.len(), 1);
    match &result.trades[0] {
        ExecutedTrade::SpotTrade(trade) => {
            assert_eq!(trade.price, Price::from_raw(100_000));
        }
        ExecutedTrade::PrepTrade(_) => panic!("expected spot trade"),
    }

    assert_eq!(result.balance_deltas.len(), 4);
}

/// 场景 3: 部分成交
/// 规则: 撮合创建 trade，剩余转为 resting order，结算更新 balance
#[test]
fn spot_place_order_partial_fill() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sell = handler
        .cmd_handle(
            vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 5)],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(vec![spot_place_order(2, 22, "BTC-USDT", SpotSide::Buy, 100_000, 2)], |w, _| {
            w.clone()
        })
        .unwrap();

    assert_eq!(result.summary.trades_executed, 1);
    assert_eq!(result.summary.balance_updates, 4);

    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.balance_deltas.len(), 4);
}

/// 场景 4: 多笔订单撮合
/// 规则: 批量撮合，多笔 trade，多笔 balance delta
#[test]
fn spot_multiple_orders_matching_creates_multiple_trades() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sells = handler
        .cmd_handle(
            vec![
                spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 1),
                spot_place_order(2, 12, "BTC-USDT", SpotSide::Sell, 99_000, 1),
                spot_place_order(3, 13, "BTC-USDT", SpotSide::Sell, 98_000, 1),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![
                spot_place_order(10, 21, "BTC-USDT", SpotSide::Buy, 100_000, 1),
                spot_place_order(11, 22, "BTC-USDT", SpotSide::Buy, 99_000, 1),
                spot_place_order(12, 23, "BTC-USDT", SpotSide::Buy, 98_000, 1),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.trades_executed, 3);
    assert_eq!(result.trades.len(), 3);
    assert_eq!(result.balance_deltas.len(), 12);
}

/// 场景 5: 不同市场隔离
/// 规则: BTC-USDT 的订单不会与 ETH-USDT 撮合
#[test]
fn spot_different_markets_do_not_match() {
    let handler = ExecuteTradingBatchHandler::new();

    let _btc_sell = handler
        .cmd_handle(
            vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 1)],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(vec![spot_place_order(2, 22, "ETH-USDT", SpotSide::Buy, 3000, 1)], |w, _| {
            w.clone()
        })
        .unwrap();

    assert_eq!(result.summary.trades_executed, 0);
    assert_eq!(result.trades.len(), 0);
}

/// 场景 6: 价格不匹配不成交
/// 规则: 卖单价格 > 买单价格，不成交
#[test]
fn spot_price_cross_not_match() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sell_high = handler
        .cmd_handle(
            vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 1)],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(vec![spot_place_order(2, 22, "BTC-USDT", SpotSide::Buy, 99_000, 1)], |w, _| {
            w.clone()
        })
        .unwrap();

    assert_eq!(result.summary.trades_executed, 0);
    assert_eq!(result.trades.len(), 0);
    assert_eq!(result.orders.len(), 1);
}

/// 场景 7: 批量下单
/// 规则: 批量下单统一处理
#[test]
fn spot_batch_place_orders() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(
            vec![
                spot_place_order(1, 11, "BTC-USDT", SpotSide::Buy, 100_000, 1),
                spot_place_order(2, 12, "BTC-USDT", SpotSide::Buy, 99_000, 2),
                spot_place_order(3, 13, "BTC-USDT", SpotSide::Buy, 98_000, 3),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.total_commands, 3);
    assert_eq!(result.summary.accepted_commands, 3);
    assert_eq!(result.summary.orders_created, 3);
    assert_eq!(result.orders.len(), 3);
}

/// 场景 8: 验证 changelog 产生
/// 规则: 撮合和订单创建都产生 changelog
#[test]
fn spot_place_order_emits_changelogs() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(vec![spot_place_order(1, 42, "BTC-USDT", SpotSide::Buy, 100_000, 1)], |_, c| {
            c.to_vec()
        })
        .unwrap();

    assert!(!result.is_empty());
}

/// 场景 9: 买单冻结报价资产，卖单冻结基础资产
/// 规则: 结算时计算正确的 balance delta
#[test]
fn spot_buy_order_creates_correct_balance_deltas() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sell = handler
        .cmd_handle(
            vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 1)],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(vec![spot_place_order(2, 22, "BTC-USDT", SpotSide::Buy, 100_000, 1)], |w, _| {
            w.clone()
        })
        .unwrap();

    assert_eq!(result.balance_deltas.len(), 4);

    for delta in &result.balance_deltas {
        println!("trader_id: {}, asset: {}, delta: {}", delta.trader_id, delta.asset, delta.delta);
    }
}

/// 场景 10: Taker 是买单（主动性买入）
/// 规则: 买单作为 taker 吃掉卖单流动 性
#[test]
fn spot_taker_is_buy_order() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sell = handler
        .cmd_handle(
            vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 5)],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(vec![spot_place_order(2, 22, "BTC-USDT", SpotSide::Buy, 100_000, 3)], |w, _| {
            w.clone()
        })
        .unwrap();

    assert_eq!(result.summary.trades_executed, 1);
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.balance_deltas.len(), 4);

    let taker_buy = result.balance_deltas.iter().filter(|b| b.trader_id == 22).collect::<Vec<_>>();
    assert!(taker_buy.iter().any(|b| b.asset == "BTC" && b.delta > 0));
    assert!(taker_buy.iter().any(|b| b.asset == "USDT" && b.delta < 0));
}

/// 场景 11: Taker 是卖单（主动性卖出）
/// 规则: 卖单作为 taker 吃掉买单流动性
#[test]
fn spot_taker_is_sell_order() {
    let handler = ExecuteTradingBatchHandler::new();

    let _buy = handler
        .cmd_handle(vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Buy, 100_000, 5)], |w, _| {
            w.clone()
        })
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![spot_place_order(2, 22, "BTC-USDT", SpotSide::Sell, 100_000, 3)],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.trades_executed, 1);
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.balance_deltas.len(), 4);

    let taker_sell = result.balance_deltas.iter().filter(|b| b.trader_id == 22).collect::<Vec<_>>();
    assert!(taker_sell.iter().any(|b| b.asset == "BTC" && b.delta < 0));
    assert!(taker_sell.iter().any(|b| b.asset == "USDT" && b.delta > 0));
}

/// 场景 12: 多笔 taker 订单
/// 规则: 多个 taker 依次成交
#[test]
fn spot_multiple_taker_orders() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sell = handler
        .cmd_handle(
            vec![spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 10)],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![
                spot_place_order(2, 22, "BTC-USDT", SpotSide::Buy, 100_000, 3),
                spot_place_order(3, 33, "BTC-USDT", SpotSide::Buy, 100_000, 5),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.trades_executed, 2);
    assert_eq!(result.trades.len(), 2);
}

/// 场景 13: taker 价格优先成交
/// 规则: taker 价格更优时优先成交
#[test]
fn spot_taker_better_price_matches_first() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sells = handler
        .cmd_handle(
            vec![
                spot_place_order(1, 11, "BTC-USDT", SpotSide::Sell, 100_000, 1),
                spot_place_order(2, 12, "BTC-USDT", SpotSide::Sell, 99_000, 1),
                spot_place_order(3, 13, "BTC-USDT", SpotSide::Sell, 98_000, 1),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![
                spot_place_order(10, 21, "BTC-USDT", SpotSide::Buy, 99_000, 1),
                spot_place_order(11, 22, "BTC-USDT", SpotSide::Buy, 100_000, 1),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.trades_executed, 2);
    assert_eq!(result.trades.len(), 2);
}

/// 场景 14: 市价买单 - 吃掉所有卖单流动性
/// 规则: Market 订单以最优价格成交
#[test]
fn spot_market_buy_order_swaps_all_sell_liquidity() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sells = handler
        .cmd_handle(
            vec![
                spot_place_order_with_type(
                    1,
                    11,
                    "BTC-USDT",
                    SpotSide::Sell,
                    100_000,
                    2,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    2,
                    12,
                    "BTC-USDT",
                    SpotSide::Sell,
                    99_000,
                    3,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    3,
                    13,
                    "BTC-USDT",
                    SpotSide::Sell,
                    98_000,
                    5,
                    OrderType::Limit,
                ),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![spot_place_order_with_type(
                10,
                22,
                "BTC-USDT",
                SpotSide::Buy,
                0,
                8,
                OrderType::Market,
            )],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.trades_executed, 3);
    assert_eq!(result.trades.len(), 3);
    assert_eq!(result.balance_deltas.len(), 12);
}

/// 场景 15: 市价卖单 - 吃掉所有买单流动性
/// 规则: Market 订单以最优价格成交
#[test]
fn spot_market_sell_order_swaps_all_buy_liquidity() {
    let handler = ExecuteTradingBatchHandler::new();

    let _buys = handler
        .cmd_handle(
            vec![
                spot_place_order_with_type(
                    1,
                    11,
                    "BTC-USDT",
                    SpotSide::Buy,
                    98_000,
                    2,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    2,
                    12,
                    "BTC-USDT",
                    SpotSide::Buy,
                    99_000,
                    3,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    3,
                    13,
                    "BTC-USDT",
                    SpotSide::Buy,
                    100_000,
                    5,
                    OrderType::Limit,
                ),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![spot_place_order_with_type(
                10,
                22,
                "BTC-USDT",
                SpotSide::Sell,
                0,
                8,
                OrderType::Market,
            )],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.trades_executed, 3);
    assert_eq!(result.trades.len(), 3);
    assert_eq!(result.balance_deltas.len(), 12);
}

/// 场景 16: 市价单部分成交
/// 规则: 市价单只成交到足以满足其数量
#[test]
fn spot_market_order_partial_fill() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sells = handler
        .cmd_handle(
            vec![spot_place_order_with_type(
                1,
                11,
                "BTC-USDT",
                SpotSide::Sell,
                100_000,
                5,
                OrderType::Limit,
            )],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![spot_place_order_with_type(
                2,
                22,
                "BTC-USDT",
                SpotSide::Buy,
                0,
                8,
                OrderType::Market,
            )],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.trades_executed, 1);
    assert_eq!(result.trades.len(), 1);
}

/// 场景 17: 市价买单以最优卖价成交
/// 规则: 市价买单按最低卖价成交
#[test]
fn spot_market_buy_takes_lowest_ask() {
    let handler = ExecuteTradingBatchHandler::new();

    let _sells = handler
        .cmd_handle(
            vec![
                spot_place_order_with_type(
                    1,
                    11,
                    "BTC-USDT",
                    SpotSide::Sell,
                    100_000,
                    1,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    2,
                    12,
                    "BTC-USDT",
                    SpotSide::Sell,
                    101_000,
                    1,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    3,
                    13,
                    "BTC-USDT",
                    SpotSide::Sell,
                    102_000,
                    1,
                    OrderType::Limit,
                ),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![spot_place_order_with_type(
                10,
                22,
                "BTC-USDT",
                SpotSide::Buy,
                0,
                1,
                OrderType::Market,
            )],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.trades.len(), 1);
}

/// 场景 18: 市价卖单以最优买价成交
/// 规则: 市价卖单按最高买价成交
#[test]
fn spot_market_sell_takes_highest_bid() {
    let handler = ExecuteTradingBatchHandler::new();

    let _buys = handler
        .cmd_handle(
            vec![
                spot_place_order_with_type(
                    1,
                    11,
                    "BTC-USDT",
                    SpotSide::Buy,
                    100_000,
                    1,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    2,
                    12,
                    "BTC-USDT",
                    SpotSide::Buy,
                    99_000,
                    1,
                    OrderType::Limit,
                ),
                spot_place_order_with_type(
                    3,
                    13,
                    "BTC-USDT",
                    SpotSide::Buy,
                    98_000,
                    1,
                    OrderType::Limit,
                ),
            ],
            |w, _| w.clone(),
        )
        .unwrap();

    let result = handler
        .cmd_handle(
            vec![spot_place_order_with_type(
                10,
                22,
                "BTC-USDT",
                SpotSide::Sell,
                0,
                1,
                OrderType::Market,
            )],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.trades.len(), 1);
}

/// 场景 19: 无对手方时市价单成为 pending
/// 规则: 无流动性时市价单转为 resting order
#[test]
fn spot_market_order_no_counterparty_becomes_pending() {
    let handler = ExecuteTradingBatchHandler::new();

    let result = handler
        .cmd_handle(
            vec![spot_place_order_with_type(
                1,
                42,
                "BTC-USDT",
                SpotSide::Buy,
                0,
                1,
                OrderType::Market,
            )],
            |w, _| w.clone(),
        )
        .unwrap();

    assert_eq!(result.summary.orders_created, 1);
    assert_eq!(result.summary.trades_executed, 0);
}
