# Rust之从0-1低时延CEX：基准测试之Criterion

## 目录

1. [什么是 Criterion](#什么是-criterion)
2. [快速开始](#快速开始)
3. [基础概念](#基础概念)
4. [编写基准测试](#编写基准测试)
5. [高级技巧](#高级技巧)
6. [最佳实践](#最佳实践)
7. [常见陷阱](#常见陷阱)
8. [实战案例](#实战案例)

---

## 什么是 Criterion

Criterion 是 Rust 生态中最流行的基准测试框架，提供：

- 📊 **统计分析**: 自动计算平均值、标准差、置信区间
- 📈 **性能回归检测**: 对比历史数据，发现性能退化
- 🎨 **可视化报告**: 生成 HTML 图表和报告
- 🔬 **精确测量**: 自动预热、异常值检测、多次采样

### 为什么需要基准测试？

```rust
// ❌ 不准确的性能测试
use std::time::Instant;

let start = Instant::now();
my_function();
let duration = start.elapsed();
println!("耗时: {:?}", duration);  // 单次测量，不可靠！
```

**问题**:
- 单次测量受噪声影响大
- 没有预热（JIT、缓存等）
- 无法检测性能回归
- 难以对比不同实现

```rust
// ✅ 使用 Criterion 的准确测试
c.bench_function("my_function", |b| {
    b.iter(|| my_function());
});
// 自动预热、多次采样、统计分析、生成报告
```

---

## 快速开始

### 1. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false  # 重要！禁用默认的 benchmark harness
```

### 2. 创建基准测试文件

创建 `benches/my_benchmark.rs`：

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 3. 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试
cargo bench --bench my_benchmark

# 只运行匹配的测试
cargo bench -- fib

# 生成 HTML 报告
cargo bench
open target/criterion/report/index.html
```

---

## 基础概念

### 1. black_box - 防止编译器优化

```rust
use criterion::black_box;

// ❌ 错误：编译器可能优化掉整个计算
c.bench_function("bad", |b| {
    b.iter(|| {
        let result = expensive_computation();
        // result 未使用，可能被优化掉
    });
});

// ✅ 正确：black_box 防止优化
c.bench_function("good", |b| {
    b.iter(|| {
        let result = expensive_computation();
        black_box(result);  // 告诉编译器：这个值会被使用
    });
});

// ✅ 更好：输入也用 black_box
c.bench_function("better", |b| {
    b.iter(|| {
        let result = expensive_computation(black_box(42));
        black_box(result);
    });
});
```

**为什么需要 black_box？**

```rust
// 编译器可能做的优化：
fn compute(x: i32) -> i32 {
    x * 2 + 1
}

// 没有 black_box
b.iter(|| compute(5));
// 编译器可能优化为：
b.iter(|| 11);  // 直接返回常量！

// 使用 black_box
b.iter(|| compute(black_box(5)));
// 编译器必须真正执行计算
```

### 2. 预热 (Warmup)

Criterion 自动进行预热，确保：
- CPU 缓存已加载
- 分支预测器已训练
- JIT 编译已完成

```rust
// 默认配置
c.bench_function("test", |b| {
    b.iter(|| my_function());
});
// 自动预热 3 秒，然后采样 5 秒

// 自定义预热时间
use criterion::*;

let mut group = c.benchmark_group("custom");
group.warm_up_time(Duration::from_secs(5));  // 预热 5 秒
group.measurement_time(Duration::from_secs(10));  // 测量 10 秒
group.bench_function("test", |b| b.iter(|| my_function()));
group.finish();
```

### 3. 采样和统计

Criterion 默认采集 100 个样本，计算：

- **平均值** (Mean): 所有样本的平均时间
- **标准差** (Std Dev): 测量的波动程度
- **中位数** (Median): 50% 的样本快于此值
- **置信区间**: 95% 置信区间，表示真实值的范围

```
Benchmarking my_function
Benchmarking my_function: Warming up for 3.0000 s
Benchmarking my_function: Collecting 100 samples in estimated 5.0002 s
Benchmarking my_function: Analyzing
my_function             time:   [142.26 ns 142.66 ns 143.10 ns]
                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                        [下界      估计值    上界]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
```

---

## 编写基准测试

### 1. 简单函数基准测试

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn bench_add(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| add(black_box(2), black_box(3)));
    });
}

criterion_group!(benches, bench_add);
criterion_main!(benches);
```

### 2. 参数化基准测试

测试不同输入大小的性能：

```rust
use criterion::{BenchmarkId, Criterion};

fn bench_vec_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_allocation");

    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    for i in 0..size {
                        vec.push(black_box(i));
                    }
                    black_box(vec);
                });
            },
        );
    }

    group.finish();
}
```

输出：
```
vec_allocation/10       time:   [45.2 ns 45.6 ns 46.0 ns]
vec_allocation/100      time:   [180.1 ns 181.2 ns 182.4 ns]
vec_allocation/1000     time:   [1.73 µs 1.74 µs 1.75 µs]
vec_allocation/10000    time:   [17.3 µs 17.4 µs 17.5 µs]
```

### 3. 对比多个实现

```rust
fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");
    let data: Vec<i32> = (0..1000).rev().collect();

    // 方法 1: 标准库排序
    group.bench_function("std_sort", |b| {
        b.iter(|| {
            let mut v = data.clone();
            v.sort();
            black_box(v);
        });
    });

    // 方法 2: 不稳定排序
    group.bench_function("std_sort_unstable", |b| {
        b.iter(|| {
            let mut v = data.clone();
            v.sort_unstable();
            black_box(v);
        });
    });

    // 方法 3: 自定义排序
    group.bench_function("custom_sort", |b| {
        b.iter(|| {
            let mut v = data.clone();
            custom_sort(&mut v);
            black_box(v);
        });
    });

    group.finish();
}
```

### 4. 带设置和清理的基准测试

```rust
fn bench_with_setup(c: &mut Criterion) {
    c.bench_function("database_query", |b| {
        // 设置阶段（不计入测量）
        let db = setup_database();

        b.iter(|| {
            // 只测量这部分
            let result = db.query(black_box("SELECT * FROM users"));
            black_box(result);
        });

        // 清理阶段（不计入测量）
        drop(db);
    });
}
```

**注意**: 如果设置很耗时，使用 `iter_batched`：

```rust
use criterion::BatchSize;

