# Spot委托单的SEDA 事件驱动架构

## 概述

本文档介绍 Spot 交易系统的事件驱动架构设计，采用 **SEDA（Staged Event-Driven Architecture）** 模式，支持单机和分布式部署。

## 核心架构

### SEDA 架构优势

| 特性 | 优势 | 说明 |
|------|------|------|
| **去耦合** | 各 Stage 独立运行，互不依赖 | 易于维护、测试、扩展 |
| **低延迟** | 异步事件处理，无阻塞等待 | 高吞吐量，响应快速 |
| **省 API 调用** | 事件驱动，减少同步调用 | 降低系统复杂度 |
| **可扩展性** | 支持水平扩展 | 分布式部署时易于扩容 |
| **容错性** | 单个 Stage 故障不影响整体 | 提高系统可靠性 |

### 与传统架构对比

```
传统同步架构：
Request → Service1 → Service2 → Service3 → Response
         (阻塞)    (阻塞)    (阻塞)

SEDA 事件驱动架构：
Request → Stage1 → Event → Stage2 → Event → Stage3 → Response
         (异步)         (异步)         (异步)
```

## Order 生命周期

订单从创建到完成的完整流程涉及多个 Stage：

```
┌─────────────────────────────────────────────────────────────┐
│                    Order Lifecycle                           │
└─────────────────────────────────────────────────────────────┘

1. AcquiringStage (委托订单)
   ↓ OrderChangeLog+BalanceChangeLog
2. MatchStage (订单撮合)
   ↓ TradeChangeLog+OrderChangeLog
3. KLineStage (K线聚合)
   ↓ KLineChangeLog
4. SettlementStage (结算处理)
   ↓ BalanceChangeLog
5. PushStage (推送客户端)
   ↓ WebSocket Message
```

## 各 Stage 职责

### 1. AcquiringStage (订单获取)
- **输入**: HTTP 请求
- **处理**: 订单验证、入库
- **输出**: OrderChangeLog+BalanceChangeLog 事件
- **Kafka Topic**: OrderChangeLog+BalanceChangeLog

```rust
// 订单进入系统
POST /api/spot/v2/ → OrderChangeLog+BalanceChangeLog → Kafka
```

### 2. MatchStage (订单撮合)
- **输入**: OrderChangeLog 事件
- **处理**: 订单匹配、成交生成
- **输出**: TradeChangeLog+OrderChangeLog 事件
- **Kafka Topic**: TradeChangeLog+OrderChangeLog

```rust
// 订单撮合产生成交
OrderChangeLog → MatchStage → TradeChangeLog+OrderChangeLog → Kafka
```

### 3. KLineStage (K线聚合)
- **输入**: TradeChangeLog 事件
- **处理**: 成交数据聚合、K线生成
- **输出**: KLineChangeLog 事件
- **Kafka Topic**: KLineChangeLog

```rust
// 成交数据聚合成K线
TradeChangeLog → KLineStage → KLineChangeLog → Kafka
```

### 4. SettlementStage (结算处理)
- **输入**: TradeChangeLog 事件
- **处理**: 账户结算、余额更新
- **输出**: BalanceChangeLog 事件
- **Kafka Topic**: BalanceChangeLog

```rust
// 成交结算更新余额
TradeChangeLog → SettlementStage → BalanceChangeLog → Kafka
```

### 5. PushStage (推送客户端)
- **输入**: 所有 ChangeLog 事件
- **处理**: 事件序列化、WebSocket 推送
- **输出**: WebSocket 消息
- **订阅 Topics**: OrderChangeLog, TradeChangeLog, KLineChangeLog, BalanceChangeLog

```rust
// 实时推送所有变更到客户端
All Topics → PushStage → WebSocket → Client
```

## 统一事件模型

### ChangeLogEntry

系统使用 **ChangeLogEntry** 作为统一的事件模型，所有 Stage 间通过此事件通信。

```rust
pub struct ChangeLogEntry {
    entity_id: String,           // 实体ID (订单ID、成交ID等)
    entity_type: String,         // 实体类型 (SpotOrder, SpotTrade等)
    change_type: ChangeType,     // 变更类型 (Created, Updated, Deleted)
    timestamp: u64,              // 时间戳
    sequence: u64,               // 序列号
}

pub enum ChangeType {
    Created { fields: Vec<FieldChange> },
    Updated { changed_fields: Vec<FieldChange> },
    Deleted,
}
```

