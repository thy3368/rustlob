/// LOB引擎集成测试
///
/// 本测试模块验证订单簿引擎的完整功能，包括：
/// - 订单匹配逻辑
/// - 价格-时间优先原则
/// - 部分成交和完全成交场景
/// - 订单取消
/// - 市场深度和价格发现
/// - 高负载场景
/// - 边界条件处理

use lob::lob::engine::{OrderBook, OrderBookSnapshot};
use lob::lob::types::{Price, Quantity, Side, Trade, TraderId};

// ============================================================================
// 测试辅助函数
// ============================================================================

/// 创建交易员ID的辅助函数
fn trader(name: &str) -> TraderId {
    TraderId::from_str(name)
}

/// 验证交易记录
fn verify_trade(trade: &Trade, buyer: TraderId, seller: TraderId, price: Price, qty: Quantity) {
    assert_eq!(trade.buyer, buyer, "买方不匹配");
    assert_eq!(trade.seller, seller, "卖方不匹配");
    assert_eq!(trade.price, price, "价格不匹配");
    assert_eq!(trade.quantity, qty, "数量不匹配");
}

/// 验证订单簿快照
fn verify_snapshot(
    snapshot: &OrderBookSnapshot,
    expected_bid: Option<Price>,
    expected_ask: Option<Price>,
    expected_active: usize,
) {
    assert_eq!(snapshot.bid_max, expected_bid, "最佳买价不匹配");
    assert_eq!(snapshot.ask_min, expected_ask, "最佳卖价不匹配");
    assert_eq!(
        snapshot.active_orders, expected_active,
        "活跃订单数不匹配"
    );
}

// ============================================================================
// 基础功能测试
// ============================================================================

#[test]
fn test_empty_order_book() {
    let book = OrderBook::new();

    assert_eq!(book.best_bid(), None, "空订单簿应无最佳买价");
    assert_eq!(book.best_ask(), None, "空订单簿应无最佳卖价");
    assert_eq!(book.spread(), None, "空订单簿应无价差");
    assert_eq!(book.mid_price(), None, "空订单簿应无中间价");
    assert_eq!(book.next_order_id(), 1, "首个订单ID应为1");

    let snapshot = book.snapshot();
    verify_snapshot(&snapshot, None, None, 0);
}

#[test]
fn test_single_buy_order() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");

    let (order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(order_id, 1, "首个订单ID应为1");
    assert_eq!(trades.len(), 0, "无对手单时不应产生成交");
    assert_eq!(book.best_bid(), Some(10000), "最佳买价应为10000");
    assert_eq!(book.best_ask(), None, "无卖单时最佳卖价应为空");

    let snapshot = book.snapshot();
    verify_snapshot(&snapshot, Some(10000), None, 1);
}

#[test]
fn test_single_sell_order() {
    let mut book = OrderBook::new();
    let seller = trader("SELLER01");

    let (order_id, trades) = book.limit_order(seller, Side::Sell, 10100, 200);

    assert_eq!(order_id, 1);
    assert_eq!(trades.len(), 0);
    assert_eq!(book.best_bid(), None);
    assert_eq!(book.best_ask(), Some(10100), "最佳卖价应为10100");

    let snapshot = book.snapshot();
    verify_snapshot(&snapshot, None, Some(10100), 1);
}

// ============================================================================
// 订单匹配测试
// ============================================================================

#[test]
fn test_exact_match() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 放置卖单
    let (sell_order_id, sell_trades) = book.limit_order(seller, Side::Sell, 10000, 100);
    assert_eq!(sell_order_id, 1);
    assert_eq!(sell_trades.len(), 0);

    // 放置完全匹配的买单
    let (buy_order_id, buy_trades) = book.limit_order(buyer, Side::Buy, 10000, 100);
    assert_eq!(buy_order_id, 2);
    assert_eq!(buy_trades.len(), 1, "应产生1笔成交");

    verify_trade(&buy_trades[0], buyer, seller, 10000, 100);

    // 订单簿应为空
    assert_eq!(book.best_bid(), None);
    assert_eq!(book.best_ask(), None);

    let snapshot = book.snapshot();
    verify_snapshot(&snapshot, None, None, 0);
}

#[test]
fn test_partial_fill_buyer_side() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 放置大卖单
    book.limit_order(seller, Side::Sell, 10000, 500);

    // 放置小买单
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 200);

    assert_eq!(trades.len(), 1, "应产生1笔成交");
    verify_trade(&trades[0], buyer, seller, 10000, 200);

    // 卖单应还有剩余
    assert_eq!(book.best_ask(), Some(10000));
    assert_eq!(book.best_bid(), None);

    let snapshot = book.snapshot();
    verify_snapshot(&snapshot, None, Some(10000), 1);
}

