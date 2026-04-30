# Hyperliquid Use Cases Reference

## 1. Purpose

这个文档定义：如何在本项目里为 Hyperliquid 交易域设计 **Use Cases（用例）**。

目标不是定义协议，也不是定义 HTTP API，而是定义应用层的交易意图、查询意图、输入输出、错误模型和验收标准。

---

## 2. Clean Architecture Position

整洁架构的四层模型：

1. **Entities（实体）** - 核心业务规则  
   → 在本项目中实现为：订单、价格、数量、用户、账户等领域对象

2. **Use Cases（用例）** - 应用特定的业务逻辑  
   → 在本项目中实现为：下单、撤单、改单、TWAP、查询、转账等动作编排

3. **Interface Adapters（接口适配器）** - 数据格式转换  
   → 在本项目中实现为：Hyperliquid 请求/响应映射、DTO、gateway adapter

4. **Frameworks & Drivers（框架与驱动）** - 外部工具  
   → 在本项目中实现为：HTTP client、serde、signer、runtime、配置

## 依赖关系规则 (The Dependency Rule)

**核心原则**：源码中的依赖关系必须只指向同心圆的内层，即由低层机制指向高层策略。

依赖方向：  
Framework → Adapter → Use Case → Entity  
(外层)                           (内层)

✅ 正确的依赖：
- Hyperliquid gateway adapter 依赖 Use Case 定义的 port
- Use Case 依赖 Entity
- mapper 依赖 Use Case 输入输出

❌ 错误的依赖：
- Use Case 依赖 reqwest
- Use Case 依赖 signer
- Use Case 依赖 serde JSON shape
- Entity 依赖外部 API payload

---

## 3. What Is a Use Case

Use Case 表达的是**业务意图**，不是**协议行为**。

### 正确的定义方式

| 意图 | 正确 Use Case 名称 |
|------|--------------------|
| 下一个限价单 | `PlaceOrderUseCase` |
| 撤掉一个订单 | `CancelOrderUseCase` |
| 修改挂单价格数量 | `ModifyOrderUseCase` |
| 查询用户当前挂单 | `QueryOpenOrdersUseCase` |
| 查询用户最近成交 | `QueryUserFillsUseCase` |
| 查询盘口 | `QueryL2BookUseCase` |

### 错误的定义方式

| 错误类型 | 例子 |
|----------|------|
| 按 endpoint 命名 | `ExchangeEndpointUseCase` |
| 按协议动作命名 | `PostExchangeUseCase` |
| 按技术实现命名 | `ReqwestPlaceOrderUseCase` |
| 按 JSON 行为命名 | `SerializeOrderPayloadUseCase` |

一句话判断：

> 如果名字描述的是“用户或系统想完成的业务动作”，那大概率是 Use Case；如果名字描述的是“怎么发请求/怎么编码/怎么签名”，那不是 Use Case。

---

## 4. Standard Shape of a Use Case

每个 Hyperliquid Use Case 的默认定义，应与 `lib/common/cmd_handler/src/use_case_def.rs` 对齐。

### 4.1 Command Use Case Core
- `Command`
- `GivenState`
- `Events`
- `Error`
- `LoadPort`

### 4.2 Core-External Split
- `UseCaseReplyMapper`：把 `Events` 映射成对外 `Reply`，放在核心外部
- `DomainEventPipeline`：定义 `persist` / `replay` / `publish` 顺序
- `CommandUseCaseExecutor`：执行固定编排并记录 latency metrics

### 4.3 Trait Shape

```rust
pub trait CommandUseCase: Send + Sync {
    type Command;
    type GivenState;
    type Events: DomainEventSet;
    type Error;
    type LoadPort: ?Sized + Send + Sync;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error>;
    fn load_state(
        &self,
        cmd: &Self::Command,
        load_port: &Self::LoadPort,
    ) -> Result<Self::GivenState, Self::Error>;
    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error>;
    fn then(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error>;
}

pub trait UseCaseReplyMapper<E>: Send + Sync {
    type Reply;
    fn map(&self, events: E) -> Self::Reply;
}

pub trait DomainEventPipeline<E, Err>: Send + Sync {
    fn persist(&self, events: &E) -> Result<(), Err>;
    fn replay(&self, events: &E) -> Result<(), Err>;
    fn publish(&self, events: &E) -> Result<(), Err>;
}
```

