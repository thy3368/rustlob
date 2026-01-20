//! OrderDelta 优化版本基准测试
//!
//! 对比不同优化方案的性能
//!
//! 运行方式：
//! ```bash
//! cargo bench --bench orderdelta_optimized
//! ```

use std::num::NonZeroU64;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lob::lob::{Side, TraderId};
use base_types::mark_data::spot::level_types::{OrderChangeType, OrderDelta};

// ============================================================================
// 优化方案 2: 使用 NonZeroU64
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct OrderDeltaV2 {
    pub timestamp: u64,
    pub sequence: u64,
    pub order_id: u64,
    pub trader_id: Option<NonZeroU64>, // 8 字节而不是 16 字节！

    pub symbol_id: u32,
    pub price: u32,
    pub quantity: u32,

    pub change_type: OrderChangeType,
    pub side: Side
}

impl OrderDeltaV2 {
    #[inline]
    fn new(
        symbol_id: u32, timestamp: u64, sequence: u64, change_type: OrderChangeType, order_id: u64, side: Side,
        price: u32, quantity: u32, trader_id: Option<u64>
    ) -> Self {
        Self {
            timestamp,
            sequence,
            order_id,
            trader_id: trader_id.and_then(NonZeroU64::new),
            symbol_id,
            price,
            quantity,
            change_type,
            side
        }
    }

    #[inline]
    pub fn trader_id_u64(&self) -> Option<u64> { self.trader_id.map(|id| id.get()) }
}

// ============================================================================
// 优化方案 3: 位域压缩
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct OrderDeltaV3 {
    pub timestamp: u64,
    pub sequence: u64,
    pub order_id: u64,
    pub trader_id: Option<NonZeroU64>,

    pub symbol_id: u32,
    pub price: u32,
    pub quantity: u32,

    pub flags: u8 // change_type (2 bits) + side (1 bit)
}

impl OrderDeltaV3 {
    const SIDE_MASK: u8 = 0b0000_0001;
    const CHANGE_TYPE_MASK: u8 = 0b0000_0110;

    #[inline]
    fn new(
        symbol_id: u32, timestamp: u64, sequence: u64, change_type: OrderChangeType, order_id: u64, side: Side,
        price: u32, quantity: u32, trader_id: Option<u64>
    ) -> Self {
        let mut flags = 0u8;

        // 设置 side
        if matches!(side, Side::Sell) {
            flags |= Self::SIDE_MASK;
        }

        // 设置 change_type
        let change_bits = match change_type {
            OrderChangeType::Add => 0,
            OrderChangeType::Modify => 1,
            OrderChangeType::Delete => 2
        };
        flags |= change_bits << 1;

        Self {
            timestamp,
            sequence,
            order_id,
            trader_id: trader_id.and_then(NonZeroU64::new),
            symbol_id,
            price,
            quantity,
            flags
        }
    }

    #[inline]
    pub fn side(&self) -> Side {
        if self.flags & Self::SIDE_MASK == 0 {
            Side::Buy
        } else {
            Side::Sell
        }
    }

    #[inline]
    pub fn change_type(&self) -> OrderChangeType {
        match (self.flags & Self::CHANGE_TYPE_MASK) >> 1 {
            0 => OrderChangeType::Add,
            1 => OrderChangeType::Modify,
            2 => OrderChangeType::Delete,
            _ => unreachable!()
        }
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

#[inline]
fn create_orderdelta_v1(index: u64) -> OrderDelta {
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

#[inline]
fn create_orderdelta_v2(index: u64) -> OrderDeltaV2 {
    let trader_id_u64 = ((index % 256) as u64) | (((index / 256) % 256) as u64) << 8;
    OrderDeltaV2::new(
        1,
        1234567890 + index,
        1000 + index,
        OrderChangeType::Add,
        10000 + index,
        if index % 2 == 0 { Side::Buy } else { Side::Sell },
        50000 + (index as u32),
        100 + (index as u32),
        Some(trader_id_u64)
    )
}

#[inline]
fn create_orderdelta_v3(index: u64) -> OrderDeltaV3 {
    let trader_id_u64 = ((index % 256) as u64) | (((index / 256) % 256) as u64) << 8;
    OrderDeltaV3::new(
        1,
        1234567890 + index,
        1000 + index,
        OrderChangeType::Add,
        10000 + index,
        if index % 2 == 0 { Side::Buy } else { Side::Sell },
        50000 + (index as u32),
        100 + (index as u32),
        Some(trader_id_u64)
    )
}

// ============================================================================
// 基准测试：内存大小
// ============================================================================

fn bench_memory_size(c: &mut Criterion) {
    c.bench_function("memory_size_v1_original", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<OrderDelta>();
            black_box(size);
        });
    });

