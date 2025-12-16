//! 集成测试：验证 Diff 和 Replayable derive 宏功能

use diff::{
    diff::diff_types::{Diff, Replayable},
    track_create, track_delete, ChangeLogEntry, ChangeType, Entity, FieldChange
};

/// 测试订单实体 - 使用 derive 宏
#[derive(Debug, Clone, PartialEq, diff::Diff, diff::Replayable)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
    quantity: u64,
    status: String
}

impl Entity for Order {
    type Id = u64;

    fn entity_id(&self) -> Self::Id { self.id }

    fn entity_type() -> &'static str { "Order" }

    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(format!("{}:{}:{}:{}:{}", self.id, self.symbol, self.price, self.quantity, self.status).into_bytes())
    }

    fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let s = String::from_utf8(data.to_vec()).map_err(|e| e.to_string())?;
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 5 {
            return Err("Invalid format".to_string());
        }
        Ok(Self {
            id: parts[0].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            symbol: parts[1].to_string(),
            price: parts[2].parse().map_err(|e: std::num::ParseFloatError| e.to_string())?,
            quantity: parts[3].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            status: parts[4].to_string()
        })
    }
}

#[test]
fn test_derive_diff() {
    let old_order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    let new_order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 51000.0,
        quantity: 15,
        status: "confirmed".to_string()
    };

    // 测试自动生成的 diff 方法
    let changes = old_order.diff(&new_order);

    assert_eq!(changes.len(), 3);

    // 验证 price 变更
    let price_change = changes.iter().find(|c| c.field_name == "price").unwrap();
    assert_eq!(price_change.old_value, "50000");
    assert_eq!(price_change.new_value, "51000");

    // 验证 quantity 变更
    let qty_change = changes.iter().find(|c| c.field_name == "quantity").unwrap();
    assert_eq!(qty_change.old_value, "10");
    assert_eq!(qty_change.new_value, "15");

    // 验证 status 变更
    let status_change = changes.iter().find(|c| c.field_name == "status").unwrap();
    assert_eq!(status_change.old_value, "pending");
    assert_eq!(status_change.new_value, "confirmed");
}

#[test]
fn test_derive_replayable() {
    let mut order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    // 创建变更日志条目
    let entry = ChangeLogEntry::new(
        "1",
        "Order",
        ChangeType::Updated {
            changed_fields: vec![
                FieldChange::new("price", "50000", "51000"),
                FieldChange::new("quantity", "10", "15"),
                FieldChange::new("status", "pending", "confirmed"),
            ]
        },
        1000,
        1
    );

    // 测试自动生成的 replay 方法
    order.replay(&entry).unwrap();

    // 验证字段已更新
    assert_eq!(order.price, 51000.0);
    assert_eq!(order.quantity, 15);
    assert_eq!(order.status, "confirmed");
}


#[test]
fn test_no_changes() {
    let old_order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    let new_order = old_order.clone();

    // 测试无变更情况
    let changes = old_order.diff(&new_order);
    assert_eq!(changes.len(), 0);
}

#[test]
fn test_replay_entity_mismatch() {
    let mut order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    // 创建一个不匹配的变更日志（不同的实体ID）
    let entry = ChangeLogEntry::new(
        "999",
        "Order",
        ChangeType::Updated {
            changed_fields: vec![FieldChange::new("price", "50000", "51000")]
        },
        1000,
        1
    );

    // 应该返回错误
    let result = order.replay(&entry);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Cannot replay: entity mismatch");
}

#[test]
fn test_track_create() {
    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    // 追踪创建操作
    let entry = track_create(&order).unwrap();

    // 验证变更日志
    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");
    assert!(matches!(entry.change_type, ChangeType::Created));
}

#[test]
fn test_track_delete() {
    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    // 追踪删除操作
    let entry = track_delete(&order).unwrap();

    // 验证变更日志
    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");
    assert!(matches!(entry.change_type, ChangeType::Deleted));
}


// ============================================================================
// 统一 API 测试
// ============================================================================

#[test]
fn test_unified_track_create() {
    use diff::Operation;

    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    // 使用统一接口追踪创建
    let entry = diff::track(&order, Operation::Create).unwrap();

    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");
    assert!(matches!(entry.change_type, ChangeType::Created));
}

#[test]
fn test_unified_track_delete() {
    use diff::Operation;

    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    // 使用统一接口追踪删除
    let entry = diff::track(&order, Operation::Delete).unwrap();

    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");
    assert!(matches!(entry.change_type, ChangeType::Deleted));
}

#[test]
fn test_unified_track_update() {
    let mut order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    // 使用统一接口追踪更新
    let entry = diff::track_update(&mut order, |o| {
        o.price = 51000.0;
        o.status = "confirmed".to_string();
    })
    .unwrap();

    // 验证实体已更新
    assert_eq!(order.price, 51000.0);
    assert_eq!(order.status, "confirmed");

    // 验证变更日志
    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");

    if let ChangeType::Updated {
        changed_fields
    } = &entry.change_type
    {
        assert_eq!(changed_fields.len(), 2);

        let price_change = changed_fields.iter().find(|c| c.field_name == "price").unwrap();
        assert_eq!(price_change.old_value, "50000");
        assert_eq!(price_change.new_value, "51000");

        let status_change = changed_fields.iter().find(|c| c.field_name == "status").unwrap();
        assert_eq!(status_change.old_value, "pending");
        assert_eq!(status_change.new_value, "confirmed");
    } else {
        panic!("Expected Updated change type");
    }
}

#[test]
fn test_unified_api_full_lifecycle() {
    use diff::Operation;

    // 1. 创建 - 使用统一 API
    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string()
    };

    let create_entry = diff::track(&order, Operation::Create).unwrap();
    assert!(matches!(create_entry.change_type, ChangeType::Created));

    // 2. 更新 - 使用统一 API
    let mut order = order;
    let update_entry = diff::track_update(&mut order, |o| {
        o.price = 51000.0;
        o.quantity = 15;
    })
    .unwrap();

    assert_eq!(order.price, 51000.0);
    assert_eq!(order.quantity, 15);

    if let ChangeType::Updated {
        changed_fields
    } = &update_entry.change_type
    {
        assert_eq!(changed_fields.len(), 2);
    } else {
        panic!("Expected Updated change type");
    }

    // 3. 删除 - 使用统一 API
    let delete_entry = diff::track(&order, Operation::Delete).unwrap();
    assert!(matches!(delete_entry.change_type, ChangeType::Deleted));
}
