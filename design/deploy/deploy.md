# rustlog 部署架构


## 单体部署 开发/测试环境

### 部署拓扑图

```
┌─────────────────────────────────────────────────────────────────┐
│                     开发/测试环境服务器                           │
├─────────────────────────────────────────────────────────────────┤
│ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│ │  rustlog主应用 │  │   PostgreSQL │  │   Redis缓存  │            │
│ │  (8080端口)   │  │  (5432端口)   │  │  (6379端口)   │            │
│ └──────────────┘  └──────────────┘  └──────────────┘            │
│        │                 │                 │                     │
│        └─────────────────┼─────────────────┘                     │
│                          ▼                                       │
│                  ┌──────────────┐                                │
│                  │  本地存储卷   │                                │
│                  │  (日志/配置)  │                                │
│                  └──────────────┘                                │
└─────────────────────────────────────────────────────────────────┘
```

### 中间件选型

| 组件 | 版本 | 用途 | 配置要求 |
|------|------|------|----------|
| PostgreSQL | 14+ | 主数据存储 | 内存 ≥ 4GB，存储 ≥ 50GB |
| Redis | 7.0+ | 缓存/会话存储 | 内存 ≥ 2GB，开启持久化 |
| Nginx | 1.20+ | 反向代理/负载均衡 | 可选，用于本地开发调试 |

### 部署步骤

1. **环境准备**
   - 安装 Docker 和 Docker Compose
   - 分配至少 8GB 内存和 100GB 存储

2. **启动服务**
   ```bash
   # 克隆代码
   git clone <repo-url>
   cd rustlog

   # 启动开发环境
   docker-compose -f docker-compose.dev.yml up -d

   # 初始化数据库
   docker exec -it rustlog-postgres psql -U postgres -d rustlog < init.sql
   ```

3. **验证部署**
   - 访问 `http://localhost:8080/health` 检查健康状态
   - 检查数据库连接和缓存服务

## 分布式部署 生产环境

### RTO/RPO 要求

| 指标 | 要求 | 说明 |
|------|------|------|
| **RTO** (恢复时间目标) | 0 (零停机) | 从故障到完全恢复的时间 |
| **RPO** (恢复点目标) | 0 (零数据丢失) | 故障后数据丢失的最大时间窗口 |
| **可用性** | 99.999% | 年度可用性目标 (五个九) |
### 交易对分片策略

#### 业务规模估算

| 指标 | 数值 | 说明 |
|------|------|------|
| **现货交易对** | 1000个 | 主流币+小币种交易对 |
| **衍生品交易对** | 1000个 | 合约、期权等衍生品 |
| **每交易对订单数** | 20,000个 | 平均每个交易对的挂单量 |
| **每日活跃用户** | 100万 | 每日在线交易用户数 |
| **峰值QPS** | 100万+ | 极端行情下的交易请求 |

#### 分片方案设计

为了支持高并发交易处理和水平扩展，系统采用**交易对分片**策略，将不同交易对的数据分布到不同的数据库分片上。

##### 分片键选择
- **主分片键**：`symbol` (交易对符号，如 BTCUSDT、ETHUSDT)
- **分片算法**：一致性哈希算法 (Consistent Hashing)
- **虚拟节点数**：每个物理分片对应 100 个虚拟节点

##### 分片架构图

```
┌──────────────────────────────────────────────────────────────────────┐
│                     交易对分片架构                                   │
├──────────────────────────────────────────────────────────────────────┤
│ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐│
│ │  交易路由层   │  │  分片元数据   │  │  分片映射表   │  │  健康检查器   ││
│ │  (Proxy)      │  │  (Metadata)  │  │  (Mapping)   │  │  (Health)    ││
│ └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘│
│          │              │              │              │              │
│          └──────────────┼──────────────┼──────────────┼──────────────┘│
│                         ▼              ▼              ▼                │
│          ┌──────────────────────────────────────────────────┐          │
│          │                 分片执行器                        │          │
│          ├──────────────────────────────────────────────────┤          │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │   分片1       │  │   分片2       │  │   分片3       │       │
│          │ │  (BTCUSDT等)  │  │  (ETHUSDT等)  │  │  (BNBUSDT等)  │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │   分片4       │  │   分片5       │  │   分片6       │       │
│          │ │  (ADAUSDT等)  │  │  (SOLUSDT等)  │  │  (XRPUSDT等)  │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          └──────────────────────────────────────────────────┘          │
└──────────────────────────────────────────────────────────────────────┘
```