#[test]
fn test_partial_fill_seller_side() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 放置小卖单
    book.limit_order(seller, Side::Sell, 10000, 150);

    // 放置大买单
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 400);

    assert_eq!(trades.len(), 1, "应产生1笔成交");
    verify_trade(&trades[0], buyer, seller, 10000, 150);

    // 买单应有剩余
    assert_eq!(book.best_bid(), Some(10000));
    assert_eq!(book.best_ask(), None);

    let snapshot = book.snapshot();
    verify_snapshot(&snapshot, Some(10000), None, 1);
}

#[test]
fn test_multiple_fills_same_price() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller1 = trader("SELLER01");
    let seller2 = trader("SELLER02");
    let seller3 = trader("SELLER03");

    // 在同一价格放置多个卖单
    book.limit_order(seller1, Side::Sell, 10000, 100);
    book.limit_order(seller2, Side::Sell, 10000, 150);
    book.limit_order(seller3, Side::Sell, 10000, 200);

    // 放置大买单吃掉所有卖单
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 450);

    assert_eq!(trades.len(), 3, "应产生3笔成交");
    verify_trade(&trades[0], buyer, seller1, 10000, 100);
    verify_trade(&trades[1], buyer, seller2, 10000, 150);
    verify_trade(&trades[2], buyer, seller3, 10000, 200);

    // 订单簿应为空
    let snapshot = book.snapshot();
    verify_snapshot(&snapshot, None, None, 0);
}

#[test]
fn test_price_improvement_buy_side() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 卖单价格10000
    book.limit_order(seller, Side::Sell, 10000, 100);

    // 买单愿意支付更高价格11000
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 11000, 100);

    assert_eq!(trades.len(), 1);
    // 应该以卖方价格成交（价格改善）
    verify_trade(&trades[0], buyer, seller, 10000, 100);
}

#[test]
fn test_price_improvement_sell_side() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 买单价格10000
    book.limit_order(buyer, Side::Buy, 10000, 100);

    // 卖单愿意以更低价格9000卖出
    let (_order_id, trades) = book.limit_order(seller, Side::Sell, 9000, 100);

    assert_eq!(trades.len(), 1);
    // 应该以买方价格成交（价格改善）
    verify_trade(&trades[0], buyer, seller, 10000, 100);
}

#[test]
fn test_multiple_price_levels_buy() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller1 = trader("SELLER01");
    let seller2 = trader("SELLER02");
    let seller3 = trader("SELLER03");

    // 在不同价格放置卖单
    book.limit_order(seller1, Side::Sell, 9900, 100); // 最低价
    book.limit_order(seller2, Side::Sell, 10000, 150);
    book.limit_order(seller3, Side::Sell, 10100, 200);

    // 买单愿意支付10050，应匹配两个价格级别
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10050, 300);

    assert_eq!(trades.len(), 2, "应产生2笔成交");
    verify_trade(&trades[0], buyer, seller1, 9900, 100); // 先匹配最低价
    verify_trade(&trades[1], buyer, seller2, 10000, 150); // 再匹配次低价

    // 还剩下50数量未成交，应在买单侧
    assert_eq!(book.best_bid(), Some(10050));
    assert_eq!(book.best_ask(), Some(10100));
}

#[test]
fn test_multiple_price_levels_sell() {
    let mut book = OrderBook::new();
    let seller = trader("SELLER01");
    let buyer1 = trader("BUYER001");
    let buyer2 = trader("BUYER002");
    let buyer3 = trader("BUYER003");

    // 在不同价格放置买单
    book.limit_order(buyer1, Side::Buy, 10100, 100); // 最高价
    book.limit_order(buyer2, Side::Buy, 10000, 150);
    book.limit_order(buyer3, Side::Buy, 9900, 200);

    // 卖单愿意接受9950，应匹配两个价格级别
    let (_order_id, trades) = book.limit_order(seller, Side::Sell, 9950, 300);

    assert_eq!(trades.len(), 2, "应产生2笔成交");
    verify_trade(&trades[0], buyer1, seller, 10100, 100); // 先匹配最高价
    verify_trade(&trades[1], buyer2, seller, 10000, 150); // 再匹配次高价

    // 还剩下50数量未成交，应在卖单侧
    assert_eq!(book.best_bid(), Some(9900));
    assert_eq!(book.best_ask(), Some(9950));
}

// ============================================================================
// 价格-时间优先测试
// ============================================================================

