use base_types::{OrderId, Price, Quantity, Side, Symbol};
use lob_repo::adapter::local_lob_impl::LocalLob;
use lob_repo::core::symbol_lob_repo::{Order, SymbolLob};

// 创建模拟订单用于测试
#[derive(Debug, Clone)]
struct MockOrder {
    id: u64,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    side: Side,
}

impl Order for MockOrder {
    fn order_id(&self) -> OrderId {
        self.id
    }

    fn price(&self) -> Price {
        self.price
    }

    fn quantity(&self) -> Quantity {
        self.quantity
    }

    fn side(&self) -> Side {
        self.side
    }

    fn symbol(&self) -> Symbol {
        self.symbol
    }
}

#[test]
fn test_last_price_initially_none() {
    let symbol = Symbol::new("BTCUSDT");
    let lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 初始状态，最后成交价应为 None
    assert_eq!(lob.last_price(), None);
}

#[test]
fn test_update_last_price() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 更新最后成交价 (50000.00 USDT)
    let trade_price = Price::from_f64(50000.0);
    lob.update_last_price(trade_price);

    // 验证最后成交价已更新
    assert_eq!(lob.last_price(), Some(trade_price));
}

#[test]
fn test_multiple_price_updates() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 第一次成交 (50000.00 USDT)
    let price1 = Price::from_f64(50000.0);
    lob.update_last_price(price1);
    assert_eq!(lob.last_price(), Some(price1));

    // 第二次成交 (50100.50 USDT)
    let price2 = Price::from_f64(50100.5);
    lob.update_last_price(price2);
    assert_eq!(lob.last_price(), Some(price2));

    // 第三次成交 (49900.99 USDT)
    let price3 = Price::from_f64(49900.99);
    lob.update_last_price(price3);
    assert_eq!(lob.last_price(), Some(price3));
}

#[test]
fn test_add_and_find_order() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    let order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50000.0),  // 50000.00 USDT
        quantity: Quantity::from_f64(1.5),  // 1.5 BTC
        side: Side::Buy,
    };

    // 添加订单
    let result = lob.add_order(order.clone());
    assert!(result.is_ok());

    // 查找订单
    let found = lob.find_order(1);
    assert!(found.is_some());
    assert_eq!(found.unwrap().order_id(), 1);
}

#[test]
fn test_best_bid_ask() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 初始状态
    assert_eq!(lob.best_bid(), None);
    assert_eq!(lob.best_ask(), None);

    // 添加买单 (50000.00 USDT)
    let buy_order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
    };
    lob.add_order(buy_order).unwrap();
    assert_eq!(lob.best_bid(), Some(Price::from_f64(50000.0)));

    // 添加卖单 (50100.50 USDT)
    let sell_order = MockOrder {
        id: 2,
        symbol,
        price: Price::from_f64(50100.5),
        quantity: Quantity::from_f64(0.5),
        side: Side::Sell,
    };
    lob.add_order(sell_order).unwrap();
    assert_eq!(lob.best_ask(), Some(Price::from_f64(50100.5)));
}

#[test]
fn test_match_orders_buy_side() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 添加多个卖单
    for i in 0..5 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(50000.0 + (i as f64 * 10.0)),
            quantity: Quantity::from_f64(1.0),
            side: Side::Sell,
        };
        lob.add_order(order).unwrap();
    }

    // 匹配买单：应该匹配最低价格的卖单
    let matched = lob.match_orders(
        Side::Buy,
        Price::from_f64(50020.0),
        Quantity::from_f64(2.5),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    assert_eq!(orders.len(), 3);  // 匹配到 3 个订单
}

#[test]
fn test_remove_order() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    let order = MockOrder {
        id: 100,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
    };

    lob.add_order(order).unwrap();
    assert!(lob.find_order(100).is_some());

    // 删除订单
    let removed = lob.remove_order(100);
    assert!(removed);
    assert!(lob.find_order(100).is_none());

    // 删除不存在的订单
    let removed = lob.remove_order(999);
    assert!(!removed);
}