#### 数据分布策略

##### 分片分配规则

| 分片编号 | 交易对数量 | 主要交易对示例 | 数据库实例配置 |
|----------|------------|----------------|----------------|
| 分片 1 | 20-30 | BTCUSDT、BTCUSD、BTCBUSD | 16核32GB SSD |
| 分片 2 | 20-30 | ETHUSDT、ETHUSD、ETHBUSD | 16核32GB SSD |
| 分片 3 | 20-30 | BNBUSDT、BNBUSD、BNBBUSD | 16核32GB SSD |
| 分片 4 | 50+ | ADAUSDT、SOLUSDT、XRPUSDT 等 | 16核32GB SSD |
| 分片 5 | 50+ | DOTUSDT、DOGEUSDT、SHIBUSDT 等 | 16核32GB SSD |
| 分片 6 | 50+ | 其他小币种交易对 | 16核32GB SSD |

##### 热点交易对处理

```rust
// 热点交易对识别和迁移算法
struct HotspotDetector {
    threshold: u64, // QPS 阈值
    window: Duration, // 检测窗口
    migrations: Vec<ShardMigration>,
}

impl HotspotDetector {
    fn detect_and_migrate(&self, metrics: &ShardMetrics) -> Vec<MigrationAction> {
        let mut actions = Vec::new();

        for shard in metrics.shards.iter() {
            if shard.qps > self.threshold {
                // 识别热交易对
                let hot_symbols = self.find_hot_symbols(shard);

                // 创建新分片
                let new_shard = self.create_new_shard();

                // 迁移热交易对
                for symbol in hot_symbols {
                    actions.push(MigrationAction::Migrate {
                        symbol,
                        from: shard.id,
                        to: new_shard.id,
                    });
                }
            }
        }

        actions
    }
}
```

#### 查询路由策略

##### 单交易对查询
```rust
// 单交易对查询路由
async fn get_order_book(symbol: &str) -> Result<OrderBook, Error> {
    // 1. 计算分片哈希
    let shard_id = consistent_hash(symbol, SHARD_COUNT);

    // 2. 获取分片连接
    let conn = shard_manager.get_connection(shard_id).await?;

    // 3. 执行查询
    let order_book = conn.query_order_book(symbol).await?;

    Ok(order_book)
}
```

##### 跨分片查询
```rust
// 跨分片查询协调
async fn get_user_orders(user_id: &str) -> Result<Vec<Order>, Error> {
    let mut all_orders = Vec::new();

    // 并行查询所有分片
    let mut tasks = Vec::with_capacity(SHARD_COUNT);
    for shard_id in 0..SHARD_COUNT {
        let conn = shard_manager.get_connection(shard_id).await?;
        let user_id = user_id.to_string();

        tasks.push(tokio::spawn(async move {
            conn.query_user_orders(&user_id).await
        }));
    }

    // 合并结果
    for task in tasks {
        let orders = task.await??;
        all_orders.extend(orders);
    }

    Ok(all_orders)
}
```

#### 分片迁移策略

##### 在线迁移流程

1. **准备阶段**
   ```rust
   // 创建迁移任务
   let migration = MigrationTask {
       symbol: "BTCUSDT".to_string(),
       from_shard: 1,
       to_shard: 7,
       status: MigrationStatus::Pending,
       start_time: None,
       end_time: None,
   };
   ```

2. **同步阶段**
   - 双写：同时写入源分片和目标分片
   - 增量同步：同步增量数据
   - 一致性验证：验证数据一致性

3. **切换阶段**
   - 更新路由表
   - 重定向流量到新分片
   - 验证服务可用性

4. **清理阶段**
   - 删除源分片数据
   - 释放资源

#### 运维管理

##### 监控指标

| 指标 | 说明 | 阈值 |
|------|------|------|
| **shard_qps** | 分片QPS | > 100,000 |
| **shard_cpu** | 分片CPU使用率 | > 80% |
| **shard_memory** | 分片内存使用率 | > 75% |
| **migration_lag** | 迁移延迟 | > 100ms |
| **failed_migrations** | 迁移失败次数 | > 0 |

##### 自动扩缩容

