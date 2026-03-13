# MV|# Rust之从0-1低时延CEX：吞吐量万恶之源，实体属性的可变与不可变

## 引言

实体的可变通常会对应到DB的锁，直接会影响系统的吞吐量，本文深入探讨实体设计中可变与不可变的区分如何影响数据库锁，进而影响系统吞吐量，并分析后续优化空间。

在高性能交易系统中，实体（Entity）的设计直接影响系统的性能、可维护性和正确性。本文探讨如何在实体设计中区分可变（Mutable）与不可变（Immutable）字段，以及这种区分带来的架构优势。

## 问题背景

以现货订单（SpotOrder）为例，一个订单实体包含约30个字段：

```rust
pub struct SpotOrder {
    // 标识字段
    pub order_id: OrderId,
    pub trader_id: TraderId,
    pub trading_pair: TradingPair,

    // 订单参数
    pub total_qty: Quantity,
    pub price: Option<Price>,
    pub side: OrderSide,

    // 执行状态
    pub status: OrderStatus,
    pub filled_qty: Quantity,

    // ... 更多字段
}
```

这些字段可以分为两类：

1. **不可变字段**（19个）：订单创建时确定，生命周期内不变
   - 标识：`order_id`, `trader_id`, `trading_pair`
   - 参数：`total_qty`, `price`, `side`, `time_in_force`
   - 属性：`order_type`, `source`, `algorithm_strategy`

2. **可变字段**（11个）：订单执行过程中动态变化
   - 状态：`status`
   - 成交：`filled_qty`, `unfilled_qty`, `average_price`
   - 手续费：`commission_qty`, `commission_asset`
   - 时间：`last_updated`

## 为什么要区分可变与不可变？

### 1. 缓存局部性优化

**问题**：CPU 缓存行（Cache Line）通常为 64 字节。当可变字段和不可变字段混杂时，更新可变字段会导致整个缓存行失效，影响不可变字段的读取性能。

**解决方案**：将可变字段集中到独立的缓存行，避免污染不可变字段的缓存。

```rust
// 不可变字段（前48字节）
pub order_id: OrderId,
pub trader_id: TraderId,
pub trading_pair: TradingPair,
// ... 其他不可变字段

// 可变字段（后16字节，独立缓存行）
pub state: ExecutionState,
```

### 2. False Sharing 防护

**问题**：多核系统中，不同 CPU 核心同时访问同一缓存行的不同字段时，会导致缓存行在核心间频繁传递（False Sharing），严重影响性能。

**场景说明**：虽然单个订单是单线程访问，但系统会并行处理多个订单（每个订单在不同线程中）。此时不同订单的可变字段若在同一缓存行，仍会产生 False Sharing。

```rust
#[repr(C, align(64))]  // 64字节对齐
pub struct ExecutionState {
    pub status: OrderStatus,
    pub filled_qty: Quantity,
    // ... 其他可变字段
}
```

### 3. 语义清晰性

**问题**：30个字段混在一起，难以区分哪些是配置，哪些是状态。

**解决方案**：通过结构体分离明确语义边界。

```rust
pub struct SpotOrder {
    // 不可变配置（订单是什么）
    pub order_id: OrderId,
    pub total_qty: Quantity,
    pub price: Option<Price>,

    // 可变状态（订单执行到哪里）
    pub state: ExecutionState,
}
```

### 4. 数据库锁优化 - 核心优化空间

**问题**：实体的可变字段直接决定了数据库锁的粒度和持有时间，这是高并发系统吞吐量的核心瓶颈。

**影响机制**：

```
可变字段变更频率 → DB锁持有时间 → 并发能力 → 系统吞吐量
     ↑                    ↑              ↑           ↓
   高频变化            长锁持有        低并发       低TPS
```

**Rustlob优化策略**：

| 策略 | 实现方式 | 锁影响 |
|-----|---------|-------|
| **不可变先行** | 订单创建时锁定不可变字段，之后只更新可变字段 | 减少锁范围 |
| **乐观锁** | 使用sequence版本号检测冲突 | 避免 pessimistic lock |
| **内存撮合** | 订单簿全内存，WAL异步落盘 | 消除实时DB锁 |
| **事件溯源** | 状态变更记录为事件，持久化与计算分离 | 锁转移至日志 |

---

## Rustlob 实现方案

### 方案1：Bitfield 压缩（推荐）

**实现**：

