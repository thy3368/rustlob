# Pingora 网关实现分区路由与容灾教程

## 概述

本教程展示如何使用 Pingora 实现基于 `user_id` 的分区路由系统，支持：
- **读写分离**: `trade` 路由写操作，`user_data` 路由读操作
- **分区路由**: 根据 `user_id` 将请求路由到指定分区
- **容灾**: 每个分区支持多台服务器，自动故障转移
- **水平扩展**: 通过增加分区实现线性扩展

## 核心架构

```
客户端请求
    ↓
Pingora Gateway (0.0.0.0:8080)
    ↓
提取 user_id (JSON/Header/Query)
    ↓
确定分区 (zone_1, zone_2, zone_3...)
    ↓
轮询选择服务器 (容灾)
    ↓
转发到后端服务器
```

## 系统组件

### 1. 分区配置 (`UserRouteConfig`)

定义用户到分区的映射和分区内的服务器列表：

```rust
pub struct UserRouteConfig {
    /// 分区 -> 服务器列表 (每分区多服务器容灾)
    pub partition_ips: HashMap<String, Vec<String>>,

    /// 用户 -> 分区映射
    pub user_partition: HashMap<String, String>,

    /// 默认后端 (用户未配置时)
    pub default_backend: String
}
```

**配置示例**:
```rust
let mut user_partition = HashMap::new();
user_partition.insert("alice".to_string(), "zone_1".to_string());
user_partition.insert("bob".to_string(), "zone_2".to_string());

let mut partition_ips = HashMap::new();
// zone_1: 2台服务器容灾
partition_ips.insert("zone_1".to_string(), vec![
    "127.0.0.1:3001".to_string(),
    "127.0.0.1:3002".to_string()
]);

// zone_2: 2台服务器容灾
partition_ips.insert("zone_2".to_string(), vec![
    "127.0.0.1:3003".to_string(),
    "127.0.0.1:3004".to_string()
]);
```

### 2. 用户路由器 (`UserRouter`)

核心路由逻辑，负责：
- 根据 `user_id` 选择分区
- 在分区内轮询选择服务器（负载均衡）
- 提供热更新配置能力

**关键代码** (`src/http/router.rs:54-77`):
```rust
pub async fn select_backend(&self, user_id: &str) -> HttpPeer {
    let config = self.config.read().await;

    if let Some(partition) = config.user_partition.get(user_id) {
        let backends = config.partition_ips.get(partition).unwrap();

        // 获取轮询索引
        let mut indices = self.round_robin_index.write().await;
        let index = indices.entry(user_id.to_string()).or_insert(0);

        // 轮询选择后端 (容灾机制)
        let backend = &backends[*index % backends.len()];
        *index = (*index + 1) % backends.len();

        self.create_peer(backend)
    } else {
        // 未配置用户使用默认后端
        self.create_peer(&config.default_backend)
    }
}
```

### 3. 用户ID提取器 (`UserIdExtractor`)

支持从多种来源提取 `user_id`：

#### 3.1 从 JSON 请求体提取
支持字段: `user_id`, `userId`, `trader_id`, `traderId`, `uid`, `accountId`

```rust
// POST /api/spot/v2/
// Body: {"user_id": "alice", "symbol": "BTCUSDT"}
pub fn extract_from_json(body: &[u8]) -> Option<String>
```

#### 3.2 从 HTTP 请求头提取
支持请求头: `X-User-Id`, `X-Trader-Id`

```rust
// X-User-Id: alice
pub fn extract_from_headers(headers: &str) -> Option<String>
```

#### 3.3 从查询参数提取
```rust
// GET /api/spot/user/data?user_id=alice
pub fn extract_from_query(url: &str) -> Option<String>
```

### 4. HTTP 代理应用 (`HttpProxyApp`)

Pingora 应用实现，负责：
1. 解析 HTTP 请求
2. 提取 `user_id`
3. 根据路径判断是否需要用户路由
4. 连接后端并转发数据

**路由判断逻辑** (`src/http/http_proxy.rs:147-149`):
```rust
fn needs_user_routing(path: &str) -> bool {
    path.starts_with("/api/spot/v2/") ||      // 交易写入
    path.starts_with("/api/spot/user/data")   // 用户数据读取
}
```

## 完整工作流程

### 请求处理流程 (`src/http/http_proxy.rs:186-238`)

