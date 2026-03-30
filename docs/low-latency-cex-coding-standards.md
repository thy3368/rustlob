# 低时延CEX编码规范: SIMD友好的POD（Plain Old Data）类型设计，涵盖Command、Query、Event、Entity四种核心数据模型

## 概述

本文档定义了低时延中心化交易所（CEX）系统的编码规范，重点关注SOA（Structure of Arrays）/SIMD友好的POD（Plain Old Data）类型设计，涵盖Command、Query、Event、Entity四种核心数据模型。

## 设计理念

### CEX系统的本质：微批向量计算流式数据系统

中心化交易所（CEX）系统的核心目标是**高吞吐**和**低时延**，其本质是一个**微批向量计算流式数据系统**。

#### 1. 流式数据系统特征

**数据流动模型**:
```
用户请求 → Command流 → 验证 → Event流 → 状态更新 → Query响应
         ↓                    ↓
      撮合引擎            事件溯源
         ↓                    ↓
      Trade流              持久化
```

**关键特性**:
- **无界数据流**: 订单、成交、行情数据持续不断流入
- **时序性**: 事件按时间戳严格排序
- **低延迟要求**: P99延迟 < 1ms（关键路径）
- **高吞吐要求**: 百万级TPS（订单处理）

#### 2. 微批处理（Micro-Batching）

**为什么需要微批**:
- 单条处理：延迟低但吞吐受限
- 大批处理：吞吐高但延迟不可控
- **微批处理**：平衡延迟和吞吐的最优解

**微批策略**:
```rust
// 微批大小：8-64条（适配SIMD寄存器宽度）
const MICRO_BATCH_SIZE: usize = 32;

// 微批超时：100-500μs（避免延迟累积）
const MICRO_BATCH_TIMEOUT_US: u64 = 200;

// 动态调整：根据负载自适应
fn adaptive_batch_size(queue_depth: usize) -> usize {
    match queue_depth {
        0..=10 => 8,      // 低负载：小批量，低延迟
        11..=100 => 32,   // 中负载：平衡
        _ => 64,          // 高负载：大批量，高吞吐
    }
}
```

**微批处理流程**:
```
1. 累积阶段（Accumulation）
   - 从队列收集N条命令
   - 或等待超时（whichever comes first）

2. 批量验证（Batch Validation）
   - SIMD并行验证
   - 过滤无效命令

3. 批量执行（Batch Execution）
   - 向量化计算
   - 批量状态更新

4. 批量输出（Batch Output）
   - 生成事件流
   - 批量持久化
```

#### 3. 向量计算（Vectorized Computing）

**SIMD向量化优势**:
- **并行度**: 一次处理4-8个数据（AVX2/AVX-512）
- **吞吐提升**: 2-8倍性能提升
- **能效比**: 更低的CPU周期/操作

**向量化场景**:

**场景1: 批量订单验证**
```rust
// 标量版本（逐个处理）
fn validate_orders_scalar(orders: &[OrderPod]) -> Vec<bool> {
    orders.iter().map(|o| o.price > 0 && o.quantity > 0).collect()
}

// 向量版本（SIMD并行）
#[target_feature(enable = "avx2")]
unsafe fn validate_orders_simd(orders: &OrderSoa) -> Vec<bool> {
    // 一次比较8个价格
    let prices = _mm256_loadu_si256(orders.prices.as_ptr() as *const __m256i);
    let zeros = _mm256_setzero_si256();
    let price_valid = _mm256_cmpgt_epi64(prices, zeros);

    // 一次比较8个数量
    let quantities = _mm256_loadu_si256(orders.quantities.as_ptr() as *const __m256i);
    let qty_valid = _mm256_cmpgt_epi64(quantities, zeros);

    // 合并结果
    let valid = _mm256_and_si256(price_valid, qty_valid);
    // ... 提取结果
}
```

