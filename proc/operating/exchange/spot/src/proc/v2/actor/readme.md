# 委托到行情的SEDA单体事件流架构


//todo 增加 user_data/market_data stage
## 概述

本文档详细介绍 Spot 委托订单在端到端场景中支持的多种 SEDA（Staged Event-Driven Architecture）部署架构，包括单机单线程版、单机多线程版（MPMC）和分布式版（Kafka）。所有架构共享统一的领域逻辑，通过配置即可切换部署模式，实现逻辑内聚与部署灵活的完美结合。

## 委托订单端到端场景

### 完整业务流程

委托订单从提交到完成的完整生命周期涉及以下 Stage：

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                               委托订单端到端流程                                           │
│                                                                                         │
│   ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│   │                              Push Stage (推送)                                   │   │
│   │  (订阅所有 Kafka Topics: Order/Trade/Balance/KLine ChangeLog)                    │   │
│   └───────────────────────────────────┬─────────────────────────────────────────────┘   │
│                                       │ WebSocket                                      │
│                                       ↓                                                  │
│                              ┌────────────────┐                                         │
│                              │    Clients     │                                         │
│                              └────────────────┘                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘

      客户端下单
           │
           ↓
  ┌──────────────────────┐
  │   Acquiring Stage    │
  │     (收单委托)        │
  └──────────┬───────────┘
             │
             │ OrderChangeLog + BalanceChangeLog (Kafka)
             │
             ↓
  ┌──────────────────────┐
  │    Match Stage       │
  │     (订单撮合)        │
  └──────────┬───────────┘
             │
             │ TradeChangeLog + OrderChangeLog (Kafka)
             │
             ├────────────────────────┬────────────────────────┐
             │                        │                        │
             ↓                        ↓                        ↓
  ┌──────────────────────┐  ┌──────────────────────┐  (Push 直接订阅 Kafka)
  │  Settlement Stage    │  │   KLine Stage        │
  │     (结算处理)        │  │     (K线聚合)         │
  └──────────┬───────────┘  └──────────┬───────────┘
             │                         │
             │ BalanceChangeLog        │ KLineChangeLog
             │ (Kafka)                 │ (Kafka)
             │                         │
             └─────────────────────────┘
```

### 各 Stage 职责

| Stage | 输入 | 输出 | 职责 |
|-------|------|------|------|
| **AcquiringStage** | HTTP 请求 | OrderChangeLog+BalanceChangeLog | 订单验证、入库、余额冻结 |
| **MatchStage** | OrderChangeLog | TradeChangeLog+OrderChangeLog | 订单撮合、成交生成 |
| **SettlementStage** | TradeChangeLog | BalanceChangeLog | 账户结算、余额更新 |
| **KLineStage** | TradeChangeLog | KLineChangeLog | 成交数据聚合、K线生成 |
| **PushStage** | All ChangeLogs | WebSocket Message | 实时推送所有变更到客户端 |

## 多部署架构支持

### 架构设计原则

```
┌─────────────────────────────────────────────────────────────────┐
│                    统一领域逻辑层                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  OrderBehavior · MatchBehavior · SettlementBehavior      │  │
│  │  (纯业务逻辑，无部署相关代码)                               │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ↓
              ┌───────────────┼───────────────┐
              ↓               ↓               ↓
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│   单机单线程版     │ │   单机多线程版    │ │    分布式版      │
│ (SPSC Queue)     │ │ (MPMC Queue)     │ │ (Kafka Cluster)  │
│                  │ │                  │ │                  │
│ · 无锁队列        │ │ · 内存队列        │ │ · 持久化消息      │
│ · 极致延迟        │ │ · 多线程并行      │ │ · 水平扩展        │
│ · 简单调试        │ │ · 吞吐量优先      │ │ · 高可用性        │
└──────────────────┘ └──────────────────┘ └──────────────────┘
```

### 1. 单机单线程版 (Single-Threaded)

适用于**极致延迟**要求的场景，如高频撮合核心。

```
┌──────────────────────────────────────────────────────────────────────┐
│                    Single Process (Main Thread)                       │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌────────┐ │
│  │Acquiring│ → │  Match  │ → │Settlement│ → │  KLine  │ → │  Push  │ │
│  │ Stage   │   │ Stage   │   │ Stage   │   │ Stage   │   │ Stage  │ │
│  └────┬────┘   └────┬────┘   └────┬────┘   └────┬────┘   └───┬────┘ │
│       │             │             │             │            │      │
│       └─────────────┴─────────────┴─────────────┴────────────┘      │
│                              ↓                                       │
│              ┌──────────────────────────────────────┐               │
│              │        In-Memory SPSC Queue          │               │
│              │   (无锁单生产者单消费者队列)          │               │
│              └──────────────────────────────────────┘               │
│                                                                       │
│  延迟: < 1μs (内存操作)                                               │
│  特点: 无序列化、无网络、无锁竞争                                      │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