```
1. 接收客户端连接
   ↓
2. 解析 HTTP 请求 (parse_http_request)
   - 读取请求头
   - 解析请求路径
   - 提取用户ID (查询参数/请求头/请求体)
   ↓
3. 路由决策 (needs_user_routing)
   ├─ /api/spot/v2/* → 用户路由 (写操作)
   ├─ /api/spot/user/data → 用户路由 (读操作)
   └─ 其他路径 → 默认后端
   ↓
4. 选择后端服务器 (user_router.select_backend)
   - 查找用户分区
   - 轮询选择服务器
   ↓
5. 建立后端连接
   ↓
6. 转发请求数据
   ↓
7. 双工数据转发 (duplex)
```

## 容灾机制

### 轮询负载均衡

每个分区内多台服务器通过轮询算法分配请求：

```rust
// 为每个用户维护独立的轮询索引
round_robin_index: HashMap<String, usize>

// 选择算法
let backend = &backends[*index % backends.len()];
*index = (*index + 1) % backends.len();
```

**示例**:
```
zone_1 有 2 台服务器: [3001, 3002]
alice 的请求序列:
  请求1 → 3001 (index=0)
  请求2 → 3002 (index=1)
  请求3 → 3001 (index=2)
  请求4 → 3002 (index=3)
```

### 故障转移

当某台服务器故障时：
1. **自动检测**: Pingora 连接失败自动重试
2. **跳过故障节点**: 从分区配置中移除故障服务器
3. **热更新配置**: 使用 `update_config` 动态调整

```rust
// 热更新配置示例
let mut new_config = config.clone();
new_config.partition_ips.get_mut("zone_1").unwrap()
    .retain(|ip| ip != "127.0.0.1:3001");  // 移除故障服务器

user_router.update_config(new_config).await;
```

## 水平扩展

### 增加新分区

```rust
// 1. 添加新分区服务器
partition_ips.insert("zone_4".to_string(), vec![
    "127.0.0.1:3005".to_string(),
    "127.0.0.1:3006".to_string()
]);

// 2. 分配用户到新分区
user_partition.insert("charlie".to_string(), "zone_4".to_string());
user_partition.insert("david".to_string(), "zone_4".to_string());
```

### 分区内增加服务器

```rust
// 为 zone_1 增加第3台服务器
partition_ips.get_mut("zone_1").unwrap()
    .push("127.0.0.1:3007".to_string());
```

## 部署配置

### 1. 启动后端服务器

```bash
# 分区1 (zone_1)
backend-server --port 3001 --partition zone_1 &
backend-server --port 3002 --partition zone_1 &

# 分区2 (zone_2)
backend-server --port 3003 --partition zone_2 &
backend-server --port 3004 --partition zone_2 &
```

### 2. 配置网关

编辑 `src/http/router.rs` 的 `UserRouteConfig::default()`:

```rust
impl Default for UserRouteConfig {
    fn default() -> Self {
        let mut user_partition = HashMap::new();
        let mut partition_ips = HashMap::new();

        // 定义分区
        partition_ips.insert("zone_1".to_string(), vec![
            "10.0.1.10:3001".to_string(),
            "10.0.1.11:3001".to_string()
        ]);
        partition_ips.insert("zone_2".to_string(), vec![
            "10.0.2.10:3001".to_string(),
            "10.0.2.11:3001".to_string()
        ]);

        // 分配用户
        user_partition.insert("alice".to_string(), "zone_1".to_string());
        user_partition.insert("bob".to_string(), "zone_2".to_string());

        UserRouteConfig {
            partition_ips,
            user_partition,
            default_backend: "10.0.1.10:3001".to_string()
        }
    }
}
```

### 3. 启动网关

```bash
cd app/pingora_gateway
cargo build --release
./target/release/pingora_gateway
```

输出示例:
```
🚀 Pingora HTTP proxy started at http://localhost:8080
📊 Default backend: http://localhost:3001
🔀 User-based routing enabled for:
   - /api/spot/v2/*
   - /api/spot/user/data

👥 User routing configuration:
   - alice → zone_1
   - bob → zone_2
```

## 测试验证

### 测试用户路由

```bash
# alice 路由到 zone_1
curl -X POST http://localhost:8080/api/spot/v2/ \
  -H "Content-Type: application/json" \
  -d '{"user_id": "alice", "symbol": "BTCUSDT", "price": 50000}'

# 使用请求头
curl -X POST http://localhost:8080/api/spot/v2/ \
  -H "X-User-Id: alice" \
  -H "Content-Type: application/json" \
  -d '{"symbol": "BTCUSDT", "price": 50000}'

# 使用查询参数
curl "http://localhost:8080/api/spot/user/data?user_id=alice"
```

