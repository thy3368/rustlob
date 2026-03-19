//! ChangeLogEntrySoa 零拷贝、零分配二进制编解码器
//!
//! 为 ChangeLogEntryBase 的 SOA 版本提供高性能的二进制序列化和反序列化
//!
//! # 二进制格式
//!
//! ```text
//! Header (24 字节，8 字节对齐):
//!   - magic: [u8; 4] = b"CLSB" (ChangeLog SOA Base)
//!   - version: u8 = 4
//!   - padding: [u8; 3] (对齐到 8 字节)
//!   - entry_count: u64
//!   - reserved: u64
//!
//! Entry Arrays:
//!   - timestamps: [u64; entry_count]
//!   - sequences: [u64; entry_count]
//!   - old_versions: [u64; entry_count]  (乐观锁支持)
//!   - new_versions: [u64; entry_count]  (乐观锁支持)
//!   - entity_ids: [i64; entry_count]  (实体ID改为i64)
//!   - entity_types: [u8; entry_count]
//!   - change_types: [u8; entry_count]
//!
//! Field Change Arrays (每个条目):
//!   - field_change_count: u32
//!   - field_names: [[u8; 32]; field_change_count]
//!   - old_values: [[u8; 64]; field_change_count]
//!   - old_value_lens: [u16; field_change_count]
//!   - new_values: [[u8; 64]; field_change_count]
//!   - new_value_lens: [u16; field_change_count]
//!   - field_types: [u8; field_change_count]
//! ```

use super::entity_change_log::{EntityChangeLogSoa, FieldChange, FieldChangeSoa};
use std::mem::size_of;

/// 魔数标识
const MAGIC: &[u8; 4] = b"CLSB";
/// 版本号（版本 4：entity_id 改为 i64）
const VERSION: u8 = 4;
/// 头部大小（包含 padding）
const HEADER_SIZE: usize = 24;

/// 编码错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    /// 缓冲区太小
    BufferTooSmall { required: usize, available: usize },
    /// 数据过大
    DataTooLarge,
}

/// 解码错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// 数据不足
    InsufficientData,
    /// 无效的魔数
    InvalidMagic,
    /// 不支持的版本
    UnsupportedVersion(u8),
    /// 数据对齐错误
    MisalignedData,
    /// 索引越界
    IndexOutOfBounds,
}

/// ChangeLogEntrySoa 编码器
///
/// 支持零分配编码，可复用缓冲区
pub struct ChangeLogEntrySoaEncoder {
    soa: EntityChangeLogSoa,
}

impl ChangeLogEntrySoaEncoder {
    /// 创建新的编码器
    pub fn new() -> Self {
        Self {
            soa: EntityChangeLogSoa::new(),
        }
    }

