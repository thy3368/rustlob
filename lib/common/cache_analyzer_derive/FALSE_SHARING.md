# 伪共享(False Sharing)检查

## 什么是伪共享

伪共享是多线程并发编程中常见但隐蔽的性能问题。当多个线程频繁访问同一缓存行中的不同变量时，即使这些变量在逻辑上是独立的，也会导致缓存行在CPU核心之间不断失效和同步，严重降低性能。

### 性能影响

- 伪共享可能导致 **10-100 倍**的性能下降
- 在高并发场景下影响尤为显著
- 是低延迟系统中最容易被忽视的性能杀手之一

## 启用伪共享检查

使用 `#[cache(check_false_sharing)]` 属性启用编译时伪共享检查:

```rust
#[derive(CacheAnalyzer)]
#[cache(check_false_sharing)]
pub struct MyStruct {
    counter1: AtomicU64,
    counter2: AtomicU64,
}
```

## 检查规则

伪共享检查会识别以下情况:

1. **多个原子类型在同一缓存行**
   - `AtomicU64`, `AtomicI64`, `AtomicBool` 等
   - 任何以 `Atomic` 开头的类型

2. **多个同步原语在同一缓存行**
   - `Mutex<T>`
   - `RwLock<T>`

3. **跨越缓存行边界的字段**
   - 计算每个字段所在的缓存行(默认64字节)
   - 检测是否有多个可能被不同线程访问的字段共享缓存行

## 错误示例

### ❌ 错误: 原子变量在同一缓存行

```rust
#[derive(CacheAnalyzer)]
#[cache(check_false_sharing)]
pub struct BadLayout {
    counter1: AtomicU64,  // 偏移 0, 缓存行 0
    counter2: AtomicU64,  // 偏移 8, 缓存行 0 ← 伪共享!
}
```

**编译错误:**
```
error: 结构体 BadLayout 存在伪共享(False Sharing)风险:
字段 'counter1' (偏移 0) 和 'counter2' (偏移 8) 在同一缓存行 0 中

伪共享会导致严重的性能下降。建议:
- 使用 #[repr(align(64))] 将字段对齐到缓存行边界
- 在多线程访问的字段间添加填充(padding)隔离
- 将不同线程访问的字段分离到不同的结构体
```

## 正确的解决方案

### ✅ 方案 1: 使用填充字节隔离

```rust
#[derive(CacheAnalyzer)]
#[repr(align(64))]  // 对齐到缓存行
pub struct GoodLayout {
    counter1: AtomicU64,
    _pad1: [u8; 56],    // 填充到 64 字节
    counter2: AtomicU64,
    _pad2: [u8; 56],    // 填充到 64 字节
}
```

### ✅ 方案 2: 分离到不同结构体

```rust
// 线程 1 使用
#[derive(CacheAnalyzer)]
#[repr(align(64))]
pub struct Thread1Data {
    counter: AtomicU64,
    _pad: [u8; 56],
}

// 线程 2 使用
#[derive(CacheAnalyzer)]
#[repr(align(64))]
pub struct Thread2Data {
    counter: AtomicU64,
    _pad: [u8; 56],
}
```

### ✅ 方案 3: 使用封装的缓存行对齐类型

```rust
#[repr(align(64))]
pub struct CacheAligned<T> {
    value: T,
    _pad: [u8; 64 - std::mem::size_of::<T>()],
}

pub struct NoPadding {
    counter1: CacheAligned<AtomicU64>,
    counter2: CacheAligned<AtomicU64>,
}
```

## 实际案例

### 案例 1: 多线程计数器

```rust
// ❌ 错误: 两个计数器会互相干扰
pub struct SharedCounters {
    thread1_counter: AtomicU64,
    thread2_counter: AtomicU64,
}

// ✅ 正确: 使用填充隔离
#[repr(align(64))]
pub struct IsolatedCounters {
    thread1_counter: AtomicU64,
    _pad1: [u8; 56],
    thread2_counter: AtomicU64,
    _pad2: [u8; 56],
}
```

### 案例 2: 生产者-消费者队列

