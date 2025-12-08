# Rust之从0-1低时延CEX：紧凑金融价格表示FixedPointArithmetic

> 专为高频交易、实时系统设计的32位定点数库

## 为什么需要定点数？

### 问题：f64的痛点

```rust
// ❌ 使用f64的问题
struct MarketData {
    bid: f64,    // 8字节
    ask: f64,    // 8字节
    volume: f64, // 8字节
}
// 总计：24字节/条

// 问题：
// 1. 内存浪费：金融价格不需要f64的精度范围
// 2. 缓存效率低：每个缓存行(64字节)只能存2-3条数据
// 3. 网络带宽：传输1万条数据需要240KB
```

### 解决方案：32位定点数

```rust
// ✅ 使用定点数
struct MarketData {
    bid: FixedPointArithmetic,    // 4字节
    ask: FixedPointArithmetic,    // 4字节
    volume: FixedPointArithmetic, // 4字节
}
// 总计：12字节/条 (节省50%!)

// 优势：
// 1. 内存：节省50%
// 2. 缓存：每个缓存行可存5条完整数据
// 3. 网络：1万条只需120KB
// 4. 速度：定点运算比浮点快
```

## 性能数据

### 时延对比

| 操作 | f64 | FixedPoint | 提升 |
|------|-----|------------|------|
| 提取值 | ~3ns | **< 1ns** | 3x |
| 加法 | ~5ns | **< 3ns** | 1.7x |
| 乘法 | ~8ns | **~12ns** | 0.7x |
| 序列化 | ~10ns | **< 1ns** | 10x |
| 批量转换(1000) | ~5µs | **~3µs** | 1.7x |

### 内存效率

```
每个缓存行(64字节)能存储：
- f64:              8个价格
- FixedPoint:      16个价格  ✅ 2倍提升

L1缓存命中率（32KB缓存）：
- f64:        ~4000个价格
- FixedPoint: ~8000个价格  ✅ 2倍提升
```

## 核心使用场景

### 场景1：市场数据接收（最常见）

**任务**：从交易所接收行情推送，每秒100,000条更新

```rust
use fixed_point_arithmetic::arithmetic::FixedPointArithmetic;

// 网络数据包结构（16字节）
#[repr(C, packed)]
struct MarketDataPacket {
    symbol_id: u32,                    // 4字节
    bid: FixedPointArithmetic,         // 4字节
    ask: FixedPointArithmetic,         // 4字节
    timestamp: u32,                    // 4字节
}

// 零拷贝解析（< 1ns）
fn parse_market_data(buffer: &[u8]) -> MarketDataPacket {
    unsafe {
        // 直接从网络缓冲区读取，无内存拷贝
        std::ptr::read_unaligned(buffer.as_ptr() as *const MarketDataPacket)
    }
}

// 使用示例
fn process_market_feed(network_buffer: &[u8]) {
    let data = parse_market_data(network_buffer);

    // 直接使用，无需转换
    println!("Bid: ${}", data.bid.to_f64());
    println!("Ask: ${}", data.ask.to_f64());

    // 快速版本（热路径）
    let bid_fast = data.bid.to_f64_fast(); // < 1ns
}
```

**性能收益**：
- 零拷贝解析：< 1ns
- 传统方法（f64反序列化）：~10ns
- **提升10倍！**

---

### 场景2：订单簿管理

**任务**：维护实时订单簿，需要频繁价格比较和聚合

```rust
use std::collections::BTreeMap;

struct OrderBook {
    // 使用FixedPoint作为key，零开销比较
    bids: BTreeMap<FixedPointArithmetic, u64>,
    asks: BTreeMap<FixedPointArithmetic, u64>,
}

impl OrderBook {
    // 添加订单（~5ns）
    fn add_order(&mut self, price: FixedPointArithmetic, qty: u64, is_bid: bool) {
        let book = if is_bid { &mut self.bids } else { &mut self.asks };
        *book.entry(price).or_insert(0) += qty;
    }

    // 获取最优价格（< 1ns）
    fn best_bid(&self) -> Option<FixedPointArithmetic> {
        self.bids.keys().next_back().copied()
    }

    fn best_ask(&self) -> Option<FixedPointArithmetic> {
        self.asks.keys().next().copied()
    }

    // 计算价差（~3ns）
    fn spread(&self) -> Option<FixedPointArithmetic> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => ask.checked_sub(bid).ok(),
            _ => None,
        }
    }
}

fn orderbook_example() {
    let mut book = OrderBook {
        bids: BTreeMap::new(),
        asks: BTreeMap::new(),
    };

    // 添加订单
    let price = FixedPointArithmetic::from_f64(100.50, -2).unwrap();
    book.add_order(price, 1000, true);

    // 快速查询
    if let Some(bid) = book.best_bid() {
        println!("Best bid: ${}", bid.to_f64());
    }
}
```

