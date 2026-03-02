//! Performance comparison: SBE vs Serde (JSON, Bincode)
//!
//! This benchmark compares the performance of SBE encoding/decoding
//! against popular Rust serialization libraries.

use sbe_derive::{SbeDecode, SbeEncode};
use sbe::{ReadBuf, WriteBuf, Writer, Encoder, Reader, Decoder, ActingVersion};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Trade message for SBE
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 1, schema_id = 1, version = 0)]
pub struct SbeTrade {
    #[sbe(id = 0)]
    pub trade_id: u64,

    #[sbe(id = 1)]
    pub symbol: u8,

    #[sbe(id = 2)]
    pub price: f64,

    #[sbe(id = 3)]
    pub quantity: i32,
}

/// Trade message for Serde
#[derive(Serialize, Deserialize, Clone)]
pub struct SerdeTrade {
    pub trade_id: u64,
    pub symbol: u8,
    pub price: f64,
    pub quantity: i32,
}

fn benchmark_sbe(iterations: usize) -> (u128, u128, usize) {
    let mut buffer = vec![0u8; 1024];

    // Encode benchmark
    let start = Instant::now();
    for i in 0..iterations {
        let write_buf = WriteBuf::new(&mut buffer);
        let mut encoder = SbeTradeEncoder::default().wrap(write_buf, 0);
        encoder.trade_id(i as u64);
        encoder.symbol(b'A');
        encoder.price(100.50 + i as f64);
        encoder.quantity(1000 + i as i32);
    }
    let encode_time = start.elapsed().as_nanos();

    // Decode benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        let read_buf = ReadBuf::new(&buffer);
        let decoder = SbeTradeDecoder::default().wrap(
            read_buf,
            0,
            sbe_trade_encoder::SBE_BLOCK_LENGTH,
            0,
        );
        let _ = decoder.trade_id();
        let _ = decoder.symbol();
        let _ = decoder.price();
        let _ = decoder.quantity();
    }
    let decode_time = start.elapsed().as_nanos();

    let size = sbe_trade_encoder::SBE_BLOCK_LENGTH as usize;
    (encode_time, decode_time, size)
}

fn benchmark_serde_json(iterations: usize) -> (u128, u128, usize) {
    let trade = SerdeTrade {
        trade_id: 12345,
        symbol: b'A',
        price: 100.50,
        quantity: 1000,
    };

    // Encode benchmark
    let start = Instant::now();
    let mut json_data = String::new();
    for i in 0..iterations {
        let mut t = trade.clone();
        t.trade_id = i as u64;
        json_data = serde_json::to_string(&t).unwrap();
    }
    let encode_time = start.elapsed().as_nanos();

    // Decode benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        let _: SerdeTrade = serde_json::from_str(&json_data).unwrap();
    }
    let decode_time = start.elapsed().as_nanos();

    let size = json_data.len();
    (encode_time, decode_time, size)
}

fn benchmark_bincode(iterations: usize) -> (u128, u128, usize) {
    let trade = SerdeTrade {
        trade_id: 12345,
        symbol: b'A',
        price: 100.50,
        quantity: 1000,
    };

    // Encode benchmark
    let start = Instant::now();
    let mut bincode_data = Vec::new();
    for i in 0..iterations {
        let mut t = trade.clone();
        t.trade_id = i as u64;
        bincode_data = bincode::serialize(&t).unwrap();
    }
    let encode_time = start.elapsed().as_nanos();

    // Decode benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        let _: SerdeTrade = bincode::deserialize(&bincode_data).unwrap();
    }
    let decode_time = start.elapsed().as_nanos();

    let size = bincode_data.len();
    (encode_time, decode_time, size)
}

#[test]
fn test_serde_comparison() {
    const ITERATIONS: usize = 100_000;

    println!("\n=== Serialization Performance Comparison ===");
    println!("Iterations: {}\n", ITERATIONS);

    // SBE benchmark
    let (sbe_encode, sbe_decode, sbe_size) = benchmark_sbe(ITERATIONS);
    let sbe_encode_ns = sbe_encode / ITERATIONS as u128;
    let sbe_decode_ns = sbe_decode / ITERATIONS as u128;

    println!("SBE (Simple Binary Encoding):");
    println!("  Encode: {} ns/op", sbe_encode_ns);
    println!("  Decode: {} ns/op", sbe_decode_ns);
    println!("  Total:  {} ns/op", sbe_encode_ns + sbe_decode_ns);
    println!("  Size:   {} bytes", sbe_size);

    // JSON benchmark
    let (json_encode, json_decode, json_size) = benchmark_serde_json(ITERATIONS);
    let json_encode_ns = json_encode / ITERATIONS as u128;
    let json_decode_ns = json_decode / ITERATIONS as u128;

    println!("\nserde_json:");
    println!("  Encode: {} ns/op", json_encode_ns);
    println!("  Decode: {} ns/op", json_decode_ns);
    println!("  Total:  {} ns/op", json_encode_ns + json_decode_ns);
    println!("  Size:   {} bytes", json_size);

    // Bincode benchmark
    let (bincode_encode, bincode_decode, bincode_size) = benchmark_bincode(ITERATIONS);
    let bincode_encode_ns = bincode_encode / ITERATIONS as u128;
    let bincode_decode_ns = bincode_decode / ITERATIONS as u128;

    println!("\nbincode:");
    println!("  Encode: {} ns/op", bincode_encode_ns);
    println!("  Decode: {} ns/op", bincode_decode_ns);
    println!("  Total:  {} ns/op", bincode_encode_ns + bincode_decode_ns);
    println!("  Size:   {} bytes", bincode_size);

    // Performance comparison
    println!("\n=== Performance Comparison ===");
    println!("SBE vs JSON:");
    println!("  Encode: {:.2}x faster", json_encode_ns as f64 / sbe_encode_ns as f64);
    println!("  Decode: {:.2}x faster", json_decode_ns as f64 / sbe_decode_ns as f64);
    println!("  Size:   {:.2}x smaller", json_size as f64 / sbe_size as f64);

    println!("\nSBE vs Bincode:");
    println!("  Encode: {:.2}x faster", bincode_encode_ns as f64 / sbe_encode_ns as f64);
    println!("  Decode: {:.2}x faster", bincode_decode_ns as f64 / sbe_decode_ns as f64);
    println!("  Size:   {:.2}x smaller", bincode_size as f64 / sbe_size as f64);

    // Assertions - SBE should be faster than JSON
    assert!(sbe_encode_ns < json_encode_ns, "SBE encode should be faster than JSON");
    assert!(sbe_decode_ns < json_decode_ns, "SBE decode should be faster than JSON");
    assert!(sbe_size < json_size, "SBE should be more compact than JSON");
}
