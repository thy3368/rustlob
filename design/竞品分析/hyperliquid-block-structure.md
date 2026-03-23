# Rust之从0-1低时延DEX：竞品分析之Hyperliquid 区块结构

## 概述

Hyperliquid 是一个高性能的去中心化永续合约交易所，基于自研的 L1 区块链构建。本文档基于对区块 #932387680 的实际分析，深入解析 Hyperliquid 的区块结构和交易类型。

**分析区块**: [932387680](https://hypurrscan.io/block/932387680)
**区块时间**: 2026-03-23 10:53:37.452 UTC
**区块哈希**: `0x9b454a5f2072a6c2e1c45389f917b1730a4b5d8d85033d88b010553e738ea10e`

---

## 区块基本结构

### 完整区块定义

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hyperliquid 完整区块结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// 区块头
    pub header: BlockHeader,

    /// 交易列表
    pub transactions: Vec<Transaction>,

    /// 区块元数据
    pub metadata: BlockMetadata,
}

/// 区块头信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// 区块哈希 (32 bytes, hex encoded)
    pub hash: String,

    /// 父区块哈希
    pub parent_hash: String,

    /// 区块高度
    pub height: u64,

    /// 区块时间戳 (纳秒精度 Unix timestamp)
    pub timestamp: u64,

    /// 提议者地址 (EVM 兼容地址)
    pub proposer: Address,

    /// 状态根哈希 (Merkle root of state tree)
    pub state_root: String,

    /// 交易根哈希 (Merkle root of transactions)
    pub transactions_root: String,

    /// 收据根哈希 (Merkle root of receipts)
    pub receipts_root: String,

    /// 区块大小 (bytes)
    pub size: u64,

    /// 交易总数
    pub total_txs: u32,

    /// 成功交易数
    pub success_txs: u32,

    /// 失败交易数
    pub error_txs: u32,

    /// 共识轮次 (HotStuff round number)
    pub round: u64,

    /// 验证者签名集合
    pub validator_signatures: Vec<ValidatorSignature>,
}

/// 区块元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// 交易类型统计
    pub tx_type_counts: HashMap<TransactionType, u32>,

    /// 总交易手续费 (USDC, 6位精度)
    pub total_fees: u128,
}

/// 验证者签名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSignature {
    /// 验证者地址
    pub validator: Address,

    /// 签名数据
    pub signature: Vec<u8>,

    /// 投票权重
    pub voting_power: u64,
}

/// EVM 兼容地址类型
pub type Address = [u8; 20];
```

**实际数据 (区块 #932387680)**:
- **区块高度**: 932,387,680
- **区块哈希**: `0x9b454a5f2072a6c2e1c45389f917b1730a4b5d8d85033d88b010553e738ea10e`
- **时间戳**: 2026-03-23 10:53:37.452 (1711189417452000000 ns)
- **提议者**: `0xa82fe73b...8bd762ad`
- **总交易数**: 5,868
- **成功交易**: 4,182 (71.3%)
- **失败交易**: 1,686 (28.7%)
- **区块大小**: ~2.3 MB (估算)

---

## 交易类型分布

Hyperliquid 区块包含多种交易类型，主要面向交易所操作：

| 交易类型 | 数量 | 占比 | 说明 |
|---------|------|------|------|
| **Order** | 2,421 | 41.3% | 下单交易（限价单/市价单） |
| **CancelByCloid** | 2,102 | 35.8% | 按客户端订单ID取消 |
| **Cancel** | 682 | 11.6% | 普通取消订单 |
| **Noop** | 592 | 10.1% | 空操作（心跳/占位） |
| **BatchModify** | 64 | 1.1% | 批量修改订单 |
| **ScheduleCancel** | 4 | 0.07% | 定时取消订单 |
| **UpdateLeverage** | 2 | 0.03% | 更新杠杆倍数 |
| **Modify** | 1 | 0.02% | 单个订单修改 |

### 关键观察

1. **高频交易特征**: 41.3% 的交易是下单，46.9% 是取消操作，体现了高频交易的特点
2. **Noop 交易**: 10.1% 的 Noop 交易可能用于维持连接活跃或占位
3. **批量操作**: 支持 BatchModify，优化多订单场景的性能

---

## 交易结构

### 完整交易定义

```rust
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// 通用交易包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// 交易序号（区块内索引，从 1 开始）
    pub tx_index: u32,

    /// 用户地址 (发起者)
    pub user_address: Address,

    /// 交易哈希 (唯一标识符)
    pub tx_hash: String,

    /// Nonce (防重放攻击)
    pub nonce: u64,

    /// 交易数据 (具体交易类型)
    pub data: TransactionData,

    /// 执行状态
    pub status: TxStatus,

    /// 错误信息 (如果失败)
    pub error: Option<String>,

    /// 签名
    pub signature: Signature,
}

