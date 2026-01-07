# 快速参考指南 (Quick Reference)

## 🚀 快速开始

### 1. 运行测试

```bash
# 正常交易流程测试
cargo test --test bdd_normal_trading_flow -- --nocapture

# 强平流程测试
cargo test --test bdd_order_to_liquidation -- --nocapture

# 运行特定测试
cargo test scenario_full_long_position_lifecycle -- --nocapture
```

### 2. 生成可视化图表

```bash
cd docs
python3 visualize_trading_flow.py
```

### 3. 查看文档

- **完整文档**: `docs/TRADING_FLOW.md`
- **总览文档**: `docs/README.md`
- **本文档**: `docs/QUICK_REFERENCE.md`

---

## 📋 核心API速查

### 设置杠杆 (Set Leverage)

```rust
use prep_proc::proc::trading_prep_order_proc::*;

let cmd = SetLeverageCommand::new(
    Symbol::new("BTCUSDT"),
    10  // 10倍杠杆
);

let result = service.set_leverage(cmd)?;
```

### 开仓 (Open Position)

```rust
// 市价多仓
let cmd = OpenPositionCommand::market_long(
    Symbol::new("BTCUSDT"),
    Quantity::from_f64(1.0)
).with_leverage(10);

let result = service.open_position(cmd)?;

// 市价空仓
let cmd = OpenPositionCommand::market_short(
    Symbol::new("BTCUSDT"),
    Quantity::from_f64(1.0)
).with_leverage(10);

// 限价多仓
let cmd = OpenPositionCommand::limit_long(
    Symbol::new("BTCUSDT"),
    Quantity::from_f64(1.0),
    Price::from_f64(50000.0)
).with_leverage(10);
```

### 平仓 (Close Position)

```rust
// 市价全仓平仓
let cmd = ClosePositionCommand::market_close_long(
    Symbol::new("BTCUSDT"),
    None  // None = 全部平仓
);

// 市价部分平仓
let cmd = ClosePositionCommand::market_close_long(
    Symbol::new("BTCUSDT"),
    Some(Quantity::from_f64(0.5))  // 平仓0.5 BTC
);

// 限价平仓 (止盈)
let cmd = ClosePositionCommand::limit_close_long(
    Symbol::new("BTCUSDT"),
    Quantity::from_f64(1.0),
    Price::from_f64(55000.0)  // 止盈价
);

let result = service.close_position(cmd)?;
```

### 查询持仓 (Query Position)

```rust
// 查询多仓
let cmd = QueryPositionCommand::long(Symbol::new("BTCUSDT"));
let position = service.query_position(cmd)?;

// 查询空仓
let cmd = QueryPositionCommand::short(Symbol::new("BTCUSDT"));
let position = service.query_position(cmd)?;

// 访问持仓信息
println!("数量: {}", position.quantity.to_f64());
println!("开仓价: {}", position.entry_price.to_f64());
println!("保证金: {}", position.margin.to_f64());
println!("杠杆: {}", position.leverage);
println!("未实现盈亏: {}", position.unrealized_pnl.to_f64());
```

---

## 🧮 核心计算公式

### 保证金计算

```
保证金 = 持仓价值 / 杠杆倍数
```

**示例**:
```
1 BTC @ 50,000 USDT, 10x杠杆
保证金 = 50,000 / 10 = 5,000 USDT
```

### 强平价格 (多仓)

```
强平价 = 开仓价 × (1 - 1/杠杆 + 0.005 + 0.005)
      = 开仓价 × (1 - 1/杠杆 + 0.01)
```

**示例**:
```
开仓价 50,000 USDT, 10x杠杆
强平价 = 50,000 × (1 - 0.1 + 0.01)
      = 50,000 × 0.91
      = 45,500 USDT
```

### 强平价格 (空仓)

```
强平价 = 开仓价 × (1 + 1/杠杆 - 0.005 - 0.005)
      = 开仓价 × (1 + 1/杠杆 - 0.01)
```