### 4.4 Meaning
- `CommandUseCase` 只定义业务输入、状态装载、业务校验、领域事件产出
- `LoadPort` 负责把命令转换成可校验的 `GivenState`
- `ReplyMapper` 属于 **Interface Adapters（接口适配器）**
- `DomainEventPipeline` 负责副作用顺序，但不泄露具体基础设施

### 4.5 Query Use Case
Query 仍然保持简单：输入是 `Query`，输出是 `View` / `Result`，不需要事件管线时不要强行套命令模型。

---

## 5. Query vs Command

### Query
特征：
- 读取状态
- 不改变业务状态
- 返回 `View` 或 `Result`

示例：
- `QueryAllMidsUseCase`
- `QueryL2BookUseCase`
- `QueryOpenOrdersUseCase`
- `QueryOrderStatusUseCase`

### Command
特征：
- 改变业务状态
- 发起交易动作
- 可能产生副作用

示例：
- `PlaceOrderUseCase`
- `CancelOrderUseCase`
- `ModifyOrderUseCase`
- `PlaceTwapOrderUseCase`
- `WithdrawUseCase`

不要把 Query 和 Command 混成一个大接口。

---

## 6. Port Design

Port 应按**能力**划分，不按**协议细节**划分。

### 推荐

```rust
#[async_trait::async_trait]
pub trait TradingPort: Send + Sync {
    async fn place_order(&self, command: PlaceOrderCommand) -> Result<PlaceOrderResult, PlaceOrderError>;
    async fn cancel_order(&self, command: CancelOrderCommand) -> Result<CancelOrderResult, CancelOrderError>;
    async fn modify_order(&self, command: ModifyOrderCommand) -> Result<ModifyOrderResult, ModifyOrderError>;
}

#[async_trait::async_trait]
pub trait MarketDataPort: Send + Sync {
    async fn query_all_mids(&self) -> Result<QueryAllMidsResult, QueryAllMidsError>;
    async fn query_l2_book(&self, query: QueryL2BookQuery) -> Result<QueryL2BookResult, QueryL2BookError>;
}

#[async_trait::async_trait]
pub trait AccountQueryPort: Send + Sync {
    async fn query_open_orders(&self, query: QueryOpenOrdersQuery) -> Result<QueryOpenOrdersResult, QueryOpenOrdersError>;
}
```

### 不推荐

```rust
pub trait HyperliquidHttpPort {
    async fn post_exchange(&self, body: serde_json::Value) -> serde_json::Value;
}
```

因为它泄露了 **Frameworks & Drivers（框架与驱动）** 的细节到 **Use Cases（用例）**。

---

## 7. Error Design

Use Case Error 应该让调用方能理解“业务上发生了什么”。

### 推荐分类
- `InvalidInput`
- `InvalidPrice`
- `InvalidSize`
- `OrderNotFound`
- `InsufficientBalance`
- `ExchangeRejected(String)`
- `TemporarilyUnavailable`

### 不推荐直接暴露
- `reqwest::Error`
- `serde_json::Error`
- HTTP status code
- 原始交易所错误包

### 推荐示例

```rust
pub enum PlaceOrderError {
    InvalidPrice,
    InvalidSize,
    InsufficientBalance,
    ExchangeRejected(String),
    TemporarilyUnavailable,
}
```

---

## 8. Hyperliquid Use Case Catalog

### Trading
- `PlaceOrderUseCase`
- `CancelOrderUseCase`
- `CancelByCloidUseCase`
- `ModifyOrderUseCase`
- `PlaceTwapOrderUseCase`
- `CancelTwapOrderUseCase`
- `ScheduleCancelUseCase`
- `UpdateLeverageUseCase`
- `UpdateIsolatedMarginUseCase`
- `UpdateIsolatedMarginByLeverageUseCase`

