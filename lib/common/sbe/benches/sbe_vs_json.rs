use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use sbe::{ReadBuf, WriteBuf};
use sbe::codec::simple_codec::{SimpleDecoder, SimpleEncoder};
use sbe::codec::codec::{SbeDecode, SbeEncode};

mod complex_order;
use complex_order::ComplexOrder;

/// SBE 序列化 benchmark
fn bench_sbe_encode(c: &mut Criterion) {
    let order = ComplexOrder::sample();
    let mut buffer = vec![0u8; 4096];

    let mut group = c.benchmark_group("sbe_encode");
    group.throughput(Throughput::Elements(1));

    group.bench_function("ComplexOrder", |b| {
        b.iter(|| {
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = SimpleEncoder::new(write_buf);
            black_box(&order).sbe_encode(&mut encoder).unwrap();
            black_box(encoder.finish())
        });
    });

    group.finish();
}

/// SBE 反序列化 benchmark
fn bench_sbe_decode(c: &mut Criterion) {
    let order = ComplexOrder::sample();
    let mut buffer = vec![0u8; 4096];

    // 预先编码
    let write_buf = WriteBuf::new(&mut buffer);
    let mut encoder = SimpleEncoder::new(write_buf);
    order.sbe_encode(&mut encoder).unwrap();
    let len = encoder.finish();

    let mut group = c.benchmark_group("sbe_decode");
    group.throughput(Throughput::Elements(1));

    group.bench_function("ComplexOrder", |b| {
        b.iter(|| {
            let read_buf = ReadBuf::new(&buffer[..len]);
            let mut decoder = SimpleDecoder::new(read_buf);
            black_box(ComplexOrder::sbe_decode(&mut decoder).unwrap())
        });
    });

    group.finish();
}

/// SBE 完整往返 (编码 + 解码) benchmark
fn bench_sbe_roundtrip(c: &mut Criterion) {
    let order = ComplexOrder::sample();
    let mut buffer = vec![0u8; 4096];

    let mut group = c.benchmark_group("sbe_roundtrip");
    group.throughput(Throughput::Elements(1));

    group.bench_function("ComplexOrder", |b| {
        b.iter(|| {
            // 编码
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = SimpleEncoder::new(write_buf);
            black_box(&order).sbe_encode(&mut encoder).unwrap();
            let len = encoder.finish();

            // 解码
            let read_buf = ReadBuf::new(&buffer[..len]);
            let mut decoder = SimpleDecoder::new(read_buf);
            black_box(ComplexOrder::sbe_decode(&mut decoder).unwrap())
        });
    });

    group.finish();
}

/// JSON 序列化 benchmark
fn bench_json_encode(c: &mut Criterion) {
    let order = ComplexOrder::sample();

    let mut group = c.benchmark_group("json_encode");
    group.throughput(Throughput::Elements(1));

    group.bench_function("ComplexOrder", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(&order).unwrap())
        });
    });

    group.finish();
}

/// JSON 反序列化 benchmark
fn bench_json_decode(c: &mut Criterion) {
    let order = ComplexOrder::sample();
    let json = serde_json::to_string(&order).unwrap();

    let mut group = c.benchmark_group("json_decode");
    group.throughput(Throughput::Elements(1));

    group.bench_function("ComplexOrder", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<ComplexOrder>(&json).unwrap())
        });
    });

    group.finish();
}

/// JSON 完整往返 (编码 + 解码) benchmark
fn bench_json_roundtrip(c: &mut Criterion) {
    let order = ComplexOrder::sample();

    let mut group = c.benchmark_group("json_roundtrip");
    group.throughput(Throughput::Elements(1));

    group.bench_function("ComplexOrder", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&order)).unwrap();
            black_box(serde_json::from_str::<ComplexOrder>(&json).unwrap())
        });
    });

    group.finish();
}

/// 对比 SBE vs JSON 编码性能
fn bench_encode_comparison(c: &mut Criterion) {
    let order = ComplexOrder::sample();
    let mut buffer = vec![0u8; 4096];

    let mut group = c.benchmark_group("encode_comparison");
    group.throughput(Throughput::Elements(1));

    group.bench_with_input(BenchmarkId::new("SBE", "ComplexOrder"), &order, |b, order| {
        b.iter(|| {
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = SimpleEncoder::new(write_buf);
            black_box(order).sbe_encode(&mut encoder).unwrap();
            black_box(encoder.finish())
        });
    });

    group.bench_with_input(BenchmarkId::new("JSON", "ComplexOrder"), &order, |b, order| {
        b.iter(|| {
            black_box(serde_json::to_string(order).unwrap())
        });
    });

    group.finish();
}