c.bench_function("with_expensive_setup", |b| {
    b.iter_batched(
        || expensive_setup(),  // 设置（不计时）
        |data| process(data),  // 测量这部分
        BatchSize::SmallInput  // 批次大小
    );
});
```

### 5. 测量吞吐量

```rust
use criterion::Throughput;

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_processing");

    for size in [1024, 4096, 16384].iter() {
        let data = vec![0u8; *size];

        // 设置吞吐量单位
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &data,
            |b, data| {
                b.iter(|| process_data(black_box(data)));
            },
        );
    }

    group.finish();
}
```

输出：
```
data_processing/1024    time:   [10.2 µs 10.3 µs 10.4 µs]
                        thrpt:  [98.5 MiB/s 99.4 MiB/s 100.3 MiB/s]
```

---

## 高级技巧

### 1. 自定义采样配置

```rust
use criterion::*;

fn custom_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("custom");

    // 自定义采样数量
    group.sample_size(200);  // 默认 100

    // 自定义预热和测量时间
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(10));

    // 自定义置信水平
    group.confidence_level(0.99);  // 默认 0.95

    // 自定义噪声阈值
    group.noise_threshold(0.05);  // 默认 0.01

    group.bench_function("test", |b| b.iter(|| my_function()));
    group.finish();
}
```

### 2. 对比基线 (Baseline)

保存当前性能作为基线，后续对比：

```bash
# 保存基线
cargo bench -- --save-baseline my-baseline

