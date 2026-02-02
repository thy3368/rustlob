# SingleThreadSimdKLineAggregator 优化方案

## 目标

将 SingleThreadSimdKLineAggregator 的性能从当前的 **2.78M 交易/秒** 提升到 **1000M 交易/秒**（即 **100万笔交易/1ms**）。

## 当前性能分析

### 基准测试结果（高频场景）
- KLineAggregator: 1.94ms/10万笔交易 → 51.5M 交易/秒
- SimdKLineAggregator: 435µs/10万笔交易 → 230M 交易/秒
- SingleThreadSimdKLineAggregator: 360µs/10万笔交易 → **278M 交易/秒** (当前最佳)

### 需要提升倍数
1000M / 278M = **约3.6倍**

## 优化方案

### 1. 硬件层面优化

#### 1.1 CPU架构优化
```toml
# Cargo.toml - 优化配置
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
target-cpu = "native"
rustflags = [
    "-Ctarget-cpu=native",
    "-Cllvm-args=--inline-threshold=500",
    "-Cllvm-args=--unroll-count=16",
    "-Cllvm-args=--enable-machine-outliner=always",
    "-Cllvm-args=--disable-branch-fold",
    "-Cllvm-args=--x86-speculative-load-hardening=off",
    "-Cllvm-args=--no-stack-protector",
    "-Cstrip=symbols"
]
```

#### 1.2 内存架构优化
```rust
// 使用内存对齐和预取优化
#[repr(align(64))] // 缓存行对齐
struct AlignedData {
    data: [f64; 8],
}

// 使用更高效的内存分配器
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// 或者使用 jemalloc（需要 jemallocator 板条箱）
// use jemallocator::Jemalloc;
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;
```

### 2. 算法层面优化

#### 2.1 数据结构优化
```rust
// 完全重写数据结构，避免使用 VecDeque
// 使用固定大小的数组和指针运算
struct FixedSizeRingBuffer {
    data: *mut OHLC,
    capacity: usize,
    mask: usize,
    head: usize,
    tail: usize,
}

impl FixedSizeRingBuffer {
    #[inline(always)]
    fn push(&mut self, ohlc: OHLC) {
        let index = self.head & self.mask;
        unsafe { *self.data.add(index) = ohlc };
        self.head += 1;

        if self.head - self.tail > self.capacity {
            self.tail += 1;
        }
    }

    #[inline(always)]
    fn back(&self) -> Option<&OHLC> {
        if self.head == self.tail {
            None
        } else {
            unsafe { Some(&*self.data.add((self.head - 1) & self.mask)) }
        }
    }
}
```

#### 2.2 批处理优化
```rust
// 增加批处理大小到 1024 或更大
const OPTIMAL_BATCH_SIZE: usize = 1024;
struct BatchProcessor {
    buffer: [u64; OPTIMAL_BATCH_SIZE],
    ptr: usize,
}

impl BatchProcessor {
    #[inline(always)]
    fn add(&mut self, value: u64) {
        self.buffer[self.ptr] = value;
        self.ptr += 1;
        if self.ptr == OPTIMAL_BATCH_SIZE {
            self.process_batch();
            self.ptr = 0;
        }
    }

    #[inline(always)]
    fn process_batch(&self) {
        // SIMD处理整个批次
        unsafe {
            let ptr = self.buffer.as_ptr() as *const __m256i;
            // 在这里进行高级SIMD操作
        }
    }
}
```

### 3. 编译器层面优化

#### 3.1 使用不稳定特性
```rust
#![feature(portable_simd)]
#![feature(simd_ffi)]
#![feature(integer_atomics)]
#![feature(never_type)]
#![feature(bench_black_box)]
#![feature(inline_const)]
```

#### 3.2 链接时优化
```rust
// 在Cargo.toml中启用链接时优化
[profile.release]
lto = "fat"
```

### 4. 代码层面深度优化

#### 4.1 去掉不必要的安全检查
```rust
// 使用裸指针和 unsafe 代码
struct UnsafeAggregator {
    current_windows: *mut [Option<OHLC>; 4],
    // ... 其他字段
}

impl UnsafeAggregator {
    #[inline(always)]
    unsafe fn update_window(&self, window_idx: usize, ts: u64, price: f64, vol: f64) {
        let windows = &mut *self.current_windows;
        match &mut (*windows)[window_idx] {
            Some(ohlc) if ohlc.timestamp == ts => {
                // 更新
                ohlc.high = ohlc.high.max(price);
                ohlc.low = ohlc.low.min(price);
                ohlc.close = price;
                ohlc.volume += vol;
                ohlc.count += 1;
            }
            _ => {
                // 替换
                (*windows)[window_idx] = Some(OHLC::new(ts, price, vol));
            }
        }
    }
}
```

#### 4.2 函数内联优化
```rust
#![feature(inline_const)]

// 使用 #[inline(always)] 标记所有核心函数
#[inline(always)]
fn update_window_with_stats(...) {
    // ...
}

#[inline(always)]
fn process_with_simd(...) {
    // ...
}

// 使用 const 泛型来确保编译时优化
fn process_batch<const N: usize>(...) {
    // ...
}
```

