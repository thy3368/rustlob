#[derive(Debug, Clone)]
pub enum ChangeType {
    Created,
    Updated { changed_fields: Vec<FieldChange> },
    Deleted
}

/// 字段变更记录
#[derive(Debug, Clone)]
pub struct FieldChange {
    pub field_name: String,
    pub old_value: String,
    pub new_value: String,
}

// 变更日志条目
#[derive(Debug, Clone)]
pub struct ChangeLogEntry {
    pub entity_id: String,
    pub entity_type: String,
    pub change_type: ChangeType,
    pub timestamp: u64,
}

/// 变更追踪器 - 在 update 闭包中使用
pub struct ChangeTracker {
    changes: Vec<FieldChange>,
}

impl ChangeTracker {
    fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    /// 记录字段变更 - 支持不同类型的old_value和new_value
    pub fn record<T: ToString, U: ToString>(&mut self, field_name: &str, old_value: T, new_value: U) {
        self.changes.push(FieldChange {
            field_name: field_name.to_string(),
            old_value: old_value.to_string(),
            new_value: new_value.to_string(),
        });
    }

    /// ✨ 更新字段并自动记录变更（推荐使用）
    ///
    /// # Example
    /// ```
    /// tracker.set("value", &mut entity.value, 150);
    /// tracker.set("name", &mut entity.name, "Updated".to_string());
    /// ```
    pub fn set<T>(&mut self, field_name: &str, field: &mut T, new_value: T)
    where
        T: ToString + Clone
    {
        let old_value = field.clone();
        self.changes.push(FieldChange {
            field_name: field_name.to_string(),
            old_value: old_value.to_string(),
            new_value: new_value.to_string(),
        });
        *field = new_value;
    }

    fn into_changes(self) -> Vec<FieldChange> {
        self.changes
    }
}

/// 便捷宏：自动追踪字段变更
///
/// # Example
/// ```
/// manager.update(|entity, tracker| {
///     track!(tracker, entity.value = 150);
///     track!(tracker, entity.name = "Updated".to_string());
/// });
/// ```
#[macro_export]
macro_rules! track {
    ($tracker:expr, $($field:tt).+ = $value:expr) => {{
        $tracker.set(stringify!($($field).+), &mut $($field).+, $value);
    }};
}

pub struct EntityManager<T> {
    entity: T
}

impl<T> EntityManager<T>
where
    T: Clone + 'static
{
    pub fn new(entity: T) -> Self {
        Self {
            entity
        }
    }

    /// 更新实体并通过 tracker 记录字段变更
    ///
    /// # Example
    /// ```
    /// manager.update(|entity, tracker| {
    ///     tracker.record("value", entity.value, 150);
    ///     entity.value = 150;
    ///
    ///     tracker.record("name", &entity.name, &"Updated");
    ///     entity.name = "Updated".to_string();
    /// }).unwrap();
    /// ```
    pub fn update<F>(&mut self, updater: F) -> Result<ChangeLogEntry, Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut T, &mut ChangeTracker)
    {
        let mut tracker = ChangeTracker::new();

        // 应用更新，同时记录变更
        updater(&mut self.entity, &mut tracker);

        let field_changes = tracker.into_changes();

        let entry = ChangeLogEntry {
            entity_id: "example_id".to_string(),
            entity_type: std::any::type_name::<T>().to_string(),
            change_type: ChangeType::Updated {
                changed_fields: field_changes
            },
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
        };

        Ok(entry)
    }

    /// 🎯 自动追踪模式 - 通过 Diff trait 自动检测变更
    ///
    /// 适用场景：实体通过方法修改属性
    ///
    /// # Example
    /// ```
    /// manager.update_auto(|entity| {
    ///     entity.increment_value();  // 方法调用
    ///     entity.set_status(Status::Active);
    /// }).unwrap();
    /// ```
    pub fn update_auto<F>(&mut self, updater: F) -> Result<ChangeLogEntry, Box<dyn std::error::Error>>
    where
        T: Diff,
        F: FnOnce(&mut T)
    {
        // 1. 克隆旧状态
        let old_entity = self.entity.clone();

        // 2. 执行更新（可以是任何方法调用）
        updater(&mut self.entity);

        // 3. 自动 diff 检测变更
        let field_changes = old_entity.diff(&self.entity);

        let entry = ChangeLogEntry {
            entity_id: "example_id".to_string(),
            entity_type: std::any::type_name::<T>().to_string(),
            change_type: ChangeType::Updated {
                changed_fields: field_changes
            },
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
        };

        Ok(entry)
    }
}