**实现代码位置**: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/s_thread_pipeline/`

**特点**:
- ✅ **极致延迟**: 单线程无锁，内存操作 < 1μs
- ✅ **简单调试**: 顺序执行，无并发问题
- ✅ **零分配**: 可预分配所有内存，无 GC 压力
- ❌ **单点故障**: 进程崩溃则全系统不可用
- ❌ **垂直扩展受限**: 受限于单核性能

### 2. 单机多线程版 (Multi-Threaded / MPMC)

适用于**开发测试环境**和中等吞吐量场景。

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                         Single Process (Multi-Threaded)                       │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Thread 1         Thread 2          Thread 3          Thread 4         Thread 5│
│  ┌─────────┐      ┌─────────┐       ┌─────────┐       ┌─────────┐      ┌─────┐│
│  │Acquiring│─────→│  Match  │──────→│Settlement│──────→│  KLine  │─────→│Push ││
│  │ Stage   │      │ Stage   │       │ Stage   │       │ Stage   │      │Stage││
│  └────┬────┘      └────┬────┘       └────┬────┘       └────┬────┘      └──┬──┘│
│       │                │                │                │               │   │
│       └────────────────┴────────────────┴────────────────┴───────────────┘   │
│                                       ↓                                       │
│                        ┌──────────────────────────┐                          │
│                        │      MPMC Queue          │                          │
│                        │     (多生产者多消费者)    │                          │
│                        │    · crossbeam::channel  │                          │
│                        │    · 无锁并发队列         │                          │
│                        └──────────────────────────┘                          │
│                                                                               │
│  延迟: < 100μs (内存操作 + 线程切换)                                            │
│  特点: 线程级并行，共享内存，跨线程通信                                          │
│                                                                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

**实现代码位置**: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/actor/`

**特点**:
- ✅ **中等延迟**: 内存队列 + 线程切换 < 100μs
- ✅ **简单部署**: 单进程，无需外部依赖
- ✅ **易于测试**: 可在本地完整复现生产逻辑
- ⚠️ **吞吐量受限**: 单机多线程共享 CPU 资源
- ❌ **单点故障**: 进程崩溃则全系统不可用

### 3. 分布式版 (Kafka Cluster)

适用于**生产环境**和高可用场景。

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                            Kafka Cluster                                     │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │  Topics:                                                              │   │
│  │  · OrderChangeLog  (10 partitions, 3 replicas)                      │   │
│  │  · TradeChangeLog  (10 partitions, 3 replicas)                      │   │
│  │  · BalanceChangeLog (10 partitions, 3 replicas)                     │   │
│  │  · KLineChangeLog   (10 partitions, 3 replicas)                     │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────────────┘
         ↑              ↑              ↑              ↑              ↑
    ┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐
    │Acquiring│    │  Match │    │KLine   │    │Settlement│   │  Push  │
    │ Stage   │    │ Stage  │    │ Stage  │    │ Stage   │   │ Stage  │
    │(Pod 1)  │    │(Pod 2) │    │(Pod 3) │    │(Pod 4)  │   │(Pod 5) │
    └────────┘    └────────┘    └────────┘    └────────┘    └────────┘
    Process 1      Process 2      Process 3      Process 4      Process 5
                                                                         ↓
                                                              WebSocket Clients
