//! ComplexOrder - 用于benchmark测试的复杂订单结构

use sbe::codec::codec::{SbeDecode, SbeDecoder, SbeEncode, SbeEncoder};
use sbe::sbe_types::{Decimal, Timestamp};

// ============================================================================
// Composite 类型 - PriceQuantity
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct PriceQuantity {
    pub price: Decimal,
    pub quantity: Decimal,
}

impl PriceQuantity {
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
// Set 类型 - OrderFlags
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct OrderFlags(pub u8);

impl OrderFlags {
    pub const NONE: u8 = 0;
    pub const REDUCE_ONLY: u8 = 1 << 0;
    pub const POST_ONLY: u8 = 1 << 1;
    pub const CLOSE_POSITION: u8 = 1 << 2;
    pub const PRICE_PROTECT: u8 = 1 << 3;

    pub fn new(flags: u8) -> Self {
        Self(flags)
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
// Repeating Group - Fill
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct Fill {
    pub fill_id: u64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub timestamp: Timestamp,
}

impl Fill {
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
// Enum 类型
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
// ComplexOrder - 完整的订单结构
// ============================================================================

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct ComplexOrder {
    // 基本类型
    pub order_id: u64,
    pub user_id: u32,
    pub sequence: u16,
    pub version: u8,
    pub priority: i8,

    // 浮点类型
    pub fee_rate: f32,
    pub slippage_tolerance: f64,

    // 字符和布尔
    pub type_code: char,
    pub is_active: bool,

    // 复合类型
    pub price: Decimal,
    pub quantity: Decimal,
    pub stop_loss: Option<PriceQuantity>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,

    // 枚举类型
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,

    // 位集合
    pub flags: OrderFlags,

    // 固定长度数组
    pub client_order_id: [u8; 16],

    // 可变长度数据
    pub symbol: String,
    pub note: Option<String>,
    pub custom_data: Vec<u8>,

    // 重复组
    pub fills: Vec<Fill>,
}

impl ComplexOrder {
    /// 创建一个用于benchmark的示例订单
    pub fn sample() -> Self {
        ComplexOrder {
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
        }
    }
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

impl ComplexOrder {
    /// 原地更新解码 - 零分配优化
    ///
    /// 复用现有对象的内存，避免创建新对象
    pub fn decode_into<'de, D: SbeDecoder<'de>>(
        &mut self,
        decoder: &mut D,
    ) -> Result<(), D::Error> {
        // 固定字段 - 直接赋值
        self.order_id = decoder.decode_u64()?;
        self.user_id = decoder.decode_u32()?;
        self.sequence = decoder.decode_u16()?;
        self.version = decoder.decode_u8()?;
        self.priority = decoder.decode_i8()?;
        self.fee_rate = decoder.decode_f32()?;
        self.slippage_tolerance = decoder.decode_f64()?;
        self.type_code = decoder.decode_char()?;
        self.is_active = decoder.decode_bool()?;
        self.price = Decimal::sbe_decode(decoder)?;
        self.quantity = Decimal::sbe_decode(decoder)?;
        self.stop_loss = Option::<PriceQuantity>::sbe_decode(decoder)?;
        self.created_at = Timestamp::sbe_decode(decoder)?;
        self.updated_at = Timestamp::sbe_decode(decoder)?;
        self.side = OrderSide::sbe_decode(decoder)?;
        self.position_side = Option::<PositionSide>::sbe_decode(decoder)?;
        self.flags = OrderFlags::sbe_decode(decoder)?;
        self.client_order_id = decoder.decode_array::<16>()?;

        // 可变长度字段 - 使用swap复用capacity
        let mut temp_symbol = String::sbe_decode(decoder)?;
        std::mem::swap(&mut self.symbol, &mut temp_symbol);

        let mut temp_note = Option::<String>::sbe_decode(decoder)?;
        std::mem::swap(&mut self.note, &mut temp_note);

        let mut temp_custom_data = decoder.decode_bytes()?.to_vec();
        std::mem::swap(&mut self.custom_data, &mut temp_custom_data);

        // Vec<Fill> - 复用capacity
        let count = decoder.decode_u16()? as usize;
        self.fills.clear();
        self.fills.reserve(count.saturating_sub(self.fills.capacity()));
        for _ in 0..count {
            self.fills.push(Fill::sbe_decode(decoder)?);
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
            custom_data: decoder.decode_bytes()?.to_vec(),
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
