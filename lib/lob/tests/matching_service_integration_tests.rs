/// 集成测试：MatchingService
///
/// 测试订单匹配服务的各种场景，包括：
/// - 基础匹配功能
/// - 价格-时间优先规则
/// - 部分成交
/// - 多价格级别匹配
/// - 订单簿状态管理

use lob::lob::{
    InMemoryOrderRepository, MatchingService, OrderEntry, OrderRepository, Side, TraderId,
};

// ==================== 辅助函数 ====================

/// 创建测试用的匹配服务
fn create_matching_service() -> MatchingService<InMemoryOrderRepository> {
    let repository = InMemoryOrderRepository::new(100_000, 10_000);
    MatchingService::new(repository)
}

/// 添加订单到订单簿
fn add_order(
    service: &mut MatchingService<InMemoryOrderRepository>,
    trader: TraderId,
    side: Side,
    price: u32,
    quantity: u32,
) -> u64 {
    let order_id = service.repository_mut().allocate_order_id();
    let entry = OrderEntry::new(order_id, trader, quantity);
    service
        .repository_mut()
        .add_order(order_id, entry, side, price)
        .unwrap();
    order_id
}

// ==================== 基础匹配测试 ====================

#[test]
fn test_simple_buy_match() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单
    add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 匹配买单
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 10000);
    assert_eq!(trades[0].quantity, 100);
    assert_eq!(trades[0].buyer, buyer);
    assert_eq!(trades[0].seller, seller);
    assert_eq!(remaining, 0);
}

#[test]
fn test_simple_sell_match() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加买单
    add_order(&mut service, buyer, Side::Buy, 10000, 100);

    // 匹配卖单
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 10000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 10000);
    assert_eq!(trades[0].quantity, 100);
    assert_eq!(remaining, 0);
}

#[test]
fn test_no_match_price_too_low() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单 @ 10100
    add_order(&mut service, seller, Side::Sell, 10100, 100);

    // 买单出价太低 @ 10000
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);
}

#[test]
fn test_no_match_price_too_high() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加买单 @ 9900
    add_order(&mut service, buyer, Side::Buy, 9900, 100);

    // 卖单要价太高 @ 10000
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 10000, 100);

    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);
}

// ==================== 部分成交测试 ====================

#[test]
fn test_partial_fill_buyer_side() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加小卖单
    add_order(&mut service, seller, Side::Sell, 10000, 50);

    // 大买单部分成交
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 50);
    assert_eq!(remaining, 50); // 剩余50未成交
}

#[test]
fn test_partial_fill_seller_side() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加小买单
    add_order(&mut service, buyer, Side::Buy, 10000, 50);

    // 大卖单部分成交
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 10000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 50);
    assert_eq!(remaining, 50);
}

#[test]
fn test_exact_match() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单
    add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 完全匹配的买单
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 100);
    assert_eq!(remaining, 0);

    // 验证订单簿为空
    assert!(service.repository().is_price_empty(10000, Side::Sell));
}

// ==================== 多价格级别匹配 ====================

#[test]
fn test_multiple_price_levels_buy() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加多个价格级别的卖单
    add_order(&mut service, seller, Side::Sell, 10000, 50);
    add_order(&mut service, seller, Side::Sell, 10100, 50);
    add_order(&mut service, seller, Side::Sell, 10200, 50);

    // 买单愿意支付更高价格，应该从低到高匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10200, 120);

    assert_eq!(trades.len(), 3);
    assert_eq!(trades[0].price, 10000); // 先匹配最低价
    assert_eq!(trades[0].quantity, 50);
    assert_eq!(trades[1].price, 10100);
    assert_eq!(trades[1].quantity, 50);
    assert_eq!(trades[2].price, 10200);
    assert_eq!(trades[2].quantity, 20);
    assert_eq!(remaining, 0);
}