**示例**:
```
开仓价 50,000 USDT, 10x杠杆
强平价 = 50,000 × (1 + 0.1 - 0.01)
      = 50,000 × 1.09
      = 54,500 USDT
```

### 未实现盈亏

```
多仓: 未实现盈亏 = (当前价 - 开仓价) × 数量
空仓: 未实现盈亏 = (开仓价 - 当前价) × 数量
```

### 收益率

```
收益率 = 盈亏 / 保证金 × 100%
```

**示例**:
```
盈亏: 5,000 USDT
保证金: 5,000 USDT
收益率 = 5,000 / 5,000 × 100% = 100%
```

---

## 🎯 常用测试场景

### 场景1: 多仓盈利

```rust
// 1. 设置杠杆
service.set_leverage(SetLeverageCommand::new(symbol, 10))?;

// 2. 开多仓
let cmd = OpenPositionCommand::market_long(symbol, qty).with_leverage(10);
service.open_position(cmd)?;

// 3. 价格上涨，盈利

// 4. 主动平仓
let cmd = ClosePositionCommand::market_close_long(symbol, None);
service.close_position(cmd)?;
```

### 场景2: 空仓盈利

```rust
// 1. 设置杠杆
service.set_leverage(SetLeverageCommand::new(symbol, 10))?;

// 2. 开空仓
let cmd = OpenPositionCommand::market_short(symbol, qty).with_leverage(10);
service.open_position(cmd)?;

// 3. 价格下跌，盈利

// 4. 主动平仓
let cmd = ClosePositionCommand::market_close_short(symbol, None);
service.close_position(cmd)?;
```

### 场景3: 强平流程

```rust
// 1. 开仓
let cmd = OpenPositionCommand::market_long(symbol, qty).with_leverage(10);
let result = service.open_position(cmd)?;

// 2. 查询持仓获取强平价
let position = service.query_position(QueryPositionCommand::long(symbol))?;
let liq_price = calculate_liquidation_price(
    position.entry_price,
    10,
    PositionSide::Long
);

// 3. 价格跌至强平价，触发强平
let processor = LiquidationProcessor::new(...);
let result = processor.execute_liquidation(position_id, mark_price).await?;
```

---

## 📊 杠杆对比速查表

| 杠杆 | 保证金 | 强平距离(多) | 强平距离(空) | 风险 |
|------|--------|-------------|-------------|-----|
| 2x   | 50%    | 48.0%       | 52.0%       | 极低 |
| 5x   | 20%    | 19.0%       | 21.0%       | 低   |
| 10x  | 10%    | 9.0%        | 11.0%       | 中   |
| 20x  | 5%     | 4.0%        | 6.0%        | 高   |
| 50x  | 2%     | 1.0%        | 3.0%        | 极高 |

**强平距离** = 从开仓价到强平价的百分比变化

---

## ⚠️ 风险管理速查

### 杠杆选择

```
新手建议:    2x - 5x
中级交易者:  5x - 10x
专业交易者:  10x - 20x
极端风险:    20x+  (不推荐)
```

### 仓位管理

```
保守策略:  单笔 10-20% 总资金
均衡策略:  单笔 20-30% 总资金
激进策略:  单笔 30-50% 总资金

⚠️ 绝不超过 50% 总资金
```

### 止损设置

```rust
// 风险控制在5%
let stop_loss = entry_price * 0.95;

// 风险控制在10%
let stop_loss = entry_price * 0.90;
```

### 止盈设置

```rust
// 目标收益10%
let take_profit = entry_price * 1.10;

// 目标收益20%
let take_profit = entry_price * 1.20;
```

---

## 🔍 故障排查

### 问题1: 开仓失败

**可能原因**:
- ❌ 余额不足
- ❌ 杠杆未设置
- ❌ 参数验证失败

**解决方法**:
```rust
// 检查余额
let info = service.query_account_info(cmd)?;
println!("可用余额: {}", info.available_balance.to_f64());

// 设置杠杆
service.set_leverage(SetLeverageCommand::new(symbol, 10))?;

// 验证参数
assert!(quantity.to_f64() > 0.0);
```

### 问题2: 平仓失败

