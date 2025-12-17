use std::{borrow::Cow, fmt::Debug};

// ============================================================================
// 错误类型
// ============================================================================

/// 实体操作错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum EntityError {
    /// 序列化失败
    SerializationError(String),
    /// 反序列化失败
    DeserializationError(String),
    /// 实体ID不匹配
    EntityIdMismatch { expected: String, actual: String },
    /// 实体类型不匹配
    EntityTypeMismatch { expected: String, actual: String },
    /// 字段解析失败
    FieldParseError { field: String, reason: String },
    /// 无变更检测到
    NoChangesDetected,
    /// 无法在已删除的实体上回放
    CannotReplayOnDeleted,
    /// 自定义错误
    Custom(String)
}

impl std::fmt::Display for EntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            EntityError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            EntityError::EntityIdMismatch {
                expected,
                actual
            } => {
                write!(f, "Entity ID mismatch: expected {}, got {}", expected, actual)
            }
            EntityError::EntityTypeMismatch {
                expected,
                actual
            } => {
                write!(f, "Entity type mismatch: expected {}, got {}", expected, actual)
            }
            EntityError::FieldParseError {
                field,
                reason
            } => {
                write!(f, "Failed to parse field '{}': {}", field, reason)
            }
            EntityError::NoChangesDetected => write!(f, "No changes detected"),
            EntityError::CannotReplayOnDeleted => write!(f, "Cannot replay on deleted entity"),
            EntityError::Custom(msg) => write!(f, "{}", msg)
        }
    }
}

impl std::error::Error for EntityError {}

// 便于从 String 转换为 EntityError
impl From<String> for EntityError {
    fn from(s: String) -> Self { EntityError::Custom(s) }
}

impl From<&str> for EntityError {
    fn from(s: &str) -> Self { EntityError::Custom(s.to_string()) }
}

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

/// 字段变更记录（零拷贝优化）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldChange {
    /// 字段名称（通常为静态字符串）
    pub field_name: Cow<'static, str>,
    /// 旧值（序列化为字符串）
    pub old_value: String,
    /// 新值（序列化为字符串）
    pub new_value: String
}

impl FieldChange {
    /// 创建字段变更记录
    #[inline]
    pub fn new(
        field_name: impl Into<Cow<'static, str>>, old_value: impl Into<String>, new_value: impl Into<String>
    ) -> Self {
        Self {
            field_name: field_name.into(),
            old_value: old_value.into(),
            new_value: new_value.into()
        }
    }

