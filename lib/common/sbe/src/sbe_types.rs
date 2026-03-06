//! SBE 复合类型定义
//!
//! 提供符合 SBE 官方规范的复合类型，包括：
//! - Decimal - mantissa + exponent（定点数）
//! - Timestamp - UTC 纳秒时间戳
//! - Composite - 嵌套结构体
//! - Set/Bitset - 位集合

// ============================================================================
// Decimal 类型 - mantissa + exponent（符合 SBE 官方定义）
// ============================================================================

use crate::codec::codec::{SbeDecode, SbeDecoder, SbeEncode, SbeEncoder};

/// Decimal 类型 - 定点数表示
///
/// 符合 SBE 官方定义，使用 mantissa（尾数）和 exponent（指数）表示定点数。
/// 实际值 = mantissa × 10^exponent
///
/// # Example
/// ```ignore
/// // 表示 50000.12345678
/// let price = Decimal {
///     mantissa: 5000012345678,
///     exponent: -8,
/// };
/// // 实际值 = 5000012345678 × 10^(-8) = 50000.12345678
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decimal {
    pub mantissa: i64,
    pub exponent: i8,
}

impl Decimal {
    /// 创建新的 Decimal
    pub fn new(mantissa: i64, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }

    /// 获取尾数
    pub fn mantissa(&self) -> i64 {
        self.mantissa
    }

    /// 获取指数
    pub fn exponent(&self) -> i8 {
        self.exponent
    }

    /// 转换为 f64（可能损失精度）
    pub fn to_f64(self) -> f64 {
        self.mantissa as f64 * 10f64.powi(self.exponent as i32)
    }

    /// 从 f64 创建 Decimal（指定指数）
    pub fn from_f64(value: f64, exponent: i8) -> Self {
        let mantissa = (value * 10f64.powi(-exponent as i32)).round() as i64;
        Self { mantissa, exponent }
    }
}

impl SbeEncode for Decimal {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_i64(self.mantissa)?;
        encoder.encode_i8(self.exponent)
    }
}

impl SbeDecode for Decimal {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(Decimal { mantissa: decoder.decode_i64()?, exponent: decoder.decode_i8()? })
    }
}

impl From<Decimal> for f64 {
    fn from(d: Decimal) -> f64 {
        d.to_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_roundtrip() {
        let d = Decimal::new(12345, -2);
        assert_eq!(d.mantissa(), 12345);
        assert_eq!(d.exponent(), -2);
        assert_eq!(d.to_f64(), 123.45);
    }

    #[test]
    fn test_decimal_from_f64() {
        let d = Decimal::from_f64(123.45, -2);
        assert_eq!(d.mantissa(), 12345);
        assert_eq!(d.exponent(), -2);
    }
}

// ============================================================================
// Timestamp 类型 - UTC 纳秒时间戳（符合 SBE 官方定义）
// ============================================================================

/// Timestamp 类型 - UTC 纳秒时间戳
///
/// 符合 SBE 官方定义，表示自 Unix epoch（1970-01-01 00:00:00 UTC）以来的纳秒数。
///
/// # Example
/// ```ignore
/// // 2024-01-01 00:00:00 UTC
/// let ts = Timestamp(1704067200000000000);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timestamp(pub i64);

impl Timestamp {
    /// 创建新的 Timestamp
    pub fn new(nanos: i64) -> Self {
        Self(nanos)
    }

    /// 获取纳秒值
    pub fn as_nanos(self) -> i64 {
        self.0
    }

    /// 转换为毫秒
    pub fn as_millis(self) -> i64 {
        self.0 / 1_000_000
    }

    /// 转换为秒
    pub fn as_secs(self) -> i64 {
        self.0 / 1_000_000_000
    }
}

impl SbeEncode for Timestamp {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_i64(self.0)
    }
}

impl SbeDecode for Timestamp {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(Timestamp(decoder.decode_i64()?))
    }
}