#[test]
fn test_fifo_same_price() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller1 = trader("SELLER01");
    let seller2 = trader("SELLER02");
    let seller3 = trader("SELLER03");

    // 同一价格的三个卖单（FIFO顺序）
    book.limit_order(seller1, Side::Sell, 10000, 100);
    book.limit_order(seller2, Side::Sell, 10000, 100);
    book.limit_order(seller3, Side::Sell, 10000, 100);

    // 部分成交，验证FIFO
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 150);

    assert_eq!(trades.len(), 2, "应产生2笔成交");
    verify_trade(&trades[0], buyer, seller1, 10000, 100); // 先进先出
    verify_trade(&trades[1], buyer, seller2, 10000, 50); // 第二个订单部分成交
}

#[test]
fn test_price_priority() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller1 = trader("SELLER01");
    let seller2 = trader("SELLER02");

    // 不同价格的卖单
    book.limit_order(seller1, Side::Sell, 10100, 100); // 高价先放
    book.limit_order(seller2, Side::Sell, 10000, 100); // 低价后放

    // 买单应先匹配低价
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10200, 100);

    assert_eq!(trades.len(), 1);
    verify_trade(&trades[0], buyer, seller2, 10000, 100); // 价格优先
}

// ============================================================================
// 订单取消测试
// ============================================================================

#[test]
fn test_cancel_unfilled_order() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");

    let (order_id, _trades) = book.limit_order(buyer, Side::Buy, 10000, 100);
    assert_eq!(book.best_bid(), Some(10000));

    // 取消订单
    let cancelled = book.cancel_order(order_id);
    assert!(cancelled, "订单应成功取消");

    // 验证订单簿状态
    let snapshot = book.snapshot();
    assert_eq!(snapshot.active_orders, 0, "取消后应无活跃订单");
}

#[test]
fn test_cancel_nonexistent_order() {
    let mut book = OrderBook::new();

    // 尝试取消不存在的订单
    let cancelled = book.cancel_order(99999);
    assert!(!cancelled, "不存在的订单应返回false");
}

#[test]
fn test_cancel_already_cancelled() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");

    let (order_id, _trades) = book.limit_order(buyer, Side::Buy, 10000, 100);

    // 第一次取消
    assert!(book.cancel_order(order_id));

    // 第二次取消同一订单
    let cancelled_again = book.cancel_order(order_id);
    assert!(!cancelled_again, "已取消的订单不应再次取消");
}

#[test]
fn test_cancel_partially_filled_order() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 放置大买单
    let (buy_order_id, _trades) = book.limit_order(buyer, Side::Buy, 10000, 500);

    // 部分成交
    book.limit_order(seller, Side::Sell, 10000, 200);

    // 取消剩余部分
    let cancelled = book.cancel_order(buy_order_id);
    assert!(cancelled, "部分成交后的订单应可取消");
}

// ============================================================================
// 市场深度和价差测试
// ============================================================================

#[test]
fn test_spread_calculation() {
    let mut book = OrderBook::new();

    // 设置买卖价差
    book.limit_order(trader("B1"), Side::Buy, 9900, 100);
    book.limit_order(trader("S1"), Side::Sell, 10100, 100);

    assert_eq!(book.best_bid(), Some(9900));
    assert_eq!(book.best_ask(), Some(10100));
    assert_eq!(book.spread(), Some(200), "价差应为200");
    assert_eq!(book.mid_price(), Some(10000), "中间价应为10000");
}

#[test]
fn test_mid_price_calculation() {
    let mut book = OrderBook::new();

    book.limit_order(trader("B1"), Side::Buy, 9950, 100);
    book.limit_order(trader("S1"), Side::Sell, 10050, 100);

    assert_eq!(book.mid_price(), Some(10000));
}

#[test]
fn test_spread_when_crossed() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 卖单
    book.limit_order(seller, Side::Sell, 10000, 100);

    // 买单价格高于卖单，应立即成交
    book.limit_order(buyer, Side::Buy, 10100, 100);

    // 成交后订单簿为空，无价差
    assert_eq!(book.spread(), None);
}

#[test]
fn test_best_bid_updates() {
    let mut book = OrderBook::new();

    // 逐步提高买价
    book.limit_order(trader("B1"), Side::Buy, 9900, 100);
    assert_eq!(book.best_bid(), Some(9900));

    book.limit_order(trader("B2"), Side::Buy, 10000, 100);
    assert_eq!(book.best_bid(), Some(10000), "最佳买价应更新");

    book.limit_order(trader("B3"), Side::Buy, 9950, 100);
    assert_eq!(book.best_bid(), Some(10000), "最佳买价应保持不变");
}