    /// 创建字段变更记录（静态字段名，零分配）
    #[inline]
    pub const fn new_static(field_name: &'static str, old_value: String, new_value: String) -> Self {
        Self {
            field_name: Cow::Borrowed(field_name),
            old_value,
            new_value
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
// 时间戳和序列号提供者
// ============================================================================

/// 时间戳提供者 trait
pub trait TimestampProvider: Send + Sync {
    /// 获取当前时间戳（纳秒）
    fn now(&self) -> u64;
}

/// 默认时间戳提供者（使用系统时间）
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemTimestampProvider;

impl TimestampProvider for SystemTimestampProvider {
    #[inline]
    fn now(&self) -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
    }
}

/// 缓存时间戳提供者（低延迟优化，适用于高频调用场景）
///
/// 使用线程本地缓存减少系统调用开销
#[derive(Debug, Default)]
pub struct CachedTimestampProvider {
    cache_nanos: std::sync::atomic::AtomicU64,
    last_update: std::sync::atomic::AtomicU64
}

impl CachedTimestampProvider {
    /// 创建缓存时间戳提供者
    pub fn new() -> Self {
        Self {
            cache_nanos: std::sync::atomic::AtomicU64::new(0),
            last_update: std::sync::atomic::AtomicU64::new(0)
        }
    }

    /// 缓存有效期（纳秒），默认100微秒
    const CACHE_DURATION_NANOS: u64 = 100_000;
}

impl TimestampProvider for CachedTimestampProvider {
    #[inline]
    fn now(&self) -> u64 {
        let current = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;

        let last_update = self.last_update.load(std::sync::atomic::Ordering::Relaxed);

        // 如果缓存仍然有效，直接返回缓存值
        if current - last_update < Self::CACHE_DURATION_NANOS {
            return self.cache_nanos.load(std::sync::atomic::Ordering::Relaxed);
        }

        // 更新缓存
        self.cache_nanos.store(current, std::sync::atomic::Ordering::Release);
        self.last_update.store(current, std::sync::atomic::Ordering::Release);

        current
    }
}

/// 序列号生成器 trait
pub trait SequenceGenerator: Send + Sync {
    /// 生成下一个序列号
    fn next(&self) -> u64;
}

/// 默认序列号生成器（返回0，需要外部管理）
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultSequenceGenerator;

impl SequenceGenerator for DefaultSequenceGenerator {
    fn next(&self) -> u64 {
        0 // 调用者需要自行管理序列号
    }
}

/// 原子递增序列号生成器（线程安全）
#[derive(Debug, Default)]
pub struct AtomicSequenceGenerator {
    counter: std::sync::atomic::AtomicU64
}

impl AtomicSequenceGenerator {
    /// 创建新的原子序列号生成器
    #[inline]
    pub fn new() -> Self {
        Self {
            counter: std::sync::atomic::AtomicU64::new(0)
        }
    }

    /// 从指定值开始
    #[inline]
    pub fn with_start(start: u64) -> Self {
        Self {
            counter: std::sync::atomic::AtomicU64::new(start)
        }
    }

    /// 批量生成序列号（低延迟优化）
    #[inline]
    pub fn next_batch(&self, count: u64) -> u64 { self.counter.fetch_add(count, std::sync::atomic::Ordering::Relaxed) }
}

impl SequenceGenerator for AtomicSequenceGenerator {
    #[inline]
    fn next(&self) -> u64 { self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) }
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
pub trait Entity: Clone + Debug + Send + Sync + 'static {
    /// 实体ID类型
    type Id: Debug + Clone + PartialEq + ToString;

    /// 获取实体唯一标识符
    fn entity_id(&self) -> Self::Id;

    /// 获取实体类型名称
    fn entity_type() -> &'static str
    where
        Self: Sized;


    // ============================================================================
    // Auto Tracking Methods
    // ============================================================================

    /// 自动追踪创建操作
    ///
    /// # Example
    /// ```ignore
    /// let order = Order::new(1, "BTCUSDT", 50000.0);
    /// let entry = order.track_create().unwrap();
    /// ```
    fn track_create(&self) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized
    {
        track(self, Operation::Create)
    }

    /// 自动追踪删除操作
    ///
    /// # Example
    /// ```ignore
    /// let entry = order.track_delete().unwrap();
    /// ```
    fn track_delete(&self) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized
    {
        track(self, Operation::Delete)
    }

    /// 自动追踪更新操作（带 diff）
    ///
    /// # Example
    /// ```ignore
    /// let mut order = Order::new(1, "BTCUSDT", 50000.0);
    /// let entry = order.track_update(|o| {
    ///     o.price = 51000.0;
    ///     o.quantity = 2.0;
    /// }).unwrap();
    /// ```
    fn track_update<F>(&mut self, updater: F) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized,
        F: FnOnce(&mut Self)
    {
        track_update(self, updater)
    }

    /// 手动追踪更新操作（已有 old 和 new 状态）
    ///
    /// # Example
    /// ```ignore
    /// let old_order = order.clone();
    /// order.price = 51000.0;
    /// let entry = order.track_update_from(&old_order).unwrap();
    /// ```
    fn track_update_from(&self, old_state: &Self) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized
    {
        let field_changes = old_state.diff(self);

        if field_changes.is_empty() {
            return Err(EntityError::NoChangesDetected);
        }

        Ok(ChangeLogEntry::new(
            self.entity_id().to_string(),
            Self::entity_type(),
            ChangeType::Updated {
                changed_fields: field_changes
            },
            current_timestamp(),
            0
        ))
    }

    /// 自动追踪创建操作（带自定义提供者）
    ///
    /// # Example
    /// ```ignore
    /// let ts_provider = SystemTimestampProvider;
    /// let seq_gen = AtomicSequenceGenerator::new();
    /// let entry = order.track_create_with(&ts_provider, &seq_gen).unwrap();
    /// ```
    fn track_create_with(
        &self, ts_provider: &impl TimestampProvider, seq_gen: &impl SequenceGenerator
    ) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized
    {
        track_with(self, Operation::Create, ts_provider, seq_gen)
    }

    /// 自动追踪删除操作（带自定义提供者）
    fn track_delete_with(
        &self, ts_provider: &impl TimestampProvider, seq_gen: &impl SequenceGenerator
    ) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized
    {
        track_with(self, Operation::Delete, ts_provider, seq_gen)
    }

    /// 自动追踪更新操作（带自定义提供者）
    fn track_update_with<F>(
        &mut self, updater: F, ts_provider: &impl TimestampProvider, seq_gen: &impl SequenceGenerator
    ) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized,
        F: FnOnce(&mut Self)
    {
        track_update_with(self, updater, ts_provider, seq_gen)
    }

    // ============================================================================
    // Diff Methods
    // ============================================================================

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

    // ============================================================================
    // Replay Methods
    // ============================================================================

    /// 从变更日志条目回放数据，更新 self 的字段
    ///
    /// # 参数
    /// - `entry`: 变更日志条目
    ///
    /// # 返回
    /// - `Ok(())`: 回放成功
    /// - `Err(EntityError)`: 回放失败
    ///
    /// # 错误
    /// - EntityError::EntityIdMismatch: 实体ID不匹配
    /// - EntityError::EntityTypeMismatch: 实体类型不匹配
    /// - EntityError::FieldParseError: 字段解析失败
    /// - EntityError::CannotReplayOnDeleted: 已删除的实体无法回放
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), EntityError>;

