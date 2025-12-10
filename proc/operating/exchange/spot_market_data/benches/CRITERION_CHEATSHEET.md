# Criterion 基准测试速查表

## 快速开始

### 1. 添加依赖

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false  # 必须！
```

### 2. 最简单的基准测试

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn my_function(n: u64) -> u64 {
    n * 2
}

fn bench(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| my_function(black_box(20)));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
```

### 3. 运行

```bash
cargo bench
```

---

## 核心 API

### black_box - 防止优化

```rust
// ❌ 错误 - 可能被优化掉
b.iter(|| compute(42));

// ✅ 正确
b.iter(|| {
    let result = compute(black_box(42));
    black_box(result);
});
```

### bench_function - 简单测试

```rust
c.bench_function("test_name", |b| {
    b.iter(|| {
        // 被测试的代码
    });
});
```

### bench_with_input - 参数化测试

```rust
use criterion::BenchmarkId;

let mut group = c.benchmark_group("group_name");

for size in [10, 100, 1000].iter() {
    group.bench_with_input(
        BenchmarkId::from_parameter(size),
        size,
        |b, &size| {
            b.iter(|| test_function(black_box(size)));
        },
    );
}

group.finish();
```

### iter_batched - 带设置的测试

```rust
use criterion::BatchSize;

c.bench_function("with_setup", |b| {
    b.iter_batched(
        || expensive_setup(),      // 设置（不计时）
        |data| process(data),       // 测试（计时）
        BatchSize::SmallInput       // 批次大小
    );
});
```

---

## 常用配置

### 自定义采样

```rust
let mut group = c.benchmark_group("custom");

group.sample_size(200);                              // 采样数（默认 100）
group.warm_up_time(Duration::from_secs(5));          // 预热时间（默认 3s）
group.measurement_time(Duration::from_secs(10));     // 测量时间（默认 5s）
group.confidence_level(0.99);                        // 置信水平（默认 0.95）

group.bench_function("test", |b| b.iter(|| test()));
group.finish();
```

### 吞吐量测量

```rust
use criterion::Throughput;

group.throughput(Throughput::Bytes(data.len() as u64));
group.bench_function("test", |b| {
    b.iter(|| process(black_box(&data)));
});
```

---

## 命令行用法

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试
cargo bench --bench my_benchmark

# 只运行匹配的测试
cargo bench -- pattern

# 保存基线
cargo bench -- --save-baseline my-baseline

# 对比基线
cargo bench -- --baseline my-baseline

# 列出所有测试
cargo bench -- --list

# 快速测试（减少采样）
cargo bench -- --quick

# 查看帮助
cargo bench -- --help
```

---

## 常见模式

### 对比多个实现

```rust
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");

    group.bench_function("method_1", |b| {
        b.iter(|| method_1(black_box(100)));
    });

    group.bench_function("method_2", |b| {
        b.iter(|| method_2(black_box(100)));
    });

    group.finish();
}
```

### 测试不同大小

```rust
fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("sizes");

    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| allocate(black_box(size)));
            },
        );
    }

    group.finish();
}
```

### 预先准备数据

```rust
fn bench_with_data(c: &mut Criterion) {
    // 准备数据（不计入测量）
    let data = prepare_test_data();

    c.bench_function("test", |b| {
        b.iter(|| {
            // 只测量这部分
            process(black_box(&data));
        });
    });
}
```

### 批量操作

```rust
fn bench_batch(c: &mut Criterion) {
    c.bench_function("batch_100", |b| {
        b.iter(|| {
            for i in 0..100 {
                operation(black_box(i));
            }
        });
    });
}
```

---

## 常见陷阱

### ❌ 忘记 black_box

```rust
// 错误 - 编译器可能优化掉
b.iter(|| compute(42));

// 正确
b.iter(|| compute(black_box(42)));
```

### ❌ 测量了设置代码

```rust
// 错误 - clone 被计入时间
b.iter(|| {
    let data = expensive_data.clone();  // 不该测量这个！
    process(&data);
});

// 正确 - 使用 iter_batched
b.iter_batched(
    || expensive_data.clone(),  // 设置（不计时）
    |data| process(&data),      // 测试（计时）
    BatchSize::SmallInput
);
```

### ❌ 测量时间太短

```rust
// 错误 - 函数太快（< 1ns）
b.iter(|| black_box(1 + 1));

