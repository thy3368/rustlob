# 现货交易流程 (Spot Trading Process)

## 流程：订单管理 (Order Management)

基于 `handler.rs` 中的 `SpotCommand` 和 `SpotCommandResult` 定义。

### 命令接口设计

#### 1. LimitOrder - 提交限价单

**命令定义**:
```rust
SpotCommand::LimitOrder {
    trader: TraderId,
    symbol: Symbol,          // ✅ 交易对（如 BTCUSDT）
    side: Side,              // Buy | Sell
    price: Price,
    quantity: Quantity,
    time_in_force: TimeInForce,  // GTC | IOC | FOK | GTD(timestamp) | PostOnly
    client_order_id: Option<String>,  // 客户端订单ID（可选）
}
```

**触发角色**: Trader
**API端点**: `POST /api/v1/orders/limit`

**请求参数**:
```json
{
  "trader": "TRADER001",
  "symbol": "BTCUSDT",
  "side": "Buy",
  "price": 50000,
  "quantity": 100,
  "timeInForce": "GTC",  // GTC | IOC | FOK | GTD | PostOnly
  "clientOrderId": "MY-ORDER-001"
}
```

**TimeInForce 说明**:
- **GTC (Good Till Cancel)**: 撤单前一直有效，未成交部分挂单
- **IOC (Immediate Or Cancel)**: 立即成交，未成交部分自动取消
- **FOK (Fill Or Kill)**: 全部成交或全部拒绝
- **GTD (Good Till Date)**: 有效至指定时间戳
- **PostOnly**: 只做 Maker，不吃单（如果会立即成交则拒绝订单）

**成功响应** (`SpotCommandResult::LimitOrder`):
```json
{
  "metadata": {
    "nonce": 123456,
    "is_duplicate": false,
    "received_at": 1704096000000
  },
  "result": {
    "order_id": 1001,
    "status": "Pending",        // Pending | PartiallyFilled | Filled | Cancelled | Rejected
    "filled_quantity": 50,
    "remaining_quantity": 50,
    "trades": [
      {
        "matched_order_id": 2001,
        "price": 50000,
        "quantity": 50
      }
    ]
  }
}
```

**错误响应** (`SpotCommandError`):
```rust
// 通用错误
CommonError::InsufficientBalance { required: 5000000, available: 3000000 }
CommonError::InvalidParameter { field: "price", reason: "Price must be positive" }
CommonError::AccountFrozen { account_id: 1001 }
CommonError::TradingPairNotFound { symbol: "BTCUSDT" }

// 现货特定错误
SpotCommandError::PriceOutOfRange { price: 100000, min: 10000, max: 80000 }
SpotCommandError::QuantityOutOfRange { quantity: 10, min: 100, max: 10000 }
SpotCommandError::InvalidTimeInForce { reason: "GTD timestamp expired" }
SpotCommandError::FillOrKillRejected { order_id: 1001, filled: 50, requested: 100 }
```

**生成的EntityEvent**:
1. **有成交的情况**:
   - `OrderUpdateEvent` (更新匹配订单的 unfilled_quantity)
   - `TradeCreateEvent` (创建成交记录)
   - `OrderCreateEvent` (如果有剩余未成交且TimeInForce为GTC/GTD)
   - `BalanceFrozenEvent` (冻结保证金)

2. **无成交的情况**:
   - `OrderCreateEvent` (GTC/GTD/PostOnly: 创建挂单)
   - `BalanceFrozenEvent` (冻结保证金)
   - 对于IOC: 不创建订单，直接返回 Cancelled
   - 对于FOK: 不创建订单，直接返回 Rejected

3. **PostOnly 特殊处理**:
   - 如果会立即成交：拒绝订单，返回 Rejected
   - 如果不会立即成交：创建挂单，按 GTC 处理

**实体状态变化**:
- Order: Initial → Pending (GTC/GTD/PostOnly) | Filled (全部成交) | PartiallyFilled (部分成交) | Cancelled (IOC) | Rejected (FOK/PostOnly立即成交)
- Balance: Available减少, Frozen增加

---

#### 2. MarketOrder - 提交市价单

**命令定义**:
```rust
SpotCommand::MarketOrder {
    trader: TraderId,
    symbol: Symbol,
    side: Side,
    quantity: Quantity,
    price_limit: Option<Price>,  // ✅ 价格保护（买单最高价/卖单最低价）
    client_order_id: Option<String>,
}
```

**触发角色**: Trader
**API端点**: `POST /api/v1/orders/market`

