# WebSocket 订单匹配服务

高性能实时订单撮合服务，基于 WebSocket 协议提供超低延迟的交易体验。

## 🚀 特性

- **超低延迟**: 订单处理延迟 < 100μs
- **实时推送**: 成交、订单簿更新实时广播
- **高并发**: 支持数千个并发连接
- **零拷贝**: 消息广播采用 `Arc` 零拷贝设计
- **无锁架构**: 使用 `DashMap` 实现无锁客户端管理
- **Clean Architecture**: 严格遵循整洁架构原则

## 📦 启动服务

### 方式1: 仅启动 WebSocket 服务（默认）

```bash
cargo run --release
# 或
cargo run --release -- websocket
```

服务监听: `ws://localhost:9090/ws`

### 方式2: 启动所有服务（WebSocket + HTTP + JSON-RPC）

```bash
cargo run --release -- all
```

- WebSocket: `ws://localhost:9090/ws`
- HTTP REST: `http://localhost:8080`
- JSON-RPC: `http://localhost:3030`

### 自定义端口

```bash
# 设置 WebSocket 端口
WS_PORT=8888 cargo run --release

# 或通过命令行参数
cargo run --release -- websocket
```

## 📡 消息协议

### 客户端 → 服务器

#### 1. 订阅市场数据

```json
{
  "type": "subscribe",
  "channels": ["trades", "book"]
}
```

#### 2. 下限价单

```json
{
  "type": "limit_order",
  "trader_id": "alice",
  "side": "buy",       // "buy" 或 "sell"
  "price": 50000,
  "quantity": 10
}
```

#### 3. 下市价单

```json
{
  "type": "market_order",
  "trader_id": "bob",
  "side": "sell",
  "quantity": 5
}
```

#### 4. 取消订单

```json
{
  "type": "cancel_order",
  "order_id": 123456
}
```

#### 5. 心跳

```json
{
  "type": "ping"
}
```

#### 6. 取消订阅

```json
{
  "type": "unsubscribe",
  "channels": ["book"]
}
```

### 服务器 → 客户端

#### 1. 订单确认

```json
{
  "type": "order_ack",
  "order_id": 123456,
  "status": "open",      // "open", "partial", "filled", "cancelled"
  "latency_us": 87       // 处理延迟（微秒）
}
```

#### 2. 成交通知（实时广播）

```json
{
  "type": "trade",
  "trade_id": 789012,
  "buyer": "alice",
  "seller": "bob",
  "price": 50000,
  "quantity": 5,
  "timestamp": 1699999999000
}
```

#### 3. 订单簿更新

```json
{
  "type": "book_update",
  "best_bid": 49900,
  "best_ask": 50100,
  "spread": 200
}
```

#### 4. 心跳响应

```json
{
  "type": "pong",
  "timestamp": 1699999999000
}
```

#### 5. 订阅确认

```json
{
  "type": "subscribed",
  "channels": ["trades", "book"]
}
```

#### 6. 错误消息

```json
{
  "type": "error",
  "code": "INVALID_SIDE",
  "message": "invalid side, must be 'buy' or 'sell'"
}
```

## 💻 客户端示例

### 运行示例客户端

```bash
# 启动服务器
cargo run --release -- websocket

# 在另一个终端运行客户端
cargo run --example ws_client --release
```

### JavaScript/TypeScript 客户端

```javascript
const ws = new WebSocket('ws://localhost:9090/ws');

ws.onopen = () => {
  console.log('已连接到服务器');

  // 订阅数据
  ws.send(JSON.stringify({
    type: 'subscribe',
    channels: ['trades', 'book']
  }));

  // 下单
  ws.send(JSON.stringify({
    type: 'limit_order',
    trader_id: 'alice',
    side: 'buy',
    price: 50000,
    quantity: 10
  }));
};

ws.onmessage = (event) => {
  const msg = JSON.parse(event.data);

  switch (msg.type) {
    case 'order_ack':
      console.log(`订单确认: ${msg.order_id}, 延迟: ${msg.latency_us}μs`);
      break;
    case 'trade':
      console.log(`成交: ${msg.quantity}@${msg.price}`);
      break;
    case 'book_update':
      console.log(`最佳买价: ${msg.best_bid}, 最佳卖价: ${msg.best_ask}`);
      break;
  }
};
```

### Python 客户端（websockets）

