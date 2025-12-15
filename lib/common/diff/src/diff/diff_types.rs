#[derive(Debug, Clone)]
pub enum ChangeType {
    Created,
    Updated { changed_fields: Vec<FieldChange> },
    Deleted
}

/// 字段变更记录
#[derive(Debug, Clone)]
pub struct FieldChange {
    pub field_name: String,
    pub old_value: String,
    pub new_value: String,
}

// 变更日志条目
#[derive(Debug, Clone)]
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
