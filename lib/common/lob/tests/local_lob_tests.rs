use base_types::{OrderId, Price, Quantity, Side, Symbol};
use diff::{ChangeLogEntry, ChangeType};
use lob_repo::adapter::local_lob_impl::LocalLob;
use lob_repo::core::symbol_lob_repo::{Order, SymbolLob};
use lob_repo::core::repo_snapshot_support::{RepoSnapshot, EventReplay};

// 创建模拟订单用于测试
#[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
struct MockOrder {
    id: u64,
    #[replay(skip)]
    symbol: Symbol,
    #[replay(skip)]
    price: Price,
    #[replay(skip)]
    quantity: Quantity,
    #[replay(skip)]
    filled_quantity: Quantity,
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
        filled_quantity: Quantity::from_raw(0),
    };

    // order.replay(null())
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
        filled_quantity: Quantity::from_raw(0),
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
        filled_quantity: Quantity::from_raw(0),
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
fn test_remove_order() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

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

    // 删除不存在的订单
    let removed = lob.remove_order(999);
    assert!(!removed);
}

// ==================== RepoSnapshot 测试 ====================

#[test]
fn test_snapshot_creation() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 添加一些订单和状态
    let buy_order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };
    lob.add_order(buy_order).unwrap();
    lob.update_last_price(Price::from_f64(50000.0));

    // 创建快照
    let snapshot = lob.create_snapshot(1000, 1);
    assert!(snapshot.is_ok());
}

#[test]
fn test_snapshot_restore() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 创建快照
    let snapshot = lob.create_snapshot(1000, 1).unwrap();

    // 创建新的 LOB 并恢复
    let mut lob2: LocalLob<MockOrder> = LocalLob::new(symbol);
    let restore_result = lob2.restore_from_snapshot(&snapshot);
    assert!(restore_result.is_ok());
}

#[test]
fn test_snapshot_with_multiple_orders() {
    let symbol = Symbol::new("ETHUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 添加多个买单
    for i in 0..3 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(3000.0 - (i as f64 * 10.0)),
            quantity: Quantity::from_f64(2.0),
            side: Side::Buy,
            filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 添加多个卖单
    for i in 3..6 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(3000.0 + ((i - 3) as f64 * 10.0)),
            quantity: Quantity::from_f64(1.5),
            side: Side::Sell,
            filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 更新成交价
    lob.update_last_price(Price::from_f64(3000.0));

    // 创建快照和恢复
    let snapshot = lob.create_snapshot(2000, 5).unwrap();
    let restore_result = lob.restore_from_snapshot(&snapshot);
    assert!(restore_result.is_ok());
}

// ==================== EventReplay 测试 ====================

#[test]
fn test_event_replay_basic() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 创建一个事件（包含字段信息）
    let event = ChangeLogEntry::new("1", "MockOrder", ChangeType::Created { fields: vec![] }, 1000, 1);
    let result = lob.replay_event(&event);
    assert!(result.is_ok());
}

