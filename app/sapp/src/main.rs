use lob::lob::{OrderBook, TraderId, Side};

fn main() {
    println!("=== LOB引擎演示 ===\n");

    // 创建订单簿
    let mut book = OrderBook::new();

    // 创建交易员
    let buyer = TraderId::from_str("BUYER001");
    let seller = TraderId::from_str("SELLER01");

    // 放置卖单
    let (sell_order_id, _) = book.limit_order(seller, Side::Sell, 10000, 100);
    println!("✓ 放置卖单 #{}: 价格=10000, 数量=100", sell_order_id);

    // 放置买单（匹配）
    let (buy_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 50);
    println!("✓ 放置买单 #{}: 价格=10000, 数量=50", buy_order_id);

    // 显示成交
    if !trades.is_empty() {
        println!("\n成交记录:");
        for (i, trade) in trades.iter().enumerate() {
            println!(
                "  {}. {} -> {} @ {} x {}",
                i + 1,
                trade.buyer,
                trade.seller,
                trade.price,
                trade.quantity
            );
        }
    }

    // 显示订单簿状态
    let snapshot = book.snapshot();
    println!("\n订单簿状态:");
    println!("  最佳买价: {:?}", book.best_bid());
    println!("  最佳卖价: {:?}", book.best_ask());
    println!("  价差: {:?}", book.spread());
    println!("  活跃订单数: {}", snapshot.active_orders);
    println!("  总成交数: {}", snapshot.total_trades);

    println!("\n✓ LOB引擎运行正常！");
}