```rust
// ❌ 错误: 读写位置在同一缓存行
pub struct RingBuffer {
    read_pos: AtomicUsize,   // 消费者读
    write_pos: AtomicUsize,  // 生产者写
    buffer: [u8; 1024],
}

// ✅ 正确: 隔离读写位置
#[repr(align(64))]
pub struct FastRingBuffer {
    read_pos: AtomicUsize,
    _pad1: [u8; 56],
    write_pos: AtomicUsize,
    _pad2: [u8; 56],
    buffer: [u8; 1024],
}
```

### 案例 3: 多线程统计信息

```rust
use std::sync::atomic::{AtomicU64, Ordering};

// ❌ 错误: 所有统计字段挤在一起
pub struct ThreadStats {
    requests: AtomicU64,
    errors: AtomicU64,
    bytes: AtomicU64,
}

// ✅ 正确: 每个线程有独立的统计结构
#[repr(align(64))]
pub struct PerThreadStats {
    requests: AtomicU64,
    errors: AtomicU64,
    bytes: AtomicU64,
    _pad: [u8; 40],  // 填充到 64 字节
}

pub struct AllStats {
    thread1: PerThreadStats,
    thread2: PerThreadStats,
    thread3: PerThreadStats,
}
```

## 性能对比

### 测试代码

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

// 伪共享版本
pub struct WithFalseSharing {
    counter1: AtomicU64,
    counter2: AtomicU64,
}

// 无伪共享版本
#[repr(align(64))]
pub struct NoFalseSharing {
    counter1: AtomicU64,
    _pad: [u8; 56],
    counter2: AtomicU64,
}

fn benchmark() {
    const ITERATIONS: u64 = 10_000_000;

    // 测试伪共享版本
    let shared = WithFalseSharing {
        counter1: AtomicU64::new(0),
        counter2: AtomicU64::new(0),
    };

    let start = std::time::Instant::now();
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..ITERATIONS {
                shared.counter1.fetch_add(1, Ordering::Relaxed);
            }
        });
        s.spawn(|| {
            for _ in 0..ITERATIONS {
                shared.counter2.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
    let duration1 = start.elapsed();

    // 测试无伪共享版本
    let isolated = NoFalseSharing {
        counter1: AtomicU64::new(0),
        _pad: [0; 56],
        counter2: AtomicU64::new(0),
    };

    let start = std::time::Instant::now();
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..ITERATIONS {
                isolated.counter1.fetch_add(1, Ordering::Relaxed);
            }
        });
        s.spawn(|| {
            for _ in 0..ITERATIONS {
                isolated.counter2.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
    let duration2 = start.elapsed();

    println!("伪共享版本: {:?}", duration1);
    println!("无伪共享版本: {:?}", duration2);
    println!("性能提升: {:.2}x", duration1.as_secs_f64() / duration2.as_secs_f64());
}
```

**典型结果:**
```
伪共享版本: 2.5s
无伪共享版本: 0.3s
性能提升: 8.33x
```

## 检查限制

### 无法检测的情况

1. **自定义类型**: 无法准确判断自定义类型的线程访问模式
2. **运行时行为**: 编译时无法确定实际的线程访问模式
3. **间接访问**: 通过指针或引用的间接访问

### 误报可能

在以下情况可能产生误报:

- 所有字段都由单个线程访问
- 字段访问是只读的
- 访问频率极低

## 最佳实践

1. **关键路径启用检查**
   ```rust
   #[cache(check_false_sharing, enforce_order)]
   ```

2. **默认使用缓存行对齐**
   ```rust
   #[repr(align(64))]
   ```

3. **分离热点数据**
   - 读多写少的数据单独一组
   - 不同线程的数据分离

4. **使用类型包装**
   ```rust
   type Aligned<T> = CacheAligned<T>;
   ```

5. **性能测试验证**
   - 在实际工作负载下测试
   - 使用 perf 等工具验证缓存命中率

## 运行示例

```bash
cargo run --example false_sharing
```

## 总结

伪共享是低延迟系统中必须重视的问题。通过 `CacheAnalyzer` 的编译时检查,可以在开发阶段就发现并修复潜在的伪共享问题,避免上线后的性能灾难。

记住:
- ⚠️ 伪共享可能导致数十倍性能下降
- ✅ 使用填充字节隔离原子变量
- ✅ 编译时检查优于运行时发现
- ✅ 实际测试验证优化效果
