//! Entity derive 宏测试

use diff::Entity;
use diff::{extract_fields_from_created_event, reconstruct_from_created, ChangeLogEntry, ChangeType, FieldChange, EntityError};

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

// ==================== Created 事件重构测试 ====================

#[test]
fn test_create_event_from_track() {
    use diff::ChangeType;

    let order = Order {
        id: 1,
        symbol: "BTCUSDT".to_string(),
        price: 50000.0,
        quantity: 10,
        status: "pending".to_string(),
    };

    // 测试从 track_create 生成的 Created 事件
    let entry = order.track_create().unwrap();

    // 验证事件格式
    assert_eq!(entry.entity_id, "1");
    assert_eq!(entry.entity_type, "Order");

    // 验证是 Created 类型
    match &entry.change_type {
        ChangeType::Created { fields } => {
            // Created 事件目前包含空字段列表（实际字段通过其他方式传递）
            assert_eq!(fields.len(), 0);
        }
        _ => panic!("Expected Created change type"),
    }
}

#[test]
fn test_reconstruct_from_created_event_with_fields() {
    use diff::{ChangeLogEntry, ChangeType, FieldChange};

    // === 场景：从 Created 事件重构实体 ===
    // Created 事件包含所有初始字段值
    let created_event = ChangeLogEntry::new(
        "2",
        "Order",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "2"),
                FieldChange::new("symbol", "", "\"ETHUSDT\""),
                FieldChange::new("price", "", "3000.0"),
                FieldChange::new("quantity", "", "100"),
                FieldChange::new("status", "", "\"new\""),
            ],
        },
        2000,
        1,
    );

    // === 从 Created 事件提取字段并重构实体 ===
    // 这演示了模式：ChangeLogEntry (with Created) → extract fields → new instance
    if let ChangeType::Created { fields } = &created_event.change_type {
        let mut order_data: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        // 提取字段值
        for field in fields {
            order_data.insert(field.field_name.to_string(), field.new_value.clone());
        }

        // 从提取的字段重构实体
        let reconstructed_order = Order {
            id: order_data
                .get("id")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0),
            symbol: order_data
                .get("symbol")
                .map(|v| {
                    // String 字段去掉引号
                    if v.starts_with('\"') && v.ends_with('\"') {
                        v[1..v.len() - 1].to_string()
                    } else {
                        v.clone()
                    }
                })
                .unwrap_or_default(),
            price: order_data
                .get("price")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0),
            quantity: order_data
                .get("quantity")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0),
            status: order_data
                .get("status")
                .map(|v| {
                    // String 字段去掉引号
                    if v.starts_with('\"') && v.ends_with('\"') {
                        v[1..v.len() - 1].to_string()
                    } else {
                        v.clone()
                    }
                })
                .unwrap_or_default(),
        };

        // 验证重构的实体
        assert_eq!(reconstructed_order.id, 2);
        assert_eq!(reconstructed_order.symbol, "ETHUSDT");
        assert_eq!(reconstructed_order.price, 3000.0);
        assert_eq!(reconstructed_order.quantity, 100);
        assert_eq!(reconstructed_order.status, "new");
    } else {
        panic!("Expected Created change type");
    }
}

#[test]
fn test_event_replay_sequence_created_to_updated_to_deleted() {
    use diff::{ChangeLogEntry, ChangeType, FieldChange};

    // === 演示完整的事件序列：Created → Updated → Deleted ===
    // 第一步：创建新实体（从 Created 事件的字段值）
    let mut order = Order {
        id: 3,
        symbol: "ADAUSDT".to_string(),
        price: 0.5,
        quantity: 1000,
        status: "pending".to_string(),
    };

    // 验证初始状态
    assert_eq!(order.id, 3);
    assert_eq!(order.status, "pending");

    // 第二步：更新事件（修改实体）
    let updated_event = ChangeLogEntry::new(
        "3",
        "Order",
        ChangeType::Updated {
            changed_fields: vec![
                FieldChange::new("price", "0.5", "0.6"),
                FieldChange::new("status", "\"pending\"", "\"confirmed\""),
            ],
        },
        2000,
        2,
    );

    // 应用更新事件
    order.replay(&updated_event).unwrap();
    assert_eq!(order.price, 0.6);
    assert_eq!(order.status, "confirmed");

    // 第三步：删除事件
    let deleted_event = ChangeLogEntry::new(
        "3",
        "Order",
        ChangeType::Deleted,
        3000,
        3,
    );

    // 删除事件不能在已删除的实体上回放
    let delete_result = order.replay(&deleted_event);
    assert!(delete_result.is_err()); // CannotReplayOnDeleted

    // 但删除事件本身是有效的，表示实体已被删除
    match &deleted_event.change_type {
        ChangeType::Deleted => {
            // 这是有效的删除标记
            assert!(true);
        }
        _ => panic!("Expected Deleted change type"),
    }
}