    /// 创建预分配容量的编码器
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            soa: EntityChangeLogSoa::with_capacity(capacity),
        }
    }

    /// 添加条目
    pub fn push(&mut self, entry: super::entity_change_log::EntityChangeLog) {
        self.soa.push_entry(entry);
    }

    /// 获取条目数量
    pub fn len(&self) -> usize {
        self.soa.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.soa.is_empty()
    }

    /// 清空数据（保留容量）
    pub fn clear(&mut self) {
        self.soa.clear();
    }

    /// 计算编码后的大小
    pub fn encoded_size(&self) -> usize {
        let entry_count = self.soa.len();

        let mut size = HEADER_SIZE;

        // Entry arrays
        size += entry_count * size_of::<u64>(); // timestamps
        size += entry_count * size_of::<u64>(); // sequences
        size += entry_count * size_of::<u64>(); // old_versions
        size += entry_count * size_of::<u64>(); // new_versions
        size += entry_count * size_of::<i64>(); // entity_ids
        size += entry_count * size_of::<u8>(); // entity_types
        size += entry_count * size_of::<u8>(); // change_types

        // Field changes for each entry
        for fc_soa in &self.soa.field_changes {
            let fc_count = fc_soa.len();
            size += size_of::<u32>(); // field_change_count
            size += fc_count * 32; // field_names
            size += fc_count * 64; // old_values
            size += fc_count * size_of::<u16>(); // old_value_lens
            size += fc_count * 64; // new_values
            size += fc_count * size_of::<u16>(); // new_value_lens
            size += fc_count * size_of::<u8>(); // field_types
        }

        size
    }

    /// 编码到新分配的 Vec
    pub fn encode(&self) -> Vec<u8> {
        let size = self.encoded_size();
        let mut buffer = vec![0u8; size];
        self.encode_to(&mut buffer).unwrap();
        buffer
    }

    /// 编码到外部缓冲区（零分配）
    pub fn encode_to(&self, buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let required_size = self.encoded_size();
        if buffer.len() < required_size {
            return Err(EncodeError::BufferTooSmall {
                required: required_size,
                available: buffer.len(),
            });
        }

        let entry_count = self.soa.len();

        let mut offset = 0;

        // 写入头部
        buffer[offset..offset + 4].copy_from_slice(MAGIC);
        offset += 4;
        buffer[offset] = VERSION;
        offset += 1;
        // padding (3 字节)
        buffer[offset..offset + 3].fill(0);
        offset += 3;
        buffer[offset..offset + 8].copy_from_slice(&(entry_count as u64).to_le_bytes());
        offset += 8;
        // reserved (8 字节)
        buffer[offset..offset + 8].fill(0);
        offset += 8;
        // 对齐检查
        debug_assert_eq!(offset, HEADER_SIZE);

        // 写入 timestamps
        for &ts in &self.soa.timestamps {
            buffer[offset..offset + 8].copy_from_slice(&ts.to_le_bytes());
            offset += 8;
        }

        // 写入 sequences
        for &seq in &self.soa.sequences {
            buffer[offset..offset + 8].copy_from_slice(&seq.to_le_bytes());
            offset += 8;
        }

        // 写入 old_versions
        for &old_ver in &self.soa.old_versions {
            buffer[offset..offset + 8].copy_from_slice(&old_ver.to_le_bytes());
            offset += 8;
        }

        // 写入 new_versions
        for &new_ver in &self.soa.new_versions {
            buffer[offset..offset + 8].copy_from_slice(&new_ver.to_le_bytes());
            offset += 8;
        }

        // 写入 entity_ids
        for &entity_id in &self.soa.entity_ids {
            buffer[offset..offset + 8].copy_from_slice(&entity_id.to_le_bytes());
            offset += 8;
        }

        // 写入 entity_types
        buffer[offset..offset + entry_count].copy_from_slice(&self.soa.entity_types);
        offset += entry_count;

        // 写入 change_types
        buffer[offset..offset + entry_count].copy_from_slice(&self.soa.change_types);
        offset += entry_count;

        // 写入每个条目的字段变更数据
        for fc_soa in &self.soa.field_changes {
            let fc_count = fc_soa.len();

            // 写入字段变更数量
            buffer[offset..offset + 4].copy_from_slice(&(fc_count as u32).to_le_bytes());
            offset += 4;

            // field_names
            for field_name in &fc_soa.field_names {
                buffer[offset..offset + 32].copy_from_slice(field_name);
                offset += 32;
            }

            // old_values
            for old_value in &fc_soa.old_values {
                buffer[offset..offset + 64].copy_from_slice(old_value);
                offset += 64;
            }

            // old_value_lens
            for &len in &fc_soa.old_value_lens {
                buffer[offset..offset + 2].copy_from_slice(&len.to_le_bytes());
                offset += 2;
            }

            // new_values
            for new_value in &fc_soa.new_values {
                buffer[offset..offset + 64].copy_from_slice(new_value);
                offset += 64;
            }

            // new_value_lens
            for &len in &fc_soa.new_value_lens {
                buffer[offset..offset + 2].copy_from_slice(&len.to_le_bytes());
                offset += 2;
            }

            // field_types
            buffer[offset..offset + fc_count].copy_from_slice(&fc_soa.field_types);
            offset += fc_count;
        }

        Ok(offset)
    }
}

impl Default for ChangeLogEntrySoaEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// ChangeLogEntrySoa 解码器（零拷贝）
///
/// 直接引用二进制数据，不进行内存拷贝
pub struct ChangeLogEntrySoaDecoder<'a> {
    data: &'a [u8],
    entry_count: usize,
    timestamps_offset: usize,
    sequences_offset: usize,
    old_versions_offset: usize,
    new_versions_offset: usize,
    entity_ids_offset: usize,
    entity_types_offset: usize,
    change_types_offset: usize,
    // 每个条目的字段变更信息
    field_change_infos: Vec<FieldChangeInfo>,
}

