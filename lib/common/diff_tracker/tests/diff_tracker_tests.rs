#![allow(clippy::unwrap_used)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::panic)]
#![allow(dead_code)]
#![allow(clippy::assign_op_pattern)]
use diff::{ChangeType, Diff, FieldChange};
use diff_tracker::track;
use diff_tracker::tracker::{track_auto, track_changes, track_with_tracker};

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
    println!("
=== ChangeLogEntry 使用 Tracker 记录变更 ===");

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

    println!("
=== Tracker 变更追踪验证通过! ===
");
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

    println!("
=== 便捷 API 测试 (tracker.set) ===");

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

    println!("=== 便捷 API 测试通过! ===
");
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

    println!("
=== 宏 API 测试 (track!) ===");

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

    println!("
=== 宏 API 测试通过! ===
");
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
    println!("
=== 自动追踪模式测试 (track_auto) ===
");

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

    println!("
=== 自动追踪测试通过! ===
");
}

#[test]
fn test_update_auto_complex_business_logic() {
    println!("
=== 复杂业务逻辑自动追踪测试 ===
");

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

    println!("
=== 复杂业务逻辑测试通过! ===
");
}

#[test]
fn test_track_changes_standalone() {
    println!("
=== 独立函数 track_changes 测试 ===
");

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

    println!("
=== 独立函数测试通过! ===
");
}

#[test]
fn test_compare_apis() {
    println!("
=== API 对比 ===
");

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

    println!("=== API 对比完成! ===
");
}

#[test]
fn test_track_with_tracker() {
    println!("
=== track_with_tracker 独立函数测试 ===
");

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

    println!("
=== track_with_tracker 测试通过! ===
");
}

#[test]
fn test_all_standalone_functions() {
    println!("
=== 完整 API 演示 ===
");

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
