/// 变更日志条目（AOS 版本）
///
/// 使用基础类型，不考虑零拷贝和零分配
/// 编解码逻辑通过 derive 宏实现
///
/// # 设计目标
/// - 使用基础类型：u64、u8、[u8; N]
/// - 直接包含字段变更列表
/// - 简单直观，易于使用
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeLogEntryBase {
    /// 变更时间戳（纳秒）
    pub timestamp: u64,
    /// 变更序列号（用于排序）
    pub sequence: u64,
    /// 实体唯一标识符（固定32字节）
    pub entity_id: [u8; 32],
    /// 实体类型标签（用户自定义映射）
    pub entity_type: u8,
    /// 变更类型标签（0=Created, 1=Updated, 2=Deleted）
    pub change_type: u8,
    /// 字段变更列表
    pub field_changes: Vec<FieldChange>,
}

/// 字段变更记录
///
/// 使用基础类型记录字段变更信息
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldChange {
    /// 字段名称（固定32字节）
    pub field_name: [u8; 32],
    /// 旧值数据（固定64字节）
    pub old_value: [u8; 64],
    /// 旧值实际长度
    pub old_value_len: u16,
    /// 新值数据（固定64字节）
    pub new_value: [u8; 64],
    /// 新值实际长度
    pub new_value_len: u16,
    /// 字段类型标签（0=String, 1=Int, 2=Float, 3=Bool, 4=Bytes）
    pub field_type: u8,
}

impl FieldChange {
    /// 创建新的字段变更记录
    pub fn new(
        field_name: [u8; 32],
        old_value: &[u8],
        new_value: &[u8],
        field_type: u8,
    ) -> Self {
        let mut old_val = [0u8; 64];
        let mut new_val = [0u8; 64];

        let old_len = old_value.len().min(64);
        let new_len = new_value.len().min(64);

        old_val[..old_len].copy_from_slice(&old_value[..old_len]);
        new_val[..new_len].copy_from_slice(&new_value[..new_len]);

        Self {
            field_name,
            old_value: old_val,
            old_value_len: old_len as u16,
            new_value: new_val,
            new_value_len: new_len as u16,
            field_type,
        }
    }

    /// 从字符串创建字段名
    pub fn field_name_from_str(s: &str) -> [u8; 32] {
        let mut name = [0u8; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        name[..len].copy_from_slice(&bytes[..len]);
        name
    }

    /// 获取字段名的字符串表示
    pub fn field_name_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let end = self.field_name.iter().position(|&b| b == 0).unwrap_or(32);
        std::str::from_utf8(&self.field_name[..end])
    }

    /// 获取旧值切片
    pub fn old_value_bytes(&self) -> &[u8] {
        &self.old_value[..self.old_value_len as usize]
    }

    /// 获取新值切片
    pub fn new_value_bytes(&self) -> &[u8] {
        &self.new_value[..self.new_value_len as usize]
    }

    /// 检查是否有旧值
    pub fn has_old_value(&self) -> bool {
        self.old_value_len > 0
    }

    /// 检查是否有新值
    pub fn has_new_value(&self) -> bool {
        self.new_value_len > 0
    }
}

impl ChangeLogEntryBase {
    /// 创建新的变更日志条目
    pub fn new(
        timestamp: u64,
        sequence: u64,
        entity_id: [u8; 32],
        entity_type: u8,
        change_type: u8,
    ) -> Self {
        Self {
            timestamp,
            sequence,
            entity_id,
            entity_type,
            change_type,
            field_changes: Vec::new(),
        }
    }

    /// 从字符串创建 entity_id
    pub fn entity_id_from_str(s: &str) -> [u8; 32] {
        let mut id = [0u8; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        id[..len].copy_from_slice(&bytes[..len]);
        id
    }

    /// 获取 entity_id 的字符串表示
    pub fn entity_id_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let end = self.entity_id.iter().position(|&b| b == 0).unwrap_or(32);
        std::str::from_utf8(&self.entity_id[..end])
    }

    /// 添加字段变更
    pub fn add_field_change(&mut self, field_change: FieldChange) {
        self.field_changes.push(field_change);
    }

    /// 检查是否为创建操作
    pub fn is_created(&self) -> bool {
        self.change_type == 0
    }

    /// 检查是否为更新操作
    pub fn is_updated(&self) -> bool {
        self.change_type == 1
    }

