//! Zero-copy view example using SbeView derive macro
//!
//! This example demonstrates:
//! 1. Basic zero-copy decoding with primitives
//! 2. Custom types implementing ZeroCopyDecode trait

use sbe::{ReadBuf, ZeroCopyDecode};
use sbe_derive::SbeView;

// ============================================================================
// Custom Type: Timestamp (8 bytes - u64 milliseconds)
// ============================================================================

/// Custom timestamp type - demonstrates ZeroCopyDecode for custom types
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timestamp(u64);

impl ZeroCopyDecode for Timestamp {
    #[inline]
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
        let buf = ReadBuf::new(data);
        Timestamp(buf.get_u64_at(offset))
    }

    fn encoded_size() -> usize {
        8
    }
}

impl Timestamp {
    pub fn as_millis(&self) -> u64 {
        self.0
    }
}

// ============================================================================
// Custom Type: Price (8 bytes - fixed-point decimal)
// ============================================================================

/// Custom price type - demonstrates another ZeroCopyDecode implementation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Price {
    raw: i64,
    scale: i32, // 10^scale for decimal places
}

impl ZeroCopyDecode for Price {
    #[inline]
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
        let buf = ReadBuf::new(data);
        Price {
            raw: buf.get_i64_at(offset),
            scale: 4, // 4 decimal places (e.g., 12345678 -> 1234.5678)
        }
    }

    fn encoded_size() -> usize {
        8
    }
}

impl Price {
    pub fn to_f64(&self) -> f64 {
        self.raw as f64 / 10_f64.powi(self.scale)
    }
}

// ============================================================================
// Test struct with custom types
// ============================================================================

/// Order message - using SbeView with custom types
#[derive(SbeView)]
#[sbe(template_id = 2, schema_id = 1, version = 0)]
pub struct Order {
    #[sbe(id = 0)]
    pub order_id: u64,

    #[sbe(id = 1, size = 8)]
    pub timestamp: Timestamp, // Custom type!

    #[sbe(id = 2, size = 8)]
    pub price: Price, // Custom type!

    #[sbe(id = 3)]
    pub quantity: i32,
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("=== SbeView Derive Macro - Custom Types Example ===\n");

    // ========================================================================
    // Test 1: Basic primitives (from previous example)
    // ========================================================================
    println!("--- Test 1: Basic primitives ---");

    let mut buffer = [0u8; 21];
    buffer[0..8].copy_from_slice(&12345u64.to_le_bytes());
    buffer[8] = b'A';
    buffer[9..17].copy_from_slice(&100.50f64.to_le_bytes());
    buffer[17..21].copy_from_slice(&1000i32.to_le_bytes());

    let view = TradeView::from_bytes(&buffer).unwrap();
    assert_eq!(view.trade_id(), 12345);
    println!("  ✓ trade_id: {}", view.trade_id());
    println!("  ✓ symbol: {}", view.symbol() as char);
    println!("  ✓ price: {}", view.price());
    println!("  ✓ quantity: {}", view.quantity());

    // ========================================================================
    // Test 2: Custom types (Timestamp, Price)
    // ========================================================================
    println!("\n--- Test 2: Custom types ---");

    // Buffer layout:
    // - order_id: 8 bytes (u64)
    // - timestamp: 8 bytes (u64)
    // - price: 8 bytes (i64)
    // - quantity: 4 bytes (i32)
    // Total: 28 bytes
    let mut order_buffer = [0u8; 28];

    // order_id = 99999
    order_buffer[0..8].copy_from_slice(&99999u64.to_le_bytes());
    // timestamp = 1700000000000 ms
    order_buffer[8..16].copy_from_slice(&1700000000000u64.to_le_bytes());
    // price = 12345678 (meaning 1234.5678 with scale 4)
    order_buffer[16..24].copy_from_slice(&12345678i64.to_le_bytes());
    // quantity = 500
    order_buffer[24..28].copy_from_slice(&500i32.to_le_bytes());

    println!("  Input buffer: {:?}", &order_buffer[..]);

    let order_view = OrderView::from_bytes(&order_buffer).unwrap();

    println!("  ✓ order_id:    {}", order_view.order_id());
    println!("  ✓ timestamp:   {} ms", order_view.timestamp().as_millis());
    println!("  ✓ price:       {} (f64: {})", order_view.price().raw, order_view.price().to_f64());
    println!("  ✓ quantity:    {}", order_view.quantity());

    assert_eq!(order_view.quantity(), 500);

    // ========================================================================
    // Test 3: Fixed-size arrays
    // ========================================================================
    println!("\n--- Test 3: Fixed-size arrays ---");
    
    // Test [u8; N] implementation directly
    let test_data = [1u8, 2, 3, 4, 5];
    let decoded: [u8; 5] = sbe::ZeroCopyDecode::zero_copy_decode(&test_data, 0);
    assert_eq!(decoded, [1, 2, 3, 4, 5]);
    println!("  ✓ [u8; 5] array decoded correctly: {:?}", decoded);
    assert_eq!(<[u8; 5] as sbe::ZeroCopyDecode>::encoded_size(), 5);
    println!("  ✓ encoded_size() for [u8; 5] = 5");

    // Test [u8; 8] for timestamp-like data
    let timestamp_data = [0x00, 0x10, 0x2B, 0x05, 0x00, 0x00, 0x00, 0x00];
    let ts: [u8; 8] = sbe::ZeroCopyDecode::zero_copy_decode(&timestamp_data, 0);
    println!("  ✓ [u8; 8] decoded: {:?}", ts);

    println!("\n✓ All tests passed!");
    println!("\nKey insight:");
    println!("  - Custom types can implement ZeroCopyDecode for zero-copy parsing");
    println!("  - SbeView works with any type that implements ZeroCopyDecode");
    println!("  - The trait-based approach allows extensible zero-copy decoding");
}

// ============================================================================
// Trade struct (from basic example)
// ============================================================================

#[derive(SbeView)]
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
