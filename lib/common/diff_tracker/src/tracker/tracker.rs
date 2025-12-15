// 从 diff 包导入核心类型
pub use diff::{ChangeType, FieldChange, ChangeLogEntry, Diff};

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
    /// ```ignore
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
/// ```ignore
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

// EntityManager 已移除，请使用独立函数：
// - track_with_tracker() 替代 manager.update()
// - track_auto() 替代 manager.update_auto()



/// 🎯 便捷函数：使用 tracker 手动追踪变更，无需创建 EntityManager
///
/// # Example
/// ```ignore
/// let mut order = Order::new();
/// let entry = track_with_tracker(&mut order, |o, tracker| {
///     track!(tracker, o.value = 200);
///     track!(tracker, o.status = "confirmed".to_string());
/// }).unwrap();
/// ```
pub fn track_with_tracker<T, F>(entity: &mut T, updater: F) -> Result<ChangeLogEntry, Box<dyn std::error::Error>>
where
    T: 'static,
    F: FnOnce(&mut T, &mut ChangeTracker)
{
    let mut tracker = ChangeTracker::new();

    // 应用更新，同时记录变更
    updater(entity, &mut tracker);

    let field_changes = tracker.into_changes();

    let entry = ChangeLogEntry {
        entity_id: "auto_generated".to_string(),
        entity_type: std::any::type_name::<T>().to_string(),
        change_type: ChangeType::Updated {
            changed_fields: field_changes
        },
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
    };

    Ok(entry)
}

/// 🎯 便捷函数：自动追踪变更（通过 Diff trait），无需创建 EntityManager
///
/// # Example
/// ```ignore
/// let mut order = Order::new();
/// let entry = track_auto(&mut order, |o| {
///     o.value = 200;
///     o.status = "confirmed".to_string();
/// }).unwrap();
/// ```
pub fn track_auto<T, F>(entity: &mut T, updater: F) -> Result<ChangeLogEntry, Box<dyn std::error::Error>>
where
    T: Clone + Diff + 'static,
    F: FnOnce(&mut T)
{
    // 1. 克隆旧状态
    let old_entity = entity.clone();

    // 2. 执行更新
    updater(entity);

    // 3. 自动 diff 检测变更
    let field_changes = old_entity.diff(entity);

    let entry = ChangeLogEntry {
        entity_id: "auto_generated".to_string(),
        entity_type: std::any::type_name::<T>().to_string(),
        change_type: ChangeType::Updated {
            changed_fields: field_changes
        },
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
    };

    Ok(entry)
}

/// 🎯 便捷函数别名：track_auto 的简短版本
#[inline]
pub fn track_changes<T, F>(entity: &mut T, updater: F) -> Result<ChangeLogEntry, Box<dyn std::error::Error>>
where
    T: Clone + Diff + 'static,
    F: FnOnce(&mut T)
{
    track_auto(entity, updater)
}
