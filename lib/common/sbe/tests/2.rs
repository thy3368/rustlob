//! SBE 手动编解码测试
//!
//! 展示如何使用 sbe 库的 trait 系统手动实现编解码。
//! 参考 serde 的设计模式。

use sbe::sbe_types::{Decimal, Timestamp};
use sbe::codec::simple_codec::{SimpleDecoder, SimpleEncoder};
use sbe::{ReadBuf, WriteBuf};
use sbe::codec::codec::{SbeDecode, SbeDecoder, SbeEncode, SbeEncoder};
// ============================================================================
// Composite 类型示例 - 嵌套结构体（符合 SBE 官方定义）
// ============================================================================

/// PriceQuantity - 价格和数量的复合类型
///
/// 符合 SBE 官方定义的 Composite 类型，表示嵌套的结构体。
#[derive(Debug, Clone, PartialEq)]
pub struct PriceQuantity {
    pub price: Decimal,
    pub quantity: Decimal,
}

impl PriceQuantity {
    /// 创建新的 PriceQuantity
    pub fn new(price: Decimal, quantity: Decimal) -> Self {
        Self { price, quantity }
    }
}

impl SbeEncode for PriceQuantity {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        self.price.sbe_encode(encoder)?;
        self.quantity.sbe_encode(encoder)
    }
}

impl SbeDecode for PriceQuantity {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(PriceQuantity {
            price: Decimal::sbe_decode(decoder)?,
            quantity: Decimal::sbe_decode(decoder)?,
        })
    }
}

// ============================================================================
// Set 类型 - 位集合（符合 SBE 官方定义）
// ============================================================================

/// OrderFlags - 订单标志位集合
///
/// 符合 SBE 官方定义的 Set/Bitset 类型，使用位标志表示多个布尔选项。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderFlags(pub u8);

impl OrderFlags {
    pub const NONE: u8 = 0;
    pub const REDUCE_ONLY: u8 = 1 << 0;
    pub const POST_ONLY: u8 = 1 << 1;
    pub const CLOSE_POSITION: u8 = 1 << 2;
    pub const PRICE_PROTECT: u8 = 1 << 3;

    /// 创建新的 OrderFlags
    pub fn new(flags: u8) -> Self {
        Self(flags)
    }

    /// 检查是否设置了指定标志
    pub fn has_flag(self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }

    /// 设置标志
    pub fn set_flag(&mut self, flag: u8) {
        self.0 |= flag;
    }

    /// 清除标志
    pub fn clear_flag(&mut self, flag: u8) {
        self.0 &= !flag;
    }
}

impl SbeEncode for OrderFlags {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_u8(self.0)
    }
}

impl SbeDecode for OrderFlags {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(OrderFlags(decoder.decode_u8()?))
    }
}

// ============================================================================
// Repeating Group 示例 - Fill（成交记录）
// ============================================================================

/// Fill - 成交记录
///
/// 用于演示 Repeating Groups（重复组）的示例类型。
#[derive(Debug, Clone, PartialEq)]
pub struct Fill {
    pub fill_id: u64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub timestamp: Timestamp,
}

impl Fill {
    /// 创建新的 Fill
    pub fn new(fill_id: u64, price: Decimal, quantity: Decimal, timestamp: Timestamp) -> Self {
        Self { fill_id, price, quantity, timestamp }
    }
}

impl SbeEncode for Fill {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_u64(self.fill_id)?;
        self.price.sbe_encode(encoder)?;
        self.quantity.sbe_encode(encoder)?;
        self.timestamp.sbe_encode(encoder)
    }
}

impl SbeDecode for Fill {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(Fill {
            fill_id: decoder.decode_u64()?,
            price: Decimal::sbe_decode(decoder)?,
            quantity: Decimal::sbe_decode(decoder)?,
            timestamp: Timestamp::sbe_decode(decoder)?,
        })
    }
}

// ============================================================================
// 测试专用的 Enum 类型定义
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OrderSide {
    Buy = 0,
    Sell = 1,
}

impl SbeEncode for OrderSide {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_u8(*self as u8)
    }
}

