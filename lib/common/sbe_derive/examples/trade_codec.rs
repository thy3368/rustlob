//! Trade message example using SBE derive macros
//!
//! This demonstrates how to use the derive macros to replace hand-written
//! encoder/decoder implementations.
//!
//! Run with: cargo run --example trade_codec

use sbe_derive::{SbeDecode, SbeEncode};
use sbe::SbeMessage;

/// Trade message
///
/// Fields:
/// - trade_id: u64 (offset 0, size 8)
/// - symbol: u8 (offset 8, size 1)
/// - price: f64 (offset 9, size 8)
/// - quantity: i32 (offset 17, size 4)
///
/// Total block length: 21 bytes
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 1, schema_id = 1, version = 0)]
pub struct Trade {
    #[sbe(id = 0)]
    pub trade_id: u64,

    #[sbe(id = 1)]
    pub symbol: u8,

    #[sbe(id = 2)]
    pub price: f64,

    #[sbe(id = 3)]
    pub quantity: i32,
}

fn main() {
    println!("=== SBE Trade Codec Example ===\n");

    // Encode using SbeMessage trait
    println!("Encoding trade message...");
    let trade = Trade {
        trade_id: 12345,
        symbol: b'A',
        price: 100.50,
        quantity: 1000,
    };

    let mut buffer = [0u8; 1024];
    let len = trade.encode_into(&mut buffer).unwrap();

    println!("  Encoded {} bytes", len);
    println!("  trade_id: {}", trade.trade_id);
    println!("  symbol: {}", trade.symbol as char);
    println!("  price: {}", trade.price);
    println!("  quantity: {}", trade.quantity);

    // Decode using SbeMessage trait
    println!("\nDecoding trade message...");
    let decoded = Trade::decode_from(&buffer[..len]).unwrap();

    println!("  trade_id: {}", decoded.trade_id);
    println!("  symbol: {}", decoded.symbol as char);
    println!("  price: {}", decoded.price);
    println!("  quantity: {}", decoded.quantity);

    // Verify roundtrip
    assert_eq!(decoded.trade_id, 12345);
    assert_eq!(decoded.symbol, b'A');
    assert_eq!(decoded.price, 100.50);
    assert_eq!(decoded.quantity, 1000);

    println!("\n✓ Roundtrip successful!");

    //todo TradeEncoder 与Trade 怎么互转？
}