### Transfer
- `CoreUsdcTransferUseCase`
- `CoreSpotTransferUseCase`
- `WithdrawUseCase`
- `UsdClassTransferUseCase`
- `SendAssetUseCase`
- `SendToEvmUseCase`
- `StakingTransferUseCase`
- `DelegateUseCase`
- `VaultTransferUseCase`

### Admin / Account
- `ApproveAgentUseCase`
- `ApproveBuilderFeeUseCase`
- `SetUserAbstractionUseCase`
- `SetAgentUserAbstractionUseCase`
- `ClaimRewardsUseCase`
- `InvalidatePendingNonceUseCase`

### Query
- `QueryAllMidsUseCase`
- `QueryOpenOrdersUseCase`
- `QueryFrontendOpenOrdersUseCase`
- `QueryUserFillsUseCase`
- `QueryUserRateLimitUseCase`
- `QueryOrderStatusUseCase`
- `QueryL2BookUseCase`
- `QueryCandleSnapshotUseCase`
- `QueryVaultDetailsUseCase`
- `QueryPortfolioUseCase`

---

## 9. Functional Acceptance Tests

Use Case 功能验收测试只验证 **Use Cases（用例）** 层行为。

### 验收关注点
- 输入是否合法
- `LoadPort` 是否把命令转换成正确的 `GivenState`
- 不合法输入或不合法状态是否被拒绝
- `Events` / `ThenDomainEventSet` 是否符合业务预期
- `ReplyMapper` 是否在 Use Case 外部映射 Reply
- `DomainEventPipeline` 是否遵守调用顺序

### 不属于 Use Case 验收范围
- URL/path
- header
- JSON field name
- serde rename
- HTTP method
- reqwest request
- 签名算法
- 原始协议包格式
- 真实数据库、真实消息系统、真实外部交易所

### 推荐文件组织
- 核心抽象：单独文件，例如 `use_case_draft.rs`
- 示例实现：单独文件，例如 `use_case_draft_example.rs`
- 功能验收：跟随示例实现放在对应文件的 `#[cfg(test)]` 中

### Rust 草案分层经验

如果从通用 command handler 演进到更像 **Use Cases（用例）** 的表达，核心文件只保留稳定抽象：

- `CommandUseCase`：定义 `Command`、`GivenState`、`Events`、`Error`、`LoadPort`
- `UseCaseReplyMapper`：把 `Events` 映射成对外 `Reply`，放在核心外部
- `DomainEventPipeline`：定义 `persist`、`replay`、`publish` 的顺序能力
- `CommandUseCaseExecutor`：执行固定流程并记录 latency metrics

示例文件再放具体业务草案，例如：

- `PlaceOrderCommand`
- `PlaceOrderError`
- `PlaceOrderStateSnapshot`
- `PlaceOrderLoadPort`
- `PlaceOrderUseCase`
- fake/stub `LoadPort`
- noop/spy `DomainEventPipeline`
- Given / When / Then 风格验收测试

### `LoadPort` 规则

Use Case 需要根据外部状态判断业务规则时，不要写成：

```rust
pub struct PlaceOrderUseCase {
    repo: PgRepo,
}
```

应写成：

```rust
pub trait PlaceOrderLoadPort: Send + Sync {
    fn load_place_order_state(
        &self,
        cmd: &PlaceOrderCommand,
    ) -> Result<PlaceOrderStateSnapshot, PlaceOrderError>;
}
```

原因：
- `LoadPort` 属于 **Use Cases（用例）** 定义的抽象
- repo / HTTP / DB client 属于外层实现细节
- 功能验收可以用 fake `LoadPort` 直接构造 `GivenState`

### `ReplyMapper` 规则

