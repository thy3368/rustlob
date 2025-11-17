# SAPP - 订单匹配服务应用层

高性能订单撮合系统的应用层实现，提供多种协议接口访问底层限价订单簿（LOB）引擎。

## 🚀 快速开始

### 启动服务

```bash
# 方式1: 启动 WebSocket 服务（默认，推荐）
cargo run --release

# 方式2: 启动 HTTP REST API
cargo run --release -- axum

# 方式3: 启动 JSON-RPC 服务
cargo run --release -- jsonrpc

# 方式4: 同时启动所有服务
cargo run --release -- all
```

### 端口配置

| 服务类型 | 默认端口 | 访问地址 |
|---------|---------|---------|
| WebSocket | 9090 | `ws://localhost:9090/ws` |
| HTTP REST | 8080 | `http://localhost:8080` |
| JSON-RPC | 3030 | `http://localhost:3030` |

## 📦 服务类型对比

| 特性 | WebSocket | HTTP REST | JSON-RPC |
|-----|-----------|-----------|----------|
| 延迟 | **< 100μs** | ~ 1ms | ~ 1ms |
| 实时推送 | ✅ | ❌ | ❌ |
| 双向通信 | ✅ | ❌ | ❌ |
| 并发连接 | > 10,000 | 中等 | 中等 |
| 易用性 | 中等 | ✅ | 中等 |
| 推荐场景 | 实时交易 | API集成 | 传统系统 |

## 📡 WebSocket 服务（推荐）

详细文档: [WEBSOCKET.md](./WEBSOCKET.md)

```bash
# 启动服务
cargo run --release

# 运行示例客户端
cargo run --example ws_client --release

# 运行性能测试
cargo run --example ws_benchmark --release
```

### 快速示例

```javascript
const ws = new WebSocket('ws://localhost:9090/ws');

ws.onmessage = (event) => {
  const msg = JSON.parse(event.data);
  if (msg.type === 'trade') {
    console.log(`成交: ${msg.quantity}@${msg.price}`);
  }
};

// 下单
ws.send(JSON.stringify({
  type: 'limit_order',
  trader_id: 'alice',
  side: 'buy',
  price: 50000,
  quantity: 10
}));
```

## 🌐 HTTP REST API

```bash
# 启动服务
cargo run --release -- axum

# 下单
curl -X POST http://localhost:8080/api/orders \
  -H "Content-Type: application/json" \
  -d '{"trader_id":"alice","side":"buy","price":50000,"quantity":10}'

# 查询深度
curl http://localhost:8080/api/market/depth
```

## 🔌 JSON-RPC 服务

```bash
# 启动服务
cargo run --release -- jsonrpc

# 调用
curl -X POST http://localhost:3030 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"place_limit_order",
    "params":{"trader_id":"alice","side":"BUY","price":50000,"quantity":10},
    "id":1
  }'
```

## 🏗️ 架构设计

### Clean Architecture 分层

```
┌─────────────────────────────────────┐
│   Interfaces Layer                  │
│   - websocket_service.rs            │
│   - rest_service.rs                 │
│   - json_rpc_service.rs             │
├─────────────────────────────────────┤
│   Application Layer                 │
│   - OrderCommandHandler             │
│   - Command/CommandResult           │
├─────────────────────────────────────┤
│   Domain Layer (lib/lob)            │
│   - MatchingService                 │
│   - Order, Trade 实体               │
├─────────────────────────────────────┤
│   Infrastructure Layer              │
│   - InMemoryOrderRepository         │
│   - Arena 分配器                    │
└─────────────────────────────────────┘
```

## 📊 性能指标

### 预期性能（Apple M1/M2 或高性能 x86-64）

| 指标 | 目标值 |
|-----|--------|
| WebSocket Ping/Pong (P99) | < 500μs |
| 订单处理延迟 (P99) | < 1ms |
| 撮合延迟 (P99) | < 2ms |
| 吞吐量 | > 50,000 订单/秒 |

### 运行性能测试

```bash
cargo run --example ws_benchmark --release
```

## 📚 项目结构

```
app/sapp/
├── src/
│   ├── main.rs                 # 应用入口
│   ├── websocket_service.rs    # WebSocket 服务
│   ├── rest_service.rs         # HTTP REST 服务
│   ├── json_rpc_service.rs     # JSON-RPC 服务
│   └── models.rs               # 数据模型
├── examples/
│   ├── ws_client.rs            # WebSocket 客户端示例
│   └── ws_benchmark.rs         # 性能基准测试
├── Cargo.toml
├── README.md                   # 本文件
├── WEBSOCKET.md               # WebSocket 详细文档
└── test_ws.sh                 # 快速测试脚本
```

## 🔧 环境变量

```bash
export WS_PORT=9090     # WebSocket 端口
export PORT=8080        # HTTP 端口
export RUST_LOG=info    # 日志级别
```

## 📖 相关文档

- [WebSocket 详细文档](./WEBSOCKET.md) - 实时推送服务完整指南
- [LOB 引擎文档](../../lib/lob/README.md) - 底层撮合引擎
- [Clean Architecture 标准](../../CLAUDE.md) - 架构设计规范
- [Rust 低延迟指南](../../ld/RUST_LOW_LATENCY_GUIDE.md) - 性能优化

## 📄 许可证

MIT License