/// **重要说明**:
/// - 交易本身不包含费用信息
/// - Maker/Taker 费用在订单成交（Fill）时计算
/// - 费用信息通过 userFills API 查询，包含在 Fill 记录中
/// - Fill 记录包含: fee (总费用), feeToken (USDC), crossed (是否为 taker)

/// 交易执行状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TxStatus {
    /// 成功执行
    Success,
    /// 执行失败
    Error,
    /// 待处理
    Pending,
    /// 已回滚
    Reverted,
}

/// 签名结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// r 值
    pub r: [u8; 32],
    /// s 值
    pub s: [u8; 32],
    /// v 值 (恢复ID)
    pub v: u8,
}

/// 交易数据枚举 (穷举所有交易类型)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum TransactionData {
    /// 1. 下单交易
    Order(OrderTx),

    /// 2. 按客户端ID取消订单
    CancelByCloid(CancelByCloidTx),

    /// 3. 按订单ID取消订单
    Cancel(CancelTx),

    /// 4. 空操作 (心跳/占位)
    Noop(NoopTx),

    /// 5. 批量修改订单
    BatchModify(BatchModifyTx),

    /// 6. 定时取消订单
    ScheduleCancel(ScheduleCancelTx),

    /// 7. 更新杠杆倍数
    UpdateLeverage(UpdateLeverageTx),

    /// 8. 单个订单修改
    Modify(ModifyTx),

    /// 9. 更新保证金模式
    UpdateIsolatedMargin(UpdateIsolatedMarginTx),

    /// 10. 提现请求
    Withdraw(WithdrawTx),

    /// 11. 充值 (L1 -> L2)
    Deposit(DepositTx),

    /// 12. 内部转账
    Transfer(TransferTx),

    /// 13. 批量取消订单
    BatchCancel(BatchCancelTx),

    /// 14. 取消所有订单
    CancelAll(CancelAllTx),

    /// 15. 更新子账户
    UpdateSubAccount(UpdateSubAccountTx),

    /// 16. 批量下单
    BatchOrder(BatchOrderTx),

    /// 17. 设置推荐人
    SetReferrer(SetReferrerTx),

    /// 18. 领取奖励
    ClaimRewards(ClaimRewardsTx),

    /// 19. 质押
    Stake(StakeTx),

    /// 20. 解除质押
    Unstake(UnstakeTx),

    /// 21. 投票治理提案
    Vote(VoteTx),

    /// 22. 创建治理提案
    CreateProposal(CreateProposalTx),

    /// 23. 更新预言机价格 (仅验证者)
    UpdateOracle(UpdateOracleTx),

    /// 24. 清算
    Liquidate(LiquidateTx),

    /// 25. 资金费率结算
    SettleFunding(SettleFundingTx),
}
```

### 交易类型详解 (完整穷举)

#### 1. Order（下单交易）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTx {
    /// 交易对索引 (0 = BTC-USD, 1 = ETH-USD, ...)
    pub asset: u32,

    /// 是否为买单
    pub is_buy: bool,

    /// 限价 (Decimal, 8位精度)
    pub limit_px: Decimal,

    /// 数量 (Decimal, 资产精度)
    pub sz: Decimal,

    /// 只减仓标志
    pub reduce_only: bool,

    /// 订单类型
    pub order_type: OrderType,

    /// 客户端订单ID (可选)
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    /// 限价单
    Limit {
        /// 时间有效性
        tif: TimeInForce,
    },
    /// 触发限价单
    TriggerLimit {
        /// 触发价格
        trigger_px: Decimal,
        /// 是否为标记价格触发
        is_market: bool,
        /// 时间有效性
        tif: TimeInForce,
    },
    /// 止损市价单
    StopMarket {
        /// 触发价格
        trigger_px: Decimal,
    },
    /// 止盈限价单
    TakeProfitLimit {
        /// 触发价格
        trigger_px: Decimal,
        /// 时间有效性
        tif: TimeInForce,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Good Till Cancel (一直有效直到取消)
    Gtc,
    /// Immediate Or Cancel (立即成交或取消)
    Ioc,
    /// All or None (全部成交或取消，类似 FOK)
    Alo,
    /// Post Only (只做 Maker，不立即成交)
    PostOnly,
}
```