```rust
// 自动扩缩容算法
struct AutoScaler {
    min_shards: usize,
    max_shards: usize,
    cpu_threshold: f64,
    qps_threshold: u64,
}

impl AutoScaler {
    async fn scale(&self, metrics: &ClusterMetrics) -> ScaleAction {
        let avg_cpu = metrics.avg_cpu();
        let avg_qps = metrics.avg_qps();

        if avg_cpu > self.cpu_threshold || avg_qps > self.qps_threshold {
            if metrics.shard_count < self.max_shards {
                return ScaleAction::AddShard;
            }
        }

        if avg_cpu < self.cpu_threshold * 0.5 && metrics.shard_count > self.min_shards {
            return ScaleAction::RemoveShard;
        }

        ScaleAction::NoChange
    }
}
```

#### 设计优势

1. **高扩展性**：支持在线扩容，无停机时间
2. **负载均衡**：将热交易对分布到独立分片
3. **容错性**：单个分片故障不影响其他分片
4. **性能优化**：减少跨分片查询，提高查询效率
5. **运维友好**：自动化监控和故障恢复
### 部署拓扑图 - 现货下单流程水平扩展设计

#### 现货下单流程架构

```
┌──────────────────────────────────────────────────────────────────────┐
│                     现货下单流程水平扩展架构                          │
├──────────────────────────────────────────────────────────────────────┤
│ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐│
│ │  API网关层    │  │  流量分发层   │  │  请求排队层   │  │  限流熔断层   ││
│ │  (Nginx/LB)   │  │  (Proxy)      │  │  (Queue)     │  │  (RateLimit) ││
│ └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘│
│          │              │              │              │              │
│          └──────────────┼──────────────┼──────────────┼──────────────┘│
│                         ▼              ▼              ▼                │
│          ┌──────────────────────────────────────────────────┐          │
│          │                 下单服务集群                        │          │
│          ├──────────────────────────────────────────────────┤          │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  下单处理服务1 │  │  下单处理服务2 │  │  下单处理服务3 │       │
│          │ │  (OrderService) │  │  (OrderService) │  │  (OrderService) │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  下单处理服务4 │  │  下单处理服务5 │  │  下单处理服务6 │       │
│          │ │  (OrderService) │  │  (OrderService) │  │  (OrderService) │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          └──────────────────────────────────────────────────┘          │
│                 │              │              │                      │
│                 └──────────────┼──────────────┼──────────────┘          │
│                                ▼              ▼                         │
│          ┌──────────────────────────────────────────────────┐          │
│          │                 交易引擎集群                        │          │
│          ├──────────────────────────────────────────────────┤          │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  交易引擎1    │  │  交易引擎2    │  │  交易引擎3    │       │
│          │ │  (Matcher)    │  │  (Matcher)    │  │  (Matcher)    │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  交易引擎4    │  │  交易引擎5    │  │  交易引擎6    │       │
│          │ │  (Matcher)    │  │  (Matcher)    │  │  (Matcher)    │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          └──────────────────────────────────────────────────┘          │
│                 │              │              │                      │
│                 └──────────────┼──────────────┼──────────────┘          │
│                                ▼              ▼                         │
│          ┌──────────────────────────────────────────────────┐          │
│          │                 订单存储集群                        │          │
│          ├──────────────────────────────────────────────────┤          │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  订单存储1    │  │  订单存储2    │  │  订单存储3    │       │
│          │ │  (OrderStore) │  │  (OrderStore) │  │  (OrderStore) │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  订单存储4    │  │  订单存储5    │  │  订单存储6    │       │
│          │ │  (OrderStore) │  │  (OrderStore) │  │  (OrderStore) │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          └──────────────────────────────────────────────────┘          │
│                 │              │              │                      │
│                 └──────────────┼──────────────┼──────────────┘          │
│                                ▼              ▼                         │
│          ┌──────────────────────────────────────────────────┐          │
│          │                 事件处理集群                        │          │
│          ├──────────────────────────────────────────────────┤          │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  事件处理器1  │  │  事件处理器2  │  │  事件处理器3  │       │
│          │ │  (EventProc) │  │  (EventProc) │  │  (EventProc) │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│          │ │  事件处理器4  │  │  事件处理器5  │  │  事件处理器6  │       │
│          │ │  (EventProc) │  │  (EventProc) │  │  (EventProc) │       │
│          │ └──────────────┘  └──────────────┘  └──────────────┘       │
│          └──────────────────────────────────────────────────┘          │
└──────────────────────────────────────────────────────────────────────┘
```

#### 水平扩展策略

##### 1. API网关层扩展

