//! Example demonstrating nested message support (conceptual)

use sbe_derive::{SbeDecode, SbeEncode};

/// Nested price component
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 101, schema_id = 1, version = 0)]
struct PriceLevel {
    #[sbe(id = 0)]
    price: f64,

    #[sbe(id = 1)]
    quantity: i32,
}

/// Trade message with nested price level
/// Note: Full nested message support requires additional infrastructure
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 9, schema_id = 1, version = 0)]
struct TradeWithNested {
    #[sbe(id = 0)]
    trade_id: u64,

    #[sbe(id = 1)]
    symbol: [u8; 8],
    // Nested message would be encoded inline
    // The nested message's fields are embedded at this offset
}

fn main() {
    println!("SBE Nested Messages - Conceptual Example");
    println!("=========================================\n");

    println!("Nested Message Structure:");
    println!("┌─────────────────────────────────────────┐");
    println!("│ Message Header (8 bytes)                │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Parent Message Fields                   │");
    println!("│   - trade_id (8 bytes)                  │");
    println!("│   - symbol (8 bytes)                    │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Nested Message Fields (inline)          │");
    println!("│   - price (8 bytes)                     │");
    println!("│   - quantity (4 bytes)                  │");
    println!("└─────────────────────────────────────────┘");

    println!("\nExample: Trade with nested price level");
    println!("───────────────────────────────────────");

    println!("\nParent Message:");
    println!("  trade_id: 12345");
    println!("  symbol: BTCUSDT");

    println!("\nNested Message (PriceLevel):");
    println!("  price: 50000.50");
    println!("  quantity: 100");

    println!("\n📝 Encoding Process:");
    println!("  1. Write message header");
    println!("  2. Write parent fields (trade_id, symbol)");
    println!("  3. Write nested message fields inline (price, quantity)");
    println!("  4. Total size = header + parent + nested");

    println!("\n📖 Decoding Process:");
    println!("  1. Read message header");
    println!("  2. Read parent fields");
    println!("  3. Access nested message at calculated offset");
    println!("  4. Read nested fields");

    println!("\n✨ Benefits of Nested Messages:");
    println!("  • Reusable composite types");
    println!("  • Inline encoding (no indirection)");
    println!("  • Type-safe field access");
    println!("  • Zero-copy nested access");

    println!("\n⚠️  Implementation Notes:");
    println!("  • Nested messages are encoded inline");
    println!("  • Offset calculation includes parent fields");
    println!("  • Each nested type has its own encoder/decoder");
    println!("  • Block length includes nested message size");

    println!("\n📐 Size Calculation:");
    println!("  Parent block length: 16 bytes (trade_id + symbol)");
    println!("  Nested block length: 12 bytes (price + quantity)");
    println!("  Total block length: 28 bytes");
}