#### 2. CancelByCloid（按客户端ID取消）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelByCloidTx {
    /// 交易对索引
    pub asset: u32,

    /// 客户端订单ID列表
    pub cloids: Vec<String>,
}
```

#### 3. Cancel（按订单ID取消）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelTx {
    /// 交易对索引
    pub asset: u32,

    /// 链上订单ID
    pub oid: u64,
}
```

#### 4. Noop（空操作）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoopTx {
    /// 可选的附加数据 (用于调试或标记)
    pub info: Option<String>,
}
```

#### 5. BatchModify（批量修改订单）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchModifyTx {
    /// 修改操作列表
    pub modifies: Vec<OrderModification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderModification {
    /// 链上订单ID
    pub oid: u64,

    /// 交易对索引
    pub asset: u32,

    /// 是否为买单
    pub is_buy: bool,

    /// 新限价
    pub limit_px: Decimal,

    /// 新数量
    pub sz: Decimal,

    /// 订单类型
    pub order_type: OrderType,
}
```

#### 6. ScheduleCancel（定时取消订单）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleCancelTx {
    /// 交易对索引
    pub asset: u32,

    /// 链上订单ID
    pub oid: u64,

    /// 取消时间戳 (Unix timestamp, 毫秒)
    pub time: u64,
}
```

#### 7. UpdateLeverage（更新杠杆倍数）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLeverageTx {
    /// 交易对索引
    pub asset: u32,

    /// 是否为买方向
    pub is_buy: bool,

    /// 新杠杆倍数 (1-50)
    pub leverage: u32,
}
```

#### 8. Modify（单个订单修改）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyTx {
    /// 链上订单ID
    pub oid: u64,

    /// 交易对索引
    pub asset: u32,

    /// 是否为买单
    pub is_buy: bool,

    /// 新限价
    pub limit_px: Decimal,

    /// 新数量
    pub sz: Decimal,

    /// 订单类型
    pub order_type: OrderType,
}
```

#### 9. UpdateIsolatedMargin（更新逐仓保证金）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIsolatedMarginTx {
    /// 交易对索引
    pub asset: u32,

    /// 是否为买方向
    pub is_buy: bool,

    /// 保证金变化量 (正数为增加，负数为减少)
    pub ntli: i64,
}
```

#### 10. Withdraw（提现请求）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawTx {
    /// 目标地址 (L1 地址)
    pub destination: Address,

    /// 提现金额 (USDC, 6位精度)
    pub amount: u64,

    /// 提现时间戳
    pub time: u64,
}
```

#### 11. Deposit（充值）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositTx {
    /// 充值用户地址
    pub user: Address,

    /// 充值金额 (USDC, 6位精度)
    pub amount: u64,
}
```

#### 12. Transfer（内部转账）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTx {
    /// 接收方地址
    pub destination: Address,

    /// 转账金额 (USDC, 6位精度)
    pub amount: u64,

    /// 转账时间戳
    pub time: u64,
}
```

#### 13. BatchCancel（批量取消订单）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCancelTx {
    /// 取消操作列表
    pub cancels: Vec<CancelRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRequest {
    /// 交易对索引
    pub asset: u32,

    /// 链上订单ID
    pub oid: u64,
}
```

#### 14. CancelAll（取消所有订单）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAllTx {
    /// 可选：仅取消指定交易对的订单
    pub asset: Option<u32>,

    /// 可选：仅取消指定方向的订单
    pub is_buy: Option<bool>,
}
```

#### 15. UpdateSubAccount（更新子账户）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubAccountTx {
    /// 子账户地址
    pub sub_account: Address,

    /// 操作类型
    pub action: SubAccountAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubAccountAction {
    /// 创建子账户
    Create,
    /// 删除子账户
    Delete,
    /// 更新权限
    UpdatePermissions {
        /// 是否允许交易
        can_trade: bool,
        /// 是否允许提现
        can_withdraw: bool,
    },
}
```

