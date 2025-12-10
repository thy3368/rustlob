//! OrderBookDelta 分配性能基准测试 - 详细注释教学版
//!
//! 这是一个完整的 Criterion 基准测试示例，包含详细的注释和最佳实践说明。
//! 适合作为学习 Criterion 的参考代码。
//!
//! # 学习目标
//!
//! 1. 理解 Criterion 的基本用法
//! 2. 学会使用 black_box 防止编译器优化
//! 3. 掌握参数化基准测试
//! 4. 了解如何对比不同实现
//!
//! # 运行方式
//!
//! ```bash
//! # 运行所有基准测试
//! cargo bench --bench orderbook_delta_allocation_annotated
//!
//! # 只运行特定测试
//! cargo bench --bench orderbook_delta_allocation_annotated -- single
//!
//! # 查看 HTML 报告
//! cargo bench --bench orderbook_delta_allocation_annotated
//! open target/criterion/report/index.html
//! ```

// ============================================================================
// 导入必要的依赖
// ============================================================================

use criterion::{
    black_box,           // 防止编译器优化掉测试代码
    criterion_group,     // 定义基准测试组
    criterion_main,      // 定义主入口
    BenchmarkId,         // 参数化测试的 ID
    Criterion,           // 基准测试上下文
};

use lob::lob::{Side, TraderId};
use spot_market_data::domain::entity::level_types::{OrderChangeType, OrderDelta};

// ============================================================================
// 辅助函数：创建测试数据
// ============================================================================

/// 创建一个示例 OrderDelta
///
/// # 为什么使用 #[inline]？
///
/// `#[inline]` 提示编译器内联这个函数，减少函数调用开销。
/// 在基准测试中，我们希望测量的是数据结构的创建成本，而不是函数调用开销。
///
/// # 参数
///
/// * `index` - 用于生成不同的测试数据，避免编译器优化
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
        trader_id: Some(TraderId::new([
            (index % 256) as u8,
            ((index / 256) % 256) as u8,
            1, 2, 3, 4, 5, 6,
        ])),
    }
}

// ============================================================================
// 基准测试 1: 单次分配
// ============================================================================

/// 基准测试：单次分配 OrderDelta
///
/// # 测试目标
///
/// 测量创建单个 OrderDelta 实例的时间。
///
/// # Criterion 基础概念
///
/// - `c.bench_function()`: 创建一个基准测试
/// - `|b|`: 闭包参数 b 是 Bencher，用于执行测试
/// - `b.iter()`: 执行测试代码，Criterion 会自动决定运行多少次
///
/// # black_box 的作用
///
/// ```rust
/// // ❌ 没有 black_box - 编译器可能优化掉
/// b.iter(|| {
///     let delta = create_orderbook_delta(0);
///     // delta 未使用，编译器可能删除整个计算
/// });
///
/// // ✅ 使用 black_box - 告诉编译器这个值会被使用
/// b.iter(|| {
///     let delta = create_orderbook_delta(black_box(0));
///     black_box(delta);  // 确保 delta 不被优化掉
/// });
/// ```
fn bench_single_allocation(c: &mut Criterion) {
    c.bench_function("single_orderbook_delta_allocation", |b| {
        b.iter(|| {
            // black_box(0): 防止编译器将 0 优化为常量
            let delta = create_orderbook_delta(black_box(0));

            // black_box(delta): 防止编译器优化掉整个 delta
            black_box(delta);
        });
    });
}

// ============================================================================
// 基准测试 2: Vec 分配（无预分配）
// ============================================================================

