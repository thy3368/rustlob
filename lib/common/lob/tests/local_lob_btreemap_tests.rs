use base_types::{OrderId, Price, Quantity, Side, Symbol};
use lob_repo::adapter::local_lob_btreemap_impl::LocalLobBTreeMap;
use lob_repo::core::symbol_lob_repo::{Order, SymbolLob};

// 创建模拟订单用于测试
#[derive(Debug, Clone, entity_derive::Entity)]
struct MockOrder {
    id: u64,
    // #[created(skip)]
    // #[replay(skip)]
    symbol: Symbol,
    // #[created(skip)]
    // #[replay(skip)]
    price: Price,
    // #[created(skip)]
    // #[replay(skip)]
    quantity: Quantity,
    // #[created(skip)]
    // #[replay(skip)]
    filled_quantity: Quantity,
    // #[created(skip)]
    // #[replay(skip)]
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

    fn symbol(&self) -> Symbol {
        self.symbol
    }
}

#[test]
fn test_btreemap_basic() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加买单
    let buy_order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };
    assert!(lob.add_order(buy_order).is_ok());

    // 添加卖单
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
fn test_btreemap_ordered_matching() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加多个卖单（无序添加）
    let prices = vec![50020.0, 50000.0, 50030.0, 50010.0, 50040.0];
    for (i, &price) in prices.iter().enumerate() {
        let order = MockOrder {
            id: i as u64,
            symbol,
            price: Price::from_f64(price),
            quantity: Quantity::from_f64(1.0),
            side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 匹配买单：应该按价格从低到高匹配（BTreeMap 自动排序）
    let matched = lob.match_orders(
        Side::Buy,
        Price::from_f64(50025.0),
        Quantity::from_f64(2.5),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    assert_eq!(orders.len(), 3);  // 匹配到 3 个订单

    // 验证匹配顺序：50000.0, 50010.0, 50020.0
    assert_eq!(orders[0].price(), Price::from_f64(50000.0));
    assert_eq!(orders[1].price(), Price::from_f64(50010.0));
    assert_eq!(orders[2].price(), Price::from_f64(50020.0));
}

#[test]
fn test_btreemap_market_depth() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加买单
    for i in 0..5 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(50000.0 - (i as f64 * 10.0)),
            quantity: Quantity::from_f64(1.0 + i as f64),
            side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 添加卖单
    for i in 0..5 {
        let order = MockOrder {
            id: 100 + i,
            symbol,
            price: Price::from_f64(50100.0 + (i as f64 * 10.0)),
            quantity: Quantity::from_f64(2.0 + i as f64),
            side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 获取市场深度（前3档）
    let (bids, asks) = lob.market_depth(3);

    assert_eq!(bids.len(), 3);
    assert_eq!(asks.len(), 3);

    // 买盘应该从高到低排列
    assert_eq!(bids[0].0, Price::from_f64(50000.0));
    assert_eq!(bids[1].0, Price::from_f64(49990.0));
    assert_eq!(bids[2].0, Price::from_f64(49980.0));

    // 卖盘应该从低到高排列
    assert_eq!(asks[0].0, Price::from_f64(50100.0));
    assert_eq!(asks[1].0, Price::from_f64(50110.0));
    assert_eq!(asks[2].0, Price::from_f64(50120.0));

    // 验证数量
    assert_eq!(bids[0].1, Quantity::from_f64(1.0));
    assert_eq!(asks[0].1, Quantity::from_f64(2.0));
}

#[test]
fn test_btreemap_shib_low_price() {
    let symbol = Symbol::new("SHIBUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.00000001));

    // 添加低价币订单
    let order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(0.00001234),
        quantity: Quantity::from_f64(1000000.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };

    assert!(lob.add_order(order).is_ok());
    assert!(lob.find_order(1).is_some());
}

#[test]
fn test_btreemap_remove_order() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

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
fn test_btreemap_last_price() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

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
fn test_btreemap_range_query() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加大量卖单（模拟真实场景）
    for i in 0..100 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(50000.0 + (i as f64)),
            quantity: Quantity::from_f64(1.0),
            side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 匹配订单：BTreeMap 的 range 方法高效查找区间
    let matched = lob.match_orders(
        Side::Buy,
        Price::from_f64(50010.0),
        Quantity::from_f64(5.5),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    assert_eq!(orders.len(), 6);  // 匹配到 6 个订单

    // 验证是按价格升序匹配
    for i in 0..6 {
        assert_eq!(orders[i].price(), Price::from_f64(50000.0 + i as f64));
    }
}

#[test]
fn test_btreemap_multiple_orders_same_price() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 在同一价格添加多个订单（测试时间优先）
    for i in 0..5 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(50000.0),  // 相同价格
            quantity: Quantity::from_f64(1.0),
            side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 匹配订单：同一价格应该按时间优先（FIFO）
    let matched = lob.match_orders(
        Side::Buy,
        Price::from_f64(50000.0),
        Quantity::from_f64(2.5),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    assert_eq!(orders.len(), 3);

    // 验证 FIFO 顺序
    assert_eq!(orders[0].order_id(), 0);
    assert_eq!(orders[1].order_id(), 1);
    assert_eq!(orders[2].order_id(), 2);
}

#[test]
fn test_btreemap_sell_side_matching() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLobBTreeMap<MockOrder> = LocalLobBTreeMap::new_with_tick(symbol, Price::from_f64(0.01));

    // 添加多个买单
    let prices = vec![49990.0, 50000.0, 49980.0, 49970.0];
    for (i, &price) in prices.iter().enumerate() {
        let order = MockOrder {
            id: i as u64,
            symbol,
            price: Price::from_f64(price),
            quantity: Quantity::from_f64(1.0),
            side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 卖单匹配：应该从最高买价开始匹配
    // 卖价 49975.0 可以匹配价格 >= 49975.0 的所有买单
    let matched = lob.match_orders(
        Side::Sell,
        Price::from_f64(49975.0),  // 调整卖价以匹配 3 个订单
        Quantity::from_f64(2.5),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    assert_eq!(orders.len(), 3);

    // 验证匹配顺序：从高到低
    assert_eq!(orders[0].price(), Price::from_f64(50000.0));
    assert_eq!(orders[1].price(), Price::from_f64(49990.0));
    assert_eq!(orders[2].price(), Price::from_f64(49980.0));
}