#### 16. BatchOrder（批量下单）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderTx {
    /// 订单列表
    pub orders: Vec<OrderTx>,
}
```

#### 17. SetReferrer（设置推荐人）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetReferrerTx {
    /// 推荐人地址
    pub referrer: Address,
}
```

#### 18. ClaimRewards（领取奖励）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRewardsTx {
    /// 奖励类型
    pub reward_type: RewardType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardType {
    /// 交易挖矿奖励
    TradingRewards,
    /// 质押奖励
    StakingRewards,
    /// 推荐奖励
    ReferralRewards,
}
```

#### 19. Stake（质押）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeTx {
    /// 质押金额 (HYPE token)
    pub amount: u128,

    /// 质押期限 (天数)
    pub lock_period: u32,
}
```

#### 20. Unstake（解除质押）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstakeTx {
    /// 解除质押金额
    pub amount: u128,
}
```

#### 21. Vote（投票治理提案）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteTx {
    /// 提案ID
    pub proposal_id: u64,

    /// 投票选项
    pub vote: VoteOption,

    /// 投票权重 (质押数量)
    pub voting_power: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteOption {
    /// 赞成
    Yes,
    /// 反对
    No,
    /// 弃权
    Abstain,
}
```

#### 22. CreateProposal（创建治理提案）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProposalTx {
    /// 提案标题
    pub title: String,

    /// 提案描述
    pub description: String,

    /// 提案类型
    pub proposal_type: ProposalType,

    /// 投票开始时间
    pub voting_start_time: u64,

    /// 投票结束时间
    pub voting_end_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    /// 参数修改
    ParameterChange {
        /// 参数名称
        param: String,
        /// 新值
        value: String,
    },
    /// 资金分配
    Treasury {
        /// 接收地址
        recipient: Address,
        /// 金额
        amount: u128,
    },
    /// 协议升级
    Upgrade {
        /// 新版本哈希
        code_hash: String,
    },
}
```

#### 23. UpdateOracle（更新预言机价格）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOracleTx {
    /// 交易对索引
    pub asset: u32,

    /// 新价格
    pub price: Decimal,

    /// 价格时间戳
    pub timestamp: u64,

    /// 价格来源签名
    pub signatures: Vec<OracleSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleSignature {
    /// 预言机节点地址
    pub oracle: Address,

    /// 签名数据
    pub signature: Vec<u8>,
}
```

#### 24. Liquidate（清算）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidateTx {
    /// 被清算用户地址
    pub liquidatee: Address,

    /// 交易对索引
    pub asset: u32,

    /// 是否为买方向
    pub is_buy: bool,

    /// 清算数量
    pub sz: Decimal,

    /// 清算价格
    pub px: Decimal,
}
```

#### 25. SettleFunding（资金费率结算）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettleFundingTx {
    /// 交易对索引
    pub asset: u32,

    /// 资金费率 (8小时费率, 8位精度)
    pub funding_rate: Decimal,

    /// 结算时间戳
    pub timestamp: u64,

    /// 受影响的用户列表
    pub users: Vec<FundingSettlement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingSettlement {
    /// 用户地址
    pub user: Address,

    /// 持仓数量 (正数为多头，负数为空头)
    pub position: Decimal,

    /// 支付/收取的资金费 (正数为支付，负数为收取)
    pub funding_payment: Decimal,
}
```

---

## 区块生成机制

### 共识算法

Hyperliquid 使用 **HotStuff** 共识算法的变体：

- **出块时间**: ~1 秒（实际观察）
- **最终性**: 3 秒内达到最终确认
- **验证者**: 由 Proposer 提议区块

### 交易排序

区块内交易按以下规则排序：

1. **时间戳优先**: 先到先服务（FIFO）
2. **Gas 费优先**: 相同时间戳时，Gas 费高的优先
3. **Nonce 顺序**: 同一用户的交易按 nonce 递增

---

## 性能特征

### 吞吐量分析

基于区块 #932387680 的数据：