/// 字段变更信息
struct FieldChangeInfo {
    offset: usize,
    count: usize,
}

impl<'a> ChangeLogEntrySoaDecoder<'a> {
    /// 创建新的解码器（零拷贝）
    pub fn new(data: &'a [u8]) -> Result<Self, DecodeError> {
        if data.len() < HEADER_SIZE {
            return Err(DecodeError::InsufficientData);
        }

        // 验证魔数
        if &data[0..4] != MAGIC {
            return Err(DecodeError::InvalidMagic);
        }

        // 验证版本
        let version = data[4];
        if version != VERSION {
            return Err(DecodeError::UnsupportedVersion(version));
        }

        // 读取头部
        let entry_count = u64::from_le_bytes([
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
        ]) as usize;

        // 计算各个数组的偏移量
        let mut offset = HEADER_SIZE;

        let timestamps_offset = offset;
        offset += entry_count * size_of::<u64>();

        let sequences_offset = offset;
        offset += entry_count * size_of::<u64>();

        let old_versions_offset = offset;
        offset += entry_count * size_of::<u64>();

        let new_versions_offset = offset;
        offset += entry_count * size_of::<u64>();

        let entity_ids_offset = offset;
        offset += entry_count * size_of::<i64>();

        let entity_types_offset = offset;
        offset += entry_count;

        let change_types_offset = offset;
        offset += entry_count;

        // 读取每个条目的字段变更信息
        let mut field_change_infos = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            if offset + 4 > data.len() {
                return Err(DecodeError::InsufficientData);
            }

            let fc_count = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            offset += 4;

            let fc_offset = offset;

            // 计算该条目字段变更数据的大小
            let fc_size = fc_count * 32 // field_names
                + fc_count * 64 // old_values
                + fc_count * 2 // old_value_lens
                + fc_count * 64 // new_values
                + fc_count * 2 // new_value_lens
                + fc_count; // field_types

            offset += fc_size;

            field_change_infos.push(FieldChangeInfo {
                offset: fc_offset,
                count: fc_count,
            });
        }

        // 验证数据长度
        if data.len() < offset {
            return Err(DecodeError::InsufficientData);
        }

