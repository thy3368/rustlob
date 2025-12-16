pub mod diff;

// 重新导出核心类型，方便使用
pub use diff::diff_types::{
    // 便捷追踪函数
    track_auto,
    track_changes,
    ChangeLogEntry,
    // 核心枚举和数据结构
    ChangeType,
    Diff,
    // 核心 trait
    Entity,
    EntitySnapshot,
    FieldChange,
    Replayable,
    Trackable
};