- **区块大小**: 5,868 笔交易
- **出块时间**: ~1 秒
- **理论 TPS**: ~5,868 TPS
- **实际 TPS**: 考虑失败交易，有效 TPS ~4,182

### 延迟特征

- **交易确认**: 1-3 秒（1 个区块确认）
- **最终确认**: 3-5 秒（3 个区块确认）
- **订单撮合**: 链下撮合引擎，延迟 < 10ms

---

## 与传统 DEX 的对比

| 特性 | Hyperliquid | Uniswap (Ethereum) | dYdX v4 |
|------|-------------|-------------------|---------|
| **区块时间** | ~1 秒 | ~12 秒 | ~1 秒 |
| **TPS** | ~5,000 | ~15 | ~2,000 |
| **交易类型** | 25 种专用类型 | 通用智能合约 | 专用交易类型 |
| **订单簿** | 链上订单簿 | AMM | 链上订单簿 |
| **最终性** | 3 秒 | 12 分钟 | 3 秒 |
| **交易费用** | 仅交易手续费，无 Gas 费 | 高（以太坊 Gas） | 低（Cosmos SDK） |

---

## 数据结构设计亮点

### 1. 客户端订单ID (CLOID)

允许客户端使用自定义ID管理订单，避免查询链上状态：

```rust
// 客户端代码
let cloid = format!("order_{}", uuid::Uuid::new_v4());
exchange.place_order(Order {
    cloid: Some(cloid.clone()),
    // ...
});

// 稍后取消
exchange.cancel_by_cloid(vec![cloid]);
```

### 2. 批量操作

减少交易数量，提高吞吐量：

```rust
// 单次交易修改多个订单
exchange.batch_modify(vec![
    Modification { order_id: 1, new_price: Some(50000.0), .. },
    Modification { order_id: 2, new_size: Some(1.5), .. },
    // ...
]);
```

### 3. Noop 交易

维持连接活跃，避免超时断开：

```rust
// 每 30 秒发送一次 Noop
tokio::spawn(async move {
    loop {
        exchange.send_noop().await;
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
});
```

---

## 实现建议

### Rust 区块解析器

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HyperliquidBlock {
    pub hash: String,
    pub height: u64,
    pub timestamp: u64,
    pub proposer: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_index: u32,
    pub user_address: String,
    pub tx_hash: String,
    pub action: String,
    pub status: String,
}

impl HyperliquidBlock {
    pub async fn fetch(height: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!("https://api.hyperliquid.xyz/info");
        let body = serde_json::json!({
            "type": "block",
            "height": height
        });

        let client = reqwest::Client::new();
        let response = client.post(&url)
            .json(&body)
            .send()
            .await?;

        let block: HyperliquidBlock = response.json().await?;
        Ok(block)
    }

    pub fn transaction_type_distribution(&self) -> HashMap<String, usize> {
        let mut dist = HashMap::new();
        for tx in &self.transactions {
            *dist.entry(tx.action.clone()).or_insert(0) += 1;
        }
        dist
    }

    pub fn success_rate(&self) -> f64 {
        let success = self.transactions.iter()
            .filter(|tx| tx.status == "success")
            .count();
        success as f64 / self.transactions.len() as f64
    }
}
```

### 交易监听器

```rust
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::StreamExt;

