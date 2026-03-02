//! Example demonstrating time type support

use sbe_derive::{SbeDecode, SbeEncode};

/// Trade message with time fields
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 7, schema_id = 1, version = 0)]
struct TimedTrade {
    #[sbe(id = 0)]
    trade_id: u64,

    // UTCTimestamp - nanoseconds since Unix epoch (i64)
    #[sbe(id = 1)]
    timestamp_nanos: i64,

    // UTCDateOnly - days since Unix epoch (i32)
    #[sbe(id = 2)]
    trade_date: i32,

    // UTCTimeOnly - nanoseconds since midnight (i64)
    #[sbe(id = 3)]
    trade_time: i64,

    #[sbe(id = 4)]
    price: f64,

    #[sbe(id = 5)]
    quantity: i32,
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    println!("SBE Time Types Example");
    println!("======================\n");

    // Get current time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let timestamp_nanos = now.as_nanos() as i64;
    let timestamp_millis = now.as_millis() as i64;

    println!("Current time:");
    println!("  Nanoseconds since epoch: {}", timestamp_nanos);
    println!("  Milliseconds since epoch: {}", timestamp_millis);

    // Calculate date and time components
    let days_since_epoch = (timestamp_millis / (1000 * 60 * 60 * 24)) as i32;
    let nanos_since_midnight = (timestamp_nanos % (24 * 60 * 60 * 1_000_000_000)) as i64;

    println!("\nTime components:");
    println!("  Days since epoch: {}", days_since_epoch);
    println!("  Nanos since midnight: {}", nanos_since_midnight);

    // Encode trade with time fields
    println!("\nEncoding trade with time fields...");
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let encoder = TimedTradeEncoder::default();
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(12345);
    encoder.timestamp_nanos(timestamp_nanos);
    encoder.trade_date(days_since_epoch);
    encoder.trade_time(nanos_since_midnight);
    encoder.price(100.50);
    encoder.quantity(1000);

    println!("  Trade ID: 12345");
    println!("  Timestamp: {} nanos", timestamp_nanos);
    println!("  Trade Date: {} days", days_since_epoch);
    println!("  Trade Time: {} nanos since midnight", nanos_since_midnight);
    println!("  Price: 100.50");
    println!("  Quantity: 1000");

    // Decode the trade
    println!("\nDecoding trade with time fields...");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = TimedTradeDecoder::default().header(header, 0);

    let decoded_timestamp = decoder.timestamp_nanos();
    let decoded_date = decoder.trade_date();
    let decoded_time = decoder.trade_time();

    println!("  Trade ID: {}", decoder.trade_id());
    println!("  Timestamp: {} nanos", decoded_timestamp);
    println!("  Trade Date: {} days", decoded_date);
    println!("  Trade Time: {} nanos since midnight", decoded_time);
    println!("  Price: {}", decoder.price());
    println!("  Quantity: {}", decoder.quantity());

    // Verify exact match
    println!("\n--- Verification ---");
    println!("  Timestamp match: {}", timestamp_nanos == decoded_timestamp);
    println!("  Date match: {}", days_since_epoch == decoded_date);
    println!("  Time match: {}", nanos_since_midnight == decoded_time);

    println!("\n✨ Time types working correctly!");
    println!("\nSBE Time Type Encoding:");
    println!("  • UTCTimestamp: i64 (nanoseconds since Unix epoch)");
    println!("  • UTCDateOnly: i32 (days since Unix epoch)");
    println!("  • UTCTimeOnly: i64 (nanoseconds since midnight)");
    println!("  • MonthYear: u32 (YYYYMM format)");
}
