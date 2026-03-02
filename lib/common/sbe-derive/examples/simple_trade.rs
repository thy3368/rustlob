//! Simple example demonstrating SBE derive macro usage
//!
//! This example shows how to use #[derive(SbeEncode, SbeDecode)] to automatically
//! generate SBE encoding/decoding code.

use sbe_derive::{SbeDecode, SbeEncode};

/// A simple trade message
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 1, schema_id = 1, version = 0)]
struct Trade {
    #[sbe(id = 0)]
    trade_id: u64,
    #[sbe(id = 1)]
    symbol: u8,
    #[sbe(id = 2)]
    price: f64,
    #[sbe(id = 3)]
    quantity: i32,
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    println!("SBE Derive Macro Example");
    println!("========================\n");

    // Create a buffer
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    // Encode a trade message with header
    println!("Encoding trade message...");
    let encoder = TradeEncoder::default().wrap(write_buf, 0);
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(12345);
    encoder.symbol(65); // 'A'
    encoder.price(100.50);
    encoder.quantity(1000);

    println!("  Trade ID: 12345");
    println!("  Symbol: A (65)");
    println!("  Price: 100.50");
    println!("  Quantity: 1000");
    println!("  Block Length: {}", trade_encoder::SBE_BLOCK_LENGTH);
    println!("  Template ID: {}", trade_encoder::SBE_TEMPLATE_ID);

    // Decode the message
    println!("\nDecoding trade message...");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = TradeDecoder::default().header(header, 0);

    println!("  Trade ID: {}", decoder.trade_id());
    println!("  Symbol: {} ({})", decoder.symbol() as char, decoder.symbol());
    println!("  Price: {}", decoder.price());
    println!("  Quantity: {}", decoder.quantity());

    println!("\n✓ Encoding and decoding successful!");
}
