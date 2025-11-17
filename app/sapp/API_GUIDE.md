# 订单匹配服务 API 指南

## 快速开始

### 1. 编译项目
```bash
cd /Users/hongyaotang/src/rustlob/app/sapp
cargo build --release
```

### 2. 启动服务

#### 启动 HTTP REST API（默认，推荐）
```bash
cargo run
# 或指定
cargo run -- axum
```

服务启动在 `http://localhost:8080`

#### 启动 JSON-RPC 服务
```bash
cargo run -- jsonrpc
```

服务启动在 `http://localhost:3030`

#### 同时启动两个服务
```bash
cargo run -- both
```

### 3. 运行测试
```bash
# HTTP API 测试
./test_http.sh

# JSON-RPC 测试
./test_rpc.sh
```

---

## HTTP REST API 文档

### 基础URL
```
http://localhost:8080
```

### 端点列表

#### 1. 健康检查
```http
GET /health
```

**响应示例**:
```json
{
  "status": "healthy",
  "service": "matching-service"
}
```

---

#### 2. 下限价单
```http
POST /api/orders
Content-Type: application/json
```

**请求体**:
```json
{
  "trader_id": "TRADER001",
  "side": "buy",        // "buy" 或 "sell"
  "price": 10000,       // 价格（以分为单位）
  "quantity": 100       // 数量
}
```

**响应示例**:
```json
{
  "order_id": 123,
  "status": "open",     // "filled", "partial", "open"
  "trades": [
    {
      "buyer": "TRADER001",
      "seller": "TRADER002",
      "price": 10000,
      "quantity": 50
    }
  ]
}
```

**状态说明**:
- `filled` - 完全成交（order_id=0）
- `partial` - 部分成交（order_id>0）
- `open` - 未成交（order_id>0）

---

#### 3. 取消订单
```http
POST /api/orders/cancel
Content-Type: application/json
```

**请求体**:
```json
{
  "order_id": 123
}
```

**响应示例**:
```json
{
  "success": true,
  "message": "订单已取消"
}
```

---

#### 4. 查询市场深度
```http
GET /api/market/depth
```

**响应示例**:
```json
{
  "best_bid": 9900,
  "best_ask": 10100,
  "spread": 200
}
```

---

## 使用示例

### cURL 示例

#### 下买单
```bash
curl -X POST http://localhost:8080/api/orders \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "BUYER1",
    "side": "buy",
    "price": 10000,
    "quantity": 100
  }'
```

#### 下卖单
```bash
curl -X POST http://localhost:8080/api/orders \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "SELLER1",
    "side": "sell",
    "price": 10100,
    "quantity": 50
  }'
```

#### 取消订单
```bash
curl -X POST http://localhost:8080/api/orders/cancel \
  -H "Content-Type: application/json" \
  -d '{
    "order_id": 1
  }'
```

#### 查询市场深度
```bash
curl http://localhost:8080/api/market/depth
```

---

### Python 示例

```python
import requests

BASE_URL = "http://localhost:8080"

# 下买单
response = requests.post(f"{BASE_URL}/api/orders", json={
    "trader_id": "PYTHON_TRADER",
    "side": "buy",
    "price": 10000,
    "quantity": 100
})
print(response.json())

# 查询市场深度
response = requests.get(f"{BASE_URL}/api/market/depth")
print(response.json())
```

---

### JavaScript 示例

```javascript
const BASE_URL = "http://localhost:8080";

// 下卖单
async function placeSellOrder() {
  const response = await fetch(`${BASE_URL}/api/orders`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      trader_id: "JS_TRADER",
      side: "sell",
      price: 10100,
      quantity: 50
    })
  });

  const data = await response.json();
  console.log(data);
}

placeSellOrder();
```

---

## 错误处理

所有错误响应格式：
```json
{
  "error": "错误描述"
}
```

**HTTP 状态码**:
- `200 OK` - 成功
- `400 Bad Request` - 请求参数错误
- `500 Internal Server Error` - 服务器内部错误

---

## 性能特性

- ✅ **低延迟**: 微秒级订单处理
- ✅ **高吞吐**: 支持百万级订单
- ✅ **价格-时间优先**: FIFO匹配算法
- ✅ **事件溯源**: 完整的事件追踪
- ✅ **内存回收**: 智能Free List机制

---

## 架构说明

### Clean Architecture 分层

```
HTTP Layer (Axum)
    ↓
Application Layer (Matching Service)
    ↓
Domain Layer (Order Book)
    ↓
Infrastructure Layer (Repository)
```

### 事件溯源

每个订单操作都会生成 `EntityEvent`，包括：
- 订单创建事件
- 订单更新事件（数量变化）
- 订单删除事件
- 交易执行事件
- 价格点更新事件

所有事件都带有 `transaction_id` 保证原子性。

---

## 配置

### 环境变量

```bash
# HTTP 端口（默认 8080）
export PORT=8080

# 日志级别
export RUST_LOG=info
```

### 容量配置

在 `matching_service.rs` 中修改：

```rust
let repository = InMemoryOrderRepository::new(
    100_000,   // 最大价格范围
    1_000_000  // 最大订单数
);
```

---

## 监控和调试

### 启用调试日志
```bash
RUST_LOG=debug cargo run
```

### 查看详细日志
```bash
RUST_LOG=matching_service=trace cargo run
```

---

## 下一步计划

- [ ] WebSocket 实时行情推送
- [ ] Prometheus 指标暴露
- [ ] 订单查询 API
- [ ] 历史交易查询
- [ ] 市场深度快照
- [ ] 性能基准测试

---

## 联系方式

如有问题，请参考：
- 主文档: `README.md`
- 性能优化: `PERFORMANCE_OPTIMIZATION.md`
- 测试报告: `TEST_SUMMARY.md`
