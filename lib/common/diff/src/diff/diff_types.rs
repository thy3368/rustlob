use std::fmt::Debug;

// ============================================================================
// 核心枚举类型
// ============================================================================

/// 变更类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChangeType {
    /// 实体创建
    Created,
    /// 实体更新（包含变更字段）
    Updated { changed_fields: Vec<FieldChange> },
    /// 实体删除
    Deleted
}

/// 字段变更记录
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldChange {
    /// 字段名称
    pub field_name: String,
    /// 旧值（序列化为字符串）
    pub old_value: String,
    /// 新值（序列化为字符串）
    pub new_value: String
}

impl FieldChange {
    /// 创建字段变更记录
    pub fn new(field_name: impl Into<String>, old_value: impl Into<String>, new_value: impl Into<String>) -> Self {
        Self {
            field_name: field_name.into(),
            old_value: old_value.into(),
            new_value: new_value.into()
        }
    }
}

/// 变更日志条目
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeLogEntry {
    /// 实体唯一标识符
    pub entity_id: String,
    /// 实体类型名称
    pub entity_type: String,
    /// 变更类型
    pub change_type: ChangeType,
    /// 变更时间戳（纳秒）
    pub timestamp: u64,
    /// 变更序列号（用于排序）
    pub sequence: u64
}

impl ChangeLogEntry {
    /// 创建变更日志条目
    pub fn new(
        entity_id: impl Into<String>, entity_type: impl Into<String>, change_type: ChangeType, timestamp: u64,
        sequence: u64
    ) -> Self {
        Self {
            entity_id: entity_id.into(),
            entity_type: entity_type.into(),
            change_type,
            timestamp,
            sequence
        }
    }
}

// ============================================================================
// 实体快照
// ============================================================================

/// 实体快照
///
/// 用于保存实体在某个时间点的完整状态
#[derive(Debug, Clone, PartialEq)]
pub struct EntitySnapshot {
    /// 实体ID
    pub entity_id: String,
    /// 实体类型
    pub entity_type: String,
    /// 快照时间戳
    pub timestamp: u64,
    /// 快照序列号
    pub sequence: u64,
    /// 序列化后的实体数据
    pub data: Vec<u8>
}

impl EntitySnapshot {
    /// 创建实体快照
    pub fn new(
        entity_id: impl Into<String>, entity_type: impl Into<String>, timestamp: u64, sequence: u64, data: Vec<u8>
    ) -> Self {
        Self {
            entity_id: entity_id.into(),
            entity_type: entity_type.into(),
            timestamp,
            sequence,
            data
        }
    }
}

// ============================================================================
// 核心 Trait 定义
// ============================================================================

/// TrackableEntity - 可追踪实体核心接口
///
/// 所有需要进行变更追踪的实体都必须实现此 trait
///
/// # 职责
/// - 提供实体唯一标识
/// - 提供实体类型信息
/// - 支持序列化/反序列化
///
/// # 示例
/// ```ignore
/// #[derive(Debug, Clone)]
/// struct Order {
///     id: u64,
///     symbol: String,
///     price: f64,
/// }
///
/// impl TrackableEntity for Order {
///     type Id = u64;
///
///     fn entity_id(&self) -> Self::Id {
///         self.id
///     }
///
///     fn entity_type() -> &'static str {
///         "Order"
///     }
///
///     fn to_bytes(&self) -> Result<Vec<u8>, String> {
///         bincode::serialize(self)
///             .map_err(|e| e.to_string())
///     }
///
///     fn from_bytes(data: &[u8]) -> Result<Self, String> {
///         bincode::deserialize(data)
///             .map_err(|e| e.to_string())
///     }
/// }
/// ```
pub trait Entity: Clone + Debug + Send + Sync {
    /// 实体ID类型
    type Id: Debug + Clone + PartialEq + ToString;

    /// 获取实体唯一标识符
    fn entity_id(&self) -> Self::Id;

    /// 获取实体类型名称
    fn entity_type() -> &'static str
    where
        Self: Sized;

    /// 序列化实体为字节流（用于快照）
    ///
    /// # 错误
    /// 序列化失败时返回错误信息
    fn to_bytes(&self) -> Result<Vec<u8>, String>;

    /// 从字节流反序列化实体（用于快照恢复）
    ///
    /// # 错误
    /// 反序列化失败时返回错误信息
    fn from_bytes(data: &[u8]) -> Result<Self, String>
    where
        Self: Sized;

    /// 创建实体快照
    fn create_snapshot(&self, timestamp: u64, sequence: u64) -> Result<EntitySnapshot, String> {
        let data = self.to_bytes()?;
        Ok(EntitySnapshot::new(self.entity_id().to_string(), Self::entity_type(), timestamp, sequence, data))
    }