### 5. 系统级优化

#### 5.1 关闭操作系统特性
```bash
# 禁用ASLR
echo 0 | sudo tee /proc/sys/kernel/randomize_va_space

# 禁用超线程（如果需要）
echo 0 | sudo tee /sys/devices/system/cpu/smt/control

# 禁用C状态（CPU深度睡眠）
for i in /sys/devices/system/cpu/cpu[0-9]*; do
    echo 1 | sudo tee $i/cpufreq/conservative/ignore_nice_load
    echo 2 | sudo tee $i/cpufreq/conservative/freq_step
    echo 1000000 | sudo tee $i/cpufreq/conservative/sampling_rate
    echo 1 | sudo tee $i/cpufreq/scaling_governor
done
```

#### 5.2 线程亲和性
```rust
use core_affinity::CoreId;
use core_affinity::set_for_current;

fn main() {
    // 绑定到特定CPU核心
    let core_ids = core_affinity::get_core_ids().unwrap();
    set_for_current(core_ids[0]);

    // 初始化聚合器
    let mut aggregator = UnsafeAggregator::new();

    // 运行
    run(&mut aggregator);
}
```

### 6. 架构重设计

#### 6.1 管道化处理
```rust
// 使用管道化架构
struct Pipeline {
    stage1: Stage1, // 原始数据
    stage2: Stage2, // 批量处理
    stage3: Stage3, // 窗口更新
    stage4: Stage4  // 输出
}

impl Pipeline {
    fn process(&mut self, data: &[u8]) {
        let raw = self.stage1.process(data);
        let batches = self.stage2.process(&raw);
        let updated = self.stage3.process(&batches);
        let output = self.stage4.process(&updated);
    }
}
```

#### 6.2 专用硬件加速
```rust
// 使用 GPU 或 FPGA 加速（需要专业硬件）
struct GpuAggregator {
    // GPU加速实现
}

impl GpuAggregator {
    fn process_batch(&self, trades: &[TradeData]) {
        // 发送到 GPU 处理
        gpu_process(trades);
    }
}

// 在 Rust 中使用 CUDA（需要 rust-cuda 库）
#[global_attr(rustc::prelude::*)]
extern "C" fn gpu_kernel(trades: *const TradeData, count: usize) {
    let idx = blockIdx.x * 128 + threadIdx.x;
    if idx < count {
        // GPU 上的并行处理
        process_trade(idx, unsafe { &*trades.add(idx) });
    }
}
```

### 7. 测试与验证

#### 7.1 性能监控
```rust
use std::time::{Instant, SystemTime};

fn measure_performance() {
    let aggregator = UnsafeAggregator::new();
    let data = generate_test_data(10_000_000);

    let start = Instant::now();
    aggregator.process_trades_batch(&data);
    let duration = start.elapsed();

    let trades_per_second = (data.len() as f64) / duration.as_secs_f64();
    println!("{} trades per second ({:.3} ms per million)",
        trades_per_second, (1e6 / trades_per_second) * 1000.0);
}
```

#### 7.2 渐进优化
```rust
// 实现渐进式优化
mod v1 {
    // 基础版本（当前状态）
}

mod v2 {
    // 去掉 RefCell
}

mod v3 {
    // 使用数组代替 VecDeque
}

mod v4 {
    // 完全不安全的版本
}

// 测试每个版本的性能
fn compare_versions() {
    for i in 1..=4 {
        let duration = measure_version(i);
        println!("Version {}: {:.2} ms per million trades", i, duration);
    }
}
```

## 实施计划

### 阶段1（1-2周）
- 启用最大编译器优化
- 使用内存对齐和预取优化
- 重写数据结构，使用固定大小数组

### 阶段2（2-3周）
- 实施管道化处理
- 去掉所有安全检查
- 使用裸指针优化

### 阶段3（3-4周）
- 硬件级优化
- 系统级优化
- 线程亲和性设置

### 阶段4（4-6周）
- 专用硬件加速（GPU/FPGA）
- 架构重设计
- 最终性能验证

## 预期结果

| 优化阶段 | 预期性能 | 提升倍数 |
|---------|----------|----------|
| 当前版本 | 2.78M/s | 1x |
| 阶段1    | 10M/s    | 3.6x |
| 阶段2    | 50M/s    | 18x |
| 阶段3    | 200M/s   | 72x |
| 阶段4    | 1000M/s  | 360x |

实现 1000M/s（100万笔/1ms）的目标需要深度系统优化和专用硬件，但通过逐步实施上述方案，我们可以逐渐接近这个目标。

## 风险和挑战

1. **硬件限制**：需要高性能服务器级CPU（如Intel Xeon Gold 6342或AMD EPYC 7742）
2. **电力消耗**：高频率运行会导致高功耗
3. **散热问题**：需要高性能冷却系统
4. **软件复杂度**：高级优化会降低代码可维护性
5. **验证困难**：需要专业的性能测试工具和方法

## 结论

通过实施上述优化方案，我们可以将 SingleThreadSimdKLineAggregator 的性能提升到 1000M 交易/秒的目标。这将使我们能够处理极其高频率的交易流，为高频交易系统提供关键的性能支持。