# Pingora Gateway - 用户路由网关

基于 Pingora 构建的高性能 HTTP/WebSocket 反向代理网关，支持基于用户 ID 的智能路由。

## 🎯 核心功能

### 1. 基于用户的智能路由

对于特定路径（`/api/spot/v2/` 和 `/api/spot/user/data`），网关会根据请求中的用户 ID 将流量路由到不同的后端服务器集群。

**路由逻辑**:
```
用户请求 → 提取 user_id → 查找用户路由表 → 负载均衡选择后端 → 转发请求
```

### 2. 用户 ID 提取策略

网关支持从多个位置提取用户 ID（按优先级）:

1. **URL 查询参数**
   ```bash
   GET /api/spot/v2/?user_id=alice&symbol=BTCUSDT
   ```

2. **HTTP 请求头**
   ```bash
   curl -H "X-User-Id: alice" http://localhost:8080/api/spot/v2/
   ```

3. **JSON 请求体**（POST 请求）
   ```json
   {
     "user_id": "alice",
     "symbol": "BTCUSDT",
     "price": 50000
   }
   ```

支持的字段名称:
- `user_id`
- `userId`
- `trader_id`
- `traderId`
- `uid`
- `accountId`
- `account_id`

### 3. 负载均衡

对于拥有多个后端服务器的用户，网关采用**轮询（Round-Robin）**策略进行负载均衡。

```rust
// 用户 alice 配置了 2 个后端
user_routes.insert(
    "alice".to_string(),
    vec!["127.0.0.1:3001".to_string(), "127.0.0.1:3002".to_string()],
);

// 请求顺序
// Request 1 → 127.0.0.1:3001
// Request 2 → 127.0.0.1:3002
// Request 3 → 127.0.0.1:3001 (轮询)
```

## 🚀 快速开始

### 启动网关

```bash
cd /Users/hongyaotang/src/rustlob/app/pingora_gateway
cargo run --release
```

输出示例:
```
🚀 Pingora HTTP proxy started at http://localhost:8080
📊 Default backend: http://localhost:3001
🔀 User-based routing enabled for:
   - /api/spot/v2/*
   - /api/spot/user/data

👥 User routing configuration:
   - alice → ["127.0.0.1:3001"]
   - bob → ["127.0.0.1:3002"]
   - user_1 → ["127.0.0.1:3001", "127.0.0.1:3002"]
   - user_2 → ["127.0.0.1:3003", "127.0.0.1:3004"]
```

### 测试用户路由

#### 1. 使用查询参数

```bash
# alice 的请求路由到 127.0.0.1:3001
curl -X POST "http://localhost:8080/api/spot/v2/?user_id=alice" \
  -H "Content-Type: application/json" \
  -d '{"symbol": "BTCUSDT", "price": 50000}'

# bob 的请求路由到 127.0.0.1:3002
curl -X POST "http://localhost:8080/api/spot/v2/?user_id=bob" \
  -H "Content-Type: application/json" \
  -d '{"symbol": "ETHUSDT", "price": 3000}'
```

#### 2. 使用 HTTP 请求头

```bash
curl -X POST "http://localhost:8080/api/spot/user/data" \
  -H "Content-Type: application/json" \
  -H "X-User-Id: alice" \
  -d '{"action": "get_balance"}'
```

#### 3. 使用 JSON 请求体

```bash
curl -X POST "http://localhost:8080/api/spot/v2/" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "alice",
    "symbol": "BTCUSDT",
    "side": "buy",
    "price": 50000,
    "quantity": 1.5
  }'
```

## ⚙️ 配置

### 用户路由配置

编辑 `src/http/router.rs` 中的 `UserRouteConfig::default()`:

```rust
impl Default for UserRouteConfig {
    fn default() -> Self {
        let mut user_routes = HashMap::new();

        // VIP 用户 - 独享服务器
        user_routes.insert(
            "vip_user_001".to_string(),
            vec!["192.168.1.100:3001".to_string()],
        );

        // 普通用户 - 共享服务器集群
        user_routes.insert(
            "normal_user_001".to_string(),
            vec![
                "192.168.1.101:3001".to_string(),
                "192.168.1.102:3001".to_string(),
                "192.168.1.103:3001".to_string(),
            ],
        );

        UserRouteConfig {
            user_routes,
            default_backend: "192.168.1.200:3001".to_string(), // 默认后端
        }
    }
}
```

### 路由规则配置

编辑 `src/http/http_proxy.rs` 中的 `needs_user_routing()` 函数来添加更多需要用户路由的路径:

```rust
fn needs_user_routing(path: &str) -> bool {
    path.starts_with("/api/spot/v2/")
        || path.starts_with("/api/spot/user/data")
        || path.starts_with("/api/futures/v1/")  // 新增期货路由
        || path.starts_with("/api/custom/route")  // 新增自定义路由
}
```

## 📊 监控和日志

### 日志输出

网关会实时输出路由信息:

```
INFO  🔀 User routing: /api/spot/v2/ -> user_id=alice
INFO  📡 Proxying /api/spot/v2/ to 127.0.0.1:3001
```

### 路由失败处理

如果无法提取用户 ID，网关会使用默认后端:

```
WARN  ⚠️  Path requires user routing but no user_id found: /api/spot/v2/
INFO  Using default backend for: /api/spot/v2/
INFO  📡 Proxying /api/spot/v2/ to 127.0.0.1:3001
```

## 🏗️ 架构设计

### 模块结构

```
pingora_gateway/
├── src/
│   ├── http/
│   │   ├── mod.rs
│   │   ├── http_proxy.rs       # 主代理逻辑
│   │   └── router.rs            # 用户路由器
│   ├── websocket/
│   │   └── mod.rs
│   └── main.rs
```