    /// 从快照恢复实体
    fn from_snapshot(snapshot: &EntitySnapshot) -> Result<Self, String>
    where
        Self: Sized
    {
        Self::from_bytes(&snapshot.data)
    }
}

/// Diff - 状态差异检测接口
///
/// 用于自动检测实体字段的变更
///
/// # 职责
/// - 比较两个实体状态
/// - 生成字段变更列表
///
/// # 示例
/// ```ignore
/// impl Diff for Order {
///     fn diff(&self, other: &Self) -> Vec<FieldChange> {
///         let mut changes = Vec::new();
///
///         if self.symbol != other.symbol {
///             changes.push(FieldChange::new(
///                 "symbol",
///                 &self.symbol,
///                 &other.symbol,
///             ));
///         }
///
///         if self.price != other.price {
///             changes.push(FieldChange::new(
///                 "price",
///                 self.price.to_string(),
///                 other.price.to_string(),
///             ));
///         }
///
///         changes
///     }
/// }
/// ```
pub trait Diff {
    /// 比较 self(旧状态) 和 other(新状态)，返回字段变更列表
    ///
    /// # 参数
    /// - `other`: 新状态实体
    ///
    /// # 返回
    /// 字段变更列表，如果没有变更则返回空列表
    fn diff(&self, other: &Self) -> Vec<FieldChange>;

    /// 检查是否有变更
    fn has_changes(&self, other: &Self) -> bool { !self.diff(other).is_empty() }
}

/// Replay - 状态回放接口
///
/// 用于从变更日志重建实体状态
///
/// # 职责
/// - 应用变更日志到实体
/// - 支持增量状态更新
///
/// # 示例
/// ```ignore
/// impl Replay for Order {
///     fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
///         match &entry.change_type {
///             ChangeType::Created => {
///                 // 实体已创建，无需额外操作
///                 Ok(())
///             }
///             ChangeType::Updated { changed_fields } => {
///                 for field in changed_fields {
///                     match field.field_name.as_str() {
///                         "symbol" => {
///                             self.symbol = field.new_value.clone();
///                         }
///                         "price" => {
///                             self.price = field.new_value.parse()
///                                 .map_err(|e| format!("Failed to parse price: {}", e))?;
///                         }
///                         _ => {}
///                     }
///                 }
///                 Ok(())
///             }
///             ChangeType::Deleted => {
///                 Err("Cannot replay on deleted entity".to_string())
///             }
///         }
///     }
///
///     fn can_replay(&self, entry: &ChangeLogEntry) -> bool {
///         self.entity_id().to_string() == entry.entity_id
///             && Self::entity_type() == entry.entity_type
///     }
/// }
/// ```
pub trait Replayable: Trackable {
    /// 从变更日志条目回放数据，更新 self 的字段
    ///
    /// # 参数
    /// - `entry`: 变更日志条目
    ///
    /// # 返回
    /// - `Ok(())`: 回放成功
    /// - `Err(String)`: 回放失败，包含错误信息
    ///
    /// # 错误
    /// - 实体ID不匹配
    /// - 实体类型不匹配
    /// - 字段解析失败
    /// - 已删除的实体无法回放
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String>;

    /// 检查是否可以应用此变更日志
    fn can_replay(&self, entry: &ChangeLogEntry) -> bool {
        self.entity_id().to_string() == entry.entity_id && Self::entity_type() == entry.entity_type
    }
}

/// Trackable - 完整的可追踪实体
///
/// 组合 trait，要求实体同时实现：
/// - `TrackableEntity`: 基础实体能力
/// - `Diff`: 状态差异检测
/// - `Replay`: 状态回放
///
/// # 职责
/// 提供完整的实体追踪和回放能力
///
/// # 示例
/// ```ignore
/// // 只需实现三个基础 trait
/// impl TrackableEntity for Order { /* ... */ }
/// impl Diff for Order { /* ... */ }
/// impl Replay for Order { /* ... */ }
///
/// // 自动获得 Trackable 能力
/// fn process_trackable<T: Trackable>(entity: &T) {
///     // 可以使用所有追踪功能
/// }
/// ```
pub trait Trackable: Entity + Diff {}

// 为所有实现了 TrackableEntity + Diff + Replay 的类型自动实现 Trackable
impl<T> Trackable for T where T: Entity + Diff {}

// ============================================================================
// 辅助类型
// ============================================================================


