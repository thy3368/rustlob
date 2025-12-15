#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Created,
    Updated { changed_fields: Vec<FieldChange> },
    Deleted
}

/// 字段变更记录
#[derive(Debug, Clone, PartialEq)]
pub struct FieldChange {
    pub field_name: String,
    pub old_value: String,
    pub new_value: String,
}

// 变更日志条目
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeLogEntry {
    pub entity_id: String,
    pub entity_type: String,
    pub change_type: ChangeType,
    pub timestamp: u64,
}

/// Diff trait - 用于自动检测字段变更
pub trait Diff {
    /// 比较 self(旧) 和 other(新)，返回字段变更列表
    fn diff(&self, other: &Self) -> Vec<FieldChange>;
}

/// Replay trait - 用于从变更日志回放数据
pub trait Replay {
    /// 从变更日志条目回放数据，更新 self 的字段
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String>;
}