/// 基准测试：分配 100 个 OrderDelta 到 Vec（无预分配）
///
/// # 测试目标
///
/// 测量不预分配容量时，Vec 的性能表现。
///
/// # 为什么慢？
///
/// Vec::new() 初始容量为 0，随着元素增加会多次重新分配：
/// - 0 -> 4 -> 8 -> 16 -> 32 -> 64 -> 128
/// - 每次重新分配都需要：
///   1. 分配新内存
///   2. 拷贝旧数据
///   3. 释放旧内存
///
/// # 预期结果
///
/// 这应该是最慢的方法，因为有多次内存重新分配。
fn bench_vec_allocation_no_reserve(c: &mut Criterion) {
    c.bench_function("vec_100_orderbook_deltas_no_reserve", |b| {
        b.iter(|| {
            let mut deltas = Vec::new();  // 初始容量为 0

            for i in 0..100 {
                // black_box(i): 防止编译器展开循环或优化掉循环
                deltas.push(create_orderbook_delta(black_box(i)));
            }

            // black_box(deltas): 确保 Vec 不被优化掉
            black_box(deltas);
        });
    });
}

// ============================================================================
// 基准测试 3: Vec 分配（预分配容量）
// ============================================================================

/// 基准测试：分配 100 个 OrderDelta 到 Vec（预分配容量）
///
/// # 测试目标
///
/// 测量预分配容量时，Vec 的性能表现。
///
/// # 为什么快？
///
/// Vec::with_capacity(100) 一次性分配足够的内存：
/// - 只分配一次内存
/// - 无需重新分配
/// - 无需拷贝数据
///
/// # 预期结果
///
/// 比无预分配快约 2 倍。
fn bench_vec_allocation_with_reserve(c: &mut Criterion) {
    c.bench_function("vec_100_orderbook_deltas_with_reserve", |b| {
        b.iter(|| {
            // 预分配 100 个元素的容量
            let mut deltas = Vec::with_capacity(100);

            for i in 0..100 {
                deltas.push(create_orderbook_delta(black_box(i)));
            }

            black_box(deltas);
        });
    });
}

// ============================================================================
// 基准测试 4: 使用 collect()
// ============================================================================

/// 基准测试：使用 collect 分配 100 个 OrderDelta
///
/// # 测试目标
///
/// 测量使用迭代器 + collect() 的性能。
///
/// # 为什么最快？
///
/// 1. **编译器可见性**: 编译器能看到完整的数据流
/// 2. **精确大小提示**: Range 迭代器提供确切的大小
/// 3. **优化友好**: 可以应用循环展开、向量化等优化
/// 4. **无运行时检查**: 使用 unsafe 快速路径
///
/// # 内部实现（简化）
///
/// ```rust
/// impl<T> FromIterator<T> for Vec<T> {
///     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
///         let mut iter = iter.into_iter();
///         let (lower, upper) = iter.size_hint();
///
///         if upper == Some(lower) {
///             // 精确大小已知 - 一次性分配
///             let mut vec = Vec::with_capacity(lower);
///             for item in iter {
///                 // 使用 unsafe 快速路径，跳过容量检查
///                 vec.push_unchecked(item);
///             }
///             vec
///         } else {
///             // 大小未知 - 增量分配
///             // ...
///         }
///     }
/// }
/// ```
///
/// # 预期结果
///
/// 最快的方法，比手动循环快约 20-30%。
fn bench_vec_allocation_collect(c: &mut Criterion) {
    c.bench_function("vec_100_orderbook_deltas_collect", |b| {
        b.iter(|| {
            // 使用迭代器 + collect()
            // Range (0..100) 提供精确的大小提示
            let deltas: Vec<OrderDelta> = (0..100)
                .map(|i| create_orderbook_delta(black_box(i)))
                .collect();

            black_box(deltas);
        });
    });
}

// ============================================================================
// 基准测试 5: 栈数组分配
// ============================================================================

/// 基准测试：分配 100 个 OrderDelta 到栈数组
///
/// # 测试目标
///
/// 测量栈分配的性能。
///
/// # 栈 vs 堆
///
/// - **栈分配**: 快速，但大小固定，空间有限（通常 1-8 MB）
/// - **堆分配**: 灵活，但需要分配器开销
///
/// # 为什么可能更慢？
///
/// 1. **栈空间开销**: 100 * 56 字节 = 5,600 字节
/// 2. **初始化顺序**: 必须按顺序初始化
/// 3. **无法并行**: 不能利用 SIMD 优化
/// 4. **栈溢出风险**: 大数组可能导致栈溢出
///
/// # 预期结果
///
/// 比 collect() 慢约 2 倍。
fn bench_array_allocation(c: &mut Criterion) {
    c.bench_function("array_100_orderbook_deltas", |b| {
        b.iter(|| {
            // std::array::from_fn: 使用闭包初始化数组
            let deltas: [OrderDelta; 100] = std::array::from_fn(|i| {
                create_orderbook_delta(black_box(i as u64))
            });

            black_box(deltas);
        });
    });
}

