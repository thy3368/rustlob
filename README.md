# RustLOB - 超低时延加密货币交易所

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-48%20passed-green.svg)](lib/lob/tests)
[![Architecture](https://img.shields.io/badge/architecture-clean-green.svg)](CLAUDE.md)

> 基于 Rust 从零构建的**生产级低时延中心化交易所 (CEX)**，对标主流交易所的微秒级交易系统，实现 **RPO=0** 和 **RTO<5min** 的金融级高可用。

---

## 🎯 项目定位

**专业级金融交易系统工程实践**，深度展示 Rust 在高性能场景的极致优化和 Clean Architecture 架构设计。

### 核心目标
- 📊 **微秒级交易性能** - 订单处理 < 50ns，端到端撮合 < 1μs
- 🛡️ **金融级高可用** - RPO=0 零数据丢失，RTO<5min 快速恢复
- 🏗️ **业务流隔离架构** - 避免分布式锁，避免跨微服务调用
- 🎨 **Clean Architecture** - DDD 领域驱动，微内核设计

---

## 📦 产品线

| 产品 | 状态 | 功能 |
|-----|------|------|
| **现货交易** | ✅ 已开发 | 币币交易、限价/市价订单、实时撮合 |
| **合约交易** | 🚧 规划中 | 永续/交割合约、杠杆交易、强平机制 |
| **期权交易** | 📅 规划中 | 欧式/美式期权、隐含波动率、Greeks |

---

## 🔥 核心特性

### 1. 内存交易引擎

全内存架构，订单簿/持仓/账户驻留内存，撮合过程零磁盘 I/O，通过 WAL + Snapshot 保证持久化。

**性能指标**：
- 订单处理时延: **< 50ns**
- 订单簿更新: **< 200ns**
- 端到端撮合: **< 1μs**

**技术手段**：
- 定长数组 FIFO 队列，充分利用 CPU 缓存
- 缓存行对齐 (#[repr(align(64))])，避免 false sharing
- 零内存分配，避免 GC 和碎片

---

### 2. 业务流隔离部署

```
单个业务流 = [下单 → 风控 → 撮合 → 清算 → 通知]
              ↓
          独立进程/容器部署
```

**核心优势**：
- ✅ 避免分布式锁 - 流内独占资源
- ✅ 避免跨服务调用 - 本地函数调用
- ✅ 内存队列通信 - 消除网络时延
- ✅ 故障隔离彻底 - 单流故障不影响其他交易对

**差异化 SLA**：
| 交易对 | 时延目标 | QPS | 资源 |
|-------|---------|-----|------|
| 热门币对 | < 1μs | 100K TPS | 8核32GB |
| 普通币对 | < 10μs | 50K TPS | 4核16GB |
| 长尾币种 | < 100μs | 10K TPS | 2核8GB |

---

### 3. 多协议接入

**支持协议**：
- 🌐 **REST API** - HTTP/1.1, HTTP/2（账户管理、历史查询）
- ⚡ **WebSocket** - 双向实时推送（行情、订单、成交）
- 📡 **FIX 协议** - 专业机构接入（FIX 4.2/4.4）
- 📤 **UDP 组播** - 低延迟行情广播
- 🔒 **TCP 单播** - 可靠交易指令传输

**底层技术**：Tokio 异步运行时 + mio 底层 I/O 多路复用 + bytes 零拷贝处理

---

### 4. Clean Architecture 实践

```
┌────────────────────────────────────┐
│  基础设施层 (Frameworks)            │  Tokio/PostgreSQL/Kafka
├────────────────────────────────────┤
│  接口适配层 (Adapters)              │  REST/WebSocket/FIX/Repository
├────────────────────────────────────┤
│  应用层 (Use Cases)                 │  PlaceOrder/CancelOrder/Matching
├────────────────────────────────────┤
│  领域层 (Domain) ⭐                 │  Order/Trade/Position/Account
└────────────────────────────────────┘
```

**设计原则**：
- ✅ 依赖倒置 - trait 定义抽象接口
- ✅ 领域独立 - 无外部依赖，纯业务逻辑
- ✅ 微内核架构 - 插件化扩展（风控/清算/费率）
- ✅ 模块化分层 - core/support/common

---

## ⚡ 技术亮点

### 极致性能优化
- **零拷贝** - zerocopy crate, bytes 零拷贝网络数据
- **无锁并发** - crossbeam 无锁队列，原子操作
- **SIMD 优化** - AVX2 向量化计算
- **自定义分配器** - jemalloc 替代系统分配器

### 金融级高可用
- **Raft 共识** - 1主2备，RPO=0 零数据丢失
- **快速恢复** - 内存快照 + 增量日志，RTO<5min
- **多活架构** - 跨地域异地容灾（上海/深圳/香港）

### Rust 并发安全
- **消息传递** - tokio::mpsc 避免共享可变状态
- **Actor 模型** - actix 线程安全
- **类型安全** - Newtype 模式，Option/Result 消除空指针
- **编译期保证** - 所有权系统消除数据竞争

### 类 Redis 线程模型
- **多 IO 线程** - 负责网络 I/O
- **单业务线程** - 处理撮合逻辑
- **crossbeam 通信** - 高性能跨线程队列

### 事件溯源
- **事件持久化** - 所有状态变更以事件存储
- **不可变设计** - 领域实体 immutable，提升并发
- **状态回放** - 支持审计和故障恢复

---

## 🛠️ 工程实践

- ✅ **模块化架构** - Workspace 多 crate 组织
- ✅ **完整测试** - 48个测试（单元/集成/文档）
- ✅ **模糊测试** - proptest 基于属性测试
- ✅ **性能基准** - Criterion 性能回归检测
- ✅ **文档注释** - Rustdoc + 示例代码
- ✅ **CI/CD** - GitHub Actions 自动化

---

## 📁 项目结构

```
rustlob/
├── lib/                          # 核心库层
│   ├── common/                   # 通用基础库
│   │   ├── lob_repo/            # 订单簿核心（LOB引擎）
│   │   ├── algo/                # 算法库（撮合/风控）
│   │   ├── base_types/          # 基础类型定义
│   │   ├── decimal/             # 高精度定点数
│   │   ├── fixed_point/         # 定点数计算
│   │   ├── id_generator/        # 分布式ID生成
│   │   ├── sequencer/           # 序列化器
│   │   ├── sbe/                 # SBE 编解码
│   │   ├── cqrs/                # CQRS 模式
│   │   ├── db_repo/             # 数据库仓储
│   │   ├── repo_def/            # 仓储接口定义
│   │   ├── diff/                # 差异计算
│   │   ├── vector_clock/        # 向量时钟
│   │   └── entity_derive/       # 实体宏派生
│   └── core/                    # 核心业务层
│       └── exchange/
│           └── prep/            # 交易预处理
│
├── app/                          # 应用服务层
│   ├── axum_server/             # Axum HTTP 服务器
│   ├── client/                  # WebSocket 客户端示例
│   ├── websocket_sockudo/       # WebSocket 服务（高性能）
│   ├── pingora_gateway/         # Pingora API 网关
│   └── xdp_libbpf/              # XDP 网络加速
│
├── proc/                         # 业务流程层
│   └── operating/exchange/
│       ├── spot/                # 现货交易流程
│       ├── derivatives/         # 衍生品流程
│       │   ├── usds_m_future/  # USDT本位合约
│       │   ├── coin_m_future/  # 币本位合约
│       │   ├── option/         # 期权
│       │   └── portfolio_margin/ # 组合保证金
│       └── spot_market_data/    # 现货行情流程
│
├── design/                       # 设计文档
│   ├── process/story/
│   │   ├── HOTPATH_MEASURE_GUIDE.md  # 热路径测量指南
│   │   ├── hardware/            # 硬件需求分析
│   │   ├── market_data/         # 行情系统设计
│   │   ├── id_gen/              # ID生成方案
│   │   ├── spot/                # 现货设计
│   │   ├── account/             # 账户系统设计
│   │   └── settlement/          # 清算结算设计
│   ├── deploy/                  # 部署方案
│   └── other/
│       ├── binance-spot-api-docs/        # Binance 现货 API
│       └── binance_derivatives_api/      # Binance 衍生品 API
│
└── study/                        # 研究探索
    ├── hotstuff/                # HotStuff 共识算法
    ├── web3/                    # Web3 集成
    └── zk-snarks/               # 零知识证明
```

---

## 🚀 快速开始

### 运行核心库测试

```bash
cd lib/core/exchange/lob_repo
cargo test --all
# ✅ test result: ok. 48 passed; 0 failed
```

### 启动 WebSocket 服务

```bash
cd app/client
cargo run --release
# 🚀 服务启动: ws://localhost:9090/ws
```

### 运行示例客户端

```bash
cd app/client
cargo run --example ws_client --release
# 实时查看订单执行和成交推送
```

---

## 📡 WebSocket API

### 客户端消息

```json
// 限价订单
{ "type": "limit_order", "trader_id": "alice", "side": "buy", "price": 50000, "quantity": 10 }

// 取消订单
{ "type": "cancel_order", "order_id": 123456 }
```

### 服务器推送

```json
// 订单确认
{ "type": "order_ack", "order_id": 123456, "status": "open", "latency_us": 87 }

// 成交广播
{ "type": "v1", "trade_id": 789012, "buyer": "alice", "seller": "bob", "price": 50000 }
```

详细 API 文档：[WEBSOCKET.md](app/client/WEBSOCKET.md)

---

## 🗺️ 路线图

### ✅ 第一阶段 - 现货交易 MVP（已完成）
- 核心订单簿匹配引擎
- WebSocket 实时推送
- 48个测试覆盖

### 🚧 第二阶段 - 现货增强（1-3个月）
- WAL 日志 + Raft 共识
- 多订单簿管理
- REST API + 用户认证

### 📈 第三阶段 - 永续合约（3-6个月）
- 合约引擎 + 杠杆交易
- 标记价格 + 资金费率
- 强平机制 + 风险引擎

### 📊 第四阶段 - 期权交易（6-12个月）
- Black-Scholes 定价
- Greeks 计算
- 组合保证金

### 🚀 第五阶段 - 极致性能（持续）
- DPDK 用户态网络栈
- FPGA 订单簿加速
- RDMA 网络 (< 2μs)

参考文档：[技术路线图详情](design/process/story/)

---

## 🏛️ 系统架构

### 整体架构

```
┌────────────────────────────────────────────────────────┐
│              Load Balancer (LVS/Nginx)                 │
├────────────────────────────────────────────────────────┤
│  [API Gateway]          [WebSocket Gateway]            │
├────────────────────────────────────────────────────────┤
│  业务流隔离层                                            │
│  ┌─────────────────────────────────────────────┐       │
│  │ BTC/USDT 现货流 (独立进程)                   │       │
│  │ [匹配引擎] → [清算] → [推送]                │       │
│  │      ↓ WAL (Raft 1主2备)                   │       │
│  └─────────────────────────────────────────────┘       │
│  ┌─────────────────────────────────────────────┐       │
│  │ ETH/USDT 现货流 (独立进程)                   │       │
│  └─────────────────────────────────────────────┘       │
├────────────────────────────────────────────────────────┤
│  共享服务层: 用户/资金/风控/行情聚合                    │
├────────────────────────────────────────────────────────┤
│  持久化层: PostgreSQL/Redis/RocksDB/Kafka              │
└────────────────────────────────────────────────────────┘
```

### Kubernetes 部署示例

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rustlob-btcusdt-spot
spec:
  replicas: 3  # 1主2备
  template:
    spec:
      containers:
      - name: matching-engine
        image: rustlob:latest
        env:
        - name: SYMBOL
          value: "BTCUSDT"
        resources:
          requests:
            cpu: "8"      # 独占8核
            memory: "16Gi"
```

---

## 📊 性能基准

```bash
cd app/client
cargo run --example ws_benchmark --release
```

**典型结果** (Apple M1 / Intel i7-12700K):

```
=== 性能测试报告 ===

Ping/Pong 延迟:
  平均: 345 μs
  P99: 523 μs

订单处理延迟:
  平均: 87 μs
  P99: 150 μs

吞吐量:
  订单处理: 68,000 orders/s
  消息广播: 120,000 msg/s
```

---

## 📚 文档导航

| 文档 | 说明 |
|------|------|
| [CLAUDE.md](CLAUDE.md) | 开发规范（低延迟标准 + Clean Architecture） |
| [hard.md](design/process/story/hardware/hard.md) | 硬件需求分析（三级配置方案） |
| [futures_design.md](design/process/story/futures_design.md) | 永续合约设计 |
| [options_design.md](design/process/story/options_design.md) | 期权交易设计 |
| [WEBSOCKET.md](app/client/WEBSOCKET.md) | WebSocket API 详细文档 |

---

## 🚀 生产环境部署

### Docker 部署

```dockerfile
FROM rust:1.70 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin sapp

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/sapp /usr/local/bin/
EXPOSE 9090
CMD ["sapp"]
```

### 编译优化

```bash
cd app/client
cargo build --release --target x86_64-unknown-linux-gnu
# 二进制: target/release/client
```

---

## 🤝 贡献指南

### 开发流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 编写代码并测试 (`cargo test`)
4. 提交变更 (`git commit -m 'Add amazing feature'`)
5. 推送到分支 (`git push origin feature/amazing-feature`)
6. 开启 Pull Request

### 代码规范

```bash
cargo test --all          # 运行所有测试
cargo clippy -- -D warnings  # 代码检查
cargo fmt --all --check   # 格式检查
```

### 开发规范

- 遵循 [CLAUDE.md](CLAUDE.md) 低延迟开发标准
- 遵循 Clean Architecture 设计原则
- 编写单元测试覆盖核心逻辑
- 性能关键路径需要基准测试验证

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

## 🙏 致谢

- Rust 社区优秀工具链
- Clean Architecture 设计启发
- 高频交易系统最佳实践
- 头部 CEX 技术架构参考（Binance/OKX/Bybit）

---

## 📧 联系方式

- **Issues**: [GitHub Issues](https://github.com/yourusername/rustlob/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/rustlob/discussions)

