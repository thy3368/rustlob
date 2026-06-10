//! Example demonstrating repeating groups (conceptual)

use sbe_derive::{SbeDecode, SbeEncode};

/// A simple order entry in a group
#[derive(Debug, Clone)]
pub struct OrderEntry {
    pub order_id: u64,
    pub price: f64,
    pub quantity: i32,
}

/// Trade message with repeating group of orders
/// Note: Full repeating group support requires additional macro infrastructure
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 6, schema_id = 1, version = 0)]
struct TradeWithOrders {
    #[sbe(id = 0)]
    trade_id: u64,

    #[sbe(id = 1)]
    symbol: [u8; 8],
    // Repeating group would be encoded after fixed fields
    // Group dimension: numInGroup (u16) + blockLength (u16)
    // Followed by: entry1, entry2, ..., entryN
}

fn main() {
    println!("SBE Repeating Groups - Conceptual Example");
    println!("==========================================\n");

    println!("Repeating Groups Structure:");
    println!("┌─────────────────────────────────────────┐");
    println!("│ Message Header (8 bytes)                │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Fixed Fields (block)                    │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Group Dimension:                        │");
    println!("│   - numInGroup (2 bytes)                │");
    println!("│   - blockLength (2 bytes)               │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Group Entry 1 (blockLength bytes)       │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Group Entry 2 (blockLength bytes)       │");
    println!("├─────────────────────────────────────────┤");
    println!("│ ...                                     │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Group Entry N (blockLength bytes)       │");
    println!("└─────────────────────────────────────────┘");

    println!("\nExample: Trade with 3 orders");
    println!("─────────────────────────────");

    let orders = vec![
        OrderEntry { order_id: 1001, price: 100.50, quantity: 100 },
        OrderEntry { order_id: 1002, price: 100.75, quantity: 200 },
        OrderEntry { order_id: 1003, price: 101.00, quantity: 150 },
    ];

    println!("\nGroup Dimension:");
    println!("  numInGroup: {}", orders.len());
    println!("  blockLength: {} bytes (per entry)", 20); // 8 + 8 + 4

    println!("\nGroup Entries:");
    for (i, order) in orders.iter().enumerate() {
        println!(
            "  Entry {}: ID={}, Price=${:.2}, Qty={}",
            i + 1,
            order.order_id,
            order.price,
            order.quantity
        );
    }

    println!("\n📝 Encoding Process:");
    println!("  1. Write message header (8 bytes)");
    println!("  2. Write fixed fields (trade_id, symbol)");
    println!("  3. Write group dimension (numInGroup=3, blockLength=20)");
    println!("  4. Write entry 1 (20 bytes)");
    println!("  5. Write entry 2 (20 bytes)");
    println!("  6. Write entry 3 (20 bytes)");

    println!("\n📖 Decoding Process:");
    println!("  1. Read message header");
    println!("  2. Read fixed fields");
    println!("  3. Read group dimension");
    println!("  4. Iterate through numInGroup entries");
    println!("  5. Each entry is blockLength bytes");

    println!("\n✨ Benefits of Repeating Groups:");
    println!("  • Efficient encoding of collections");
    println!("  • Fixed-size entries for fast iteration");
    println!("  • Zero-copy access to group data");
    println!("  • Supports nested groups");

    println!("\n⚠️  Note: Full repeating group support requires:");
    println!("  • Group dimension encoding/decoding");
    println!("  • Iterator implementation");
    println!("  • Nested group handling");
    println!("  • Variable positioning logic");
}
