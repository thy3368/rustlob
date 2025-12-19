# FixedPointArithmetic 速查表

## 快速创建

```rust
use fixed_point_arithmetic::arithmetic::FixedPointArithmetic;

// 股票 (0.01)
let stock = FixedPointArithmetic::from_f64(123.45, -2)?;

// 加密货币 (0.001)
let btc = FixedPointArithmetic::from_f64(45678.123, -3)?;

// 高精度 (0.00000001)
let satoshi = FixedPointArithmetic::from_f64(0.00000123, -8)?;
```

## 常用操作

| 操作 | 代码 | 时延 |
|------|------|------|
| 创建 | `from_f64(123.45, -2)?` | ~15ns |
| 提取值 | `fp.value()` | < 1ns |
| 转f64 | `fp.to_f64()` | ~5ns |
| 快速转f64 | `fp.to_f64_fast()` | ~4ns |
| 加法 | `fp1.checked_add(fp2)?` | ~3ns |
| 减法 | `fp1.checked_sub(fp2)?` | ~3ns |
| 乘法 | `fp1.checked_mul(fp2)?` | ~12ns |
| 除法 | `fp1.checked_div(fp2)?` | ~15ns |
| 序列化 | `fp.to_bytes()` | < 1ns |
| 反序列化 | `from_bytes([...])`  | < 1ns |
| 比较 | `fp1 < fp2` | < 1ns |

## Unsafe极速版本

```rust
unsafe {
    // 创建 (~10ns)
    let fp = FixedPointArithmetic::from_f64_unchecked(100.0, -2);

    // 加法 (~2ns)
    let sum = fp1.add_unchecked(fp2);

    // 减法 (~2ns)
    let diff = fp1.sub_unchecked(fp2);

    // 快速转换 (~4ns)
    let value = fp.to_f64_fast();
}
```

## 批量处理

```rust
// 批量转换
let prices: Vec<FixedPointArithmetic> = vec![...];
let f64_prices = FixedPointArithmetic::batch_to_f64(&prices);

// x86_64 SIMD (4个一组)
#[cfg(target_arch = "x86_64")]
let batch: [FixedPointArithmetic; 4] = [...];
let results = FixedPointArithmetic::batch_to_f64_x4(&batch);
```

## 零拷贝网络

```rust
// 序列化
let bytes = price.to_bytes();  // [u8; 4]

// 反序列化
let price = FixedPointArithmetic::from_bytes(bytes);

// 零拷贝读取（最快）
unsafe {
    let price = FixedPointArithmetic::from_ptr(buffer.as_ptr());
}
```

## 性能数据

| 指标 | 值 |
|------|-----|
| 大小 | 4字节 (vs f64: 8字节) |
| 范围 | 0 - 268,435,455 ticks |
| 精度 | tick_power: -8 到 7 |
| 最大价格 | $2,684,354.55 (at -2) |
| 缓存行 | 16个/64字节 (vs f64: 8个) |

## 错误处理

```rust
match FixedPointArithmetic::from_f64(price, -2) {
    Ok(fp) => { /* 使用fp */ },
    Err(FixedPointError::ValueOverflow) => { /* 溢出 */ },
    Err(FixedPointError::InvalidTickPower) => { /* 精度错误 */ },
    Err(e) => { /* 其他错误 */ },
}
```

## 编译优化

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
target-cpu = "native"
```

```bash
# 构建
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## 典型场景

### 市场数据接收
```rust
#[repr(C, packed)]
struct MarketData {
    symbol: u32,
    bid: FixedPointArithmetic,
    ask: FixedPointArithmetic,
}
// 12字节 vs 20字节 (f64版本)
```

### 订单簿
```rust
use std::collections::BTreeMap;
let book: BTreeMap<FixedPointArithmetic, u64> = BTreeMap::new();
// 价格作为key，零开销比较
```

### 订单匹配
```rust
unsafe fn match_fast(bid: Order, ask: Order) -> Option<Trade> {
    if bid.price.value() >= ask.price.value() {
        Some(/* ... */)
    } else {
        None
    }
}
```

## 运行示例

```bash
# 基础示例
cargo run --example basic

# 交易示例
cargo run --example trading

# 测试
cargo test

# 文档
cargo doc --open
```

## 关键提醒

⚠️ **精度必须匹配**：不同tick_power不能直接运算
⚠️ **检查溢出**：最大值268,435,455 ticks
✅ **性能优先**：热路径使用unsafe版本
✅ **批量优化**：使用batch_*方法处理多条数据