**可能原因**:
- ❌ 没有持仓
- ❌ 平仓数量超过持仓
- ❌ 订单类型错误

**解决方法**:
```rust
// 查询持仓
let position = service.query_position(QueryPositionCommand::long(symbol))?;
assert!(position.has_position());

// 确保平仓数量不超过持仓
let close_qty = position.quantity;
```

### 问题3: 测试失败

**可能原因**:
- ❌ 并发测试冲突
- ❌ 状态污染

**解决方法**:
```bash
# 单线程运行测试
cargo test -- --test-threads=1

# 查看详细输出
cargo test -- --nocapture

# 运行单个测试
cargo test scenario_full_long -- --nocapture
```

---

## 📚 常用命令

### 编译

```bash
# 编译库
cargo build

# 编译测试
cargo test --no-run

# 发布编译
cargo build --release
```

### 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试文件
cargo test --test bdd_normal_trading_flow

# 运行特定测试函数
cargo test scenario_full_long_position_lifecycle

# 详细输出
cargo test -- --nocapture

# 单线程运行
cargo test -- --test-threads=1

# 显示忽略的测试
cargo test -- --ignored
```

### 文档

```bash
# 生成文档
cargo doc

# 生成并打开文档
cargo doc --open

# 生成私有项文档
cargo doc --document-private-items
```

### 代码检查

```bash
# 检查代码
cargo check

# Clippy检查
cargo clippy

# 格式化代码
cargo fmt

# 检查格式
cargo fmt -- --check
```

---

## 🎨 可视化工具使用

### 生成所有图表

```bash
cd docs
python3 visualize_trading_flow.py
```

### 生成单个图表

```python
import matplotlib.pyplot as plt
from visualize_trading_flow import *

# 1. 正常交易流程
plot_normal_trading_flow()
plt.savefig('normal_flow.png', dpi=300)
plt.show()

# 2. 强平流程
plot_liquidation_flow()
plt.savefig('liquidation.png', dpi=300)
plt.show()

# 3. 杠杆对比
plot_leverage_comparison()
plt.savefig('leverage.png', dpi=300)
plt.show()

# 4. 盈亏分析
plot_pnl_analysis()
plt.savefig('pnl.png', dpi=300)
plt.show()
```

---

## 🔗 相关链接

- **详细文档**: [TRADING_FLOW.md](TRADING_FLOW.md)
- **总览**: [README.md](README.md)
- **源码**: `../src/proc/`
- **测试**: `../tests/`

---

## 💡 提示与技巧

### Tip 1: 使用Builder模式

```rust
let cmd = OpenPositionCommand::market_long(symbol, qty)
    .with_leverage(10)
    .with_stop_loss(stop_price);  // 如果支持
```

### Tip 2: 错误处理

```rust
match service.open_position(cmd) {
    Ok(result) => {
        println!("✅ 开仓成功: {}", result.order_id.as_str());
    }
    Err(e) => {
        eprintln!("❌ 开仓失败: {:?}", e);
    }
}
```

### Tip 3: 测试辅助函数

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;

    pub fn create_test_service() -> MatchingService {
        MatchingService::new(Price::from_f64(10000.0))
    }

    pub fn setup_leverage(service: &MatchingService, symbol: Symbol, lev: u8) {
        service.set_leverage(SetLeverageCommand::new(symbol, lev)).unwrap();
    }

    pub fn open_long(service: &MatchingService, symbol: Symbol, qty: f64) {
        let cmd = OpenPositionCommand::market_long(
            symbol,
            Quantity::from_f64(qty)
        ).with_leverage(10);
        service.open_position(cmd).unwrap();
    }
}
```

### Tip 4: 使用常量

```rust
const BTCUSDT: &str = "BTCUSDT";
const DEFAULT_LEVERAGE: u8 = 10;
const DEFAULT_BALANCE: f64 = 10000.0;

let symbol = Symbol::new(BTCUSDT);
let service = MatchingService::new(Price::from_f64(DEFAULT_BALANCE));
```

---

**最后更新**: 2025-12-13
**版本**: v1.0.0
