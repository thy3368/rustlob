# CQRS 统一框架使用指南

## 概述

基于 CQRS (Command Query Responsibility Segregation) 模式的统一抽象框架，所有应用服务遵守统一的 `Command<T>`/`Query<T>`/`Result<T>` 模式。

## 核心设计

### 统一的类型系统

```rust
Command<T>        // 命令包装器：包含业务命令 + 元数据
CommandResult<T>  // 命令结果：包含结果数据 + 元数据
Query<T>          // 查询包装器：包含查询参数 + 元数据
QueryResult<T>    // 查询结果：包含查询数据 + 元数据
```

### 核心优势

1. **统一接口**：所有服务使用相同的模式，降低认知负担
2. **可序列化**：支持持久化、网络传输、事件溯源
3. **可追溯**：内置元数据（ID、时间戳、关联ID、执行者）
4. **类型安全**：编译时检查，避免运行时错误
5. **易扩展**：方便添加中间件（日志、监控、权限、缓存）
6. **测试友好**：统一接口易于 mock 和测试

## 快速开始

### 1. 定义业务命令

```rust
use lob::lob::cqrs::*;

// 业务命令负载（纯数据）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceOrder {
    pub trader_id: TraderId,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
}

// 命令结果数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceOrderData {
    pub order_id: OrderId,
    pub status: OrderStatus,
    pub matched_quantity: Quantity,
    pub remaining_quantity: Quantity,
}
```

### 2. 实现命令处理器

```rust
pub struct OrderCommandService {
    repository: Arc<dyn OrderRepository>,
    event_bus: Arc<dyn EventBus>,
}

#[async_trait::async_trait]
impl CommandHandler<PlaceOrder> for OrderCommandService {
    type Result = PlaceOrderData;

    async fn handle(
        &self,
        command: Command<PlaceOrder>,
    ) -> Result<CommandResult<Self::Result>, CqrsError> {
        // 1. 验证命令
        if command.payload.quantity == 0 {
            return Err(CqrsError::ValidationError {
                field: "quantity".to_string(),
                message: "Quantity must be greater than 0".to_string(),
            });
        }

        // 2. 执行业务逻辑
        let order_id = self.repository.allocate_order_id();

        // 3. 持久化
        self.repository.save_order(...).await?;

        // 4. 发布领域事件
        self.event_bus.publish(OrderPlacedEvent { ... }).await?;

        // 5. 返回结果
        let result = PlaceOrderData {
            order_id,
            status: OrderStatus::Pending,
            matched_quantity: 0,
            remaining_quantity: command.payload.quantity,
        };

        Ok(CommandResult::success(result))
    }
}
```

### 3. 使用命令

```rust
// 创建命令（自动生成元数据）
let cmd = Command::new(PlaceOrder {
    trader_id: "TRADER1".into(),
    side: Side::Buy,
    price: 10000,
    quantity: 100,
});

// 执行命令
let result: CommandResult<PlaceOrderData> = handler.handle(cmd).await?;

// 检查结果
if result.is_success() {
    println!("Order placed: {:?}", result.data.order_id);
    println!("Execution time: {:?}μs", result.metadata.duration_micros);
}
```

### 4. 定义查询

```rust
// 查询负载
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetOrderById {
    pub order_id: OrderId,
}

// 查询结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderData {
    pub order_id: OrderId,
    pub trader_id: TraderId,
    pub side: Side,
    pub price: Price,
    pub status: OrderStatus,
    // ...
}
```

### 5. 实现查询处理器

```rust
pub struct OrderQueryService {
    repository: Arc<dyn OrderRepository>,
}

#[async_trait::async_trait]
impl QueryHandler<GetOrderById> for OrderQueryService {
    type Result = Option<OrderData>;

    async fn handle(
        &self,
        query: Query<GetOrderById>,
    ) -> Result<QueryResult<Self::Result>, CqrsError> {
        // 从 repo 查询
        let order = self.repository
            .find_by_id(query.payload.order_id)
            .await?;

        Ok(QueryResult::success(order))
    }
}
```

### 6. 使用查询

```rust
// 创建查询
let query = Query::new(GetOrderById {
    order_id: 12345
});

// 执行查询
let result: QueryResult<Option<OrderData>> = handler.handle(query).await?;

// 使用结果
if let Some(order) = result.data {
    println!("Found order: {:?}", order);
    println!("From cache: {}", result.is_from_cache());
}
```

