use diff::{ChangeLogEntry, ChangeType, FieldChange, Replay};
use diff_tracker::tracker::tracker::track_auto;

// ========== Replay Derive 宏测试 ==========

/// 简单实体测试
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff, diff_tracker::Replay)]
struct Product {
    id: String,
    name: String,
    price: i64,
    stock: u32,
    is_available: bool,
}

#[test]
fn test_replay_derive_basic() {
    println!("\n=== Replay Derive 宏测试 - 基础功能 ===\n");

    // Given: 创建初始产品
    let mut product = Product {
        id: "PROD-001".to_string(),
        name: "Laptop".to_string(),
        price: 5000,
        stock: 10,
        is_available: true,
    };

    println!("初始状态:");
    println!("  ID: {}", product.id);
    println!("  名称: {}", product.name);
    println!("  价格: {}", product.price);
    println!("  库存: {}", product.stock);
    println!("  可用: {}", product.is_available);

    // 保存快照（clone）
    let product_snapshot = product.clone();

    // When: 使用 track_auto 录制变更
    println!("\n执行变更并录制...");
    let change_log = track_auto(&mut product, |p| {
        p.price = 4500;  // 降价
        p.stock = 15;    // 补货
    }).unwrap();

    println!("录制的变更:");
    if let ChangeType::Updated { changed_fields } = &change_log.change_type {
        for change in changed_fields {
            println!("  - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }
    }

    // 验证变更后的状态
    assert_eq!(product.price, 4500);
    assert_eq!(product.stock, 15);

    // Then: 从快照回放变更
    println!("\n从快照回放变更...");
    let mut product_replay = product_snapshot;  // 使用 clone 的快照

    println!("回放前: price={}, stock={}", product_replay.price, product_replay.stock);

    product_replay.replay(&change_log).unwrap();

    println!("回放后: price={}, stock={}", product_replay.price, product_replay.stock);

    // 验证回放结果与原始变更一致
    assert_eq!(product_replay.price, product.price);
    assert_eq!(product_replay.stock, product.stock);
    assert_eq!(product_replay.name, product.name);
    assert_eq!(product_replay.is_available, product.is_available);

    println!("\n✅ Replay Derive 基础测试通过!\n");
}

#[test]
fn test_replay_derive_all_types() {
    println!("\n=== Replay Derive - 支持的类型测试 ===\n");

    #[derive(Debug, Clone, diff_tracker::Diff, diff_tracker::Replay)]
    struct AllTypes {
        field_string: String,
        field_i64: i64,
        field_u32: u32,
        field_f64: f64,
        field_bool: bool,
    }

    // 创建初始实体
    let mut entity = AllTypes {
        field_string: "original".to_string(),
        field_i64: 100,
        field_u32: 50,
        field_f64: 3.14,
        field_bool: false,
    };

    // 保存快照
    let entity_snapshot = entity.clone();

    // 使用 track_auto 录制所有类型的变更
    let change_log = track_auto(&mut entity, |e| {
        e.field_string = "updated".to_string();
        e.field_i64 = 200;
        e.field_u32 = 75;
        e.field_f64 = 2.718;
        e.field_bool = true;
    }).unwrap();

    println!("录制的变更:");
    if let ChangeType::Updated { changed_fields } = &change_log.change_type {
        for change in changed_fields {
            println!("  - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }
    }

    // 从快照回放
    let mut entity_replay = entity_snapshot;
    entity_replay.replay(&change_log).unwrap();

    // 验证回放结果与原始变更一致
    assert_eq!(entity_replay.field_string, entity.field_string);
    assert_eq!(entity_replay.field_i64, entity.field_i64);
    assert_eq!(entity_replay.field_u32, entity.field_u32);
    assert_eq!(entity_replay.field_f64, entity.field_f64);
    assert_eq!(entity_replay.field_bool, entity.field_bool);

    println!("\n✓ String 类型回放成功");
    println!("✓ i64 类型回放成功");
    println!("✓ u32 类型回放成功");
    println!("✓ f64 类型回放成功");
    println!("✓ bool 类型回放成功");

    println!("\n✅ 所有类型回放测试通过!\n");
}

#[test]
fn test_replay_derive_error_handling() {
    println!("\n=== Replay Derive - 错误处理测试 ===\n");

    #[derive(Debug, diff_tracker::Replay)]
    struct TestEntity {
        number: i64,
    }

    let mut entity = TestEntity { number: 100 };

    // 测试无效的数字格式
    let invalid_log = ChangeLogEntry {
        entity_id: "test".to_string(),
        entity_type: "TestEntity".to_string(),
        change_type: ChangeType::Updated {
            changed_fields: vec![FieldChange {
                field_name: "number".to_string(),
                old_value: "100".to_string(),
                new_value: "not_a_number".to_string(),
            }],
        },
        timestamp: 0,
    };

    let result = entity.replay(&invalid_log);

    assert!(result.is_err());
    if let Err(e) = result {
        println!("✓ 捕获解析错误: {}", e);
        assert!(e.contains("Failed to parse field 'number'"));
    }

    // 测试 Created/Deleted 类型
    let wrong_type_log = ChangeLogEntry {
        entity_id: "test".to_string(),
        entity_type: "TestEntity".to_string(),
        change_type: ChangeType::Created,
        timestamp: 0,
    };

    let result = entity.replay(&wrong_type_log);
    assert!(result.is_err());
    if let Err(e) = result {
        println!("✓ 捕获类型错误: {}", e);
        assert_eq!(e, "Cannot replay: not an Update change");
    }

    println!("\n✅ 错误处理测试通过!\n");
}

#[test]
fn test_replay_derive_partial_update() {
    println!("\n=== Replay Derive - 部分字段更新测试 ===\n");

    // 创建初始产品
    let mut product = Product {
        id: "PROD-002".to_string(),
        name: "Phone".to_string(),
        price: 3000,
        stock: 20,
        is_available: true,
    };

    println!("初始状态: price={}, name={}, stock={}", product.price, product.name, product.stock);

    // 保存快照
    let product_snapshot = product.clone();

    // 使用 track_auto 只更新部分字段
    let partial_log = track_auto(&mut product, |p| {
        p.price = 2500;  // 只修改价格
    }).unwrap();

    println!("\n录制的变更:");
    if let ChangeType::Updated { changed_fields } = &partial_log.change_type {
        for change in changed_fields {
            println!("  - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }
    }

    // 从快照回放
    let mut product_replay = product_snapshot;

    println!("\n回放前: price={}, name={}, stock={}",
        product_replay.price, product_replay.name, product_replay.stock);

    product_replay.replay(&partial_log).unwrap();

    println!("回放后: price={}, name={}, stock={}",
        product_replay.price, product_replay.name, product_replay.stock);

    // 验证回放结果与原始变更一致
    assert_eq!(product_replay.price, product.price);
    assert_eq!(product_replay.name, product.name);
    assert_eq!(product_replay.stock, product.stock);
    assert_eq!(product_replay.is_available, product.is_available);

    println!("\n✓ 部分字段更新成功");
    println!("  - 已更新字段: price = {}", product_replay.price);
    println!("  - 未变更字段: name = {}", product_replay.name);
    println!("  - 未变更字段: stock = {}", product_replay.stock);

    println!("\n✅ 部分字段更新测试通过!\n");
}