    /// 检查是否可以应用此变更日志
    fn can_replay(&self, entry: &ChangeLogEntry) -> bool {
        self.entity_id().to_string() == entry.entity_id && Self::entity_type() == entry.entity_type
    }
}


// ============================================================================
// 辅助类型
// ============================================================================

/// 变更追踪结果
#[derive(Debug, Clone, PartialEq)]
pub enum TrackingResult {
    /// 无变更
    NoChange,
    /// 有变更（包含变更日志）
    Changed(ChangeLogEntry),
    /// 实体创建
    Created(ChangeLogEntry),
    /// 实体删除
    Deleted(ChangeLogEntry)
}

impl TrackingResult {
    /// 检查是否有变更
    pub fn has_change(&self) -> bool { !matches!(self, TrackingResult::NoChange) }

    /// 获取变更日志条目
    pub fn entry(&self) -> Option<&ChangeLogEntry> {
        match self {
            TrackingResult::NoChange => None,
            TrackingResult::Changed(entry) | TrackingResult::Created(entry) | TrackingResult::Deleted(entry) => {
                Some(entry)
            }
        }
    }
}

// ============================================================================
// 统一追踪接口
// ============================================================================

/// 操作类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// 创建操作
    Create,
    /// 删除操作
    Delete
}

/// 统一的变更追踪接口
///
/// 用于追踪 Create 和 Delete 操作（不需要修改实体）
/// Update 操作请使用 `track_update` 函数
///
/// # 示例
/// ```ignore
/// // 创建
/// let entry = track(&order, Operation::Create).unwrap();
///
/// // 删除
/// let entry = track(&order, Operation::Delete).unwrap();
/// ```
#[inline]
pub fn track<T>(entity: &T, operation: Operation) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + 'static
{
    track_with(entity, operation, &SystemTimestampProvider, &DefaultSequenceGenerator)
}

/// 统一的变更追踪接口（带自定义提供者）
#[inline]
pub fn track_with<T>(
    entity: &T, operation: Operation, ts_provider: &impl TimestampProvider, seq_gen: &impl SequenceGenerator
) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + 'static
{
    let change_type = match operation {
        Operation::Create => ChangeType::Created,
        Operation::Delete => ChangeType::Deleted
    };

    let entry = ChangeLogEntry::new(
        entity.entity_id().to_string(),
        T::entity_type(),
        change_type,
        ts_provider.now(),
        seq_gen.next()
    );

    Ok(entry)
}

/// 批量追踪实体操作（低延迟优化）
///
/// # 示例
/// ```ignore
/// let orders = vec![order1, order2, order3];
/// let entries = track_batch(&orders, Operation::Create).unwrap();
/// ```
#[inline]
pub fn track_batch<T>(entities: &[T], operation: Operation) -> Result<Vec<ChangeLogEntry>, EntityError>
where
    T: Entity + 'static
{
    track_batch_with(entities, operation, &SystemTimestampProvider, &AtomicSequenceGenerator::new())
}

/// 批量追踪实体操作（带自定义提供者）
pub fn track_batch_with<T>(
    entities: &[T], operation: Operation, ts_provider: &impl TimestampProvider, seq_gen: &impl SequenceGenerator
) -> Result<Vec<ChangeLogEntry>, EntityError>
where
    T: Entity + 'static
{
    let change_type = match operation {
        Operation::Create => ChangeType::Created,
        Operation::Delete => ChangeType::Deleted
    };

    let timestamp = ts_provider.now();
    let mut entries = Vec::with_capacity(entities.len());

    for entity in entities {
        entries.push(ChangeLogEntry::new(
            entity.entity_id().to_string(),
            T::entity_type(),
            change_type.clone(),
            timestamp,
            seq_gen.next()
        ));
    }

    Ok(entries)
}

/// 追踪实体更新操作（带自动 diff）
///
/// # 示例
/// ```ignore
/// let entry = track_update(&mut order, |o| {
///     o.price = 51000.0;
///     o.status = "confirmed".to_string();
/// }).unwrap();
/// ```
#[inline]
pub fn track_update<T, F>(entity: &mut T, updater: F) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + Clone + 'static,
    F: FnOnce(&mut T)
{
    track_update_with(entity, updater, &SystemTimestampProvider, &DefaultSequenceGenerator)
}

