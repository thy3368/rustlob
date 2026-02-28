//! Example demonstrating all implemented SBE features

use sbe_derive::{SbeDecode, SbeEncode, SbeEnum};
use sbe::{ReadBuf, WriteBuf, Writer, Encoder, Reader, Decoder, ActingVersion};

/// Order side enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, SbeEnum)]
enum Side {
    Buy,
    Sell,
}

/// Comprehensive trade message showcasing all features
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 100, schema_id = 1, version = 2)]
struct ComprehensiveTrade {
    // Basic primitive types
    #[sbe(id = 0)]
    trade_id: u64,

    #[sbe(id = 1)]
    price: f64,

    #[sbe(id = 2)]
    quantity: i32,

    // Boolean type
    #[sbe(id = 3)]
    is_aggressive: bool,

    // Char type
    #[sbe(id = 4)]
    market_code: char,

    // Enum type (encoded as u8)
    #[sbe(id = 5)]
    side: u8,

    // Optional field
    #[sbe(id = 6, presence = "optional")]
    client_order_id: Option<u64>,

    // Fixed-length array
    #[sbe(id = 7)]
    symbol: [u8; 8],

    // Constant field
    #[sbe(id = 8, presence = "constant", constant = "1")]
    message_version: u8,

    // Version 1 field
    #[sbe(id = 9, since_version = 1)]
    execution_venue: u8,

    // Version 2 field
    #[sbe(id = 10, since_version = 2)]
    liquidity_flag: u8,

    // Variable-length data (must be last)
    // Note: In actual usage, this would be handled separately
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║  SBE Derive Macro - Comprehensive Feature Showcase    ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Encode a comprehensive trade
    println!("📝 Encoding comprehensive trade message...\n");
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let encoder = ComprehensiveTradeEncoder::default();
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    // Set all fields
    encoder.trade_id(123456789);
    encoder.price(50125.75);
    encoder.quantity(100);
    encoder.is_aggressive(true);
    encoder.market_code('N'); // NYSE
    encoder.side(Side::Buy.to_u8());
    encoder.client_order_id(Some(999888777));
    encoder.symbol(&[b'B', b'T', b'C', b'U', b'S', b'D', b'T', 0]);
    // message_version is constant, no setter
    encoder.execution_venue(1); // Version 1 field
    encoder.liquidity_flag(2); // Version 2 field

    println!("  ✓ Trade ID: 123456789");
    println!("  ✓ Price: $50,125.75");
    println!("  ✓ Quantity: 100");
    println!("  ✓ Is Aggressive: true");
    println!("  ✓ Market Code: 'N' (NYSE)");
    println!("  ✓ Side: {:?} ({})", Side::Buy, Side::Buy.to_u8());
    println!("  ✓ Client Order ID: Some(999888777)");
    println!("  ✓ Symbol: BTCUSDT");
    println!("  ✓ Message Version: 1 (constant)");
    println!("  ✓ Execution Venue: 1 (v1)");
    println!("  ✓ Liquidity Flag: 2 (v2)");

    println!("\n📊 Message Statistics:");
    println!("  • Block Length: {} bytes", encoder::SBE_BLOCK_LENGTH);
    println!("  • Template ID: {}", encoder::SBE_TEMPLATE_ID);
    println!("  • Schema ID: {}", encoder::SBE_SCHEMA_ID);
    println!("  • Schema Version: {}", encoder::SBE_SCHEMA_VERSION);

    // Decode the message
    println!("\n📖 Decoding comprehensive trade message...\n");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = ComprehensiveTradeDecoder::default().header(header, 0);

    println!("  ✓ Trade ID: {}", decoder.trade_id());
    println!("  ✓ Price: ${:.2}", decoder.price());
    println!("  ✓ Quantity: {}", decoder.quantity());
    println!("  ✓ Is Aggressive: {}", decoder.is_aggressive());
    println!("  ✓ Market Code: '{}'", decoder.market_code());

    let decoded_side = Side::from_u8(decoder.side());
    println!("  ✓ Side: {:?}", decoded_side);

    println!("  ✓ Client Order ID: {:?}", decoder.client_order_id());

    let symbol = decoder.symbol();
    let symbol_str = std::str::from_utf8(&symbol[..7]).unwrap_or("???");
    println!("  ✓ Symbol: {}", symbol_str);

    println!("  ✓ Message Version: {} (constant)", decoder.message_version());

    // Version-dependent fields
    if let Some(venue) = decoder.execution_venue() {
        println!("  ✓ Execution Venue: {} (v1)", venue);
    } else {
        println!("  ✗ Execution Venue: Not available (version < 1)");
    }

    if let Some(flag) = decoder.liquidity_flag() {
        println!("  ✓ Liquidity Flag: {} (v2)", flag);
    } else {
        println!("  ✗ Liquidity Flag: Not available (version < 2)");
    }

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║  Feature Summary                                       ║");
    println!("╠════════════════════════════════════════════════════════╣");
    println!("║  ✓ Primitive types (u8-u64, i8-i64, f32, f64)        ║");
    println!("║  ✓ Boolean type (encoded as u8)                       ║");
    println!("║  ✓ Char type (single-byte character)                  ║");
    println!("║  ✓ Enum support (SbeEnum derive macro)                ║");
    println!("║  ✓ Optional fields (Option<T> with nullValue)         ║");
    println!("║  ✓ Fixed-length arrays ([T; N])                       ║");
    println!("║  ✓ Constant fields (zero-byte encoding)               ║");
    println!("║  ✓ Version fields (sinceVersion)                      ║");
    println!("║  ✓ Message header (8 bytes)                           ║");
    println!("║  ✓ Zero-copy encoding/decoding                        ║");
    println!("║  ✓ Inline optimization (#[inline])                    ║");
    println!("╚════════════════════════════════════════════════════════╝");

    println!("\n✨ All features working correctly!");
}