#[test]
fn test_created_event_field_values_format() {
    use diff::{ChangeLogEntry, ChangeType, FieldChange};

    // === 验证 Created 事件中字段值的格式 ===
    // Created 事件中每个 FieldChange 应该有：
    // - old_value: 空（因为是新创建）
    // - new_value: 初始值

    let created_event = ChangeLogEntry::new(
        "4",
        "Order",
        ChangeType::Created {
            fields: vec![
                // 数值类型：直接序列化
                FieldChange::new("id", "", "4"),
                FieldChange::new("quantity", "", "500"),
                FieldChange::new("price", "", "100.5"),
                // 字符串类型：Debug 格式（带引号）
                FieldChange::new("symbol", "", "\"BNBUSDT\""),
                FieldChange::new("status", "", "\"active\""),
            ],
        },
        4000,
        4,
    );

    if let ChangeType::Created { fields } = &created_event.change_type {
        // 验证字段格式
        for field in fields {
            // Created 事件中，old_value 总是空字符串
            assert_eq!(field.old_value, "");

            // new_value 包含初始值
            assert!(!field.new_value.is_empty());

            // 字符串字段应该有引号
            if field.field_name == "symbol" || field.field_name == "status" {
                assert!(field.new_value.starts_with('\"'));
                assert!(field.new_value.ends_with('\"'));
            }

            // 数值字段不应该有引号
            if field.field_name == "id" || field.field_name == "quantity"
                || field.field_name == "price"
            {
                assert!(!field.new_value.starts_with('\"'));
            }
        }
    } else {
        panic!("Expected Created change type");
    }
}

// ==================== 使用 Created 事件重构 API 的测试 ====================

#[test]
fn test_reconstruct_order_from_created_event() {
    // === 演示完整的重构流程：Created Event + Type → Order Instance ===

    // 第一步：创建包含字段信息的 Created 事件
    let created_event = ChangeLogEntry::new(
        "100",
        "Order",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "100"),
                FieldChange::new("symbol", "", "\"ETHUSDT\""),
                FieldChange::new("price", "", "3000.5"),
                FieldChange::new("quantity", "", "50"),
                FieldChange::new("status", "", "\"pending\""),
            ],
        },
        5000,
        100,
    );

    // 第二步：从事件中提取字段映射表
    let fields = extract_fields_from_created_event(&created_event).unwrap();

    // 验证字段提取
    assert_eq!(fields.get("id").map(|s| s.as_str()), Some("100"));
    assert_eq!(
        fields.get("symbol").map(|s| s.as_str()),
        Some("\"ETHUSDT\"")
    );
    assert_eq!(
        fields.get("price").map(|s| s.as_str()),
        Some("3000.5")
    );
    assert_eq!(
        fields.get("quantity").map(|s| s.as_str()),
        Some("50")
    );
    assert_eq!(fields.get("status").map(|s| s.as_str()), Some("\"pending\""));

    // 第三步：使用闭包从字段映射重构 Order 实例
    let reconstructed = reconstruct_from_created::<Order, _>(&created_event, |fields| {
        let id = fields
            .get("id")
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or(EntityError::Custom("missing id".to_string()))?;

        let symbol = fields
            .get("symbol")
            .map(|v| {
                if v.starts_with('\"') && v.ends_with('\"') {
                    v[1..v.len() - 1].to_string()
                } else {
                    v.clone()
                }
            })
            .ok_or(EntityError::Custom("missing symbol".to_string()))?;

        let price = fields
            .get("price")
            .and_then(|v| v.parse::<f64>().ok())
            .ok_or(EntityError::Custom("missing price".to_string()))?;

        let quantity = fields
            .get("quantity")
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or(EntityError::Custom("missing quantity".to_string()))?;

        let status = fields
            .get("status")
            .map(|v| {
                if v.starts_with('\"') && v.ends_with('\"') {
                    v[1..v.len() - 1].to_string()
                } else {
                    v.clone()
                }
            })
            .ok_or(EntityError::Custom("missing status".to_string()))?;

        Ok(Order {
            id,
            symbol,
            price,
            quantity,
            status,
        })
    })
    .unwrap();

    // 第四步：验证重构的实体
    assert_eq!(reconstructed.id, 100);
    assert_eq!(reconstructed.symbol, "ETHUSDT");
    assert_eq!(reconstructed.price, 3000.5);
    assert_eq!(reconstructed.quantity, 50);
    assert_eq!(reconstructed.status, "pending");
}