```

**实现代码位置**: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/actor/`

**特点**:
- ✅ **高可用性**: Kafka 副本机制保证数据不丢失
- ✅ **水平扩展**: 增加消费者即可提升吞吐量
- ✅ **容错能力**: 单个 Pod 故障自动恢复，不影响整体
- ✅ **持久化**: 事件持久化到 Kafka，支持重放和审计
- ⚠️ **延迟增加**: 网络 + 序列化 + Kafka 延迟（1-10ms）
- ⚠️ **运维复杂**: 需要维护 Kafka 集群

## 架构对比

| 维度 | 单机单线程版 | 单机多线程版 | 分布式版 |
|------|-------------|-------------|----------|
| **延迟** | < 1μs | < 100μs | 1-10ms |
| **吞吐量** | 中等 | 高 | 极高 |
| **扩展性** | 垂直扩展 | 垂直扩展 | 水平扩展 |
| **可用性** | 低（单点） | 低（单点） | 高（多副本） |
| **部署复杂度** | 极低 | 低 | 中等 |
| **运维成本** | 极低 | 低 | 中等 |
| **适用场景** | 高频撮合核心 | 开发测试 | 生产环境 |

## 逻辑内聚设计

### 统一事件模型：ChangeLogEntry

所有部署架构共享统一的事件模型，确保逻辑一致性。实际源码定义于 `lib/common/diff/src/diff/diff_types.rs`：

```rust
/// 变更日志条目
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[immutable]
pub struct ChangeLogEntry {
    /// 实体唯一标识符
    entity_id: String,
    /// 实体类型名称
    entity_type: String,
    /// 变更类型
    change_type: ChangeType,
    /// 变更时间戳（纳秒）
    timestamp: u64,
    /// 变更序列号（用于排序）
    sequence: u64,
}

impl Entity for ChangeLogEntry {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.entity_id.clone()
    }

    fn entity_type() -> &'static str
    where
        Self: Sized,
    {
        "ChangeLogEntry"
    }

    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        // 比较两个变更日志的差异
        todo!()
    }

    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), EntityError> {
        // 重放变更日志
        todo!()
    }
}
```

**关键设计**：
- ✅ **序列号保证顺序**: `sequence` 字段确保事件有序处理
- ✅ **时间戳精确到纳秒**: `timestamp` 使用纳秒级时间戳
- ✅ **实体类型标识**: `entity_type` 区分 SpotOrder/SpotTrade 等不同实体
- ✅ **JSON 序列化**: 支持 Kafka 传输和事件溯源

### 架构无关的领域逻辑

核心业务逻辑完全独立于部署架构。以 `SpotTradeBehaviorV2Impl` 为例（位于 `proc/operating/exchange/spot/src/proc/v2/spot_trade_v2.rs`）：

```rust
pub struct SpotTradeBehaviorV2Impl {
    // uid路由
    balance_repo: Arc<MySqlDbRepo<Balance>>,
    // uid路由
    trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
    // uid路由
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,
    // uid路由
    user_data_repo: Arc<MySqlDbRepo<SpotOrder>>,
    // 交易对路由
    market_data_repo: Arc<MySqlDbRepo<SpotOrder>>,
    // lob_repo 可以是 EmbeddedLobRepo<SpotOrder> 或者 DistributedLobRepo<SpotOrder>
    // 交易对路由 - 动态分发
    lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
}

impl SpotTradeBehaviorV2Impl {
    /// 订单预处理 - 负责创建订单、冻结余额和生成事件
    /// 包括验证、生成订单、冻结资金等
    pub fn handle_acquiring2(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<(ChangeLogEntry, ChangeLogEntry), SpotCmdErrorAny> {
        // 1. 参数验证
        self.validate_order_cmd(&cmd)?;
        
        // 2. 创建订单实体
        let order = SpotOrder::from(cmd);
        
        // 3. 计算并冻结余额
        let balance_change = self.freeze_balance(&order)?;
        
        // 4. 生成变更日志
        let order_change_log = ChangeLogEntry::new(
            order.id().to_string(),
            "SpotOrder".to_string(),
            ChangeType::Created { fields: order.fields() },
            Timestamp::now_as_nanos(),
            self.next_sequence(),
        );
        
        Ok((balance_change, order_change_log))
    }
    
    /// 撮合处理 - 纯业务逻辑，与部署无关
    pub fn handle_match3(
        &self,
        change_log: ChangeLogEntry,
    ) -> Result<(Option<Vec<ChangeLogEntry>>, Option<Vec<ChangeLogEntry>>), SpotCmdErrorAny> {
        // 1. 获取订单簿
        let lob = self.lob_repo.get_lob(order.symbol())?;
        
        // 2. 执行撮合
        let trades = lob.match_order(order)?;
        
        // 3. 生成成交和订单变更日志
        let order_change_logs = trades.iter().map(|t| t.to_order_changelog()).collect();
        let trade_change_logs = trades.iter().map(|t| t.to_changelog()).collect();
        
        Ok((Some(order_change_logs), Some(trade_change_logs)))
    }
}
```

