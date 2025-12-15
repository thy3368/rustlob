use diff::{ChangeType, Diff, FieldChange};
use diff_tracker::track;
use diff_tracker::tracker::tracker::{track_auto, track_changes, track_with_tracker};

// 简单的测试用实体 - 不需要实现 Diff trait！
#[derive(Debug, Clone)]
struct TestEntity {
    id: String,
    value: i64,
    name: String,
}

#[test]
fn test_record_log_with_tracker() {
    let mut entity = TestEntity {
        id: "test_1".to_string(),
        value: 100,
        name: "Initial".to_string(),
    };

    // ✨ 使用 track_with_tracker 记录变更
    let entry = track_with_tracker(&mut entity, |entity, tracker| {
        // 记录 value 的变更
        tracker.record("value", entity.value, 150);
        entity.value = 150;

        // 记录 name 的变更
        tracker.record("name", &entity.name, "Updated");
        entity.name = "Updated".to_string();
    }).unwrap();

    // 验证 entry
    println!("\n=== ChangeLogEntry 使用 Tracker 记录变更 ===");

    assert_eq!(entry.entity_id, "auto_generated");
    println!("✓ entity_id: {}", entry.entity_id);

    assert!(entry.entity_type.contains("TestEntity"));
    println!("✓ entity_type: {}", entry.entity_type);

    // 验证字段变更
    match &entry.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 2);
            println!("✓ 记录了 {} 个字段变更", changed_fields.len());

            // 验证 value 变更
            let value_change = &changed_fields[0];
            assert_eq!(value_change.field_name, "value");
            assert_eq!(value_change.old_value, "100");
            assert_eq!(value_change.new_value, "150");
            println!("  ✓ 字段: {} | 旧值: {} → 新值: {}",
                value_change.field_name,
                value_change.old_value,
                value_change.new_value);

            // 验证 name 变更
            let name_change = &changed_fields[1];
            assert_eq!(name_change.field_name, "name");
            assert_eq!(name_change.old_value, "Initial");
            assert_eq!(name_change.new_value, "Updated");
            println!("  ✓ 字段: {} | 旧值: {} → 新值: {}",
                name_change.field_name,
                name_change.old_value,
                name_change.new_value);
        }
        _ => panic!("Expected ChangeType::Updated"),
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!(entry.timestamp <= now);
    println!("✓ timestamp: {}", entry.timestamp);

    println!("\n=== Tracker 变更追踪验证通过! ===\n");
}

#[test]
fn test_convenient_api() {
    let mut entity = TestEntity {
        id: "test_6".to_string(),
        value: 100,
        name: "Initial".to_string(),
    };

    // 🎉 使用 set() 方法 - 一步完成记录和更新，不会不同步！
    let entry = track_with_tracker(&mut entity, |entity, tracker| {
        tracker.set("value", &mut entity.value, 150);
        tracker.set("name", &mut entity.name, "Updated".to_string());
    }).unwrap();

    println!("\n=== 便捷 API 测试 (tracker.set) ===");

    // 验证字段已经更新
    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ 记录了 {} 个字段变更", changed_fields.len());

        // 验证 value
        let value_change = &changed_fields[0];
        assert_eq!(value_change.field_name, "value");
        assert_eq!(value_change.old_value, "100");
        assert_eq!(value_change.new_value, "150");
        println!("  ✓ {}: {} → {} (自动同步)",
            value_change.field_name,
            value_change.old_value,
            value_change.new_value);

        // 验证 name
        let name_change = &changed_fields[1];
        assert_eq!(name_change.field_name, "name");
        assert_eq!(name_change.old_value, "Initial");
        assert_eq!(name_change.new_value, "Updated");
        println!("  ✓ {}: {} → {} (自动同步)",
            name_change.field_name,
            name_change.old_value,
            name_change.new_value);
    }

    println!("=== 便捷 API 测试通过! ===\n");
}

#[test]
fn test_macro_api() {
    let mut entity = TestEntity {
        id: "test_8".to_string(),
        value: 100,
        name: "Initial".to_string(),
    };

    // 🎉 使用宏 - 最简洁的方式！
    let entry = track_with_tracker(&mut entity, |entity, tracker| {
        track!(tracker, entity.value = 150);
        track!(tracker, entity.name = "Updated".to_string());
    }).unwrap();

    println!("\n=== 宏 API 测试 (track!) ===");

    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ 使用 track! 宏记录了 {} 个字段变更", changed_fields.len());

        let value_change = &changed_fields[0];
        assert_eq!(value_change.field_name, "entity.value");
        assert_eq!(value_change.old_value, "100");
        assert_eq!(value_change.new_value, "150");
        println!("  ✓ {}: {} → {}",
            value_change.field_name,
            value_change.old_value,
            value_change.new_value);

        let name_change = &changed_fields[1];
        assert_eq!(name_change.field_name, "entity.name");
        assert_eq!(name_change.old_value, "Initial");
        assert_eq!(name_change.new_value, "Updated");
        println!("  ✓ {}: {} → {}",
            name_change.field_name,
            name_change.old_value,
            name_change.new_value);
    }

    println!("\n=== 宏 API 测试通过! ===\n");
}