**请求参数**:
```json
{
  "trader": "TRADER001",
  "symbol": "BTCUSDT",
  "side": "Buy",
  "quantity": 100,
  "priceLimit": 55000,  // 买单：最多花这个价格；卖单：最少卖这个价格
  "clientOrderId": "MY-MARKET-ORDER-001"
}
```

**价格保护机制**:
- **买单**: `price_limit` 为最高买入价，超过此价则停止吃单
- **卖单**: `price_limit` 为最低卖出价，低于此价则停止吃单
- **不设置**: 无价格保护，有巨大滑点风险（不推荐）

**示例场景**:
```
市场深度：
  卖1: 50000 @ 10
  卖2: 51000 @ 20
  卖3: 60000 @ 100  ← 深度不足，价格跳涨

买入 100 BTC 市价单：
  - 无 price_limit: 会吃完所有卖单，最后以 60000 成交 → 亏损巨大
  - price_limit = 52000: 只成交 30 BTC (50k×10 + 51k×20)，剩余 70 BTC 取消 → 保护用户
```

**成功响应** (`SpotCommandResult::MarketOrder`):
```json
{
  "metadata": {
    "nonce": 123457,
    "is_duplicate": false,
    "received_at": 1704096000000
  },
  "result": {
    "status": "Filled",  // Filled | PartiallyFilled
    "filled_quantity": 100,
    "trades": [
      {
        "matched_order_id": 2001,
        "price": 50000,
        "quantity": 100
      }
    ]
  }
}
```

**错误响应**:
```rust
CommonError::InvalidParameter { field: "command", reason: "MarketOrder not implemented yet" }
CommonError::InsufficientBalance { required, available }
SpotCommandError::PriceOutOfRange { price: 60000, min: 0, max: 55000 }  // 超过 price_limit
```

**实现状态**: ⚠️ 未实现 (返回 `InvalidParameter` 错误)

**生成的EntityEvent** (待实现):
- `MarketOrderSubmittedEvent`
- `OrderMatchedEvent`
- `TradeSettledEvent`

**实体状态变化**:
- Order: Initial → Filled | PartiallyFilled (如果 price_limit 导致部分成交)
- Balance: 即时结算

---

#### 3. CancelOrder - 取消订单

**命令定义**:
```rust
SpotCommand::CancelOrder {
    order_id: OrderId,
}
```

**触发角色**: Trader
**API端点**: `DELETE /api/v1/orders/{orderId}`

**请求参数**:
```json
{
  "order_id": 1001
}
```

**成功响应** (`SpotCommandResult::CancelOrder`):
```json
{
  "metadata": {
    "nonce": 123458,
    "is_duplicate": false,
    "received_at": 1704096000000
  },
  "result": {
    "order_id": 1001,
    "status": "Cancelled"
  }
}
```

**错误响应**:
```rust
CommonError::OrderNotFound { order_id: 1001 }
CommonError::InvalidStatusTransition { from: Filled, to: Cancelled }
```

**生成的EntityEvent**:
- `OrderCancelRequestedEvent`
- `OrderCancelledEvent`
- `BalanceUnfrozenEvent` (解冻保证金)

**实体状态变化**:
- Order: Pending/PartiallyFilled → Cancelled
- Balance: Frozen减少, Available增加

---

#### 4. CancelAllOrders - 批量取消订单

**命令定义**:
```rust
SpotCommand::CancelAllOrders {
    trader: TraderId,
    symbol: Option<Symbol>,  // ✅ 可选：只取消指定交易对
    side: Option<Side>,      // 可选：只取消某一方向
}
```

**触发角色**: Trader
**API端点**: `DELETE /api/v1/orders`

**请求参数**:
```json
{
  "trader": "TRADER001",
  "symbol": "BTCUSDT",  // 可选
  "side": "Buy"         // 可选
}
```

**使用场景**:
- 取消所有订单: `{}`
- 只取消 BTC/USDT 订单: `{ "symbol": "BTCUSDT" }`
- 只取消买单: `{ "side": "Buy" }`
- 只取消 BTC/USDT 买单: `{ "symbol": "BTCUSDT", "side": "Buy" }`

**成功响应** (`SpotCommandResult::CancelAllOrders`):
```json
{
  "metadata": {
    "nonce": 123460,
    "is_duplicate": false,
    "received_at": 1704096000000
  },
  "result": {
    "cancelled_count": 5,
    "order_ids": [1001, 1002, 1003, 1004, 1005]
  }
}
```

