# OrderQueryService CQRS 重构对比

## 概述

将 `OrderQueryService` 从传统的直接参数方式重构为统一的 CQRS `Query<T>` → `QueryResult<T>` 模式。

## 重构前后对比

### 1. 根据订单ID查询

#### 重构前 (mgn.rs)
```rust
fn get_order_by_id(&self, order_id: OrderId) -> Option<OrderView>;
```

**使用方式**：
```rust
let order = service.get_order_by_id(12345);
```

**问题**：
- ❌ 无法追踪查询
- ❌ 无元数据
- ❌ 不可序列化
- ❌ 难以扩展参数

#### 重构后 (order_query_service_v2.rs)
```rust
async fn get_order_by_id(
    &self,
    query: Query<GetOrderById>,
) -> Result<QueryResult<GetOrderByIdData>, CqrsError>;
```

**查询定义**：
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderById {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderByIdData {
    pub order: Option<OrderView>,
}
```

**使用方式**：
```rust
let query = Query::new(GetOrderById { order_id: 12345 });
let result = service.get_order_by_id(query).await?;

if let Some(order) = result.data.order {
    println!("Found order: {:?}", order);
    println!("Query ID: {}", query.metadata.query_id);
}
```

**优势**：
- ✅ 完整的查询追踪（ID、时间戳）
- ✅ 支持序列化和持久化
- ✅ 支持缓存策略
- ✅ 易于扩展（添加新字段不破坏接口）

---

### 2. 查询交易员的订单

#### 重构前
```rust
fn get_orders_by_trader(
    &self,
    trader_id: TraderId,
    active_only: bool
) -> Vec<OrderView>;
```

**使用方式**：
```rust
let orders = service.get_orders_by_trader(trader_id, true);
```

**问题**：
- ❌ 没有分页支持
- ❌ 添加新参数需要修改接口
- ❌ 没有总数信息
- ❌ 无法序列化查询条件

#### 重构后
```rust
async fn get_orders_by_trader(
    &self,
    query: Query<GetOrdersByTrader>,
) -> Result<QueryResult<GetOrdersByTraderData>, CqrsError>;
```

**查询定义**：
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrdersByTrader {
    pub trader_id: TraderId,
    pub active_only: bool,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl GetOrdersByTrader {
    pub fn all(trader_id: TraderId) -> Self;
    pub fn active_only(trader_id: TraderId) -> Self;
    pub fn with_pagination(self, limit: usize, offset: usize) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrdersByTraderData {
    pub orders: Vec<OrderView>,
    pub total_count: usize,
    pub has_more: bool,
}
```

**使用方式**：
```rust
// 查询活跃订单
let query = Query::new(
    GetOrdersByTrader::active_only(trader_id)
        .with_pagination(20, 0)
);

let result = service.get_orders_by_trader(query).await?;

println!("Orders: {}/{}", result.data.orders.len(), result.data.total_count);
if result.data.has_more {
    println!("More orders available");
}
```

**优势**：
- ✅ 内置分页支持
- ✅ 提供总数和是否有更多数据
- ✅ 构建器模式，易于使用
- ✅ 可扩展（未来可添加排序、过滤等）

---

### 3. 获取最优买价/卖价

#### 重构前
```rust
fn get_best_bid(&self) -> Option<(Price, Quantity)>;
fn get_best_ask(&self) -> Option<(Price, Quantity)>;
```

**使用方式**：
```rust
let best_bid = service.get_best_bid();
let best_ask = service.get_best_ask();
```

**问题**：
- ❌ 无参数查询无法追踪
- ❌ 返回元组不够语义化
- ❌ 无法缓存
- ❌ 无性能监控

#### 重构后
```rust
async fn get_best_bid(
    &self,
    query: Query<GetBestBid>,
) -> Result<QueryResult<BestBidData>, CqrsError>;

async fn get_best_ask(
    &self,
    query: Query<GetBestAsk>,
) -> Result<QueryResult<BestAskData>, CqrsError>;
```

**查询定义**：
```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBestBid {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestBidData {
    pub price: Option<Price>,
    pub quantity: Option<Quantity>,
}
```

**使用方式**：
```rust
// 创建带缓存的查询
let metadata = QueryMetadata::new().with_cache(1); // 缓存 1 秒

let query = Query::with_metadata(GetBestBid::default(), metadata);
let result = service.get_best_bid(query).await?;

if let Some(price) = result.data.price {
    println!("Best bid: {} @ {}", price, result.data.quantity.unwrap());
    println!("Query time: {}μs", result.metadata.duration_micros.unwrap());
    println!("From cache: {}", result.is_from_cache());
}
```

**优势**：
- ✅ 统一的查询接口
- ✅ 支持缓存（高频查询性能优化）
- ✅ 性能监控
- ✅ 语义化的结果类型

---

### 4. 获取市场深度

#### 重构前 (MarketDataQueryService)
```rust
fn get_market_depth(&self, levels: usize) -> MarketDepth;
```

**使用方式**：
```rust
let depth = service.get_market_depth(10);
```

**问题**：
- ❌ 简单参数，难以扩展
- ❌ 无时间戳信息
- ❌ 无查询追踪

#### 重构后
```rust
async fn get_market_depth(
    &self,
    query: Query<GetMarketDepth>,
) -> Result<QueryResult<MarketDepthData>, CqrsError>;
```

**查询定义**：
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketDepth {
    pub levels: usize,
}

impl GetMarketDepth {
    pub fn new(levels: usize) -> Self;
    pub fn level1() -> Self; // 1 层
    pub fn level2() -> Self; // 10 层
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepthData {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub timestamp: u64,
}
```

**使用方式**：
```rust
// Level 2 数据（10 层深度）
let query = Query::new(GetMarketDepth::level2());
let result = service.get_market_depth(query).await?;

println!("Market depth at {}", result.data.timestamp);
println!("Bids: {:?}", result.data.bids);
println!("Asks: {:?}", result.data.asks);
```

**优势**：
- ✅ 预定义的便利方法（level1, level2）
- ✅ 包含时间戳
- ✅ 易于扩展（未来可添加聚合方式等）

---

### 5. 获取订单统计

#### 重构前
```rust
fn get_order_statistics(&self) -> OrderStatistics;
```

**使用方式**：
```rust
let stats = service.get_order_statistics();
```

**问题**：
- ❌ 只能获取全局统计
- ❌ 无法按交易员或方向过滤
- ❌ 无法扩展

#### 重构后
```rust
async fn get_order_statistics(
    &self,
    query: Query<GetOrderStatistics>,
) -> Result<QueryResult<OrderStatisticsData>, CqrsError>;
```

**查询定义**：
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderStatistics {
    pub trader_id: Option<TraderId>,
    pub side: Option<Side>,
}

impl GetOrderStatistics {
    pub fn global() -> Self;
    pub fn by_trader(trader_id: TraderId) -> Self;
    pub fn by_side(side: Side) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatisticsData {
    pub total_orders: usize,
    pub active_orders: usize,
    pub filled_orders: usize,
    pub cancelled_orders: usize,
    pub total_volume: Quantity,
    pub by_side: Option<SideStatistics>,
}
```

**使用方式**：
```rust
// 全局统计
let query = Query::new(GetOrderStatistics::global());
let result = service.get_order_statistics(query).await?;

// 按交易员统计
let query = Query::new(GetOrderStatistics::by_trader(trader_id));
let result = service.get_order_statistics(query).await?;

// 按方向统计
let query = Query::new(GetOrderStatistics::by_side(Side::Buy));
let result = service.get_order_statistics(query).await?;

println!("Total orders: {}", result.data.total_orders);
println!("Active: {}", result.data.active_orders);
```

**优势**：
- ✅ 支持多种过滤维度
- ✅ 灵活的构建器模式
- ✅ 可扩展（未来可添加时间范围等）

---

## 完整使用示例

### 示例 1: 基本查询

```rust
use lob::lob::order_query_service_v2::*;
use lob::lob::cqrs::*;

#[tokio::main]
async fn main() -> Result<(), CqrsError> {
    let service = OrderQueryServiceImpl::new();

    // 查询订单
    let query = Query::new(GetOrderById { order_id: 12345 });
    let result = service.get_order_by_id(query).await?;

    if let Some(order) = result.data.order {
        println!("Order: {:?}", order);
    }

    Ok(())
}
```

### 示例 2: 带缓存的查询

```rust
// 高频查询启用缓存
let metadata = QueryMetadata::new()
    .with_cache(1) // 缓存 1 秒
    .with_actor("TRADER1".to_string());

let query = Query::with_metadata(GetBestBid::default(), metadata);
let result = service.get_best_bid(query).await?;

if result.is_from_cache() {
    println!("Served from cache");
} else {
    println!("Fresh data");
}
```

### 示例 3: 复杂查询

```rust
// 查询交易员的活跃订单，分页显示
let query = Query::new(
    GetOrdersByTrader::active_only(TraderId::from_str("TRADER1"))
        .with_pagination(20, 0)
);

let result = service.get_orders_by_trader(query).await?;

for order in &result.data.orders {
    println!("Order {}: {} @ {}", order.order_id, order.quantity, order.price);
}

if result.data.has_more {
    println!("Total: {}, showing {}", result.data.total_count, result.data.orders.len());
}
```

### 示例 4: 查询序列化（事件溯源）

```rust
use serde_json;

// 序列化查询（用于日志、审计）
let query = Query::new(GetOrderById { order_id: 12345 });
let json = serde_json::to_string(&query)?;

// 持久化查询
save_to_log(&json);

// 后续可以重放查询
let replayed_query: Query<GetOrderById> = serde_json::from_str(&json)?;
let result = service.get_order_by_id(replayed_query).await?;
```

### 示例 5: 性能监控

```rust
let start = std::time::Instant::now();

let query = Query::new(GetMarketDepth::level2());
let result = service.get_market_depth(query).await?;

// 结果元数据包含执行时间
if let Some(duration) = result.metadata.duration_micros {
    println!("Query executed in {}μs", duration);
}

// 检查警告
for warning in &result.metadata.warnings {
    eprintln!("Warning: {}", warning);
}
```

---

## 迁移指南

### 步骤 1: 更新依赖

```rust
use lob::lob::order_query_service_v2::*;
use lob::lob::cqrs::*;
```

### 步骤 2: 替换服务接口

```rust
// 旧代码
// let order = service.get_order_by_id(order_id);

// 新代码
let query = Query::new(GetOrderById { order_id });
let result = service.get_order_by_id(query).await?;
let order = result.data.order;
```

### 步骤 3: 实现新接口

```rust
pub struct MyOrderQueryService {
    repository: Arc<dyn OrderRepository>,
}

#[async_trait::async_trait]
impl OrderQueryServiceV2 for MyOrderQueryService {
    async fn get_order_by_id(
        &self,
        query: Query<GetOrderById>,
    ) -> Result<QueryResult<GetOrderByIdData>, CqrsError> {
        // 实现查询逻辑
        let order = self.repository
            .find_by_id(query.payload.order_id)
            .await?;

        Ok(QueryResult::success(GetOrderByIdData { order }))
    }

    // 实现其他方法...
}
```

---

## 对比总结

| 特性 | 重构前 | 重构后 |
|-----|--------|--------|
| **接口风格** | 直接参数 | `Query<T>` → `QueryResult<T>` |
| **可序列化** | ❌ | ✅ |
| **查询追踪** | ❌ | ✅ (ID + 时间戳) |
| **元数据支持** | ❌ | ✅ (执行者、关联ID) |
| **缓存支持** | ❌ | ✅ |
| **性能监控** | ❌ | ✅ (执行时间) |
| **可扩展性** | 差 | 优秀 |
| **分页支持** | 部分 | 完整 |
| **异步支持** | ❌ | ✅ |
| **类型安全** | ✅ | ✅ |
| **测试友好** | 一般 | 优秀 |

---

## 文件位置

- **原接口**: `lib/lob/src/lob/mgn.rs` (保留，向后兼容)
- **新接口**: `lib/lob/src/lob/order_query_service_v2.rs`
- **CQRS 框架**: `lib/lob/src/lob/cqrs.rs`
- **使用指南**: `lib/lob/CQRS_GUIDE.md`

---

## 测试

运行测试：
```bash
cargo test order_query_service_v2
```

所有测试通过：✅ 6 个测试
