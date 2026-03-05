//! SBE 编解码核心 Trait 定义
//!
//! 参考 serde 的设计模式：
//! - `SbeEncode`/`SbeDecode` - 定义类型如何映射到 SBE 基础类型（类似 serde::Serialize/Deserialize）
//! - `SbeEncoder`/`SbeDecoder` - 提供 SBE 基础类型的编解码方法（类似 serde::Serializer/Deserializer）

/// 定义类型如何编码到 SBE（类似 serde::Serialize）
///
/// 用户类型实现此 trait 来定义如何将自己编码为 SBE 格式。
/// 通过调用 `SbeEncoder` 的方法，类型可以将自己映射到 SBE 基础类型。
///
/// # Example
/// ```ignore
/// impl SbeEncode for MyType {
///     fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
///         encoder.encode_u64(self.id)?;
///         encoder.encode_string(&self.name)
///     }
/// }
/// ```
pub trait SbeEncode {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error>;
}

/// 定义类型如何从 SBE 解码（类似 serde::Deserialize）
///
/// 用户类型实现此 trait 来定义如何从 SBE 格式解码。
/// 通过调用 `SbeDecoder` 的方法，类型可以从 SBE 基础类型重建自己。
///
/// # Example
/// ```ignore
/// impl SbeDecode for MyType {
///     fn sbe_decode<D: SbeDecoder>(decoder: &mut D) -> Result<Self, D::Error> {
///         Ok(MyType {
///             id: decoder.decode_u64()?,
///             name: decoder.decode_str()?.to_string(),
///         })
///     }
/// }
/// ```
pub trait SbeDecode: Sized {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error>;
}

/// SBE 编码器接口 - 提供基础类型的编码方法（类似 serde::Serializer）
///
/// 具体的编码器实现此 trait 来提供 SBE 基础类型的编码能力。
/// 用户类型通过调用这些方法将自己编码为 SBE 格式。
pub trait SbeEncoder {
    type Error;

    // ===== 整数类型 =====
    fn encode_i8(&mut self, v: i8) -> Result<(), Self::Error>;
    fn encode_u8(&mut self, v: u8) -> Result<(), Self::Error>;
    fn encode_i16(&mut self, v: i16) -> Result<(), Self::Error>;
    fn encode_u16(&mut self, v: u16) -> Result<(), Self::Error>;
    fn encode_i32(&mut self, v: i32) -> Result<(), Self::Error>;
    fn encode_u32(&mut self, v: u32) -> Result<(), Self::Error>;
    fn encode_i64(&mut self, v: i64) -> Result<(), Self::Error>;
    fn encode_u64(&mut self, v: u64) -> Result<(), Self::Error>;
    fn encode_i128(&mut self, v: i128) -> Result<(), Self::Error>;
    fn encode_u128(&mut self, v: u128) -> Result<(), Self::Error>;

    // ===== 浮点类型 =====
    fn encode_f32(&mut self, v: f32) -> Result<(), Self::Error>;
    fn encode_f64(&mut self, v: f64) -> Result<(), Self::Error>;

    // ===== 字符和布尔 =====
    fn encode_char(&mut self, v: char) -> Result<(), Self::Error>;
    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error>;

    // ===== 字符串和字节 =====
    /// 编码可变长度字符串（带长度前缀）
    fn encode_str(&mut self, v: &str) -> Result<(), Self::Error>;

    /// 编码可变长度字节数组（带长度前缀）
    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error>;

    // ===== 固定长度数组 =====
    /// 编码固定长度数组（无长度前缀）
    fn encode_array<const N: usize>(&mut self, v: &[u8; N]) -> Result<(), Self::Error>;
}

/// SBE 解码器接口 - 提供基础类型的解码方法（类似 serde::Deserializer）
///
/// 具体的解码器实现此 trait 来提供 SBE 基础类型的解码能力。
/// 用户类型通过调用这些方法从 SBE 格式重建自己。
///
/// 生命周期参数 `'de` 表示解码数据的生命周期，用于零拷贝解码。
pub trait SbeDecoder<'de> {
    type Error;

    // ===== 整数类型 =====
    fn decode_i8(&mut self) -> Result<i8, Self::Error>;
    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_i16(&mut self) -> Result<i16, Self::Error>;
    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_i32(&mut self) -> Result<i32, Self::Error>;
    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_i64(&mut self) -> Result<i64, Self::Error>;
    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_i128(&mut self) -> Result<i128, Self::Error>;
    fn decode_u128(&mut self) -> Result<u128, Self::Error>;

    // ===== 浮点类型 =====
    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;

    // ===== 字符和布尔 =====
    fn decode_char(&mut self) -> Result<char, Self::Error>;
    fn decode_bool(&mut self) -> Result<bool, Self::Error>;

    // ===== 字符串和字节 =====
    /// 解码可变长度字符串（读取长度前缀，零拷贝）
    fn decode_str(&mut self) -> Result<&'de str, Self::Error>;

    /// 解码可变长度字节数组（读取长度前缀）
    fn decode_bytes(&mut self) -> Result<Vec<u8>, Self::Error>;

    // ===== 固定长度数组 =====
    /// 解码固定长度数组（无长度前缀）
    fn decode_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;
}