    /// 检查是否为删除操作
    pub fn is_deleted(&self) -> bool {
        self.change_type == 2
    }

    /// 获取字段变更数量
    pub fn field_change_count(&self) -> usize {
        self.field_changes.len()
    }
}

/// 字段变更记录（SOA 版本）
///
/// Structure of Arrays 布局，存储单个条目的所有字段变更
///
/// # 设计目标
/// - 缓存友好的内存布局
/// - 支持 SIMD 批量处理字段变更
/// - 每个条目独立管理
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldChangeSoa {
    /// 字段名称数组（每个32字节）
    pub field_names: Vec<[u8; 32]>,
    /// 旧值数据数组（每个64字节）
    pub old_values: Vec<[u8; 64]>,
    /// 旧值实际长度数组
    pub old_value_lens: Vec<u16>,
    /// 新值数据数组（每个64字节）
    pub new_values: Vec<[u8; 64]>,
    /// 新值实际长度数组
    pub new_value_lens: Vec<u16>,
    /// 字段类型标签数组（0=String, 1=Int, 2=Float, 3=Bool, 4=Bytes）
    pub field_types: Vec<u8>,
}

impl FieldChangeSoa {
    /// 创建新的空 SOA 结构
    pub fn new() -> Self {
        Self {
            field_names: Vec::new(),
            old_values: Vec::new(),
            old_value_lens: Vec::new(),
            new_values: Vec::new(),
            new_value_lens: Vec::new(),
            field_types: Vec::new(),
        }
    }

    /// 创建预分配容量的 SOA 结构
    pub fn with_capacity(field_capacity: usize) -> Self {
        Self {
            field_names: Vec::with_capacity(field_capacity),
            old_values: Vec::with_capacity(field_capacity),
            old_value_lens: Vec::with_capacity(field_capacity),
            new_values: Vec::with_capacity(field_capacity),
            new_value_lens: Vec::with_capacity(field_capacity),
            field_types: Vec::with_capacity(field_capacity),
        }
    }

    /// 添加字段变更
    pub fn push(&mut self, field_change: FieldChange) {
        self.field_names.push(field_change.field_name);
        self.old_values.push(field_change.old_value);
        self.old_value_lens.push(field_change.old_value_len);
        self.new_values.push(field_change.new_value);
        self.new_value_lens.push(field_change.new_value_len);
        self.field_types.push(field_change.field_type);
    }

    /// 从 Vec<FieldChange> 创建
    pub fn from_vec(field_changes: Vec<FieldChange>) -> Self {
        let mut soa = Self::with_capacity(field_changes.len());
        for fc in field_changes {
            soa.push(fc);
        }
        soa
    }

    /// 获取字段变更数量
    pub fn len(&self) -> usize {
        self.field_names.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.field_names.is_empty()
    }

    /// 清空所有数据（保留容量）
    pub fn clear(&mut self) {
        self.field_names.clear();
        self.old_values.clear();
        self.old_value_lens.clear();
        self.new_values.clear();
        self.new_value_lens.clear();
        self.field_types.clear();
    }

    /// 获取指定索引的字段变更
    pub fn get(&self, index: usize) -> Option<FieldChange> {
        if index >= self.field_names.len() {
            return None;
        }

        Some(FieldChange {
            field_name: self.field_names[index],
            old_value: self.old_values[index],
            old_value_len: self.old_value_lens[index],
            new_value: self.new_values[index],
            new_value_len: self.new_value_lens[index],
            field_type: self.field_types[index],
        })
    }

    /// 获取字段名称切片（用于批量操作）
    pub fn field_names(&self) -> &[[u8; 32]] {
        &self.field_names
    }

    /// 获取字段类型切片（用于批量操作）
    pub fn field_types(&self) -> &[u8] {
        &self.field_types
    }

    /// 转换为 Vec<FieldChange>
    pub fn to_vec(&self) -> Vec<FieldChange> {
        (0..self.len())
            .map(|i| self.get(i).unwrap())
            .collect()
    }
}

impl Default for FieldChangeSoa {
    fn default() -> Self {
        Self::new()
    }
}