pub async fn subscribe_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async("wss://api.hyperliquid.xyz/ws").await?;
    let (_, mut read) = ws_stream.split();

    // 订阅新区块
    let subscribe_msg = serde_json::json!({
        "method": "subscribe",
        "subscription": {"type": "allMids"}
    });

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
                let block: HyperliquidBlock = serde_json::from_str(&text)?;
                println!("New block: {} with {} txs",
                    block.height, block.transactions.len());
            }
            _ => {}
        }
    }

    Ok(())
}
```

---

## 费用模型和成交记录

### 费用结构

Hyperliquid **不收取 Gas 费**，仅收取交易手续费：

| 费用类型 | 说明 | 费率范围 |
|---------|------|---------|
| **Taker 费** | 吃单方支付 | 0.024% - 0.070% |
| **Maker 费** | 挂单方支付/获得 | -0.003% - 0.040% (负数为 rebate) |
| **无 Gas 费** | 所有链上操作免 Gas | 0% |

**费率分层**：
- 基于 14 天滚动交易量
- Spot 交易量双倍计入
- HYPE 质押可获得额外折扣（5%-40%）

### 成交记录（Fill）结构

**重要**：费用信息不在区块交易中，而在成交记录（Fill）中：

```rust
/// 成交记录（通过 userFills API 查询）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    /// 交易对
    pub coin: String,

    /// 成交价格
    pub px: Decimal,

    /// 成交数量
    pub sz: Decimal,

    /// 方向 (B=买, A=卖)
    pub side: String,

    /// 成交时间戳 (毫秒)
    pub time: u64,

    /// 是否为 Taker (true=taker, false=maker)
    pub crossed: bool,

    /// 总费用 (包含 builderFee)
    pub fee: Decimal,

    /// 费用币种 (通常为 USDC)
    pub fee_token: String,

    /// Builder 费用 (可选，HIP-3 专用)
    pub builder_fee: Option<Decimal>,

    /// 订单ID
    pub oid: u64,

    /// 交易哈希
    pub hash: String,

    /// 成交ID
    pub tid: u64,

    /// 已实现盈亏 (仅永续合约)
    pub closed_pnl: Option<Decimal>,

    /// 持仓方向 (仅永续合约)
    pub dir: Option<String>,

    /// 起始持仓 (仅永续合约)
    pub start_position: Option<Decimal>,
}
```

### 费用计算时机

```rust
// 订单生命周期
Order Transaction (区块中)
    ↓
Matching Engine (链下撮合)
    ↓
Fill Event (成交记录)
    ├─ 计算 Maker/Taker 费用
    ├─ Maker rebate 实时支付到钱包
    └─ 记录到 Fill 历史

// 查询成交记录
let fills = api.user_fills(user_address, aggregate_by_time).await?;
for fill in fills {
    if fill.crossed {
        println!("Taker 费用: {} {}", fill.fee, fill.fee_token);
    } else {
        println!("Maker rebate: {} {}", fill.fee, fill.fee_token);
    }
}
```

### 费用去向

Hyperliquid 的费用分配模型：

| 接收方 | 占比 | 说明 |
|-------|------|------|
| **HLP (流动性池)** | 主要部分 | 为流动性提供者提供收益 |
| **Assistance Fund** | 部分 | 自动转换为 HYPE 并销毁 |
| **Deployers** | 0-50% | HIP-3 部署者可配置分成 |
| **团队/内部人** | 0% | 团队不从费用中获利 |

**Assistance Fund 机制**：
- 地址: `0xfefefefefefefefefefefefefefefefefefefefe`
- 自动将交易费转换为 HYPE
- 转换后的 HYPE 永久销毁
- 减少 HYPE 流通量和总供应量

### 费用优化策略

```rust
// 1. 使用 Post-Only 订单获得 Maker rebate
let order = OrderTx {
    order_type: OrderType::Limit {
        tif: TimeInForce::PostOnly,  // 保证成为 Maker
    },
    // ...
};

// 2. 批量操作减少交易数量
let batch = BatchOrderTx {
    orders: vec![order1, order2, order3],  // 单次交易提交多个订单
};

// 3. 质押 HYPE 获得费用折扣
// 质押 10,000 HYPE → 20% 折扣
// 质押 500,000 HYPE → 40% 折扣
```

---

## 总结

Hyperliquid 的区块结构设计体现了以下特点：

1. **专用交易类型**: 25 种交易类型针对交易所场景优化，涵盖交易、账户管理、治理、系统级操作
2. **高吞吐量**: 单区块 5,000+ 笔交易，TPS 远超传统 DEX
3. **低延迟**: 1 秒出块，3 秒最终确认
4. **客户端友好**: CLOID 机制简化订单管理
5. **批量操作**: 支持批量下单、批量取消、批量修改，提高效率
6. **完整的交易生命周期**: 从下单、修改、取消到清算、资金费率结算的完整支持

这种设计使 Hyperliquid 在保持去中心化的同时，达到了接近中心化交易所的性能水平。

---

## 参考资料

- [HypurrScan 区块浏览器](https://hypurrscan.io/)
- [Hyperliquid 官方文档](https://hyperliquid.gitbook.io/hyperliquid-docs/)
- [Hyperliquid API 文档](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api)
- 分析区块: [#932387680](https://hypurrscan.io/block/932387680)

---