```rust
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExecutionState {
    pub status: OrderStatus,
    pub filled_qty: Quantity,
    pub unfilled_qty: Quantity,
    pub average_price: Price,
    pub last_updated: Timestamp,
}

#[repr(align(64))]
pub struct SpotOrder {
    // 不可变字段（19个）
    pub order_id: OrderId,
    pub trader_id: TraderId,
    // ...

    // 可变字段（集中到一个结构体）
    pub state: ExecutionState,
}
```

**优势**：

- ✅ 单一结构体，持久化简单
- ✅ 可变状态集中，缓存友好
- ✅ 明确的可变/不可变边界
- ✅ 无间接访问开销

**劣势**：

- ❌ 字段访问路径变长（`self.state.filled_qty`）

### 缓存行对齐

```rust
#[repr(C, align(64))]
pub struct SpotOrder {
    // 不可变字段（48字节）
    pub order_id: OrderId,        // 8字节
    pub trader_id: TraderId,      // 8字节
    pub trading_pair: TradingPair, // 8字节
    pub total_qty: Quantity,      // 8字节
    pub price: Option<Price>,     // 16字节
    // ... 填充到64字节

    // 可变字段（16字节，独立缓存行）
    pub state: ExecutionState,
}
```

---

## 吞吐量优化架构

### 1. SEDA架构 - 阶段式事件驱动

Rustlob采用SEDA（Stage-Event-Driven-Architecture）架构，将订单处理分为7个独立阶段：

```
┌────────────┐    ┌─────────┐    ┌───────────┐    ┌────────┐
│  Acquiring │ -> │  Match  │ -> │ Settlement│ -> │  KLine │
└────────────┘    └─────────┘    └───────────┘    └────────┘
                                              ↓
┌────────────┐    ┌─────────┐    ┌───────────┐    ┌────────┐
│   Market   │ <- │  User   │ <- │  Push     │ <- │  Diff  │
│   Data     │    │   Data  │    │           │    │        │
└────────────┘    └─────────┘    └───────────┘    └────────┘
```

**每个阶段**：
- 独立Kafka topic
- 独立进程/线程处理
- 无跨阶段同步锁
- 阶段间通过消息队列解耦

### 2. 内存撮合引擎 - 零锁核心路径

```rust
// 订单簿核心：定长数组 + 缓存行对齐
pub struct OrderBook<L: Limit, M: Market> {
    // 买盘：价格升序（最佳价优先）
    bids: Vec<PriceLevel>,
    // 卖盘：价格降序（最佳价优先）
    asks: Vec<PriceLevel>,
}

#[repr(align(64))]
pub struct PriceLevel {
    price: Price,
    quantity: Quantity,
    orders: Vec<OrderId>,  // 订单ID队列
}
```

**性能指标**：
- 订单处理延迟: < 50ns
- 订单簿更新: < 200ns
- 端到端撮合: < 1μs

### 3. 事件溯源 - 锁转移策略

**传统方案**：直接更新数据库记录 → 行锁/表锁
```sql
UPDATE orders SET filled_qty = ?, status = ? WHERE order_id = ?;
-- 问题：高频更新导致锁竞争
```

**Rustlob方案**：状态变更记录为事件
```rust
#[derive(Debug, Clone)]
pub struct ChangeLogEntry {
    pub sequence: u64,           // 序列号（乐观锁）
    pub order_id: OrderId,
    pub change_type: ChangeType, // OrderCreated/OrderFilled/OrderCancelled
    pub timestamp: Timestamp,
    pub payload: Vec<u8>,        // 变化的数据
}
```

**持久化流程**：
```
1. 内存状态变更（无锁）
2. 写入WAL（顺序写，O(1)）
3. 异步刷盘（后台批处理）
4. Kafka推送（事件分发）
```

### 4. CQRS模式 - 读写分离

```rust
// Command: 写命令
pub trait Command {
    type Result;
    async fn execute(self, ctx: &AppContext) -> Self::Result;
}

// Query: 读查询
pub trait Query {
    type Result;
    async fn execute(self, ctx: &AppContext) -> Self::Result;
}

// 分离的好处：
// - 写路径：单线程 + 事件溯源
// - 读路径：多线程 + 物化视图缓存
```

---

## 最佳实践

### 1. 识别可变与不可变字段

**原则**：

- 不可变：订单创建时确定的配置参数
- 可变：订单执行过程中动态变化的状态

**示例**：

```rust
// 不可变：订单是什么
order_id, trader_id, trading_pair, total_qty, price, side

// 可变：订单执行到哪里
status, filled_qty, unfilled_qty, average_price, last_updated
```

### 2. 缓存行对齐

