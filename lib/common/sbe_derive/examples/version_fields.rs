//! Example demonstrating version field support

use sbe_derive::{SbeDecode, SbeEncode};

/// Trade message with versioned fields
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 4, schema_id = 1, version = 2)]
struct VersionedTrade {
    // Version 0 fields
    #[sbe(id = 0)]
    trade_id: u64,

    #[sbe(id = 1)]
    price: f64,

    #[sbe(id = 2)]
    quantity: i32,

    // Version 1 field
    #[sbe(id = 3, since_version = 1)]
    client_order_id: u64,

    // Version 2 field
    #[sbe(id = 4, since_version = 2)]
    execution_venue: u8,
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    println!("SBE Version Field Support Example");
    println!("==================================\n");

    // Encode with version 2 (all fields available)
    println!("Encoding with version 2 (all fields)...");
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let encoder = VersionedTradeEncoder::default().wrap(write_buf, 0);
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(12345);
    encoder.price(100.50);
    encoder.quantity(1000);
    encoder.client_order_id(99999); // Version 1 field
    encoder.execution_venue(1); // Version 2 field

    println!("  Trade ID: 12345");
    println!("  Price: 100.50");
    println!("  Quantity: 1000");
    println!("  Client Order ID: 99999 (v1)");
    println!("  Execution Venue: 1 (v2)");

    // Decode with version 2 (all fields available)
    println!("\nDecoding with version 2...");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = VersionedTradeDecoder::default().header(header, 0);

    println!("  Trade ID: {}", decoder.trade_id());
    println!("  Price: {}", decoder.price());
    println!("  Quantity: {}", decoder.quantity());

    // Version 1+ fields return Option
    if let Some(client_id) = decoder.client_order_id() {
        println!("  Client Order ID: {} (v1)", client_id);
    } else {
        println!("  Client Order ID: Not available (version < 1)");
    }

    if let Some(venue) = decoder.execution_venue() {
        println!("  Execution Venue: {} (v2)", venue);
    } else {
        println!("  Execution Venue: Not available (version < 2)");
    }

    // Simulate decoding with older version
    println!("\n--- Simulating decode with version 0 ---");
    let decoder_v0 = VersionedTradeDecoder::default().wrap(
        ReadBuf::new(&buffer),
        0,
        versioned_trade_encoder::SBE_BLOCK_LENGTH,
        0, // Acting version 0
    );

    println!("  Trade ID: {}", decoder_v0.trade_id());
    println!("  Price: {}", decoder_v0.price());
    println!("  Quantity: {}", decoder_v0.quantity());

    if let Some(client_id) = decoder_v0.client_order_id() {
        println!("  Client Order ID: {} (v1)", client_id);
    } else {
        println!("  Client Order ID: Not available (version < 1)");
    }

    if let Some(venue) = decoder_v0.execution_venue() {
        println!("  Execution Venue: {} (v2)", venue);
    } else {
        println!("  Execution Venue: Not available (version < 2)");
    }

    println!("\n✓ Version field handling working correctly!");
}