    c.bench_function("memory_size_v2_nonzero", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<OrderDeltaV2>();
            black_box(size);
        });
    });

    c.bench_function("memory_size_v3_bitfield", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<OrderDeltaV3>();
            black_box(size);
        });
    });

    // 打印实际大小
    println!("\n=== 内存大小对比 ===");
    println!("V1 (原始):     {} 字节", std::mem::size_of::<OrderDelta>());
    println!("V2 (NonZero):  {} 字节", std::mem::size_of::<OrderDeltaV2>());
    println!("V3 (位域):     {} 字节", std::mem::size_of::<OrderDeltaV3>());
    println!(
        "节省 (V2):     {} 字节 ({:.1}%)",
        std::mem::size_of::<OrderDelta>() - std::mem::size_of::<OrderDeltaV2>(),
        ((std::mem::size_of::<OrderDelta>() - std::mem::size_of::<OrderDeltaV2>()) as f64
            / std::mem::size_of::<OrderDelta>() as f64)
            * 100.0
    );
}

// ============================================================================
// 基准测试：分配 100 个实例
// ============================================================================

fn bench_allocation_100(c: &mut Criterion) {
    c.bench_function("allocation_100_v1_original", |b| {
        b.iter(|| {
            let deltas: Vec<OrderDelta> = (0..100).map(|i| create_orderdelta_v1(black_box(i))).collect();
            black_box(deltas);
        });
    });

    c.bench_function("allocation_100_v2_nonzero", |b| {
        b.iter(|| {
            let deltas: Vec<OrderDeltaV2> = (0..100).map(|i| create_orderdelta_v2(black_box(i))).collect();
            black_box(deltas);
        });
    });

    c.bench_function("allocation_100_v3_bitfield", |b| {
        b.iter(|| {
            let deltas: Vec<OrderDeltaV3> = (0..100).map(|i| create_orderdelta_v3(black_box(i))).collect();
            black_box(deltas);
        });
    });
}

// ============================================================================
// 基准测试：字段访问性能
// ============================================================================

fn bench_field_access(c: &mut Criterion) {
    let delta_v1 = create_orderdelta_v1(42);
    let delta_v2 = create_orderdelta_v2(42);
    let delta_v3 = create_orderdelta_v3(42);

    c.bench_function("field_access_v1_direct", |b| {
        b.iter(|| {
            let _ = black_box(delta_v1.side);
            let _ = black_box(delta_v1.change_type);
            let _ = black_box(delta_v1.price);
        });
    });

    c.bench_function("field_access_v2_direct", |b| {
        b.iter(|| {
            let _ = black_box(delta_v2.side);
            let _ = black_box(delta_v2.change_type);
            let _ = black_box(delta_v2.price);
        });
    });

    c.bench_function("field_access_v3_bitfield", |b| {
        b.iter(|| {
            let _ = black_box(delta_v3.side());
            let _ = black_box(delta_v3.change_type());
            let _ = black_box(delta_v3.price);
        });
    });
}

// ============================================================================
// 基准测试：trader_id 访问
// ============================================================================

fn bench_trader_id_access(c: &mut Criterion) {
    let delta_v1 = create_orderdelta_v1(42);
    let delta_v2 = create_orderdelta_v2(42);

    c.bench_function("trader_id_access_v1", |b| {
        b.iter(|| {
            let _ = black_box(delta_v1.trader_id);
        });
    });

    c.bench_function("trader_id_access_v2", |b| {
        b.iter(|| {
            let _ = black_box(delta_v2.trader_id_u64());
        });
    });
}

// ============================================================================
// 基准测试：顺序访问（缓存性能）
// ============================================================================

fn bench_sequential_access(c: &mut Criterion) {
    let deltas_v1: Vec<OrderDelta> = (0..1000).map(create_orderdelta_v1).collect();

    let deltas_v2: Vec<OrderDeltaV2> = (0..1000).map(create_orderdelta_v2).collect();

    c.bench_function("sequential_access_v1", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for delta in &deltas_v1 {
                sum += black_box(delta.price) as u64;
                sum += black_box(delta.quantity) as u64;
            }
            black_box(sum);
        });
    });

    c.bench_function("sequential_access_v2", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for delta in &deltas_v2 {
                sum += black_box(delta.price) as u64;
                sum += black_box(delta.quantity) as u64;
            }
            black_box(sum);
        });
    });
}

// ============================================================================
// 基准测试：Copy 性能
// ============================================================================

fn bench_copy_performance(c: &mut Criterion) {
    let delta_v1 = create_orderdelta_v1(42);
    let delta_v2 = create_orderdelta_v2(42);

    c.bench_function("copy_v1", |b| {
        b.iter(|| {
            let copied = black_box(delta_v1);
            black_box(copied);
        });
    });

    c.bench_function("copy_v2", |b| {
        b.iter(|| {
            let copied = black_box(delta_v2);
            black_box(copied);
        });
    });
}

criterion_group!(
    benches,
    bench_memory_size,
    bench_allocation_100,
    bench_field_access,
    bench_trader_id_access,
    bench_sequential_access,
    bench_copy_performance,
);

criterion_main!(benches);