### 优势

- ✅ **统一格式**: 所有事件使用同一结构
- ✅ **易于序列化**: JSON 序列化，支持 Kafka 传输；后续支持sbe
- ✅ **完整信息**: 包含实体类型、变更类型、时间戳
- ✅ **可追溯**: 序列号支持事件顺序追踪

## 部署架构

### 1. 单机版 (MPMC Queue)

适用于开发、测试环境。

```
┌─────────────────────────────────────┐
│      Single Process                 │
├─────────────────────────────────────┤
│  AcquiringStage                     │
│  MatchStage                         │
│  KLineStage                         │
│  SettlementStage                    │
│  PushStage                          │
│                                     │
│  ↓ (MPMC Queue - In-Memory)        │
│                                     │
│  All Stages Share Same Queue        │
└─────────────────────────────────────┘
```

**特点**:
- 低延迟（内存队列）
- 简单部署
- 单点故障

### 2. 分布式版 (Kafka)

适用于生产环境。

```
┌──────────────────────────────────────────────────────────┐
│                    Kafka Cluster                         │
│  ┌─────────────────────────────────────────────────────┐ │
│  │ Topics:                                             │ │
│  │ - OrderChangeLog (10 partitions, 3 replicas)      │ │
│  │ - TradeChangeLog (10 partitions, 3 replicas)      │ │
│  │ - KLineChangeLog (10 partitions, 3 replicas)      │ │
│  │ - BalanceChangeLog (10 partitions, 3 replicas)    │ │
│  └─────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────┘
         ↑              ↑              ↑              ↑
    AcquiringStage  MatchStage   KLineStage   SettlementStage
    (Process 1)     (Process 2)  (Process 3)  (Process 4)
         ↓              ↓              ↓              ↓
    ┌──────────────────────────────────────────────────────┐
    │              PushStage (Process 5)                   │
    │         (Subscribes to All Topics)                   │
    └──────────────────────────────────────────────────────┘
         ↓
    WebSocket Clients
```

**特点**:
- 高可用性（副本机制）
- 水平扩展（多进程）
- 容错能力强
- 支持消费者组

## 快速开始

### 初始化 Stage

所有 Stage 在 `ins_repo.rs` 中初始化为单例，通过 Lazy 加载自动启动：

```rust
// ins_repo.rs
static SPOT_MATCH_STAGE: Lazy<Arc<SpotMatchStage>> = Lazy::new(|| {
    let kafka_config = SpotKafkaConfig::default_local();
    SpotMatchStage::create_and_start(
        SPOT_TRADE_BEHAVIOR_V2_EMBEDDED.clone(),
        kafka_config,
    )
});
```

### 启动 HTTP 服务器

在 `http_server.rs` 中启动所有 Stage：

```rust
// http_server.rs
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // ... HTTP 路由配置 ...

    // 初始化并启动所有 Stage
    let _match_stage = ins_repo::get_spot_match_stage();
    tracing::info!("✅ SpotMatchStage started");

    let _kline_stage = ins_repo::get_spot_k_line_stage();
    tracing::info!("✅ SpotKLineStage started");

    let _push_stage = ins_repo::get_spot_push_stage();
    tracing::info!("✅ SpotPushStage started");

    let _settlement_stage = ins_repo::get_spot_settlement_stage();
    tracing::info!("✅ SpotSettlementStage started");

    Ok(())
}
```

### 处理事件

每个 Stage 实现 `ActorX` trait，在 `start()` 方法中订阅 Kafka 事件：

```rust
impl ActorX for SpotMatchStage {
    fn start(self: &Arc<Self>) {
        let self_clone = Arc::clone(self);
        tokio::spawn(async move {
            // 创建 Kafka 消费者
            let consumer: StreamConsumer = ClientConfig::new()
                .set("bootstrap.servers", &self_clone.kafka_config.brokers)
                .set("group.id", &self_clone.kafka_config.group_id)
                .create()
                .expect("Failed to create Kafka consumer");

            // 订阅 Topic
            consumer.subscribe(&[SpotTopic::OrderChangeLog.name()])
                .expect("Failed to subscribe");

            // 事件循环
            loop {
                match consumer.recv().await {
                    Ok(msg) => {
                        if let Some(payload) = msg.payload() {
                            let change_log: ChangeLogEntry =
                                serde_json::from_slice(payload)?;

                            // 处理事件
                            self_clone.trade_behavior.handle_match3(change_log)?;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Kafka error: {}", e);
                    }
                }
            }
        });
    }
}
```

