//! 交易员使用 MatchingService 的简化示例
//!
//! 本示例展示了交易员如何使用订单匹配服务的基本概念
//! 注意：这是一个概念演示，实际运行需要完整的基础设施实现

use lob::lob::{
    Cmd, OrderId, OrderStatus, Price, Quantity, Side, SpotCmdAny, SpotCmdResult, Symbol, TimeInForce, TraderId
};

fn main() {
    println!("=== 交易员使用 MatchingService 示例 ===\n");

    // 1. 创建交易员身份
    let trader_id = TraderId::new([1u8; 8]);
    println!("交易员ID: {:?}", trader_id);

    // 2. 创建交易对符号
    let symbol = Symbol::from_str("BTCUSDT");
    println!("交易对: {:?}\n", symbol);

    // 3. 场景1：限价买单（GTC）
    println!("--- 场景1: 限价买单 (GTC) ---");
    let limit_buy_order = SpotCmdAny::LimitOrder {
        trader: trader_id,
        trading_pair: symbol,
        side: Side::Buy,
        price: 50000,   // 50000 USDT
        quantity: 1500, // 1.5 BTC (假设精度为1000)
        time_in_force: TimeInForce::GoodTillCancel,
        client_order_id: Some("CLIENT-BUY-001".to_string())
    };

    let cmd = Cmd::new(1001, limit_buy_order);
    println!("命令: {:?}", cmd);
    println!("说明: 以 50000 USDT 的价格买入 1.5 BTC");
    println!("有效期: GTC (撤单前一直有效)\n");

    // 4. 场景2：限价卖单（PostOnly）
    println!("--- 场景2: 限价卖单 (PostOnly) ---");
    let limit_sell_order = SpotCmdAny::LimitOrder {
        trader: trader_id,
        trading_pair: symbol,
        side: Side::Sell,
        price: 50100,   // 50100 USDT
        quantity: 2000, // 2.0 BTC
        time_in_force: TimeInForce::PostOnly,
        client_order_id: Some("CLIENT-SELL-001".to_string())
    };

    let cmd = Cmd::new(1002, limit_sell_order);
    println!("命令: {:?}", cmd);
    println!("说明: 以 50100 USDT 的价格卖出 2.0 BTC");
    println!("有效期: PostOnly (只做 Maker，不吃单)\n");

    // 5. 场景3：市价买单
    println!("--- 场景3: 市价买单 ---");
    let market_buy_order = SpotCmdAny::MarketOrder {
        trader: trader_id,
        symbol,
        side: Side::Buy,
        quantity: 1000,           // 1.0 BTC
        price_limit: Some(51000), // 价格保护：最高 51000 USDT
        time_in_force: None,      // 默认 IOC
        client_order_id: Some("CLIENT-MARKET-001".to_string())
    };

    let cmd = Cmd::new(1003, market_buy_order);
    println!("命令: {:?}", cmd);
    println!("说明: 立即买入 1.0 BTC，价格保护最高 51000 USDT\n");

    // 6. 场景4：取消订单
    println!("--- 场景4: 取消订单 ---");
    let cancel_order = SpotCmdAny::CancelOrder {
        order_id: 12345
    };

    let cmd = Cmd::new(1004, cancel_order);
    println!("命令: {:?}", cmd);
    println!("说明: 取消订单 ID 12345\n");

    // 7. 订单状态说明
    println!("--- 订单状态说明 ---");
    println!("Initial: 初始状态");
    println!("Pending: 已挂单，等待成交");
    println!("PartiallyFilled: 部分成交");
    println!("Filled: 完全成交");
    println!("Cancelled: 已取消");
    println!("Rejected: 被拒绝（如 PostOnly 会立即成交）");
    println!("Expired: 已过期（GTD）\n");

    // 8. TimeInForce 说明
    println!("--- TimeInForce 说明 ---");
    println!("GTC (Good Till Cancel): 撤单前一直有效");
    println!("IOC (Immediate Or Cancel): 立即成交，未成交部分取消");
    println!("FOK (Fill Or Kill): 全部成交或全部拒绝");
    println!("PostOnly: 只做 Maker，不吃单");
    println!("GTD (Good Till Date): 有效至指定时间\n");

    // 9. 实际使用流程
    println!("--- 实际使用流程 ---");
    println!("1. 创建 MatchingService 实例");
    println!("2. 创建交易员和订单命令");
    println!("3. 调用 matching_service.handle(command)");
    println!("4. 处理返回的 CommandResponse");
    println!("5. 根据订单状态决定后续操作\n");

    // 10. 示例：处理命令响应
    println!("--- 示例：处理命令响应 ---");
    println!("match result {{");
    println!("    Ok(response) => {{");
    println!("        match response.result {{");
    println!("            SpotCommandResult::LimitOrder {{ order_id, status, .. }} => {{");
    println!("                match status {{");
    println!("                    OrderStatus::Filled => println!(\"✅ 订单已完全成交\"),");
    println!("                    OrderStatus::PartiallyFilled => println!(\"⚠️ 订单部分成交\"),");
    println!("                    OrderStatus::Pending => println!(\"⏳ 订单已挂单\"),");
    println!("                    OrderStatus::Rejected => println!(\"❌ 订单被拒绝\"),");
    println!("                    _ => {{}}");
    println!("                }}");
    println!("            }}");
    println!("            _ => {{}}");
    println!("        }}");
    println!("    }}");
    println!("    Err(e) => println!(\"❌ 错误: {{}}\", e),");
    println!("}}\n");

    println!("=== 示例结束 ===");
    println!("\n提示：");
    println!("- 查看完整文档: lib/core/exchange/lob_repo/examples/TRADER_GUIDE.md");
    println!("- 查看完整代码: lib/core/exchange/lob_repo/examples/trader_example.rs");
    println!("- 运行完整测试: cargo test --example trader_example");
}