**场景2: 批量余额检查**
```rust
// SoA布局 + SIMD
pub struct BalanceSoa {
    pub available: Vec<i64>,  // 连续内存，SIMD友好
    pub frozen: Vec<i64>,
}

// 批量检查余额是否充足（向量化）
pub fn batch_check_sufficient(&self, required: &[i64]) -> Vec<bool> {
    // SIMD并行比较：available[i] >= required[i]
    // 一次处理8个余额
}
```

**场景3: 批量价格计算**
```rust
// 批量计算成交金额：quantity * price
pub fn batch_calculate_quote_qty(
    quantities: &[i64],
    prices: &[i64],
) -> Vec<i64> {
    // SIMD向量乘法：一次计算4-8个
    quantities.iter()
        .zip(prices.iter())
        .map(|(&q, &p)| q * p)
        .collect()
}
```

#### 4. 流式处理架构

**数据流水线**:
```
┌─────────────┐
│ Input Queue │ ← 用户请求（Command）
└──────┬──────┘
       │ 微批累积（8-64条）
       ↓
┌─────────────┐
│  Validator  │ ← SIMD批量验证
└──────┬──────┘
       │ 有效命令
       ↓
┌─────────────┐
│   Matcher   │ ← 向量化撮合
└──────┬──────┘
       │ 成交事件
       ↓
┌─────────────┐
│State Update │ ← 批量状态更新
└──────┬──────┘
       │ 事件流
       ↓
┌─────────────┐
│Event Stream │ ← 持久化 + 分发
└─────────────┘
```

**流式处理特性**:
- **背压控制（Backpressure）**: 下游慢时，上游限流
- **流量整形（Traffic Shaping）**: 平滑突发流量
- **优先级队列**: 关键命令优先处理
- **并行流水线**: 多个流水线并行处理

#### 5. 高吞吐设计策略

**策略1: 无锁数据结构**
```rust
// 使用无锁队列（SPSC/MPSC）
use crossbeam::queue::ArrayQueue;

let command_queue: ArrayQueue<CommandPod> = ArrayQueue::new(10000);

// 生产者：零拷贝入队
command_queue.push(command);

// 消费者：批量出队
let mut batch = Vec::with_capacity(32);
while batch.len() < 32 {
    if let Some(cmd) = command_queue.pop() {
        batch.push(cmd);
    } else {
        break;
    }
}
```

**策略2: 零拷贝传输**
```rust
// POD类型支持零拷贝
let command = CommandPod::new(...);

// 直接转换为字节流（无序列化开销）
let bytes = command.as_bytes();

// 网络发送（零拷贝）
socket.send(bytes);

// 接收端：零拷贝解析
let received = unsafe { CommandPod::from_bytes(bytes) };
```

**策略3: 批量持久化**
```rust
// 批量写入事件日志
pub fn batch_append_events(events: &[EventPod]) -> Result<()> {
    // 转换为连续字节流
    let bytes = EventPod::as_bytes_slice(events);

    // 一次性写入（减少系统调用）
    file.write_all(bytes)?;

    // 批量fsync（可选）
    file.sync_data()?;

    Ok(())
}
```

#### 6. 低时延设计策略

**策略1: CPU亲和性绑定**
```rust
// 关键线程绑定到独立CPU核心
use core_affinity;

// 撮合引擎线程 → CPU 0
core_affinity::set_for_current(CoreId { id: 0 });

// 事件处理线程 → CPU 1
core_affinity::set_for_current(CoreId { id: 1 });
```

**策略2: 内存预分配**
```rust
// 预分配所有数据结构，避免运行时分配
pub struct MatchingEngine {
    orders: OrderSoa,           // 预分配100万订单
    events: Vec<EventPod>,      // 预分配事件缓冲
    batch_buffer: Vec<CommandPod>, // 预分配批处理缓冲
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            orders: OrderSoa::with_capacity(1_000_000),
            events: Vec::with_capacity(100_000),
            batch_buffer: Vec::with_capacity(64),
        }
    }
}
```