### 测试容灾

```bash
# 停止 zone_1 的第一台服务器
kill <pid-of-3001>

# alice 的请求自动路由到 3002
curl -X POST http://localhost:8080/api/spot/v2/ \
  -d '{"user_id": "alice", "action": "buy"}'
```

### 单元测试

```bash
cd app/pingora_gateway
cargo test

# 运行特定测试
cargo test test_user_router_selection
cargo test test_round_robin
cargo test test_extract_user_id_from_json
```

## 性能优化

### 1. 缓存行对齐

```rust
#[repr(align(64))]
struct CacheAlignedRouter {
    router: UserRouter
}
```

### 2. 零拷贝转发

Pingora 使用零拷贝技术，数据直接在内核空间转发：
```rust
// duplex 函数实现双向零拷贝转发
async fn duplex(&self, mut server_session: Stream, mut client_session: Stream)
```

### 3. 连接池复用

```rust
// TransportConnector 自动管理连接池
client_connector: TransportConnector::new(None)
```

## 监控指标

### 关键指标

- **路由延迟**: `user_router.select_backend()` 执行时间
- **连接成功率**: 后端连接成功/失败比例
- **分区负载**: 每个分区的请求量
- **轮询分布**: 验证负载均衡均匀性

### 日志示例

```
🔀 User routing: /api/spot/v2/ -> user_id=alice
📡 Proxying /api/spot/v2/ to 127.0.0.1:3001
⚠️  Path requires user routing but no user_id found: /api/spot/v2/
```

## 最佳实践

### 1. 分区划分策略

- **地理位置**: 按用户地区分区减少延迟
- **负载均衡**: 确保每个分区用户数相近
- **数据局部性**: 相关用户分配到同一分区

### 2. 容灾配置

- **最小副本数**: 每分区至少2台服务器
- **健康检查**: 定期检测后端服务器状态
- **自动恢复**: 故障服务器恢复后自动加入

### 3. 安全考虑

- **用户ID验证**: 防止伪造 `user_id`
- **访问控制**: 限制跨分区访问
- **审计日志**: 记录所有路由决策

## 故障排查

### 问题1: 用户未路由到正确分区

**检查清单**:
1. 确认 `user_id` 提取成功 (查看日志)
2. 检查 `user_partition` 配置
3. 验证路径匹配 `needs_user_routing()`

### 问题2: 后端连接失败

**检查清单**:
1. 后端服务器是否运行
2. IP/端口配置是否正确
3. 网络连通性测试

### 问题3: 负载不均衡

**检查清单**:
1. 分区内服务器数量
2. 轮询索引是否正常递增
3. 用户请求分布是否均匀

## 扩展功能

### 1. 一致性哈希路由

替代用户ID映射表，使用一致性哈希算法：
```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_user_id(user_id: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    user_id.hash(&mut hasher);
    hasher.finish()
}

let partition_id = format!("zone_{}", hash_user_id(user_id) % num_partitions);
```

### 2. 动态权重负载均衡

根据服务器性能动态调整权重：
```rust
pub struct WeightedBackend {
    addr: String,
    weight: u32,  // 权重值
    current_load: Arc<AtomicU32>
}
```

### 3. 熔断器模式

防止故障传播：
```rust
pub struct CircuitBreaker {
    failure_count: AtomicU32,
    threshold: u32,
    state: AtomicU8  // Open/Half-Open/Closed
}
```

## 参考文件

- **路由实现**: `app/pingora_gateway/src/http/router.rs`
- **代理逻辑**: `app/pingora_gateway/src/http/http_proxy.rs`
- **应用入口**: `app/pingora_gateway/src/main.rs`
- **测试脚本**: `app/pingora_gateway/test_user_routing.sh`

## 总结

本教程展示了如何使用 Pingora 实现生产级的分区路由系统：

✅ **分区路由**: 根据 `user_id` 自动分发到指定分区
✅ **容灾机制**: 分区内多服务器轮询，自动故障转移
✅ **水平扩展**: 通过增加分区支持线性扩展
✅ **低延迟**: 零拷贝转发 + 连接池复用
✅ **灵活配置**: 支持热更新、多种提取方式

通过这套架构，可以轻松构建支持百万级并发的分布式交易系统。