        Ok(Self {
            data,
            entry_count,
            timestamps_offset,
            sequences_offset,
            old_versions_offset,
            new_versions_offset,
            entity_ids_offset,
            entity_types_offset,
            change_types_offset,
            field_change_infos,
        })
    }

    /// 获取条目数量
    pub fn len(&self) -> usize {
        self.entry_count
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.entry_count == 0
    }

    /// 获取时间戳数组（零拷贝）
    pub fn timestamps(&self) -> &[u64] {
        let start = self.timestamps_offset;
        let end = start + self.entry_count * size_of::<u64>();
        let bytes = &self.data[start..end];
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, self.entry_count) }
    }

    /// 获取序列号数组（零拷贝）
    pub fn sequences(&self) -> &[u64] {
        let start = self.sequences_offset;
        let end = start + self.entry_count * size_of::<u64>();
        let bytes = &self.data[start..end];
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, self.entry_count) }
    }

    /// 获取旧版本号数组（零拷贝）
    pub fn old_versions(&self) -> &[u64] {
        let start = self.old_versions_offset;
        let end = start + self.entry_count * size_of::<u64>();
        let bytes = &self.data[start..end];
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, self.entry_count) }
    }

    /// 获取新版本号数组（零拷贝）
    pub fn new_versions(&self) -> &[u64] {
        let start = self.new_versions_offset;
        let end = start + self.entry_count * size_of::<u64>();
        let bytes = &self.data[start..end];
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, self.entry_count) }
    }

    /// 获取实体类型数组（零拷贝）
    pub fn entity_types(&self) -> &[u8] {
        let start = self.entity_types_offset;
        let end = start + self.entry_count;
        &self.data[start..end]
    }

    /// 获取变更类型数组（零拷贝）
    pub fn change_types(&self) -> &[u8] {
        let start = self.change_types_offset;
        let end = start + self.entry_count;
        &self.data[start..end]
    }

    /// 获取指定索引的实体 ID
    pub fn entity_id(&self, index: usize) -> Result<i64, DecodeError> {
        if index >= self.entry_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.entity_ids_offset + index * size_of::<i64>();
        let bytes = [
            self.data[start],
            self.data[start + 1],
            self.data[start + 2],
            self.data[start + 3],
            self.data[start + 4],
            self.data[start + 5],
            self.data[start + 6],
            self.data[start + 7],
        ];
        Ok(i64::from_le_bytes(bytes))
    }

    /// 获取指定条目的字段变更数量
    pub fn entry_field_change_count(&self, entry_index: usize) -> Result<usize, DecodeError> {
        if entry_index >= self.entry_count {
            return Err(DecodeError::IndexOutOfBounds);
        }
        Ok(self.field_change_infos[entry_index].count)
    }

    /// 获取指定条目的字段变更 SOA（需要内存拷贝）
    pub fn get_field_changes(&self, entry_index: usize) -> Result<FieldChangeSoa, DecodeError> {
        if entry_index >= self.entry_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let info = &self.field_change_infos[entry_index];
        let fc_count = info.count;
        let mut offset = info.offset;

        let mut soa = FieldChangeSoa::with_capacity(fc_count);

        // 读取 field_names
        for _ in 0..fc_count {
            let mut field_name = [0u8; 32];
            field_name.copy_from_slice(&self.data[offset..offset + 32]);
            soa.field_names.push(field_name);
            offset += 32;
        }

        // 读取 old_values
        for _ in 0..fc_count {
            let mut old_value = [0u8; 64];
            old_value.copy_from_slice(&self.data[offset..offset + 64]);
            soa.old_values.push(old_value);
            offset += 64;
        }

        // 读取 old_value_lens
        for _ in 0..fc_count {
            let len = u16::from_le_bytes([self.data[offset], self.data[offset + 1]]);
            soa.old_value_lens.push(len);
            offset += 2;
        }

        // 读取 new_values
        for _ in 0..fc_count {
            let mut new_value = [0u8; 64];
            new_value.copy_from_slice(&self.data[offset..offset + 64]);
            soa.new_values.push(new_value);
            offset += 64;
        }

        // 读取 new_value_lens
        for _ in 0..fc_count {
            let len = u16::from_le_bytes([self.data[offset], self.data[offset + 1]]);
            soa.new_value_lens.push(len);
            offset += 2;
        }

        // 读取 field_types
        for _ in 0..fc_count {
            soa.field_types.push(self.data[offset]);
            offset += 1;
        }

        Ok(soa)
    }

    /// 转换为 ChangeLogEntrySoa（需要内存拷贝）
    pub fn to_soa(&self) -> EntityChangeLogSoa {
        let mut soa = EntityChangeLogSoa::with_capacity(self.entry_count);

        // 拷贝基础数据
        soa.timestamps.extend_from_slice(self.timestamps());
        soa.sequences.extend_from_slice(self.sequences());
        soa.old_versions.extend_from_slice(self.old_versions());
        soa.new_versions.extend_from_slice(self.new_versions());
        soa.entity_types.extend_from_slice(self.entity_types());
        soa.change_types.extend_from_slice(self.change_types());

        // 拷贝 entity_ids
        for i in 0..self.entry_count {
            soa.entity_ids.push(self.entity_id(i).unwrap());
        }

        // 拷贝字段变更数据
        for i in 0..self.entry_count {
            soa.field_changes.push(self.get_field_changes(i).unwrap());
        }

        soa
    }
}

#[cfg(test)]
mod tests {
    use super::super::entity_change_log::{EntityChangeLog, FieldChange};
    use super::*;

    #[test]
    fn test_encode_decode_empty() {
        let encoder = ChangeLogEntrySoaEncoder::new();
        let data = encoder.encode();

        let decoder = ChangeLogEntrySoaDecoder::new(&data).unwrap();
        assert_eq!(decoder.len(), 0);
        assert!(decoder.is_empty());
    }

    #[test]
    fn test_encode_decode_single_entry() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        let entity_id = EntityChangeLog::entity_id_from_str("123").unwrap();
        let entry = EntityChangeLog::new(1000, 1, 0, 1, entity_id, 1, 0);
        encoder.push(entry);

