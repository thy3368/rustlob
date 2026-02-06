use std::borrow::Cow;
use std::fmt::Debug;

use immutable_derive::immutable;
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
    Custom(String),
}

impl std::fmt::Display for EntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            EntityError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            EntityError::EntityIdMismatch { expected, actual } => {
                write!(f, "Entity ID mismatch: expected {}, got {}", expected, actual)
            }
            EntityError::EntityTypeMismatch { expected, actual } => {
                write!(f, "Entity type mismatch: expected {}, got {}", expected, actual)
            }
            EntityError::FieldParseError { field, reason } => {
                write!(f, "Failed to parse field '{}': {}", field, reason)
            }
            EntityError::NoChangesDetected => write!(f, "No changes detected"),
            EntityError::CannotReplayOnDeleted => write!(f, "Cannot replay on deleted entity"),
            EntityError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for EntityError {}

// 便于从 String 转换为 EntityError
impl From<String> for EntityError {
    fn from(s: String) -> Self {
        EntityError::Custom(s)
    }
}

impl From<&str> for EntityError {
    fn from(s: &str) -> Self {
        EntityError::Custom(s.to_string())
    }
}

// ============================================================================
// 核心枚举类型
// ============================================================================

/// 变更类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ChangeType {
    /// 实体创建（包含初始字段）
    Created { fields: Vec<FieldChange> },
    /// 实体更新（包含变更字段）
    Updated { changed_fields: Vec<FieldChange> },
    /// 实体删除
    Deleted,
}

/// 字段变更记录（零拷贝优化）
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FieldChange {
    /// 字段名称（通常为静态字符串）
    pub field_name: Cow<'static, str>,
    /// 旧值（序列化为字符串）
    pub old_value: String,
    /// 新值（序列化为字符串）
    pub new_value: String,
}

impl FieldChange {
    /// 创建字段变更记录
    #[inline]
    pub fn new(
        field_name: impl Into<Cow<'static, str>>,
        old_value: impl Into<String>,
        new_value: impl Into<String>,
    ) -> Self {
        Self {
            field_name: field_name.into(),
            old_value: old_value.into(),
            new_value: new_value.into(),
        }
    }

    /// 创建字段变更记录（静态字段名，零分配）
    #[inline]
    pub const fn new_static(
        field_name: &'static str,
        old_value: String,
        new_value: String,
    ) -> Self {
        Self { field_name: Cow::Borrowed(field_name), old_value, new_value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldSchema {
    /// 实体唯一标识符
    pub field_name: String,
    /// 实体类型名称
    pub field_type: String,
    /// 变更类型
    pub default_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableSchema {
    /// 实体唯一标识符
    pub table_name: String,
    pub fields: Vec<FieldSchema>,
}

impl TableSchema {
    /// 创建新的表结构定义
    #[inline]
    pub fn new(table_name: impl Into<String>) -> Self {
        Self { table_name: table_name.into(), fields: Vec::new() }
    }

    /// 查找指定名称的字段
    #[inline]
    pub fn find_field(&self, name: &str) -> Option<&FieldSchema> {
        self.fields.iter().find(|f| f.field_name == name)
    }

    /// 查找并修改指定名称的字段（可变引用）
    #[inline]
    pub fn find_field_mut(&mut self, name: &str) -> Option<&mut FieldSchema> {
        self.fields.iter_mut().find(|f| f.field_name == name)
    }

    /// 添加字段到表结构
    #[inline]
    pub fn add_field(&mut self, field: FieldSchema) -> &mut Self {
        if !self.fields.iter().any(|f| f.field_name == field.field_name) {
            self.fields.push(field);
        }
        self
    }

    /// 批量添加字段
    #[inline]
    pub fn add_fields(&mut self, fields: Vec<FieldSchema>) -> &mut Self {
        for field in fields {
            self.add_field(field);
        }
        self
    }

    /// 移除指定名称的字段
    #[inline]
    pub fn remove_field(&mut self, name: &str) -> Option<FieldSchema> {
        self.fields.iter().position(|f| f.field_name == name).map(|i| self.fields.remove(i))
    }

    /// 获取字段数量
    #[inline]
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// 检查是否包含指定字段
    #[inline]
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.iter().any(|f| f.field_name == name)
    }

    /// 获取所有字段名称
    #[inline]
    pub fn field_names(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.field_name.as_str()).collect()
    }

    /// 验证表结构的完整性
    ///
    /// 检查：
    /// 1. 表名不为空
    /// 2. 至少有一个字段
    /// 3. 字段名称唯一
    /// 4. 字段名称不为空
    #[inline]
    pub fn validate(&self) -> Result<(), String> {
        if self.table_name.is_empty() {
            return Err("Table name cannot be empty".to_string());
        }

        if self.fields.is_empty() {
            return Err(format!("Table '{}' must have at least one field", self.table_name));
        }

        // 检查字段名称唯一性
        let mut seen = std::collections::HashSet::new();
        for field in &self.fields {
            if field.field_name.is_empty() {
                return Err("Field name cannot be empty".to_string());
            }
            if !seen.insert(&field.field_name) {
                return Err(format!("Duplicate field name: '{}'", field.field_name));
            }
        }

        Ok(())
    }

    /// 清空所有字段
    #[inline]
    pub fn clear(&mut self) {
        self.fields.clear();
    }

    /// 获取表结构的摘要信息
    #[inline]
    pub fn summary(&self) -> String {
        let field_list = self
            .fields
            .iter()
            .map(|f| format!("{}({})", f.field_name, f.field_type))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Table '{}' with {} fields: [{}]", self.table_name, self.fields.len(), field_list)
    }
}

impl Default for TableSchema {
    fn default() -> Self {
        Self { table_name: String::new(), fields: Vec::new() }
    }
}

impl std::fmt::Display for TableSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// 变更日志条目
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[immutable]
pub struct ChangeLogEntry {
    /// 实体唯一标识符
    entity_id: String,
    /// 实体类型名称
    entity_type: String,
    /// 变更类型
    change_type: ChangeType,
    /// 变更时间戳（纳秒）
    timestamp: u64,
    /// 变更序列号（用于排序）
    sequence: u64,
}

impl Entity for ChangeLogEntry {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        todo!()
    }

    fn entity_type() -> &'static str
    where
        Self: Sized,
    {
        todo!()
    }

    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        todo!()
    }

    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), EntityError> {
        todo!()
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
    pub data: Vec<u8>,
}

