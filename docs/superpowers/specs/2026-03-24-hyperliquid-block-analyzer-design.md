# Hyperliquid 区块分析器设计文档

**日期**: 2026-03-24
**版本**: v1.0.0
**作者**: Claude & Hongyao Tang

---

## 1. 项目概述

### 1.1 目标

开发一个 Rust 工具，用于下载和分析 Hyperliquid L1 区块链的单个区块数据，为开发类似的 L1 链撮合系统提供参考。通过逆向工程区块数据，推导出 Hyperliquid 的领域模型和系统设计。

### 1.2 核心功能

- 下载指定高度的区块数据
- 解析区块头、交易、订单等所有数据
- 实时终端输出完整的区块信息明细
- 提供可复用的 Library API

### 1.3 非目标（本阶段）

- ❌ 批量下载历史区块
- ❌ 实时流式监听
- ❌ 数据库持久化
- ❌ 账户级分析
- ❌ 协议级分析

---

## 2. 架构设计

### 2.1 项目结构

```
study/hyperliquid_analyzer/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                 # Library 入口，导出公共 API
│   ├── types.rs               # 数据类型定义
│   ├── client.rs              # HTTP 客户端
│   ├── parser.rs              # 区块解析器
│   ├── analyzer.rs            # 区块分析器
│   ├── reporter.rs            # 终端输出格式化
│   └── bin/
│       └── hl_analyzer.rs     # CLI 工具
└── examples/
    └── analyze_block.rs       # 使用示例
```

### 2.2 模块职责

#### **types.rs** - 数据类型
定义 Hyperliquid 的所有数据结构，基于已有的区块结构文档：
- `Block` - 完整区块
- `BlockHeader` - 区块头
- `Transaction` - 交易
- `TransactionData` - 交易数据枚举（25 种类型）
- `TxStatus` - 交易状态

使用 `serde` 进行 JSON 序列化/反序列化。

#### **client.rs** - HTTP 客户端
封装 Hyperliquid API 调用：
- `fetch_block(height: u64) -> Result<Block>` - 下载指定区块
- `fetch_latest_block() -> Result<Block>` - 下载最新区块
- 错误处理和重试机制（最多 3 次）
- 超时设置（30 秒）

#### **parser.rs** - 解析器
解析区块 JSON 数据：
- 解析区块头信息
- 解析交易列表
- 提取关键字段
- 数据验证

#### **analyzer.rs** - 分析引擎
计算统计指标：
- 交易类型分布
- 成功率统计
- 活跃用户统计
- 交易对分布
- 失败原因分析

#### **reporter.rs** - 终端输出
格式化输出分析结果：
- 区块头信息
- 交易统计
- 交易明细列表
- 失败交易分析
- 活跃用户 Top 10
- 性能指标

使用 `colored` crate 实现彩色输出。

#### **bin/hl_analyzer.rs** - CLI 工具
命令行接口：
- 参数解析（使用 `clap` crate）
- 调用 library 功能
- 错误处理和用户提示

---

## 3. 数据模型

### 3.1 核心数据结构

```rust
/// 完整区块
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

/// 区块头
pub struct BlockHeader {
    pub hash: String,
    pub parent_hash: String,
    pub height: u64,
    pub timestamp: u64,
    pub proposer: String,
    pub state_root: String,
    pub transactions_root: String,
    pub receipts_root: String,
    pub size: u64,
    pub total_txs: u32,
    pub success_txs: u32,
    pub error_txs: u32,
    pub round: u64,
}

/// 交易
pub struct Transaction {
    pub tx_index: u32,
    pub user_address: String,
    pub tx_hash: String,
    pub nonce: u64,
    pub data: TransactionData,
    pub status: TxStatus,
    pub error: Option<String>,
}

/// 交易数据枚举
#[serde(tag = "type")]
pub enum TransactionData {
    Order(OrderTx),
    Cancel(CancelTx),
    CancelByCloid(CancelByCloidTx),
    Noop(NoopTx),
    BatchModify(BatchModifyTx),
    ScheduleCancel(ScheduleCancelTx),
    UpdateLeverage(UpdateLeverageTx),
    Modify(ModifyTx),
    // ... 其他 17 种类型
}

/// 交易状态
pub enum TxStatus {
    Success,
    Error,
}
```

