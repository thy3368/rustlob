//! ChangeLogEntrySoa 零拷贝、零分配二进制编解码器
//!
//! 为 ChangeLogEntryBase 的 SOA 版本提供高性能的二进制序列化和反序列化
//!
//! # 二进制格式
//!
//! ```text
//! Header (24 字节，8 字节对齐):
//!   - magic: [u8; 4] = b"CLSB" (ChangeLog SOA Base)
//!   - version: u8 = 1
//!   - padding: [u8; 3] (对齐到 8 字节)
//!   - entry_count: u64
//!   - total_field_changes: u64
//!
//! Entry Arrays:
//!   - timestamps: [u64; entry_count]
//!   - sequences: [u64; entry_count]
//!   - entity_ids: [[u8; 32]; entry_count]
//!   - entity_types: [u8; entry_count]
//!   - change_types: [u8; entry_count]
//!
//! Field Change Arrays:
//!   - field_names: [[u8; 32]; total_field_changes]
//!   - old_values: [[u8; 64]; total_field_changes]
//!   - old_value_lens: [u16; total_field_changes]
//!   - new_values: [[u8; 64]; total_field_changes]
//!   - new_value_lens: [u16; total_field_changes]
//!   - field_types: [u8; total_field_changes]
//!   - entry_offsets: [usize; entry_count]
//! ```

use super::changelog_entry_base::{ChangeLogEntrySoa, FieldChange, FieldChangeSoa};
use std::mem::size_of;

/// 魔数标识
const MAGIC: &[u8; 4] = b"CLSB";
/// 版本号
const VERSION: u8 = 1;
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
    soa: ChangeLogEntrySoa,
}

impl ChangeLogEntrySoaEncoder {
    /// 创建新的编码器
    pub fn new() -> Self {
        Self {
            soa: ChangeLogEntrySoa::new(),
        }
    }