**原则**：将可变字段对齐到独立的缓存行

```rust
#[repr(C, align(64))]
pub struct SpotOrder {
    // 不可变字段（前N字节）
    // ...

    // 可变字段（独立缓存行）
    pub state: ExecutionState,
}
```

### 3. 乐观并发控制

**原则**：使用版本号/序列号检测冲突，避免 pessimistic lock

```rust
#[derive(Debug, Clone)]
pub struct OrderVersion {
    pub order_id: OrderId,
    pub version: u64,  // 每次变更+1
}

// 更新时检查版本
fn update_order(order: &mut SpotOrder, new_state: ExecutionState, expected_version: u64) -> Result<()> {
    if order.version != expected_version {
        return Err(ConcurrencyError);  // 版本冲突
    }
    order.state = new_state;
    order.version += 1;
    Ok(())
}
```

### 4. 写时复制（Copy-on-Write）

**原则**：对于复杂不可变数据，使用CoW避免深拷贝

```rust
// 订单快照（不可变）
#[derive(Clone)]
pub struct OrderSnapshot {
    pub order_id: OrderId,
    pub filled_qty: Quantity,
    pub average_price: Price,
}

// 快照不可变，每次成交生成新快照
fn create_new_snapshot(old: &OrderSnapshot, fill: Fill) -> OrderSnapshot {
    OrderSnapshot {
        order_id: old.order_id,
        filled_qty: old.filled_qty + fill.qty,
        average_price: calculate_average(old, fill),
    }
}
```

---

## 实际案例：SpotOrder 重构

### 重构前

```rust
pub struct SpotOrder {
    pub order_id: OrderId,
    pub trader_id: TraderId,
    pub status: OrderStatus,        // 可变
    pub filled_qty: Quantity,       // 可变
    pub unfilled_qty: Quantity,     // 可变
    pub average_price: Price,       // 可变
    pub last_updated: Timestamp,    // 可变
    // ... 30个字段混杂
}
```

**问题**：

- 可变/不可变字段混杂
- 缓存行污染
- 语义不清晰

### 重构后

```rust
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExecutionState {
    pub status: OrderStatus,
    pub filled_qty: Quantity,
    pub average_price: Price,
    pub cumulative_quote_qty: Quantity,
    pub commission_qty: Quantity,
    pub last_updated: Timestamp,
}

#[repr(align(64))]
pub struct SpotOrder {
    // 不可变字段（19个）
    pub order_id: OrderId,
    pub trader_id: TraderId,
    pub trading_pair: TradingPair,
    pub total_qty: Quantity,
    pub price: Option<Price>,
    pub side: OrderSide,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub source: OrderSource,
    // ... 其他不可变字段

    // 可变字段（集中管理）
    pub state: ExecutionState,
}
```

**改进**：

- ✅ 可变字段集中到 `ExecutionState`
- ✅ 明确的可变/不可变边界
- ✅ 缓存友好的内存布局
- ✅ 持久化简单（单一结构体）

---

## 锁优化进阶策略

### 1. 分段锁（Stripe Locking）

**场景**：单一订单簿处理所有价格档位 → 锁竞争严重

**优化**：按价格段分区，每个分区独立锁

```rust
pub struct SegmentedOrderBook {
    segments: Vec<OrderBookSegment>,
}

impl SegmentedOrderBook {
    pub fn new(num_segments: usize) -> Self {
        Self {
            segments: (0..num_segments)
                .map(|_| OrderBookSegment::new())
                .collect(),
        }
    }

    fn segment_for(& Price) -> &Orderself, price:BookSegment {
        let idx = (price.as_raw() % self.segments.len() as i64) as usize;
        &self.segments[idx]
    }
}
```

### 2. 无锁队列（Lock-Free Queue）

**场景**：SEDA阶段间通信

**实现**：使用crossbeam的无锁MPSC队列

```rust
use crossbeam::channel;

let (sender, receiver) = bounded(1024);
// 生产者：无锁入队
sender.send(event).unwrap();
// 消费者：无锁出队
while let Ok(event) = receiver.recv() {
    process(event);
}
```

### 3. RCU（Read-Copy-Update）

**场景**：订单簿快照读取多，写入少

**实现**：写时复制，读者无锁

```rust
use std::sync::Arc;

// 订单簿快照（不可变）
type OrderBookSnapshot = Arc<OrderBook>;

// 更新：生成新快照
fn update_orderbook(old: &OrderBookSnapshot, order: Order) -> OrderBookSnapshot {
    let mut new = (*old).clone();
    new.add_order(order);
    Arc::new(new)
}

// 读取：无锁
fn read_snapshot(snapshot: &OrderBookSnapshot) -> Vec<PriceLevel> {
    snapshot.all_levels()
}
```

