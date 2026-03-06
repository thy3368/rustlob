//! 简单的 SBE 编码器和解码器实现
//!
//! 提供基于 `WriteBuf` 和 `ReadBuf` 的简单编解码器实现。

use crate::codec::codec::{SbeDecoder, SbeEncoder};
use crate::{ReadBuf, WriteBuf};

/// 简单的编解码错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodecError;

impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Codec error")
    }
}

impl std::error::Error for CodecError {}

// ============================================================================
// SimpleEncoder - 简单的 SBE 编码器实现
// ============================================================================

/// 简单的 SBE 编码器
///
/// 使用 `WriteBuf` 进行编码，按顺序写入字段。
pub struct SimpleEncoder<'a> {
    buf: WriteBuf<'a>,
    offset: usize,
}

impl<'a> SimpleEncoder<'a> {
    /// 创建新的编码器
    pub fn new(buf: WriteBuf<'a>) -> Self {
        Self { buf, offset: 0 }
    }

    /// 完成编码，返回编码的总长度
    pub fn finish(self) -> usize {
        self.offset
    }

    /// 获取当前偏移量
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl<'a> SbeEncoder for SimpleEncoder<'a> {
    type Error = CodecError;

    fn encode_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        self.buf.put_i8_at(self.offset, v);
        self.offset += 1;
        Ok(())
    }

    fn encode_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.buf.put_u8_at(self.offset, v);
        self.offset += 1;
        Ok(())
    }

    fn encode_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.buf.put_i16_at(self.offset, v);
        self.offset += 2;
        Ok(())
    }

    fn encode_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        self.buf.put_u16_at(self.offset, v);
        self.offset += 2;
        Ok(())
    }

    fn encode_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.buf.put_i32_at(self.offset, v);
        self.offset += 4;
        Ok(())
    }

    fn encode_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.buf.put_u32_at(self.offset, v);
        self.offset += 4;
        Ok(())
    }

    fn encode_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        self.buf.put_i64_at(self.offset, v);
        self.offset += 8;
        Ok(())
    }

    fn encode_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        self.buf.put_u64_at(self.offset, v);
        self.offset += 8;
        Ok(())
    }

    fn encode_i128(&mut self, v: i128) -> Result<(), Self::Error> {
        self.buf.put_i128_at(self.offset, v);
        self.offset += 16;
        Ok(())
    }

    fn encode_u128(&mut self, v: u128) -> Result<(), Self::Error> {
        self.buf.put_u128_at(self.offset, v);
        self.offset += 16;
        Ok(())
    }

    fn encode_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.buf.put_f32_at(self.offset, v);
        self.offset += 4;
        Ok(())
    }

    fn encode_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        self.buf.put_f64_at(self.offset, v);
        self.offset += 8;
        Ok(())
    }

    fn encode_char(&mut self, v: char) -> Result<(), Self::Error> {
        self.buf.put_u8_at(self.offset, v as u8);
        self.offset += 1;
        Ok(())
    }

    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.buf.put_u8_at(self.offset, if v { 1 } else { 0 });
        self.offset += 1;
        Ok(())
    }

    fn encode_str(&mut self, v: &str) -> Result<(), Self::Error> {
        let bytes = v.as_bytes();
        self.buf.put_u16_at(self.offset, bytes.len() as u16);
        self.offset += 2;
        self.buf.put_slice_at(self.offset, bytes);
        self.offset += bytes.len();
        Ok(())
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        self.buf.put_u16_at(self.offset, v.len() as u16);
        self.offset += 2;
        self.buf.put_slice_at(self.offset, v);
        self.offset += v.len();
        Ok(())
    }

    fn encode_array<const N: usize>(&mut self, v: &[u8; N]) -> Result<(), Self::Error> {
        self.buf.put_slice_at(self.offset, v);
        self.offset += N;
        Ok(())
    }
}

// ============================================================================
// SimpleDecoder - 简单的 SBE 解码器实现
// ============================================================================

/// 简单的 SBE 解码器
///
/// 使用 `ReadBuf` 进行解码，按顺序读取字段。
pub struct SimpleDecoder<'a> {
    buf: ReadBuf<'a>,
    offset: usize,
}

impl<'a> SimpleDecoder<'a> {
    /// 创建新的解码器
    pub fn new(buf: ReadBuf<'a>) -> Self {
        Self { buf, offset: 0 }
    }

    /// 获取当前偏移量
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl<'a> SbeDecoder<'a> for SimpleDecoder<'a> {
    type Error = CodecError;

    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        let v = self.buf.get_i8_at(self.offset);
        self.offset += 1;
        Ok(v)
    }

    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        let v = self.buf.get_u8_at(self.offset);
        self.offset += 1;
        Ok(v)
    }

    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        let v = self.buf.get_i16_at(self.offset);
        self.offset += 2;
        Ok(v)
    }

    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        let v = self.buf.get_u16_at(self.offset);
        self.offset += 2;
        Ok(v)
    }

    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        let v = self.buf.get_i32_at(self.offset);
        self.offset += 4;
        Ok(v)
    }

    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        let v = self.buf.get_u32_at(self.offset);
        self.offset += 4;
        Ok(v)
    }

    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        let v = self.buf.get_i64_at(self.offset);
        self.offset += 8;
        Ok(v)
    }

    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        let v = self.buf.get_u64_at(self.offset);
        self.offset += 8;
        Ok(v)
    }

    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        let v = self.buf.get_i128_at(self.offset);
        self.offset += 16;
        Ok(v)
    }

    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        let v = self.buf.get_u128_at(self.offset);
        self.offset += 16;
        Ok(v)
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        let v = self.buf.get_f32_at(self.offset);
        self.offset += 4;
        Ok(v)
    }

    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        let v = self.buf.get_f64_at(self.offset);
        self.offset += 8;
        Ok(v)
    }

    fn decode_char(&mut self) -> Result<char, Self::Error> {
        let v = self.buf.get_u8_at(self.offset);
        self.offset += 1;
        Ok(v as char)
    }

    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        let v = self.buf.get_u8_at(self.offset);
        self.offset += 1;
        Ok(v != 0)
    }

    fn decode_str(&mut self) -> Result<&'a str, Self::Error> {
        let len = self.buf.get_u16_at(self.offset) as usize;
        self.offset += 2;
        let bytes = self.buf.get_slice_at(self.offset, len);
        self.offset += len;
        std::str::from_utf8(bytes).map_err(|_| CodecError)
    }

    fn decode_bytes(&mut self) -> Result<&'a [u8], Self::Error> {
        let len = self.buf.get_u16_at(self.offset) as usize;
        self.offset += 2;
        let bytes = self.buf.get_slice_at(self.offset, len);
        self.offset += len;
        Ok(bytes)
    }

    fn decode_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let bytes = self.buf.get_slice_at(self.offset, N);
        self.offset += N;
        let mut arr = [0u8; N];
        arr.copy_from_slice(bytes);
        Ok(arr)
    }
}
