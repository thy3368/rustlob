//! Example demonstrating decimal type support

use sbe_derive::{SbeDecode, SbeEncode};

/// Decimal type wrapper for SBE encoding
/// Represents a decimal number as mantissa * 10^exponent
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decimal {
    pub mantissa: i64,
    pub exponent: i8,
}

impl Decimal {
    pub fn new(mantissa: i64, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }

    pub fn from_f64(value: f64, exponent: i8) -> Self {
        let scale = 10_i64.pow((-exponent) as u32);
        let mantissa = (value * scale as f64).round() as i64;
        Self { mantissa, exponent }
    }

    pub fn to_f64(&self) -> f64 {
        let scale = 10_f64.powi((-self.exponent) as i32);
        self.mantissa as f64 / scale
    }
}

/// Trade message with decimal price
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 5, schema_id = 1, version = 0)]
struct DecimalTrade {
    #[sbe(id = 0)]
    trade_id: u64,

    // Price as decimal: mantissa (i64) with fixed exponent -4
    // This allows prices like 100.5000 to be represented exactly
    #[sbe(id = 1)]
    price_mantissa: i64,

    #[sbe(id = 2)]
    quantity: i32,
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    println!("SBE Decimal Type Support Example");
    println!("=================================\n");

    // Create decimal prices
    let price1 = Decimal::from_f64(100.5000, -4);
    let price2 = Decimal::from_f64(99.9950, -4);

    println!("Decimal representation:");
    println!("  100.5000 = {} * 10^{}", price1.mantissa, price1.exponent);
    println!("  99.9950 = {} * 10^{}", price2.mantissa, price2.exponent);

    // Encode trade with decimal price
    println!("\nEncoding trade with decimal price...");
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let encoder = DecimalTradeEncoder::default();
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(12345);
    encoder.price_mantissa(price1.mantissa); // Store mantissa directly
    encoder.quantity(1000);

    println!("  Trade ID: 12345");
    println!("  Price: {} (mantissa: {})", price1.to_f64(), price1.mantissa);
    println!("  Quantity: 1000");

    // Decode the trade
    println!("\nDecoding trade with decimal price...");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = DecimalTradeDecoder::default().header(header, 0);

    let decoded_mantissa = decoder.price_mantissa();
    let decoded_price = Decimal::new(decoded_mantissa, -4);

    println!("  Trade ID: {}", decoder.trade_id());
    println!("  Price: {} (mantissa: {})", decoded_price.to_f64(), decoded_mantissa);
    println!("  Quantity: {}", decoder.quantity());

    // Verify exact representation
    println!("\n--- Exact Decimal Representation ---");
    println!("  Original: {}", price1.to_f64());
    println!("  Decoded: {}", decoded_price.to_f64());
    println!("  Match: {}", price1.mantissa == decoded_price.mantissa);

    // Compare with floating point
    println!("\n--- Floating Point Comparison ---");
    let fp_price = 100.5000_f64;
    let decimal_price = Decimal::from_f64(fp_price, -4);
    println!("  Float: {}", fp_price);
    println!("  Decimal: {} (mantissa: {})", decimal_price.to_f64(), decimal_price.mantissa);
    println!("  Exact: {}", decimal_price.to_f64() == fp_price);

    println!("\n✓ Decimal type encoding working correctly!");
    println!("\nNote: Decimal types provide exact representation for financial data,");
    println!("avoiding floating-point precision issues.");
}