#[test]
fn test_multiple_price_levels_sell() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加多个价格级别的买单
    add_order(&mut service, buyer, Side::Buy, 10200, 50);
    add_order(&mut service, buyer, Side::Buy, 10100, 50);
    add_order(&mut service, buyer, Side::Buy, 10000, 50);

    // 卖单愿意接受更低价格，应该从高到低匹配
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 10000, 120);

    assert_eq!(trades.len(), 3);
    assert_eq!(trades[0].price, 10200); // 先匹配最高价
    assert_eq!(trades[0].quantity, 50);
    assert_eq!(trades[1].price, 10100);
    assert_eq!(trades[1].quantity, 50);
    assert_eq!(trades[2].price, 10000);
    assert_eq!(trades[2].quantity, 20);
    assert_eq!(remaining, 0);
}

// ==================== 价格优先测试 ====================

#[test]
fn test_price_priority_buy_side() {
    let mut service = create_matching_service();

    let seller1 = TraderId::from_str("SELLER1");
    let seller2 = TraderId::from_str("SELLER2");
    let buyer = TraderId::from_str("BUYER");

    // 添加不同价格的卖单
    add_order(&mut service, seller1, Side::Sell, 10100, 100); // 高价
    add_order(&mut service, seller2, Side::Sell, 10000, 100); // 低价

    // 买单应该先匹配低价卖单
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10100, 150);

    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].price, 10000); // 先匹配低价
    assert_eq!(trades[0].seller, seller2);
    assert_eq!(trades[1].price, 10100);
    assert_eq!(trades[1].seller, seller1);
    assert_eq!(remaining, 0);
}

#[test]
fn test_price_priority_sell_side() {
    let mut service = create_matching_service();

    let buyer1 = TraderId::from_str("BUYER1");
    let buyer2 = TraderId::from_str("BUYER2");
    let seller = TraderId::from_str("SELLER");

    // 添加不同价格的买单
    add_order(&mut service, buyer1, Side::Buy, 9900, 100); // 低价
    add_order(&mut service, buyer2, Side::Buy, 10000, 100); // 高价

    // 卖单应该先匹配高价买单
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 9900, 150);

    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].price, 10000); // 先匹配高价
    assert_eq!(trades[0].buyer, buyer2);
    assert_eq!(trades[1].price, 9900);
    assert_eq!(trades[1].buyer, buyer1);
    assert_eq!(remaining, 0);
}

// ==================== 时间优先测试（FIFO） ====================

#[test]
fn test_time_priority_same_price() {
    let mut service = create_matching_service();

    let seller1 = TraderId::from_str("SELLER1");
    let seller2 = TraderId::from_str("SELLER2");
    let seller3 = TraderId::from_str("SELLER3");
    let buyer = TraderId::from_str("BUYER");

    // 添加相同价格的卖单（时间顺序）
    add_order(&mut service, seller1, Side::Sell, 10000, 50);
    add_order(&mut service, seller2, Side::Sell, 10000, 50);
    add_order(&mut service, seller3, Side::Sell, 10000, 50);

    // 买单应该按时间顺序匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 120);

    assert_eq!(trades.len(), 3);
    assert_eq!(trades[0].seller, seller1); // 最早的订单
    assert_eq!(trades[0].quantity, 50);
    assert_eq!(trades[1].seller, seller2);
    assert_eq!(trades[1].quantity, 50);
    assert_eq!(trades[2].seller, seller3);
    assert_eq!(trades[2].quantity, 20);
    assert_eq!(remaining, 0);
}

// ==================== 价格改善测试 ====================

#[test]
fn test_price_improvement_buy() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 卖单 @ 10000
    add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 买单愿意支付更高价格 @ 11000
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 11000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 10000); // 应该以卖单价格成交（价格改善）
    assert_eq!(trades[0].quantity, 100);
    assert_eq!(remaining, 0);
}