**关键设计**：
- ✅ **仓储抽象**: `lob_repo` 使用 `Arc<dyn MultiSymbolLobRepo>`，可切换 Embedded/Distributed 实现
- ✅ **队列抽象**: `queue` 使用 `Arc<MPMCQueue>`，底层可以是内存队列或 Kafka
- ✅ **无部署代码**: 业务逻辑完全不关心事件如何传递（内存/Kafka）
- ✅ **依赖注入**: 通过构造函数注入依赖，易于测试和切换实现

### 部署适配层

#### ActorX Trait 定义

所有 Stage 实现统一的 `ActorX` trait（位于 `lib/common/base_types/src/actor_x.rs`）：

```rust
pub trait ActorX {
    /// 启动后台事件监听任务
    ///
    /// 该方法不获取 self 所有权，而是克隆 Arc 引用在后台任务中使用。
    /// 这样可以在启动后台任务后，继续使用当前的服务实例。
    fn start(self: &Arc<Self>);
}
```

#### Stage 实现示例：SpotMatchStage

```rust
pub struct SpotMatchStage {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    kafka_config: KafkaConfig,
}

impl SpotMatchStage {
    pub fn new(trade_behavior: Arc<SpotTradeBehaviorV2Impl>, kafka_config: KafkaConfig) -> Self {
        Self { trade_behavior, kafka_config }
    }

    /// 创建并启动 SpotMatchStage（便捷方法）
    pub fn create_and_start(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
        kafka_config: KafkaConfig,
    ) -> Arc<Self> {
        let actor = Arc::new(Self::new(trade_behavior, kafka_config));
        actor.start();
        actor
    }
}

impl ActorX for SpotMatchStage {
    fn start(self: &Arc<Self>) {
        let self_clone = Arc::clone(self);
        tokio::spawn(async move {
            // 创建 Kafka 消费者
            let consumer: StreamConsumer = ClientConfig::new()
                .set("bootstrap.servers", &self_clone.kafka_config.brokers)
                .set("group.id", &self_clone.kafka_config.group_id)
                .set("auto.offset.reset", "latest")
                .create()
                .expect("Failed to create Kafka consumer");

            // 创建 Kafka 生产者（启用批量发送优化）
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &self_clone.kafka_config.brokers)
                .set("linger.ms", "5")         // 延迟5ms批量发送
                .set("batch.size", "32768")    // 批量大小32KB
                .set("compression.type", "lz4") // LZ4压缩
                .create()
                .expect("Failed to create Kafka producer");

            // 订阅 OrderChangeLog topic
            consumer.subscribe(&[SpotTopic::OrderChangeLog.name()])
                .expect("Failed to subscribe");

            // 事件循环
            loop {
                match consumer.recv().await {
                    Ok(msg) => {
                        if let Some(payload) = msg.payload() {
                            let change_log: ChangeLogEntry = 
                                serde_json::from_slice(payload).unwrap();
                            
                            // 调用业务逻辑处理
                            match self_clone.trade_behavior.handle_match3(change_log) {
                                Ok((order_logs, trade_logs)) => {
                                    // 发送结果到下游 topics
                                    Self::send_change_logs(&producer, order_logs, trade_logs).await;
                                }
                                Err(e) => tracing::error!("Match failed: {:?}", e),
                            }
                        }
                    }
                    Err(e) => tracing::error!("Kafka error: {}", e),
                }
            }
        });
    }
}
```