**策略3: 热路径优化**
```rust
// 关键路径：内联 + 无分支
#[inline(always)]
pub fn fast_path_match(
    order: &OrderPod,
    best_price: i64,
) -> bool {
    // 使用位运算避免分支
    let is_buy = order.side == 1;
    let price_match = (is_buy & (order.price >= best_price))
                    | (!is_buy & (order.price <= best_price));
    price_match != 0
}
```

#### 7. 系统吞吐量计算

**理论吞吐量**:
```
吞吐量 = (批量大小 × CPU核心数) / 处理延迟

示例：
- 批量大小：32条
- CPU核心数：8核
- 处理延迟：100μs

吞吐量 = (32 × 8) / 0.0001s = 2,560,000 TPS
```

**实际优化目标**:
- **订单处理**: 100万 TPS
- **撮合吞吐**: 50万 TPS
- **事件生成**: 200万 TPS
- **P99延迟**: < 1ms

#### 8. 设计权衡

| 维度 | 单条处理 | 微批处理 | 大批处理 |
|------|---------|---------|---------|
| **延迟** | 极低（<100μs） | 低（<500μs） | 高（>1ms） |
| **吞吐** | 低（10万TPS） | 高（100万TPS） | 极高（500万TPS） |
| **复杂度** | 简单 | 中等 | 复杂 |
| **适用场景** | 低频交易 | **CEX主流** | 批量结算 |

**CEX选择微批处理的原因**:
- ✅ 延迟可控（P99 < 1ms）
- ✅ 吞吐充足（百万级TPS）
- ✅ SIMD优化友好
- ✅ 实现复杂度适中

## 核心设计原则 






### 1. POD类型要求

**定义**: Plain Old Data类型，只包含基础数据类型，无复杂逻辑。

**强制要求**:
- ✅ 只使用基础类型：`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`
- ✅ 固定大小：`#[repr(C)]`或`#[repr(C, align(N))]`
- ✅ 可拷贝：实现`Copy` trait
- ✅ 无堆分配：不包含`String`, `Vec`, `Box`等
- ✅ 无Option：使用特殊值表示（如`u64::MAX`表示None）
- ✅ 无enum：使用`u8`编码
- ✅ 缓存行对齐：关键类型64/128字节对齐

**禁止使用**:
- ❌ `Option<T>`（使用特殊值代替）
- ❌ `enum`（使用`u8`编码代替）
- ❌ `String`（使用固定大小数组`[u8; N]`）
- ❌ `Vec<T>`（使用固定大小数组或SoA结构）
- ❌ `Box<T>`, `Rc<T>`, `Arc<T>`
- ❌ 动态分配的任何类型

### 2. SOA（Structure of Arrays）布局

**定义**: 将多个对象的相同字段连续存储，而非对象本身连续存储。

**优势**:
- 缓存友好：相同字段连续访问
- SIMD优化：可向量化处理
- 内存效率：减少padding浪费

**对比**:

```rust
// ❌ AoS (Array of Structures) - 传统布局
struct Order {
    order_id: u64,
    price: u64,
    quantity: u64,
}
let orders: Vec<Order> = vec![...];

// ✅ SoA (Structure of Arrays) - 推荐布局
struct OrderSoa {
    order_ids: Vec<u64>,    // 连续内存，SIMD友好
    prices: Vec<u64>,       // 连续内存，SIMD友好
    quantities: Vec<u64>,   // 连续内存，SIMD友好
}
```

### 3. SIMD友好设计

**要求**:
- 数据对齐到SIMD寄存器宽度（16/32/64字节）
- 使用SoA布局
- 避免条件分支
- 使用位运算代替分支

**SIMD寄存器宽度**:
- SSE: 128位（16字节）- 2个u64
- AVX2: 256位（32字节）- 4个u64
- AVX-512: 512位（64字节）- 8个u64
- ARM NEON: 128位（16字节）- 2个u64

## 四种核心数据模型

### 1. Command（命令）

**定义**: 用户发起的操作请求，表达意图。

**特征**:
- 不可变
- 包含操作参数
- 需要验证
- 可能失败

**POD设计规范**:

```rust
/// 下单命令（POD版本）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(128))]
pub struct PlaceOrderCommandPod {
    // ===== 第一缓存行（64字节）=====
    /// 命令ID（全局唯一）
    pub command_id: u64,
    /// 账户ID
    pub account_id: u64,
    /// 交易对ID
    pub trading_pair: u64,
    /// 订单类型（1=Limit, 2=Market）
    pub order_type: u8,
    /// 买卖方向（1=Buy, 2=Sell）
    pub side: u8,
    /// 有效期类型（1=GTC, 2=IOC, 3=FOK）
    pub time_in_force: u8,
    /// 保留字段
    pub _padding1: [u8; 5],
    /// 价格（i64，8位小数）
    pub price: i64,
    /// 数量（i64，8位小数）
    pub quantity: i64,
    /// 时间戳（纳秒）
    pub timestamp: u64,

    // ===== 第二缓存行（64字节）=====
    /// 客户端订单ID（可选，u64::MAX表示None）
    pub client_order_id: u64,
    /// 止损价（可选）
    pub stop_price: i64,
    /// 过期时间（可选）
    pub expire_time: u64,
    /// 保留字段（未来扩展）
    pub _reserved: [u64; 4],
}

impl PlaceOrderCommandPod {
    /// 验证命令有效性
    #[inline]
    pub const fn validate(&self) -> bool {
        // 价格和数量必须为正
        self.price > 0 && self.quantity > 0
            // 订单类型有效
            && self.order_type >= 1 && self.order_type <= 2
            // 方向有效
            && self.side >= 1 && self.side <= 2
    }

    /// 零拷贝：从字节数组创建
    #[inline]
    pub unsafe fn from_bytes(bytes: &[u8; 128]) -> &Self {
        &*(bytes.as_ptr() as *const Self)
    }

    /// 零拷贝：转换为字节数组
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 128] {
        unsafe { &*(self as *const Self as *const [u8; 128]) }
    }
}

// 静态断言
const _: () = assert!(std::mem::size_of::<PlaceOrderCommandPod>() == 128);
const _: () = assert!(std::mem::align_of::<PlaceOrderCommandPod>() == 128);
```

**SoA版本**:

```rust
/// 批量命令（SoA布局）
#[derive(Debug, Clone)]
pub struct PlaceOrderCommandSoa {
    pub command_ids: Vec<u64>,
    pub account_ids: Vec<u64>,
    pub trading_pairs: Vec<u64>,
    pub order_types: Vec<u8>,
    pub sides: Vec<u8>,
    pub prices: Vec<i64>,
    pub quantities: Vec<i64>,
    pub timestamps: Vec<u64>,
}

impl PlaceOrderCommandSoa {
    /// 批量验证（SIMD优化潜力）
    pub fn batch_validate(&self) -> Vec<bool> {
        (0..self.command_ids.len())
            .map(|i| {
                self.prices[i] > 0
                    && self.quantities[i] > 0
                    && self.order_types[i] >= 1
                    && self.order_types[i] <= 2
            })
            .collect()
    }
}
```

### 2. Query（查询）

**定义**: 只读查询请求，不改变系统状态。

**特征**:
- 只读操作
- 无副作用
- 可缓存
- 幂等性

**POD设计规范**:

```rust
/// 查询订单命令（POD版本）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct QueryOrderPod {
    // ===== 单缓存行（64字节）=====
    /// 查询ID
    pub query_id: u64,
    /// 订单ID（可选，u64::MAX表示查询所有）
    pub order_id: u64,
    /// 账户ID（可选，u64::MAX表示不过滤）
    pub account_id: u64,
    /// 交易对ID（可选，u64::MAX表示不过滤）
    pub trading_pair: u64,
    /// 状态过滤（0=全部，1=Pending，2=Filled等）
    pub status_filter: u8,
    /// 保留字段
    pub _padding: [u8; 7],
    /// 开始时间（纳秒）
    pub start_time: u64,
    /// 结束时间（纳秒）
    pub end_time: u64,
    /// 最大返回数量
    pub limit: u32,
    /// 偏移量
    pub offset: u32,
}

impl QueryOrderPod {
    /// 检查是否查询单个订单
    #[inline]
    pub const fn is_single_order(&self) -> bool {
        self.order_id != u64::MAX
    }

    /// 检查是否有时间范围过滤
    #[inline]
    pub const fn has_time_filter(&self) -> bool {
        self.start_time > 0 || self.end_time > 0
    }
}

const _: () = assert!(std::mem::size_of::<QueryOrderPod>() == 64);
const _: () = assert!(std::mem::align_of::<QueryOrderPod>() == 64);
```