```python
import asyncio
import json
import websockets

async def trading_client():
    uri = "ws://localhost:9090/ws"
    async with websockets.connect(uri) as ws:
        # 订阅
        await ws.send(json.dumps({
            "type": "subscribe",
            "channels": ["trades"]
        }))

        # 下单
        await ws.send(json.dumps({
            "type": "limit_order",
            "trader_id": "alice",
            "side": "buy",
            "price": 50000,
            "quantity": 10
        }))

        # 接收消息
        async for message in ws:
            msg = json.loads(message)
            print(f"收到: {msg}")

asyncio.run(trading_client())
```

## 🔬 性能测试

### 运行基准测试

```bash
# 先启动服务器
cargo run --release -- websocket

# 在另一个终端运行基准测试
cargo run --example ws_benchmark --release
```

### 测试套件

1. **Ping/Pong 延迟测试** (1000样本)
   - 测量心跳往返延迟

2. **限价单处理延迟测试** (1000样本)
   - 测量订单提交到确认的延迟

3. **订单撮合延迟测试** (500样本)
   - 测量订单匹配成交的端到端延迟

4. **吞吐量测试** (10秒)
   - 测量系统最大订单处理能力

5. **并发连接测试** (10客户端 × 100订单)
   - 测量多客户端并发场景下的性能

### 预期性能指标

基于 Apple M1/M2 或 x86-64 高性能 CPU：

- **Ping/Pong 延迟**: P99 < 500μs
- **订单处理延迟**: P99 < 1ms
- **撮合延迟**: P99 < 2ms
- **吞吐量**: > 50,000 订单/秒（单核）
- **并发连接**: > 10,000 连接

## 🏗️ 架构设计

### Clean Architecture 分层

```
┌─────────────────────────────────────┐
│   WebSocket Interface Layer         │  websocket_service.rs
│   - 消息序列化/反序列化              │
│   - 客户端连接管理                   │
│   - 广播机制                         │
├─────────────────────────────────────┤
│   Application Layer (Use Cases)     │  handler.rs
│   - PlaceOrderUseCase               │
│   - CancelOrderUseCase              │
│   - MatchingService                 │
├─────────────────────────────────────┤
│   Domain Layer (Entities)           │  lob/types.rs
│   - Order                           │
│   - Trade                           │
│   - OrderBook                       │
├─────────────────────────────────────┤
│   Infrastructure Layer              │  lob/repository/
│   - InMemoryOrderRepository         │
│   - EventRepository                 │
└─────────────────────────────────────┘
```

### 关键优化技术

1. **零拷贝广播**: 使用 `Arc<ServerMessage>` 避免消息克隆
2. **无锁并发**: `DashMap` 实现无锁客户端管理
3. **无界通道**: 避免发送端阻塞（背压由客户端处理）
4. **异步I/O**: 基于 Tokio 的异步运行时
5. **缓存行对齐**: 关键数据结构对齐到 64/128 字节
6. **编译优化**: `--release` + `lto = "fat"` + `codegen-units = 1`

## 📊 监控与调试

### 健康检查

```bash
curl http://localhost:9090/health
```

响应:
```json
{
  "status": "healthy",
  "service": "websocket-matching-service",
  "protocol": "ws"
}
```

### 日志级别

```bash
# 调试模式
RUST_LOG=debug cargo run --release

# 仅关键日志
RUST_LOG=info cargo run --release
```

### 性能分析

```bash
# Flamegraph 性能分析
cargo flamegraph --example ws_benchmark

# CPU 分析
perf record -g cargo run --example ws_benchmark --release
perf report
```

## 🔧 配置优化

### Cargo.toml 优化配置

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
```

### 操作系统优化

#### Linux

```bash
# 增加最大连接数
ulimit -n 100000

# CPU 隔离（绑定核心）
taskset -c 0,1 cargo run --release

# 实时优先级
sudo chrt -f 99 cargo run --release
```

#### macOS

```bash
# 增加文件描述符限制
ulimit -n 10000
```

## 🐛 故障排查

### 连接失败

```bash
# 检查服务是否运行
curl http://localhost:9090/health

# 检查端口占用
lsof -i :9090
```

### 高延迟

1. 确认使用 `--release` 模式编译
2. 检查 CPU 占用率（`htop`）
3. 运行基准测试对比
4. 查看日志是否有大量错误

### 连接断开

- 检查网络稳定性
- 确认客户端正确处理 `Ping/Pong`
- 查看服务器日志

## 📚 相关文档

- [Rust 低延迟开发指南](../../ld/RUST_LOW_LATENCY_GUIDE.md)
- [Clean Architecture 标准](../../CLAUDE.md)
- [订单簿实现](../../lib/lob/README.md)

## 🤝 贡献

欢迎提交问题和改进建议！

## 📄 许可证

MIT