impl EntitySnapshot {
    /// 创建实体快照
    pub fn new(
        entity_id: impl Into<String>,
        entity_type: impl Into<String>,
        timestamp: u64,
        sequence: u64,
        data: Vec<u8>,
    ) -> Self {
        Self {
            entity_id: entity_id.into(),
            entity_type: entity_type.into(),
            timestamp,
            sequence,
            data,
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
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
            as u64
    }
}

/// 缓存时间戳提供者（低延迟优化，适用于高频调用场景）
///
/// 使用线程本地缓存减少系统调用开销
#[derive(Debug, Default)]
pub struct CachedTimestampProvider {
    cache_nanos: std::sync::atomic::AtomicU64,
    last_update: std::sync::atomic::AtomicU64,
}

impl CachedTimestampProvider {
    /// 创建缓存时间戳提供者
    pub fn new() -> Self {
        Self {
            cache_nanos: std::sync::atomic::AtomicU64::new(0),
            last_update: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// 缓存有效期（纳秒），默认100微秒
    const CACHE_DURATION_NANOS: u64 = 100_000;
}

impl TimestampProvider for CachedTimestampProvider {
    #[inline]
    fn now(&self) -> u64 {
        let current =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
                as u64;

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
    counter: std::sync::atomic::AtomicU64,
}

impl AtomicSequenceGenerator {
    /// 创建新的原子序列号生成器
    #[inline]
    pub fn new() -> Self {
        Self { counter: std::sync::atomic::AtomicU64::new(0) }
    }

    /// 从指定值开始
    #[inline]
    pub fn with_start(start: u64) -> Self {
        Self { counter: std::sync::atomic::AtomicU64::new(start) }
    }

    /// 批量生成序列号（低延迟优化）
    #[inline]
    pub fn next_batch(&self, count: u64) -> u64 {
        self.counter.fetch_add(count, std::sync::atomic::Ordering::Relaxed)
    }
}

impl SequenceGenerator for AtomicSequenceGenerator {
    #[inline]
    fn next(&self) -> u64 {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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
        Self: Sized,
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
        Self: Sized,
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
        F: FnOnce(&mut Self),
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
        Self: Sized,
    {
        let field_changes = old_state.diff(self);

        if field_changes.is_empty() {
            return Err(EntityError::NoChangesDetected);
        }

        Ok(ChangeLogEntry::new(
            self.entity_id().to_string(),
            Self::entity_type().parse().unwrap(),
            ChangeType::Updated { changed_fields: field_changes },
            current_timestamp(),
            0,
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
        &self,
        ts_provider: &impl TimestampProvider,
        seq_gen: &impl SequenceGenerator,
    ) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized,
    {
        track_with(self, Operation::Create, ts_provider, seq_gen)
    }

    /// 自动追踪删除操作（带自定义提供者）
    fn track_delete_with(
        &self,
        ts_provider: &impl TimestampProvider,
        seq_gen: &impl SequenceGenerator,
    ) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized,
    {
        track_with(self, Operation::Delete, ts_provider, seq_gen)
    }

    /// 自动追踪更新操作（带自定义提供者）
    fn track_update_with<F>(
        &mut self,
        updater: F,
        ts_provider: &impl TimestampProvider,
        seq_gen: &impl SequenceGenerator,
    ) -> Result<ChangeLogEntry, EntityError>
    where
        Self: Sized,
        F: FnOnce(&mut Self),
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
    fn has_changes(&self, other: &Self) -> bool {
        !self.diff(other).is_empty()
    }

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
        self.entity_id().to_string() == *entry.entity_id()
            && Self::entity_type() == entry.entity_type()
    }

    fn table_schema() -> TableSchema {
        todo!()
    }
}

// ============================================================================
// 实体构造 Trait - 从 Created 事件重构实体
// ============================================================================

/// 从 Created 事件构造实体的 trait
///
/// 允许从 ChangeLogEntry (Created 类型) 中的字段信息重构出实体实例
///
/// # 模式
/// Input: ChangeLogEntry { ChangeType::Created { fields } } + Type Information
/// Output: New instance of the type
///
/// # 示例
/// ```ignore
/// let created_event = ChangeLogEntry::new(
///     "1", "Order",
///     ChangeType::Created {
///         fields: vec![
///             FieldChange::new("id", "", "1"),
///             FieldChange::new("symbol", "", "\"BTCUSDT\""),
///             FieldChange::new("price", "", "50000.0"),
///         ],
///     },
///     1000, 1
/// );
///
/// // 从 Created 事件重构 Order 实例
/// let order = Order::from_created_event(&created_event)?;
/// ```
pub trait FromCreatedEvent: Sized {
    /// 从 Created 事件的字段信息构造实体实例
    ///
    /// # 参数
    /// - `entry`: 包含 Created 事件和字段列表的 ChangeLogEntry
    ///
    /// # 返回
    /// 新构造的实体实例，或错误信息
    ///
    /// # 错误
    /// - EntityError::EntityTypeMismatch: 事件类型与期望类型不匹配
    /// - EntityError::FieldParseError: 无法解析字段值
    fn from_created_event(entry: &ChangeLogEntry) -> Result<Self, EntityError>;

    /// 从字段映射表构造实体（内部方法，可选重写）
    ///
    /// 默认实现：返回错误，提示需要自定义实现
    /// 子类可重写此方法简化构造逻辑
    fn from_field_map(
        fields: &std::collections::HashMap<String, String>,
    ) -> Result<Self, EntityError> {
        Err(EntityError::Custom("from_field_map not implemented for this type".to_string()))
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
    Deleted(ChangeLogEntry),
}

impl TrackingResult {
    /// 检查是否有变更
    pub fn has_change(&self) -> bool {
        !matches!(self, TrackingResult::NoChange)
    }

    /// 获取变更日志条目
    pub fn entry(&self) -> Option<&ChangeLogEntry> {
        match self {
            TrackingResult::NoChange => None,
            TrackingResult::Changed(entry)
            | TrackingResult::Created(entry)
            | TrackingResult::Deleted(entry) => Some(entry),
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
    Delete,
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
    T: Entity + 'static,
{
    track_with(entity, operation, &SystemTimestampProvider, &DefaultSequenceGenerator)
}

/// 统一的变更追踪接口（带自定义提供者）
#[inline]
pub fn track_with<T>(
    entity: &T,
    operation: Operation,
    ts_provider: &impl TimestampProvider,
    seq_gen: &impl SequenceGenerator,
) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + 'static,
{
    let change_type = match operation {
        Operation::Create => {
            // Created 事件包含空字段列表（实际字段需要通过 to_bytes 序列化）
            ChangeType::Created { fields: Vec::new() }
        }
        Operation::Delete => ChangeType::Deleted,
    };

    let entry = ChangeLogEntry::new(
        entity.entity_id().to_string(),
        T::entity_type().parse().unwrap(),
        change_type,
        ts_provider.now(),
        seq_gen.next(),
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
pub fn track_batch<T>(
    entities: &[T],
    operation: Operation,
) -> Result<Vec<ChangeLogEntry>, EntityError>
where
    T: Entity + 'static,
{
    track_batch_with(entities, operation, &SystemTimestampProvider, &AtomicSequenceGenerator::new())
}

/// 批量追踪实体操作（带自定义提供者）
pub fn track_batch_with<T>(
    entities: &[T],
    operation: Operation,
    ts_provider: &impl TimestampProvider,
    seq_gen: &impl SequenceGenerator,
) -> Result<Vec<ChangeLogEntry>, EntityError>
where
    T: Entity + 'static,
{
    let change_type = match operation {
        Operation::Create => ChangeType::Created { fields: Vec::new() },
        Operation::Delete => ChangeType::Deleted,
    };

    let timestamp = ts_provider.now();
    let mut entries = Vec::with_capacity(entities.len());

    for entity in entities {
        entries.push(ChangeLogEntry::new(
            entity.entity_id().to_string(),
            T::entity_type().parse().unwrap(),
            change_type.clone(),
            timestamp,
            seq_gen.next(),
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
    F: FnOnce(&mut T),
{
    track_update_with(entity, updater, &SystemTimestampProvider, &DefaultSequenceGenerator)
}

/// 追踪实体更新操作（带自定义提供者）
#[inline]
pub fn track_update_with<T, F>(
    entity: &mut T,
    updater: F,
    ts_provider: &impl TimestampProvider,
    seq_gen: &impl SequenceGenerator,
) -> Result<ChangeLogEntry, EntityError>
where
    T: Entity + Clone + 'static,
    F: FnOnce(&mut T),
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
        T::entity_type().parse().unwrap(),
        ChangeType::Updated { changed_fields: field_changes },
        ts_provider.now(),
        seq_gen.next(),
    );

    Ok(entry)
}

// ============================================================================
// 从 Created 事件重构实体的辅助函数
// ============================================================================

/// 从 Created 事件中提取字段值，构建字段映射表
///
/// # 参数
/// - `entry`: ChangeLogEntry，必须包含 Created 类型的变更
///
/// # 返回
/// 字段名 -> 字段值的映射表
///
/// # 错误
/// - 如果事件不是 Created 类型，返回错误
pub fn extract_fields_from_created_event(
    entry: &ChangeLogEntry,
) -> Result<std::collections::HashMap<String, String>, EntityError> {
    match entry.change_type() {
        ChangeType::Created { fields } => {
            let mut field_map = std::collections::HashMap::new();
            for field in fields {
                // Created 事件中，new_value 包含初始值
                field_map.insert(field.field_name.to_string(), field.new_value.clone());
            }
            Ok(field_map)
        }
        _ => Err(EntityError::Custom("Event is not a Created type event".to_string())),
    }
}

/// 从字符串解析值（支持多种基础类型）
///
/// # 参数
/// - `value`: 字符串值
/// - `type_hint`: 类型提示（"u64", "i64", "f64", "bool", "string"）
///
/// # 返回
/// 解析后的值（作为字符串）
pub fn parse_field_value(value: &str, type_hint: &str) -> Result<String, EntityError> {
    match type_hint {
        "u64" | "i64" | "u32" | "i32" | "f64" | "f32" | "bool" => {
            // 数值类型直接验证可解析性
            match type_hint {
                "u64" => {
                    value.parse::<u64>().map_err(|_| EntityError::FieldParseError {
                        field: "value".to_string(),
                        reason: format!("Cannot parse '{}' as u64", value),
                    })?;
                }
                "i64" => {
                    value.parse::<i64>().map_err(|_| EntityError::FieldParseError {
                        field: "value".to_string(),
                        reason: format!("Cannot parse '{}' as i64", value),
                    })?;
                }
                "f64" => {
                    value.parse::<f64>().map_err(|_| EntityError::FieldParseError {
                        field: "value".to_string(),
                        reason: format!("Cannot parse '{}' as f64", value),
                    })?;
                }
                "bool" => {
                    value.parse::<bool>().map_err(|_| EntityError::FieldParseError {
                        field: "value".to_string(),
                        reason: format!("Cannot parse '{}' as bool", value),
                    })?;
                }
                _ => {}
            }
            Ok(value.to_string())
        }
        "string" => {
            // String 类型：去掉引号
            if value.starts_with('\"') && value.ends_with('\"') && value.len() >= 2 {
                Ok(value[1..value.len() - 1].to_string())
            } else {
                Ok(value.to_string())
            }
        }
        _ => Ok(value.to_string()),
    }
}

/// 从 Created 事件重构实体的通用函数（闭包风格）
///
/// # 参数
/// - `entry`: Created 类型的 ChangeLogEntry
/// - `constructor`: 接收字段映射表，返回新实体的闭包
///
/// # 返回
/// 新构造的实体，或错误信息
///
/// # 示例
/// ```ignore
/// let order = reconstruct_from_created::<Order, _>(&entry, |fields| {
///     let id = fields.get("id").and_then(|v| v.parse().ok()).unwrap_or(0);
///     let symbol = fields.get("symbol").map(|v| v.clone()).unwrap_or_default();
///     Ok(Order { id, symbol })
/// })?;
/// ```
pub fn reconstruct_from_created<T, F>(
    entry: &ChangeLogEntry,
    constructor: F,
) -> Result<T, EntityError>
where
    F: Fn(&std::collections::HashMap<String, String>) -> Result<T, EntityError>,
{
    let field_map = extract_fields_from_created_event(entry)?;
    constructor(&field_map)
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
    T: Entity + 'static,
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
    T: Entity + 'static,
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
        value: String,
    }

    impl Entity for TestEntity {
        type Id = u64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> &'static str {
            "TestEntity"
        }

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
                    actual: entry.entity_id().to_string(),
                });
            }

            match entry.change_type() {
                ChangeType::Updated { changed_fields } => {
                    for field in changed_fields {
                        if field.field_name == "value" {
                            self.value = field.new_value.clone();
                        }
                    }
                    Ok(())
                }
                ChangeType::Deleted => Err(EntityError::CannotReplayOnDeleted),
                ChangeType::Created { fields: _ } => Ok(()),
            }
        }
    }

    #[test]
    fn test_auto_track_create() {
        let entity = TestEntity { id: 1, value: "test".to_string() };

        let entry = entity.track_create().unwrap();
        assert_eq!(entry.entity_id(), "1");
        assert_eq!(entry.entity_type(), "TestEntity");
        match entry.change_type() {
            ChangeType::Created { fields } => {
                assert_eq!(fields.len(), 0);
            }
            _ => panic!("Expected Created change type"),
        }
    }

    #[test]
    fn test_auto_track_update() {
        let mut entity = TestEntity { id: 1, value: "old".to_string() };

        let entry = entity
            .track_update(|e| {
                e.value = "new".to_string();
            })
            .unwrap();

        assert_eq!(entry.entity_id(), "1");
        assert_eq!(entry.entity_type(), "TestEntity");

        match &entry.change_type() {
            ChangeType::Updated { changed_fields } => {
                assert_eq!(changed_fields.len(), 1);
                assert_eq!(changed_fields[0].field_name, "value");
                // Debug formatting adds quotes for String types
                assert!(changed_fields[0].old_value.contains("old"));
                assert!(changed_fields[0].new_value.contains("new"));
            }
            _ => panic!("Expected Updated change type"),
        }

        // 确认实体已更新
        assert_eq!(entity.value, "new");
    }

    #[test]
    fn test_auto_track_update_from() {
        let old_entity = TestEntity { id: 1, value: "old".to_string() };

        let new_entity = TestEntity { id: 1, value: "new".to_string() };

        let entry = new_entity.track_update_from(&old_entity).unwrap();

        assert_eq!(entry.entity_id(), "1");
        match &entry.change_type() {
            ChangeType::Updated { changed_fields } => {
                assert_eq!(changed_fields.len(), 1);
                assert_eq!(changed_fields[0].field_name, "value");
            }
            _ => panic!("Expected Updated change type"),
        }
    }

    #[test]
    fn test_auto_track_no_changes() {
        let old_entity = TestEntity { id: 1, value: "same".to_string() };

        let new_entity = TestEntity { id: 1, value: "same".to_string() };

        let result = new_entity.track_update_from(&old_entity);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EntityError::NoChangesDetected);
    }

    // ==================== 从 Created 事件重构实体的测试 ====================

    #[test]
    fn test_parse_field_value_numeric() {
        // 数值类型
        let result = parse_field_value("42", "u64").unwrap();
        assert_eq!(result, "42");

        let result = parse_field_value("3.14", "f64").unwrap();
        assert_eq!(result, "3.14");

        let result = parse_field_value("true", "bool").unwrap();
        assert_eq!(result, "true");

        // 解析失败
        let result = parse_field_value("not_a_number", "u64");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_field_value_string() {
        // String 类型：去掉引号
        let result = parse_field_value("\"hello world\"", "string").unwrap();
        assert_eq!(result, "hello world");

        let result = parse_field_value("unquoted", "string").unwrap();
        assert_eq!(result, "unquoted");
    }
}