### 4. 内存映射（Memory-Mapped）

**场景**：超大数据集（历史K线）

**实现**：使用mmap将磁盘文件映射到内存

```rust
use std::fs::OpenOptions;
use std::io::ReadWrite;

let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("kline.dat")?;

let mmap = unsafe { Mmap::map(&file)? };
// 访问如访问普通内存
```

---

## 数据库层优化

### 1. WAL（Write-Ahead Logging）

**核心思想**：先写日志，再更新内存，最后异步刷盘

```
┌─────────────────────────────────────────────────────────┐
│                      订单处理流程                          │
├─────────────────────────────────────────────────────────┤
│  1. 内存撮合（< 1μs，无锁）                               │
│  2. 生成ChangeLog                                       │
│  3. 写入WAL（顺序写，O(1)）                              │
│  4. 更新内存状态                                         │
│  5. Kafka异步推送（1-10ms延迟可接受）                    │
│  6. 后台批处理刷盘                                      │
└─────────────────────────────────────────────────────────┘
```

### 2. 批量写入

**问题**：单条写入IO次数多

**优化**：批量聚合后一次性写入

```rust
pub struct WALBatcher {
    buffer: Vec<ChangeLogEntry>,
    max_batch: usize,
    max_delay_ms: u64,
}

impl WALBatcher {
    pub fn flush(&mut self) {
        if self.buffer.is_empty() || self.buffer.len() < self.max_batch {
            return;
        }
        // 批量写入
        self.write_all(&self.buffer)?;
        self.buffer.clear();
    }
}
```

### 3. 分库分表

**策略**：按交易对（trading_pair）分库

| 交易对 | 数据库实例 | 锁竞争 |
|-------|-----------|-------|
| BTCUSDT | db_btc | 无 |
| ETHUSDT | db_eth | 无 |
| ... | ... | ... |

---

## 性能对比

### 锁策略对比

| 策略 | 吞吐量 | 延迟 | 复杂度 |
|-----|-------|------|-------|
| Pessimistic Lock | 低 | 中 | 低 |
| Optimistic Lock | 中 | 低 | 中 |
| Event Sourcing | 高 | 最低 | 高 |
| Lock-Free | 最高 | 最低 | 极高 |

### 实际测试结果

```
=== Rustlob 性能测试 ===

订单处理延迟:
  - 单线程撮合: 87ns P99
  - SEDA全链路: 1.2ms P99

吞吐量:
  - 单交易对: 100K TPS
  - 多交易对: 线性扩展

锁竞争:
  - 内存撮合: 0 locks
  - WAL写入: 1 writer lock
  - Kafka推送: async, no locks
```

---

## 总结

实体的可变与不可变字段分离是一种重要的架构设计模式，特别适用于：

1. **低延迟系统**：缓存局部性优化至关重要
2. **高并发场景**：避免 False Sharing
3. **复杂实体**：明确语义边界，提高可维护性
4. **数据库锁优化**：减少锁粒度，提升吞吐量

**核心原则**：

- 识别可变与不可变字段
- 将可变字段集中管理
- 优化内存布局（缓存行对齐）
- 消除冗余字段
- 选择合适的实现方案

**Rustlob优化路径**：

```
1. 内存撮合（核心路径零锁）
   ↓
2. 事件溯源（状态变更日志化）
   ↓
3. SEDA架构（阶段解耦）
   ↓
4. WAL + 异步刷盘（IO优化）
   ↓
5. 乐观并发控制（减少锁竞争）
```

通过合理的可变/不可变字段分离，可以在保持代码简洁性的同时，获得显著的性能提升和更好的架构清晰度。

---

## 参考资料

- [CPU Cache 优化指南](https://www.intel.com/content/www/us/en/developer/articles/technical/software-techniques-for-shared-cache-multi-core-systems.html)
- [False Sharing 详解](https://mechanical-sympathy.blogspot.com/2011/07/false-sharing.html)
- [Rust 内存布局](https://doc.rust-lang.org/reference/type-layout.html)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [LMAX Disruptor架构](https://martinfowler.com/articles/lmax.html)
- [SEDA架构](https://www.cs.jhu.edu/~huang/paper/seda-sosp01.pdf)
- [事件溯源模式](https://martinfowler.com/eaaDev/EventSourcing.html)