// 为TestEntity实现Diff trait以支持自动追踪
impl Diff for TestEntity {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        if self.value != other.value {
            changes.push(FieldChange {
                field_name: "value".to_string(),
                old_value: self.value.to_string(),
                new_value: other.value.to_string(),
            });
        }

        if self.name != other.name {
            changes.push(FieldChange {
                field_name: "name".to_string(),
                old_value: self.name.clone(),
                new_value: other.name.clone(),
            });
        }

        changes
    }
}

// 为TestEntity添加方法来模拟业务操作
impl TestEntity {
    fn increment_value(&mut self) {
        self.value += 50;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn apply_business_logic(&mut self) {
        self.value = self.value * 2;
        self.name = format!("{}_processed", self.name);
    }
}

#[test]
fn test_update_auto_with_methods() {
    println!("\n=== 自动追踪模式测试 (track_auto) ===\n");

    let mut entity = TestEntity {
        id: "test_10".to_string(),
        value: 100,
        name: "Initial".to_string(),
    };

    // 🎯 使用 track_auto - 通过方法修改实体
    let entry = track_auto(&mut entity, |entity| {
        entity.increment_value();  // 方法调用: 100 -> 150
        entity.set_name("Updated".to_string());  // 方法调用
    }).unwrap();

    println!("✓ 使用 track_auto() 自动检测方法调用的变更");

    // 验证变更
    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ 自动检测到 {} 个字段变更", changed_fields.len());

        // 验证 value 变更
        let value_change = &changed_fields[0];
        assert_eq!(value_change.field_name, "value");
        assert_eq!(value_change.old_value, "100");
        assert_eq!(value_change.new_value, "150");
        println!("  ✓ {}: {} → {} (方法: increment_value)",
            value_change.field_name,
            value_change.old_value,
            value_change.new_value);

        // 验证 name 变更
        let name_change = &changed_fields[1];
        assert_eq!(name_change.field_name, "name");
        assert_eq!(name_change.old_value, "Initial");
        assert_eq!(name_change.new_value, "Updated");
        println!("  ✓ {}: {} → {} (方法: set_name)",
            name_change.field_name,
            name_change.old_value,
            name_change.new_value);
    }

    println!("\n=== 自动追踪测试通过! ===\n");
}

#[test]
fn test_update_auto_complex_business_logic() {
    println!("\n=== 复杂业务逻辑自动追踪测试 ===\n");

    let mut entity = TestEntity {
        id: "test_11".to_string(),
        value: 50,
        name: "Order".to_string(),
    };

    // 🎯 调用包含多个字段变更的业务方法
    let entry = track_auto(&mut entity, |entity| {
        entity.apply_business_logic();  // 内部修改了 value 和 name
    }).unwrap();

    println!("✓ 使用 track_auto() 追踪复杂业务方法");

    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ 自动检测到 {} 个字段变更", changed_fields.len());

        for change in changed_fields {
            println!("  - {}: {} → {}",
                change.field_name,
                change.old_value,
                change.new_value);
        }

        // 验证具体变更
        assert_eq!(changed_fields[0].old_value, "50");
        assert_eq!(changed_fields[0].new_value, "100");  // 50 * 2
        assert_eq!(changed_fields[1].old_value, "Order");
        assert_eq!(changed_fields[1].new_value, "Order_processed");
    }

    println!("\n=== 复杂业务逻辑测试通过! ===\n");
}

#[test]
fn test_track_changes_standalone() {
    println!("\n=== 独立函数 track_changes 测试 ===\n");

    let mut entity = TestEntity {
        id: "test_standalone".to_string(),
        value: 100,
        name: "Original".to_string(),
    };

    // 🎯 直接使用 track_changes，无需创建 EntityManager！
    let entry = track_changes(&mut entity, |e| {
        e.value = 200;
        e.name = "Modified".to_string();
    }).unwrap();

    println!("✓ 使用 track_changes() 直接追踪变更（无需 EntityManager::new）");

    // 验证变更
    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ 检测到 {} 个字段变更", changed_fields.len());

        for change in changed_fields {
            println!("  - {}: {} → {}",
                change.field_name,
                change.old_value,
                change.new_value);
        }
    }

    // 验证实体已更新
    assert_eq!(entity.value, 200);
    assert_eq!(entity.name, "Modified");
    println!("✓ 实体状态已正确更新");

    println!("\n=== 独立函数测试通过! ===\n");
}