        let data = encoder.encode();
        let decoder = ChangeLogEntrySoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.len(), 1);
        assert_eq!(decoder.timestamps()[0], 1000);
        assert_eq!(decoder.sequences()[0], 1);
        assert_eq!(decoder.old_versions()[0], 0);
        assert_eq!(decoder.new_versions()[0], 1);
        assert_eq!(decoder.entity_types()[0], 1);
        assert_eq!(decoder.change_types()[0], 0);
    }

    #[test]
    fn test_encode_decode_with_field_changes() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        let entity_id = EntityChangeLog::entity_id_from_str("456").unwrap();
        let mut entry = EntityChangeLog::new(1000, 1, 1, 2, entity_id, 1, 1);

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        entry.add_field_change(fc);

        encoder.push(entry);

        let data = encoder.encode();
        let decoder = ChangeLogEntrySoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.len(), 1);
        assert_eq!(decoder.entry_field_change_count(0).unwrap(), 1);
    }

    #[test]
    fn test_encode_to_external_buffer() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        let entity_id = EntityChangeLog::entity_id_from_str("789").unwrap();
        let entry = EntityChangeLog::new(1000, 1, 0, 1, entity_id, 1, 0);
        encoder.push(entry);

        let size = encoder.encoded_size();
        let mut buffer = vec![0u8; size];
        let written = encoder.encode_to(&mut buffer).unwrap();

        assert_eq!(written, size);

        let decoder = ChangeLogEntrySoaDecoder::new(&buffer).unwrap();
        assert_eq!(decoder.len(), 1);
    }

    #[test]
    fn test_roundtrip_conversion() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        for i in 0..5 {
            let entity_id = i as i64;
            let mut entry = EntityChangeLog::new(1000 + i, i, i, i + 1, entity_id, 1, 1);

            let field_name = FieldChange::field_name_from_str("price");
            let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
            entry.add_field_change(fc);

            encoder.push(entry);
        }

        let data = encoder.encode();
        let decoder = ChangeLogEntrySoaDecoder::new(&data).unwrap();
        let soa = decoder.to_soa();

        assert_eq!(soa.len(), 5);
        assert_eq!(soa.timestamps[0], 1000);
        assert_eq!(soa.timestamps[4], 1004);
        assert_eq!(soa.old_versions[0], 0);
        assert_eq!(soa.new_versions[0], 1);
        assert_eq!(soa.old_versions[4], 4);
        assert_eq!(soa.new_versions[4], 5);
        let total_field_changes: usize = soa.field_changes.iter().map(|fc| fc.len()).sum();
        assert_eq!(total_field_changes, 5);
    }

    #[test]
    fn test_buffer_too_small() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        let entity_id = EntityChangeLog::entity_id_from_str("999").unwrap();
        let entry = EntityChangeLog::new(1000, 1, 0, 1, entity_id, 1, 0);
        encoder.push(entry);

        let mut small_buffer = vec![0u8; 10];
        let result = encoder.encode_to(&mut small_buffer);

        assert!(matches!(result, Err(EncodeError::BufferTooSmall { .. })));
    }

    #[test]
    fn test_invalid_magic() {
        let mut data = vec![0u8; HEADER_SIZE];
        data[0..4].copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
        let result = ChangeLogEntrySoaDecoder::new(&data);

        assert!(matches!(result, Err(DecodeError::InvalidMagic)));
    }

    #[test]
    fn test_insufficient_data() {
        let data = vec![0u8; 10];
        let result = ChangeLogEntrySoaDecoder::new(&data);

        assert!(matches!(result, Err(DecodeError::InsufficientData)));
    }
}

// ============================================================================
// FieldChangeSoa 独立编解码器
// ============================================================================

/// FieldChangeSoa 魔数标识
const FC_MAGIC: &[u8; 4] = b"FCSB";
/// FieldChangeSoa 版本号
const FC_VERSION: u8 = 1;
/// FieldChangeSoa 头部大小
const FC_HEADER_SIZE: usize = 24;

/// FieldChangeSoa 编码器
///
/// 支持零分配编码，可复用缓冲区
pub struct FieldChangeSoaEncoder {
    soa: FieldChangeSoa,
}