### 核心组件

#### 1. UserRouter（用户路由器）

负责根据用户 ID 选择后端服务器：

```rust
pub struct UserRouter {
    config: Arc<RwLock<UserRouteConfig>>,
    round_robin_index: Arc<RwLock<HashMap<String, usize>>>,
}

impl UserRouter {
    pub async fn select_backend(&self, user_id: &str) -> HttpPeer { ... }
    pub async fn update_config(&self, new_config: UserRouteConfig) { ... }
}
```

#### 2. UserIdExtractor（用户ID提取器）

从 HTTP 请求中提取用户 ID：

```rust
pub struct UserIdExtractor;

impl UserIdExtractor {
    pub fn extract_from_json(body: &[u8]) -> Option<String> { ... }
    pub fn extract_from_headers(headers: &str) -> Option<String> { ... }
    pub fn extract_from_query(url: &str) -> Option<String> { ... }
}
```

#### 3. HttpProxyApp（代理应用）

主代理服务器逻辑：

```rust
pub struct HttpProxyApp {
    client_connector: TransportConnector,
    proxy_to: HttpPeer,
    user_router: Arc<UserRouter>,
}
```

### 请求处理流程

```
┌─────────────────────────────────────────────────────────────┐
│  1. 客户端请求                                                │
│     POST /api/spot/v2/                                       │
│     {"user_id": "alice", "symbol": "BTCUSDT"}               │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  2. 解析 HTTP 请求                                            │
│     - 提取请求路径: /api/spot/v2/                            │
│     - 提取用户ID: alice                                       │
│     - 缓存完整请求数据                                         │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  3. 路由决策                                                  │
│     if needs_user_routing("/api/spot/v2/") {                │
│         select_backend("alice") → 127.0.0.1:3001            │
│     } else {                                                 │
│         use default_backend                                  │
│     }                                                        │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  4. 建立后端连接                                              │
│     TransportConnector::new_stream(127.0.0.1:3001)          │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  5. 转发请求                                                  │
│     client_session.write_all(request_data)                   │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  6. 双工代理                                                  │
│     duplex(client_stream, backend_stream)                    │
│     - 客户端 ⇄ 网关 ⇄ 后端                                    │
└─────────────────────────────────────────────────────────────┘
```

## 🧪 测试

### 单元测试

```bash
cd /Users/hongyaotang/src/rustlob/app/pingora_gateway
cargo test
```

测试覆盖:
- ✅ 用户路由选择
- ✅ 轮询负载均衡
- ✅ 用户ID提取（JSON/Header/Query）
- ✅ 默认后端回退

### 集成测试示例

```rust
#[tokio::test]
async fn test_user_routing() {
    let config = UserRouteConfig::default();
    let router = UserRouter::new(config);

    let peer = router.select_backend("alice").await;
    assert!(peer.address().to_string().contains("3001"));
}
```

## 🔧 性能优化

### 低时延设计

1. **零拷贝转发**: 请求数据直接转发，避免多次拷贝
2. **异步 I/O**: 基于 Tokio 异步运行时
3. **连接复用**: 后端连接可复用（Pingora 内置）
4. **内存预分配**: 使用 `Vec::with_capacity` 预分配缓冲区

### 并发处理

- 异步非阻塞模型
- 每个连接独立处理
- 支持数万并发连接

## 📈 生产部署

### Docker 部署

```dockerfile
FROM rust:1.70 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin pingora_gateway

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/pingora_gateway /usr/local/bin/
EXPOSE 8080
CMD ["pingora_gateway"]
```

### Kubernetes 部署

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pingora-inbound_adapter
spec:
  replicas: 3
  selector:
    matchLabels:
      app: pingora-inbound_adapter
  template:
    metadata:
      labels:
        app: pingora-inbound_adapter
    spec:
      containers:
      - name: inbound_adapter
        image: rustlob/pingora-inbound_adapter:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            cpu: "2"
            memory: "4Gi"
          limits:
            cpu: "4"
            memory: "8Gi"
---
apiVersion: v1
kind: Service
metadata:
  name: pingora-inbound_adapter
spec:
  type: LoadBalancer
  selector:
    app: pingora-inbound_adapter
  ports:
  - port: 80
    targetPort: 8080
```

## 🛡️ 安全考虑

### 请求大小限制

当前实现中，HTTP 请求大小限制为 1MB：

```rust
if buffer.len() > 1024 * 1024 {
    warn!("HTTP request too large, aborting");
    return None;
}
```

### 用户 ID 验证

建议在生产环境中添加用户 ID 验证逻辑：

```rust
fn validate_user_id(user_id: &str) -> bool {
    // 长度检查
    if user_id.len() > 64 {
        return false;
    }

    // 字符集检查（仅允许字母数字和下划线）
    user_id.chars().all(|c| c.is_alphanumeric() || c == '_')
}
```

## 🔮 未来计划

- [ ] 热更新路由配置（不重启服务）
- [ ] 基于权重的负载均衡
- [ ] 健康检查和自动故障转移
- [ ] 限流和熔断机制
- [ ] Prometheus 指标导出
- [ ] WebSocket 用户路由支持
- [ ] 动态路由规则（基于请求内容）

## 📚 相关文档

- [Pingora 官方文档](https://github.com/cloudflare/pingora)
- [RustLOB 项目主页](../../README.md)
- [Clean Architecture 设计规范](../../CLAUDE.md)

## 📄 许可证

MIT License - 详见 [LICENSE](../../LICENSE)