**错误响应**:
```rust
CommonError::InvalidParameter { field: "command", reason: "CancelAllOrders not implemented yet" }
```

**实现状态**: ⚠️ 未实现 (返回 `InvalidParameter` 错误)

**生成的EntityEvent** (待实现):
- `BatchCancelRequestedEvent`
- `OrdersCancelledEvent[]` (每个订单一个事件)

**实体状态变化**:
- 批量Order: Pending/PartiallyFilled → Cancelled

---

## 如何实现订单修改功能？

ModifyOrder 已从基础命令中移除，因为它不是原子操作，而是 **CancelOrder + LimitOrder** 的组合。

### 客户端实现方式

**方式 1: 应用层组合（推荐）**
```rust
// 应用层或客户端代码
async fn modify_order(
    handler: &mut impl SpotOrderHandler,
    order_id: OrderId,
    original_order: &Order,
    new_price: Price,
    new_quantity: Quantity,
) -> Result<OrderId, Error> {
    // 1. 取消旧订单
    handler.handle(Command::new(
        generate_nonce(),
        SpotCommand::CancelOrder { order_id }
    )).await?;

    // 2. 创建新订单
    let result = handler.handle(Command::new(
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
    )).await?;

    Ok(result.result.order_id)
}
```

**方式 2: 创建 UseCase（需要事务保证）**
```rust
// application/usecases/modify_order.rs
pub struct ModifyOrderUseCase<H: SpotOrderHandler> {
    spot_handler: H,
    order_query: Arc<dyn OrderQueryHandler>,
}

impl<H: SpotOrderHandler> ModifyOrderUseCase<H> {
    pub async fn execute(
        &mut self,
        request: ModifyOrderRequest,
    ) -> Result<ModifyOrderResponse, ModifyOrderError> {
        // 1. 查询原订单
        let original = self.order_query
            .handle(OrderQueryCommand::QueryOrderDetail {
                order_id: request.order_id
            })
            .await?;

        // 2. 事务：取消 + 创建
        // ... 实现事务逻辑和补偿
    }
}
```

**为什么移除？**
1. **不是原子操作**: 修改 = 取消 + 创建，不应该在领域层伪装成单一操作
2. **失去时间优先级**: 修改订单会丢失原有的队列位置
3. **事务边界复杂**: 取消成功但创建失败时需要补偿逻辑
4. **违反单一职责**: MatchingService 应该只处理原子命令

**真实交易所实践**:
- **币安**: 无 modify 接口，需要先 cancel 再 place
- **FIX 协议**: 有 OrderCancelReplaceRequest，但明确标注为"非原子操作"
- **NASDAQ**: Replace Order 只是 Cancel + Add 的便捷封装

---

## 流程：订单查询 (Order Query) - CQRS 读侧

基于 `handler.rs` 中的 `OrderQueryCommand` 和 `OrderQueryResult` 定义。

### 查询接口设计

#### 1. QueryOpenOrders - 查询活跃订单

**命令定义**:
```rust
OrderQueryCommand::QueryOpenOrders {
    trader: TraderId,
    symbol: Option<String>,
    side: Option<Side>,
    page: Option<u32>,
}
```

**触发角色**: Trader
**API端点**: `GET /api/v1/orders/open`

**请求参数**:
```
GET /api/v1/orders/open?trader=TRADER001&symbol=BTCUSDT&side=Buy&page=1
```

**成功响应** (`OrderQueryResult::OpenOrders`):
```json
{
  "orders": [
    {
      "order_id": 1001,
      "trader": "TRADER001",
      "side": "Buy",
      "price": 50000,
      "quantity": 100,
      "filled_quantity": 50,
      "status": "PartiallyFilled",
      "time_in_force": "GTC",
      "created_at": 1704096000000
    }
  ],
  "total": 10,
  "page": 1
}
```

**错误响应** (`QueryError`):
```rust
QueryError::PermissionDenied { reason: "Access denied" }
QueryError::InvalidParameter { field: "page", reason: "Page must be >= 1" }
QueryError::DatabaseError { message: "Connection timeout" }
```

**生成的EntityEvent**: 无 (查询类不产生事件)

---

#### 2. QueryOrderDetail - 查询订单详情

**命令定义**:
```rust
OrderQueryCommand::QueryOrderDetail {
    order_id: OrderId,
}
```

**触发角色**: Trader
**API端点**: `GET /api/v1/orders/{orderId}`

**请求参数**:
```
GET /api/v1/orders/1001
```

