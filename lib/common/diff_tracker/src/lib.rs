pub mod tracker;

// 重新导出核心类型
pub use diff::{ChangeType, FieldChange, ChangeLogEntry, Diff};

// 重新导出 Diff derive 宏
pub use diff_derive::Diff;

// 重新导出 tracker 模块的公共API
pub use tracker::tracker::{ChangeTracker, track_with_tracker, track_auto, track_changes};