// ============================================================================
// 基准测试 6: 参数化测试
// ============================================================================

/// 基准测试：不同数量的 OrderDelta 分配（参数化测试）
///
/// # 测试目标
///
/// 测量不同数量下的性能表现，找出性能特征。
///
/// # 参数化测试的优势
///
/// 1. **发现性能拐点**: 找出性能突变的临界点
/// 2. **验证复杂度**: 验证算法的时间复杂度（O(n)、O(n²) 等）
/// 3. **对比不同规模**: 小数据 vs 大数据的表现
///
/// # BenchmarkGroup 的作用
///
/// - 将相关的基准测试组织在一起
/// - 生成对比图表
/// - 共享配置（采样数、预热时间等）
///
/// # 预期结果
///
/// 应该呈现线性关系：时间 ∝ 数量
fn bench_varying_sizes(c: &mut Criterion) {
    // 创建基准测试组
    let mut group = c.benchmark_group("orderbook_delta_allocation_varying_sizes");

    // 测试不同的大小
    for size in [10, 50, 100, 200, 500, 1000].iter() {
        // bench_with_input: 参数化测试
        // BenchmarkId: 为每个参数创建唯一的 ID
        group.bench_with_input(
            BenchmarkId::new("vec_with_reserve", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut deltas = Vec::with_capacity(size);
                    for i in 0..size {
                        deltas.push(create_orderbook_delta(black_box(i as u64)));
                    }
                    black_box(deltas);
                });
            },
        );
    }

    // 完成测试组（生成报告）
    group.finish();
}

// ============================================================================
// 基准测试 7: Copy 性能
// ============================================================================

/// 基准测试：OrderDelta 的 Copy 性能
///
/// # 测试目标
///
/// 测量 Copy trait 的性能（memcpy）。
///
/// # Copy vs Clone
///
/// - **Copy**: 按位拷贝，非常快（memcpy）
/// - **Clone**: 可能有额外逻辑，可能慢
///
/// # OrderDelta 的 Copy
///
/// OrderDelta 是 56 字节，Copy 就是拷贝 56 字节。
///
/// # 预期结果
///
/// 非常快，约 3-5 ns（几个 CPU 周期）。
fn bench_copy_performance(c: &mut Criterion) {
    let delta = create_orderbook_delta(0);

    c.bench_function("orderbook_delta_copy", |b| {
        b.iter(|| {
            // Copy 是隐式的
            let copied = black_box(delta);
            black_box(copied);
        });
    });
}

// ============================================================================
// 基准测试 8: Clone 性能
// ============================================================================

/// 基准测试：OrderDelta 的 Clone 性能
///
/// # 测试目标
///
/// 测量 Clone trait 的性能。
///
/// # 对于 Copy 类型
///
/// 如果类型实现了 Copy，Clone 通常就是 Copy。
///
/// # 预期结果
///
/// 应该和 Copy 性能相同。
fn bench_clone_performance(c: &mut Criterion) {
    let delta = create_orderbook_delta(0);

    c.bench_function("orderbook_delta_clone", |b| {
        b.iter(|| {
            // 显式调用 clone()
            let cloned = black_box(delta.clone());
            black_box(cloned);
        });
    });
}

// ============================================================================
// 基准测试 9: 批量 Copy
// ============================================================================