#[test]
fn test_created_event_input_output_pattern() {
    // === 演示核心模式：Input (Created Event + Type) → Output (Instance) ===

    // Input: ChangeLogEntry with Created type
    let input_event = ChangeLogEntry::new(
        "200",
        "Order",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "200"),
                FieldChange::new("symbol", "", "\"BTCUSDT\""),
                FieldChange::new("price", "", "60000.75"),
                FieldChange::new("quantity", "", "0.5"),
                FieldChange::new("status", "", "\"active\""),
            ],
        },
        6000,
        200,
    );

    // Process: Type information + Constructor logic
    let output_order = reconstruct_from_created::<Order, _>(&input_event, |fields| {
        // 自定义构造逻辑
        Ok(Order {
            id: fields
                .get("id")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            symbol: fields
                .get("symbol")
                .map(|v| v.trim_matches('\"').to_string())
                .unwrap_or_default(),
            price: fields
                .get("price")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.0),
            quantity: fields
                .get("quantity")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            status: fields
                .get("status")
                .map(|v| v.trim_matches('\"').to_string())
                .unwrap_or_default(),
        })
    })
    .unwrap();

    // Output: New instance of Order type
    assert_eq!(output_order.id, 200);
    assert_eq!(output_order.symbol, "BTCUSDT");
    assert_eq!(output_order.price, 60000.75);
    assert_eq!(output_order.quantity, 0);  // quantity 是 u64，0.5 被截断为 0
    assert_eq!(output_order.status, "active");
}

#[test]
fn test_extract_fields_from_various_types() {
    // 测试从 Created 事件提取各种类型的字段

    let event = ChangeLogEntry::new(
        "5",
        "Order",
        ChangeType::Created {
            fields: vec![
                // 整数
                FieldChange::new("id", "", "999"),
                FieldChange::new("quantity", "", "1000"),
                // 浮点数
                FieldChange::new("price", "", "99.99"),
                // 字符串（带引号）
                FieldChange::new("symbol", "", "\"XYZABC\""),
                FieldChange::new("status", "", "\"completed\""),
            ],
        },
        7000,
        5,
    );

    let fields = extract_fields_from_created_event(&event).unwrap();

    // 验证各种类型的字段
    // 整数字段
    assert_eq!(fields.get("id").unwrap(), "999");
    assert_eq!(fields.get("quantity").unwrap(), "1000");

    // 浮点数字段
    assert_eq!(fields.get("price").unwrap(), "99.99");

    // 字符串字段（保留引号，需要手动处理）
    assert_eq!(fields.get("symbol").unwrap(), "\"XYZABC\"");
    assert_eq!(fields.get("status").unwrap(), "\"completed\"");
}

// ==================== FromCreatedEvent Derive 测试 ====================

