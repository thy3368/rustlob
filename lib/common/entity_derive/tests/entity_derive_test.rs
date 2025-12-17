//! Entity derive 宏测试

use diff::Entity;

/// 测试订单实体 - 使用 Entity derive 宏
#[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
    quantity: u64,
    status: String,
}

#[test]
fn test_entity_derive_basic() {
    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    // 测试 entity_id
    assert_eq!(order.entity_id(), 1);

    // 测试 entity_type
    assert_eq!(Order::entity_type(), "Order");
}

#[test]
fn test_entity_derive_diff() {
    let old_order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    let new_order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 51000.0,
        quantity: 15,
        status: "confirmed".to_string(),
    };

    // 测试 diff
    let changes = old_order.diff(&new_order);
    assert_eq!(changes.len(), 3); // price, quantity, status

    // 验证变更内容
    let price_change = changes.iter().find(|c| c.field_name == "price").unwrap();
    assert!(price_change.old_value.contains("50000"));
    assert!(price_change.new_value.contains("51000"));
}

#[test]
fn test_entity_derive_no_changes() {
    let order1 = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    let order2 = order1.clone();

    // 测试无变更情况
    let changes = order1.diff(&order2);
    assert_eq!(changes.len(), 0);
}

#[test]
fn test_entity_derive_track_create() {
    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    // 测试追踪创建
    let entry = order.track_create().unwrap();
    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");
}

#[test]
fn test_entity_derive_track_delete() {
    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    // 测试追踪删除
    let entry = order.track_delete().unwrap();
    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");
}

#[test]
fn test_entity_derive_track_update() {
    let mut order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    // 测试追踪更新
    let entry = order
        .track_update(|o| {
            o.price = 51000.0;
            o.status = "confirmed".to_string();
        })
        .unwrap();

    // 验证实体已更新
    assert_eq!(order.price, 51000.0);
    assert_eq!(order.status, "confirmed");

    // 验证变更日志
    assert_eq!(entry.entity_id, "1");
}

#[test]
fn test_entity_derive_replay() {
    use diff::{ChangeLogEntry, ChangeType, FieldChange};

    let mut order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    // 创建变更日志
    let entry = ChangeLogEntry::new(
        "1",
        "Order",
        ChangeType::Updated {
            changed_fields: vec![
                FieldChange::new("price", "50000.0", "51000.0"),
                FieldChange::new("quantity", "10", "15"),
                FieldChange::new("status", "\"pending\"", "\"confirmed\""),
            ],
        },
        1000,
        1,
    );

    // 测试 replay
    order.replay(&entry).unwrap();

    // 验证字段已更新
    assert_eq!(order.price, 51000.0);
    assert_eq!(order.quantity, 15);
    assert_eq!(order.status, "confirmed");
}

/// 测试自定义ID字段
#[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
#[entity(id = "order_id", type_name = "CustomOrder")]
struct CustomOrder {
    order_id: String,
    symbol: String,
    price: f64,
}

#[test]
fn test_entity_derive_custom_id() {
    let order = CustomOrder {
        order_id: "ABC123".to_string(),
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
    };

    // 测试自定义ID字段
    assert_eq!(order.entity_id(), "ABC123");

    // 测试自定义类型名称
    assert_eq!(CustomOrder::entity_type(), "CustomOrder");
}

/// 测试字段跳过
#[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
struct OrderWithCache {
    id: u64,
    symbol: String,
    price: f64,
    #[diff(skip)]
    cache: String,
}

#[test]
fn test_entity_derive_skip_fields() {
    let old_order = OrderWithCache {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        cache: "old_cache".to_string(),
    };

    let new_order = OrderWithCache {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        cache: "new_cache".to_string(), // cache 字段应该被跳过
    };

    // diff 应该跳过 cache 字段
    let changes = old_order.diff(&new_order);
    assert_eq!(changes.len(), 0);
}