impl FieldChangeSoaEncoder {
    /// 创建新的编码器
    pub fn new() -> Self {
        Self {
            soa: FieldChangeSoa::new(),
        }
    }

    /// 创建预分配容量的编码器
    pub fn with_capacity(field_capacity: usize) -> Self {
        Self {
            soa: FieldChangeSoa::with_capacity(field_capacity),
        }
    }

    /// 添加字段变更
    pub fn push(&mut self, field_change: FieldChange) {
        self.soa.push(field_change);
    }

    /// 从 Vec<FieldChange> 创建
    pub fn from_vec(field_changes: Vec<FieldChange>) -> Self {
        Self {
            soa: FieldChangeSoa::from_vec(field_changes),
        }
    }

    /// 获取字段变更数量
    pub fn len(&self) -> usize {
        self.soa.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.soa.is_empty()
    }

    /// 清空数据（保留容量）
    pub fn clear(&mut self) {
        self.soa.clear();
    }

    /// 计算编码后的大小
    pub fn encoded_size(&self) -> usize {
        let field_count = self.soa.len();

        FC_HEADER_SIZE
            + field_count * 32 // field_names
            + field_count * 64 // old_values
            + field_count * size_of::<u16>() // old_value_lens
            + field_count * 64 // new_values
            + field_count * size_of::<u16>() // new_value_lens
            + field_count * size_of::<u8>() // field_types
    }

    /// 编码到新分配的 Vec
    pub fn encode(&self) -> Vec<u8> {
        let size = self.encoded_size();
        let mut buffer = vec![0u8; size];
        self.encode_to(&mut buffer).unwrap();
        buffer
    }

    /// 编码到外部缓冲区（零分配）
    pub fn encode_to(&self, buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let required_size = self.encoded_size();
        if buffer.len() < required_size {
            return Err(EncodeError::BufferTooSmall {
                required: required_size,
                available: buffer.len(),
            });
        }

        let field_count = self.soa.len();

        let mut offset = 0;

        // 写入头部
        buffer[offset..offset + 4].copy_from_slice(FC_MAGIC);
        offset += 4;
        buffer[offset] = FC_VERSION;
        offset += 1;
        // padding (3 字节)
        buffer[offset..offset + 3].fill(0);
        offset += 3;
        buffer[offset..offset + 8].copy_from_slice(&(field_count as u64).to_le_bytes());
        offset += 8;
        buffer[offset..offset + 8].copy_from_slice(&0u64.to_le_bytes()); // reserved
        offset += 8;
        debug_assert_eq!(offset, FC_HEADER_SIZE);

        // 写入字段变更数据
        let fc = &self.soa;

        // field_names
        for field_name in &fc.field_names {
            buffer[offset..offset + 32].copy_from_slice(field_name);
            offset += 32;
        }

        // old_values
        for old_value in &fc.old_values {
            buffer[offset..offset + 64].copy_from_slice(old_value);
            offset += 64;
        }

        // old_value_lens
        for &len in &fc.old_value_lens {
            buffer[offset..offset + 2].copy_from_slice(&len.to_le_bytes());
            offset += 2;
        }

        // new_values
        for new_value in &fc.new_values {
            buffer[offset..offset + 64].copy_from_slice(new_value);
            offset += 64;
        }

        // new_value_lens
        for &len in &fc.new_value_lens {
            buffer[offset..offset + 2].copy_from_slice(&len.to_le_bytes());
            offset += 2;
        }

        // field_types
        buffer[offset..offset + field_count].copy_from_slice(&fc.field_types);
        offset += field_count;

        Ok(offset)
    }
}

impl Default for FieldChangeSoaEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// FieldChangeSoa 解码器（零拷贝）
///
/// 直接引用二进制数据，不进行内存拷贝
/// 用于解码单个 FieldChangeSoa 结构（一个条目的字段变更）
pub struct FieldChangeSoaDecoder<'a> {
    data: &'a [u8],
    field_count: usize,
    field_names_offset: usize,
    old_values_offset: usize,
    old_value_lens_offset: usize,
    new_values_offset: usize,
    new_value_lens_offset: usize,
    field_types_offset: usize,
}