// 正确 - 批量执行
b.iter(|| {
    for _ in 0..1000 {
        black_box(1 + 1);
    }
});
```

### ❌ 缓存效应

```rust
// 错误 - 数据一直在缓存中
let data = vec![1; 1000];
b.iter(|| process(black_box(&data)));

// 正确 - 每次新数据
b.iter_batched(
    || vec![1; 1000],
    |data| process(&data),
    BatchSize::SmallInput
);
```

---

## BatchSize 选择

```rust
// 设置很快 - 每次迭代都设置
BatchSize::SmallInput

// 设置中等 - 每 10 次迭代设置一次
BatchSize::NumIterations(10)

// 设置很慢 - 多次迭代共享一次设置
BatchSize::LargeInput

// 每批次 1 秒 - 自动调整
BatchSize::PerIteration
```

---

## 输出解读

```
my_function             time:   [142.26 ns 142.66 ns 143.10 ns]
                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                        [下界      估计值    上界]
                        change: [-5.2% -4.8% -4.3%] (p = 0.00 < 0.05)
                        Performance has improved.

Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
```

- **时间范围**: 95% 置信区间
- **change**: 与上次运行的对比
- **p < 0.05**: 统计显著性
- **outliers**: 异常值（可能是噪声）

---

## 最佳实践

### ✅ DO

- 始终使用 `black_box`
- 在稳定环境中运行
- 测试多个输入大小
- 对比多个实现
- 查看 HTML 报告
- 保存基线用于回归检测

### ❌ DON'T

- 不要忘记 `harness = false`
- 不要在循环内分配（除非测试分配）
- 不要测量太短的时间（< 1ns）
- 不要在不稳定环境中运行
- 不要忽略异常值警告

---

## 调试技巧

### 检查是否被优化

```rust
// 添加 println! 临时调试
b.iter(|| {
    let result = compute(black_box(42));
    println!("Result: {}", result);  // 确保没被优化
    black_box(result);
});
```

### 查看汇编代码

```bash
cargo install cargo-show-asm
cargo asm --bench my_benchmark function_name
```

### 使用 profiler

```bash
# Linux
cargo bench -- --profile-time=5

# macOS
# 用 Instruments 附加到进程

# Flamegraph
cargo install flamegraph
cargo flamegraph --bench my_benchmark
```

---

## 报告和可视化

### 查看 HTML 报告

```bash
cargo bench
open target/criterion/report/index.html
```

### 报告包含

- 📊 性能图表
- 📈 历史趋势
- 📉 性能回归检测
- 📋 详细统计数据
- 🎯 异常值分析

---

## 高级用法

### 自定义绘图

```rust
use criterion::PlotConfiguration;

group.plot_config(PlotConfiguration::default()
    .summary_scale(AxisScale::Logarithmic));
```

### 多线程测试

```rust
use std::thread;

c.bench_function("parallel", |b| {
    b.iter(|| {
        let handles: Vec<_> = (0..4)
            .map(|_| thread::spawn(|| compute(black_box(100))))
            .collect();

        for h in handles {
            h.join().unwrap();
        }
    });
});
```

### 自定义统计

```rust
use criterion::*;

let mut group = c.benchmark_group("custom");
group.significance_level(0.01);  // 更严格的显著性
group.noise_threshold(0.05);     // 噪声阈值
```

---

## 快速参考

| 操作 | 代码 |
|------|------|
| 简单测试 | `c.bench_function("name", \|b\| b.iter(\|\| test()))` |
| 参数化 | `group.bench_with_input(id, param, \|b, p\| ...)` |
| 带设置 | `b.iter_batched(setup, test, BatchSize)` |
| 防止优化 | `black_box(value)` |
| 测量吞吐量 | `group.throughput(Throughput::Bytes(n))` |
| 自定义配置 | `group.sample_size(200)` |
| 保存基线 | `cargo bench -- --save-baseline name` |
| 对比基线 | `cargo bench -- --baseline name` |

---

## 相关资源

- 📚 [完整教程](./CRITERION_TUTORIAL.md)
- 📝 [注释示例](./orderbook_delta_allocation_annotated.rs)
- 📊 [性能分析](./PERFORMANCE_ANALYSIS.md)
- 🔗 [官方文档](https://bheisler.github.io/criterion.rs/book/)

---

**提示**: 将此文件保存为书签，随时查阅！
