# FixedPointArithmetic 示例

## 快速开始

### 运行基础示例
```bash
cargo run --example basic
```

展示：
- 创建定点数
- 加减乘除运算
- 序列化/反序列化
- 批量处理
- Unsafe极速版本

### 运行交易示例
```bash
cargo run --example trading
```

展示：
- 订单价格计算
- 价差和中间价
- 手续费计算
- 网络传输（序列化）
- 批量订单处理

## 核心功能

### 1. 创建定点数
```rust
// 股票价格（0.01精度）
let price = FixedPointArithmetic::from_f64(123.45, -2)?;

// 加密货币（0.001精度）
let btc = FixedPointArithmetic::from_f64(45678.123, -3)?;
```

### 2. 算术运算
```rust
let sum = price1.checked_add(price2)?;
let diff = price1.checked_sub(price2)?;
let product = price.checked_mul(quantity)?;
let quotient = price.checked_div(divisor)?;
```

### 3. 网络传输
```rust
// 序列化（4字节）
let bytes = price.to_bytes();

// 反序列化
let restored = FixedPointArithmetic::from_bytes(bytes);
```

### 4. 高性能版本
```rust
unsafe {
    // 跳过所有检查，极速版本
    let fast = FixedPointArithmetic::from_f64_unchecked(100.0, -2);
    let result = fast.add_unchecked(other);
}
```

## 性能特点

- **内存**: 4字节（比f64节省50%）
- **精度**: 支持-8到7的tick_power
- **速度**:
  - 提取操作 < 1ns
  - 加减运算 < 3ns
  - 序列化 < 1ns
  - 批量处理优化

## 使用场景

- ✅ 高频交易
- ✅ 订单簿管理
- ✅ 市场数据传输
- ✅ 实时定价引擎
- ✅ 资金结算系统