```rust
// API网关水平扩展配置
struct ApiGatewayConfig {
    // 每台Nginx实例处理的请求数
    requests_per_instance: usize,

    // 自动扩缩容阈值
    cpu_threshold: f64,
    qps_threshold: u64,

    // 集群配置
    cluster_size: usize,
    max_cluster_size: usize,
}

impl ApiGatewayConfig {
    fn should_scale(&self, metrics: &GatewayMetrics) -> bool {
        metrics.cpu_usage > self.cpu_threshold || metrics.qps > self.qps_threshold
    }
}
```

##### 2. 下单服务层扩展

```rust
// 下单服务水平扩展策略
struct OrderServiceScaler {
    // 每台服务实例处理的下单请求数
    orders_per_instance: usize,

    // 服务响应时间阈值
    response_time_threshold: Duration,

    // 自动扩缩容配置
    min_instances: usize,
    max_instances: usize,
}

impl OrderServiceScaler {
    async fn scale(&self, metrics: &OrderServiceMetrics) -> ScaleAction {
        let avg_response_time = metrics.avg_response_time;
        let orders_per_instance = metrics.total_orders / metrics.instance_count;

        if avg_response_time > self.response_time_threshold ||
           orders_per_instance > self.orders_per_instance {
            if metrics.instance_count < self.max_instances {
                return ScaleAction::AddInstance;
            }
        }

        if orders_per_instance < self.orders_per_instance * 0.3 &&
           metrics.instance_count > self.min_instances {
            return ScaleAction::RemoveInstance;
        }

        ScaleAction::NoChange
    }
}
```

##### 3. 交易引擎层扩展

```rust
// 交易引擎分片策略
struct MatcherSharding {
    // 交易对分片映射
    symbol_shards: HashMap<String, usize>,

    // 每个交易引擎处理的交易对数量
    symbols_per_matcher: usize,

    // 负载监控
    matcher_metrics: Vec<MatcherMetrics>,
}

impl MatcherSharding {
    fn rebalance(&mut self) -> Vec<ShardMigration> {
        let mut migrations = Vec::new();

        // 检查每个交易引擎的负载
        for (matcher_id, metrics) in self.matcher_metrics.iter().enumerate() {
            if metrics.symbol_count > self.symbols_per_matcher {
                // 找到需要迁移的交易对
                let overloaded_symbols = self.find_overloaded_symbols(matcher_id);

                // 找到负载最低的交易引擎
                let target_matcher = self.find_least_loaded_matcher();

                // 创建迁移任务
                for symbol in overloaded_symbols {
                    migrations.push(ShardMigration {
                        symbol,
                        from: matcher_id,
                        to: target_matcher,
                    });
                }
            }
        }

        migrations
    }
}
```

##### 4. 订单存储层扩展

```rust
// 订单存储水平扩展配置
struct OrderStoreConfig {
    // 每个存储分片处理的订单数量
    orders_per_shard: usize,

    // 存储响应时间阈值
    response_time_threshold: Duration,

    // 分片健康检查
    shard_health_check: ShardHealthChecker,
}

impl OrderStoreConfig {
    async fn auto_scale(&self, metrics: &OrderStoreMetrics) -> Vec<ScaleAction> {
        let mut actions = Vec::new();

        for shard in metrics.shards.iter() {
            if shard.order_count > self.orders_per_shard ||
               shard.avg_response_time > self.response_time_threshold {
                actions.push(ScaleAction::SplitShard { shard_id: shard.id });
            }
        }

        actions
    }
}
```

#### 现货下单流程详细设计

##### 下单请求流程

```rust
// 现货下单请求处理流程
async fn place_spot_order(request: SpotOrderRequest) -> Result<OrderResponse, Error> {
    // 1. 请求验证和限流
    validate_order_request(&request)?;
    rate_limiter.check_rate_limit(request.user_id).await?;

    // 2. 路由到对应交易引擎
    let matcher = matcher_router.route(request.symbol.clone()).await?;

    // 3. 交易引擎处理订单
    let order = matcher.place_order(request).await?;

    // 4. 订单存储
    order_store.save(&order).await?;

    // 5. 事件发布
    event_publisher.publish(OrderPlacedEvent::from(order)).await?;

    Ok(order.into())
}
```

##### 性能优化策略