### 3.2 订单交易详细结构

```rust
/// Order 交易
pub struct OrderTx {
    pub asset: u32,              // 交易对索引
    pub is_buy: bool,            // 是否买单
    pub limit_px: Decimal,       // 限价
    pub sz: Decimal,             // 数量
    pub reduce_only: bool,       // 只减仓
    pub order_type: OrderType,   // 订单类型
    pub cloid: Option<String>,   // 客户端订单ID
}

/// 订单类型
pub enum OrderType {
    Limit { tif: TimeInForce },
    TriggerLimit { trigger_px: Decimal, is_market: bool, tif: TimeInForce },
    StopMarket { trigger_px: Decimal },
    TakeProfitLimit { trigger_px: Decimal, tif: TimeInForce },
}

/// 时间有效性
pub enum TimeInForce {
    Gtc,       // Good Till Cancel
    Ioc,       // Immediate Or Cancel
    Alo,       // All or None
    PostOnly,  // 只做 Maker
}
```

---

## 4. API 设计

### 4.1 Hyperliquid API

**端点**: `https://api.hyperliquid.xyz/info`

**请求格式**:
```json
POST /info
Content-Type: application/json

{
  "type": "block",
  "height": 932387680
}
```

**响应格式**:
```json
{
  "hash": "0x9b454a5f...",
  "height": 932387680,
  "timestamp": 1711189417452000000,
  "proposer": "0xa82fe73b...",
  "transactions": [
    {
      "tx_index": 1,
      "user_address": "0x1234...",
      "tx_hash": "0xabc123...",
      "nonce": 12345,
      "data": {
        "type": "Order",
        "asset": 0,
        "is_buy": true,
        "limit_px": "50000.0",
        "sz": "1.5",
        ...
      },
      "status": "success"
    },
    ...
  ]
}
```

### 4.2 Library API

```rust
// 下载区块
pub async fn fetch_block(height: u64) -> Result<Block, ClientError>;
pub async fn fetch_latest_block() -> Result<Block, ClientError>;

// 解析区块
pub fn parse_block(json: &str) -> Result<Block, ParseError>;

// 分析区块
pub fn analyze_block(block: &Block) -> BlockAnalysis;

// 格式化输出
pub fn format_block_report(block: &Block, analysis: &BlockAnalysis) -> String;
```

---

## 5. CLI 设计

### 5.1 命令格式

```bash
hl_analyzer block <HEIGHT> [OPTIONS]
```

### 5.2 参数说明

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `<HEIGHT>` | 区块高度，或 "latest" | 必需 |
| `--verbose, -v` | 显示所有交易详情 | false |
| `--limit <N>` | 限制显示交易数量 | 20 |
| `--export <FORMAT>` | 导出数据 (json\|csv) | - |
| `--filter <TYPE>` | 只显示特定类型交易 | - |
| `--show-failed` | 只显示失败交易 | false |
| `--show-success` | 只显示成功交易 | false |

### 5.3 使用示例

```bash
# 基本分析
hl_analyzer block 932387680

# 详细模式（显示所有交易）
hl_analyzer block 932387680 --verbose

# 显示前 100 笔交易
hl_analyzer block 932387680 --limit 100

# 只显示 Order 交易
hl_analyzer block 932387680 --filter Order

# 只显示失败交易
hl_analyzer block 932387680 --show-failed

# 导出为 JSON
hl_analyzer block 932387680 --export json > block.json

# 分析最新区块
hl_analyzer block latest
```

---

## 6. 输出格式

### 6.1 终端输出结构