**关键设计**：
- ✅ **统一接口**: 所有 Stage 实现 `ActorX` trait，统一启动方式
- ✅ **业务分离**: Stage 只负责消息收发，业务逻辑委托给 `trade_behavior`
- ✅ **批量优化**: Kafka 生产者配置批量发送（linger.ms + batch.size）
- ✅ **压缩传输**: 使用 LZ4 压缩减少网络传输量

## SEDA vs 传统架构

### 传统同步架构

```
Request → Service1 → Service2 → Service3 → Response
         (阻塞等待) (阻塞等待) (阻塞等待)
         
缺点:
- ❌ 级联调用增加延迟
- ❌ 服务故障级联扩散
- ❌ 难以水平扩展
- ❌ 资源利用率低
```

### SEDA 事件驱动架构

```
Request → Stage1 → Event → Stage2 → Event → Stage3 → Response
         (异步处理)     (异步处理)     (异步处理)
         
优点:
- ✅ 异步处理，无阻塞等待
- ✅ 各 Stage 独立运行，故障隔离
- ✅ 天然支持水平扩展
- ✅ 资源利用率高
```

### 关键优势总结

| 特性 | 说明 |
|------|------|
| **去耦合** | 各 Stage 通过事件队列通信，无直接依赖 |
| **低延迟** | 异步事件处理，消除阻塞等待 |
| **省 API 调用** | 事件驱动替代同步 API 调用，降低复杂度 |
| **可扩展性** | 支持垂直和水平扩展 |
| **容错性** | 单个 Stage 故障不影响整体系统 |

## 快速开始

### 切换部署架构

通过单例模式和懒加载实现架构切换。实际代码位于 `app/axum_server/src/interfaces/common/ins_repo.rs`：

#### Stage 单例定义

```rust
// Stage 单例（Kafka 事件驱动流程）
static SPOT_MATCH_STAGE: Lazy<Arc<SpotMatchStage>> = Lazy::new(|| {
    let kafka_config = SpotKafkaConfig::default_local();
    SpotMatchStage::create_and_start(
        SPOT_TRADE_BEHAVIOR_V2_EMBEDDED.clone(),
        kafka_config,
    )
});

static SPOT_K_LINE_STAGE: Lazy<Arc<SpotKLineStage>> = Lazy::new(|| {
    let kafka_config = SpotKafkaConfig::default_local();
    SpotKLineStage::create_and_start(
        K_LINE_SERVICE.clone(),
        kafka_config,
    )
});

static SPOT_PUSH_STAGE: Lazy<Arc<SpotPushStage>> = Lazy::new(|| {
    let kafka_config = SpotKafkaConfig::default_local();
    SpotPushStage::create_and_start(
        PUSH_SERVICE.clone(),
        kafka_config,
    )
});

static SPOT_SETTLEMENT_STAGE: Lazy<Arc<SpotSettlementStage>> = Lazy::new(|| {
    let kafka_config = SpotKafkaConfig::default_local();
    SpotSettlementStage::create_and_start(
        SPOT_TRADE_BEHAVIOR_V2_EMBEDDED.clone(),
        kafka_config,
    )
});

// 队列服务单例
static MPMC_QUEUE: Lazy<Arc<MPMCQueue>> = Lazy::new(|| {
    let queue = MPMCQueue::new();
    queue.get_or_create_channel(SpotTopic::OrderChangeLog.name(), None);
    queue.get_or_create_channel(SpotTopic::TradeChangeLog.name(), None);
    queue.get_or_create_channel(SpotTopic::BalanceChangeLog.name(), None);
    queue.get_or_create_channel(SpotTopic::KLineChangeLog.name(), None);
    Arc::new(queue)
});

// Kafka 队列配置：10分区 3副本
static KAFKA_QUEUE: Lazy<Arc<KafkaQueue>> = Lazy::new(|| {
    let config = KafkaConfig::default()
        .with_num_partitions(10)
        .with_replication_factor(3);
    let queue = KafkaQueue::new_with_config(config);
    
    queue.get_or_create_channel(SpotTopic::OrderChangeLog.name(), None);
    queue.get_or_create_channel(SpotTopic::TradeChangeLog.name(), None);
    Arc::new(queue)
});
```

