pub mod diff;
pub use diff::diff_types;

// 重新导出核心类型，方便使用
pub use diff::diff_types::{
    // 统一追踪接口
    track,
    track_update,
    track_update_with,
    track_batch,
    track_batch_with,
    Operation,
    // 便捷追踪函数
    track_create,
    track_delete,
    // 核心数据结构
    ChangeLogEntry,
    ChangeType,
    FieldChange,
    TrackingResult,
    EntitySnapshot,
    // 表结构定义
    TableSchema,
    FieldSchema,
    // 核心 trait（Entity 现在包含了 Diff, Replayable, Trackable 的所有功能）
    Entity,
    EntityError,
    // 时间戳和序列号提供者
    TimestampProvider,
    SystemTimestampProvider,
    CachedTimestampProvider,
    SequenceGenerator,
    DefaultSequenceGenerator,
    AtomicSequenceGenerator,
    // 从 Created 事件重构实体的 trait 和函数
    FromCreatedEvent,
    extract_fields_from_created_event,
    parse_field_value,
    reconstruct_from_created,
};

// Entity derive 宏从 entity_derive crate 导入
// 使用方法: #[derive(entity_derive::Entity)]