#[test]
fn test_price_improvement_sell() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 买单 @ 10000
    add_order(&mut service, buyer, Side::Buy, 10000, 100);

    // 卖单愿意接受更低价格 @ 9000
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 9000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 10000); // 应该以买单价格成交（价格改善）
    assert_eq!(trades[0].quantity, 100);
    assert_eq!(remaining, 0);
}

// ==================== 订单簿状态测试 ====================

#[test]
fn test_order_book_cleanup_after_match() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单
    let order_id = add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 完全匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(remaining, 0);

    // 验证订单已从订单簿移除
    assert!(service.repository().is_price_empty(10000, Side::Sell));
    assert!(service.repository().find_order(order_id).is_none());
}

#[test]
fn test_partial_match_order_remains() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加大卖单
    let order_id = add_order(&mut service, seller, Side::Sell, 10000, 200);

    // 部分匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 50);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 50);
    assert_eq!(remaining, 0);

    // 验证剩余订单仍在订单簿中
    let remaining_order = service.repository().find_order(order_id).unwrap();
    assert_eq!(remaining_order.quantity, 150); // 200 - 50 = 150
}

// ==================== 大量订单测试 ====================

#[test]
fn test_multiple_orders_at_same_price() {
    let mut service = create_matching_service();

    let buyer = TraderId::from_str("BUYER");

    // 添加多个卖单在同一价格
    for i in 0..10 {
        let seller = TraderId::from_str(&format!("SELLER{}", i));
        add_order(&mut service, seller, Side::Sell, 10000, 10);
    }

    // 匹配大买单
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 10); // 应该匹配所有10个订单
    assert_eq!(trades.iter().map(|t| t.quantity).sum::<u32>(), 100);
    assert_eq!(remaining, 0);
}

#[test]
fn test_deep_order_book() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 建立深度订单簿
    for i in 0..20 {
        let price = 10000 + i * 10;
        add_order(&mut service, seller, Side::Sell, price, 50);
    }

    // 大买单扫过多个价格级别
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10200, 500);

    // 应该匹配前21个价格级别（10000-10200）
    assert!(trades.len() >= 10);
    assert_eq!(trades.iter().map(|t| t.quantity).sum::<u32>(), 500);
    assert_eq!(remaining, 0);

    // 验证第一笔成交是最低价
    assert_eq!(trades[0].price, 10000);
}

// ==================== 边界条件测试 ====================

#[test]
fn test_zero_quantity_remaining() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单
    add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 完全匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(remaining, 0);
    assert!(service.repository().is_price_empty(10000, Side::Sell));
}

#[test]
fn test_empty_order_book() {
    let mut service = create_matching_service();

    let buyer = TraderId::from_str("BUYER");

    // 空订单簿
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);
}

#[test]
fn test_single_match_with_large_spread() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单 @ 10000
    add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 买单愿意支付远超卖价 @ 50000
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 50000, 100);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 10000); // 应该以订单簿价格成交
    assert_eq!(remaining, 0);
}

// ==================== Repository访问测试 ====================

#[test]
fn test_repository_access() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");

    // 通过 repository_mut 添加订单
    let order_id = service.repository_mut().allocate_order_id();
    let entry = OrderEntry::new(order_id, seller, 100);
    service
        .repository_mut()
        .add_order(order_id, entry, Side::Sell, 10000)
        .unwrap();

    // 通过 repository 查询
    assert!(!service.repository().is_price_empty(10000, Side::Sell));
    let found_order = service.repository().find_order(order_id);
    assert!(found_order.is_some());
    assert_eq!(found_order.unwrap().quantity, 100);
}

#[test]
fn test_order_id_allocation() {
    let mut service = create_matching_service();

    let id1 = service.repository_mut().allocate_order_id();
    let id2 = service.repository_mut().allocate_order_id();
    let id3 = service.repository_mut().allocate_order_id();

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
}

// ==================== 取消订单测试 ====================

#[test]
fn test_cancel_order_before_match() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单
    let order_id = add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 取消订单
    assert!(service.repository_mut().cancel_order(order_id));

    // 尝试匹配，应该无匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 100);

    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);
}

