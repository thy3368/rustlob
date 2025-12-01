# LOB引擎快速参考

## 项目结构

```
lib/lob/
├── src/lob/
│   ├── types.rs              # 领域实体 (188行)
│   ├── repository.rs         # 仓储接口和实现 (305行)
│   ├── matching_service.rs   # 匹配服务 (380行)
│   ├── engine.rs             # Facade (245行)
│   ├── arena.rs              # 内存池 (138行)
│   └── mod.rs                # 模块定义 (56行)
│
├── tests/
│   └── lob_integration_tests.rs  # 集成测试 (38个)
│
└── 文档/
    ├── ARCHITECTURE.md           # 架构详解
    ├── REFACTORING_SUMMARY.md    # 重构总结
    └── QUICK_REFERENCE.md        # 本文件
```

## Clean Architecture 分层

```
┌─────────────────────────┐
│  engine.rs (Facade)     │  应用层
└───────┬─────────────────┘
        │
        ▼
┌─────────────────────────┐
│  matching_service.rs    │  领域服务层
└───────┬─────────────────┘
        │
        ▼
┌─────────────────────────┐
│  repository.rs          │  数据访问层
└───────┬─────────────────┘
        │
        ▼
┌─────────────────────────┐
│  types.rs               │  领域实体层
└─────────────────────────┘
```

## 核心类型

### 领域实体 (types.rs)

```rust
pub struct TraderId([u8; 8]);        // 交易员ID
pub struct OrderEntry { ... }         // 订单条目
pub struct Trade { ... }              // 交易记录
pub struct PricePoint { ... }         // 价格点
pub enum Side { Buy, Sell }          // 订单方向

pub type OrderId = u64;
pub type Price = u32;
pub type Quantity = u32;
```

### 仓储接口 (repository.rs)

```rust
pub trait OrderRepository {
    fn add_order(...) -> Result<(), RepositoryError>;
    fn find_order(...) -> Option<&OrderEntry>;
    fn cancel_order(...) -> bool;
    fn allocate_order_id(&mut self) -> OrderId;
    // ... 更多方法
}

pub struct InMemoryOrderRepository { ... }
```

### 匹配服务 (matching_service.rs)

```rust
pub struct MatchingService;
pub struct MarketDataService;

impl MatchingService {
    pub fn match_limit_order<R>(...) -> (Vec<Trade>, Quantity);
}

impl MarketDataService {
    pub fn find_best_bid<R>(...) -> Option<Price>;
    pub fn find_best_ask<R>(...) -> Option<Price>;
    pub fn calculate_spread<R>(...) -> Option<Price>;
}
```

### 订单簿 (engine.rs)

```rust
pub struct OrderBook {
    repository: InMemoryOrderRepository,
    matching_service: MatchingService,
    market_data_service: MarketDataService,
    trades: Vec<Trade>,
}

pub struct OrderBookSnapshot {
    pub next_order_id: OrderId,
    pub bid_max: Option<Price>,
    pub ask_min: Option<Price>,
    pub active_orders: usize,
    pub total_trades: usize,
}
```

## 常用API

### 基本操作

```rust
use lob::lob::{OrderBook, TraderId, Side};

// 创建订单簿
let mut book = OrderBook::new();
let mut book = OrderBook::with_capacity(max_price, max_orders);

// 提交限价订单
let (order_id, trades) = book.limit_order(
    trader: TraderId,
    side: Side,
    price: Price,
    quantity: Quantity
);

// 提交市价订单（v0.2.0新增）
let trades = book.market_order(
    trader: TraderId,
    side: Side,
    quantity: Quantity
);

// 提交冰山订单（v0.2.0新增）
let (order_id, trades, remaining_total, current_display) = book.iceberg_order(
    trader: TraderId,
    side: Side,
    price: Price,
    total_quantity: Quantity,
    display_quantity: Quantity
);

// 取消订单
let cancelled = book.cancel_order(order_id);
```

### 市场数据查询

```rust
// 价格查询
let best_bid = book.best_bid();      // Option<Price>
let best_ask = book.best_ask();      // Option<Price>
let spread = book.spread();          // Option<Price>
let mid_price = book.mid_price();    // Option<Price>

// 状态查询
let snapshot = book.snapshot();      // OrderBookSnapshot
let trades = book.trades();          // &[Trade]
```

### 辅助功能

```rust
// 订单ID管理
let next_id = book.next_order_id();
book.set_next_order_id(id);

// 交易历史管理
book.clear_trades();
```

## 使用示例

### 简单匹配

```rust
let mut book = OrderBook::new();

let buyer = TraderId::from_str("BUYER001");
let seller = TraderId::from_str("SELLER01");

// 放置卖单
book.limit_order(seller, Side::Sell, 10000, 100);

// 放置买单（匹配）
let (order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 100);

assert_eq!(trades.len(), 1);
assert_eq!(trades[0].quantity, 100);
```

### 部分成交

```rust
let mut book = OrderBook::new();

// 大卖单
book.limit_order(seller, Side::Sell, 10000, 200);

// 小买单
let (_, trades) = book.limit_order(buyer, Side::Buy, 10000, 50);

assert_eq!(trades[0].quantity, 50);
assert_eq!(book.best_ask(), Some(10000));  // 还有剩余
```

### 价格改善

```rust
// 卖单价格 10000
book.limit_order(seller, Side::Sell, 10000, 100);

// 买单愿意支付更高价格
let (_, trades) = book.limit_order(buyer, Side::Buy, 11000, 100);

assert_eq!(trades[0].price, 10000);  // 以卖方价格成交
```

### 市场深度