/// 使用 Diff trait 自动追踪变更
///
/// # 示例
/// ```ignore
/// let mut order = Order::new(1, "pending");
/// let entry = track_auto(&mut order, |o| {
///     o.value = 200;
///     o.status = "confirmed".to_string();
/// }).unwrap();
/// ```
pub fn track_auto<T, F>(entity: &mut T, updater: F) -> Result<ChangeLogEntry, String>
where
    T: Entity + Diff + Clone + 'static,
    F: FnOnce(&mut T)
{
    // 1. 克隆旧状态
    let old_entity = entity.clone();

    // 2. 执行更新
    updater(entity);

    // 3. 自动 diff 检测变更
    let field_changes = old_entity.diff(entity);

    if field_changes.is_empty() {
        return Err("No changes detected".to_string());
    }

    let entry = ChangeLogEntry::new(
        entity.entity_id().to_string(),
        T::entity_type(),
        ChangeType::Updated {
            changed_fields: field_changes
        },
        current_timestamp(),
        0 // 序列号由调用者管理
    );

    Ok(entry)
}

/// track_auto 的简短别名
#[inline]
pub fn track_changes<T, F>(entity: &mut T, updater: F) -> Result<ChangeLogEntry, String>
where
    T: Entity + Diff + Clone + 'static,
    F: FnOnce(&mut T)
{
    track_auto(entity, updater)
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 获取当前时间戳（纳秒）
fn current_timestamp() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestEntity {
        id: u64,
        value: String
    }

    impl Entity for TestEntity {
        type Id = u64;

        fn entity_id(&self) -> Self::Id { self.id }

        fn entity_type() -> &'static str { "TestEntity" }

        fn to_bytes(&self) -> Result<Vec<u8>, String> { Ok(format!("{}:{}", self.id, self.value).into_bytes()) }

        fn from_bytes(data: &[u8]) -> Result<Self, String> {
            let s = String::from_utf8(data.to_vec()).map_err(|e| e.to_string())?;
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() != 2 {
                return Err("Invalid format".to_string());
            }
            Ok(Self {
                id: parts[0].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
                value: parts[1].to_string()
            })
        }
    }

    impl Diff for TestEntity {
        fn diff(&self, other: &Self) -> Vec<FieldChange> {
            let mut changes = Vec::new();
            if self.value != other.value {
                changes.push(FieldChange::new("value", &self.value, &other.value));
            }
            changes
        }
    }

    impl Replayable for TestEntity {
        fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
            if !self.can_replay(entry) {
                return Err("Cannot replay: entity mismatch".to_string());
            }

            match &entry.change_type {
                ChangeType::Updated {
                    changed_fields
                } => {
                    for field in changed_fields {
                        if field.field_name == "value" {
                            self.value = field.new_value.clone();
                        }
                    }
                    Ok(())
                }
                ChangeType::Deleted => Err("Cannot replay on deleted entity".to_string()),
                ChangeType::Created => Ok(())
            }
        }
    }


    #[test]
    fn test_snapshot() {
        let entity = TestEntity {
            id: 1,
            value: "test".to_string()
        };

        let snapshot = entity.create_snapshot(1000, 1).unwrap();
        assert_eq!(snapshot.entity_id, "1");
        assert_eq!(snapshot.entity_type, "TestEntity");

        let restored = TestEntity::from_snapshot(&snapshot).unwrap();
        assert_eq!(restored, entity);
    }

    #[test]
    fn test_replay() {
        let mut entity = TestEntity {
            id: 1,
            value: "old".to_string()
        };

        let entry = ChangeLogEntry::new(
            "1",
            "TestEntity",
            ChangeType::Updated {
                changed_fields: vec![FieldChange::new("value", "old", "new")]
            },
            1000,
            1
        );

        entity.replay(&entry).unwrap();
        assert_eq!(entity.value, "new");
    }


    #[test]
    fn test_track_auto() {
        let mut entity = TestEntity {
            id: 1,
            value: "old".to_string()
        };


        let mut entity2 = entity.clone();

        let entry = track_auto(&mut entity, |e| {
            e.value = "new".to_string();
        })
        .unwrap();

        entity2.replay(&entry).unwrap();

        assert_eq!(entity2.value, "new");


        assert_eq!(entity.value, "new");
        assert_eq!(entry.entity_id, "1");
        assert_eq!(entry.entity_type, "TestEntity");

        if let ChangeType::Updated {
            changed_fields
        } = entry.change_type
        {
            assert_eq!(changed_fields.len(), 1);
            assert_eq!(changed_fields[0].field_name, "value");
        } else {
            panic!("Expected Updated change type");
        }
    }


    #[test]
    fn test_track_auto_with_no_changes() {
        let mut entity = TestEntity {
            id: 1,
            value: "old".to_string()
        };

        // 没有任何变更
        let result = track_auto(&mut entity, |_e| {
            // 不做任何修改
        });

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No changes detected");
    }
}