/// 变更日志条目（SOA 版本）
///
/// Structure of Arrays 布局，将相同类型的字段存储在连续数组中
/// 适合批量处理和 SIMD 优化
///
/// # 设计目标
/// - 缓存友好的内存布局
/// - 支持 SIMD 批量操作
/// - 高效的列式访问
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeLogEntrySoa {
    /// 变更时间戳数组（纳秒）
    pub timestamps: Vec<u64>,
    /// 变更序列号数组（用于排序）
    pub sequences: Vec<u64>,
    /// 实体唯一标识符数组（每个32字节）
    pub entity_ids: Vec<[u8; 32]>,
    /// 实体类型标签数组（用户自定义映射）
    pub entity_types: Vec<u8>,
    /// 变更类型标签数组（0=Created, 1=Updated, 2=Deleted）
    pub change_types: Vec<u8>,
    /// 字段变更列表（每个条目对应一个 FieldChangeSoa）
    pub field_changes: Vec<FieldChangeSoa>,
}

impl ChangeLogEntrySoa {
    /// 创建新的空 SOA 结构
    pub fn new() -> Self {
        Self {
            timestamps: Vec::new(),
            sequences: Vec::new(),
            entity_ids: Vec::new(),
            entity_types: Vec::new(),
            change_types: Vec::new(),
            field_changes: Vec::new(),
        }
    }

    /// 创建预分配容量的 SOA 结构
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            timestamps: Vec::with_capacity(capacity),
            sequences: Vec::with_capacity(capacity),
            entity_ids: Vec::with_capacity(capacity),
            entity_types: Vec::with_capacity(capacity),
            change_types: Vec::with_capacity(capacity),
            field_changes: Vec::with_capacity(capacity),
        }
    }

    /// 添加一个变更日志条目
    pub fn push(
        &mut self,
        timestamp: u64,
        sequence: u64,
        entity_id: [u8; 32],
        entity_type: u8,
        change_type: u8,
        field_changes: Vec<FieldChange>,
    ) {
        self.timestamps.push(timestamp);
        self.sequences.push(sequence);
        self.entity_ids.push(entity_id);
        self.entity_types.push(entity_type);
        self.change_types.push(change_type);
        self.field_changes.push(FieldChangeSoa::from_vec(field_changes));
    }

    /// 从 AOS 条目添加
    pub fn push_entry(&mut self, entry: ChangeLogEntryBase) {
        self.timestamps.push(entry.timestamp);
        self.sequences.push(entry.sequence);
        self.entity_ids.push(entry.entity_id);
        self.entity_types.push(entry.entity_type);
        self.change_types.push(entry.change_type);
        self.field_changes.push(FieldChangeSoa::from_vec(entry.field_changes));
    }

    /// 获取条目数量
    pub fn len(&self) -> usize {
        self.timestamps.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.timestamps.is_empty()
    }

    /// 清空所有数据（保留容量）
    pub fn clear(&mut self) {
        self.timestamps.clear();
        self.sequences.clear();
        self.entity_ids.clear();
        self.entity_types.clear();
        self.change_types.clear();
        self.field_changes.clear();
    }

    /// 获取指定索引的条目（转换为 AOS）
    pub fn get(&self, index: usize) -> Option<ChangeLogEntryBase> {
        if index >= self.len() {
            return None;
        }

        Some(ChangeLogEntryBase {
            timestamp: self.timestamps[index],
            sequence: self.sequences[index],
            entity_id: self.entity_ids[index],
            entity_type: self.entity_types[index],
            change_type: self.change_types[index],
            field_changes: self.field_changes[index].to_vec(),
        })
    }

    /// 获取时间戳切片（用于批量操作）
    pub fn timestamps(&self) -> &[u64] {
        &self.timestamps
    }

    /// 获取序列号切片（用于批量操作）
    pub fn sequences(&self) -> &[u64] {
        &self.sequences
    }

    /// 获取实体 ID 切片（用于批量操作）
    pub fn entity_ids(&self) -> &[[u8; 32]] {
        &self.entity_ids
    }

    /// 获取实体类型切片（用于批量操作）
    pub fn entity_types(&self) -> &[u8] {
        &self.entity_types
    }

    /// 获取变更类型切片（用于批量操作）
    pub fn change_types(&self) -> &[u8] {
        &self.change_types
    }

    /// 批量过滤：获取指定时间范围内的索引
    pub fn filter_by_time_range(&self, start: u64, end: u64) -> Vec<usize> {
        self.timestamps
            .iter()
            .enumerate()
            .filter(|(_, &ts)| ts >= start && ts <= end)
            .map(|(i, _)| i)
            .collect()
    }

    /// 批量过滤：获取指定变更类型的索引
    pub fn filter_by_change_type(&self, change_type: u8) -> Vec<usize> {
        self.change_types
            .iter()
            .enumerate()
            .filter(|(_, &ct)| ct == change_type)
            .map(|(i, _)| i)
            .collect()
    }

    /// 批量过滤：获取指定实体类型的索引
    pub fn filter_by_entity_type(&self, entity_type: u8) -> Vec<usize> {
        self.entity_types
            .iter()
            .enumerate()
            .filter(|(_, &et)| et == entity_type)
            .map(|(i, _)| i)
            .collect()
    }
}