**查询结果（SoA布局）**:

```rust
/// 查询结果（SoA布局）
#[derive(Debug, Clone)]
pub struct QueryOrderResultSoa {
    pub order_ids: Vec<u64>,
    pub account_ids: Vec<u64>,
    pub trading_pairs: Vec<u64>,
    pub prices: Vec<i64>,
    pub quantities: Vec<i64>,
    pub filled_quantities: Vec<i64>,
    pub statuses: Vec<u8>,
    pub timestamps: Vec<u64>,
}

impl QueryOrderResultSoa {
    /// 批量过滤（SIMD优化）
    pub fn filter_by_status(&self, status: u8) -> Vec<usize> {
        self.statuses
            .iter()
            .enumerate()
            .filter_map(|(idx, &s)| if s == status { Some(idx) } else { None })
            .collect()
    }
}
```

### 3. Event（事件）

**定义**: 系统中已发生的事实，不可变。

**特征**:
- 不可变
- 已发生的事实
- 包含完整上下文
- 用于事件溯源

**POD设计规范**:

```rust
/// 订单创建事件（POD版本）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(128))]
pub struct OrderCreatedEventPod {
    // ===== 第一缓存行（64字节）=====
    /// 事件ID（全局唯一序列号）
    pub event_id: u64,
    /// 订单ID
    pub order_id: u64,
    /// 账户ID
    pub account_id: u64,
    /// 交易对ID
    pub trading_pair: u64,
    /// 订单类型
    pub order_type: u8,
    /// 买卖方向
    pub side: u8,
    /// 有效期类型
    pub time_in_force: u8,
    /// 保留字段
    pub _padding1: [u8; 5],
    /// 价格
    pub price: i64,
    /// 数量
    pub quantity: i64,

    // ===== 第二缓存行（64字节）=====
    /// 事件时间戳（纳秒）
    pub timestamp: u64,
    /// 命令ID（关联的命令）
    pub command_id: u64,
    /// 冻结金额
    pub frozen_amount: i64,
    /// 冻结资产ID
    pub frozen_asset_id: u64,
    /// 保留字段
    pub _reserved: [u64; 3],
}

impl OrderCreatedEventPod {
    /// 创建事件
    #[inline]
    pub const fn new(
        event_id: u64,
        order_id: u64,
        account_id: u64,
        trading_pair: u64,
        order_type: u8,
        side: u8,
        time_in_force: u8,
        price: i64,
        quantity: i64,
        timestamp: u64,
        command_id: u64,
        frozen_amount: i64,
        frozen_asset_id: u64,
    ) -> Self {
        Self {
            event_id,
            order_id,
            account_id,
            trading_pair,
            order_type,
            side,
            time_in_force,
            _padding1: [0; 5],
            price,
            quantity,
            timestamp,
            command_id,
            frozen_amount,
            frozen_asset_id,
            _reserved: [0; 3],
        }
    }

    /// 零拷贝序列化
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 128] {
        unsafe { &*(self as *const Self as *const [u8; 128]) }
    }

    /// 零拷贝反序列化
    #[inline]
    pub unsafe fn from_bytes(bytes: &[u8; 128]) -> &Self {
        &*(bytes.as_ptr() as *const Self)
    }
}

const _: () = assert!(std::mem::size_of::<OrderCreatedEventPod>() == 128);
const _: () = assert!(std::mem::align_of::<OrderCreatedEventPod>() == 128);
```

**事件流（SoA布局）**:

```rust
/// 事件流（SoA布局）
#[derive(Debug, Clone)]
pub struct OrderEventStreamSoa {
    pub event_ids: Vec<u64>,
    pub order_ids: Vec<u64>,
    pub account_ids: Vec<u64>,
    pub event_types: Vec<u8>,  // 1=Created, 2=Filled, 3=Cancelled
    pub timestamps: Vec<u64>,
}

impl OrderEventStreamSoa {
    /// 按时间范围过滤
    pub fn filter_by_time_range(&self, start: u64, end: u64) -> Vec<usize> {
        self.timestamps
            .iter()
            .enumerate()
            .filter_map(|(idx, &ts)| {
                if ts >= start && ts <= end {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }

    /// 按账户过滤（SIMD优化潜力）
    pub fn filter_by_account(&self, account_id: u64) -> Vec<usize> {
        self.account_ids
            .iter()
            .enumerate()
            .filter_map(|(idx, &id)| if id == account_id { Some(idx) } else { None })
            .collect()
    }
}
```

### 4. Entity（实体）

**定义**: 具有唯一标识的领域对象，可变状态。

**特征**:
- 唯一标识（ID）
- 可变状态
- 生命周期
- 业务逻辑

**POD设计规范**:

```rust
/// 订单实体（POD版本）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(128))]
pub struct OrderEntityPod {
    // ===== 第一缓存行（64字节）- 不可变字段 =====
    /// 订单ID（唯一标识）
    pub order_id: u64,
    /// 账户ID
    pub account_id: u64,
    /// 交易对ID
    pub trading_pair: u64,
    /// 订单类型
    pub order_type: u8,
    /// 买卖方向
    pub side: u8,
    /// 有效期类型
    pub time_in_force: u8,
    /// 保留字段
    pub _padding1: [u8; 5],
    /// 价格
    pub price: i64,
    /// 总数量
    pub total_quantity: i64,
    /// 创建时间戳
    pub created_at: u64,

    // ===== 第二缓存行（64字节）- 可变字段 =====
    /// 订单状态（1=Pending, 2=PartiallyFilled, 3=Filled, 4=Cancelled）
    pub status: u8,
    /// 保留字段
    pub _padding2: [u8; 7],
    /// 已成交数量
    pub filled_quantity: i64,
    /// 平均成交价
    pub average_price: i64,
    /// 累计成交金额
    pub cumulative_quote_qty: i64,
    /// 手续费
    pub commission: i64,
    /// 最后更新时间
    pub updated_at: u64,
    /// 版本号（乐观锁）
    pub version: u64,
}

impl OrderEntityPod {
    /// 创建新订单
    #[inline]
    pub const fn new(
        order_id: u64,
        account_id: u64,
        trading_pair: u64,
        order_type: u8,
        side: u8,
        time_in_force: u8,
        price: i64,
        quantity: i64,
        timestamp: u64,
    ) -> Self {
        Self {
            order_id,
            account_id,
            trading_pair,
            order_type,
            side,
            time_in_force,
            _padding1: [0; 5],
            price,
            total_quantity: quantity,
            created_at: timestamp,
            status: 1, // Pending
            _padding2: [0; 7],
            filled_quantity: 0,
            average_price: 0,
            cumulative_quote_qty: 0,
            commission: 0,
            updated_at: timestamp,
            version: 0,
        }
    }

    /// 检查是否完全成交
    #[inline]
    pub const fn is_filled(&self) -> bool {
        self.filled_quantity == self.total_quantity
    }

    /// 获取未成交数量
    #[inline]
    pub const fn unfilled_quantity(&self) -> i64 {
        self.total_quantity - self.filled_quantity
    }

    /// 更新成交信息（返回新版本）
    #[inline]
    pub const fn with_fill(
        mut self,
        fill_quantity: i64,
        fill_price: i64,
        timestamp: u64,
    ) -> Self {
        self.filled_quantity += fill_quantity;
        self.cumulative_quote_qty += fill_quantity * fill_price;
        if self.filled_quantity > 0 {
            self.average_price = self.cumulative_quote_qty / self.filled_quantity;
        }
        self.status = if self.is_filled() { 3 } else { 2 }; // Filled or PartiallyFilled
        self.updated_at = timestamp;
        self.version += 1;
        self
    }
}

const _: () = assert!(std::mem::size_of::<OrderEntityPod>() == 128);
const _: () = assert!(std::mem::align_of::<OrderEntityPod>() == 128);
```