impl<'a> FieldChangeSoaDecoder<'a> {
    /// 创建新的解码器（零拷贝）
    pub fn new(data: &'a [u8]) -> Result<Self, DecodeError> {
        if data.len() < FC_HEADER_SIZE {
            return Err(DecodeError::InsufficientData);
        }

        // 验证魔数
        if &data[0..4] != FC_MAGIC {
            return Err(DecodeError::InvalidMagic);
        }

        // 验证版本
        let version = data[4];
        if version != FC_VERSION {
            return Err(DecodeError::UnsupportedVersion(version));
        }

        // 读取头部
        let field_count = u64::from_le_bytes([
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
        ]) as usize;

        // 计算各个数组的偏移量
        let mut offset = FC_HEADER_SIZE;

        let field_names_offset = offset;
        offset += field_count * 32;

        let old_values_offset = offset;
        offset += field_count * 64;

        let old_value_lens_offset = offset;
        offset += field_count * size_of::<u16>();

        let new_values_offset = offset;
        offset += field_count * 64;

        let new_value_lens_offset = offset;
        offset += field_count * size_of::<u16>();

        let field_types_offset = offset;
        offset += field_count;

        // 验证数据长度
        if data.len() < offset {
            return Err(DecodeError::InsufficientData);
        }

        Ok(Self {
            data,
            field_count,
            field_names_offset,
            old_values_offset,
            old_value_lens_offset,
            new_values_offset,
            new_value_lens_offset,
            field_types_offset,
        })
    }

    /// 从嵌入的数据创建解码器（用于 ChangeLogEntrySoaDecoder）
    pub fn from_embedded(
        data: &'a [u8],
        field_count: usize,
        field_names_offset: usize,
        old_values_offset: usize,
        old_value_lens_offset: usize,
        new_values_offset: usize,
        new_value_lens_offset: usize,
        field_types_offset: usize,
    ) -> Self {
        Self {
            data,
            field_count,
            field_names_offset,
            old_values_offset,
            old_value_lens_offset,
            new_values_offset,
            new_value_lens_offset,
            field_types_offset,
        }
    }

    /// 获取字段变更数量
    pub fn len(&self) -> usize {
        self.field_count
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.field_count == 0
    }

    /// 获取字段类型数组（零拷贝）
    pub fn field_types(&self) -> &[u8] {
        let start = self.field_types_offset;
        let end = start + self.field_count;
        &self.data[start..end]
    }

