//! OrderBookDelta 分配性能基准测试
//!
//! 测试分配 100 个 OrderBookDelta 实例的性能
//!
//! # 运行方式
//!
//! ```bash
//! cargo bench --bench orderbook_delta_allocation
//! ```
//!
//! # 测试场景
//!
//! - **单次分配**: 测量分配单个 OrderBookDelta 的时间
//! - **批量分配 (Vec)**: 测量分配 100 个 OrderBookDelta 到 Vec 的时间
//! - **批量分配 (预分配)**: 测量使用预分配容量的 Vec 分配 100 个 OrderBookDelta
//!   的时间
//! - **批量分配 (数组)**: 测量分配 100 个 OrderBookDelta 到栈数组的时间

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lob::lob::{Side, TraderId};
use spot_market_data::domain::entity::level_types::{OrderChangeType, OrderDelta};

/// 创建一个示例 OrderBookDelta
#[inline]
fn create_orderbook_delta(index: u64) -> OrderDelta {
    OrderDelta {
        symbol_id: 1,
        timestamp: 1234567890 + index,
        sequence: 1000 + index,
        change_type: OrderChangeType::Add,
        order_id: 10000 + index,
        side: if index % 2 == 0 { Side::Buy } else { Side::Sell },
        price: 50000 + (index as u32),
        quantity: 100 + (index as u32),
        trader_id: Some(TraderId::new([(index % 256) as u8, ((index / 256) % 256) as u8, 1, 2, 3, 4, 5, 6]))
    }
}

/// 基准测试：单次分配 OrderBookDelta
fn bench_single_allocation(c: &mut Criterion) {
    c.bench_function("single_orderbook_delta_allocation", |b| {
        b.iter(|| {
            let delta = create_orderbook_delta(black_box(0));
            black_box(delta);
        });
    });
}

/// 基准测试：分配 100 个 OrderBookDelta 到 Vec（无预分配）
fn bench_vec_allocation_no_reserve(c: &mut Criterion) {
    c.bench_function("vec_100_orderbook_deltas_no_reserve", |b| {
        b.iter(|| {
            let mut deltas = Vec::new();
            for i in 0..100 {
                deltas.push(create_orderbook_delta(black_box(i)));
            }
            black_box(deltas);
        });
    });
}

/// 基准测试：分配 100 个 OrderBookDelta 到 Vec（预分配容量）
fn bench_vec_allocation_with_reserve(c: &mut Criterion) {
    c.bench_function("vec_100_orderbook_deltas_with_reserve", |b| {
        b.iter(|| {
            let mut deltas = Vec::with_capacity(100);
            for i in 0..100 {
                deltas.push(create_orderbook_delta(black_box(i)));
            }
            black_box(deltas);
        });
    });
}

/// 基准测试：使用 collect 分配 100 个 OrderBookDelta
fn bench_vec_allocation_collect(c: &mut Criterion) {
    c.bench_function("vec_100_orderbook_deltas_collect", |b| {
        b.iter(|| {
            let deltas: Vec<OrderDelta> = (0..100).map(|i| create_orderbook_delta(black_box(i))).collect();
            black_box(deltas);
        });
    });
}

/// 基准测试：分配 100 个 OrderBookDelta 到栈数组
fn bench_array_allocation(c: &mut Criterion) {
    c.bench_function("array_100_orderbook_deltas", |b| {
        b.iter(|| {
            let deltas: [OrderDelta; 100] = std::array::from_fn(|i| create_orderbook_delta(black_box(i as u64)));
            black_box(deltas);
        });
    });
}

/// 基准测试：不同数量的 OrderBookDelta 分配（参数化测试）
fn bench_varying_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("orderbook_delta_allocation_varying_sizes");

    for size in [10, 50, 100, 200, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("vec_with_reserve", size), size, |b, &size| {
            b.iter(|| {
                let mut deltas = Vec::with_capacity(size);
                for i in 0..size {
                    deltas.push(create_orderbook_delta(black_box(i as u64)));
                }
                black_box(deltas);
            });
        });
    }

    group.finish();
}

/// 基准测试：OrderBookDelta 的 Copy 性能
fn bench_copy_performance(c: &mut Criterion) {
    let delta = create_orderbook_delta(0);

    c.bench_function("orderbook_delta_copy", |b| {
        b.iter(|| {
            let copied = black_box(delta);
            black_box(copied);
        });
    });
}

/// 基准测试：OrderBookDelta 的 Clone 性能
fn bench_clone_performance(c: &mut Criterion) {
    let delta = create_orderbook_delta(0);

    c.bench_function("orderbook_delta_clone", |b| {
        b.iter(|| {
            let cloned = black_box(delta.clone());
            black_box(cloned);
        });
    });
}

/// 基准测试：批量 Copy 100 个 OrderBookDelta
fn bench_batch_copy(c: &mut Criterion) {
    let source: Vec<OrderDelta> = (0..100).map(create_orderbook_delta).collect();

    c.bench_function("batch_copy_100_orderbook_deltas", |b| {
        b.iter(|| {
            let mut dest = Vec::with_capacity(100);
            for delta in source.iter() {
                dest.push(black_box(*delta));
            }
            black_box(dest);
        });
    });
}

/// 基准测试：内存布局分析 - 测量 OrderBookDelta 的大小
fn bench_memory_layout(c: &mut Criterion) {
    c.bench_function("orderbook_delta_size_of", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<OrderDelta>();
            black_box(size);
        });
    });
}

criterion_group!(
    benches,
    bench_single_allocation,
    bench_vec_allocation_no_reserve,
    bench_vec_allocation_with_reserve,
    bench_vec_allocation_collect,
    bench_array_allocation,
    bench_varying_sizes,
    bench_copy_performance,
    bench_clone_performance,
    bench_batch_copy,
    bench_memory_layout,
);

criterion_main!(benches);