**实体集合（SoA布局）**:

```rust
/// 订单实体集合（SoA布局）
#[derive(Debug, Clone)]
pub struct OrderEntitySoa {
    // 不可变字段
    pub order_ids: Vec<u64>,
    pub account_ids: Vec<u64>,
    pub trading_pairs: Vec<u64>,
    pub order_types: Vec<u8>,
    pub sides: Vec<u8>,
    pub prices: Vec<i64>,
    pub total_quantities: Vec<i64>,
    pub created_ats: Vec<u64>,

    // 可变字段
    pub statuses: Vec<u8>,
    pub filled_quantities: Vec<i64>,
    pub average_prices: Vec<i64>,
    pub cumulative_quote_qtys: Vec<i64>,
    pub commissions: Vec<i64>,
    pub updated_ats: Vec<u64>,
    pub versions: Vec<u64>,
}

impl OrderEntitySoa {
    /// 批量检查是否完全成交（SIMD优化）
    pub fn batch_is_filled(&self, indices: &[usize]) -> Vec<bool> {
        indices
            .iter()
            .map(|&idx| self.filled_quantities[idx] == self.total_quantities[idx])
            .collect()
    }

    /// 批量更新成交信息
    pub fn batch_update_fill(
        &mut self,
        indices: &[usize],
        fill_quantities: &[i64],
        fill_prices: &[i64],
        timestamp: u64,
    ) {
        for ((&idx, &fill_qty), &fill_price) in
            indices.iter().zip(fill_quantities.iter()).zip(fill_prices.iter())
        {
            self.filled_quantities[idx] += fill_qty;
            self.cumulative_quote_qtys[idx] += fill_qty * fill_price;

            if self.filled_quantities[idx] > 0 {
                self.average_prices[idx] =
                    self.cumulative_quote_qtys[idx] / self.filled_quantities[idx];
            }

            self.statuses[idx] = if self.filled_quantities[idx] == self.total_quantities[idx] {
                3 // Filled
            } else {
                2 // PartiallyFilled
            };

            self.updated_ats[idx] = timestamp;
            self.versions[idx] += 1;
        }
    }
}
```

## 编码规范清单

### POD类型检查清单

- [ ] 使用`#[repr(C)]`或`#[repr(C, align(N))]`
- [ ] 实现`Copy` trait
- [ ] 只包含基础类型（u8/u16/u32/u64/i8/i16/i32/i64/f32/f64）
- [ ] 无Option（使用特殊值如u64::MAX）
- [ ] 无enum（使用u8编码）
- [ ] 无String（使用[u8; N]）
- [ ] 无Vec（使用固定数组或SoA）
- [ ] 缓存行对齐（64或128字节）
- [ ] 提供`as_bytes()`和`from_bytes()`方法
- [ ] 添加静态断言验证大小和对齐

### SoA布局检查清单

- [ ] 相同类型字段使用Vec连续存储
- [ ] 提供批量操作方法
- [ ] 考虑SIMD优化潜力
- [ ] 提供过滤和查询方法
- [ ] 避免在热路径中分配内存

### SIMD优化检查清单

- [ ] 数据对齐到SIMD寄存器宽度
- [ ] 使用SoA布局
- [ ] 避免条件分支（使用位运算）
- [ ] 批量处理（一次处理多个元素）
- [ ] 使用`#[target_feature]`标记SIMD函数

### 性能检查清单

- [ ] 零堆分配（热路径）
- [ ] 缓存行对齐
- [ ] 避免false sharing
- [ ] 预分配容量
- [ ] 使用内联函数（`#[inline]`）
- [ ] 使用const函数（`const fn`）
- [ ] 避免不必要的拷贝

