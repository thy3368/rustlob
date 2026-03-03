# SBE Message Conversion Design

**Date**: 2026-03-03
**Status**: Approved
**Author**: Design Session

## 概述

为 SBE (Simple Binary Encoding) derive macros 添加高层 API，实现 `Trade` 结构体与 `TradeEncoder`/`TradeDecoder` 之间的便捷转换。设计遵循低延迟原则，提供零分配热路径。

## 目标

1. 提供 trait-based 统一 API 用于编码/解码
2. 支持三种缓冲区策略：栈缓冲区（最快）、池化缓冲区（平衡）、Vec 分配（便捷）
3. 保持零分配热路径，满足微秒级延迟要求
4. 自动代码生成，减少样板代码

## 架构设计

### 核心 Trait

在 `lib/common/sbe/src/message.rs` 定义：

```rust
pub trait SbeMessage: Sized {
    /// 将消息编码到提供的缓冲区中
    /// 返回实际写入的字节数
    fn encode_into(&self, buffer: &mut [u8]) -> Result<usize, SbeError>;

    /// 从缓冲区解码消息
    fn decode_from(buffer: &[u8]) -> Result<Self, SbeError>;

    /// 返回消息的最大编码长度（包括 header）
    fn max_encoded_length() -> usize;

    /// 便捷方法：编码到新分配的 Vec（默认实现）
    fn encode_to_bytes(&self) -> Result<Vec<u8>, SbeError> {
        let mut buf = vec![0u8; Self::max_encoded_length()];
        let len = self.encode_into(&mut buf)?;
        buf.truncate(len);
        Ok(buf)
    }

    /// 便捷方法：从 Vec 解码（默认实现）
    fn decode_from_bytes(bytes: &[u8]) -> Result<Self, SbeError> {
        Self::decode_from(bytes)
    }
}
```

### 自动实现

当用户使用 `#[derive(SbeEncode, SbeDecode)]` 时，宏自动生成 `SbeMessage` 实现：

```rust
impl SbeMessage for Trade {
    fn encode_into(&self, buffer: &mut [u8]) -> Result<usize, SbeError> {
        if buffer.len() < Self::max_encoded_length() {
            return Err(SbeError::BufferTooSmall {
                required: Self::max_encoded_length(),
                available: buffer.len(),
            });
        }

        let write_buf = WriteBuf::new(buffer);
        let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

        encoder.trade_id(self.trade_id);
        encoder.symbol(self.symbol);
        encoder.price(self.price);
        encoder.quantity(self.quantity);

        Ok(encoder.encoded_length())
    }

    fn decode_from(buffer: &[u8]) -> Result<Self, SbeError> {
        let read_buf = ReadBuf::new(buffer);
        let decoder = TradeDecoder::default().wrap(
            read_buf,
            0,
            trade_encoder::SBE_BLOCK_LENGTH,
            0,
        );

        Ok(Self {
            trade_id: decoder.trade_id(),
            symbol: decoder.symbol(),
            price: decoder.price(),
            quantity: decoder.quantity(),
        })
    }

    fn max_encoded_length() -> usize {
        trade_encoder::SBE_BLOCK_LENGTH as usize
    }
}
```

## 缓冲区池化方案

### 方案选择：无锁对象池

使用 `crossbeam::queue::ArrayQueue` 实现无锁缓冲区池，在 `lib/common/sbe/src/pool.rs`：

```rust
use crossbeam::queue::ArrayQueue;
use std::sync::Arc;

pub struct BufferPool {
    pool: Arc<ArrayQueue<Vec<u8>>>,
    buffer_size: usize,
}

impl BufferPool {
    pub fn new(capacity: usize, buffer_size: usize) -> Self {
        let pool = Arc::new(ArrayQueue::new(capacity));
        // 预填充池
        for _ in 0..capacity {
            let _ = pool.push(vec![0u8; buffer_size]);
        }
        Self { pool, buffer_size }
    }

    pub fn acquire(&self) -> PooledBuffer {
        let buf = self.pool.pop()
            .unwrap_or_else(|| vec![0u8; self.buffer_size]);
        PooledBuffer {
            buf,
            pool: self.pool.clone(),
        }
    }
}

pub struct PooledBuffer {
    pub buf: Vec<u8>,
    pool: Arc<ArrayQueue<Vec<u8>>>,
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        let mut buf = std::mem::take(&mut self.buf);
        buf.clear();
        let _ = self.pool.push(buf);
    }
}
```

### 性能特性

- **栈缓冲区**: ~10-20ns（零分配，最快）
- **池化缓冲区**: ~40-60ns（无锁队列开销）
- **Vec 分配**: ~150-250ns（每次分配/释放）

### 栈缓冲区优势

1. **分配开销**: 栈指针移动（1 CPU 指令）vs 堆分配器调用（~100-200ns）
2. **内存局部性**: L1 缓存命中（~4 周期）vs 可能的缓存未命中（~200 周期）
3. **释放开销**: 函数返回自动释放（零开销）vs deallocator 调用
4. **无竞争**: 线程独立栈，无锁

## 错误处理

在 `lib/common/sbe/src/error.rs` 定义：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SbeError {
    BufferTooSmall {
        required: usize,
        available: usize,
    },
    InsufficientData {
        required: usize,
        available: usize,
    },
    InvalidTemplateId {
        expected: u16,
        actual: u16,
    },
    VersionMismatch {
        expected: u16,
        actual: u16,
    },
    ValueOutOfRange {
        field: &'static str,
        value: String,
    },
}