`Reply` 是对外接口回复，不是 Use Case 核心的业务事实。核心 Use Case 应产出 `Events` / `Result`，再由 **Interface Adapters（接口适配器）** 映射成 HTTP/WebSocket/CLI 所需的回复结构。

### 推荐测试命名
使用 Given / When / Then 语义命名：
- `accept_place_order_when_loaded_state_allows_placing`
- `reject_place_order_when_loaded_state_disallows_placing`
- `map_reply_outside_use_case_core`
- `execute_runs_pipeline_in_order`

### 示例：Place Order

```md
Feature: PlaceOrderUseCase

Scenario: valid limit order
  Given asset is valid
  And price > 0
  And size > 0
  And TradingPort is available
  When execute PlaceOrderUseCase with a limit order command
  Then TradingPort is called once
  And a PlaceOrderResult is returned

Scenario: invalid price
  Given price <= 0
  When execute PlaceOrderUseCase
  Then PlaceOrderError::InvalidPrice is returned
  And TradingPort is not called
```

---

## 10. Rust Type Rules

### Precision
金融场景必须使用 `rust_decimal::Decimal`，不要使用 `f64`。

### Semantic Types
如果某字段有明确语义，优先使用更强的类型表达，而不是裸 `String` / `u64`。

例如：
- `ClientOrderId`
- `ExchangeOrderId`
- `Asset`
- `TriggerPrice`

### Valid States
尽量让非法状态在类型上不可表达。

---

## 11. Recommended File Layout

```text
operating/dex/src/
├── domain/
│   └── entities/
├── use_case/
│   ├── command.rs
│   ├── query.rs
│   ├── port.rs
│   ├── error.rs
│   ├── place_order.rs
│   ├── cancel_order.rs
│   ├── modify_order.rs
│   └── query_open_orders.rs
├── adapter/
│   ├── hyperliquid_command_mapper.rs
│   ├── hyperliquid_query_mapper.rs
│   └── hyperliquid_gateway.rs
└── infrastructure/
    ├── hyperliquid_http_client.rs
    ├── hyperliquid_signer.rs
    └── config.rs
```

---

## 12. Anti-Patterns

### Anti-pattern 1
Use Case 直接持有 HTTP client：

```rust
pub struct QueryOpenOrdersUseCase {
    client: reqwest::Client,
}
```

### Anti-pattern 2
Use Case 直接处理 JSON：

```rust
pub async fn execute(&self, query: QueryOpenOrdersQuery) -> serde_json::Value
```

### Anti-pattern 3
Use Case 依赖 signer：

```rust
pub struct PlaceOrderUseCase {
    signer: HyperliquidSigner,
}
```

### Anti-pattern 4
Use Case 返回原始协议字段：
- `status: "ok"`
- `response.type`
- `data.statuses`

这些都应该在 **Interface Adapters（接口适配器）** 中被转换成业务可读模型。

---

## 13. Practical Review Checklist

定义一个新的 Hyperliquid Use Case 时，逐项检查：

- [ ] 名字是否表达业务意图，而不是 endpoint / protocol
- [ ] 输入是否是 `Command` / `Query`
- [ ] 输出是否是 `Result` / `View`
- [ ] 错误是否是业务错误，而不是技术错误
- [ ] 是否只依赖 port trait，而不依赖具体实现
- [ ] 是否没有把 JSON / HTTP / signer 泄露进 Use Cases
- [ ] 是否可以用 fake port 做功能验收测试

---

## 14. Suggested Workflow

当用户说“帮我定义一个 Hyperliquid use case”时，建议按这个顺序输出：

1. 识别这是 Query 还是 Command
2. 定义 Use Case 名称
3. 定义输入 `Command/Query`
4. 定义输出 `Result/View`
5. 定义错误 `Error`
6. 定义所需 `Port`
7. 定义 `execute()` 的核心编排
8. 列出功能验收场景（Given / When / Then）

这样可以保持 **Use Cases（用例）** 的边界清晰，并与 **Interface Adapters（接口适配器）** / **Frameworks & Drivers（框架与驱动）** 严格分离。