#[test]
fn test_from_created_event_derived() {
    use diff::FromCreatedEvent;

    // 创建 Created 事件
    let created_event = ChangeLogEntry::new(
        "300",
        "Order",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "300"),
                FieldChange::new("symbol", "", "\"DOGEUSDT\""),
                FieldChange::new("price", "", "0.45"),
                FieldChange::new("quantity", "", "5000"),
                FieldChange::new("status", "", "\"filled\""),
            ],
        },
        8000,
        300,
    );

    // 使用自动生成的 FromCreatedEvent 实现
    let order = Order::from_created_event(&created_event).unwrap();

    // 验证重构的实体
    assert_eq!(order.id, 300);
    assert_eq!(order.symbol, "DOGEUSDT");
    assert_eq!(order.price, 0.45);
    assert_eq!(order.quantity, 5000);
    assert_eq!(order.status, "filled");
}

#[test]
fn test_from_created_event_field_map() {
    use diff::FromCreatedEvent;
    use std::collections::HashMap;

    // 直接使用 from_field_map 构造
    let mut fields = HashMap::new();
    fields.insert("id".to_string(), "400".to_string());
    fields.insert("symbol".to_string(), "\"SHIBAINU\"".to_string());
    fields.insert("price".to_string(), "0.000025".to_string());
    fields.insert("quantity".to_string(), "10000000".to_string());
    fields.insert("status".to_string(), "\"pending\"".to_string());

    let order = Order::from_field_map(&fields).unwrap();

    assert_eq!(order.id, 400);
    assert_eq!(order.symbol, "SHIBAINU");
    assert_eq!(order.price, 0.000025);
    assert_eq!(order.quantity, 10000000);
    assert_eq!(order.status, "pending");
}

#[test]
fn test_from_created_event_missing_field() {
    use diff::FromCreatedEvent;

    // 缺少必要字段的 Created 事件
    let created_event = ChangeLogEntry::new(
        "500",
        "Order",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "500"),
                // 缺少 symbol 字段
                FieldChange::new("price", "", "100.0"),
            ],
        },
        9000,
        500,
    );

    // 应该返回错误
    let result = Order::from_created_event(&created_event);
    assert!(result.is_err());

    if let Err(diff::EntityError::FieldParseError { field, .. }) = result {
        assert_eq!(field, "symbol");
    } else {
        panic!("Expected FieldParseError");
    }
}

#[test]
fn test_from_created_event_invalid_value() {
    use diff::FromCreatedEvent;

    // 包含无效字段值的 Created 事件
    let created_event = ChangeLogEntry::new(
        "600",
        "Order",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "not_a_number"),  // 无效的 u64
                FieldChange::new("symbol", "", "\"TEST\""),
                FieldChange::new("price", "", "100.0"),
                FieldChange::new("quantity", "", "10"),
                FieldChange::new("status", "", "\"active\""),
            ],
        },
        10000,
        600,
    );

    // 应该返回错误
    let result = Order::from_created_event(&created_event);
    assert!(result.is_err());
}

#[test]
fn test_from_created_event_with_skip_attribute() {
    use diff::FromCreatedEvent;

    // 测试带有 #[created(skip)] 属性的字段
    #[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
    struct OrderWithCache {
        id: u64,
        symbol: String,
        price: f64,
        #[created(skip)]
        cache: String,  // 这个字段会被跳过，使用 Default::default()
    }

    let created_event = ChangeLogEntry::new(
        "700",
        "OrderWithCache",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "700"),
                FieldChange::new("symbol", "", "\"ETHUSDT\""),
                FieldChange::new("price", "", "3500.0"),
                FieldChange::new("cache", "", "\"this_is_ignored\""),  // 被忽略
            ],
        },
        11000,
        700,
    );

    let order = OrderWithCache::from_created_event(&created_event).unwrap();

    assert_eq!(order.id, 700);
    assert_eq!(order.symbol, "ETHUSDT");
    assert_eq!(order.price, 3500.0);
    assert_eq!(order.cache, "");  // Default::default() for String
}