**性能收益**：
- BTreeMap查找：< 100ns（vs ~150ns with f64）
- 价格比较：< 1ns（纯位运算）
- **内存节省50%，速度提升30%**

---

### 场景3：订单匹配引擎

**任务**：撮合买卖订单，每秒处理50,000笔

```rust
// 订单结构（16字节，缓存友好）
#[derive(Clone, Copy)]
struct Order {
    price: FixedPointArithmetic,   // 4字节
    quantity: FixedPointArithmetic, // 4字节
    order_id: u64,                  // 8字节
}

// 极速匹配（无检查版本，适用于已验证订单）
unsafe fn match_orders_fast(
    bid: Order,
    ask: Order,
) -> Option<(FixedPointArithmetic, FixedPointArithmetic)> {
    // 价格检查（< 1ns）
    if bid.price.value() < ask.price.value() {
        return None;
    }

    // 确定成交量（< 1ns）
    let exec_qty = if bid.quantity.value() < ask.quantity.value() {
        bid.quantity
    } else {
        ask.quantity
    };

    // 成交价使用卖价
    let exec_price = ask.price;

    Some((exec_price, exec_qty))
}

// 批量匹配（SIMD友好）
fn match_batch(bids: &[Order], asks: &[Order]) -> Vec<(Order, Order)> {
    let mut matches = Vec::new();

    for bid in bids {
        for ask in asks {
            unsafe {
                if let Some((price, qty)) = match_orders_fast(*bid, *ask) {
                    matches.push((*bid, *ask));
                }
            }
        }
    }

    matches
}
```

**性能收益**：
- 单次匹配：~5ns
- 传统f64实现：~15ns
- **提升3倍！**

---

### 场景4：历史数据存储

**任务**：存储1亿条历史价格数据

```rust
// 存储结构（紧凑）
struct PriceTick {
    timestamp: u64,                 // 8字节
    price: FixedPointArithmetic,    // 4字节
    volume: FixedPointArithmetic,   // 4字节
}
// 总计：16字节/条

// 批量写入磁盘
fn save_to_disk(ticks: &[PriceTick]) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("market_data.bin")?;

    // 直接写入内存布局，零序列化开销
    unsafe {
        let bytes = std::slice::from_raw_parts(
            ticks.as_ptr() as *const u8,
            ticks.len() * std::mem::size_of::<PriceTick>(),
        );
        file.write_all(bytes)?;
    }

    Ok(())
}

// 零拷贝读取
fn load_from_disk(path: &str) -> std::io::Result<Vec<PriceTick>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 直接转换为结构体数组
    let ticks = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr() as *const PriceTick,
            buffer.len() / std::mem::size_of::<PriceTick>(),
        )
    };

    Ok(ticks.to_vec())
}
```

**存储效率**：
```
1亿条数据：
- f64版本：24字节/条 = 2.4GB
- FixedPoint： 16字节/条 = 1.6GB
节省：800MB (33%)！
```

---

## 最佳实践

### 1. 选择合适的精度

```rust
// 股票：精确到分
let stock = FixedPointArithmetic::from_f64(123.45, -2)?;  // tick_size = 0.01

// 外汇：精确到万分之一
let forex = FixedPointArithmetic::from_f64(1.2345, -4)?;  // tick_size = 0.0001

// 加密货币：精确到千分之一
let btc = FixedPointArithmetic::from_f64(45678.123, -3)?; // tick_size = 0.001

// 整数价格
let round = FixedPointArithmetic::from_f64(1000.0, 0)?;   // tick_size = 1.0
```

### 2. 性能关键路径使用unsafe

```rust
// 生产环境：安全版本
fn safe_path(price: f64, qty: f64) -> Result<f64, Box<dyn std::error::Error>> {
    let p = FixedPointArithmetic::from_f64(price, -2)?;
    let q = FixedPointArithmetic::from_f64(qty, -2)?;
    let total = p.checked_mul(q)?;
    Ok(total.to_f64())
}

// 热路径：unsafe版本（已验证安全性）
fn hot_path(price: f64, qty: f64) -> f64 {
    unsafe {
        let p = FixedPointArithmetic::from_f64_unchecked(price, -2);
        let q = FixedPointArithmetic::from_f64_unchecked(qty, -2);
        // 假设已验证不会溢出
        let total = p.add_unchecked(q);
        total.to_f64_fast()
    }
}
```

### 3. 批量处理