## 高级特性

### 元数据追踪

```rust
// 自定义命令元数据
let metadata = CommandMetadata::new()
    .with_actor("USER123".to_string())
    .with_correlation_id("TRACE-456".to_string())
    .add_attribute("source".to_string(), "mobile_app".to_string());

let cmd = Command::with_metadata(
    PlaceOrder { ... },
    metadata,
);

// 命令可追溯
println!("Command ID: {}", cmd.metadata.command_id);
println!("Actor: {:?}", cmd.metadata.actor);
println!("Correlation ID: {:?}", cmd.metadata.correlation_id);
```

### 查询缓存策略

```rust
// 启用缓存的查询
let metadata = QueryMetadata::new()
    .with_cache(300) // 缓存 300 秒
    .with_actor("USER123".to_string());

let query = Query::with_metadata(
    GetMarketDepth { levels: 10 },
    metadata,
);

let result = handler.handle(query).await?;

// 检查是否来自缓存
if result.is_from_cache() {
    println!("Served from cache");
}
```

### 性能监控

```rust
// 结果元数据包含执行时间
let result = handler.handle(cmd).await?;

if let Some(duration) = result.metadata.duration_micros {
    println!("Execution time: {}μs", duration);
}

// 检查警告
for warning in &result.metadata.warnings {
    eprintln!("Warning: {}", warning);
}
```

### 无参数查询

```rust
// 即使没有参数，也使用统一的 Query 包装
#[derive(Debug, Clone)]
pub struct GetBestBidAsk {}

let query = Query::new(GetBestBidAsk {});
let result = handler.handle(query).await?;
```

## 错误处理

```rust
// 使用统一的错误类型
match handler.handle(cmd).await {
    Ok(result) => {
        // 成功处理
        println!("Success: {:?}", result.data);
    }
    Err(CqrsError::ValidationError { field, message }) => {
        eprintln!("Validation failed on {}: {}", field, message);
    }
    Err(CqrsError::NotFound { entity, id }) => {
        eprintln!("{} not found: {}", entity, id);
    }
    Err(CqrsError::Conflict { message }) => {
        eprintln!("Conflict: {}", message);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## 序列化和持久化

```rust
// 启用 serde 特性
// Cargo.toml: lob = { version = "0.1", features = ["serde"] }

use serde_json;

// 序列化命令（用于命令日志）
let cmd = Command::new(PlaceOrder { ... });
let json = serde_json::to_string(&cmd)?;

// 反序列化
let cmd: Command<PlaceOrder> = serde_json::from_str(&json)?;

// 命令重放
let result = handler.handle(cmd).await?;
```

## 中间件扩展

### 日志中间件

```rust
pub struct LoggingMiddleware<H> {
    inner: H,
}

#[async_trait::async_trait]
impl<T, H> CommandHandler<T> for LoggingMiddleware<H>
where
    T: Send + Sync,
    H: CommandHandler<T>,
{
    type Result = H::Result;

    async fn handle(
        &self,
        command: Command<T>,
    ) -> Result<CommandResult<Self::Result>, CqrsError> {
        let start = std::time::Instant::now();

        println!("Executing command: {}", command.metadata.command_id);

        let result = self.inner.handle(command).await;

        let duration = start.elapsed();
        println!("Command completed in {:?}", duration);

        result
    }
}
```

### 性能监控中间件

```rust
pub struct MetricsMiddleware<H> {
    inner: H,
    metrics: Arc<Metrics>,
}

#[async_trait::async_trait]
impl<T, H> CommandHandler<T> for MetricsMiddleware<H>
where
    T: Send + Sync,
    H: CommandHandler<T>,
{
    type Result = H::Result;

    async fn handle(
        &self,
        command: Command<T>,
    ) -> Result<CommandResult<Self::Result>, CqrsError> {
        let start = std::time::Instant::now();

        let result = self.inner.handle(command).await;

        let duration = start.elapsed();
        self.metrics.record_command_duration(duration);

        result
    }
}
```

## 测试示例

```rust
#[tokio::test]
async fn test_place_order_command() {
    // 创建处理器
    let handler = OrderCommandService::new();

    // 创建命令
    let cmd = Command::new(PlaceOrder {
        trader_id: "TRADER1".into(),
        side: Side::Buy,
        price: 10000,
        quantity: 100,
    });

    // 执行命令
    let result = handler.handle(cmd).await.unwrap();

    // 验证结果
    assert!(result.is_success());
    assert_eq!(result.data.status, OrderStatus::Pending);
    assert_eq!(result.data.remaining_quantity, 100);
}

