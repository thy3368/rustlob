//! Trade message example using SBE derive macros
//!
//! This demonstrates how to use the derive macros to replace hand-written
//! encoder/decoder implementations.
//!
//! Run with: cargo run --example trade_codec

use sbe_derive::{SbeDecode, SbeEncode};
use sbe::{ReadBuf, WriteBuf, Writer, Encoder, Reader, Decoder, ActingVersion};

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

    let mut buffer = vec![0u8; 1024];

    // Encode a trade message
    println!("Encoding trade message...");
    let write_buf = WriteBuf::new(&mut buffer);
    let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

    encoder.trade_id(12345);
    encoder.symbol(b'A');
    encoder.price(100.50);
    encoder.quantity(1000);

    println!("  trade_id: 12345");
    println!("  symbol: A");
    println!("  price: 100.50");
    println!("  quantity: 1000");
    println!("  block_length: {}", trade_encoder::SBE_BLOCK_LENGTH);



    // Decode the trade message
    println!("\nDecoding trade message...");
    let read_buf = ReadBuf::new(&buffer);
    let decoder = TradeDecoder::default().wrap(
        read_buf,
        0,
        trade_encoder::SBE_BLOCK_LENGTH,
        0,
    );

    println!("  trade_id: {}", decoder.trade_id());
    println!("  symbol: {}", decoder.symbol() as char);
    println!("  price: {}", decoder.price());
    println!("  quantity: {}", decoder.quantity());

    // Verify roundtrip
    assert_eq!(decoder.trade_id(), 12345);
    assert_eq!(decoder.symbol(), b'A');
    assert_eq!(decoder.price(), 100.50);
    assert_eq!(decoder.quantity(), 1000);

    println!("\n✓ Roundtrip successful!");
}