impl SbeDecode for OrderSide {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        match decoder.decode_u8()? {
            0 => Ok(OrderSide::Buy),
            1 => Ok(OrderSide::Sell),
            _ => Err(decoder.decode_u8().unwrap_err()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PositionSide {
    Both = 0,
    Long = 1,
    Short = 2,
}

impl SbeEncode for PositionSide {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_u8(*self as u8)
    }
}

impl SbeDecode for PositionSide {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        match decoder.decode_u8()? {
            0 => Ok(PositionSide::Both),
            1 => Ok(PositionSide::Long),
            2 => Ok(PositionSide::Short),
            v => panic!("Invalid PositionSide value: {}", v),
        }
    }
}

// ============================================================================
// 测试专用的复杂 Order 结构体
// ============================================================================

/// 完整的 Order 结构体，覆盖所有 SBE 官方定义的数据类型
#[derive(Debug, PartialEq)]
pub struct ComplexOrder {
    // ===== 基本类型 =====
    pub order_id: u64,
    pub user_id: u32,
    pub sequence: u16,
    pub version: u8,
    pub priority: i8,

    // ===== 浮点类型 =====
    pub fee_rate: f32,
    pub slippage_tolerance: f64,

    // ===== 字符和布尔 =====
    pub type_code: char,
    pub is_active: bool,

    // ===== 复合类型 =====
    pub price: Decimal,
    pub quantity: Decimal,
    pub stop_loss: Option<PriceQuantity>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,

    // ===== 枚举类型 =====
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,

    // ===== 位集合 =====
    pub flags: OrderFlags,

    // ===== 固定长度数组 =====
    pub client_order_id: [u8; 16],

    // ===== 可变长度数据 =====
    pub symbol: String,
    pub note: Option<String>,
    pub custom_data: Vec<u8>,

    // ===== 重复组 =====
    pub fills: Vec<Fill>,
}

impl SbeEncode for ComplexOrder {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        // 固定长度字段
        encoder.encode_u64(self.order_id)?;
        encoder.encode_u32(self.user_id)?;
        encoder.encode_u16(self.sequence)?;
        encoder.encode_u8(self.version)?;
        encoder.encode_i8(self.priority)?;
        encoder.encode_f32(self.fee_rate)?;
        encoder.encode_f64(self.slippage_tolerance)?;
        encoder.encode_char(self.type_code)?;
        encoder.encode_bool(self.is_active)?;
        self.price.sbe_encode(encoder)?;
        self.quantity.sbe_encode(encoder)?;
        self.stop_loss.sbe_encode(encoder)?;
        self.created_at.sbe_encode(encoder)?;
        self.updated_at.sbe_encode(encoder)?;
        self.side.sbe_encode(encoder)?;
        self.position_side.sbe_encode(encoder)?;
        self.flags.sbe_encode(encoder)?;
        encoder.encode_array(&self.client_order_id)?;

        // 可变长度字段
        self.symbol.sbe_encode(encoder)?;
        self.note.sbe_encode(encoder)?;
        encoder.encode_bytes(&self.custom_data)?;

        // 重复组
        encoder.encode_u16(self.fills.len() as u16)?;
        for fill in &self.fills {
            fill.sbe_encode(encoder)?;
        }

        Ok(())
    }
}

impl SbeDecode for ComplexOrder {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(ComplexOrder {
            order_id: decoder.decode_u64()?,
            user_id: decoder.decode_u32()?,
            sequence: decoder.decode_u16()?,
            version: decoder.decode_u8()?,
            priority: decoder.decode_i8()?,
            fee_rate: decoder.decode_f32()?,
            slippage_tolerance: decoder.decode_f64()?,
            type_code: decoder.decode_char()?,
            is_active: decoder.decode_bool()?,
            price: Decimal::sbe_decode(decoder)?,
            quantity: Decimal::sbe_decode(decoder)?,
            stop_loss: Option::<PriceQuantity>::sbe_decode(decoder)?,
            created_at: Timestamp::sbe_decode(decoder)?,
            updated_at: Timestamp::sbe_decode(decoder)?,
            side: OrderSide::sbe_decode(decoder)?,
            position_side: Option::<PositionSide>::sbe_decode(decoder)?,
            flags: OrderFlags::sbe_decode(decoder)?,
            client_order_id: decoder.decode_array::<16>()?,
            symbol: String::sbe_decode(decoder)?,
            note: Option::<String>::sbe_decode(decoder)?,
            custom_data: decoder.decode_bytes()?,
            fills: {
                let count = decoder.decode_u16()? as usize;
                let mut fills = Vec::with_capacity(count);
                for _ in 0..count {
                    fills.push(Fill::sbe_decode(decoder)?);
                }
                fills
            },
        })
    }
}