#[tokio::test]
async fn test_validation_error() {
    let handler = OrderCommandService::new();

    // 无效的命令（数量为0）
    let cmd = Command::new(PlaceOrder {
        trader_id: "TRADER1".into(),
        side: Side::Buy,
        price: 10000,
        quantity: 0,
    });

    // 应该返回验证错误
    let result = handler.handle(cmd).await;
    assert!(matches!(result, Err(CqrsError::ValidationError { .. })));
}
```

## 完整示例

参考 `src/lob/cqrs_example.rs` 文件，包含：

- ✅ 完整的命令定义（PlaceOrder, CancelOrder, ModifyOrder）
- ✅ 完整的查询定义（GetOrderById, GetOrdersByTrader, GetBestBidAsk, GetMarketDepth）
- ✅ 处理器实现示例
- ✅ 单元测试示例
- ✅ 元数据使用示例

## 最佳实践

### 1. 命令设计
- ✅ 命令负载只包含业务数据
- ✅ 使用描述性的命令名（动词+名词：PlaceOrder, CancelOrder）
- ✅ 保持命令的原子性
- ✅ 添加必要的验证逻辑

### 2. 查询设计
- ✅ 查询不修改系统状态
- ✅ 使用描述性的查询名（Get/Find/Search + 实体名）
- ✅ 支持分页和过滤
- ✅ 考虑缓存策略

### 3. 结果设计
- ✅ 结果数据应该是完整的（避免客户端再次查询）
- ✅ 包含必要的状态信息
- ✅ 考虑添加警告信息

### 4. 错误处理
- ✅ 使用统一的 CqrsError 类型
- ✅ 提供详细的错误信息
- ✅ 区分不同的错误类型（验证错误、业务错误、系统错误）

### 5. 性能优化
- ✅ 查询使用缓存
- ✅ 监控执行时间
- ✅ 异步处理
- ✅ 批量操作

## 架构图

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                     │
│  ┌─────────────────────┐    ┌──────────────────────┐   │
│  │  Command Handler    │    │   Query Handler      │   │
│  │  ┌───────────────┐  │    │  ┌────────────────┐  │   │
│  │  │ PlaceOrder    │  │    │  │ GetOrderById   │  │   │
│  │  │ CancelOrder   │  │    │  │ GetMarketDepth │  │   │
│  │  │ ModifyOrder   │  │    │  │ GetBestBidAsk  │  │   │
│  │  └───────────────┘  │    │  └────────────────┘  │   │
│  └─────────────────────┘    └──────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                         ▲           ▲
                         │           │
        ┌────────────────┴───────────┴────────────────┐
        │         CQRS Framework (cqrs.rs)            │
        │  ┌────────────┐          ┌───────────────┐  │
        │  │ Command<T> │          │   Query<T>    │  │
        │  │ + metadata │          │   + metadata  │  │
        │  └────────────┘          └───────────────┘  │
        │  ┌────────────────┐      ┌───────────────┐  │
        │  │CommandResult<T>│      │ QueryResult<T>│  │
        │  │   + metadata   │      │   + metadata  │  │
        │  └────────────────┘      └───────────────┘  │
        └─────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                    Domain Layer                          │
│  ┌──────────────┐  ┌─────────────┐  ┌──────────────┐   │
│  │  Repository  │  │   Service   │  │    Events    │   │
│  └──────────────┘  └─────────────┘  └──────────────┘   │
└─────────────────────────────────────────────────────────┘
```

## 总结

CQRS 统一框架提供了：

1. **一致的接口**：所有服务遵守相同的模式
2. **强大的元数据**：追踪、审计、监控一应俱全
3. **灵活的扩展**：中间件、序列化、缓存
4. **类型安全**：编译时检查，运行时安全
5. **测试友好**：统一接口易于测试

遵循 Clean Architecture 和 CQRS 原则，构建高质量、可维护、可扩展的应用服务。
