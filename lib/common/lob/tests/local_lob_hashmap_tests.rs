use base_types::{OrderId, Price, Quantity, Side, TradingPair};
use lob_repo::adapter::local_lob_hashmap_impl::LocalLobHashMap;
use lob_repo::core::symbol_lob_repo::{Order, SymbolLob};

// 创建模拟订单用于测试
#[derive(Debug, Clone, entity_derive::Entity)]
struct MockOrder {
    id: u64,
    #[created(skip)]
    #[replay(skip)]
    symbol: TradingPair,
    #[created(skip)]
    #[replay(skip)]
    price: Price,
    #[created(skip)]
    #[replay(skip)]
    quantity: Quantity,
    #[created(skip)]
    #[replay(skip)]
    filled_quantity: Quantity,
    #[created(skip)]
    #[replay(skip)]
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

    fn filled_quantity(&self) -> Quantity {
        self.filled_quantity
    }

    fn side(&self) -> Side {
        self.side
    }

    fn symbol(&self) -> TradingPair {
        self.symbol
    }
}

#[test]
fn test_btc_high_price_coin() {
    let symbol = TradingPair::new("BTCUSDT");
    // BTC 使用 0.01 tick size
    let mut lob = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加买单 (50000.00 USDT)
    let buy_order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };
    assert!(lob.add_order(buy_order).is_ok());

    // 添加卖单 (50100.50 USDT)
    let sell_order = MockOrder {
        id: 2,
        symbol,
        price: Price::from_f64(50100.5),
        quantity: Quantity::from_f64(0.5),
        side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
    };
    assert!(lob.add_order(sell_order).is_ok());

    // 验证最佳价格
    assert_eq!(lob.best_bid(), Some(Price::from_f64(50000.0)));
    assert_eq!(lob.best_ask(), Some(Price::from_f64(50100.5)));
}

#[test]
fn test_shib_low_price_coin() {
    let symbol = TradingPair::new("SHIBUSDT");
    // SHIB 使用 0.00000001 tick size (8 位小数)
    let mut lob = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.00000001));

    // 添加买单 (0.00001234 USDT)
    let buy_order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(0.00001234),
        quantity: Quantity::from_f64(1000000.0), // 100万 SHIB
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };
    assert!(lob.add_order(buy_order).is_ok());

    // 添加卖单 (0.00001250 USDT)
    let sell_order = MockOrder {
        id: 2,
        symbol,
        price: Price::from_f64(0.00001250),
        quantity: Quantity::from_f64(500000.0), // 50万 SHIB
        side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
    };
    assert!(lob.add_order(sell_order).is_ok());

    // 验证订单已添加
    assert!(lob.find_order(1).is_some());
    assert!(lob.find_order(2).is_some());
}

#[test]
fn test_doge_medium_price_coin() {
    let symbol = TradingPair::new("DOGEUSDT");
    // DOGE 使用 0.0001 tick size (4 位小数)
    let mut lob = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.0001));

    // 添加多个订单
    for i in 0..10 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(0.08 + (i as f64 * 0.001)),
            quantity: Quantity::from_f64(10000.0),
            side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
        filled_quantity: Quantity::from_raw(0),
        };
        assert!(lob.add_order(order).is_ok());
    }

    // 验证订单数量
    for i in 0..10 {
        assert!(lob.find_order(i).is_some());
    }
}

#[test]
fn test_match_orders_hashmap() {
    let symbol = TradingPair::new("BTCUSDT");
    let mut lob = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加多个卖单
    for i in 0..5 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(50000.0 + (i as f64 * 10.0)),
            quantity: Quantity::from_f64(1.0),
            side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
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
fn test_remove_order_hashmap() {
    let symbol = TradingPair::new("BTCUSDT");
    let mut lob = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.01));

    let order = MockOrder {
        id: 100,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };

    lob.add_order(order).unwrap();
    assert!(lob.find_order(100).is_some());

    // 删除订单
    let removed = lob.remove_order(100);
    assert!(removed);
    assert!(lob.find_order(100).is_none());
}

#[test]
fn test_last_price_hashmap() {
    let symbol = TradingPair::new("BTCUSDT");
    let mut lob: LocalLobHashMap<MockOrder> = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 初始状态
    assert_eq!(lob.last_price(), None);

    // 更新成交价
    lob.update_last_price(Price::from_f64(50000.0));
    assert_eq!(lob.last_price(), Some(Price::from_f64(50000.0)));

    // 再次更新
    lob.update_last_price(Price::from_f64(50100.5));
    assert_eq!(lob.last_price(), Some(Price::from_f64(50100.5)));
}

#[test]
fn test_pepe_ultra_low_price() {
    let symbol = TradingPair::new("PEPEUSDT");
    // PEPE 使用最小 tick size
    let mut lob: LocalLobHashMap<MockOrder> = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.00000001));

    // 添加订单 (0.000000123 USDT)
    let order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(0.000000123),
        quantity: Quantity::from_f64(10000000.0), // 1千万 PEPE
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };

    assert!(lob.add_order(order).is_ok());
    assert!(lob.find_order(1).is_some());
}

#[test]
fn test_hashmap_memory_efficiency() {
    let symbol = TradingPair::new("BTCUSDT");
    let mut lob: LocalLobHashMap<MockOrder> = LocalLobHashMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加稀疏分布的订单（不连续的价格）
    let prices = vec![10000.0, 20000.0, 30000.0, 50000.0, 100000.0];

    for (i, &price) in prices.iter().enumerate() {
        let order = MockOrder {
            id: i as u64,
            symbol,
            price: Price::from_f64(price),
            quantity: Quantity::from_f64(1.0),
            side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
        };
        assert!(lob.add_order(order).is_ok());
    }

    // HashMap 只存储实际存在的价格点，内存高效
    for i in 0..prices.len() {
        assert!(lob.find_order(i as u64).is_some());
    }
}