#### 核心服务单例

```rust
// 嵌入式 LOB（单机版）
static EMBEDDED_LOB_REPO: Lazy<Arc<EmbeddedLobRepo<SpotOrder>>> = Lazy::new(|| {
    let lobs = TradingPair::all()
        .iter()
        .map(|&symbol| LocalLob::new(symbol))
        .collect::<Vec<_>>();
    Arc::new(EmbeddedLobRepo::new(lobs))
});

// 分布式 LOB（Kafka版）
static DISTRIBUTED_LOB_REPO: Lazy<Arc<DistributedLobRepo<SpotOrder>>> = Lazy::new(|| {
    let lobs = TradingPair::all()
        .iter()
        .map(|&symbol| RemoteLob::new(symbol))
        .collect::<Vec<_>>();
    Arc::new(DistributedLobRepo::new(lobs))
});

// 业务行为单例（可切换 Embedded/Distributed）
static SPOT_TRADE_BEHAVIOR_V2_EMBEDDED: Lazy<Arc<SpotTradeBehaviorV2Impl>> = Lazy::new(|| {
    Arc::new(SpotTradeBehaviorV2Impl::new(
        BALANCE_REPO.clone(),
        TRADE_REPO.clone(),
        ORDER_REPO.clone(),
        USER_DATA_REPO.clone(),
        MARKET_DATA_REPO.clone(),
        EMBEDDED_LOB_REPO.clone(),  // 使用嵌入式 LOB
        MPMC_QUEUE.clone(),
    ))
});

static SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED: Lazy<Arc<SpotTradeBehaviorV2Impl>> = Lazy::new(|| {
    Arc::new(SpotTradeBehaviorV2Impl::new(
        BALANCE_REPO.clone(),
        TRADE_REPO.clone(),
        ORDER_REPO.clone(),
        USER_DATA_REPO.clone(),
        MARKET_DATA_REPO.clone(),
        DISTRIBUTED_LOB_REPO.clone(),  // 使用分布式 LOB
        MPMC_QUEUE.clone(),
    ))
});
```

#### 访问方法

```rust
// Stage 访问方法
pub fn get_spot_match_stage() -> Arc<SpotMatchStage> {
    SPOT_MATCH_STAGE.clone()
}

pub fn get_spot_k_line_stage() -> Arc<SpotKLineStage> {
    SPOT_K_LINE_STAGE.clone()
}

pub fn get_spot_push_stage() -> Arc<SpotPushStage> {
    SPOT_PUSH_STAGE.clone()
}

pub fn get_spot_settlement_stage() -> Arc<SpotSettlementStage> {
    SPOT_SETTLEMENT_STAGE.clone()
}

// 业务行为访问方法
pub fn get_spot_trade_behavior_v2_embedded() -> Arc<SpotTradeBehaviorV2Impl> {
    SPOT_TRADE_BEHAVIOR_V2_EMBEDDED.clone()
}

pub fn get_spot_trade_behavior_v2_distributed() -> Arc<SpotTradeBehaviorV2Impl> {
    SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED.clone()
}
```

**切换方式**：
- 修改 `SPOT_TRADE_BEHAVIOR_V2_*` 使用的 LOB 仓库类型即可切换单机/分布式
- 修改 `KafkaConfig::default_local()` 为 `KafkaConfig::new(brokers, group_id)` 可切换 Kafka 配置

### 启动 HTTP 服务器