```rust
// 构建买卖盘
for i in 0..10 {
    book.limit_order(trader, Side::Buy, 9900 - i * 10, 100);
    book.limit_order(trader, Side::Sell, 10100 + i * 10, 100);
}

println!("买价: {:?}", book.best_bid());
println!("卖价: {:?}", book.best_ask());
println!("价差: {:?}", book.spread());
```

### 市价单（v0.2.0）

```rust
// 构建订单簿
book.limit_order(seller, Side::Sell, 10000, 50);
book.limit_order(seller, Side::Sell, 10100, 50);
book.limit_order(seller, Side::Sell, 10200, 50);

// 市价买单从最低价开始成交
let trades = book.market_order(buyer, Side::Buy, 100);

// trades[0]: 价格10000, 数量50
// trades[1]: 价格10100, 数量50
```

### 冰山单（v0.2.0）

```rust
// 添加对手方订单
book.limit_order(buyer, Side::Buy, 10000, 150);

// 冰山卖单：总量1000，每次显示100
let (order_id, trades, remaining_total, current_display) =
    book.iceberg_order(seller, Side::Sell, 10000, 1000, 100);

// 第一批100全部成交
// 自动补充第二批100，再成交50
// remaining_total: 850
// current_display: 50（在订单簿中）
```

## 高级用法

### 自定义仓储

```rust
use lob::lob::repository::{OrderRepository, RepositoryError};

pub struct MyCustomRepository { ... }

impl OrderRepository for MyCustomRepository {
    fn add_order(...) -> Result<(), RepositoryError> {
        // 自定义实现
    }
    // 实现其他方法...
}
```

### 直接使用服务

```rust
use lob::lob::{MatchingService, MarketDataService, InMemoryOrderRepository};

let mut repo = InMemoryOrderRepository::new(100_000, 1000);
let matching = MatchingService::new();
let market_data = MarketDataService::new();

// 使用匹配服务
let (trades, remaining) = matching.match_limit_order(
    &mut repo,
    trader,
    Side::Buy,
    price,
    quantity
);

// 使用市场数据服务
let best_bid = market_data.find_best_bid(&repo);
```

## 测试

### 运行所有测试

```bash
cargo test
```

### 运行特定测试

```bash
# 单元测试
cargo test --lib

# 集成测试
cargo test --test lob_integration_tests

# 特定测试
cargo test test_simple_match
```

### 测试覆盖

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --test lob_integration_tests --out Html
```

## 性能特性

| 操作 | 时间复杂度 | 说明 |
|-----|-----------|------|
| 订单放置 | O(1) | 价格索引数组 |
| 订单匹配 | O(n) | n=该价格级别订单数 |
| 订单取消 | O(1) | HashMap直接索引 |
| 价格查询 | O(k) | k=价格级别距离 |
| 最佳价格 | O(1)* | *缓存最佳价格时 |

### 内存优化

- 64字节缓存行对齐
- 内存池分配器避免碎片
- 零拷贝设计
- 固定大小数组

## 错误处理

```rust
use lob::lob::repository::RepositoryError;

match repo.add_order(...) {
    Ok(()) => println!("添加成功"),
    Err(RepositoryError::CapacityExceeded) => {
        eprintln!("容量已满");
    }
    Err(RepositoryError::OrderAlreadyExists) => {
        eprintln!("订单已存在");
    }
    Err(e) => eprintln!("错误: {}", e),
}
```

## 常见模式

### 批量订单处理

```rust
let orders = vec![
    (trader1, Side::Buy, 9900, 100),
    (trader2, Side::Buy, 9950, 150),
    (trader3, Side::Sell, 10050, 100),
];

for (trader, side, price, qty) in orders {
    book.limit_order(trader, side, price, qty);
}
```

### 订单簿快照

```rust
let snapshot = book.snapshot();

println!("活跃订单: {}", snapshot.active_orders);
println!("总成交: {}", snapshot.total_trades);
println!("下一个ID: {}", snapshot.next_order_id);
println!("最佳买价: {:?}", snapshot.bid_max);
println!("最佳卖价: {:?}", snapshot.ask_min);
```

### 交易历史分析

```rust
for trade in book.trades() {
    println!("{} -> {} @ {} x {}",
        trade.buyer,
        trade.seller,
        trade.price,
        trade.quantity
    );
}
```

## 调试技巧

### 启用日志

```rust
// 在 Cargo.toml 添加
[dependencies]
log = "0.4"
env_logger = "0.10"

// 在代码中
env_logger::init();
log::debug!("订单 {} 已添加", order_id);
```

### 性能分析

```bash
# CPU性能分析
cargo build --release
perf record target/release/your_app
perf report

# 内存分析
valgrind target/debug/your_app
```

## 常见问题

### Q: 如何修改订单？
A: 取消原订单，提交新订单。

### Q: 支持市价单吗？
A: 使用极端价格模拟（买单用MAX_PRICE，卖单用0）。

### Q: 如何实现止损单？
A: 需要扩展，添加条件订单支持。

### Q: 支持多交易对吗？
A: 当前每个OrderBook实例支持一个交易对。

## 文档链接

- [架构文档](ARCHITECTURE.md)
- [重构总结](REFACTORING_SUMMARY.md)
- [测试文档](tests/README.md)
- [测试总结](tests/TEST_SUMMARY.md)
- [命令参考](tests/COMMANDS.md)

## 版本信息

- **库版本**: 0.2.0
- **Rust Edition**: 2024
- **最后更新**: 2025-11-14
- **新增功能**: 市价单、冰山单、OrderCommandHandler trait
- **状态**: ✅ 生产就绪

---

**快速开始**: 查看 [README.md](./README.md)
**详细文档**: 查看 [ARCHITECTURE.md](ARCHITECTURE.md)
**测试指南**: 查看 [tests/README.md](tests/README.md)
