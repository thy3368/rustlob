//! Performance benchmarks for SBE derive macros

use sbe_derive::{SbeDecode, SbeEncode};
use sbe::{ReadBuf, WriteBuf, Writer, Encoder, Reader, Decoder, ActingVersion};
use std::time::Instant;

/// Simple trade message for benchmarking
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 1, schema_id = 1, version = 0)]
struct BenchTrade {
    #[sbe(id = 0)]
    trade_id: u64,

    #[sbe(id = 1)]
    price: f64,

    #[sbe(id = 2)]
    quantity: i32,

    #[sbe(id = 3)]
    symbol: [u8; 8],
}

fn benchmark_encode(iterations: usize) -> std::time::Duration {
    let mut buffer = vec![0u8; 1024];

    let start = Instant::now();
    for i in 0..iterations {
        let write_buf = sbe::WriteBuf::new(&mut buffer);
        let mut encoder = BenchTradeEncoder::default().wrap(write_buf, 0);

        encoder.trade_id(i as u64);
        encoder.price(100.50 + i as f64);
        encoder.quantity(1000 + i as i32);
        encoder.symbol(&[b'B', b'T', b'C', b'U', b'S', b'D', b'T', 0]);
    }
    start.elapsed()
}

fn benchmark_decode(iterations: usize) -> std::time::Duration {
    let mut buffer = vec![0u8; 1024];
    let write_buf = sbe::WriteBuf::new(&mut buffer);
    let mut encoder = BenchTradeEncoder::default().wrap(write_buf, 0);

    encoder.trade_id(12345);
    encoder.price(100.50);
    encoder.quantity(1000);
    encoder.symbol(&[b'B', b'T', b'C', b'U', b'S', b'D', b'T', 0]);

    let start = Instant::now();
    for _ in 0..iterations {
        let read_buf = sbe::ReadBuf::new(&buffer);
        let decoder = BenchTradeDecoder::default().wrap(
            read_buf,
            0,
            encoder::SBE_BLOCK_LENGTH,
            0,
        );

        let _ = decoder.trade_id();
        let _ = decoder.price();
        let _ = decoder.quantity();
        let _ = decoder.symbol();
    }
    start.elapsed()
}

fn benchmark_roundtrip(iterations: usize) -> std::time::Duration {
    let mut buffer = vec![0u8; 1024];

    let start = Instant::now();
    for i in 0..iterations {
        // Encode
        let write_buf = sbe::WriteBuf::new(&mut buffer);
        let mut encoder = BenchTradeEncoder::default().wrap(write_buf, 0);
        encoder.trade_id(i as u64);
        encoder.price(100.50 + i as f64);
        encoder.quantity(1000 + i as i32);
        encoder.symbol(&[b'B', b'T', b'C', b'U', b'S', b'D', b'T', 0]);

        // Decode
        let read_buf = sbe::ReadBuf::new(&buffer);
        let decoder = BenchTradeDecoder::default().wrap(
            read_buf,
            0,
            encoder::SBE_BLOCK_LENGTH,
            0,
        );

        let _ = decoder.trade_id();
        let _ = decoder.price();
        let _ = decoder.quantity();
        let _ = decoder.symbol();
    }
    start.elapsed()
}

#[test]
fn test_performance_benchmarks() {
    const ITERATIONS: usize = 100_000;

    println!("\n=== SBE Derive Macro Performance Benchmarks ===");
    println!("Iterations: {}", ITERATIONS);

    // Encode benchmark
    let encode_duration = benchmark_encode(ITERATIONS);
    let encode_ns_per_op = encode_duration.as_nanos() / ITERATIONS as u128;
    println!("\nEncode:");
    println!("  Total: {:?}", encode_duration);
    println!("  Per operation: {} ns", encode_ns_per_op);
    println!("  Throughput: {:.2} ops/sec", ITERATIONS as f64 / encode_duration.as_secs_f64());

    // Decode benchmark
    let decode_duration = benchmark_decode(ITERATIONS);
    let decode_ns_per_op = decode_duration.as_nanos() / ITERATIONS as u128;
    println!("\nDecode:");
    println!("  Total: {:?}", decode_duration);
    println!("  Per operation: {} ns", decode_ns_per_op);
    println!("  Throughput: {:.2} ops/sec", ITERATIONS as f64 / decode_duration.as_secs_f64());

    // Roundtrip benchmark
    let roundtrip_duration = benchmark_roundtrip(ITERATIONS);
    let roundtrip_ns_per_op = roundtrip_duration.as_nanos() / ITERATIONS as u128;
    println!("\nRoundtrip (encode + decode):");
    println!("  Total: {:?}", roundtrip_duration);
    println!("  Per operation: {} ns", roundtrip_ns_per_op);
    println!("  Throughput: {:.2} ops/sec", ITERATIONS as f64 / roundtrip_duration.as_secs_f64());

    // Performance assertions (should be very fast)
    assert!(encode_ns_per_op < 1000, "Encode should be < 1000ns per operation");
    assert!(decode_ns_per_op < 1000, "Decode should be < 1000ns per operation");
    assert!(roundtrip_ns_per_op < 2000, "Roundtrip should be < 2000ns per operation");
}