## 示例：完整的订单系统

### Command

```rust
// 下单命令（POD）
#[repr(C, align(128))]
pub struct PlaceOrderCommandPod { /* ... */ }

// 批量命令（SoA）
pub struct PlaceOrderCommandSoa { /* ... */ }
```

### Query

```rust
// 查询命令（POD）
#[repr(C, align(64))]
pub struct QueryOrderPod { /* ... */ }

// 查询结果（SoA）
pub struct QueryOrderResultSoa { /* ... */ }
```

### Event

```rust
// 订单创建事件（POD）
#[repr(C, align(128))]
pub struct OrderCreatedEventPod { /* ... */ }

// 事件流（SoA）
pub struct OrderEventStreamSoa { /* ... */ }
```

### Entity

```rust
// 订单实体（POD）
#[repr(C, align(128))]
pub struct OrderEntityPod { /* ... */ }

// 实体集合（SoA）
pub struct OrderEntitySoa { /* ... */ }
```

## 性能基准

### 内存布局对比

| 类型 | 大小 | 对齐 | 缓存行 | 零拷贝 |
|------|------|------|--------|--------|
| 传统结构体 | 不定 | 8字节 | 跨行 | ❌ |
| POD类型 | 固定 | 64/128字节 | 对齐 | ✅ |
| SoA布局 | 动态 | 字段对齐 | 友好 | 部分 |

### 性能提升

- **缓存命中率**: 提升30-50%
- **SIMD加速**: 2-8倍（取决于操作）
- **内存占用**: 减少10-20%（消除padding）
- **序列化开销**: 接近零（零拷贝）

## 工具和验证

### 静态断言

```rust
// 验证大小
const _: () = assert!(std::mem::size_of::<OrderEntityPod>() == 128);

// 验证对齐
const _: () = assert!(std::mem::align_of::<OrderEntityPod>() == 128);

// 验证是Copy
const _: () = {
    fn assert_copy<T: Copy>() {}
    assert_copy::<OrderEntityPod>();
};
```

### 性能测试

```rust
#[bench]
fn bench_pod_creation(b: &mut Bencher) {
    b.iter(|| {
        OrderEntityPod::new(1, 100, 1, 1, 1, 1, 50000, 1000, 123456789)
    });
}

#[bench]
fn bench_soa_batch_filter(b: &mut Bencher) {
    let soa = create_test_soa(10000);
    b.iter(|| {
        soa.filter_by_account(100)
    });
}
```

## 最佳实践

### 1. 优先使用POD类型

在以下场景优先使用POD类型：
- 网络传输
- 持久化存储
- 共享内存
- 事件溯源
- 高频操作

### 2. 合理使用SoA布局

在以下场景使用SoA布局：
- 批量处理
- SIMD优化
- 列式查询
- 数据分析

### 3. 混合使用

```rust
// POD用于单个对象
let order = OrderEntityPod::new(...);

// SoA用于批量处理
let mut orders_soa = OrderEntitySoa::with_capacity(1000);
orders_soa.push(&order);

// 批量操作
orders_soa.batch_update_fill(&indices, &quantities, &prices, now);
```

### 4. 版本演进

```rust
// V1版本
#[repr(C, align(128))]
pub struct OrderEntityPodV1 {
    // 字段...
    pub _reserved: [u64; 4],  // 预留空间
}

// V2版本（向后兼容）
#[repr(C, align(128))]
pub struct OrderEntityPodV2 {
    // 原有字段...
    pub new_field: u64,       // 使用预留空间
    pub _reserved: [u64; 3],  // 剩余预留
}
```

## 总结

遵循本规范可以实现：
- ✅ 零拷贝序列化/反序列化
- ✅ 缓存友好的内存布局
- ✅ SIMD向量化优化
- ✅ 极低的内存分配开销
- ✅ 高性能的批量处理
- ✅ 可预测的性能表现

这些特性对于构建低时延、高吞吐的CEX系统至关重要。