impl Default for ChangeLogEntrySoa {
    fn default() -> Self {
        Self::new()
    }
}

/// AOS 到 SOA 的转换
impl From<Vec<ChangeLogEntryBase>> for ChangeLogEntrySoa {
    fn from(entries: Vec<ChangeLogEntryBase>) -> Self {
        let mut soa = Self::with_capacity(entries.len());
        for entry in entries {
            soa.push_entry(entry);
        }
        soa
    }
}

/// SOA 到 AOS 的转换
impl From<ChangeLogEntrySoa> for Vec<ChangeLogEntryBase> {
    fn from(soa: ChangeLogEntrySoa) -> Self {
        (0..soa.len())
            .map(|i| ChangeLogEntryBase {
                timestamp: soa.timestamps[i],
                sequence: soa.sequences[i],
                entity_id: soa.entity_ids[i],
                entity_type: soa.entity_types[i],
                change_type: soa.change_types[i],
                field_changes: soa.field_changes[i].to_vec(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry() {
        let entity_id = ChangeLogEntryBase::entity_id_from_str("order_123");
        let entry = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 0);

        assert_eq!(entry.timestamp, 1000);
        assert_eq!(entry.sequence, 1);
        assert_eq!(entry.entity_id_as_str().unwrap(), "order_123");
        assert_eq!(entry.entity_type, 1);
        assert_eq!(entry.change_type, 0);
        assert_eq!(entry.field_change_count(), 0);
    }

    #[test]
    fn test_change_type_checks() {
        let entity_id = ChangeLogEntryBase::entity_id_from_str("test");

        let created = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 0);
        assert!(created.is_created());
        assert!(!created.is_updated());
        assert!(!created.is_deleted());

        let updated = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 1);
        assert!(!updated.is_created());
        assert!(updated.is_updated());
        assert!(!updated.is_deleted());

        let deleted = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 2);
        assert!(!deleted.is_created());
        assert!(!deleted.is_updated());
        assert!(deleted.is_deleted());
    }

    #[test]
    fn test_add_field_change() {
        let entity_id = ChangeLogEntryBase::entity_id_from_str("order_123");
        let mut entry = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 1);

        let field_name = FieldChange::field_name_from_str("price");
        let field_change = FieldChange::new(
            field_name,
            b"100.0",
            b"120.0",
            0,
        );

        entry.add_field_change(field_change);
        assert_eq!(entry.field_change_count(), 1);
    }

    #[test]
    fn test_field_change() {
        let field_name = FieldChange::field_name_from_str("price");
        let field_change = FieldChange::new(
            field_name,
            b"100.0",
            b"120.0",
            0,
        );

        assert_eq!(field_change.field_name_as_str().unwrap(), "price");
        assert_eq!(field_change.old_value_bytes(), b"100.0");
        assert_eq!(field_change.new_value_bytes(), b"120.0");
        assert!(field_change.has_old_value());
        assert!(field_change.has_new_value());
    }

    #[test]
    fn test_entity_id_conversion() {
        let id1 = ChangeLogEntryBase::entity_id_from_str("short");
        assert_eq!(&id1[..5], b"short");
        assert_eq!(&id1[5..], &[0u8; 27]);

        let long_str = "this_is_a_very_long_entity_id_that_exceeds_32_bytes";
        let id2 = ChangeLogEntryBase::entity_id_from_str(long_str);
        assert_eq!(&id2[..], &long_str.as_bytes()[..32]);

        let entity_id = ChangeLogEntryBase::entity_id_from_str("test_order_123");
        let entry = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 0);
        assert_eq!(entry.entity_id_as_str().unwrap(), "test_order_123");
    }
}

//todo 在新文件为 ChangeLogEntrySoa 生成0copy 0alloc的二进制编解码


