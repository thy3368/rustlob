pub mod diff;

// 重新导出核心类型，方便使用
pub use diff::diff_types::{ChangeType, FieldChange, ChangeLogEntry, Diff};