#[test]
fn test_from_created_event_numeric_types() {
    use diff::FromCreatedEvent;

    // 测试各种数值类型
    #[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
    struct NumericOrder {
        id: u64,
        quantity: i64,
        price: f64,
    }

    let created_event = ChangeLogEntry::new(
        "800",
        "NumericOrder",
        ChangeType::Created {
            fields: vec![
                FieldChange::new("id", "", "800"),
                FieldChange::new("quantity", "", "-100"),  // 负数 i64
                FieldChange::new("price", "", "99.99"),
            ],
        },
        12000,
        800,
    );

    let order = NumericOrder::from_created_event(&created_event).unwrap();

    assert_eq!(order.id, 800);
    assert_eq!(order.quantity, -100);
    assert_eq!(order.price, 99.99);
}

// ============================================================================
// TableSchema 相关测试
// ============================================================================

#[test]
fn test_table_schema_auto_generation() {
    // 自动生成表结构
    let schema = Order::table_schema();

    // 验证表名
    assert_eq!(schema.table_name, "order");

    // 验证字段数量
    assert_eq!(schema.fields.len(), 5);

    // 验证字段信息
    assert_eq!(schema.fields[0].field_name, "id");
    assert_eq!(schema.fields[0].field_type, "u64");
    assert_eq!(schema.fields[0].default_value, "0");

    assert_eq!(schema.fields[1].field_name, "symbol");
    assert_eq!(schema.fields[1].field_type, "String");

    assert_eq!(schema.fields[2].field_name, "price");
    assert_eq!(schema.fields[2].field_type, "f64");
    assert_eq!(schema.fields[2].default_value, "0.0");

    assert_eq!(schema.fields[3].field_name, "quantity");
    assert_eq!(schema.fields[3].field_type, "u64");

    assert_eq!(schema.fields[4].field_name, "status");
    assert_eq!(schema.fields[4].field_type, "String");
}

#[test]
fn test_table_name_const_method() {
    // 测试 table_name() const 方法
    let name = Order::table_name();
    assert_eq!(name, "order");
}

#[test]
fn test_table_schema_validation() {
    let schema = Order::table_schema();

    // 验证表结构的完整性
    assert!(schema.validate().is_ok());

    // 验证字段数量
    assert_eq!(schema.field_count(), 5);

    // 检查包含指定字段
    assert!(schema.has_field("id"));
    assert!(schema.has_field("symbol"));
    assert!(schema.has_field("price"));
    assert!(!schema.has_field("nonexistent"));
}

#[test]
fn test_table_schema_field_lookup() {
    let schema = Order::table_schema();

    // 查找字段
    let id_field = schema.find_field("id");
    assert!(id_field.is_some());
    assert_eq!(id_field.unwrap().field_name, "id");

    let symbol_field = schema.find_field("symbol");
    assert!(symbol_field.is_some());
    assert_eq!(symbol_field.unwrap().field_type, "String");

    // 查找不存在的字段
    let nonexistent = schema.find_field("nonexistent");
    assert!(nonexistent.is_none());
}

#[test]
fn test_table_schema_field_names() {
    let schema = Order::table_schema();

    let names = schema.field_names();
    assert_eq!(names, vec!["id", "symbol", "price", "quantity", "status"]);
}

#[test]
fn test_table_schema_summary() {
    let schema = Order::table_schema();
    let summary = schema.summary();

    assert!(summary.contains("Table 'order'"));
    assert!(summary.contains("5 fields"));
    assert!(summary.contains("id(u64)"));
    assert!(summary.contains("symbol(String)"));
}

#[test]
fn test_table_schema_display() {
    let schema = Order::table_schema();
    let display_str = format!("{}", schema);

    assert!(display_str.contains("Table 'order'"));
    assert!(display_str.contains("5 fields"));
}

#[test]
fn test_custom_entity_table_schema() {
    // 测试自定义类型的表结构
    #[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
    #[entity(type_name = "CustomOrder")]
    struct CustomOrder {
        id: u64,
        amount: f64,
    }

    let schema = CustomOrder::table_schema();

    // 表名应该基于结构体名小写
    assert_eq!(schema.table_name, "customorder");

    // 验证字段
    assert_eq!(schema.fields.len(), 2);
    assert_eq!(schema.fields[0].field_name, "id");
    assert_eq!(schema.fields[1].field_name, "amount");
}

