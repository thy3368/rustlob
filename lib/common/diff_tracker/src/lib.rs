pub mod tracker;

// 重新导出核心类型
pub use diff::{ChangeType, FieldChange, ChangeLogEntry, Diff, Replay};

// 重新导出 Diff 和 Replay derive 宏
pub use diff_derive::{Diff, Replay, Tracked};

// 重新导出 tracker 模块的公共API
pub use tracker::{ChangeTracker, track_with_tracker, track_auto, track_changes};

