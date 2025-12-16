pub mod diff;

// 重新导出 derive 宏
pub use diff_derive::{Diff, Replayable};

// 重新导出核心类型，方便使用
pub use diff::diff_types::{
    // 统一追踪接口
    track,
    track_update,
    Operation,
    // 便捷追踪函数（向后兼容）
    track_create,
    track_delete,
    ChangeLogEntry,
    // 核心枚举和数据结构
    ChangeType,
    Diff as DiffTrait,
    // 核心 trait
    Entity,
    EntitySnapshot,
    FieldChange,
    Replayable as ReplayableTrait,
    Trackable,
    TrackingResult,
};