/// 追踪实体更新操作（带自定义提供者）
#[inline]
pub fn track_update_with<T, F>(
    entity: &mut T, updater: F, ts_provider: &impl TimestampProvider, seq_gen: &impl SequenceGenerator
) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + Clone + 'static,
    F: FnOnce(&mut T)
{
    // 1. 克隆旧状态
    let old_entity = entity.clone();

    // 2. 执行更新
    updater(entity);

    // 3. 自动 diff 检测变更
    let field_changes = old_entity.diff(entity);

    if field_changes.is_empty() {
        return Err(EntityError::NoChangesDetected);
    }

    let entry = ChangeLogEntry::new(
        entity.entity_id().to_string(),
        T::entity_type(),
        ChangeType::Updated {
            changed_fields: field_changes
        },
        ts_provider.now(),
        seq_gen.next()
    );

    Ok(entry)
}

// ============================================================================
// 便捷别名函数
// ============================================================================

/// 追踪实体创建操作（便捷别名）
///
/// # 示例
/// ```ignore
/// let order = Order::new(1, "BTCUSDT", 50000.0);
/// let entry = track_create(&order).unwrap();
/// ```
#[inline]
pub fn track_create<T>(entity: &T) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + 'static
{
    track(entity, Operation::Create)
}

/// 追踪实体删除操作（便捷别名）
///
/// # 示例
/// ```ignore
/// let order = Order::new(1, "BTCUSDT", 50000.0);
/// let entry = track_delete(&order).unwrap();
/// ```
#[inline]
pub fn track_delete<T>(entity: &T) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + 'static
{
    track(entity, Operation::Delete)
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


        fn diff(&self, other: &Self) -> Vec<FieldChange> {
            let mut changes = Vec::new();
            if self.value != other.value {
                changes.push(FieldChange::new("value", &self.value, &other.value));
            }
            changes
        }

        fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), EntityError> {
            if !self.can_replay(entry) {
                return Err(EntityError::EntityIdMismatch {
                    expected: self.entity_id().to_string(),
                    actual: entry.entity_id.clone()
                });
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
                ChangeType::Deleted => Err(EntityError::CannotReplayOnDeleted),
                ChangeType::Created => Ok(())
            }
        }
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
    fn test_auto_track_create() {
        let entity = TestEntity {
            id: 1,
            value: "test".to_string()
        };

        let entry = entity.track_create().unwrap();
        assert_eq!(entry.entity_id, "1");
        assert_eq!(entry.entity_type, "TestEntity");
        assert_eq!(entry.change_type, ChangeType::Created);
    }

    #[test]
    fn test_auto_track_delete() {
        let entity = TestEntity {
            id: 1,
            value: "test".to_string()
        };

        let entry = entity.track_delete().unwrap();
        assert_eq!(entry.entity_id, "1");
        assert_eq!(entry.entity_type, "TestEntity");
        assert_eq!(entry.change_type, ChangeType::Deleted);
    }

    #[test]
    fn test_auto_track_update() {
        let mut entity = TestEntity {
            id: 1,
            value: "old".to_string()
        };

        let entry = entity
            .track_update(|e| {
                e.value = "new".to_string();
            })
            .unwrap();

        assert_eq!(entry.entity_id, "1");
        assert_eq!(entry.entity_type, "TestEntity");

        match &entry.change_type {
            ChangeType::Updated {
                changed_fields
            } => {
                assert_eq!(changed_fields.len(), 1);
                assert_eq!(changed_fields[0].field_name, "value");
                // Debug formatting adds quotes for String types
                assert!(changed_fields[0].old_value.contains("old"));
                assert!(changed_fields[0].new_value.contains("new"));
            }
            _ => panic!("Expected Updated change type")
        }

        // 确认实体已更新
        assert_eq!(entity.value, "new");
    }

    #[test]
    fn test_auto_track_update_from() {
        let old_entity = TestEntity {
            id: 1,
            value: "old".to_string()
        };

        let new_entity = TestEntity {
            id: 1,
            value: "new".to_string()
        };

        let entry = new_entity.track_update_from(&old_entity).unwrap();

        assert_eq!(entry.entity_id, "1");
        match &entry.change_type {
            ChangeType::Updated {
                changed_fields
            } => {
                assert_eq!(changed_fields.len(), 1);
                assert_eq!(changed_fields[0].field_name, "value");
            }
            _ => panic!("Expected Updated change type")
        }
    }

    #[test]
    fn test_auto_track_no_changes() {
        let old_entity = TestEntity {
            id: 1,
            value: "same".to_string()
        };

        let new_entity = TestEntity {
            id: 1,
            value: "same".to_string()
        };

        let result = new_entity.track_update_from(&old_entity);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EntityError::NoChangesDetected);
    }
}