#[test]
fn test_compare_apis() {
    println!("\n=== API 对比 ===\n");

    // 方式 1: track_with_tracker（手动追踪）
    println!("方式 1: track_with_tracker（手动追踪）");
    let mut entity1 = TestEntity {
        id: "test_1".to_string(),
        value: 50,
        name: "Test1".to_string(),
    };
    let _entry1 = track_with_tracker(&mut entity1, |e, t| {
        track!(t, e.value = 100);
    }).unwrap();
    println!("  - 适用场景: 需要精确控制哪些字段被追踪");
    println!("  - 代码: track_with_tracker(&mut entity, |e, t| {{ ... }})");
    println!();

    // 方式 2: track_auto（自动追踪）
    println!("方式 2: track_auto（自动追踪）");
    let mut entity2 = TestEntity {
        id: "test_2".to_string(),
        value: 50,
        name: "Test2".to_string(),
    };
    let _entry2 = track_auto(&mut entity2, |e| {
        e.value = 100;
    }).unwrap();
    println!("  - 适用场景: 自动检测所有字段变更");
    println!("  - 代码: track_auto(&mut entity, |e| {{ ... }})");
    println!("  - 优势: 更简洁，自动检测变更");
    println!();

    println!("=== API 对比完成! ===\n");
}

#[test]
fn test_track_with_tracker() {
    println!("\n=== track_with_tracker 独立函数测试 ===\n");

    let mut entity = TestEntity {
        id: "test_tracker".to_string(),
        value: 100,
        name: "Initial".to_string(),
    };

    // 🎯 使用 track_with_tracker，无需 EntityManager！
    let entry = track_with_tracker(&mut entity, |e, tracker| {
        track!(tracker, e.value = 250);
        track!(tracker, e.name = "TrackerUpdated".to_string());
    }).unwrap();

    println!("✓ 使用 track_with_tracker() 直接追踪（无需 EntityManager::new）");

    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ 检测到 {} 个字段变更", changed_fields.len());

        for change in changed_fields {
            println!("  - {}: {} → {}",
                change.field_name,
                change.old_value,
                change.new_value);
        }
    }

    // 验证实体已更新
    assert_eq!(entity.value, 250);
    assert_eq!(entity.name, "TrackerUpdated");
    println!("✓ 实体状态已正确更新");

    println!("\n=== track_with_tracker 测试通过! ===\n");
}