```
╔══════════════════════════════════════════════════════════╗
║        区块 #932387680 完整分析                          ║
╚══════════════════════════════════════════════════════════╝

📦 区块头信息
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  区块高度:     932,387,680
  区块哈希:     0x9b454a5f...
  时间戳:       2026-03-23 10:53:37.452 UTC
  提议者:       0xa82fe73b...
  ...

📊 交易统计
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  总交易数:     5,868
  成功交易:     4,182 (71.3%)
  失败交易:     1,686 (28.7%)

📈 交易类型分布
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Order           2,421 (41.3%) ████████████████████
  CancelByCloid   2,102 (35.8%) ██████████████████
  ...

📝 交易明细 (前 20 笔)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#    类型            用户地址          状态    详情
──────────────────────────────────────────────────────────────
1    Order          0x1234...5678    ✓       BTC-USD Buy 1.5 @ 50000.0
2    Order          0xabcd...ef01    ✓       ETH-USD Sell 10.0 @ 3000.0
...

❌ 失败交易分析
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  总失败数: 1,686

  失败原因分布:
    insufficient margin        756 (44.8%)
    invalid price              389 (23.1%)
    ...

👥 活跃用户分析
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  独立用户数:   3,456

  Top 10 活跃用户:
    #1  0x1234...5678    234 txs (4.0%)
    ...

📊 交易对分析
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  BTC-USD (asset: 0)    3,245 txs (55.3%)
  ETH-USD (asset: 1)    1,876 txs (32.0%)
  ...

⏱️ 性能指标
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  区块间隔:     1.013s
  TPS:          5,868 tx/s
  有效 TPS:     4,182 tx/s
  ...
```

### 6.2 JSON 导出格式

```json
{
  "block": {
    "header": { ... },
    "transactions": [ ... ]
  },
  "analysis": {
    "total_txs": 5868,
    "success_txs": 4182,
    "error_txs": 1686,
    "tx_type_distribution": {
      "Order": 2421,
      "CancelByCloid": 2102,
      ...
    },
    "error_distribution": {
      "insufficient margin": 756,
      "invalid price": 389,
      ...
    },
    "top_users": [ ... ],
    "asset_distribution": { ... }
  }
}
```

---

## 7. 技术栈

### 7.1 核心依赖

| Crate | 版本 | 用途 |
|-------|------|------|
| `tokio` | 1.x | 异步运行时 |
| `reqwest` | 0.11 | HTTP 客户端 |
| `serde` | 1.x | 序列化/反序列化 |
| `serde_json` | 1.x | JSON 处理 |
| `rust_decimal` | 1.x | 高精度小数 |
| `clap` | 4.x | CLI 参数解析 |
| `colored` | 2.x | 彩色终端输出 |
| `anyhow` | 1.x | 错误处理 |
| `thiserror` | 1.x | 自定义错误类型 |

### 7.2 Cargo.toml

```toml
[package]
name = "hyperliquid_analyzer"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rust_decimal = "1"
clap = { version = "4", features = ["derive"] }
colored = "2"
anyhow = "1"
thiserror = "1"

[[bin]]
name = "hl_analyzer"
path = "src/bin/hl_analyzer.rs"
```

---

## 8. 性能要求

基于 CLAUDE.md 中的低延迟标准：

### 8.1 内存优化
- 单区块数据 < 10 MB 内存占用
- 零拷贝解析（使用 `serde` 的 zero-copy 特性）
- 避免不必要的字符串克隆

### 8.2 网络优化
- HTTP 连接复用
- 请求超时 30 秒
- 失败重试最多 3 次，指数退避

### 8.3 解析性能
- JSON 解析 < 100ms（5000+ 交易）
- 统计计算 < 50ms
- 终端输出 < 10ms

---

## 9. 错误处理

### 9.1 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Block not found: {0}")]
    BlockNotFound(u64),

    #[error("API rate limit exceeded")]
    RateLimitExceeded,

    #[error("Network timeout")]
    Timeout,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid block format: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}
