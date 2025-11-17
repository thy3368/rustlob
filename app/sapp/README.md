# SAPP - LOB Matching Service (JSON-RPC Server)

## 概述

SAPP 是基于 LOB (Limit Order Book) 库的高性能订单匹配服务，提供 JSON-RPC 2.0 接口。

**特性**:
- ✅ **JSON-RPC 2.0** - 标准化 RPC 接口
- ✅ **异步处理** - 支持并发请求
- ✅ **模块化架构** - 服务、模型、主程序分离
- ✅ **Clean Architecture** - 领域驱动设计
- ✅ **低延迟** - 内存订单簿，零拷贝操作

## 架构

```
app/sapp/                    # JSON-RPC 服务层
├── Cargo.toml              # 依赖: lob, jsonrpc, tokio
├── src/
│   ├── main.rs             # 服务启动入口
│   ├── rpc_service.rs      # JSON-RPC 服务实现
│   └── models.rs           # 请求/响应数据模型
├── test_rpc.sh             # API 测试脚本
└── README.md               # 本文档

lib/lob/                    # 订单簿核心库
├── src/lob/
│   ├── matching_service.rs # 匹配服务
│   ├── repository/         # 仓储层
│   └── types.rs            # 领域类型
└── tests/                  # 单元测试
```

### 分层架构

```
┌──────────────────────────────────┐
│  RPC Layer (rpc_service.rs)      │
│  • JSON-RPC 2.0 接口             │
│  • 异步请求处理                   │
│  • 数据转换                       │
└────────────┬─────────────────────┘
             │
┌────────────▼─────────────────────┐
│  Service Layer (MatchingService) │
│  • 订单匹配逻辑                   │
│  • 事件溯源                       │
│  • 命令处理                       │
└────────────┬─────────────────────┘
             │
┌────────────▼─────────────────────┐
│  Repository Layer                │
│  • 内存订单簿                     │
│  • 价格-时间优先队列               │
│  • 零拷贝操作                     │
└──────────────────────────────────┘
```

## 快速开始

### 1. 启动服务

```bash
cargo run --release
```

服务输出：
```
╔══════════════════════════════════════════════════════════════╗
║           LOB Matching Service - JSON-RPC Server            ║
╚══════════════════════════════════════════════════════════════╝

✓ 服务器已启动
  监听地址: http://127.0.0.1:3030
  工作线程: 4
  订单容量: 100000
  价格范围: 0 - 1000000

可用的 RPC 方法:
  📝 place_limit_order    - 提交限价单
  🚀 place_market_order   - 提交市价单
  🧊 place_iceberg_order  - 提交冰山单
  ❌ cancel_order         - 取消订单
  📊 get_book_status      - 获取订单簿状态
  💚 health               - 健康检查

按 Ctrl+C 停止服务器...
```

### 2. 测试 API

运行自动化测试：
```bash
./test_rpc.sh
```

## API 文档

### 1. `health` - 健康检查

```bash
curl -X POST http://127.0.0.1:3030 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"health","params":{},"id":1}'
```

### 2. `place_limit_order` - 提交限价单

```bash
curl -X POST http://127.0.0.1:3030 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"place_limit_order",
    "params":{
      "trader_id":"TRADER001",
      "side":"BUY",
      "price":10000,
      "quantity":100
    },
    "id":2
  }'
```

**参数**:
- `trader_id`: 交易员ID (字符串)
- `side`: `"BUY"` 或 `"SELL"`
- `price`: 价格 (u32)
- `quantity`: 数量 (u32)

**响应**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "order_id": 1,
    "trades": [
      {
        "buyer": "BUYER001",
        "seller": "SELLER01",
        "price": 10000,
        "quantity": 50
      }
    ],
    "status": "success"
  },
  "id": 2
}
```

### 3. `get_book_status` - 获取订单簿状态

```bash
curl -X POST http://127.0.0.1:3030 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"get_book_status","params":{},"id":3}'
```

**响应**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "best_bid": 9999,
    "best_ask": 10001,
    "spread": 2,
    "active_orders": 5
  },
  "id": 3
}
```

完整 API 文档请参考源代码注释。

## 使用 LOB 库

### 依赖配置

在 `Cargo.toml` 中：
```toml
[dependencies]
lob = { path = "../../lib/lob" }
```

### 基本用法

```rust
use lob::lob::{OrderBook, TraderId, Side};

fn main() {
    // 创建订单簿
    let mut book = OrderBook::new();

    // 创建交易员
    let buyer = TraderId::from_str("BUYER001");
    let seller = TraderId::from_str("SELLER01");

    // 放置卖单
    let (sell_order_id, _) = book.limit_order(seller, Side::Sell, 10000, 100);

    // 放置买单（匹配）
    let (buy_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 50);

    // 查询订单簿状态
    println!("最佳买价: {:?}", book.best_bid());
    println!("最佳卖价: {:?}", book.best_ask());
}
```

## Clean Architecture 实践

```
┌─────────────────────────────────────┐
│  应用层 (app/sapp)                   │
│  - 用户交互和展示逻辑                 │
└──────────────┬──────────────────────┘
               │ 依赖
               ▼
┌─────────────────────────────────────┐
│  库层 (lib/lob)                      │
│  - 领域模型和业务逻辑                 │
│  - 无外部依赖                        │
└─────────────────────────────────────┘
```

## 开发指南

### 修改库代码
```bash
cd ../../lib/lob
# 编辑 src/lob/*.rs
cargo test  # 运行测试
```

### 修改应用代码
```bash
cd app/sapp
# 编辑 src/main.rs
cargo run
```

## 参考资料

- [LOB库文档](../../lib/lob/README.md)
- [集成测试文档](../../lib/lob/tests/README.md)
- [Clean Architecture指南](../../CLAUDE.md)