#[test]
fn test_best_ask_updates() {
    let mut book = OrderBook::new();

    // 逐步降低卖价
    book.limit_order(trader("S1"), Side::Sell, 10100, 100);
    assert_eq!(book.best_ask(), Some(10100));

    book.limit_order(trader("S2"), Side::Sell, 10000, 100);
    assert_eq!(book.best_ask(), Some(10000), "最佳卖价应更新");

    book.limit_order(trader("S3"), Side::Sell, 10050, 100);
    assert_eq!(book.best_ask(), Some(10000), "最佳卖价应保持不变");
}

// ============================================================================
// 边界条件测试
// ============================================================================

#[test]
fn test_zero_quantity() {
    let mut book = OrderBook::new();

    // 注意：零数量订单在实际系统中应被拒绝，这里测试引擎行为
    let (_order_id, trades) = book.limit_order(trader("T1"), Side::Buy, 10000, 0);

    assert_eq!(trades.len(), 0);
}

#[test]
fn test_minimum_quantity() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    book.limit_order(seller, Side::Sell, 10000, 1);
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 1);

    assert_eq!(trades.len(), 1);
    verify_trade(&trades[0], buyer, seller, 10000, 1);
}

#[test]
fn test_large_quantity() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    let large_qty = u32::MAX / 2;

    book.limit_order(seller, Side::Sell, 10000, large_qty);
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, large_qty);

    assert_eq!(trades.len(), 1);
    verify_trade(&trades[0], buyer, seller, 10000, large_qty);
}

#[test]
fn test_minimum_price() {
    let mut book = OrderBook::new();

    let (_order_id, _trades) = book.limit_order(trader("T1"), Side::Buy, 1, 100);

    assert_eq!(book.best_bid(), Some(1));
}

#[test]
fn test_high_price() {
    let mut book = OrderBook::new();

    let high_price = 100_000; // $1000.00
    let (_order_id, _trades) = book.limit_order(trader("T1"), Side::Sell, high_price, 100);

    assert_eq!(book.best_ask(), Some(high_price));
}

// ============================================================================
// 复杂场景测试
// ============================================================================

#[test]
fn test_order_book_buildup() {
    let mut book = OrderBook::new();

    // 构建买卖盘深度
    book.limit_order(trader("B1"), Side::Buy, 9900, 100);
    book.limit_order(trader("B2"), Side::Buy, 9950, 150);
    book.limit_order(trader("B3"), Side::Buy, 10000, 200);

    book.limit_order(trader("S1"), Side::Sell, 10050, 100);
    book.limit_order(trader("S2"), Side::Sell, 10100, 150);
    book.limit_order(trader("S3"), Side::Sell, 10150, 200);

    assert_eq!(book.best_bid(), Some(10000));
    assert_eq!(book.best_ask(), Some(10050));
    assert_eq!(book.spread(), Some(50));

    let snapshot = book.snapshot();
    assert_eq!(snapshot.active_orders, 6);
}

#[test]
fn test_aggressive_sweep() {
    let mut book = OrderBook::new();

    // 构建卖盘
    book.limit_order(trader("S1"), Side::Sell, 10000, 100);
    book.limit_order(trader("S2"), Side::Sell, 10050, 150);
    book.limit_order(trader("S3"), Side::Sell, 10100, 200);
    book.limit_order(trader("S4"), Side::Sell, 10150, 250);

    // 激进买单扫单
    let (_order_id, trades) = book.limit_order(trader("BUYER"), Side::Buy, 10200, 1000);

    assert_eq!(trades.len(), 4, "应成交4笔");

    // 部分剩余应留在买单侧
    assert_eq!(book.best_bid(), Some(10200));
    assert_eq!(book.best_ask(), None);

    let snapshot = book.snapshot();
    assert_eq!(snapshot.active_orders, 1);

    // 验证成交价格
    assert_eq!(trades[0].price, 10000);
    assert_eq!(trades[1].price, 10050);
    assert_eq!(trades[2].price, 10100);
    assert_eq!(trades[3].price, 10150);
}

#[test]
fn test_iceberg_simulation() {
    let mut book = OrderBook::new();
    let buyer = trader("BUYER001");
    let seller = trader("SELLER01");

    // 模拟冰山订单：分批提交
    for _ in 0..10 {
        book.limit_order(seller, Side::Sell, 10000, 50);
    }

    // 大买单
    let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 500);

    assert_eq!(trades.len(), 10, "应产生10笔成交");

    // 验证总成交量
    let total_qty: Quantity = trades.iter().map(|t| t.quantity).sum();
    assert_eq!(total_qty, 500);
}