```

### 9.2 错误处理策略

- 网络错误：自动重试 3 次
- 解析错误：返回详细错误信息
- 用户输入错误：友好提示

---

## 10. 测试策略

### 10.1 单元测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_block_header() {
        let json = r#"{"hash": "0x...", "height": 123, ...}"#;
        let header = parse_block_header(json).unwrap();
        assert_eq!(header.height, 123);
    }

    #[test]
    fn test_analyze_tx_distribution() {
        let block = create_test_block();
        let analysis = analyze_block(&block);
        assert_eq!(analysis.total_txs, 100);
    }
}
```

### 10.2 集成测试

```rust
#[tokio::test]
async fn test_fetch_real_block() {
    let block = fetch_block(932387680).await.unwrap();
    assert_eq!(block.header.height, 932387680);
    assert!(block.transactions.len() > 0);
}
```

### 10.3 性能测试

```rust
#[bench]
fn bench_parse_large_block(b: &mut Bencher) {
    let json = load_test_block_json();
    b.iter(|| {
        parse_block(&json).unwrap()
    });
}
```

---

## 11. 开发计划

### 11.1 阶段 1：基础框架（1 天）
- [x] 项目结构搭建
- [ ] 数据类型定义（types.rs）
- [ ] HTTP 客户端实现（client.rs）
- [ ] 基本错误处理

### 11.2 阶段 2：核心功能（1 天）
- [ ] 区块解析器（parser.rs）
- [ ] 分析引擎（analyzer.rs）
- [ ] 单元测试

### 11.3 阶段 3：CLI 工具（0.5 天）
- [ ] CLI 参数解析
- [ ] 终端输出格式化（reporter.rs）
- [ ] 错误提示优化

### 11.4 阶段 4：测试和优化（0.5 天）
- [ ] 集成测试
- [ ] 性能优化
- [ ] 文档完善

**总计**: 3 天

---

## 12. 未来扩展

### 12.1 短期扩展（v1.1）
- 批量下载区块（指定范围）
- 区块间对比分析
- 导出 CSV 格式

### 12.2 中期扩展（v1.2）
- 账户级分析（指定地址的交易历史）
- 持仓分析
- 余额变更追踪

### 12.3 长期扩展（v2.0）
- 领域模型逆向工程
- 状态机分析
- 流程模型推导
- WebSocket 实时监听

---

## 13. 参考资料

- [Hyperliquid 官方文档](https://hyperliquid.gitbook.io/hyperliquid-docs/)
- [Hyperliquid API 文档](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api)
- [HypurrScan 区块浏览器](https://hypurrscan.io/)
- [区块结构分析文档](../../../proc/operating/dex/竞品分析/hyperliquid-block-structure.md)

---

## 附录 A：完整交易类型列表

1. Order - 下单
2. CancelByCloid - 按客户端ID取消
3. Cancel - 按订单ID取消
4. Noop - 空操作
5. BatchModify - 批量修改
6. ScheduleCancel - 定时取消
7. UpdateLeverage - 更新杠杆
8. Modify - 单个修改
9. UpdateIsolatedMargin - 更新逐仓保证金
10. Withdraw - 提现
11. Deposit - 充值
12. Transfer - 内部转账
13. BatchCancel - 批量取消
14. CancelAll - 取消所有
15. UpdateSubAccount - 更新子账户
16. BatchOrder - 批量下单
17. SetReferrer - 设置推荐人
18. ClaimRewards - 领取奖励
19. Stake - 质押
20. Unstake - 解除质押
21. Vote - 投票
22. CreateProposal - 创建提案
23. UpdateOracle - 更新预言机
24. Liquidate - 清算
25. SettleFunding - 资金费率结算

---

**文档版本**: v1.0.0
**最后更新**: 2026-03-24
**下次审查**: 实现完成后