**成功响应** (`OrderQueryResult::OrderDetail`):
```json
{
  "order": {
    "order_id": 1001,
    "trader": "TRADER001",
    "side": "Buy",
    "price": 50000,
    "quantity": 100,
    "filled_quantity": 50,
    "remaining_quantity": 50,
    "status": "PartiallyFilled",
    "time_in_force": "GTC",
    "created_at": 1704096000000,
    "updated_at": 1704096010000,
    "trades": [
      {
        "trade_id": 5001,
        "order_id": 1001,
        "price": 50000,
        "quantity": 50,
        "side": "Buy",
        "timestamp": 1704096005000,
        "is_maker": false
      }
    ]
  }
}
```

**错误响应**:
```rust
QueryError::OrderNotFound { order_id: 1001 }
QueryError::PermissionDenied { reason: "Not your order" }
```

**生成的EntityEvent**: 无

---

#### 3. QueryOrderHistory - 查询历史订单

**命令定义**:
```rust
OrderQueryCommand::QueryOrderHistory {
    trader: TraderId,
    symbol: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    page: Option<u32>,
}
```

**触发角色**: Trader
**API端点**: `GET /api/v1/orders/history`

**请求参数**:
```
GET /api/v1/orders/history?trader=TRADER001&symbol=BTCUSDT&start_time=1704000000000&end_time=1704096000000&page=1
```

**成功响应** (`OrderQueryResult::OrderHistory`):
```json
{
  "orders": [
    {
      "order_id": 1001,
      "trader": "TRADER001",
      "side": "Buy",
      "price": 50000,
      "quantity": 100,
      "filled_quantity": 100,
      "status": "Filled",
      "time_in_force": "GTC",
      "created_at": 1704096000000
    }
  ],
  "total": 50,
  "page": 1
}
```

**错误响应**:
```rust
QueryError::InvalidParameter { field: "time_range", reason: "Invalid time range" }
QueryError::DatabaseError { message: "Query timeout" }
```

**生成的EntityEvent**: 无

---

#### 4. QueryTradeHistory - 查询成交历史

**命令定义**:
```rust
OrderQueryCommand::QueryTradeHistory {
    trader: TraderId,
    symbol: Option<String>,
    order_id: Option<OrderId>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}
```

**触发角色**: Trader
**API端点**: `GET /api/v1/trades`

**请求参数**:
```
GET /api/v1/trades?trader=TRADER001&symbol=BTCUSDT&order_id=1001&start_time=1704000000000&end_time=1704096000000
```

**成功响应** (`OrderQueryResult::TradeHistory`):
```json
{
  "trades": [
    {
      "trade_id": 5001,
      "order_id": 1001,
      "price": 50000,
      "quantity": 50,
      "side": "Buy",
      "timestamp": 1704096005000,
      "is_maker": false
    },
    {
      "trade_id": 5002,
      "order_id": 1001,
      "price": 50100,
      "quantity": 50,
      "side": "Buy",
      "timestamp": 1704096010000,
      "is_maker": true
    }
  ],
  "total": 2
}
```

**错误响应**:
```rust
QueryError::InvalidParameter { field: "time_range", reason: "Time range too large" }
QueryError::DatabaseError { message: "Database connection lost" }
```

**生成的EntityEvent**: 无

---

## 幂等性设计

所有命令通过 `Command<C>` 包装实现幂等性：

```rust
pub struct Command<C> {
    pub nonce: Nonce,           // 客户端生成的唯一标识
    pub timestamp_ms: u64,      // 命令时间戳
    pub payload: C,             // 实际命令内容
}
```

**幂等性保证**:
- 同一 `nonce` 的命令只处理一次
- 重复命令返回缓存的结果，`metadata.is_duplicate = true`

**CommandResponse 结构**:
```rust
pub struct CommandResponse<T> {
    pub metadata: CommandMetadata {
        pub nonce: Nonce,
        pub is_duplicate: bool,
        pub received_at: u64,
    },
    pub result: T,
}
```

---

## 错误处理设计

### 混合错误方案

#### CommonError (通用错误)
所有命令共享：
- `InsufficientBalance`: 余额不足
- `OrderNotFound`: 订单不存在
- `InvalidStatusTransition`: 非法状态转换
- `InvalidParameter`: 非法参数
- `AccountFrozen`: 账户冻结
- `TradingPairNotFound`: 交易对不存在
- `Internal`: 系统内部错误

#### SpotCommandError (现货特定错误)
- `Common(CommonError)`: 包含通用错误
- `FillOrKillRejected`: FOK订单拒绝
- `InvalidTimeInForce`: 非法的TimeInForce
- `PriceOutOfRange`: 价格超出范围
- `QuantityOutOfRange`: 数量超出范围

