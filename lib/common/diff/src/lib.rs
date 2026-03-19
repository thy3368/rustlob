#![feature(portable_simd)]

pub mod diff;
pub use diff::diff_types;
// 重新导出核心类型，方便使用
pub use diff::diff_types::{
    // 核心数据结构
    ChangeLogEntry,
    ChangeType,
    // 核心 trait（Entity 现在包含了 Diff, Replayable, Trackable 的所有功能）
    Entity,
    EntityError,
    FieldChange,
    FieldSchema,
    // 从 Created 事件重构实体的 trait 和函数
    FromCreatedEvent,
    Operation,
    // 表结构定义
    TableSchema,
    extract_fields_from_created_event,
    parse_field_value,
    reconstruct_from_created,
    // 统一追踪接口
    track,
    track_batch,
    // 便捷追踪函数
    track_create,
    track_delete,
    track_update,
};

// Entity derive 宏从 entity_derive crate 导入
// 使用方法: #[derive(entity_derive::Entity)]