/// Diff trait - 用于自动检测字段变更
pub trait Diff {
    /// 比较 self(旧) 和 other(新)，返回字段变更列表
    fn diff(&self, other: &Self) -> Vec<FieldChange>;
}


#[cfg(test)]
mod tests {
    use super::*;

    // 简单的测试用实体 - 不需要实现 Diff trait！
    #[derive(Debug, Clone)]
    struct TestEntity {
        id: String,
        value: i64,
        name: String,
    }

    #[test]
    fn test_record_log_with_tracker() {
        let entity = TestEntity {
            id: "test_1".to_string(),
            value: 100,
            name: "Initial".to_string(),
        };

        let mut entity_manager = EntityManager::new(entity.clone());

        // ✨ 使用 tracker 在 update 中记录变更
        let entry = entity_manager.update(|entity, tracker| {
            // 记录 value 的变更
            tracker.record("value", entity.value, 150);
            entity.value = 150;

            // 记录 name 的变更
            tracker.record("name", &entity.name, "Updated");
            entity.name = "Updated".to_string();
        }).unwrap();

        // 验证 entry
        println!("\n=== ChangeLogEntry 使用 Tracker 记录变更 ===");

        assert_eq!(entry.entity_id, "example_id");
        println!("✓ entity_id: {}", entry.entity_id);

        assert_eq!(entry.entity_type, "prep_proc::proc::repo::EntityManager::tests::TestEntity");
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
    fn test_single_field_with_tracker() {
        let entity = TestEntity {
            id: "test_2".to_string(),
            value: 50,
            name: "Original".to_string(),
        };

        let mut entity_manager = EntityManager::new(entity);

        // 只更新一个字段
        let entry = entity_manager.update(|e, tracker| {
            tracker.record("value", e.value, 75);
            e.value = 75;
        }).unwrap();

        println!("\n=== 单字段更新 (Tracker) ===");
        if let ChangeType::Updated { changed_fields } = &entry.change_type {
            assert_eq!(changed_fields.len(), 1);
            println!("✓ 记录了 1 个字段变更");

            let change = &changed_fields[0];
            assert_eq!(change.field_name, "value");
            println!("  - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }

        println!("=== 单字段更新测试通过! ===\n");
    }

    #[test]
    fn test_no_tracking() {
        let entity = TestEntity {
            id: "test_3".to_string(),
            value: 100,
            name: "Unchanged".to_string(),
        };

        let mut entity_manager = EntityManager::new(entity);

        // 不记录任何变更（开发者可以选择不追踪）
        let entry = entity_manager.update(|e, _tracker| {
            e.value = 200;  // 改了但没记录
        }).unwrap();

        println!("\n=== 不使用 Tracker 测试 ===");
        if let ChangeType::Updated { changed_fields } = &entry.change_type {
            assert_eq!(changed_fields.len(), 0);
            println!("✓ 开发者选择不追踪，变更数为 0");
        }

        println!("=== 不追踪测试通过! ===\n");
    }

    #[test]
    fn test_partial_tracking() {
        let entity = TestEntity {
            id: "test_4".to_string(),
            value: 50,
            name: "Original".to_string(),
        };

        let mut entity_manager = EntityManager::new(entity);

        // 只追踪部分字段
        let entry = entity_manager.update(|e, tracker| {
            // 追踪 value
            tracker.record("value", e.value, 75);
            e.value = 75;

            // 不追踪 name 的变更
            e.name = "Modified".to_string();
        }).unwrap();

        println!("\n=== 部分字段追踪测试 ===");
        if let ChangeType::Updated { changed_fields } = &entry.change_type {
            assert_eq!(changed_fields.len(), 1);
            println!("✓ 只追踪了 1 个字段（开发者选择）");
            println!("  - {}: {} → {}",
                changed_fields[0].field_name,
                changed_fields[0].old_value,
                changed_fields[0].new_value);
        }

        println!("=== 部分追踪测试通过! ===\n");
    }

    #[test]
    fn test_convenient_api() {
        let entity = TestEntity {
            id: "test_6".to_string(),
            value: 100,
            name: "Initial".to_string(),
        };

        let mut entity_manager = EntityManager::new(entity);

        // 🎉 使用 set() 方法 - 一步完成记录和更新，不会不同步！
        let entry = entity_manager.update(|entity, tracker| {
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
    fn test_compare_old_vs_new_api() {
        println!("\n=== API 对比测试 ===\n");

        // ❌ 旧方式：容易不同步
        println!("❌ 旧方式（容易出错）:");
        println!("   tracker.record(\"value\", entity.value, 150);");
        println!("   entity.value = 150;  // 可能忘记或写错");
        println!("");

        // ✅ 新方式：自动同步
        println!("✅ 新方式（不会出错）:");
        println!("   tracker.set(\"value\", &mut entity.value, 150);  // 一步完成！");
        println!("");

        let entity = TestEntity {
            id: "test_7".to_string(),
            value: 50,
            name: "Test".to_string(),
        };

        let mut manager = EntityManager::new(entity);

        let entry = manager.update(|e, tracker| {
            // 使用新的便捷API
            tracker.set("value", &mut e.value, 75);
            tracker.set("name", &mut e.name, "Modified".to_string());
        }).unwrap();

        if let ChangeType::Updated { changed_fields } = &entry.change_type {
            assert_eq!(changed_fields.len(), 2);
            println!("✓ 使用 tracker.set() 成功记录了 {} 个变更", changed_fields.len());
            for change in changed_fields {
                println!("  - {}: {} → {}",
                    change.field_name,
                    change.old_value,
                    change.new_value);
            }
        }

        println!("\n=== API 对比测试通过! ===\n");
    }

    #[test]
    fn test_macro_api() {
        let entity = TestEntity {
            id: "test_8".to_string(),
            value: 100,
            name: "Initial".to_string(),
        };

        let mut manager = EntityManager::new(entity);

        // 🎉 使用宏 - 最简洁的方式！
        let entry = manager.update(|entity, tracker| {
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

    #[test]
    fn test_all_approaches() {
        println!("\n=== 三种方式对比 ===\n");

        // 方式1: 手动 record（不推荐）
        println!("方式1: tracker.record()");
        println!("  tracker.record(\"value\", entity.value, 150);");
        println!("  entity.value = 150;");
        println!("  问题: 容易不同步");
        println!();

        // 方式2: tracker.set()（推荐）
        println!("方式2: tracker.set()");
        println!("  tracker.set(\"value\", &mut entity.value, 150);");
        println!("  优势: 自动同步");
        println!();

        // 方式3: track! 宏（最简洁）
        println!("方式3: track! 宏");
        println!("  track!(tracker, entity.value = 150);");
        println!("  优势: 最简洁，自动获取字段名");
        println!();

        let entity = TestEntity {
            id: "test_9".to_string(),
            value: 50,
            name: "Test".to_string(),
        };

        let mut manager = EntityManager::new(entity);

        let entry = manager.update(|entity, tracker| {
            track!(tracker, entity.value = 100);
            track!(tracker, entity.name = "Macro Test".to_string());
        }).unwrap();

        if let ChangeType::Updated { changed_fields } = &entry.change_type {
            println!("实际效果:");
            for change in changed_fields {
                println!("  {} : {} → {}",
                    change.field_name,
                    change.old_value,
                    change.new_value);
            }
        }

        println!("\n=== 对比测试完成! ===\n");
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
        println!("\n=== 自动追踪模式测试 (update_auto) ===\n");

        let entity = TestEntity {
            id: "test_10".to_string(),
            value: 100,
            name: "Initial".to_string(),
        };

        let mut manager = EntityManager::new(entity);

        // 🎯 使用 update_auto - 通过方法修改实体
        let entry = manager.update_auto(|entity| {
            entity.increment_value();  // 方法调用: 100 -> 150
            entity.set_name("Updated".to_string());  // 方法调用
        }).unwrap();

        println!("✓ 使用 update_auto() 自动检测方法调用的变更");

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

        let entity = TestEntity {
            id: "test_11".to_string(),
            value: 50,
            name: "Order".to_string(),
        };

        let mut manager = EntityManager::new(entity);

        // 🎯 调用包含多个字段变更的业务方法
        let entry = manager.update_auto(|entity| {
            entity.apply_business_logic();  // 内部修改了 value 和 name
        }).unwrap();

        println!("✓ 使用 update_auto() 追踪复杂业务方法");

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
    fn test_compare_update_vs_update_auto() {
        println!("\n=== update() vs update_auto() 对比 ===\n");

        // 场景1: 直接赋值 - 使用 update() + track! 宏
        println!("场景1: 直接字段赋值");
        println!("  推荐使用: update() + track! 宏");
        println!("  示例:");
        println!("    manager.update(|entity, tracker| {{");
        println!("        track!(tracker, entity.value = 150);");
        println!("    }})");
        println!();

        let entity1 = TestEntity {
            id: "test_12a".to_string(),
            value: 100,
            name: "Test1".to_string(),
        };

        let mut manager1 = EntityManager::new(entity1);
        let entry1 = manager1.update(|entity, tracker| {
            track!(tracker, entity.value = 150);
        }).unwrap();

        if let ChangeType::Updated { changed_fields } = &entry1.change_type {
            println!("  结果: 检测到 {} 个变更", changed_fields.len());
            for change in changed_fields {
                println!("    - {}: {} → {}", change.field_name, change.old_value, change.new_value);
            }
        }

        println!();

        // 场景2: 方法调用 - 使用 update_auto()
        println!("场景2: 通过方法修改");
        println!("  推荐使用: update_auto()");
        println!("  示例:");
        println!("    manager.update_auto(|entity| {{");
        println!("        entity.increment_value();");
        println!("    }})");
        println!();

        let entity2 = TestEntity {
            id: "test_12b".to_string(),
            value: 100,
            name: "Test2".to_string(),
        };

        let mut manager2 = EntityManager::new(entity2);
        let entry2 = manager2.update_auto(|entity| {
            entity.increment_value();
        }).unwrap();

        if let ChangeType::Updated { changed_fields } = &entry2.change_type {
            println!("  结果: 检测到 {} 个变更", changed_fields.len());
            for change in changed_fields {
                println!("    - {}: {} → {}", change.field_name, change.old_value, change.new_value);
            }
        }

        println!("\n=== 对比测试完成! ===\n");
        println!("总结:");
        println!("  ✅ 直接赋值 → 使用 track! 宏 (最简洁)");
        println!("  ✅ 方法调用 → 使用 update_auto() (自动检测)");
        println!();
    }

    #[test]
    fn test_performance_comparison() {
        use std::time::Instant;

        println!("\n=== 性能对比测试 ===\n");

        const ITERATIONS: usize = 10_000;

        // 测试 1: track! 宏性能
        let entity = TestEntity {
            id: "perf_test".to_string(),
            value: 0,
            name: "Test".to_string(),
        };
        let mut manager = EntityManager::new(entity);

        let start = Instant::now();
        for i in 0..ITERATIONS {
            manager.update(|e, t| {
                track!(t, e.value = i as i64);
            }).unwrap();
        }
        let track_duration = start.elapsed();

        println!("✓ track! 宏:");
        println!("  - 总耗时: {:?}", track_duration);
        println!("  - 平均每次: {:?}", track_duration / ITERATIONS as u32);
        println!("  - 吞吐量: {:.2} ops/sec", ITERATIONS as f64 / track_duration.as_secs_f64());
        println!();

        // 测试 2: update_auto() 性能
        let entity2 = TestEntity {
            id: "perf_test2".to_string(),
            value: 0,
            name: "Test2".to_string(),
        };
        let mut manager2 = EntityManager::new(entity2);

        let start2 = Instant::now();
        for i in 0..ITERATIONS {
            manager2.update_auto(|e| {
                e.value = i as i64;
            }).unwrap();
        }
        let auto_duration = start2.elapsed();

        println!("✓ update_auto():");
        println!("  - 总耗时: {:?}", auto_duration);
        println!("  - 平均每次: {:?}", auto_duration / ITERATIONS as u32);
        println!("  - 吞吐量: {:.2} ops/sec", ITERATIONS as f64 / auto_duration.as_secs_f64());
        println!();

        // 性能对比
        let speedup = auto_duration.as_secs_f64() / track_duration.as_secs_f64();
        println!("📊 性能对比:");
        println!("  - track! 宏相比 update_auto() 快 {:.2}x", speedup);
        println!("  - 原因: update_auto() 需要 Clone 整个实体");
        println!();

        println!("=== 性能测试完成! ===\n");
    }
}