#### QueryError (查询错误)
- `OrderNotFound`: 订单不存在
- `PermissionDenied`: 权限不足
- `DatabaseError`: 数据库错误
- `InvalidParameter`: 非法参数
- `Internal`: 系统内部错误

---

## 订单状态机

```
Initial → Pending → PartiallyFilled → Filled
           ↓              ↓
        Cancelled    Cancelled
           ↓
        Rejected
           ↓
        Expired
```

**状态说明**:
- `Initial`: 订单初始状态
- `Pending`: 挂单等待成交
- `PartiallyFilled`: 部分成交
- `Filled`: 完全成交
- `Cancelling`: 取消中
- `Cancelled`: 已取消
- `Rejected`: 被拒绝 (如FOK无法全部成交、PostOnly会立即成交)
- `Expired`: 已过期 (如GTD超时)

---

## 新增功能说明 (v2.1)

### ✅ Symbol 参数
- **用途**: 指定交易对（如 BTCUSDT、ETHUSDT）
- **必要性**: 路由到正确的订单簿
- **类型**: `Symbol` (8字节固定长度，缓存对齐)

### ✅ PostOnly TimeInForce
- **用途**: 只做 Maker，不吃单
- **行为**: 如果订单会立即成交，则拒绝订单
- **适用场景**: 做市商策略，避免支付 Taker 手续费

### ✅ Price Limit (MarketOrder)
- **用途**: 市价单价格保护
- **买单**: 设置最高买入价
- **卖单**: 设置最低卖出价
- **风险**: 不设置会导致巨大滑点

### ✅ Client Order ID
- **用途**: 客户端自定义订单ID
- **场景**: 业务系统追踪、幂等性检查
- **可选**: 不影响核心功能

---

## 实现状态总结

| 命令 | 实现状态 | 备注 |
|------|---------|------|
| LimitOrder | ✅ 已实现 | 完整支持 GTC/IOC/FOK/GTD/PostOnly + symbol + client_order_id |
| MarketOrder | ❌ 未实现 | 已添加 symbol + price_limit + client_order_id，等待实现 |
| CancelOrder | ✅ 已实现 | 完整功能 |
| CancelAllOrders | ❌ 未实现 | 已添加 symbol 过滤，等待实现 |
| QueryOpenOrders | ⚠️ 接口定义 | 需实现 `OrderQueryHandler` |
| QueryOrderDetail | ⚠️ 接口定义 | 需实现 `OrderQueryHandler` |
| QueryOrderHistory | ⚠️ 接口定义 | 需实现 `OrderQueryHandler` |
| QueryTradeHistory | ⚠️ 接口定义 | 需实现 `OrderQueryHandler` |

**注意**:
- ✅ `ModifyOrder` 已被移除，不是基础命令。需要修改订单时，应在应用层组合 `CancelOrder + LimitOrder`。
- ✅ **v2.1 新增**: Symbol、PostOnly、PriceLimit、ClientOrderId 支持

---

## 交易闭环检查 ✅

| 流程 | 需要的功能 | 当前状态 |
|------|----------|---------|
| 下单 | LimitOrder + symbol | ✅ 完整 |
| 市价下单 | MarketOrder + price_limit | ⚠️ 接口完整，等待实现 |
| 取消订单 | CancelOrder | ✅ 完整 |
| 批量取消 | CancelAllOrders + symbol filter | ⚠️ 接口完整，等待实现 |
| 做市商策略 | PostOnly | ✅ 已实现 |
| 价格保护 | price_limit | ✅ 已添加 |
| 订单追踪 | client_order_id | ✅ 已添加 |
| 查询订单 | QueryOpenOrders | ⚠️ 需实现 |
| 查询成交 | QueryTradeHistory | ⚠️ 需实现 |

**结论**:
- ✅ 核心命令接口设计完整，可以闭环现货交易
- ⚠️ 部分命令需要实现
- ✅ 已解决所有严重设计缺陷（symbol、price_limit）

---

**文档版本**: v2.1
**最后更新**: 2025-01-05
**基于代码**: `handler.rs`, `matching_service.rs`, `lob_types.rs`
**重要变更**:
- 添加 Symbol 参数到所有订单命令
- 添加 PostOnly TimeInForce
- 添加 price_limit 到 MarketOrder
- 添加 client_order_id 支持
- 移除 ModifyOrder 命令
