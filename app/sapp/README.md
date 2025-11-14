# SAPP - 交易应用演示

## 概述

SAPP 是一个使用 LOB (Limit Order Book) 库的示例交易应用程序。

**特性**:
- 单线程 单个交易对（当前实现）
- 未来支持：单线程多个交易对

## 架构

```
app/sapp/              # 应用层
├── Cargo.toml        # 依赖 lib/lob
└── src/
    └── main.rs       # 应用入口

lib/lob/              # 库层（独立）
├── src/
│   └── lob/          # LOB引擎实现
└── tests/            # 集成测试（38个测试）
```

## 快速开始

### 运行应用
```bash
cargo run
```

### 输出示例
```
=== LOB引擎演示 ===

✓ 放置卖单 #1: 价格=10000, 数量=100
✓ 放置买单 #2: 价格=10000, 数量=50

成交记录:
  1. BUYER001 -> SELLER01 @ 10000 x 50

订单簿状态:
  最佳买价: None
  最佳卖价: Some(10000)
  价差: None
  活跃订单数: 1
  总成交数: 1

✓ LOB引擎运行正常！
```

## 使用 LOB 库

### 依赖配置

在 `Cargo.toml` 中：
```toml
[dependencies]
lob = { path = "../../lib/lob" }
```

### 基本用法

```rust
use lob::lob::{OrderBook, TraderId, Side};

fn main() {
    // 创建订单簿
    let mut book = OrderBook::new();

    // 创建交易员
    let buyer = TraderId::from_str("BUYER001");
    let seller = TraderId::from_str("SELLER01");

    // 放置卖单
    let (sell_order_id, _) = book.limit_order(seller, Side::Sell, 10000, 100);

    // 放置买单（匹配）
    let (buy_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 50);

    // 查询订单簿状态
    println!("最佳买价: {:?}", book.best_bid());
    println!("最佳卖价: {:?}", book.best_ask());
}
```

## Clean Architecture 实践

```
┌─────────────────────────────────────┐
│  应用层 (app/sapp)                   │
│  - 用户交互和展示逻辑                 │
└──────────────┬──────────────────────┘
               │ 依赖
               ▼
┌─────────────────────────────────────┐
│  库层 (lib/lob)                      │
│  - 领域模型和业务逻辑                 │
│  - 无外部依赖                        │
└─────────────────────────────────────┘
```

## 开发指南

### 修改库代码
```bash
cd ../../lib/lob
# 编辑 src/lob/*.rs
cargo test  # 运行测试
```

### 修改应用代码
```bash
cd app/sapp
# 编辑 src/main.rs
cargo run
```

## 参考资料

- [LOB库文档](../../lib/lob/README.md)
- [集成测试文档](../../lib/lob/tests/README.md)
- [Clean Architecture指南](../../CLAUDE.md)

