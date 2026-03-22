# Rust之从0-1低时延CEX：状态：吞吐量万恶之源，实体属性的可变与不可变分析

## 引言

实体的可变 通常会对应到DB的锁直接影响系统的吞吐量，分析的主要目的是探索后续优化空间

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

### 1. 缓存局部性优化 （单线程绑核 不存在）

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

### 2. False Sharing 防护 （单线程绑核 不存在）

**问题**：多核系统中，不同 CPU 核心同时访问同一缓存行的不同字段时，会导致缓存行在核心间频繁传递（False Sharing），严重影响性能。

**场景说明**：虽然单个订单是单线程访问，但系统会并行处理多个订单（每个订单在不同线程中）。此时不同订单的可变字段若在同一缓存行，仍会产生
False Sharing。

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

## 实现方案对比

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


## 总结

实体的可变与不可变字段分离是一种重要的架构设计模式，特别适用于：

1. **低延迟系统**：缓存局部性优化至关重要
2. **高并发场景**：避免 False Sharing
3. **复杂实体**：明确语义边界，提高可维护性



通过合理的可变/不可变字段分离，可以在保持代码简洁性的同时，获得显著的性能提升和更好的架构清晰度。

## 参考资料

- [CPU Cache 优化指南](https://www.intel.com/content/www/us/en/developer/articles/technical/software-techniques-for-shared-cache-multi-core-systems.html)
- [False Sharing 详解](https://mechanical-sympathy.blogspot.com/2011/07/false-sharing.html)
- [Rust 内存布局](https://doc.rust-lang.org/reference/type-layout.html)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