    /// 创建预分配容量的编码器
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            soa: ChangeLogEntrySoa::with_capacity(capacity),
        }
    }

    /// 添加条目
    pub fn push(&mut self, entry: super::changelog_entry_base::ChangeLogEntryBase) {
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
        let total_field_changes = self.soa.field_changes.total_field_changes();

        HEADER_SIZE
            + entry_count * size_of::<u64>() // timestamps
            + entry_count * size_of::<u64>() // sequences
            + entry_count * 32 // entity_ids
            + entry_count * size_of::<u8>() // entity_types
            + entry_count * size_of::<u8>() // change_types
            + total_field_changes * 32 // field_names
            + total_field_changes * 64 // old_values
            + total_field_changes * size_of::<u16>() // old_value_lens
            + total_field_changes * 64 // new_values
            + total_field_changes * size_of::<u16>() // new_value_lens
            + total_field_changes * size_of::<u8>() // field_types
            + entry_count * size_of::<usize>() // entry_offsets
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
        let total_field_changes = self.soa.field_changes.total_field_changes();

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
        buffer[offset..offset + 8]
            .copy_from_slice(&(total_field_changes as u64).to_le_bytes());
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

        // 写入 entity_ids
        for entity_id in &self.soa.entity_ids {
            buffer[offset..offset + 32].copy_from_slice(entity_id);
            offset += 32;
        }

        // 写入 entity_types
        buffer[offset..offset + entry_count].copy_from_slice(&self.soa.entity_types);
        offset += entry_count;

        // 写入 change_types
        buffer[offset..offset + entry_count].copy_from_slice(&self.soa.change_types);
        offset += entry_count;

        // 写入字段变更数据
        let fc = &self.soa.field_changes;

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
        buffer[offset..offset + total_field_changes].copy_from_slice(&fc.field_types);
        offset += total_field_changes;

        // entry_offsets
        for &entry_offset in &fc.entry_offsets {
            buffer[offset..offset + size_of::<usize>()]
                .copy_from_slice(&entry_offset.to_le_bytes());
            offset += size_of::<usize>();
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
    total_field_changes: usize,
    timestamps_offset: usize,
    sequences_offset: usize,
    entity_ids_offset: usize,
    entity_types_offset: usize,
    change_types_offset: usize,
    field_names_offset: usize,
    old_values_offset: usize,
    old_value_lens_offset: usize,
    new_values_offset: usize,
    new_value_lens_offset: usize,
    field_types_offset: usize,
    entry_offsets_offset: usize,
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

        // 读取头部 (跳过 padding)
        let entry_count = u64::from_le_bytes([
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
        ]) as usize;
        let total_field_changes = u64::from_le_bytes([
            data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23],
        ]) as usize;

        // 计算各个数组的偏移量
        let mut offset = HEADER_SIZE;

        let timestamps_offset = offset;
        offset += entry_count * size_of::<u64>();

        let sequences_offset = offset;
        offset += entry_count * size_of::<u64>();

        let entity_ids_offset = offset;
        offset += entry_count * 32;

        let entity_types_offset = offset;
        offset += entry_count;

        let change_types_offset = offset;
        offset += entry_count;

        let field_names_offset = offset;
        offset += total_field_changes * 32;

        let old_values_offset = offset;
        offset += total_field_changes * 64;

        let old_value_lens_offset = offset;
        offset += total_field_changes * size_of::<u16>();

        let new_values_offset = offset;
        offset += total_field_changes * 64;

        let new_value_lens_offset = offset;
        offset += total_field_changes * size_of::<u16>();

        let field_types_offset = offset;
        offset += total_field_changes;

        let entry_offsets_offset = offset;
        offset += entry_count * size_of::<usize>();

        // 验证数据长度
        if data.len() < offset {
            return Err(DecodeError::InsufficientData);
        }

        Ok(Self {
            data,
            entry_count,
            total_field_changes,
            timestamps_offset,
            sequences_offset,
            entity_ids_offset,
            entity_types_offset,
            change_types_offset,
            field_names_offset,
            old_values_offset,
            old_value_lens_offset,
            new_values_offset,
            new_value_lens_offset,
            field_types_offset,
            entry_offsets_offset,
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

    /// 获取总字段变更数量
    pub fn total_field_changes(&self) -> usize {
        self.total_field_changes
    }

    /// 获取时间戳数组（零拷贝）
    pub fn timestamps(&self) -> &[u64] {
        let start = self.timestamps_offset;
        let end = start + self.entry_count * size_of::<u64>();
        let bytes = &self.data[start..end];
        unsafe {
            std::slice::from_raw_parts(bytes.as_ptr() as *const u64, self.entry_count)
        }
    }

    /// 获取序列号数组（零拷贝）
    pub fn sequences(&self) -> &[u64] {
        let start = self.sequences_offset;
        let end = start + self.entry_count * size_of::<u64>();
        let bytes = &self.data[start..end];
        unsafe {
            std::slice::from_raw_parts(bytes.as_ptr() as *const u64, self.entry_count)
        }
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
    pub fn entity_id(&self, index: usize) -> Result<&[u8; 32], DecodeError> {
        if index >= self.entry_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.entity_ids_offset + index * 32;
        let end = start + 32;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 获取字段类型数组（零拷贝）
    pub fn field_types(&self) -> &[u8] {
        let start = self.field_types_offset;
        let end = start + self.total_field_changes;
        &self.data[start..end]
    }

    /// 获取指定条目的字段变更范围
    pub fn entry_field_change_range(&self, entry_index: usize) -> Result<(usize, usize), DecodeError> {
        if entry_index >= self.entry_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start_offset = self.entry_offsets_offset + entry_index * size_of::<usize>();
        let start = usize::from_le_bytes(
            self.data[start_offset..start_offset + size_of::<usize>()]
                .try_into()
                .unwrap(),
        );

        let end = if entry_index + 1 < self.entry_count {
            let end_offset = self.entry_offsets_offset + (entry_index + 1) * size_of::<usize>();
            usize::from_le_bytes(
                self.data[end_offset..end_offset + size_of::<usize>()]
                    .try_into()
                    .unwrap(),
            )
        } else {
            self.total_field_changes
        };

        Ok((start, end))
    }

    /// 获取指定条目的字段变更数量
    pub fn entry_field_change_count(&self, entry_index: usize) -> Result<usize, DecodeError> {
        let (start, end) = self.entry_field_change_range(entry_index)?;
        Ok(end - start)
    }

    /// 获取字段变更解码器（零拷贝）
    ///
    /// 返回一个 `FieldChangeSoaDecoder`，可以用来访问所有字段变更数据
    pub fn field_changes(&self) -> FieldChangeSoaDecoder<'a> {
        FieldChangeSoaDecoder::from_embedded(
            self.data,
            self.entry_count,
            self.total_field_changes,
            self.field_names_offset,
            self.old_values_offset,
            self.old_value_lens_offset,
            self.new_values_offset,
            self.new_value_lens_offset,
            self.field_types_offset,
            self.entry_offsets_offset,
        )
    }

    /// 获取指定字段变更的字段名
    pub fn field_name(&self, field_index: usize) -> Result<&[u8; 32], DecodeError> {
        if field_index >= self.total_field_changes {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.field_names_offset + field_index * 32;
        let end = start + 32;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 转换为 ChangeLogEntrySoa（需要内存拷贝）
    pub fn to_soa(&self) -> ChangeLogEntrySoa {
        let mut soa = ChangeLogEntrySoa::with_capacity(self.entry_count);

        // 拷贝基础数据
        soa.timestamps.extend_from_slice(self.timestamps());
        soa.sequences.extend_from_slice(self.sequences());
        soa.entity_types.extend_from_slice(self.entity_types());
        soa.change_types.extend_from_slice(self.change_types());

        // 拷贝 entity_ids
        for i in 0..self.entry_count {
            soa.entity_ids.push(*self.entity_id(i).unwrap());
        }

        // 拷贝字段变更数据
        let fc = &mut soa.field_changes;

        // 拷贝字段名
        for i in 0..self.total_field_changes {
            fc.field_names.push(*self.field_name(i).unwrap());
        }

        // 拷贝其他字段变更数据
        for i in 0..self.total_field_changes {
            // old_values
            let start = self.old_values_offset + i * 64;
            let mut old_value = [0u8; 64];
            old_value.copy_from_slice(&self.data[start..start + 64]);
            fc.old_values.push(old_value);

            // old_value_lens
            let start = self.old_value_lens_offset + i * 2;
            let old_len = u16::from_le_bytes([self.data[start], self.data[start + 1]]);
            fc.old_value_lens.push(old_len);

            // new_values
            let start = self.new_values_offset + i * 64;
            let mut new_value = [0u8; 64];
            new_value.copy_from_slice(&self.data[start..start + 64]);
            fc.new_values.push(new_value);

            // new_value_lens
            let start = self.new_value_lens_offset + i * 2;
            let new_len = u16::from_le_bytes([self.data[start], self.data[start + 1]]);
            fc.new_value_lens.push(new_len);

            // field_types
            fc.field_types.push(self.field_types()[i]);
        }

        // 拷贝 entry_offsets
        for i in 0..self.entry_count {
            let start = self.entry_offsets_offset + i * size_of::<usize>();
            let offset = usize::from_le_bytes(
                self.data[start..start + size_of::<usize>()]
                    .try_into()
                    .unwrap(),
            );
            fc.entry_offsets.push(offset);
        }

        soa
    }
}

#[cfg(test)]
mod tests {
    use super::super::changelog_entry_base::{ChangeLogEntryBase, FieldChange};
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

        let entity_id = ChangeLogEntryBase::entity_id_from_str("order_123");
        let entry = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 0);
        encoder.push(entry);

        let data = encoder.encode();
        let decoder = ChangeLogEntrySoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.len(), 1);
        assert_eq!(decoder.timestamps()[0], 1000);
        assert_eq!(decoder.sequences()[0], 1);
        assert_eq!(decoder.entity_types()[0], 1);
        assert_eq!(decoder.change_types()[0], 0);
    }

    #[test]
    fn test_encode_decode_with_field_changes() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        let entity_id = ChangeLogEntryBase::entity_id_from_str("order_123");
        let mut entry = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 1);

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        entry.add_field_change(fc);

        encoder.push(entry);

        let data = encoder.encode();
        let decoder = ChangeLogEntrySoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.len(), 1);
        assert_eq!(decoder.total_field_changes(), 1);
        assert_eq!(decoder.entry_field_change_count(0).unwrap(), 1);
    }

    #[test]
    fn test_encode_to_external_buffer() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        let entity_id = ChangeLogEntryBase::entity_id_from_str("order_123");
        let entry = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 0);
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
            let entity_id = ChangeLogEntryBase::entity_id_from_str(&format!("order_{}", i));
            let mut entry = ChangeLogEntryBase::new(1000 + i, i, entity_id, 1, 1);

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
        assert_eq!(soa.field_changes.total_field_changes(), 5);
    }

    #[test]
    fn test_buffer_too_small() {
        let mut encoder = ChangeLogEntrySoaEncoder::new();

        let entity_id = ChangeLogEntryBase::entity_id_from_str("order_123");
        let entry = ChangeLogEntryBase::new(1000, 1, entity_id, 1, 0);
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
    pub fn with_capacity(field_capacity: usize, entry_capacity: usize) -> Self {
        Self {
            soa: FieldChangeSoa::with_capacity(field_capacity, entry_capacity),
        }
    }

    /// 添加一批字段变更（对应一个条目）
    pub fn push_batch(&mut self, field_changes: Vec<FieldChange>) {
        self.soa.push_batch(field_changes);
    }

    /// 添加空批次
    pub fn push_empty_batch(&mut self) {
        self.soa.push_empty_batch();
    }

    /// 获取条目数量
    pub fn entry_count(&self) -> usize {
        self.soa.entry_count()
    }

    /// 获取总字段变更数量
    pub fn total_field_changes(&self) -> usize {
        self.soa.total_field_changes()
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
        let entry_count = self.soa.entry_count();
        let total_field_changes = self.soa.total_field_changes();

        FC_HEADER_SIZE
            + total_field_changes * 32 // field_names
            + total_field_changes * 64 // old_values
            + total_field_changes * size_of::<u16>() // old_value_lens
            + total_field_changes * 64 // new_values
            + total_field_changes * size_of::<u16>() // new_value_lens
            + total_field_changes * size_of::<u8>() // field_types
            + entry_count * size_of::<usize>() // entry_offsets
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

        let entry_count = self.soa.entry_count();
        let total_field_changes = self.soa.total_field_changes();

        let mut offset = 0;

        // 写入头部
        buffer[offset..offset + 4].copy_from_slice(FC_MAGIC);
        offset += 4;
        buffer[offset] = FC_VERSION;
        offset += 1;
        // padding (3 字节)
        buffer[offset..offset + 3].fill(0);
        offset += 3;
        buffer[offset..offset + 8].copy_from_slice(&(entry_count as u64).to_le_bytes());
        offset += 8;
        buffer[offset..offset + 8]
            .copy_from_slice(&(total_field_changes as u64).to_le_bytes());
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
        buffer[offset..offset + total_field_changes].copy_from_slice(&fc.field_types);
        offset += total_field_changes;

        // entry_offsets
        for &entry_offset in &fc.entry_offsets {
            buffer[offset..offset + size_of::<usize>()]
                .copy_from_slice(&entry_offset.to_le_bytes());
            offset += size_of::<usize>();
        }

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
pub struct FieldChangeSoaDecoder<'a> {
    data: &'a [u8],
    entry_count: usize,
    total_field_changes: usize,
    field_names_offset: usize,
    old_values_offset: usize,
    old_value_lens_offset: usize,
    new_values_offset: usize,
    new_value_lens_offset: usize,
    field_types_offset: usize,
    entry_offsets_offset: usize,
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
        let entry_count = u64::from_le_bytes([
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
        ]) as usize;
        let total_field_changes = u64::from_le_bytes([
            data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23],
        ]) as usize;

        // 计算各个数组的偏移量
        let mut offset = FC_HEADER_SIZE;

        let field_names_offset = offset;
        offset += total_field_changes * 32;

        let old_values_offset = offset;
        offset += total_field_changes * 64;

        let old_value_lens_offset = offset;
        offset += total_field_changes * size_of::<u16>();

        let new_values_offset = offset;
        offset += total_field_changes * 64;

        let new_value_lens_offset = offset;
        offset += total_field_changes * size_of::<u16>();

        let field_types_offset = offset;
        offset += total_field_changes;

        let entry_offsets_offset = offset;
        offset += entry_count * size_of::<usize>();

        // 验证数据长度
        if data.len() < offset {
            return Err(DecodeError::InsufficientData);
        }

        Ok(Self {
            data,
            entry_count,
            total_field_changes,
            field_names_offset,
            old_values_offset,
            old_value_lens_offset,
            new_values_offset,
            new_value_lens_offset,
            field_types_offset,
            entry_offsets_offset,
        })
    }

    /// 从嵌入的数据创建解码器（用于 ChangeLogEntrySoaDecoder）
    pub fn from_embedded(
        data: &'a [u8],
        entry_count: usize,
        total_field_changes: usize,
        field_names_offset: usize,
        old_values_offset: usize,
        old_value_lens_offset: usize,
        new_values_offset: usize,
        new_value_lens_offset: usize,
        field_types_offset: usize,
        entry_offsets_offset: usize,
    ) -> Self {
        Self {
            data,
            entry_count,
            total_field_changes,
            field_names_offset,
            old_values_offset,
            old_value_lens_offset,
            new_values_offset,
            new_value_lens_offset,
            field_types_offset,
            entry_offsets_offset,
        }
    }

    /// 获取条目数量
    pub fn entry_count(&self) -> usize {
        self.entry_count
    }

    /// 获取总字段变更数量
    pub fn total_field_changes(&self) -> usize {
        self.total_field_changes
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.entry_count == 0
    }

    /// 获取字段类型数组（零拷贝）
    pub fn field_types(&self) -> &[u8] {
        let start = self.field_types_offset;
        let end = start + self.total_field_changes;
        &self.data[start..end]
    }

    /// 获取指定字段变更的字段名
    pub fn field_name(&self, field_index: usize) -> Result<&[u8; 32], DecodeError> {
        if field_index >= self.total_field_changes {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.field_names_offset + field_index * 32;
        let end = start + 32;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 获取指定字段变更的旧值
    pub fn old_value(&self, field_index: usize) -> Result<&[u8; 64], DecodeError> {
        if field_index >= self.total_field_changes {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.old_values_offset + field_index * 64;
        let end = start + 64;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 获取指定字段变更的新值
    pub fn new_value(&self, field_index: usize) -> Result<&[u8; 64], DecodeError> {
        if field_index >= self.total_field_changes {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.new_values_offset + field_index * 64;
        let end = start + 64;
        Ok(self.data[start..end].try_into().unwrap())
    }

    /// 获取指定字段变更的旧值长度
    pub fn old_value_len(&self, field_index: usize) -> Result<u16, DecodeError> {
        if field_index >= self.total_field_changes {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start = self.old_value_lens_offset + field_index * 2;
        Ok(u16::from_le_bytes([self.data[start], self.data[start + 1]]))
    }

    /// 获取指定字段变更的新值长度
    pub fn new_value_len(&self, field_index: usize) -> Result<u16, DecodeError> {
        if field_index >= self.total_field_changes {
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

    /// 获取指定条目的字段变更范围
    pub fn entry_field_change_range(
        &self,
        entry_index: usize,
    ) -> Result<(usize, usize), DecodeError> {
        if entry_index >= self.entry_count {
            return Err(DecodeError::IndexOutOfBounds);
        }

        let start_offset = self.entry_offsets_offset + entry_index * size_of::<usize>();
        let start = usize::from_le_bytes(
            self.data[start_offset..start_offset + size_of::<usize>()]
                .try_into()
                .unwrap(),
        );

        let end = if entry_index + 1 < self.entry_count {
            let end_offset = self.entry_offsets_offset + (entry_index + 1) * size_of::<usize>();
            usize::from_le_bytes(
                self.data[end_offset..end_offset + size_of::<usize>()]
                    .try_into()
                    .unwrap(),
            )
        } else {
            self.total_field_changes
        };

        Ok((start, end))
    }

    /// 获取指定条目的字段变更数量
    pub fn entry_field_change_count(&self, entry_index: usize) -> Result<usize, DecodeError> {
        let (start, end) = self.entry_field_change_range(entry_index)?;
        Ok(end - start)
    }

    /// 转换为 FieldChangeSoa（需要内存拷贝）
    pub fn to_soa(&self) -> FieldChangeSoa {
        let mut soa =
            FieldChangeSoa::with_capacity(self.total_field_changes, self.entry_count);

        // 拷贝字段名
        for i in 0..self.total_field_changes {
            soa.field_names.push(*self.field_name(i).unwrap());
        }

        // 拷贝其他字段变更数据
        for i in 0..self.total_field_changes {
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

        // 拷贝 entry_offsets
        for i in 0..self.entry_count {
            let start = self.entry_offsets_offset + i * size_of::<usize>();
            let offset = usize::from_le_bytes(
                self.data[start..start + size_of::<usize>()]
                    .try_into()
                    .unwrap(),
            );
            soa.entry_offsets.push(offset);
        }

        soa
    }
}

#[cfg(test)]
mod field_change_soa_tests {
    use super::super::changelog_entry_base::FieldChange;
    use super::*;

    #[test]
    fn test_fc_encode_decode_empty() {
        let encoder = FieldChangeSoaEncoder::new();
        let data = encoder.encode();

        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();
        assert_eq!(decoder.entry_count(), 0);
        assert_eq!(decoder.total_field_changes(), 0);
        assert!(decoder.is_empty());
    }

    #[test]
    fn test_fc_encode_decode_single_batch() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        encoder.push_batch(vec![fc]);

        let data = encoder.encode();
        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.entry_count(), 1);
        assert_eq!(decoder.total_field_changes(), 1);
        assert_eq!(decoder.entry_field_change_count(0).unwrap(), 1);
    }

    #[test]
    fn test_fc_encode_decode_multiple_batches() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");

        // 批次 0: 2 个字段变更
        let fc1 = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        let fc2 = FieldChange::new(field_name, b"120.0", b"150.0", 0);
        encoder.push_batch(vec![fc1, fc2]);

        // 批次 1: 1 个字段变更
        let fc3 = FieldChange::new(field_name, b"150.0", b"200.0", 0);
        encoder.push_batch(vec![fc3]);

        // 批次 2: 0 个字段变更
        encoder.push_empty_batch();

        let data = encoder.encode();
        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();

        assert_eq!(decoder.entry_count(), 3);
        assert_eq!(decoder.total_field_changes(), 3);
        assert_eq!(decoder.entry_field_change_count(0).unwrap(), 2);
        assert_eq!(decoder.entry_field_change_count(1).unwrap(), 1);
        assert_eq!(decoder.entry_field_change_count(2).unwrap(), 0);
    }

    #[test]
    fn test_fc_encode_to_external_buffer() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        encoder.push_batch(vec![fc]);

        let size = encoder.encoded_size();
        let mut buffer = vec![0u8; size];
        let written = encoder.encode_to(&mut buffer).unwrap();

        assert_eq!(written, size);

        let decoder = FieldChangeSoaDecoder::new(&buffer).unwrap();
        assert_eq!(decoder.entry_count(), 1);
        assert_eq!(decoder.total_field_changes(), 1);
    }

    #[test]
    fn test_fc_roundtrip_conversion() {
        let mut encoder = FieldChangeSoaEncoder::new();

        for i in 0..3 {
            let field_name = FieldChange::field_name_from_str(&format!("field_{}", i));
            let fc = FieldChange::new(field_name, b"old", b"new", 0);
            encoder.push_batch(vec![fc]);
        }

        let data = encoder.encode();
        let decoder = FieldChangeSoaDecoder::new(&data).unwrap();
        let soa = decoder.to_soa();

        assert_eq!(soa.entry_count(), 3);
        assert_eq!(soa.total_field_changes(), 3);
    }

    #[test]
    fn test_fc_buffer_too_small() {
        let mut encoder = FieldChangeSoaEncoder::new();

        let field_name = FieldChange::field_name_from_str("price");
        let fc = FieldChange::new(field_name, b"100.0", b"120.0", 0);
        encoder.push_batch(vec![fc]);

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