/// 基准测试：批量 Copy 100 个 OrderDelta
///
/// # 测试目标
///
/// 测量批量拷贝的性能，验证缓存效应。
///
/// # 缓存局部性
///
/// - **顺序访问**: 缓存友好，预取效率高
/// - **随机访问**: 缓存不友好，缓存未命中多
///
/// # 预期结果
///
/// 约 350 ns（每个 3.5 ns），受缓存影响。
fn bench_batch_copy(c: &mut Criterion) {
    // 预先创建源数据（不计入测量时间）
    let source: Vec<OrderDelta> = (0..100)
        .map(create_orderbook_delta)
        .collect();

    c.bench_function("batch_copy_100_orderbook_deltas", |b| {
        b.iter(|| {
            let mut dest = Vec::with_capacity(100);

            // 顺序拷贝
            for delta in source.iter() {
                dest.push(black_box(*delta));
            }

            black_box(dest);
        });
    });
}

// ============================================================================
// 基准测试 10: 内存布局分析
// ============================================================================

/// 基准测试：内存布局分析 - 测量 OrderDelta 的大小
///
/// # 测试目标
///
/// 虽然这不是真正的性能测试，但可以验证 size_of 的开销。
///
/// # size_of 是编译时常量
///
/// `std::mem::size_of::<T>()` 在编译时就确定了，运行时开销为 0。
///
/// # 预期结果
///
/// 极快，约 0.3 ps（皮秒），基本是测量噪声。
fn bench_memory_layout(c: &mut Criterion) {
    c.bench_function("orderbook_delta_size_of", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<OrderDelta>();
            black_box(size);
        });
    });
}

// ============================================================================
// 组织基准测试
// ============================================================================

/// criterion_group! 宏
///
/// 将所有基准测试函数组织成一个组。
///
/// # 语法
///
/// ```rust
/// criterion_group!(
///     组名,
///     测试函数1,
///     测试函数2,
///     ...
/// );
/// ```
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

/// criterion_main! 宏
///
/// 生成 main 函数，运行所有基准测试。
///
/// # 语法
///
/// ```rust
/// criterion_main!(组名1, 组名2, ...);
/// ```
///
/// # 为什么需要这个？
///
/// Criterion 需要禁用 Rust 默认的 benchmark harness（在 Cargo.toml 中设置 `harness = false`），
/// 所以需要自己提供 main 函数。
criterion_main!(benches);

// ============================================================================
// 学习总结
// ============================================================================

/*
# Criterion 基准测试关键要点

## 1. 必须使用 black_box

```rust
// ❌ 错误
b.iter(|| compute(42));

// ✅ 正确
b.iter(|| {
    let result = compute(black_box(42));
    black_box(result);
});
```

## 2. 理解预热的重要性

Criterion 自动预热 3 秒，确保：
- CPU 缓存已加载
- 分支预测器已训练
- 稳定的测量环境

## 3. 参数化测试找出性能特征

```rust
for size in [10, 100, 1000, 10000].iter() {
    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
        // 测试代码
    });
}
```

## 4. 对比多个实现

```rust
group.bench_function("method_1", |b| { /* ... */ });
group.bench_function("method_2", |b| { /* ... */ });
group.bench_function("method_3", |b| { /* ... */ });
```

## 5. 查看 HTML 报告

```bash
cargo bench
open target/criterion/report/index.html
```

## 6. 保存基线用于回归检测

```bash
# 保存基线
cargo bench -- --save-baseline my-baseline

# 对比基线
cargo bench -- --baseline my-baseline
```

## 7. 常见陷阱

- 忘记使用 black_box
- 测量了不该测量的东西（如设置代码）
- 测量时间太短（< 1ns）
- 不稳定的测量环境

## 8. 最佳实践

- 在稳定环境中运行（关闭后台程序）
- 测试多个输入大小
- 对比多个实现
- 记录优化前后的数据
- 查看生成的图表和报告

# 进一步学习

- 📚 [Criterion.rs 官方文档](https://bheisler.github.io/criterion.rs/book/)
- 📖 [CRITERION_TUTORIAL.md](./CRITERION_TUTORIAL.md)
- 📊 [PERFORMANCE_ANALYSIS.md](./PERFORMANCE_ANALYSIS.md)
*/