/// 对比 SBE vs JSON 解码性能
fn bench_decode_comparison(c: &mut Criterion) {
    let order = ComplexOrder::sample();

    // 准备 SBE 数据
    let mut sbe_buffer = vec![0u8; 4096];
    let write_buf = WriteBuf::new(&mut sbe_buffer);
    let mut encoder = SimpleEncoder::new(write_buf);
    order.sbe_encode(&mut encoder).unwrap();
    let sbe_len = encoder.finish();

    // 准备 JSON 数据
    let json = serde_json::to_string(&order).unwrap();

    let mut group = c.benchmark_group("decode_comparison");
    group.throughput(Throughput::Elements(1));

    group.bench_function(BenchmarkId::new("SBE", "ComplexOrder"), |b| {
        b.iter(|| {
            let read_buf = ReadBuf::new(&sbe_buffer[..sbe_len]);
            let mut decoder = SimpleDecoder::new(read_buf);
            black_box(ComplexOrder::sbe_decode(&mut decoder).unwrap())
        });
    });

    group.bench_function(BenchmarkId::new("JSON", "ComplexOrder"), |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<ComplexOrder>(&json).unwrap())
        });
    });

    group.finish();
}

/// 对比 SBE vs JSON 完整往返性能
fn bench_roundtrip_comparison(c: &mut Criterion) {
    let order = ComplexOrder::sample();
    let mut buffer = vec![0u8; 4096];

    let mut group = c.benchmark_group("roundtrip_comparison");
    group.throughput(Throughput::Elements(1));

    group.bench_with_input(BenchmarkId::new("SBE", "ComplexOrder"), &order, |b, order| {
        b.iter(|| {
            // 编码
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = SimpleEncoder::new(write_buf);
            black_box(order).sbe_encode(&mut encoder).unwrap();
            let len = encoder.finish();

            // 解码
            let read_buf = ReadBuf::new(&buffer[..len]);
            let mut decoder = SimpleDecoder::new(read_buf);
            black_box(ComplexOrder::sbe_decode(&mut decoder).unwrap())
        });
    });

    group.bench_with_input(BenchmarkId::new("JSON", "ComplexOrder"), &order, |b, order| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(order)).unwrap();
            black_box(serde_json::from_str::<ComplexOrder>(&json).unwrap())
        });
    });

    group.finish();
}

/// 对比编码后的数据大小
fn bench_size_comparison(c: &mut Criterion) {
    let order = ComplexOrder::sample();
    let mut buffer = vec![0u8; 4096];

    // SBE 编码大小
    let write_buf = WriteBuf::new(&mut buffer);
    let mut encoder = SimpleEncoder::new(write_buf);
    order.sbe_encode(&mut encoder).unwrap();
    let sbe_size = encoder.finish();

    // JSON 编码大小
    let json = serde_json::to_string(&order).unwrap();
    let json_size = json.len();

    println!("\n=== 编码大小对比 ===");
    println!("SBE:  {} bytes", sbe_size);
    println!("JSON: {} bytes", json_size);
    println!("压缩率: {:.2}% (SBE相比JSON)", (sbe_size as f64 / json_size as f64) * 100.0);
    println!("节省: {} bytes ({:.2}%)", json_size - sbe_size, ((json_size - sbe_size) as f64 / json_size as f64) * 100.0);

    // 这个不是真正的benchmark，只是为了显示大小信息
    let mut group = c.benchmark_group("size_info");
    group.bench_function("display", |b| b.iter(|| {}));
    group.finish();
}

criterion_group!(
    benches,
    bench_sbe_encode,
    bench_sbe_decode,
    bench_sbe_roundtrip,
    bench_json_encode,
    bench_json_decode,
    bench_json_roundtrip,
    bench_encode_comparison,
    bench_decode_comparison,
    bench_roundtrip_comparison,
    bench_size_comparison,
);

criterion_main!(benches);