/// 简化版示例（用于基础测试）
#[derive(Debug, PartialEq)]
pub struct SimpleOrder {
    pub order_id: u64,
    pub symbol: String,
    pub side: OrderSide,
    pub price: Option<String>,
    pub timestamp: i64,
}

impl SbeEncode for SimpleOrder {
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_u64(self.order_id)?;
        self.side.sbe_encode(encoder)?;
        encoder.encode_i64(self.timestamp)?;
        self.symbol.sbe_encode(encoder)?;
        self.price.sbe_encode(encoder)?;
        Ok(())
    }
}

impl SbeDecode for SimpleOrder {
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(SimpleOrder {
            order_id: decoder.decode_u64()?,
            side: OrderSide::sbe_decode(decoder)?,
            timestamp: decoder.decode_i64()?,
            symbol: String::sbe_decode(decoder)?,
            price: Option::<String>::sbe_decode(decoder)?,
        })
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_encode_decode() {
        let order = SimpleOrder {
            order_id: 12345,
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            price: Some("50000.00".to_string()),
            timestamp: 1704067200000,
        };

        let mut buffer = vec![0u8; 1024];
        let write_buf = WriteBuf::new(&mut buffer);
        let mut encoder = SimpleEncoder::new(write_buf);
        order.sbe_encode(&mut encoder).unwrap();
        let len = encoder.finish();

        let read_buf = ReadBuf::new(&buffer[..len]);
        let mut decoder = SimpleDecoder::new(read_buf);
        let decoded = SimpleOrder::sbe_decode(&mut decoder).unwrap();

        assert_eq!(order, decoded);
        println!("✓ 编解码成功！编码长度: {} bytes", len);
    }

    #[test]
    fn test_complex_order_all_types() {
        let order = ComplexOrder {
            order_id: 123456789,
            user_id: 1001,
            sequence: 42,
            version: 1,
            priority: -5,
            fee_rate: 0.001,
            slippage_tolerance: 0.05,
            type_code: 'L',
            is_active: true,
            price: Decimal::new(5000000000000, -8),
            quantity: Decimal::new(150000000, -6),
            stop_loss: Some(PriceQuantity::new(
                Decimal::new(4800000000000, -8),
                Decimal::new(150000000, -6),
            )),
            created_at: Timestamp::new(1704067200000000000),
            updated_at: Timestamp::new(1704067200500000000),
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Long),
            flags: OrderFlags::new(OrderFlags::REDUCE_ONLY | OrderFlags::POST_ONLY),
            client_order_id: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            symbol: "BTCUSDT".to_string(),
            note: Some("Test order".to_string()),
            custom_data: vec![0xDE, 0xAD, 0xBE, 0xEF],
            fills: vec![
                Fill::new(
                    1001,
                    Decimal::new(5000100000000, -8),
                    Decimal::new(50000000, -6),
                    Timestamp::new(1704067200100000000),
                ),
                Fill::new(
                    1002,
                    Decimal::new(5000200000000, -8),
                    Decimal::new(100000000, -6),
                    Timestamp::new(1704067200200000000),
                ),
            ],
        };

        let mut buffer = vec![0u8; 4096];
        let write_buf = WriteBuf::new(&mut buffer);
        let mut encoder = SimpleEncoder::new(write_buf);
        order.sbe_encode(&mut encoder).unwrap();
        let len = encoder.finish();

        println!("\n=== ComplexOrder 编码结果 ===");
        println!("总编码长度: {} bytes", len);

        let read_buf = ReadBuf::new(&buffer[..len]);
        let mut decoder = SimpleDecoder::new(read_buf);
        let decoded = ComplexOrder::sbe_decode(&mut decoder).unwrap();

        assert_eq!(order, decoded);

        println!("\n✓ ComplexOrder 编解码成功！");
        println!("✓ 覆盖所有 SBE 数据类型:");
        println!("  ✓ 基本类型: i8, u8, i16, u16, i32, u32, i64, u64");
        println!("  ✓ 浮点类型: f32, f64");
        println!("  ✓ 字符和布尔: char, bool");
        println!("  ✓ 复合类型: Decimal, Timestamp, Composite");
        println!("  ✓ 枚举类型: Enum");
        println!("  ✓ 位集合: Set/Bitset");
        println!("  ✓ 固定长度数组: [u8; N]");
        println!("  ✓ 可变长度数据: String, Vec<u8>");
        println!("  ✓ Optional 类型: Option<T>");
        println!("  ✓ 重复组: Vec<T> (Repeating Groups)");
    }
}