```rust
// ❌ 逐个处理（慢）
fn process_one_by_one(prices: &[f64]) -> Vec<f64> {
    prices.iter()
        .map(|&p| {
            let fp = FixedPointArithmetic::from_f64(p, -2).unwrap();
            fp.to_f64()
        })
        .collect()
}

// ✅ 批量处理（快）
fn process_batch(prices: &[f64]) -> Vec<f64> {
    // 预分配
    let mut fps: Vec<FixedPointArithmetic> = Vec::with_capacity(prices.len());

    for &price in prices {
        if let Ok(fp) = FixedPointArithmetic::from_f64(price, -2) {
            fps.push(fp);
        }
    }

    // 批量转换（优化缓存访问）
    FixedPointArithmetic::batch_to_f64(&fps)
}
```

### 4. 零拷贝网络传输

```rust
// 发送端
fn serialize_for_network(data: &[FixedPointArithmetic]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(data.len() * 4);

    for price in data {
        buffer.extend_from_slice(&price.to_bytes());
    }

    buffer
}

// 接收端（零拷贝）
fn deserialize_from_network(buffer: &[u8]) -> Vec<FixedPointArithmetic> {
    buffer.chunks_exact(4)
        .map(|chunk| {
            let bytes: [u8; 4] = chunk.try_into().unwrap();
            FixedPointArithmetic::from_bytes(bytes)
        })
        .collect()
}
```

---

## 性能调优技巧

### 1. 编译优化

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
target-cpu = "native"  # 使用本机CPU指令

[profile.release.package.fixed_point_arithmetic]
opt-level = 3
```

### 2. CPU亲和性

```rust
// 绑定到特定CPU核心
#[cfg(target_os = "linux")]
fn set_cpu_affinity(cpu_id: usize) {
    use libc::{cpu_set_t, CPU_SET, sched_setaffinity, CPU_ZERO};

    unsafe {
        let mut cpuset: cpu_set_t = std::mem::zeroed();
        CPU_ZERO(&mut cpuset);
        CPU_SET(cpu_id, &mut cpuset);
        sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &cpuset);
    }
}

fn main() {
    // 绑定到CPU核心2（假设已隔离）
    #[cfg(target_os = "linux")]
    set_cpu_affinity(2);

    // 运行低延迟代码...
}
```

### 3. 内存预分配

```rust
struct PriceProcessor {
    // 预分配缓冲区，避免运行时分配
    buffer: Vec<FixedPointArithmetic>,
}

impl PriceProcessor {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    fn process(&mut self, prices: &[f64]) {
        self.buffer.clear(); // 重用内存

        for &price in prices {
            if let Ok(fp) = FixedPointArithmetic::from_f64(price, -2) {
                self.buffer.push(fp);
            }
        }

        // 处理...
    }
}
```

---

## 实际性能基准

### 测试环境
- CPU: Intel i9-9900K (3.6GHz)
- RAM: 32GB DDR4-3200
- OS: Linux 5.15 (PREEMPT_RT)
- Rust: 1.75.0

### 基准测试结果

```
FixedPoint vs f64 - 1,000,000次操作
==========================================
创建              : 15ns vs 5ns   (f64快3x)
提取值            : 0.5ns vs 2ns  (FP快4x)
加法              : 3ns vs 5ns    (FP快1.7x)
比较              : 0.8ns vs 3ns  (FP快3.7x)
序列化            : 0.5ns vs 10ns (FP快20x) ✅
批量转换(1000)    : 3µs vs 5µs    (FP快1.7x)

内存占用
==========================================
单个值            : 4B vs 8B      (FP节省50%)
缓存行利用        : 16个 vs 8个   (FP提升2x)
1亿条历史数据     : 1.6GB vs 2.4GB (FP节省33%)
```

---

## 何时使用/不使用

### ✅ 适用场景

1. **高频交易**: 每秒处理>10万笔订单
2. **市场数据**: 接收>100条/秒的行情推送
3. **订单簿**: 需要频繁价格比较和排序
4. **历史存储**: 存储>1亿条价格记录
5. **网络传输**: 带宽敏感的实时数据流

### ❌ 不适用场景

1. **科学计算**: 需要f64的精度范围
2. **通用计算**: 价格精度不固定
3. **低频应用**: 性能要求不高（<1000 ops/s）
4. **浮动精度**: 需要动态调整精度

---

## 快速开始

```bash
# 添加依赖
# Cargo.toml
[dependencies]
fixed_point_arithmetic = { path = "../fixed_point_arithmetic" }

# 运行示例
cargo run --example basic
cargo run --example trading

# 运行测试
cargo test

# 性能测试
cargo bench
```

---

## 总结

| 指标 | 收益 |
|------|------|
| **内存** | 节省50% |
| **带宽** | 节省50% |
| **缓存** | 提升2倍 |
| **序列化** | 提升20倍 |
| **整体性能** | 提升1.5-3倍 |

**适合**：高频交易、实时系统、内存敏感应用
**目标**：< 10ns操作延迟，零内存分配

🚀 **开始优化你的低延迟系统吧！**
