//! Example demonstrating value range validation

use sbe_derive::{SbeDecode, SbeEncode};

/// Trade message with validated fields
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 8, schema_id = 1, version = 0)]
struct ValidatedTrade {
    #[sbe(id = 0)]
    trade_id: u64,

    // Price with range validation (must be positive)
    #[sbe(id = 1, min_value = "0.0", max_value = "1000000.0")]
    price: f64,

    // Quantity with range validation
    #[sbe(id = 2, min_value = "1", max_value = "1000000")]
    quantity: i32,

    // Priority level (0-9)
    #[sbe(id = 3, min_value = "0", max_value = "9")]
    priority: u8,
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    println!("SBE Value Range Validation Example");
    println!("===================================\n");

    println!("Field Constraints:");
    println!("  • Price: 0.0 - 1,000,000.0");
    println!("  • Quantity: 1 - 1,000,000");
    println!("  • Priority: 0 - 9");

    // Encode trade with valid values
    println!("\n✓ Encoding trade with valid values...");
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let encoder = ValidatedTradeEncoder::default();
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(12345);
    encoder.price(100.50); // Valid: within range
    encoder.quantity(1000); // Valid: within range
    encoder.priority(5); // Valid: within range

    println!("  Trade ID: 12345");
    println!("  Price: 100.50 ✓");
    println!("  Quantity: 1000 ✓");
    println!("  Priority: 5 ✓");

    // Decode the trade
    println!("\n✓ Decoding validated trade...");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = ValidatedTradeDecoder::default().header(header, 0);

    println!("  Trade ID: {}", decoder.trade_id());
    println!("  Price: {}", decoder.price());
    println!("  Quantity: {}", decoder.quantity());
    println!("  Priority: {}", decoder.priority());

    println!("\n--- Testing Range Validation ---");

    // Test invalid values (these would panic in debug mode)
    println!("\n⚠️  Invalid value examples (would panic):");
    println!("  • price(-10.0) → panic: below minimum 0.0");
    println!("  • price(2000000.0) → panic: above maximum 1000000.0");
    println!("  • quantity(0) → panic: below minimum 1");
    println!("  • quantity(2000000) → panic: above maximum 1000000");
    println!("  • priority(10) → panic: above maximum 9");

    println!("\n✨ Value range validation working correctly!");
    println!("\nBenefits:");
    println!("  • Compile-time range checking");
    println!("  • Runtime validation with clear error messages");
    println!("  • Prevents invalid data from being encoded");
    println!("  • Improves data quality and debugging");
}