#[test]
fn test_cancel_nonexistent_order() {
    let mut service = create_matching_service();

    // 取消不存在的订单
    assert!(!service.repository_mut().cancel_order(999999));
}

// ==================== 性能压力测试 ====================

#[test]
fn test_high_volume_matching() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加1000个小订单
    for _ in 0..1000 {
        add_order(&mut service, seller, Side::Sell, 10000, 1);
    }

    // 一次性匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 1000);

    assert_eq!(trades.len(), 1000);
    assert_eq!(trades.iter().map(|t| t.quantity).sum::<u32>(), 1000);
    assert_eq!(remaining, 0);
}

#[test]
fn test_wide_price_range() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加价格范围很大的订单
    add_order(&mut service, seller, Side::Sell, 1000, 10);
    add_order(&mut service, seller, Side::Sell, 50000, 10);
    add_order(&mut service, seller, Side::Sell, 99000, 10);

    // 匹配买单
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 99000, 30);

    assert_eq!(trades.len(), 3);
    assert_eq!(trades[0].price, 1000); // 从低到高匹配
    assert_eq!(trades[1].price, 50000);
    assert_eq!(trades[2].price, 99000);
    assert_eq!(remaining, 0);
}

// ============================================
// 早期退出优化测试
// ============================================

#[test]
fn test_early_exit_buy_price_too_low() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单，最低卖价为10100
    add_order(&mut service, seller, Side::Sell, 10100, 100);
    add_order(&mut service, seller, Side::Sell, 10200, 100);

    // 买价9900远低于最低卖价10100，应该立即返回
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 9900, 100);

    assert_eq!(trades.len(), 0); // 无成交
    assert_eq!(remaining, 100);  // 全部剩余
}

#[test]
fn test_early_exit_sell_price_too_high() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加买单，最高买价为9900
    add_order(&mut service, buyer, Side::Buy, 9900, 100);
    add_order(&mut service, buyer, Side::Buy, 9800, 100);

    // 卖价10100远高于最高买价9900，应该立即返回
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 10100, 100);

    assert_eq!(trades.len(), 0); // 无成交
    assert_eq!(remaining, 100);  // 全部剩余
}

#[test]
fn test_early_exit_empty_orderbook() {
    let mut service = create_matching_service();

    let trader = TraderId::from_str("TRADER");

    // 空订单簿，买单应该立即返回
    let (trades, remaining) = service.match_limit_order(trader, Side::Buy, 10000, 100);
    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);

    // 空订单簿，卖单应该立即返回
    let (trades, remaining) = service.match_limit_order(trader, Side::Sell, 10000, 100);
    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);
}

#[test]
fn test_boundary_match_at_exact_spread() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 添加卖单在10000
    add_order(&mut service, seller, Side::Sell, 10000, 50);

    // 买单正好在10000，应该匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 10000, 50);

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 10000);
    assert_eq!(trades[0].quantity, 50);
    assert_eq!(remaining, 0);
}

#[test]
fn test_no_match_just_below_ask() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 最低卖价10000
    add_order(&mut service, seller, Side::Sell, 10000, 100);

    // 买价9999，刚好低于最低卖价，不应匹配
    let (trades, remaining) = service.match_limit_order(buyer, Side::Buy, 9999, 100);

    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);
}

#[test]
fn test_no_match_just_above_bid() {
    let mut service = create_matching_service();

    let seller = TraderId::from_str("SELLER");
    let buyer = TraderId::from_str("BUYER");

    // 最高买价10000
    add_order(&mut service, buyer, Side::Buy, 10000, 100);

    // 卖价10001，刚好高于最高买价，不应匹配
    let (trades, remaining) = service.match_limit_order(seller, Side::Sell, 10001, 100);

    assert_eq!(trades.len(), 0);
    assert_eq!(remaining, 100);
}