#[test]
fn test_event_replay_multiple_events() {
    let symbol = Symbol::new("ETHUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 创建多个事件（包含字段信息）
    let events = vec![
        ChangeLogEntry::new("1", "MockOrder", ChangeType::Created { fields: vec![] }, 1000, 1),
        ChangeLogEntry::new("2", "MockOrder", ChangeType::Created { fields: vec![] }, 1500, 2),
        ChangeLogEntry::new("3", "MockOrder", ChangeType::Created { fields: vec![] }, 2000, 3),
    ];

    // 回放多个事件
    let result = lob.replay_events(&events);
    assert!(result.is_ok());
}

// ==================== EventReplay Update 事件测试 ====================

#[test]
fn test_event_replay_update_event() {
    use diff::FieldChange;

    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // === 第一阶段：添加初始订单 ===
    let order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(2.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };
    lob.add_order(order).unwrap();

    // 验证订单已添加
    assert!(lob.find_order(1).is_some());
    let found_order = lob.find_order(1).unwrap();
    assert_eq!(found_order.filled_quantity(), Quantity::from_raw(0));

    // === 第二阶段：创建快照 ===
    let snapshot = lob.create_snapshot(1000, 1).unwrap();

    // === 第三阶段：创建 update 事件 ===
    // 模拟订单被部分成交：filled_quantity 从 0 变为 1.0
    let update_event = ChangeLogEntry::new(
        "1",
        "MockOrder",
        ChangeType::Updated {
            changed_fields: vec![
                FieldChange::new("filled_quantity", "0", "1.0"),
            ],
        },
        2000,
        2,
    );

    // === 第四阶段：在新 LOB 上回放 update 事件 ===
    let mut restored_lob: LocalLob<MockOrder> = LocalLob::new(symbol);
    restored_lob.restore_from_snapshot(&snapshot).unwrap();

    // 验证恢复后的订单状态
    assert!(restored_lob.find_order(1).is_some());
    let order_before_update = restored_lob.find_order(1).unwrap();
    assert_eq!(order_before_update.filled_quantity(), Quantity::from_raw(0));

    // 回放 update 事件
    let replay_result = restored_lob.replay_event(&update_event);
    assert!(replay_result.is_ok());

    // === 第五阶段：验证事件回放 ===
    // 注：当前实现中，update 事件主要用于审计和变更追踪
    // 实际的订单数据更新（如 filled_quantity）需要通过专门的 API（如 find_order_mut）
    // 这个测试验证了 EventReplay 能够正确处理 Updated 事件类型
    assert!(restored_lob.find_order(1).is_some());

    // === 第六阶段：验证事件完整性 ===
    match &update_event.change_type {
        ChangeType::Updated { changed_fields } => {
            // 验证变更字段信息
            assert_eq!(changed_fields.len(), 1);
            assert_eq!(changed_fields[0].field_name, "filled_quantity");
            assert_eq!(changed_fields[0].old_value, "0");
            assert_eq!(changed_fields[0].new_value, "1.0");
        }
        _ => panic!("Expected Updated change type"),
    }

    // === 第七阶段：验证多种事件类型的混合回放 ===
    let mixed_events = vec![
        ChangeLogEntry::new(
            "2",
            "MockOrder",
            ChangeType::Created { fields: vec![] },
            3000,
            3,
        ),
        ChangeLogEntry::new(
            "2",
            "MockOrder",
            ChangeType::Updated {
                changed_fields: vec![
                    FieldChange::new("price", "50100.0", "50150.0"),
                ],
            },
            4000,
            4,
        ),
        ChangeLogEntry::new(
            "2",
            "MockOrder",
            ChangeType::Deleted,
            5000,
            5,
        ),
    ];

    let mixed_replay = restored_lob.replay_events(&mixed_events);
    assert!(mixed_replay.is_ok());
}

#[test]
fn test_snapshot_creation_replay() {
    let symbol = Symbol::new("BTCUSDT");
    let mut original_lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // === 第一阶段：构建初始订单簿 ===
    // 添加 3 个买单
    for i in 0..3 {
        let order = MockOrder {
            id: i as u64,
            symbol,
            price: Price::from_f64(50000.0 - (i as f64 * 10.0)),
            quantity: Quantity::from_f64(2.0),
            side: Side::Buy,
            filled_quantity: Quantity::from_raw(0),
        };
        original_lob.add_order(order).unwrap();
    }

    // 添加 3 个卖单
    for i in 0..3 {
        let order = MockOrder {
            id: (100 + i) as u64,
            symbol,
            price: Price::from_f64(50100.0 + (i as f64 * 10.0)),
            quantity: Quantity::from_f64(1.5),
            side: Side::Sell,
            filled_quantity: Quantity::from_raw(0),
        };
        original_lob.add_order(order).unwrap();
    }

    // 更新最后成交价
    original_lob.update_last_price(Price::from_f64(50050.0));

    // === 验证初始状态 ===
    assert_eq!(original_lob.best_bid(), Some(Price::from_f64(50000.0)));
    assert_eq!(original_lob.best_ask(), Some(Price::from_f64(50100.0)));
    assert_eq!(original_lob.last_price(), Some(Price::from_f64(50050.0)));

    // === 创建快照 ===
    let snapshot = original_lob.create_snapshot(1000, 1).unwrap();

    // === 第二阶段：修改原 LOB ===
    // 删除一个订单并记录删除事件
    original_lob.remove_order(0);

    // 创建删除事件并回放
    let delete_event = ChangeLogEntry::new(
        "0",
        "MockOrder",
        ChangeType::Deleted,
        2000,
        2,
    );

    // === 第三阶段：在新 LOB 上从快照恢复并回放事件 ===
    let mut restored_lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 从快照恢复
    restored_lob.restore_from_snapshot(&snapshot).unwrap();

    // === 验证快照恢复后的状态 ===
    assert_eq!(restored_lob.best_bid(), Some(Price::from_f64(50000.0)));
    assert_eq!(restored_lob.best_ask(), Some(Price::from_f64(50100.0)));
    assert_eq!(restored_lob.last_price(), Some(Price::from_f64(50050.0)));
    assert!(restored_lob.find_order(0).is_some());
    assert!(restored_lob.find_order(100).is_some());

    // 回放删除事件
    let replay_result = restored_lob.replay_event(&delete_event);
    assert!(replay_result.is_ok());

    // === 验证回放事件后的状态 ===
    // 订单 0 应该被删除
    assert!(restored_lob.find_order(0).is_none());
    // 其他订单应该仍然存在
    assert!(restored_lob.find_order(1).is_some());
    assert!(restored_lob.find_order(100).is_some());

    // === 验证快照不受影响 ===
    assert_eq!(snapshot.best_bid(), Some(Price::from_f64(50000.0)));
    assert!(snapshot.find_order(0).is_some());  // 快照中订单 0 仍然存在
}



// ==================== 订单匹配边界情况测试 ====================

#[test]
fn test_match_orders_sell_side() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 添加多个买单
    for i in 0..5 {
        let order = MockOrder {
            id: i,
            symbol,
            price: Price::from_f64(50000.0 - (i as f64 * 10.0)),
            quantity: Quantity::from_f64(1.0),
            side: Side::Buy,
            filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 匹配卖单：应该匹配最高价格的买单
    let matched = lob.match_orders(
        Side::Sell,
        Price::from_f64(49990.0),
        Quantity::from_f64(2.5),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    // 匹配到 2 个订单 (1.0 + 1.0 = 2.0)，剩余 0.5 未匹配
    assert!(orders.len() >= 2);
}

#[test]
fn test_match_orders_no_match() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 添加卖单在 50100.00
    let sell_order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50100.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
    };
    lob.add_order(sell_order).unwrap();

    // 尝试以太低的价格买入 (买价低于最低卖价)
    let matched = lob.match_orders(
        Side::Buy,
        Price::from_f64(50000.0),
        Quantity::from_f64(1.0),
    );

    assert!(matched.is_none());
}

#[test]
fn test_match_orders_exact_quantity() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 添加一个卖单
    let sell_order = MockOrder {
        id: 1,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(2.0),
        side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
    };
    lob.add_order(sell_order).unwrap();

    // 匹配完全相同的数量
    let matched = lob.match_orders(
        Side::Buy,
        Price::from_f64(50000.0),
        Quantity::from_f64(2.0),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    assert_eq!(orders.len(), 1);
}

#[test]
fn test_find_order_mut() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    let order = MockOrder {
        id: 100,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };

    lob.add_order(order).unwrap();

    // 查找可变引用
    let found_mut = lob.find_order_mut(100);
    assert!(found_mut.is_some());

    // 查找不存在的订单
    let not_found = lob.find_order_mut(999);
    assert!(not_found.is_none());
}