实际启动代码位于 `app/axum_server/src/interfaces/spot/http_server.rs`：

```rust
impl HttpServer {
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // 使用 id_repo 中的单例服务
        let trade_v2_behavior = ins_repo::get_spot_trade_behavior_v2_embedded();
        let kafka_config = KafkaConfig::default_local();
        
        // 创建 AcquiringStage（收单阶段）
        let trade_v2_service = Arc::new(SpotAcquiringStage::new(
            trade_v2_behavior, 
            kafka_config
        ));
        
        // 启动 SpotAcquiringStage
        trade_v2_service.start();
        tracing::info!("✅ SpotAcquiringStage started");

        // 配置路由
        let trade_v2_routes = Router::new()
            .route(
                "/api/spot/v2/",
                post(handle_generic::<
                    SpotAcquiringStage,
                    SpotTradeCmdAny,
                    SpotTradeResAny,
                >),
            )
            .with_state(trade_v2_service);

        let http_app = Router::new()
            .route("/api/spot/health", get(Self::health_check))
            .merge(trade_v2_routes)
            .merge(market_data_routes)
            .merge(user_data_routes);

        // 启动 HTTP 服务器
        let http_listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
        tracing::info!("🚀 Spot HTTP server started at http://localhost:3001");

        tokio::spawn(async move {
            axum::serve(http_listener, http_app.into_make_service())
                .await
                .expect("Spot HTTP server failed to start");
        });

        // 启动 K 线服务
        let kline_service = ins_repo::get_k_line_service();
        kline_service.start();
        tracing::info!("✅ K-Line service started");

        // 启动 Push 服务
        let push_service = ins_repo::get_push_service();
        push_service.start();
        tracing::info!("✅ Push service started");

        // 初始化并启动所有 Stage（Kafka 事件驱动流程）
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
}
```

**启动流程**：
1. **创建 AcquiringStage**: 接收 HTTP 请求，处理下单命令
2. **启动 HTTP 服务器**: 监听 3001 端口，处理 API 请求
3. **启动基础服务**: K 线服务、Push 服务
4. **启动事件驱动 Stage**: Match、KLine、Push、Settlement 通过 Kafka 消费事件

**关键设计**：
- ✅ **懒加载**: 所有 Stage 使用 `Lazy` 懒加载，首次访问时自动启动
- ✅ **非阻塞启动**: HTTP 服务器在后台运行，不影响 Stage 初始化
- ✅ **日志追踪**: 使用 `tracing` 记录启动过程，便于排查问题

## 最佳实践

### 1. 开发环境
- 使用**单机多线程版**进行开发和单元测试
- 快速启动，无需 Kafka 等外部依赖
- 完整验证业务逻辑

### 2. 性能测试
- 使用**单机单线程版**测试极致延迟
- 使用**分布式版**测试吞吐量和可用性
- 对比不同架构的性能表现

### 3. 生产环境
- 核心业务使用**分布式版**保证高可用
- 撮合核心可使用**单机单线程版**（配合热备）
- 根据业务特点混合部署

### 4. 灰度发布
- 新功能先在**单机多线程版**验证
- 通过配置逐步切换到**分布式版**
- 支持快速回滚

## 参考

- HTTP 服务器启动: `/Users/hongyaotang/src/rustlob/app/axum_server/src/interfaces/spot/http_server.rs`
- Stage 实现: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/actor/`
- 单线程管道: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/s_thread_pipeline/`
- 架构文档: `/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/actor/ARCHITECTURE.md`

## 总结

本设计通过**统一事件模型**（ChangeLogEntry）和**架构无关的领域逻辑**，实现了委托订单端到端场景中多种 SEDA 部署架构的支持：

- **单机单线程版**: 极致延迟，适用于高频撮合核心
- **单机多线程版**: 简单部署，适用于开发和测试
- **分布式版**: 高可用、水平扩展，适用于生产环境

三种架构**逻辑内聚、部署灵活**，通过配置即可切换，无需修改业务代码，充分体现了 SEDA 架构**去耦合、低延迟、省 API 调用**的优势。