```rust
// 下单流程性能优化
struct OrderPerformanceOptimizer {
    // 请求合并配置
    request_batching: RequestBatcher,

    // 响应缓存
    response_caching: ResponseCache,

    // 异步处理
    async_processing: AsyncProcessor,
}

impl OrderPerformanceOptimizer {
    async fn optimize(&self, request: SpotOrderRequest) -> Result<OrderResponse, Error> {
        // 尝试从缓存获取相同订单的响应
        if let Some(cached) = self.response_caching.get(&request).await {
            return Ok(cached);
        }

        // 使用批量处理提高效率
        let response = self.request_batching.process(request).await?;

        // 缓存响应
        self.response_caching.set(&request, &response).await;

        Ok(response)
    }
}
```

#### 监控和告警

##### 下单流程监控指标

| 指标 | 说明 | 阈值 |
|------|------|------|
| **order_qps** | 下单QPS | > 50,000 |
| **order_response_time** | 平均响应时间 | > 100ms |
| **order_failure_rate** | 失败率 | > 1% |
| **matcher_load** | 交易引擎负载 | > 80% |
| **order_store_latency** | 订单存储延迟 | > 50ms |

##### 告警配置

```yaml
# 下单流程告警规则
- name: high_order_qps
  condition: order_qps > 50000
  duration: 30s
  severity: critical
  action: scale_order_service

- name: high_response_time
  condition: order_response_time > 100ms
  duration: 60s
  severity: warning
  action: check_matcher_load

- name: high_failure_rate
  condition: order_failure_rate > 0.01
  duration: 60s
  severity: critical
  action: investigate_errors
```

#### 水平扩展收益分析

| 扩展层级 | 单实例能力 | 扩展倍数 | 总处理能力 |
|----------|------------|----------|------------|
| API网关 | 100,000 QPS | ×5 | 500,000 QPS |
| 下单服务 | 20,000 订单/秒 | ×10 | 200,000 订单/秒 |
| 交易引擎 | 10,000 订单/秒 | ×20 | 200,000 订单/秒 |
| 订单存储 | 50,000 订单/秒 | ×10 | 500,000 订单/秒 |

通过以上水平扩展设计，系统可以支持每日处理：
- **200万+订单数**
- **100万+ QPS**
- **100万+每日活跃用户**

### 中间件选型

| 组件 | 版本 | 用途 | 配置要求 | 高可用策略 |
|------|------|------|----------|------------|
| **应用服务器** | Rust 1.70+ | 核心业务逻辑 | 8核16GB/实例 | 集群部署，负载均衡 |
| **PostgreSQL** | 14+ | 主数据存储 | 16核32GB/实例 | 主从复制，自动故障转移 |
| **Redis** | 7.0+ | 缓存/会话 | 8核16GB/实例 | 主从复制，哨兵模式 |
| **Nginx** | 1.20+ | 反向代理 | 4核8GB/实例 | 双机热备 |
| **Elasticsearch** | 8.0+ | 日志存储 | 8核16GB/实例 | 3节点集群 |
| **Kibana** | 8.0+ | 日志可视化 | 4核8GB/实例 | 双机热备 |
| **Prometheus** | 2.40+ | 监控指标 | 8核16GB/实例 | 主从部署 |
| **Grafana** | 9.0+ | 监控可视化 | 4核8GB/实例 | 双机热备 |

### 部署策略

1. **基础设施**
   - 使用 Kubernetes 集群管理容器化部署
   - 配置 HPA (水平自动扩缩容) 策略
   - 实现滚动更新和蓝绿部署

2. **数据持久化**
   - 数据库使用 SSD 存储
   - 配置定期备份策略 (每日全量 + 每小时增量)
   - 跨可用区数据同步

3. **网络架构**
   - 内部服务通过 Service 发现
   - 外部访问通过 Ingress 控制
   - 实现网络策略和访问控制

4. **监控告警**
   - 应用指标监控 (QPS、响应时间、错误率)
   - 系统资源监控 (CPU、内存、磁盘)
   - 数据库性能监控 (查询耗时、连接数)
   - 配置关键指标告警 (Slack/邮件通知)

### 灾难恢复

1. **备份策略**
   - 数据库备份到对象存储
   - 配置备份验证和恢复测试
   - 保留最近 30 天备份

2. **故障转移**
   - 数据库自动故障转移 (≤ 5分钟)
   - 应用实例自动重启和重新调度
   - 缓存数据热备和快速恢复

3. **灾难恢复演练**
   - 定期进行故障模拟测试
   - 验证 RTO 和 RPO 指标
   - 更新灾难恢复文档