    /// 获取指定字段变更的字段名
    pub fn field_name(&self, field_index: usize) -> Result<&[u8; 32], DecodeError> {
        if field_index >= self.field_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.field_names_offset + field_index * 32;
        let end = start + 32;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 获取指定字段变更的旧值
    pub fn old_value(&self, field_index: usize) -> Result<&[u8; 64], DecodeError> {
        if field_index >= self.field_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.old_values_offset + field_index * 64;
        let end = start + 64;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 获取指定字段变更的新值
    pub fn new_value(&self, field_index: usize) -> Result<&[u8; 64], DecodeError> {
        if field_index >= self.field_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.new_values_offset + field_index * 64;
        let end = start + 64;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 获取指定字段变更的旧值长度
    pub fn old_value_len(&self, field_index: usize) -> Result<u16, DecodeError> {
        if field_index >= self.field_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.old_value_lens_offset + field_index * 2;
        Ok(u16::from_le_bytes([self.data[start], self.data[start + 1]]))
    }

    /// 获取指定字段变更的新值长度
    pub fn new_value_len(&self, field_index: usize) -> Result<u16, DecodeError> {
        if field_index >= self.field_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.new_value_lens_offset + field_index * 2;
        Ok(u16::from_le_bytes([self.data[start], self.data[start + 1]]))
    }

    /// 获取指定字段变更的旧值切片（根据实际长度）
    pub fn old_value_bytes(&self, field_index: usize) -> Result<&[u8], DecodeError> {
        let old_value = self.old_value(field_index)?;
        let len = self.old_value_len(field_index)? as usize;
        Ok(&old_value[..len])
    }

    /// 获取指定字段变更的新值切片（根据实际长度）
    pub fn new_value_bytes(&self, field_index: usize) -> Result<&[u8], DecodeError> {
        let new_value = self.new_value(field_index)?;
        let len = self.new_value_len(field_index)? as usize;
        Ok(&new_value[..len])
    }

    /// 转换为 FieldChangeSoa（需要内存拷贝）
    pub fn to_soa(&self) -> FieldChangeSoa {
        let mut soa = FieldChangeSoa::with_capacity(self.field_count);

        // 拷贝字段名
        for i in 0..self.field_count {
            soa.field_names.push(*self.field_name(i).unwrap());
        }

        // 拷贝其他字段变更数据
        for i in 0..self.field_count {
            // old_values
            let start = self.old_values_offset + i * 64;
            let mut old_value = [0u8; 64];
            old_value.copy_from_slice(&self.data[start..start + 64]);
            soa.old_values.push(old_value);

            // old_value_lens
            let start = self.old_value_lens_offset + i * 2;
            let old_len = u16::from_le_bytes([self.data[start], self.data[start + 1]]);
            soa.old_value_lens.push(old_len);

            // new_values
            let start = self.new_values_offset + i * 64;
            let mut new_value = [0u8; 64];
            new_value.copy_from_slice(&self.data[start..start + 64]);
            soa.new_values.push(new_value);

            // new_value_lens
            let start = self.new_value_lens_offset + i * 2;
            let new_len = u16::from_le_bytes([self.data[start], self.data[start + 1]]);
            soa.new_value_lens.push(new_len);

            // field_types
            soa.field_types.push(self.field_types()[i]);
        }

        soa
    }
}

#[cfg(test)]
mod field_change_soa_tests {
    use super::super::entity_change_log::FieldChange;
    use super::*;

    #[test]
    fn test_fc_encode_decode_empty() {
        let encoder = FieldChangeSoaEncoder::new();
        let data = encoder.encode();

        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();
        assert_eq!(decoder.len(), 0);
        assert!(decoder.is_empty());
    }

    #[test]
    fn test_fc_encode_decode_single_field() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        encoder.push(fc);

        let data = encoder.encode();
        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.len(), 1);
        assert!(!decoder.is_empty());
    }

    #[test]
    fn test_fc_encode_decode_multiple_fields() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");

        // 添加 3 个字段变更
        let fc1 = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        let fc2 = FieldChange::new(field_name, b"120.0", b"150.0", 0);
        let fc3 = FieldChange::new(field_name, b"150.0", b"200.0", 0);
        encoder.push(fc1);
        encoder.push(fc2);
        encoder.push(fc3);

        let data = encoder.encode();
        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.len(), 3);
    }

    #[test]
    fn test_fc_encode_to_external_buffer() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        encoder.push(fc);

        let size = encoder.encoded_size();
        let mut buffer = vec![0u8; size];
        let written = encoder.encode_to(&mut buffer).unwrap();

        assert_eq!(written, size);

        let decoder = FieldChangeSoaDecoder::new(&buffer).unwrap();
        assert_eq!(decoder.len(), 1);
    }

    #[test]
    fn test_fc_roundtrip_conversion() {
        let mut encoder = FieldChangeSoaEncoder::new();

        for i in 0..3 {
            let field_name = FieldChange::field_name_from_str(&format!("field_{}", i));
            let fc = FieldChange::new(field_name, b"old", b"new", 0);
            encoder.push(fc);
        }

        let data = encoder.encode();
        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();
        let soa = decoder.to_soa();

        assert_eq!(soa.len(), 3);
    }

    #[test]
    fn test_fc_buffer_too_small() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        encoder.push(fc);

        let mut small_buffer = vec![0u8; 10];
        let result = encoder.encode_to(&mut small_buffer);

        assert!(matches!(result, Err(EncodeError::BufferTooSmall { .. })));
    }

    #[test]
    fn test_fc_invalid_magic() {
        let mut data = vec![0u8; FC_HEADER_SIZE];
        data[0..4].copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
        let result = FieldChangeSoaDecoder::new(&data);

        assert!(matches!(result, Err(DecodeError::InvalidMagic)));
    }

    #[test]
    fn test_fc_insufficient_data() {
        let data = vec![0u8; 10];
        let result = FieldChangeSoaDecoder::new(&data);

        assert!(matches!(result, Err(DecodeError::InsufficientData)));
    }
}