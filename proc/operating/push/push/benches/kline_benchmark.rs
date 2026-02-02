use criterion::{black_box, criterion_group, criterion_main, Criterion};
use push::k_line::aggregator::k_line_aggregator::KLineAggregator;
use push::k_line::aggregator::simd_k_line_aggregator::SimdKLineAggregator;
use rand::Rng;
use std::time::SystemTime;
use push::k_line::k_line_types::KLineAgg;

// 生成模拟交易数据
fn generate_test_data(count: usize) -> Vec<(u64, f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut price = 100.0;
    let mut data = Vec::with_capacity(count);

    for _ in 0..count {
        let change = rng.gen_range(-0.5..0.5);
        let new_price = price + change;
        price = if new_price > 1.0 { new_price } else { 1.0 };
        let volume = rng.gen_range(1.0..100.0);

        data.push((timestamp, price, volume));
        timestamp += rng.gen_range(0..2);
    }

    data
}

// 基准测试 KLineAggregator 的处理
fn benchmark_kline_aggregator(c: &mut Criterion) {
    let test_data = generate_test_data(100_000); // 10万笔交易

    c.bench_function("KLineAggregator::process_trades_batch", |b| {
        b.iter(|| {
            let aggregator = KLineAggregator::new();
            aggregator.process_trades_batch(black_box(&test_data)).unwrap();
        });
    });
}

fn benchmark_kline_aggregator2(c: &mut Criterion) {
    let test_data = generate_test_data(100_000); // 10万笔交易

    c.bench_function("KLineAggregator::process_trades_batch", |b| {
        b.iter(|| {
            let aggregator = SimdKLineAggregator::new();
            aggregator.process_trades_batch(black_box(&test_data)).unwrap();
        });
    });
}





criterion_group!(
    benches,
    benchmark_kline_aggregator,
    benchmark_kline_aggregator2,

);
criterion_main!(benches);