impl std::fmt::Display for SbeError { /* ... */ }
impl std::error::Error for SbeError {}
```

## 代码生成实现

### 修改位置

在 `lib/common/sbe_derive/src/codegen.rs` 中：

1. 修改 `generate_encoder()` 函数，在生成 encoder 代码后，检查是否同时派生了 `SbeDecode`
2. 修改 `generate_decoder()` 函数，在生成 decoder 代码后，检查是否同时派生了 `SbeEncode`
3. 当两者都派生时，生成完整的 `SbeMessage` trait 实现

### 生成策略

- 只有当同时派生 `SbeEncode` 和 `SbeDecode` 时才生成 `SbeMessage` 实现
- 使用已生成的 encoder/decoder 类型，避免代码重复
- 自动计算 `max_encoded_length()` 基于 `SBE_BLOCK_LENGTH`

## 性能验证

### Benchmark 设计

在 `lib/common/sbe_derive/benches/buffer_allocation.rs` 创建 Criterion benchmark：

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_stack_buffer(c: &mut Criterion) {
    let trade = Trade { /* ... */ };
    c.bench_function("stack_buffer", |b| {
        b.iter(|| {
            let mut buffer = [0u8; 1024];
            let len = trade.encode_into(&mut buffer).unwrap();
            black_box(&buffer[..len]);
        });
    });
}

fn bench_pooled_buffer(c: &mut Criterion) {
    let trade = Trade { /* ... */ };
    let pool = BufferPool::new(128, 1024);
    c.bench_function("pooled_buffer", |b| {
        b.iter(|| {
            let mut pooled = pool.acquire();
            let len = trade.encode_into(&mut pooled.buf).unwrap();
            black_box(&pooled.buf[..len]);
        });
    });
}

fn bench_vec_allocation(c: &mut Criterion) {
    let trade = Trade { /* ... */ };
    c.bench_function("vec_allocation", |b| {
        b.iter(|| {
            let bytes = trade.encode_to_bytes().unwrap();
            black_box(&bytes);
        });
    });
}

criterion_group!(benches, bench_stack_buffer, bench_pooled_buffer, bench_vec_allocation);
criterion_main!(benches);
```

### 性能目标

- 栈缓冲区：< 20ns
- 池化缓冲区：< 60ns
- Vec 分配：< 250ns

## 使用示例

### 场景 1: 热路径 - 栈缓冲区

```rust
fn hot_path_encoding() {
    let trade = Trade {
        trade_id: 12345,
        symbol: b'A',
        price: 100.50,
        quantity: 1000,
    };

    // 栈上分配 - 零分配
    let mut buffer = [0u8; Trade::max_encoded_length()];
    let len = trade.encode_into(&mut buffer).unwrap();

    send_to_exchange(&buffer[..len]);
}
```

### 场景 2: 中等频率路径 - 池化缓冲区

```rust
fn medium_frequency_path(pool: &BufferPool) {
    let trade = Trade { /* ... */ };

    // 从池获取缓冲区
    let mut pooled = pool.acquire();
    let len = trade.encode_into(&mut pooled.buf).unwrap();

    send_to_exchange(&pooled.buf[..len]);
    // 自动归还池
}
```

### 场景 3: 非关键路径 - Vec 分配

```rust
fn convenience_path() {
    let trade = Trade { /* ... */ };

    // 一行代码完成编码
    let bytes = trade.encode_to_bytes().unwrap();

    save_to_database(&bytes);
}
```

### 解码示例

```rust
fn decode_example(data: &[u8]) -> Result<Trade, SbeError> {
    Trade::decode_from(data)
}
```

## 文件变更清单

1. **新增文件**:
   - `lib/common/sbe/src/message.rs` - `SbeMessage` trait 定义
   - `lib/common/sbe/src/error.rs` - `SbeError` 错误类型
   - `lib/common/sbe/src/pool.rs` - `BufferPool` 实现
   - `lib/common/sbe_derive/benches/buffer_allocation.rs` - 性能 benchmark

2. **修改文件**:
   - `lib/common/sbe/src/lib.rs` - 导出新模块
   - `lib/common/sbe_derive/src/codegen.rs` - 生成 `SbeMessage` 实现
   - `lib/common/sbe_derive/examples/trade_codec.rs` - 更新示例使用新 API
   - `lib/common/sbe_derive/Cargo.toml` - 添加 benchmark 配置

## 实现计划

1. 实现 `SbeError` 错误类型
2. 实现 `SbeMessage` trait 定义
3. 实现 `BufferPool` 缓冲区池
4. 修改 codegen 生成 `SbeMessage` 实现
5. 创建性能 benchmark
6. 更新示例代码
7. 运行 benchmark 验证性能目标

## 符合性检查

### Clean Architecture
- ✅ 领域层独立：`SbeMessage` trait 不依赖具体实现
- ✅ 依赖倒置：通过 trait 抽象编码/解码行为
- ✅ 可测试性：可 mock `SbeMessage` 实现

### 低延迟要求
- ✅ 零分配热路径：栈缓冲区方案
- ✅ 缓存友好：连续内存访问
- ✅ 无锁设计：池化方案使用 `ArrayQueue`
- ✅ 内联优化：关键方法标记 `#[inline]`

## 总结

本设计提供了一个灵活、高性能的 SBE 消息转换 API，满足以下需求：

1. **便捷性**: Trait-based API，一行代码完成编码/解码
2. **性能**: 提供零分配栈缓冲区路径，满足微秒级延迟要求
3. **灵活性**: 三种缓冲区策略适应不同场景
4. **自动化**: Derive macro 自动生成实现，减少样板代码
5. **可验证**: Benchmark 验证性能目标