## 事件流示例

### 完整订单流程

```
1. 客户端下单
   POST /api/spot/v2/
   {
     "symbol": "BTCUSDT",
     "side": "BUY",
     "price": 50000.0,
     "quantity": 1.0
   }

2. AcquiringStage 处理
   ✓ 验证订单
   ✓ 入库
   → 发送 OrderChangeLog 事件

3. MatchStage 处理
   ✓ 订单匹配
   ✓ 生成成交
   → 发送 TradeChangeLog 事件

4. KLineStage 处理
   ✓ 聚合成交数据
   ✓ 生成 K 线
   → 发送 KLineChangeLog 事件

5. SettlementStage 处理
   ✓ 账户结算
   ✓ 更新余额
   → 发送 BalanceChangeLog 事件

6. PushStage 处理
   ✓ 序列化所有事件
   ✓ 推送到 WebSocket 客户端
   → 客户端实时收到更新
```

## 配置

### Kafka 配置

```rust
// 默认本地配置
let kafka_config = SpotKafkaConfig::default_local();
// brokers: "localhost:9092"
// group_id: "spot-match-actor-group"

// 自定义配置
let kafka_config = SpotKafkaConfig::new(
    "kafka1:9092,kafka2:9092,kafka3:9092",
    "spot-production-group"
);
```

### Topic 配置

```rust
// ins_repo.rs 中的 Kafka 队列配置
let config = KafkaConfig::default()
    .with_num_partitions(10)        // 10 个分区
    .with_replication_factor(3);    // 3 个副本
```

## 监控和调试

### 日志

系统使用 `tracing` 进行日志记录：

```
✅ SpotAcquiringStage started
✅ SpotMatchStage started
✅ SpotKLineStage started
✅ SpotPushStage started
✅ SpotSettlementStage started
```

### Kafka 监控

```bash
# 查看 Topic 列表
kafka-topics --list --bootstrap-server localhost:9092

# 查看 Topic 详情
kafka-topics --describe --topic OrderChangeLog --bootstrap-server localhost:9092

# 消费消息
kafka-console-consumer --topic OrderChangeLog --bootstrap-server localhost:9092 --from-beginning
```

## 性能优化

### 批量处理

各 Stage 支持批量处理事件以提高吞吐量：

```rust
// 单个事件处理
pub fn handle_event(&self, change_log: ChangeLogEntry) {
    self.handle_events(&[change_log]);
}

// 批量处理（性能更优）
pub fn handle_events(&self, change_logs: &[ChangeLogEntry]) {
    // 批量处理逻辑
}
```

### 异步处理

所有 Stage 使用 Tokio 异步运行时，支持高并发：

```rust
tokio::spawn(async move {
    // 后台异步处理
});
```

## 故障处理

### 单个 Stage 故障

- 其他 Stage 继续运行
- 事件在 Kafka 中持久化
- Stage 恢复后可重新消费事件

### Kafka 故障

- 使用副本机制保证数据不丢失
- 消费者自动重连
- 支持消费者组故障转移

## 最佳实践

1. **事件设计**
   - 使用 ChangeLogEntry 统一格式
   - 包含完整的实体信息
   - 支持事件溯源

2. **Stage 开发**
   - 实现 ActorX trait
   - 使用异步处理
   - 添加适当的错误处理和日志

3. **部署**
   - 开发环境使用 MPMC Queue
   - 生产环境使用 Kafka
   - 配置适当的分区数和副本数

4. **监控**
   - 监控 Kafka Topic 消费延迟
   - 记录 Stage 处理时间
   - 设置告警阈值

## 参考资源

- [SEDA 论文](http://www.eecs.harvard.edu/~mdw/papers/seda-sosp01.pdf)
- [Kafka 官方文档](https://kafka.apache.org/documentation/)
- [Tokio 异步运行时](https://tokio.rs/)

## 相关代码

- HTTP 服务器启动: `/Users/hongyaotang/src/rustlob/app/axum_server/src/interfaces/spot/http_server.rs`
- Stage 实现: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/actor/`
- 单线程管道: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/s_thread_pipeline/`