# 对比基线
cargo bench -- --baseline my-baseline
```

输出：
```
my_function             time:   [142.26 ns 142.66 ns 143.10 ns]
                        change: [-5.2% -4.8% -4.3%] (p = 0.00 < 0.05)
                        Performance has improved.  # 性能提升！
```

### 3. 使用 Criterion.rs 的绘图功能

```rust
use criterion::*;

fn plot_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("plot");

    // 启用绘图
    group.plot_config(PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic));

    for size in [10, 100, 1000, 10000, 100000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| allocate_vec(black_box(size)));
            },
        );
    }

    group.finish();
}
```

### 4. 测量内存分配

```rust
use criterion::*;

fn bench_allocations(c: &mut Criterion) {
    c.bench_function("with_allocation", |b| {
        b.iter(|| {
            let v = vec![1, 2, 3, 4, 5];
            black_box(v);
        });
    });

    c.bench_function("without_allocation", |b| {
        let v = vec![1, 2, 3, 4, 5];
        b.iter(|| {
            black_box(&v);
        });
    });
}
```

### 5. 多线程基准测试

```rust
use std::thread;

fn bench_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel");

    for threads in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(threads),
            threads,
            |b, &threads| {
                b.iter(|| {
                    let handles: Vec<_> = (0..threads)
                        .map(|_| {
                            thread::spawn(|| {
                                expensive_computation(black_box(1000));
                            })
                        })
                        .collect();

                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }

    group.finish();
}
```

---

## 最佳实践

### 1. 始终使用 black_box

```rust
// ❌ 错误
b.iter(|| compute(42));

// ✅ 正确
b.iter(|| compute(black_box(42)));

// ✅ 更好
b.iter(|| {
    let result = compute(black_box(42));
    black_box(result);
});
```

### 2. 避免在循环内分配

```rust
// ❌ 错误：每次迭代都分配
b.iter(|| {
    let data = vec![1, 2, 3, 4, 5];
    process(&data);
});

// ✅ 正确：在外部分配
let data = vec![1, 2, 3, 4, 5];
b.iter(|| {
    process(black_box(&data));
});
```

### 3. 使用合适的批次大小

```rust
use criterion::BatchSize;

// 设置很快
b.iter_batched(
    || cheap_setup(),
    |data| process(data),
    BatchSize::SmallInput  // 每次迭代都设置
);

// 设置很慢
b.iter_batched(
    || expensive_setup(),
    |data| process(data),
    BatchSize::LargeInput  // 多次迭代共享一次设置
);
```

### 4. 测试真实场景

```rust
// ❌ 不现实：数据太小
b.iter(|| sort(black_box(&[1, 2, 3])));

// ✅ 现实：使用实际大小
let data: Vec<_> = (0..10000).collect();
b.iter(|| sort(black_box(&data)));
```

### 5. 组织基准测试

```rust
// 按功能分组
fn bench_allocation(c: &mut Criterion) { /* ... */ }
fn bench_sorting(c: &mut Criterion) { /* ... */ }
fn bench_searching(c: &mut Criterion) { /* ... */ }

criterion_group!(
    benches,
    bench_allocation,
    bench_sorting,
    bench_searching
);
criterion_main!(benches);
```

---

## 常见陷阱

### 1. 忘记使用 black_box

```rust
// ❌ 问题：编译器优化掉了计算
c.bench_function("bad", |b| {
    b.iter(|| {
        let x = 2 + 2;  // 编译器：这是常量 4
        // x 未使用，整个计算被优化掉
    });
});
// 结果：测量的是空循环，不是实际计算！

// ✅ 解决
c.bench_function("good", |b| {
    b.iter(|| {
        let x = black_box(2) + black_box(2);
        black_box(x);
    });
});
```

### 2. 测量了不该测量的东西

```rust
// ❌ 问题：测量了 clone
c.bench_function("bad", |b| {
    let data = vec![1; 1000];
    b.iter(|| {
        let cloned = data.clone();  // 这个 clone 被计入时间！
        process(&cloned);
    });
});

// ✅ 解决：使用 iter_batched
c.bench_function("good", |b| {
    b.iter_batched(
        || vec![1; 1000],  // 设置（不计时）
        |data| process(&data),  // 只测量这个
        BatchSize::SmallInput
    );
});
```

### 3. 缓存效应

```rust
// ❌ 问题：数据在缓存中，不真实
let data = vec![1; 1000];
b.iter(|| {
    process(black_box(&data));  // 数据一直在 L1 缓存
});

// ✅ 解决：测试冷缓存场景
b.iter_batched(
    || vec![1; 1000],  // 每次新数据
    |data| process(&data),
    BatchSize::SmallInput
);
```

### 4. 测量时间太短

```rust
// ❌ 问题：函数太快，测量不准
c.bench_function("too_fast", |b| {
    b.iter(|| black_box(1 + 1));  // < 1ns，噪声大
});

// ✅ 解决：批量执行
c.bench_function("batched", |b| {
    b.iter(|| {
        for _ in 0..1000 {
            black_box(1 + 1);
        }
    });
});
```

### 5. 不稳定的测量环境

```bash
# ❌ 问题：后台程序干扰
cargo bench  # 同时运行浏览器、IDE、音乐播放器

# ✅ 解决：
# 1. 关闭不必要的程序
# 2. 固定 CPU 频率
sudo cpupower frequency-set --governor performance

# 3. 禁用超线程（如果需要）
echo off | sudo tee /sys/devices/system/cpu/smt/control

# 4. 绑定到特定 CPU 核心
taskset -c 0 cargo bench
```

---

## 实战案例

### 案例 1: OrderBookDelta 分配优化

这是本项目的实际案例，展示如何对比不同实现：

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// 原始版本
#[derive(Clone, Copy)]
struct OrderDeltaV1 {
    // ... 56 字节
    trader_id: Option<TraderId>,  // 16 字节
}

// 优化版本
#[derive(Clone, Copy)]
struct OrderDeltaV2 {
    // ... 48 字节
    trader_id: Option<NonZeroU64>,  // 8 字节
}

fn bench_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation");

    // 测试 V1
    group.bench_function("v1_100_items", |b| {
        b.iter(|| {
            let deltas: Vec<OrderDeltaV1> = (0..100)
                .map(|i| create_delta_v1(black_box(i)))
                .collect();
            black_box(deltas);
        });
    });

    // 测试 V2
    group.bench_function("v2_100_items", |b| {
        b.iter(|| {
            let deltas: Vec<OrderDeltaV2> = (0..100)
                .map(|i| create_delta_v2(black_box(i)))
                .collect();
            black_box(deltas);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_allocation);
criterion_main!(benches);
```

**结果**:
```
allocation/v1_100_items time:   [142.26 ns 142.66 ns 143.10 ns]
allocation/v2_100_items time:   [127.33 ns 127.60 ns 127.90 ns]
                        change: [-10.8% -10.3% -9.8%] (p = 0.00 < 0.05)
                        Performance has improved.  # V2 快 10.3%！
```

### 案例 2: 字符串处理优化

```rust
fn bench_string_concat(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_concat");
    let strings: Vec<String> = (0..100)
        .map(|i| format!("string_{}", i))
        .collect();

    // 方法 1: + 操作符
    group.bench_function("plus_operator", |b| {
        b.iter(|| {
            let mut result = String::new();
            for s in &strings {
                result = result + s;  // 每次都重新分配！
            }
            black_box(result);
        });
    });

    // 方法 2: push_str
    group.bench_function("push_str", |b| {
        b.iter(|| {
            let mut result = String::new();
            for s in &strings {
                result.push_str(s);
            }
            black_box(result);
        });
    });

    // 方法 3: 预分配容量
    group.bench_function("with_capacity", |b| {
        let total_len: usize = strings.iter().map(|s| s.len()).sum();
        b.iter(|| {
            let mut result = String::with_capacity(total_len);
            for s in &strings {
                result.push_str(s);
            }
            black_box(result);
        });
    });

    // 方法 4: join
    group.bench_function("join", |b| {
        b.iter(|| {
            let result = strings.join("");
            black_box(result);
        });
    });

    group.finish();
}
```

**结果**:
```
string_concat/plus_operator   time:   [45.2 µs 45.6 µs 46.0 µs]
string_concat/push_str        time:   [12.3 µs 12.4 µs 12.5 µs]  # 快 3.7x
string_concat/with_capacity   time:   [8.1 µs 8.2 µs 8.3 µs]     # 快 5.6x
string_concat/join            time:   [7.9 µs 8.0 µs 8.1 µs]     # 最快！
```

### 案例 3: 缓存友好性测试

```rust
fn bench_cache_locality(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache");
    let size = 1000;

    // 顺序访问（缓存友好）
    group.bench_function("sequential", |b| {
        let data: Vec<i32> = (0..size).collect();
        b.iter(|| {
            let mut sum = 0;
            for i in 0..size {
                sum += black_box(data[i]);
            }
            black_box(sum);
        });
    });

    // 随机访问（缓存不友好）
    group.bench_function("random", |b| {
        let data: Vec<i32> = (0..size).collect();
        let indices: Vec<usize> = {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            let mut v: Vec<_> = (0..size).collect();
            v.shuffle(&mut rng);
            v
        };

        b.iter(|| {
            let mut sum = 0;
            for &i in &indices {
                sum += black_box(data[i]);
            }
            black_box(sum);
        });
    });

    group.finish();
}
```

**结果**:
```
cache/sequential        time:   [580.3 ns 582.6 ns 585.3 ns]
cache/random            time:   [2.1 µs 2.2 µs 2.3 µs]  # 慢 3.8x！
```

---

## 调试技巧

### 1. 查看生成的汇编代码

```bash
# 安装 cargo-asm
cargo install cargo-show-asm

# 查看函数的汇编
cargo asm --bench my_benchmark my_function

# 查看优化后的 LLVM IR
cargo llvm-ir --bench my_benchmark my_function
```

### 2. 使用 profiler

```bash
# Linux: perf
cargo bench --bench my_benchmark -- --profile-time=5

# macOS: Instruments
cargo bench --bench my_benchmark
# 然后用 Instruments 附加到进程

# 通用: flamegraph
cargo install flamegraph
cargo flamegraph --bench my_benchmark
```

### 3. 检查是否被优化掉

```rust
// 添加 println! 检查
c.bench_function("debug", |b| {
    b.iter(|| {
        let result = compute(black_box(42));
        println!("Result: {}", result);  // 临时调试
        black_box(result);
    });
});
```

---

## 总结

### Criterion 基准测试检查清单

- [ ] 添加 `criterion` 依赖和 `[[bench]]` 配置
- [ ] 使用 `black_box` 防止编译器优化
- [ ] 使用 `iter_batched` 分离设置和测量
- [ ] 测试多个输入大小（参数化）
- [ ] 对比多个实现
- [ ] 设置合适的采样配置
- [ ] 保存基线用于回归检测
- [ ] 在稳定环境中运行
- [ ] 查看 HTML 报告分析结果
- [ ] 记录优化前后的性能数据

### 常用命令速查

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

# 生成报告
cargo bench
open target/criterion/report/index.html

# 列出所有基准测试
cargo bench -- --list

# 快速测试（减少采样）
cargo bench -- --quick
```

### 进一步学习

- 📚 [Criterion.rs 官方文档](https://bheisler.github.io/criterion.rs/book/)
- 📖 [Rust 性能手册](https://nnethercote.github.io/perf-book/)
- 🎥 [Jon Gjengset 的性能视频](https://www.youtube.com/c/JonGjengset)
- 📊 [Benchmarking 最佳实践](https://easyperf.net/blog/)

---

**最后更新**: 2025-12-10
**版本**: 1.0.0