#[test]
fn test_all_standalone_functions() {
    println!("\n=== 完整 API 演示 ===\n");

    // === 场景 1: 使用 track! 宏 ===
    println!("【场景 1】使用 track! 宏手动追踪");
    println!();

    println!("  ✅ 使用 track_with_tracker:");
    println!("     track_with_tracker(&mut entity, |e, t| {{ track!(t, e.value = 100); }})");
    println!();

    let mut entity1 = TestEntity {
        id: "test1".to_string(),
        value: 50,
        name: "Test1".to_string(),
    };

    let entry1 = track_with_tracker(&mut entity1, |e, t| {
        track!(t, e.value = 100);
    }).unwrap();

    if let ChangeType::Updated { changed_fields } = &entry1.change_type {
        println!("  结果: {} 个字段变更", changed_fields.len());
        for change in changed_fields {
            println!("    - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }
    }
    println!();

    // === 场景 2: 使用自动追踪 ===
    println!("【场景 2】自动追踪（方法调用）");
    println!();

    println!("  ✅ 使用 track_auto:");
    println!("     track_auto(&mut entity, |e| {{ e.increment_value(); }})");
    println!();

    let mut entity2 = TestEntity {
        id: "test2".to_string(),
        value: 100,
        name: "Test2".to_string(),
    };

    let entry2 = track_auto(&mut entity2, |e| {
        e.increment_value();
    }).unwrap();

    if let ChangeType::Updated { changed_fields } = &entry2.change_type {
        println!("  结果: {} 个字段变更", changed_fields.len());
        for change in changed_fields {
            println!("    - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }
    }
    println!();

    println!("=== 总结 ===");
    println!("✅ track_with_tracker: 手动追踪，精确控制");
    println!("✅ track_auto / track_changes: 自动追踪，简洁方便");
    println!();
}

// ========== Diff Derive 宏测试 ==========

// 🎉 使用 Diff derive 宏的测试实体
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct User {
    id: String,
    name: String,
    age: i32,
}

#[test]
fn test_diff_derive_basic() {
    println!("\n=== Diff Derive 宏测试 - 基础功能 ===\n");

    let user1 = User {
        id: "user_001".to_string(),
        name: "Alice".to_string(),
        age: 30,
    };

    let user2 = User {
        id: "user_001".to_string(),
        name: "Alice Smith".to_string(),
        age: 31,
    };

    // 使用自动生成的 diff 方法
    let changes = user1.diff(&user2);

    assert_eq!(changes.len(), 2);
    println!("✓ 检测到 {} 个字段变更", changes.len());

    // 验证 name 变更
    let name_change = changes.iter().find(|c| c.field_name == "name").unwrap();
    assert_eq!(name_change.old_value, "Alice");
    assert_eq!(name_change.new_value, "Alice Smith");
    println!("  ✓ name: {} → {}", name_change.old_value, name_change.new_value);

    // 验证 age 变更
    let age_change = changes.iter().find(|c| c.field_name == "age").unwrap();
    assert_eq!(age_change.old_value, "30");
    assert_eq!(age_change.new_value, "31");
    println!("  ✓ age: {} → {}", age_change.old_value, age_change.new_value);

    println!("\n=== Diff Derive 基础测试通过! ===\n");
}

#[test]
fn test_diff_derive_no_changes() {
    let user1 = User {
        id: "user_002".to_string(),
        name: "Bob".to_string(),
        age: 25,
    };

    let user2 = user1.clone();

    let changes = user1.diff(&user2);

    assert_eq!(changes.len(), 0);
    println!("✓ 正确检测到 0 个字段变更");
}

// 测试 skip 属性
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct Account {
    id: String,
    balance: i64,
    #[diff(skip)]
    internal_cache: Option<String>,
}

#[test]
fn test_diff_skip_attribute() {
    println!("\n=== Diff Derive - skip 属性测试 ===\n");

    let acc1 = Account {
        id: "acc_001".to_string(),
        balance: 1000,
        internal_cache: Some("cache1".to_string()),
    };

    let acc2 = Account {
        id: "acc_001".to_string(),
        balance: 2000,
        internal_cache: Some("cache2".to_string()),  // 这个字段会被跳过
    };

    let changes = acc1.diff(&acc2);

    // 只应该检测到 balance 的变更，internal_cache 被跳过
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].field_name, "balance");
    assert_eq!(changes[0].old_value, "1000");
    assert_eq!(changes[0].new_value, "2000");

    println!("✓ skip 属性生效：只检测到 1 个字段变更 (balance)");
    println!("✓ internal_cache 被正确跳过");

    println!("\n=== skip 属性测试通过! ===\n");
}

// 测试 mask 属性
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct SecureUser {
    id: String,
    username: String,
    #[diff(mask)]
    password: String,
}

#[test]
fn test_diff_mask_attribute() {
    println!("\n=== Diff Derive - mask 属性测试 ===\n");

    let user1 = SecureUser {
        id: "user_003".to_string(),
        username: "alice".to_string(),
        password: "old_password".to_string(),
    };

    let user2 = SecureUser {
        id: "user_003".to_string(),
        username: "alice_updated".to_string(),
        password: "new_password".to_string(),
    };

    let changes = user1.diff(&user2);

    assert_eq!(changes.len(), 2);

    // 验证 username 变更
    let username_change = changes.iter().find(|c| c.field_name == "username").unwrap();
    assert_eq!(username_change.old_value, "alice");
    assert_eq!(username_change.new_value, "alice_updated");
    println!("✓ username: {} → {}", username_change.old_value, username_change.new_value);

    // 验证 password 被脱敏
    let password_change = changes.iter().find(|c| c.field_name == "password").unwrap();
    assert_eq!(password_change.old_value, "***");
    assert_eq!(password_change.new_value, "***");
    println!("✓ password 被脱敏为: ***");

    println!("\n=== mask 属性测试通过! ===\n");
}

// 测试 Diff derive 与 track_auto 集成
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct Order {
    id: String,
    amount: i64,
    status: String,
}

#[test]
fn test_diff_derive_with_track_auto() {
    println!("\n=== Diff Derive + track_auto 集成测试 ===\n");

    let mut order = Order {
        id: "ord_001".to_string(),
        amount: 100,
        status: "pending".to_string(),
    };

    // 使用 track_auto，自动使用 derive 生成的 Diff 实现
    let entry = track_auto(&mut order, |o| {
        o.amount = 200;
        o.status = "completed".to_string();
    }).unwrap();

    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ track_auto + Diff derive: 检测到 {} 个字段变更", changed_fields.len());

        for change in changed_fields {
            println!("  - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }
    }

    println!("\n=== 集成测试通过! ===\n");
}