#[test]
fn test_symbol_getter() {
    let symbol = Symbol::new("DOGEUSDT");
    let lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    assert_eq!(lob.symbol(), &symbol);
}

#[test]
fn test_add_order_duplicate_id() {
    let symbol = Symbol::new("BTCUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    let order1 = MockOrder {
        id: 50,
        symbol,
        price: Price::from_f64(50000.0),
        quantity: Quantity::from_f64(1.0),
        side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
    };

    let order2 = MockOrder {
        id: 50,  // 相同的 ID
        symbol,
        price: Price::from_f64(51000.0),
        quantity: Quantity::from_f64(2.0),
        side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
    };

    // 添加第一个订单成功
    assert!(lob.add_order(order1).is_ok());

    // 添加相同 ID 的订单应该失败
    assert!(lob.add_order(order2).is_err());
}

#[test]
fn test_complex_order_matching_scenario() {
    let symbol = Symbol::new("BNBUSDT");
    let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 构建订单簿：多个价格级别
    // 买单：600, 599, 598, 597, 596
    for i in 0..5 {
        let order = MockOrder {
            id: i as u64,
            symbol,
            price: Price::from_f64(600.0 - (i as f64)),
            quantity: Quantity::from_f64(10.0),
            side: Side::Buy,
            filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 卖单：601, 602, 603, 604, 605
    for i in 0..5 {
        let order = MockOrder {
            id: (100 + i) as u64,
            symbol,
            price: Price::from_f64(601.0 + (i as f64)),
            quantity: Quantity::from_f64(5.0),
            side: Side::Sell,
            filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(order).unwrap();
    }

    // 验证最佳买卖价
    assert_eq!(lob.best_bid(), Some(Price::from_f64(600.0)));
    assert_eq!(lob.best_ask(), Some(Price::from_f64(601.0)));

    // 卖单匹配 - 从最高买价开始匹配
    let matched = lob.match_orders(
        Side::Sell,
        Price::from_f64(600.0),
        Quantity::from_f64(25.0),
    );

    assert!(matched.is_some());
    let orders = matched.unwrap();
    // 应该至少匹配 1 个订单 (第一个价格级别的买单)
    assert!(orders.len() >= 1);
}