#[test]
fn test_trade_history() {
    let mut book = OrderBook::new();

    book.limit_order(trader("S1"), Side::Sell, 10000, 100);
    book.limit_order(trader("B1"), Side::Buy, 10000, 50);

    assert_eq!(book.trades().len(), 1, "交易历史应有1笔记录");

    book.limit_order(trader("B2"), Side::Buy, 10000, 50);
    assert_eq!(book.trades().len(), 2, "交易历史应有2笔记录");

    // 清空交易历史
    book.clear_trades();
    assert_eq!(book.trades().len(), 0, "交易历史应已清空");
}

// ============================================================================
// 性能和压力测试
// ============================================================================

#[test]
fn test_high_volume_orders() {
    let mut book = OrderBook::with_capacity(100_000, 10_000);

    // 提交1000个买单（价格范围在8000-8099）
    for i in 1..=1000 {
        book.limit_order(trader("BUYER"), Side::Buy, 8000 + (i % 100), 10);
    }

    // 提交1000个卖单（价格范围在12000-12099）
    for i in 1..=1000 {
        book.limit_order(trader("SELLER"), Side::Sell, 12000 + (i % 100), 10);
    }

    let snapshot = book.snapshot();
    assert!(snapshot.active_orders > 0, "应有活跃订单");
    assert_eq!(snapshot.total_trades, 0, "无交叉订单时不应有成交");
}

#[test]
fn test_order_id_sequence() {
    let mut book = OrderBook::new();

    let (id1, _) = book.limit_order(trader("T1"), Side::Buy, 10000, 100);
    let (id2, _) = book.limit_order(trader("T2"), Side::Sell, 10100, 100);
    let (id3, _) = book.limit_order(trader("T3"), Side::Buy, 9900, 100);

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
    assert_eq!(book.next_order_id(), 4);
}

#[test]
fn test_order_id_restore() {
    let mut book = OrderBook::new();

    book.set_next_order_id(1000);
    let (order_id, _) = book.limit_order(trader("T1"), Side::Buy, 10000, 100);

    assert_eq!(order_id, 1000);
    assert_eq!(book.next_order_id(), 1001);
}

// ============================================================================
// 多交易员场景测试
// ============================================================================

#[test]
fn test_multiple_traders_same_price() {
    let mut book = OrderBook::new();

    // 三个买家在同一价格下单
    let buyer1 = trader("BUYER001");
    let buyer2 = trader("BUYER002");
    let buyer3 = trader("BUYER003");

    book.limit_order(buyer1, Side::Buy, 10000, 100);
    book.limit_order(buyer2, Side::Buy, 10000, 150);
    book.limit_order(buyer3, Side::Buy, 10000, 200);

    // 大卖单
    let seller = trader("SELLER01");
    let (_order_id, trades) = book.limit_order(seller, Side::Sell, 10000, 450);

    assert_eq!(trades.len(), 3);
    assert_eq!(trades[0].buyer, buyer1);
    assert_eq!(trades[1].buyer, buyer2);
    assert_eq!(trades[2].buyer, buyer3);
}

#[test]
fn test_self_trade_prevention_not_implemented() {
    // 注意：当前实现未阻止自成交
    // 此测试记录当前行为，未来可能需要修改
    let mut book = OrderBook::new();
    let trader_id = trader("TRADER01");

    book.limit_order(trader_id, Side::Sell, 10000, 100);
    let (_order_id, trades) = book.limit_order(trader_id, Side::Buy, 10000, 100);

    // 当前会产生自成交
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].buyer, trader_id);
    assert_eq!(trades[0].seller, trader_id);
}

// ============================================================================
// 边界和异常场景
// ============================================================================

#[test]
fn test_alternating_orders() {
    let mut book = OrderBook::new();

    for i in 0..100 {
        if i % 2 == 0 {
            book.limit_order(trader("B"), Side::Buy, 9900 + i, 10);
        } else {
            book.limit_order(trader("S"), Side::Sell, 10100 - i, 10);
        }
    }

    let snapshot = book.snapshot();
    assert!(snapshot.active_orders > 0);
}

#[test]
fn test_snapshot_consistency() {
    let mut book = OrderBook::new();

    let snapshot1 = book.snapshot();
    assert_eq!(snapshot1.active_orders, 0);
    assert_eq!(snapshot1.total_trades, 0);

    book.limit_order(trader("T1"), Side::Buy, 10000, 100);
    let snapshot2 = book.snapshot();
    assert_eq!(snapshot2.active_orders, 1);

    book.limit_order(trader("T2"), Side::Sell, 10000, 100);
    let snapshot3 = book.snapshot();
    assert_eq!(snapshot3.active_orders, 0);
    assert_eq!(snapshot3.total_trades, 1);
}
