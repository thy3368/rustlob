# Rust之从0-1低时延CEX：Spot订单管理流程主要Command



**版本**: v2.1
**最后更新**: 2025-01-05
**基于代码**: `trading_spot_order_mng.rs`

---

## 📚 目录

- [API 概览](#api-概览)
- [快速开始](#快速开始)
- [命令接口](#命令接口)
  - [LimitOrder - 限价单](#limitorder---限价单)
  - [MarketOrder - 市价单](#marketorder---市价单)
  - [CancelOrder - 取消订单](#cancelorder---取消订单)
  - [CancelAllOrders - 批量取消](#cancelallorders---批量取消)
- [查询接口](#查询接口)
  - [QueryOpenOrders - 查询活跃订单](#queryopenorders---查询活跃订单)
  - [QueryOrderDetail - 查询订单详情](#queryorderdetail---查询订单详情)
  - [QueryOrderHistory - 查询历史订单](#queryorderhistory---查询历史订单)
  - [QueryTradeHistory - 查询成交记录](#querytradehistory---查询成交记录)
- [数据类型](#数据类型)
- [错误处理](#错误处理)
- [最佳实践](#最佳实践)

---

## API 概览

### 架构模式

本 API 采用 **CQRS (Command Query Responsibility Segregation)** 模式：

- **命令端 (Command)**: 写操作，修改系统状态
- **查询端 (Query)**: 读操作，不修改状态

### 幂等性保证

所有命令通过 `Command<C>` 包装实现幂等性：

```rust
pub struct Command<C> {
    pub nonce: Nonce,           // 客户端生成的唯一标识
    pub timestamp_ms: u64,      // 命令时间戳
    pub payload: C,             // 实际命令内容
}
```

**重要**: 同一 `nonce` 的命令只会被执行一次，重复提交会返回缓存结果。

### 响应格式

```rust
pub struct CommandResponse<T> {
    pub metadata: CommandMetadata {
        pub nonce: Nonce,
        pub is_duplicate: bool,      // 是否为重复命令
        pub received_at: u64,        // 服务器接收时间
    },
    pub result: T,                   // 实际结果
}
```

---

## 快速开始

### 安装依赖

```toml
[dependencies]
lob = { path = "../lib/core/exchange/lob" }
tokio = { version = "1", features = ["full"] }
```

### 基础使用示例

```rust
use lob::lob::{
    SpotCommand, SpotOrderHandler, Command,
    Symbol, TraderId, Side, Price, Quantity, TimeInForce,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建匹配服务（示例）
    let mut matching_service = create_matching_service();

    // 2. 创建限价单命令
    let nonce = generate_nonce();
    let command = Command::new(nonce, SpotCommand::LimitOrder {
        trader: TraderId::from_str("TRADER001"),
        symbol: Symbol::from_str("BTCUSDT"),
        side: Side::Buy,
        price: 50000,
        quantity: 100,
        time_in_force: TimeInForce::GoodTillCancel,
        client_order_id: Some("MY-ORDER-001".to_string()),
    });

    // 3. 执行命令
    let response = matching_service.handle(command)?;

    // 4. 处理响应
    if let SpotCommandResult::LimitOrder { order_id, status, .. } = response.result {
        println!("订单创建成功: order_id={}, status={:?}", order_id, status);
    }

    Ok(())
}
```

---

## 命令接口

### LimitOrder - 限价单

**用途**: 以指定价格或更优价格执行订单

#### Rust API

```rust
SpotCommand::LimitOrder {
    trader: TraderId,
    symbol: Symbol,
    side: Side,
    price: Price,
    quantity: Quantity,
    time_in_force: TimeInForce,
    client_order_id: Option<String>,
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `trader` | `TraderId` | ✅ | 交易员ID（8字节固定长度） |
| `symbol` | `Symbol` | ✅ | 交易对（如 BTCUSDT） |
| `side` | `Side` | ✅ | 买卖方向：`Side::Buy` / `Side::Sell` |
| `price` | `Price` | ✅ | 限价价格（u32） |
| `quantity` | `Quantity` | ✅ | 订单数量（u32） |
| `time_in_force` | `TimeInForce` | ✅ | 订单有效期类型 |
| `client_order_id` | `Option<String>` | ❌ | 客户端订单ID（可选） |

#### TimeInForce 类型

```rust
pub enum TimeInForce {
    /// GTC - Good Till Cancel (撤单前一直有效)
    GoodTillCancel,

    /// IOC - Immediate Or Cancel (立即成交，未成交部分自动取消)
    ImmediateOrCancel,

    /// FOK - Fill Or Kill (全部成交或全部拒绝)
    FillOrKill,

    /// GTD - Good Till Date (有效至指定时间戳)
    GoodTillDate(u64),

    /// PostOnly - 只做 Maker，不吃单（如果会立即成交则拒绝）
    PostOnly,
}
```

#### 返回值

```rust
SpotCommandResult::LimitOrder {
    order_id: OrderId,              // 订单ID
    status: OrderStatus,            // 订单状态
    filled_quantity: Quantity,      // 已成交数量
    remaining_quantity: Quantity,   // 剩余数量
    trades: Vec<Trade>,             // 成交记录列表
}
```

#### 订单状态

```rust
pub enum OrderStatus {
    Initial,           // 初始状态
    Pending,           // 等待成交
    PartiallyFilled,   // 部分成交
    Filled,            // 完全成交
    Cancelling,        // 取消中
    Cancelled,         // 已取消
    Rejected,          // 被拒绝（FOK/PostOnly）
    Expired,           // 已过期（GTD）
}
```

#### 代码示例

##### 示例 1: GTC 限价买单

```rust
let command = Command::new(generate_nonce(), SpotCommand::LimitOrder {
    trader: TraderId::from_str("TRADER001"),
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    price: 50000,
    quantity: 100,
    time_in_force: TimeInForce::GoodTillCancel,
    client_order_id: Some("GTC-BUY-001".to_string()),
});

let response = handler.handle(command)?;
```

##### 示例 2: PostOnly 挂单（做市商）

```rust
let command = Command::new(generate_nonce(), SpotCommand::LimitOrder {
    trader: TraderId::from_str("MARKET_MAKER_01"),
    symbol: Symbol::from_str("ETHUSDT"),
    side: Side::Sell,
    price: 3000,
    quantity: 500,
    time_in_force: TimeInForce::PostOnly,  // 如果会立即成交则拒绝
    client_order_id: Some("MM-SELL-001".to_string()),
});

match handler.handle(command)?.result {
    SpotCommandResult::LimitOrder { status: OrderStatus::Rejected, .. } => {
        println!("PostOnly订单被拒绝：会立即成交");
    }
    SpotCommandResult::LimitOrder { order_id, status: OrderStatus::Pending, .. } => {
        println!("PostOnly订单挂单成功: {}", order_id);
    }
    _ => {}
}
```

##### 示例 3: IOC 立即成交或取消

```rust
let command = Command::new(generate_nonce(), SpotCommand::LimitOrder {
    trader: TraderId::from_str("TRADER002"),
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    price: 51000,
    quantity: 50,
    time_in_force: TimeInForce::ImmediateOrCancel,
    client_order_id: None,
});

let response = handler.handle(command)?;
match response.result {
    SpotCommandResult::LimitOrder { filled_quantity, status, .. } => {
        if status == OrderStatus::Filled {
            println!("全部成交: {} 单位", filled_quantity);
        } else if status == OrderStatus::Cancelled {
            println!("部分成交或未成交，剩余取消");
        }
    }
    _ => {}
}
```

##### 示例 4: FOK 全部成交或全部拒绝

```rust
let command = Command::new(generate_nonce(), SpotCommand::LimitOrder {
    trader: TraderId::from_str("TRADER003"),
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    price: 50000,
    quantity: 1000,
    time_in_force: TimeInForce::FillOrKill,
    client_order_id: Some("FOK-001".to_string()),
});

let response = handler.handle(command)?;
match response.result {
    SpotCommandResult::LimitOrder { status: OrderStatus::Filled, .. } => {
        println!("FOK订单全部成交");
    }
    SpotCommandResult::LimitOrder { status: OrderStatus::Rejected, .. } => {
        println!("FOK订单被拒绝：无法全部成交");
    }
    _ => {}
}
```

#### 可能的错误

```rust
// 余额不足
CommonError::InsufficientBalance {
    required: 5000000,
    available: 3000000
}

// 价格超出范围
SpotCommandError::PriceOutOfRange {
    price: 100000,
    min: 10000,
    max: 80000
}

// 数量超出范围
SpotCommandError::QuantityOutOfRange {
    quantity: 10,
    min: 100,
    max: 10000
}

// FOK 订单被拒绝
SpotCommandError::FillOrKillRejected {
    order_id: 1001,
    filled: 500,
    requested: 1000
}
```

---

### MarketOrder - 市价单

**用途**: 以当前市场最优价格立即执行订单

**⚠️ 实现状态**: 接口已定义，等待实现

#### Rust API

```rust
SpotCommand::MarketOrder {
    trader: TraderId,
    symbol: Symbol,
    side: Side,
    quantity: Quantity,
    price_limit: Option<Price>,      // 价格保护（强烈推荐）
    client_order_id: Option<String>,
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `trader` | `TraderId` | ✅ | 交易员ID |
| `symbol` | `Symbol` | ✅ | 交易对 |
| `side` | `Side` | ✅ | 买卖方向 |
| `quantity` | `Quantity` | ✅ | 订单数量 |
| `price_limit` | `Option<Price>` | ⚠️ | **价格保护**：买单最高价/卖单最低价 |
| `client_order_id` | `Option<String>` | ❌ | 客户端订单ID |

#### 价格保护机制（price_limit）

**强烈推荐设置 `price_limit`，否则有巨大滑点风险！**

- **买单**: `price_limit` 为最高买入价，超过此价则停止吃单
- **卖单**: `price_limit` 为最低卖出价，低于此价则停止吃单

**示例场景**:

```
市场深度：
  卖1: 50000 @ 10
  卖2: 51000 @ 20
  卖3: 60000 @ 100  ← 深度不足，价格跳涨

买入 100 BTC 市价单：
  - 无 price_limit: 会吃完所有卖单，最后以 60000 成交 → 亏损巨大 ❌
  - price_limit = 52000: 只成交 30 BTC (50k×10 + 51k×20)，剩余 70 BTC 取消 → 保护用户 ✅
```

#### 返回值

```rust
SpotCommandResult::MarketOrder {
    status: OrderStatus,        // Filled | PartiallyFilled
    filled_quantity: Quantity,  // 已成交数量
    trades: Vec<Trade>,         // 成交记录
}
```

#### 代码示例

```rust
// 带价格保护的市价买单（推荐）
let command = Command::new(generate_nonce(), SpotCommand::MarketOrder {
    trader: TraderId::from_str("TRADER001"),
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    quantity: 100,
    price_limit: Some(52000),  // 最高买入价：52000
    client_order_id: Some("MARKET-BUY-001".to_string()),
});

let response = handler.handle(command)?;
match response.result {
    SpotCommandResult::MarketOrder { filled_quantity, status, .. } => {
        if status == OrderStatus::Filled {
            println!("市价单全部成交: {} 单位", filled_quantity);
        } else {
            println!("市价单部分成交: {} 单位（价格超出限制）", filled_quantity);
        }
    }
    _ => {}
}
```

#### 当前实现状态

```rust
// 暂时返回错误
CommonError::InvalidParameter {
    field: "command",
    reason: "MarketOrder not implemented yet"
}
```

---

### CancelOrder - 取消订单

**用途**: 取消指定的活跃订单

#### Rust API

```rust
SpotCommand::CancelOrder {
    order_id: OrderId,
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `order_id` | `OrderId` | ✅ | 要取消的订单ID（u64） |

#### 返回值

```rust
SpotCommandResult::CancelOrder {
    order_id: OrderId,
    status: OrderStatus,  // Cancelled
}
```

#### 代码示例

```rust
let command = Command::new(generate_nonce(), SpotCommand::CancelOrder {
    order_id: 1001,
});

let response = handler.handle(command)?;
match response.result {
    SpotCommandResult::CancelOrder { order_id, status } => {
        println!("订单 {} 已取消，状态: {:?}", order_id, status);
    }
    _ => {}
}
```

#### 可能的错误

```rust
// 订单不存在
CommonError::OrderNotFound { order_id: 1001 }

// 非法状态转换（如订单已完全成交）
CommonError::InvalidStatusTransition {
    from: OrderStatus::Filled,
    to: OrderStatus::Cancelled
}
```

---

### CancelAllOrders - 批量取消

**用途**: 批量取消符合条件的所有活跃订单

**⚠️ 实现状态**: 接口已定义，等待实现

#### Rust API

```rust
SpotCommand::CancelAllOrders {
    trader: TraderId,
    symbol: Option<Symbol>,  // 可选：只取消指定交易对
    side: Option<Side>,      // 可选：只取消某一方向
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `trader` | `TraderId` | ✅ | 交易员ID |
| `symbol` | `Option<Symbol>` | ❌ | 可选：只取消指定交易对的订单 |
| `side` | `Option<Side>` | ❌ | 可选：只取消买单或卖单 |

#### 返回值

```rust
SpotCommandResult::CancelAllOrders {
    cancelled_count: usize,      // 已取消订单数量
    order_ids: Vec<OrderId>,     // 已取消的订单ID列表
}
```

#### 代码示例

##### 示例 1: 取消所有订单

```rust
let command = Command::new(generate_nonce(), SpotCommand::CancelAllOrders {
    trader: TraderId::from_str("TRADER001"),
    symbol: None,
    side: None,
});
```

##### 示例 2: 只取消 BTC/USDT 订单

```rust
let command = Command::new(generate_nonce(), SpotCommand::CancelAllOrders {
    trader: TraderId::from_str("TRADER001"),
    symbol: Some(Symbol::from_str("BTCUSDT")),
    side: None,
});
```

##### 示例 3: 只取消买单

```rust
let command = Command::new(generate_nonce(), SpotCommand::CancelAllOrders {
    trader: TraderId::from_str("TRADER001"),
    symbol: None,
    side: Some(Side::Buy),
});
```

##### 示例 4: 只取消 BTC/USDT 的买单

```rust
let command = Command::new(generate_nonce(), SpotCommand::CancelAllOrders {
    trader: TraderId::from_str("TRADER001"),
    symbol: Some(Symbol::from_str("BTCUSDT")),
    side: Some(Side::Buy),
});

let response = handler.handle(command)?;
match response.result {
    SpotCommandResult::CancelAllOrders { cancelled_count, order_ids } => {
        println!("已取消 {} 个订单", cancelled_count);
        for order_id in order_ids {
            println!("  - 订单 {}", order_id);
        }
    }
    _ => {}
}
```

#### 当前实现状态

```rust
// 暂时返回错误
CommonError::InvalidParameter {
    field: "command",
    reason: "CancelAllOrders not implemented yet"
}
```

---

## 查询接口

### QueryOpenOrders - 查询活跃订单

**用途**: 查询当前未完全成交的活跃订单

#### Rust API

```rust
OrderQueryCommand::QueryOpenOrders {
    trader: TraderId,
    symbol: Option<String>,
    side: Option<Side>,
    page: Option<u32>,
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `trader` | `TraderId` | ✅ | 交易员ID |
| `symbol` | `Option<String>` | ❌ | 可选：按交易对过滤 |
| `side` | `Option<Side>` | ❌ | 可选：按买卖方向过滤 |
| `page` | `Option<u32>` | ❌ | 可选：分页页码（默认1） |

#### 返回值

```rust
OrderQueryResult::OpenOrders {
    orders: Vec<OrderView>,
    total: usize,
    page: u32,
}

pub struct OrderView {
    pub order_id: OrderId,
    pub trader: TraderId,
    pub side: Side,
    pub price: Option<Price>,
    pub quantity: Quantity,
    pub filled_quantity: Quantity,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub created_at: u64,
}
```

#### 代码示例

```rust
let query = OrderQueryCommand::QueryOpenOrders {
    trader: TraderId::from_str("TRADER001"),
    symbol: Some("BTCUSDT".to_string()),
    side: Some(Side::Buy),
    page: Some(1),
};

let result = query_handler.handle(query)?;
match result {
    OrderQueryResult::OpenOrders { orders, total, page } => {
        println!("活跃订单列表 (第 {} 页，共 {} 个):", page, total);
        for order in orders {
            println!("  订单 {}: {} {} @ {}",
                order.order_id,
                order.side,
                order.quantity,
                order.price.unwrap_or(0)
            );
        }
    }
    _ => {}
}
```

---

### QueryOrderDetail - 查询订单详情

**用途**: 查询指定订单的详细信息和成交记录

#### Rust API

```rust
OrderQueryCommand::QueryOrderDetail {
    order_id: OrderId,
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `order_id` | `OrderId` | ✅ | 订单ID |

#### 返回值

```rust
OrderQueryResult::OrderDetail {
    order: Option<OrderDetailView>,
}

pub struct OrderDetailView {
    pub order_id: OrderId,
    pub trader: TraderId,
    pub side: Side,
    pub price: Option<Price>,
    pub quantity: Quantity,
    pub filled_quantity: Quantity,
    pub remaining_quantity: Quantity,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub created_at: u64,
    pub updated_at: u64,
    pub trades: Vec<TradeView>,  // 成交记录列表
}

pub struct TradeView {
    pub trade_id: u64,
    pub order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
    pub side: Side,
    pub timestamp: u64,
    pub is_maker: bool,  // 是否为 Maker
}
```

#### 代码示例

```rust
let query = OrderQueryCommand::QueryOrderDetail {
    order_id: 1001,
};

let result = query_handler.handle(query)?;
match result {
    OrderQueryResult::OrderDetail { order: Some(detail) } => {
        println!("订单详情:");
        println!("  订单ID: {}", detail.order_id);
        println!("  状态: {:?}", detail.status);
        println!("  已成交: {} / {}", detail.filled_quantity, detail.quantity);
        println!("  成交记录:");
        for trade in detail.trades {
            println!("    - 成交 {}: {} @ {} ({})",
                trade.trade_id,
                trade.quantity,
                trade.price,
                if trade.is_maker { "Maker" } else { "Taker" }
            );
        }
    }
    OrderQueryResult::OrderDetail { order: None } => {
        println!("订单不存在");
    }
    _ => {}
}
```

---

### QueryOrderHistory - 查询历史订单

**用途**: 查询历史订单记录（包括已完成、已取消的订单）

#### Rust API

```rust
OrderQueryCommand::QueryOrderHistory {
    trader: TraderId,
    symbol: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    page: Option<u32>,
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `trader` | `TraderId` | ✅ | 交易员ID |
| `symbol` | `Option<String>` | ❌ | 可选：按交易对过滤 |
| `start_time` | `Option<u64>` | ❌ | 可选：开始时间戳（毫秒） |
| `end_time` | `Option<u64>` | ❌ | 可选：结束时间戳（毫秒） |
| `page` | `Option<u32>` | ❌ | 可选：分页页码 |

#### 返回值

```rust
OrderQueryResult::OrderHistory {
    orders: Vec<OrderView>,
    total: usize,
    page: u32,
}
```

#### 代码示例

```rust
use std::time::{SystemTime, UNIX_EPOCH};

let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
let one_day_ago = now - 24 * 60 * 60 * 1000;

let query = OrderQueryCommand::QueryOrderHistory {
    trader: TraderId::from_str("TRADER001"),
    symbol: Some("BTCUSDT".to_string()),
    start_time: Some(one_day_ago),
    end_time: Some(now),
    page: Some(1),
};

let result = query_handler.handle(query)?;
match result {
    OrderQueryResult::OrderHistory { orders, total, page } => {
        println!("历史订单 (第 {} 页，共 {} 个):", page, total);
        for order in orders {
            println!("  订单 {}: {:?} - {} / {} 已成交",
                order.order_id,
                order.status,
                order.filled_quantity,
                order.quantity
            );
        }
    }
    _ => {}
}
```

---

### QueryTradeHistory - 查询成交记录

**用途**: 查询成交历史记录

#### Rust API

```rust
OrderQueryCommand::QueryTradeHistory {
    trader: TraderId,
    symbol: Option<String>,
    order_id: Option<OrderId>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}
```

#### 参数说明

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `trader` | `TraderId` | ✅ | 交易员ID |
| `symbol` | `Option<String>` | ❌ | 可选：按交易对过滤 |
| `order_id` | `Option<OrderId>` | ❌ | 可选：按订单ID过滤 |
| `start_time` | `Option<u64>` | ❌ | 可选：开始时间戳 |
| `end_time` | `Option<u64>` | ❌ | 可选：结束时间戳 |

#### 返回值

```rust
OrderQueryResult::TradeHistory {
    trades: Vec<TradeView>,
    total: usize,
}
```

#### 代码示例

```rust
let query = OrderQueryCommand::QueryTradeHistory {
    trader: TraderId::from_str("TRADER001"),
    symbol: Some("BTCUSDT".to_string()),
    order_id: None,
    start_time: Some(one_day_ago),
    end_time: Some(now),
};

let result = query_handler.handle(query)?;
match result {
    OrderQueryResult::TradeHistory { trades, total } => {
        println!("成交历史 (共 {} 笔):", total);
        for trade in trades {
            println!("  成交 {}: {} {} @ {} ({})",
                trade.trade_id,
                trade.side,
                trade.quantity,
                trade.price,
                if trade.is_maker { "Maker" } else { "Taker" }
            );
        }
    }
    _ => {}
}
```

---

## 数据类型

### 基础类型

```rust
/// 订单ID
pub type OrderId = u64;

/// 价格（以分为单位，避免浮点运算）
pub type Price = u32;

/// 数量
pub type Quantity = u32;

/// Nonce（幂等性标识）
pub type Nonce = u64;
```

### 复合类型

#### TraderId - 交易员ID

```rust
/// 8字节固定长度，缓存对齐
#[repr(align(8))]
pub struct TraderId([u8; 8]);

// 创建方式
let trader = TraderId::from_str("TRADER01");  // 最多8字节
let trader = TraderId::new([b'T', b'R', b'A', b'D', b'E', b'R', 0, 0]);
```

#### Symbol - 交易对符号

```rust
/// 8字节固定长度，缓存对齐
#[repr(align(8))]
pub struct Symbol([u8; 8]);

// 创建方式
let symbol = Symbol::from_str("BTCUSDT");  // 最多8字节
let symbol = Symbol::new([b'B', b'T', b'C', b'U', b'S', b'D', b'T', 0]);
```

#### Side - 买卖方向

```rust
#[repr(u8)]
pub enum Side {
    Buy = b'B',   // 买入
    Sell = b'S',  // 卖出
}

// 使用方式
let side = Side::Buy;
let opposite = side.opposite();  // Side::Sell
```

---

## 错误处理

### 错误类型层次

```rust
// 通用错误（所有命令共享）
pub enum CommonError {
    InsufficientBalance { required: u64, available: u64 },
    OrderNotFound { order_id: OrderId },
    InvalidStatusTransition { from: OrderStatus, to: OrderStatus },
    InvalidParameter { field: &'static str, reason: &'static str },
    AccountFrozen { account_id: u64 },
    TradingPairNotFound { symbol: String },
    Internal { message: String },
}

// 现货特定错误
pub enum SpotCommandError {
    Common(CommonError),
    FillOrKillRejected { order_id: OrderId, filled: Quantity, requested: Quantity },
    InvalidTimeInForce { reason: &'static str },
    PriceOutOfRange { price: Price, min: Price, max: Price },
    QuantityOutOfRange { quantity: Quantity, min: Quantity, max: Quantity },
}

// 查询错误
pub enum QueryError {
    OrderNotFound { order_id: OrderId },
    PermissionDenied { reason: &'static str },
    DatabaseError { message: String },
    InvalidParameter { field: &'static str, reason: &'static str },
    Internal { message: String },
}
```

### 错误处理示例

```rust
use lob::lob::{SpotCommandError, CommonError};

match handler.handle(command) {
    Ok(response) => {
        println!("命令执行成功");
    }
    Err(SpotCommandError::Common(CommonError::InsufficientBalance { required, available })) => {
        eprintln!("余额不足: 需要 {}, 可用 {}", required, available);
    }
    Err(SpotCommandError::FillOrKillRejected { order_id, filled, requested }) => {
        eprintln!("FOK订单 {} 被拒绝: 只成交 {}/{}", order_id, filled, requested);
    }
    Err(SpotCommandError::PriceOutOfRange { price, min, max }) => {
        eprintln!("价格 {} 超出范围 [{}, {}]", price, min, max);
    }
    Err(e) => {
        eprintln!("其他错误: {}", e);
    }
}
```

### 使用 ? 操作符

```rust
fn place_order_example() -> Result<OrderId, SpotCommandError> {
    let command = Command::new(generate_nonce(), SpotCommand::LimitOrder {
        // ... 参数
    });

    let response = handler.handle(command)?;  // 自动错误传播

    match response.result {
        SpotCommandResult::LimitOrder { order_id, .. } => Ok(order_id),
        _ => Err(SpotCommandError::Common(CommonError::Internal {
            message: "Unexpected result type".to_string(),
        })),
    }
}
```

---

## 最佳实践

### 1. 幂等性处理

```rust
// ✅ 正确：为每个命令生成唯一 nonce
fn generate_nonce() -> Nonce {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
}

let nonce = generate_nonce();
let command = Command::new(nonce, SpotCommand::LimitOrder { /* ... */ });

// 检查是否为重复命令
if response.metadata.is_duplicate {
    println!("这是一个重复命令，返回缓存结果");
}
```

### 2. 市价单必须设置价格保护

```rust
// ❌ 错误：没有价格保护，有巨大滑点风险
let command = SpotCommand::MarketOrder {
    trader: trader_id,
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    quantity: 100,
    price_limit: None,  // 危险！
    client_order_id: None,
};

// ✅ 正确：设置合理的价格保护
let current_price = get_current_market_price("BTCUSDT")?;
let max_slippage = 0.01;  // 1% 滑点容忍
let price_limit = (current_price as f64 * (1.0 + max_slippage)) as Price;

let command = SpotCommand::MarketOrder {
    trader: trader_id,
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    quantity: 100,
    price_limit: Some(price_limit),  // 安全
    client_order_id: None,
};
```

### 3. 使用 PostOnly 避免 Taker 手续费

```rust
// 做市商策略：挂单时使用 PostOnly
let command = Command::new(generate_nonce(), SpotCommand::LimitOrder {
    trader: market_maker_id,
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    price: best_bid - 1,  // 挂在最优买价下方1个单位
    quantity: 1000,
    time_in_force: TimeInForce::PostOnly,  // 确保只做 Maker
    client_order_id: Some(format!("MM-BID-{}", nonce)),
});

match handler.handle(command)?.result {
    SpotCommandResult::LimitOrder { status: OrderStatus::Rejected, .. } => {
        // PostOnly 被拒绝，说明会立即成交
        // 调整价格重新挂单
    }
    SpotCommandResult::LimitOrder { order_id, status: OrderStatus::Pending, .. } => {
        // 挂单成功
    }
    _ => {}
}
```

### 4. 客户端订单ID追踪

```rust
// 使用 client_order_id 进行业务追踪
let business_id = generate_business_order_id();

let command = Command::new(generate_nonce(), SpotCommand::LimitOrder {
    trader: trader_id,
    symbol: Symbol::from_str("BTCUSDT"),
    side: Side::Buy,
    price: 50000,
    quantity: 100,
    time_in_force: TimeInForce::GoodTillCancel,
    client_order_id: Some(business_id.clone()),  // 业务订单ID
});

// 保存映射关系
save_order_mapping(business_id, nonce)?;

// 后续可以通过 client_order_id 查询订单
```

### 5. 订单修改的正确实现

```rust
// ❌ 错误：ModifyOrder 已被移除（不是原子操作）

// ✅ 正确：通过 CancelOrder + LimitOrder 实现
async fn modify_order(
    handler: &mut impl SpotOrderHandler,
    order_id: OrderId,
    original_order: &Order,
    new_price: Price,
    new_quantity: Quantity,
) -> Result<OrderId, SpotCommandError> {
    // 1. 取消旧订单
    let cancel_cmd = Command::new(
        generate_nonce(),
        SpotCommand::CancelOrder { order_id }
    );
    handler.handle(cancel_cmd)?;

    // 2. 创建新订单
    let new_cmd = Command::new(
        generate_nonce(),
        SpotCommand::LimitOrder {
            trader: original_order.trader,
            symbol: original_order.symbol,
            side: original_order.side,
            price: new_price,
            quantity: new_quantity,
            time_in_force: original_order.time_in_force,
            client_order_id: None,
        }
    );
    let response = handler.handle(new_cmd)?;

    match response.result {
        SpotCommandResult::LimitOrder { order_id, .. } => Ok(order_id),
        _ => Err(SpotCommandError::Common(CommonError::Internal {
            message: "Unexpected result".to_string(),
        })),
    }
}
```

### 6. 错误处理和重试策略

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn place_order_with_retry(
    handler: &mut impl SpotOrderHandler,
    command: Command<SpotCommand>,
    max_retries: u32,
) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError> {
    let mut retries = 0;

    loop {
        match handler.handle(command.clone()) {
            Ok(response) => return Ok(response),
            Err(SpotCommandError::Common(CommonError::Internal { .. })) if retries < max_retries => {
                // 内部错误可以重试
                retries += 1;
                eprintln!("重试 {}/{}", retries, max_retries);
                sleep(Duration::from_millis(100 * retries as u64)).await;
            }
            Err(e) => return Err(e),  // 其他错误不重试
        }
    }
}
```

### 7. 批量操作优化

```rust
// 批量下单
async fn place_multiple_orders(
    handler: &mut impl SpotOrderHandler,
    orders: Vec<SpotCommand>,
) -> Vec<Result<OrderId, SpotCommandError>> {
    let mut results = Vec::new();

    for order in orders {
        let command = Command::new(generate_nonce(), order);
        let result = handler.handle(command)
            .and_then(|response| {
                match response.result {
                    SpotCommandResult::LimitOrder { order_id, .. } => Ok(order_id),
                    _ => Err(SpotCommandError::Common(CommonError::Internal {
                        message: "Unexpected result".to_string(),
                    })),
                }
            });
        results.push(result);
    }

    results
}
```

---

## 附录

### 完整示例程序

```rust
use lob::lob::{
    SpotCommand, SpotCommandResult, SpotOrderHandler, Command,
    Symbol, TraderId, Side, OrderStatus, TimeInForce,
    SpotCommandError, CommonError,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_nonce() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建匹配服务
    let mut matching_service = create_matching_service();

    // 1. 下限价买单
    let buy_order = Command::new(generate_nonce(), SpotCommand::LimitOrder {
        trader: TraderId::from_str("TRADER001"),
        symbol: Symbol::from_str("BTCUSDT"),
        side: Side::Buy,
        price: 50000,
        quantity: 100,
        time_in_force: TimeInForce::GoodTillCancel,
        client_order_id: Some("BUY-001".to_string()),
    });

    match matching_service.handle(buy_order) {
        Ok(response) => {
            if let SpotCommandResult::LimitOrder { order_id, status, filled_quantity, .. } = response.result {
                println!("买单创建: order_id={}, status={:?}, filled={}",
                    order_id, status, filled_quantity);

                // 2. 如果部分成交，查询订单详情
                if status == OrderStatus::PartiallyFilled {
                    // 使用 QueryOrderDetail 查询
                }

                // 3. 稍后取消订单
                if status == OrderStatus::Pending {
                    let cancel = Command::new(generate_nonce(), SpotCommand::CancelOrder {
                        order_id,
                    });
                    matching_service.handle(cancel)?;
                    println!("订单 {} 已取消", order_id);
                }
            }
        }
        Err(SpotCommandError::Common(CommonError::InsufficientBalance { required, available })) => {
            eprintln!("余额不足: 需要 {}, 可用 {}", required, available);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
        }
    }

    Ok(())
}
```

---

**文档版本**: v2.1
**基于代码**: `trading_spot_order_mng.rs` (2025-01-05)
**维护者**: Exchange Development Team
**反馈**: 请提交 Issue 或 PR
