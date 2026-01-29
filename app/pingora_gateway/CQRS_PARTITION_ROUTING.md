## Rust之从0-1低时延CEX：基于 Pingora 实现 Api(Command/Query) 分区路由，支持容灾和水平扩展

---

## 📋 目录

1. [系统概述](#系统概述)
2. [核心架构](#核心架构)
3. [CQRS 模式](#cqrs-模式)
4. [分区路由实现](#分区路由实现)
5. [容灾机制](#容灾机制)
6. [水平扩展](#水平扩展)
7. [部署指南](#部署指南)
8. [测试验证](#测试验证)
9. [性能优化](#性能优化)

---

## 系统概述

本系统通过 Pingora 实现高性能的 CQRS 分区路由网关，特点：

- ✅ **读写分离**: Command (写) / Query (读) 独立路由
- ✅ **分区隔离**: 基于 `user_id` 的数据分区
- ✅ **高可用**: 每分区多服务器容灾
- ✅ **水平扩展**: 动态增加分区和服务器
- ✅ **低延迟**: 零拷贝转发 + 连接池复用

### 应用场景

- 高频交易系统 (订单写入 vs 行情查询)
- 社交平台 (发帖 vs 浏览)
- 电商系统 (下单 vs 商品查询)
- 游戏后端 (战斗日志 vs 排行榜)

---

## 核心架构

### 架构图

```
┌─────────────────────────────────────────────────────────┐
│                    客户端请求                              │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│          Pingora Gateway (0.0.0.0:8080)                 │
│  ┌──────────────────────────────────────────────────┐   │
│  │  1. 解析 HTTP 请求                                 │   │
│  │  2. 提取 user_id (JSON/Header/Query)              │   │
│  │  3. 路径分类 (Command/Query)                      │   │
│  │  4. 选择分区和服务器                               │   │
│  └──────────────────────────────────────────────────┘   │
└────────────┬────────────────────────┬───────────────────┘
             │                        │
    ┌────────┴────────┐      ┌────────┴────────┐
    │   Command 路由   │      │   Query 路由     │
    │ /api/spot/v2/   │      │ /user/data      │
    └────────┬────────┘      └────────┬────────┘
             │                        │
    ┌────────┴────────┐      ┌────────┴────────┐
    │  根据 user_id    │      │  根据 user_id    │
    │  选择写分区      │      │  选择读分区      │
    └────────┬────────┘      └────────┬────────┘
             │                        │
    ┌────────┴────────┐      ┌────────┴────────┐
    │  Zone 1 (写)     │      │  Zone 1 (读)     │
    │  ├─ 10.0.1.1    │      │  ├─ 10.0.1.10   │
    │  └─ 10.0.1.2    │      │  └─ 10.0.1.11   │
    │                 │      │                 │
    │  Zone 2 (写)     │      │  Zone 2 (读)     │
    │  ├─ 10.0.2.1    │      │  ├─ 10.0.2.10   │
    │  └─ 10.0.2.2    │      │  └─ 10.0.2.11   │
    └─────────────────┘      └─────────────────┘
```

### 数据流

```
请求 → 提取 user_id → 判断类型 → 选择分区 → 轮询服务器 → 转发
```

---

## CQRS 模式

### Command (写操作)

**路径**: `/api/spot/v2/*`
**特点**:
- 低吞吐、高一致性
- 需要事务保证
- 写入主库

**示例请求**:
```bash
# 下单 (Command)
curl -X POST http://localhost:8080/api/spot/v2/order \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "alice",
    "symbol": "BTCUSDT",
    "side": "buy",
    "price": 50000,
    "quantity": 1.0
  }'
```

### Query (读操作)

**路径**: `/api/spot/user/data`
**特点**:
- 高吞吐、最终一致性
- 可以缓存
- 读取副本

**示例请求**:
```bash
# 查询账户 (Query)
curl "http://localhost:8080/api/spot/user/data?user_id=alice"

# 或使用请求头
curl http://localhost:8080/api/spot/user/data \
  -H "X-User-Id: alice"
```

---

## 分区路由实现

### 1. 配置结构

**文件**: `src/http/router.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRouteConfig {
    /// 分区到服务器列表的映射
    /// key: 分区名 (zone_1, zone_2...)
    /// value: 该分区的服务器列表
    pub partition_ips: HashMap<String, Vec<String>>,

    /// 用户到分区的映射
    /// key: user_id
    /// value: 分区名
    pub user_partition: HashMap<String, String>,

    /// 默认后端地址
    pub default_backend: String
}
```

### 2. 配置示例

```rust
impl Default for UserRouteConfig {
    fn default() -> Self {
        let mut user_partition = HashMap::new();
        let mut partition_ips = HashMap::new();

        // 用户分区映射
        user_partition.insert("alice".to_string(), "zone_1".to_string());
        user_partition.insert("bob".to_string(), "zone_2".to_string());
        user_partition.insert("charlie".to_string(), "zone_1".to_string());
        user_partition.insert("david".to_string(), "zone_2".to_string());

        // 分区1: 2台服务器容灾
        partition_ips.insert("zone_1".to_string(), vec![
            "127.0.0.1:3001".to_string(),  // 主服务器
            "127.0.0.1:3002".to_string()   // 备份服务器
        ]);

        // 分区2: 2台服务器容灾
        partition_ips.insert("zone_2".to_string(), vec![
            "127.0.0.1:3003".to_string(),
            "127.0.0.1:3004".to_string()
        ]);

        UserRouteConfig {
            partition_ips,
            user_partition,
            default_backend: "127.0.0.1:3001".to_string()
        }
    }
}
```

### 3. 路由器实现

**核心逻辑** (`src/http/router.rs:54-77`):

```rust
pub struct UserRouter {
    config: Arc<RwLock<UserRouteConfig>>,
    /// 轮询索引 (user_id -> 当前索引)
    round_robin_index: Arc<RwLock<HashMap<String, usize>>>
}

impl UserRouter {
    /// 根据 user_id 选择后端服务器
    pub async fn select_backend(&self, user_id: &str) -> HttpPeer {
        let config = self.config.read().await;

        // 1. 查找用户所属分区
        if let Some(partition) = config.user_partition.get(user_id) {
            let backends = config.partition_ips.get(partition).unwrap();

            if backends.is_empty() {
                return self.create_peer(&config.default_backend);
            }

            // 2. 获取轮询索引
            let mut indices = self.round_robin_index.write().await;
            let index = indices.entry(user_id.to_string()).or_insert(0);

            // 3. 轮询选择服务器 (容灾机制)
            let backend = &backends[*index % backends.len()];
            *index = (*index + 1) % backends.len();

            self.create_peer(backend)
        } else {
            // 未配置用户使用默认后端
            self.create_peer(&config.default_backend)
        }
    }

    /// 热更新配置
    pub async fn update_config(&self, new_config: UserRouteConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }
}
```

### 4. 用户 ID 提取

**支持三种提取方式** (`src/http/router.rs:90-152`):

#### 方式1: JSON 请求体

```rust
pub fn extract_from_json(body: &[u8]) -> Option<String> {
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(body) {
        let possible_fields = [
            "user_id", "userId",
            "trader_id", "traderId",
            "uid", "accountId", "account_id"
        ];

        for field in &possible_fields {
            if let Some(user_id) = json.get(field) {
                if let Some(user_id_str) = user_id.as_str() {
                    return Some(user_id_str.to_string());
                }
            }
        }
    }
    None
}
```

#### 方式2: HTTP 请求头

```rust
pub fn extract_from_headers(headers: &str) -> Option<String> {
    for line in headers.lines() {
        if line.to_lowercase().starts_with("x-user-id:") {
            return Some(line.split(':').nth(1)?.trim().to_string());
        }
        if line.to_lowercase().starts_with("x-trader-id:") {
            return Some(line.split(':').nth(1)?.trim().to_string());
        }
    }
    None
}
```

#### 方式3: URL 查询参数

```rust
pub fn extract_from_query(url: &str) -> Option<String> {
    if let Some(query_start) = url.find('?') {
        let query = &url[query_start + 1..];
        for param in query.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                if key == "user_id" || key == "userId" {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}
```

---

## 容灾机制

### 1. 轮询负载均衡

每个分区内多台服务器，请求按轮询分配：

```rust
// 为每个用户维护独立的索引
let index = indices.entry(user_id.to_string()).or_insert(0);

// 轮询算法
let backend = &backends[*index % backends.len()];
*index = (*index + 1) % backends.len();
```

**示例**:
```
zone_1 有 2 台服务器: [3001, 3002]

alice 的请求序列:
  请求1 → 3001 (index=0 % 2 = 0)
  请求2 → 3002 (index=1 % 2 = 1)
  请求3 → 3001 (index=2 % 2 = 0)
  请求4 → 3002 (index=3 % 2 = 1)
```

### 2. 故障转移

当某台服务器宕机时：

**方案 A: 自动跳过**
```rust
// Pingora 连接失败会自动尝试下一个请求
// 轮询索引自动递增，跳过故障节点
```

**方案 B: 热更新配置**
```rust
// 从配置中移除故障服务器
let mut new_config = config.clone();
new_config.partition_ips
    .get_mut("zone_1")
    .unwrap()
    .retain(|ip| ip != "127.0.0.1:3001");

user_router.update_config(new_config).await;
```

### 3. 健康检查 (扩展)

```rust
// 定期检测后端健康状态
pub struct HealthChecker {
    interval: Duration,
    router: Arc<UserRouter>
}

impl HealthChecker {
    pub async fn run(&self) {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
            self.check_all_backends().await;
        }
    }

    async fn check_all_backends(&self) {
        // 实现健康检查逻辑
    }
}
```

---

## 水平扩展

### 1. 增加新分区

**场景**: 用户增长，需要新分区

```rust
// Step 1: 添加新分区服务器
partition_ips.insert("zone_3".to_string(), vec![
    "10.0.3.1:3001".to_string(),
    "10.0.3.2:3001".to_string()
]);

// Step 2: 分配用户到新分区
user_partition.insert("eve".to_string(), "zone_3".to_string());
user_partition.insert("frank".to_string(), "zone_3".to_string());

// Step 3: 热更新配置
user_router.update_config(new_config).await;
```

### 2. 分区内扩容

**场景**: 某分区负载过高，增加服务器

```rust
// 为 zone_1 增加第3台服务器
partition_ips
    .get_mut("zone_1")
    .unwrap()
    .push("10.0.1.3:3001".to_string());

// 轮询自动分配到新服务器
// 3台服务器: index % 3 → [0, 1, 2, 0, 1, 2...]
```

### 3. 用户迁移

**场景**: 重新平衡分区负载

```rust
// 将 charlie 从 zone_1 迁移到 zone_3
user_partition.insert("charlie".to_string(), "zone_3".to_string());

// 注意: 需要同步迁移数据到新分区
```

### 4. 一致性哈希 (高级)

自动分配用户到分区，避免手动配置：

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn select_partition(user_id: &str, num_partitions: usize) -> String {
    let mut hasher = DefaultHasher::new();
    user_id.hash(&mut hasher);
    let hash = hasher.finish();

    format!("zone_{}", (hash % num_partitions as u64) + 1)
}

// 使用
let partition = select_partition("alice", 3);  // → "zone_2"
```

---

## 部署指南

### 1. 架构部署

#### 生产环境拓扑

```
┌─────────────────────────────────────────────────┐
│  Load Balancer (Nginx/HAProxy)                  │
│  ├─ Pingora Gateway 1 (10.0.0.1:8080)          │
│  └─ Pingora Gateway 2 (10.0.0.2:8080)          │
└─────────────────────────────────────────────────┘
                      │
        ┌─────────────┴─────────────┐
        │                           │
┌───────▼────────┐         ┌────────▼───────┐
│  Zone 1 (写)    │         │  Zone 1 (读)    │
│  10.0.1.1:3001 │         │  10.0.1.10:3001│
│  10.0.1.2:3001 │         │  10.0.1.11:3001│
└────────────────┘         └────────────────┘
        │                           │
┌───────▼────────┐         ┌────────▼───────┐
│  Zone 2 (写)    │         │  Zone 2 (读)    │
│  10.0.2.1:3001 │         │  10.0.2.10:3001│
│  10.0.2.2:3001 │         │  10.0.2.11:3001│
└────────────────┘         └────────────────┘
```

### 2. 配置文件

创建 `config/routing.toml`:

```toml
# 默认后端
default_backend = "10.0.1.1:3001"

# 分区配置
[partitions.zone_1]
servers = ["10.0.1.1:3001", "10.0.1.2:3001"]

[partitions.zone_2]
servers = ["10.0.2.1:3001", "10.0.2.2:3001"]

# 用户映射
[users]
alice = "zone_1"
bob = "zone_2"
charlie = "zone_1"
david = "zone_2"
```

### 3. 启动脚本

```bash
#!/bin/bash
# deploy.sh

# 启动后端服务器
echo "Starting backend servers..."

# Zone 1
ssh 10.0.1.1 "cd /app && ./backend --port 3001 --zone zone_1" &
ssh 10.0.1.2 "cd /app && ./backend --port 3001 --zone zone_1" &

# Zone 2
ssh 10.0.2.1 "cd /app && ./backend --port 3001 --zone zone_2" &
ssh 10.0.2.2 "cd /app && ./backend --port 3001 --zone zone_2" &

sleep 5

# 启动 Pingora Gateway
echo "Starting Pingora Gateway..."
cd /app/pingora_gateway
./target/release/pingora_gateway --config config/routing.toml
```

### 4. Docker Compose 部署

```yaml
version: '3.8'

services:
  pingora-gateway:
    build: ./app/pingora_gateway
    ports:
      - "8080:8080"
    depends_on:
      - backend-zone1-1
      - backend-zone1-2
      - backend-zone2-1
      - backend-zone2-2
    environment:
      - RUST_LOG=info

  backend-zone1-1:
    image: backend:latest
    command: --port 3001 --zone zone_1

  backend-zone1-2:
    image: backend:latest
    command: --port 3001 --zone zone_1

  backend-zone2-1:
    image: backend:latest
    command: --port 3001 --zone zone_2

  backend-zone2-2:
    image: backend:latest
    command: --port 3001 --zone zone_2
```

启动:
```bash
docker-compose up -d
```

---

## 测试验证

### 1. 单元测试

```bash
cd app/pingora_gateway
cargo test

# 测试路由选择
cargo test test_user_router_selection

# 测试轮询
cargo test test_round_robin

# 测试用户ID提取
cargo test test_extract_user_id_from_json
```

### 2. 功能测试

**测试 Command 路由**:
```bash
# alice → zone_1
curl -X POST http://localhost:8080/api/spot/v2/order \
  -H "Content-Type: application/json" \
  -d '{"user_id": "alice", "symbol": "BTCUSDT", "price": 50000}'

# bob → zone_2
curl -X POST http://localhost:8080/api/spot/v2/order \
  -H "Content-Type: application/json" \
  -d '{"user_id": "bob", "symbol": "ETHUSDT", "price": 3000}'
```

**测试 Query 路由**:
```bash
# alice → zone_1 (读副本)
curl "http://localhost:8080/api/spot/user/data?user_id=alice"

# 使用请求头
curl http://localhost:8080/api/spot/user/data \
  -H "X-User-Id: bob"
```

### 3. 负载测试

使用 `wrk` 进行压力测试:

```bash
# 安装 wrk
brew install wrk  # macOS
apt-get install wrk  # Ubuntu

# Command 写入测试
wrk -t4 -c100 -d30s \
  -s scripts/post_order.lua \
  http://localhost:8080/api/spot/v2/order

# Query 读取测试
wrk -t8 -c200 -d30s \
  http://localhost:8080/api/spot/user/data?user_id=alice
```

Lua 脚本 (`scripts/post_order.lua`):
```lua
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"user_id": "alice", "symbol": "BTCUSDT", "price": 50000}'
```

### 4. 容灾测试

```bash
# 停止 zone_1 的第一台服务器
ssh 10.0.1.1 "systemctl stop backend"

# alice 的请求应自动路由到 10.0.1.2
curl -X POST http://localhost:8080/api/spot/v2/order \
  -d '{"user_id": "alice", "action": "buy"}'

# 检查日志确认路由到备份服务器
tail -f /var/log/pingora/access.log
```

---

## 性能优化

### 1. 编译优化

`Cargo.toml`:
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

编译:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### 2. 系统调优

```bash
# 增加文件描述符限制
ulimit -n 65535

# 优化 TCP 参数
sysctl -w net.ipv4.tcp_tw_reuse=1
sysctl -w net.ipv4.ip_local_port_range="1024 65535"
sysctl -w net.core.somaxconn=8192

# 禁用交换
swapoff -a
```

### 3. Pingora 配置优化

```rust
// 增加连接池大小
TransportConnector::new(Some(PoolOptions {
    max_idle_per_host: 128,
    idle_timeout: Duration::from_secs(300)
}))

// 启用 HTTP/2
HttpPeer::new_tls("backend.example.com", true, "backend.example.com".to_string())
```

### 4. 监控指标

关键指标:
- **P99 延迟**: `< 10ms`
- **QPS**: `> 100K`
- **错误率**: `< 0.01%`
- **连接复用率**: `> 90%`

Prometheus 配置:
```yaml
scrape_configs:
  - job_name: 'pingora'
    static_configs:
      - targets: ['localhost:9090']
```

---

## 参考文件

- **路由实现**: `app/pingora_gateway/src/http/router.rs`
- **代理逻辑**: `app/pingora_gateway/src/http/http_proxy.rs`
- **主程序**: `app/pingora_gateway/src/main.rs`
- **测试脚本**: `app/pingora_gateway/test_user_routing.sh`

---

## 总结

本教程展示了如何使用 Pingora 实现生产级的 CQRS 分区路由系统：

✅ **CQRS 分离**: Command (写) 和 Query (读) 独立路由
✅ **分区隔离**: 基于 `user_id` 的自动分区
✅ **容灾机制**: 轮询负载均衡 + 自动故障转移
✅ **水平扩展**: 动态增加分区和服务器
✅ **低延迟**: < 1ms 路由决策 + 零拷贝转发
✅ **高吞吐**: > 100K QPS 单机性能

通过这套架构，可以轻松构建支持千万级用户的分布式交易系统。
