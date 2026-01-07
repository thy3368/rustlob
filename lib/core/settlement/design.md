# Settlement Module Design Document

## 1. Overview

Settlement module handles fund clearing and position settlement after trade execution. This module supports two trading instruments:
- **Spot**: Instant delivery, involving asset transfers
- **Prep (Perpetual)**: Margin trading, involving position management, PnL calculation, liquidation mechanism

## 2. Command Design

### 2.1 Priority Hierarchy

| Priority | Category | Commands |
|----------|----------|----------|
| P0 | Core Settlement | SettleSpotTrade, SettlePrepOpen, SettlePrepClose, SettlePrepReduce |
| P1 | Funding Settlement | SettleFunding |
| P2 | Risk Processing | SettleLiquidation, SettleADL |
| P3 | Admin Operations | AdjustMargin, ReverseSettlement, BatchSettle |

> **设计原则**:
> - P0 核心命令内含保证金锁定/释放、盈亏结算逻辑，不单独暴露
> - `LockMargin/ReleaseMargin/RealizePnl` 等作为内部操作，由 P0 命令处理时调用
> - `ReconcileAccount` 移至独立对账模块

### 2.2 Settlement Command Definition

```rust
//! Settlement Command Definitions
//!
//! ## Priority Hierarchy
//!
//! - P0: Core Settlement (SettleSpotTrade, SettlePrepOpen, SettlePrepClose, SettlePrepReduce)
//! - P1: Funding Settlement (SettleFunding)
//! - P2: Risk Processing (SettleLiquidation, SettleADL)
//! - P3: Admin Operations (AdjustMargin, ReverseSettlement, BatchSettle)
//!
//! ## 设计原则
//!
//! - 统一使用 AssetId(u32) 替代 Asset(String)，高性能
//! - P0 命令内含保证金/盈亏逻辑，不单独暴露 LockMargin/RealizePnl
//! - 每个命令生成对应的 BalanceCommand 批次

use crate::domain::entity::{
    SettlementId, ClearingId, AccountId, TradeId, PositionId, TraderId,
    AssetId, Price, Quantity, Amount, Timestamp,
    Side, PositionSide,
};

/// Settlement Commands
#[derive(Debug, Clone)]
pub enum Command {
    // ==================== P0: Core Settlement ====================

    /// 现货交易结算
    ///
    /// 生成 BalanceCommand:
    /// - Buyer: TransferOut(quote) + TransferIn(base) + TradingFee
    /// - Seller: TransferOut(base) + TransferIn(quote) + TradingFee
    SettleSpotTrade {
        /// 交易ID
        trade_id: TradeId,
        /// 买方账户
        buyer_account: AccountId,
        /// 卖方账户
        seller_account: AccountId,
        /// 基础资产 (如 BTC)
        base_asset_id: AssetId,
        /// 计价资产 (如 USDT)
        quote_asset_id: AssetId,
        /// 成交价格
        price: Price,
        /// 成交数量
        quantity: Quantity,
        /// 买方手续费
        buyer_fee: Amount,
        /// 卖方手续费
        seller_fee: Amount,
        /// 买方是否 Maker
        buyer_is_maker: bool,
    },

    /// 永续开仓结算
    ///
    /// 生成 BalanceCommand:
    /// - Freeze(margin) + TradingFee
    ///
    /// 不变量:
    /// - margin = notional / leverage
    /// - available_balance >= margin + fee
    SettlePrepOpen {
        /// 交易ID
        trade_id: TradeId,
        /// 交易者账户
        account: AccountId,
        /// 仓位ID
        position_id: PositionId,
        /// 保证金资产
        margin_asset_id: AssetId,
        /// 交易方向 (Buy=开多, Sell=开空)
        side: Side,
        /// 仓位方向
        position_side: PositionSide,
        /// 开仓价格
        price: Price,
        /// 开仓数量
        quantity: Quantity,
        /// 名义价值 (price * quantity)
        notional: Amount,
        /// 所需保证金 (notional / leverage)
        margin: Amount,
        /// 杠杆倍数
        leverage: u32,
        /// 交易手续费
        fee: Amount,
        /// 是否 Maker
        is_maker: bool,
    },

    /// 永续平仓结算（全部平仓）
    ///
    /// 生成 BalanceCommand:
    /// - Unfreeze(margin) + RealizedPnl + TradingFee
    ///
    /// 不变量:
    /// - quantity == position.quantity
    /// - realized_pnl = (exit_price - entry_price) * quantity * direction
    SettlePrepClose {
        /// 交易ID
        trade_id: TradeId,
        /// 交易者账户
        account: AccountId,
        /// 仓位ID
        position_id: PositionId,
        /// 保证金资产
        margin_asset_id: AssetId,
        /// 交易方向 (Sell=平多, Buy=平空)
        side: Side,
        /// 仓位方向
        position_side: PositionSide,
        /// 平仓价格
        price: Price,
        /// 平仓数量
        quantity: Quantity,
        /// 开仓均价
        entry_price: Price,
        /// 释放保证金
        released_margin: Amount,
        /// 已实现盈亏 (正=盈利, 负=亏损)
        realized_pnl: i64,
        /// 交易手续费
        fee: Amount,
        /// 是否 Maker
        is_maker: bool,
    },

    /// 永续减仓结算（部分平仓）
    ///
    /// 生成 BalanceCommand:
    /// - Unfreeze(released_margin) + RealizedPnl + TradingFee
    ///
    /// 不变量:
    /// - reduce_quantity < position.quantity
    /// - released_margin = position.margin * (reduce_quantity / position.quantity)
    SettlePrepReduce {
        /// 交易ID
        trade_id: TradeId,
        /// 交易者账户
        account: AccountId,
        /// 仓位ID
        position_id: PositionId,
        /// 保证金资产
        margin_asset_id: AssetId,
        /// 交易方向
        side: Side,
        /// 仓位方向
        position_side: PositionSide,
        /// 平仓价格
        price: Price,
        /// 减仓数量
        reduce_quantity: Quantity,
        /// 剩余数量
        remaining_quantity: Quantity,
        /// 开仓均价
        entry_price: Price,
        /// 释放保证金
        released_margin: Amount,
        /// 剩余保证金
        remaining_margin: Amount,
        /// 已实现盈亏
        realized_pnl: i64,
        /// 交易手续费
        fee: Amount,
        /// 是否 Maker
        is_maker: bool,
    },

    // ==================== P1: Funding Settlement ====================

    /// 资金费率结算
    ///
    /// 生成 BalanceCommand:
    /// - FundingFee (多笔，可正可负)
    ///
    /// funding_rate > 0: 多头付给空头
    /// funding_rate < 0: 空头付给多头
    SettleFunding {
        /// 结算时间戳
        settlement_time: Timestamp,
        /// 资金费率 (1e8 精度)
        funding_rate: i64,
        /// 结算条目
        entries: Vec<FundingEntry>,
    },

    // ==================== P2: Risk Processing ====================

    /// 强平结算
    ///
    /// 生成 BalanceCommand:
    /// - DebitFrozen(remaining_margin) + LiquidationFee + InsuranceFundIn
    SettleLiquidation {
        /// 仓位ID
        position_id: PositionId,
        /// 账户ID
        account: AccountId,
        /// 保证金资产
        margin_asset_id: AssetId,
        /// 强平价格
        liquidation_price: Price,
        /// 强平数量
        quantity: Quantity,
        /// 剩余保证金
        remaining_margin: Amount,
        /// 强平罚金
        liquidation_fee: Amount,
        /// 保险基金贡献
        insurance_contribution: Amount,
        /// 社会化分摊 (穿仓时)
        socialized_loss: Option<Amount>,
    },

    /// ADL 自动减仓结算
    ///
    /// 生成 BalanceCommand:
    /// - 被强平方: DebitFrozen + 亏损
    /// - 对手方: Unfreeze + RealizedPnl
    SettleADL {
        /// 被强平仓位ID
        liquidated_position: PositionId,
        /// 被强平账户
        liquidated_account: AccountId,
        /// 对手方仓位ID
        counterparty_position: PositionId,
        /// 对手方账户
        counterparty_account: AccountId,
        /// 保证金资产
        margin_asset_id: AssetId,
        /// ADL 数量
        quantity: Quantity,
        /// ADL 价格
        price: Price,
        /// 被强平方亏损
        liquidated_loss: Amount,
        /// 对手方盈亏
        counterparty_pnl: i64,
    },

    // ==================== P3: Admin Operations ====================

    /// 调整保证金（逐仓模式）
    ///
    /// 生成 BalanceCommand:
    /// - amount > 0: Freeze (追加)
    /// - amount < 0: Unfreeze (减少)
    AdjustMargin {
        /// 账户ID
        account: AccountId,
        /// 仓位ID
        position_id: PositionId,
        /// 保证金资产
        margin_asset_id: AssetId,
        /// 金额 (正=追加, 负=减少)
        amount: i64,
    },

    /// 冲正结算
    ReverseSettlement {
        /// 原结算ID
        settlement_id: SettlementId,
        /// 冲正原因
        reason: String,
    },

    /// 批量结算
    BatchSettle {
        /// 批次ID
        batch_id: u64,
        /// 命令列表
        commands: Vec<Box<Command>>,
    },
}

/// 资金费率结算条目
#[derive(Debug, Clone)]
pub struct FundingEntry {
    /// 交易者ID
    pub trader_id: TraderId,
    /// 账户ID
    pub account: AccountId,
    /// 仓位ID
    pub position_id: PositionId,
    /// 仓位方向
    pub position_side: PositionSide,
    /// 仓位价值 (mark_price * quantity)
    pub position_value: Amount,
    /// 资金费用 (正=支付, 负=收取)
    pub funding_fee: i64,
}
```

### 2.3 Command Result Definition

```rust
/// Error Code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Balance related
    InsufficientBalance = 1001,
    InsufficientMargin = 1002,
    BalanceFrozen = 1003,

    // Entity not found
    AccountNotFound = 2001,
    PositionNotFound = 2002,
    SettlementNotFound = 2003,
    TradeNotFound = 2004,

    // Status error
    InvalidSettlementStatus = 3001,
    SettlementAlreadyCompleted = 3002,
    SettlementAlreadyReversed = 3003,
    PositionAlreadyClosed = 3004,

    // Validation error
    InvalidAmount = 4001,
    InvalidPrice = 4002,
    InvalidQuantity = 4003,
    AssetMismatch = 4004,
    AccountMismatch = 4005,

    // Concurrency error
    ConcurrencyConflict = 5001,
    OptimisticLockFailed = 5002,

    // System error
    DatabaseError = 9001,
    GatewayTimeout = 9002,
    SystemError = 9999,
}

/// Command Result
#[derive(Debug, Clone)]
pub enum CommandResult {
    // ==================== P0: Core Settlement Results ====================

    /// Spot Trade Settlement Result
    SettleSpotTrade {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Clearing ID
        clearing_id: ClearingId,
        /// Buyer new balance (quote)
        buyer_quote_balance: Amount,
        /// Buyer new balance (base)
        buyer_base_balance: Amount,
        /// Seller new balance (quote)
        seller_quote_balance: Amount,
        /// Seller new balance (base)
        seller_base_balance: Amount,
        /// Total fee
        total_fee: Amount,
        /// Status
        status: SettlementStatus,
    },

    /// Perpetual Open Position Settlement Result
    SettlePrepOpen {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Clearing ID
        clearing_id: ClearingId,
        /// Account new available balance
        available_balance: Amount,
        /// Position new margin (locked)
        position_margin: Amount,
        /// Position new avg price (weighted average if adding)
        position_avg_price: Price,
        /// Position new quantity
        position_quantity: Quantity,
        /// Liquidation price
        liquidation_price: Price,
        /// Fee deducted
        fee: Amount,
        /// Status
        status: SettlementStatus,
    },

    /// Perpetual Close Position Settlement Result (Full Close)
    SettlePrepClose {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Clearing ID
        clearing_id: ClearingId,
        /// Account new available balance (margin returned + PnL - fee)
        available_balance: Amount,
        /// Released margin
        released_margin: Amount,
        /// Realized PnL
        realized_pnl: i64,
        /// Net credit/debit (released_margin + realized_pnl - fee)
        net_amount: i64,
        /// Fee deducted
        fee: Amount,
        /// Status
        status: SettlementStatus,
    },

    /// Perpetual Reduce Position Settlement Result (Partial Close)
    SettlePrepReduce {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Clearing ID
        clearing_id: ClearingId,
        /// Account new available balance
        available_balance: Amount,
        /// Released margin (proportional)
        released_margin: Amount,
        /// Remaining position margin
        remaining_margin: Amount,
        /// Remaining position quantity
        remaining_quantity: Quantity,
        /// Realized PnL for this reduction
        realized_pnl: i64,
        /// New liquidation price (recalculated)
        liquidation_price: Price,
        /// Fee deducted
        fee: Amount,
        /// Status
        status: SettlementStatus,
    },

    // ==================== P1: Margin Management Results ====================

    /// Lock Margin Result
    LockMargin {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Account available balance
        available_balance: Amount,
        /// Position margin
        position_margin: Amount,
        /// Liquidation price
        liquidation_price: Price,
    },

    /// Release Margin Result
    ReleaseMargin {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Account available balance
        available_balance: Amount,
        /// Released amount
        released_amount: Amount,
    },

    /// Transfer Margin Result
    TransferMargin {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Account available balance
        available_balance: Amount,
        /// Position margin
        position_margin: Amount,
        /// New liquidation price
        new_liquidation_price: Price,
    },

    // ==================== P2: PnL Settlement Results ====================

    /// Realize PnL Result
    RealizePnl {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Account balance
        account_balance: Amount,
        /// Total realized PnL
        total_realized_pnl: i64,
    },

    /// Funding Settlement Result
    SettleFunding {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Settlement time
        settlement_time: Timestamp,
        /// Total long payment
        total_long_payment: Amount,
        /// Total short receipt
        total_short_receipt: Amount,
        /// Affected positions count
        affected_positions: u64,
    },

    /// Unrealized PnL Settlement Result
    SettleUnrealizedPnl {
        /// Settlement ID
        settlement_id: SettlementId,
        /// New unrealized PnL
        unrealized_pnl: i64,
        /// New margin ratio
        margin_ratio: u32,
    },

    // ==================== P3: Risk Processing Results ====================

    /// Liquidation Settlement Result
    SettleLiquidation {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Position ID
        position_id: PositionId,
        /// Liquidated value
        liquidated_value: Amount,
        /// Returned margin (to account)
        returned_margin: Amount,
        /// Insurance contribution
        insurance_contribution: Amount,
        /// Socialized loss
        socialized_loss: Amount,
    },

    /// ADL Settlement Result
    SettleADL {
        /// Settlement ID
        settlement_id: SettlementId,
        /// Liquidated party new margin
        liquidated_new_margin: Amount,
        /// Counterparty new margin
        counterparty_new_margin: Amount,
        /// Counterparty realized PnL
        counterparty_realized_pnl: i64,
    },

    /// Insurance Fund Contribution Result
    ContributeInsuranceFund {
        /// Settlement ID
        settlement_id: SettlementId,
        /// New insurance fund balance
        new_fund_balance: Amount,
    },

    // ==================== P4: Admin Operations Results ====================

    /// Reverse Settlement Result
    ReverseSettlement {
        /// Reverse settlement ID
        reverse_settlement_id: SettlementId,
        /// Original settlement ID
        original_settlement_id: SettlementId,
        /// Reversed entries count
        reversed_entries: usize,
    },

    /// Account Reconciliation Result
    ReconcileAccount {
        /// Account ID
        account: AccountId,
        /// Asset
        asset: Asset,
        /// Actual balance
        actual_balance: Amount,
        /// Expected balance
        expected_balance: Amount,
        /// Difference
        difference: i64,
        /// Is matched
        is_matched: bool,
    },

    /// Batch Settlement Result
    BatchSettle {
        /// Batch ID
        batch_id: u64,
        /// Success count
        success_count: usize,
        /// Failure count
        failure_count: usize,
        /// Settlement IDs
        settlement_ids: Vec<SettlementId>,
    },

    /// Error Result
    Error {
        /// Error code
        code: ErrorCode,
        /// Error message
        message: String,
        /// Related entity ID
        related_id: Option<u64>,
    },
}
```

### 2.4 Command Handler Trait

```rust
/// Settlement Command Handler
///
/// Following Clean Architecture:
/// - Handler focuses on business logic
/// - No dependency on concrete repo implementations
/// - Obtain external services via dependency injection
pub trait SettlementCommandHandler: Send + Sync {
    /// Handle single command
    fn handle(&mut self, command: Command) -> CommandResult;

    /// Handle batch commands (transactional)
    fn handle_batch(&mut self, commands: Vec<Command>) -> Vec<CommandResult>;

    /// Handle command async
    fn handle_async(&self, command: Command) -> impl std::future::Future<Output = CommandResult> + Send;

    /// Handler name
    fn handler_name(&self) -> &'static str;
}

/// Spot Settlement Handler
pub trait SpotSettlementHandler: SettlementCommandHandler {
    /// Handle spot trade settlement
    fn settle_spot_trade(&mut self, command: Command) -> CommandResult;
}

/// Perpetual Settlement Handler
pub trait PrepSettlementHandler: SettlementCommandHandler {
    /// Handle perpetual trade settlement
    fn settle_prep_trade(&mut self, command: Command) -> CommandResult;

    /// Handle funding settlement
    fn settle_funding(&mut self, command: Command) -> CommandResult;

    /// Handle liquidation settlement
    fn settle_liquidation(&mut self, command: Command) -> CommandResult;
}
```

## 3. Core Concepts

### 3.1 Clearing vs Settlement

| Term | Definition |
|------|------------|
| Clearing | Trade confirmation, counterparty matching, obligation calculation |
| Settlement | Fund transfer, asset delivery, final completion |

### 3.2 Business Flow

```
[Trade Execution] -> [Clearing] -> [Settlement] -> [Account Update]
        |               |               |               |
    Matching      Calculation      Transfer       Balance Update
```

## 4. Domain Model (领域模型)

### 4.1 设计原则

1. **聚合边界清晰**: Settlement 为聚合根，Entry 通过 Repository 访问，不直接嵌套
2. **类型安全**: 使用 newtype 包装基础类型，编译时防止混用
3. **高性能**: AssetId 用 u32 替代 String，结构体 64 字节缓存行对齐
4. **幂等性**: idempotency_key 防止重复结算
5. **可追溯**: 完整的关联链路 Trade → Clearing → Settlement → Entry

### 4.2 核心实体

#### 4.2.1 Settlement (结算聚合根)

```rust
/// 结算记录 - 聚合根
///
/// 代表一次原子结算操作。
/// 不直接包含 Entry，通过 SettlementEntryRepository 查询。
#[repr(align(64))]
pub struct Settlement {
    /// 结算ID
    pub id: SettlementId,

    /// 幂等键 - 防止重复结算
    /// 格式: "{source}:{trade_id}" 或 "{source}:{batch_id}:{seq}"
    pub idempotency_key: IdempotencyKey,

    /// 关联的清算记录ID
    pub clearing_id: ClearingId,

    /// 关联的交易ID（交易结算时有值）
    pub trade_id: Option<TradeId>,

    /// 结算类型
    pub settlement_type: SettlementType,

    /// 主账户（便于索引查询）
    pub primary_account: AccountId,

    /// 结算资产ID
    pub asset_id: AssetId,

    /// 结算总金额（绝对值）
    pub total_amount: Amount,

    /// 分录数量（用于校验）
    pub entry_count: u8,

    /// 状态
    pub status: SettlementStatus,

    /// 创建时间（纳秒时间戳）
    pub created_at: Timestamp,

    /// 完成时间
    pub settled_at: Option<Timestamp>,

    /// 乐观锁版本号
    pub version: u64,
}

/// 幂等键（固定长度，高性能）
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdempotencyKey(pub [u8; 32]);

impl IdempotencyKey {
    /// 从交易结算创建
    /// 格式: "TRADE:{trade_id:016x}"
    pub fn from_trade(trade_id: TradeId) -> Self {
        let mut key = [0u8; 32];
        key[0..6].copy_from_slice(b"TRADE:");
        let hex = format!("{:016x}", trade_id.0);
        key[6..22].copy_from_slice(hex.as_bytes());
        Self(key)
    }

    /// 从资金费率结算创建
    /// 格式: "FUND:{round}:{position_id:016x}"
    pub fn from_funding(round: u32, position_id: PositionId) -> Self {
        let mut key = [0u8; 32];
        key[0..5].copy_from_slice(b"FUND:");
        let content = format!("{:08x}:{:016x}", round, position_id.0);
        key[5..30].copy_from_slice(content.as_bytes());
        Self(key)
    }
}

/// 结算类型
///
/// 注意: 保证金操作（锁定/释放/调整）已内化到各 Settlement Command 中，
/// 不再作为独立的结算类型。参见 Section 2.1 设计原则。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SettlementType {
    // === Spot (0x1X) ===
    /// 现货即时结算
    SpotInstant = 0x10,

    // === Perpetual Trade (0x2X) ===
    /// 永续开仓结算（含保证金锁定）
    PrepOpen = 0x20,
    /// 永续平仓结算（含保证金释放 + 盈亏结算）
    PrepClose = 0x21,
    /// 永续减仓结算（含部分保证金释放 + 盈亏结算）
    PrepReduce = 0x22,

    // === Perpetual Periodic (0x3X) ===
    /// 资金费率结算
    PrepFundingRate = 0x30,

    // === Risk (0x4X) ===
    /// 强平结算（含保证金清算 + 保险基金）
    PrepLiquidation = 0x40,
    /// ADL自动减仓结算
    PrepADL = 0x41,

    // === Admin (0xFX) ===
    /// 保证金调整（逐仓模式手动追加/减少）
    MarginAdjust = 0xF0,
    /// 冲正/回滚
    Reversal = 0xF1,
}

/// 结算状态（状态机）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SettlementStatus {
    /// 待处理
    Pending = 0,
    /// 处理中
    Processing = 1,
    /// 已完成
    Completed = 2,
    /// 失败
    Failed = 3,
    /// 已冲正
    Reversed = 4,
}

impl SettlementStatus {
    /// 状态转换校验
    pub fn can_transition_to(&self, target: Self) -> bool {
        use SettlementStatus::*;
        matches!(
            (self, target),
            (Pending, Processing) |
            (Processing, Completed) |
            (Processing, Failed) |
            (Completed, Reversed) |
            (Failed, Pending)  // 重试
        )
    }
}
```

#### 4.2.2 SettlementEntry (结算分录)

```rust
/// 结算分录 - 单条资金变动记录
///
/// 创建后不可变（append-only 模式）。
/// 通过 SettlementEntryRepository 查询。
#[repr(align(64))]
pub struct SettlementEntry {
    /// 分录ID
    pub id: EntryId,

    /// 所属结算ID
    pub settlement_id: SettlementId,

    /// 分录序号（0-based，保证顺序）
    pub sequence: u8,

    /// 账户ID
    pub account_id: AccountId,

    /// 仓位ID（永续合约用，现货为0）
    pub position_id: PositionId,

    /// 资产ID
    pub asset_id: AssetId,

    /// 分录类型
    pub entry_type: EntryType,

    /// 资金方向（借/贷）
    pub direction: Direction,

    /// 金额（始终为正，direction 表示方向）
    pub amount: Amount,

    /// 变动前余额
    pub balance_before: Amount,

    /// 变动后余额
    pub balance_after: Amount,

    /// 时间戳
    pub timestamp: Timestamp,

    /// 备注（可存 trade_id, funding_round 等）
    pub memo: u64,
}

/// 资金方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    /// 贷方（余额增加）
    Credit = 0,
    /// 借方（余额减少）
    Debit = 1,
}

/// 分录类型（按业务分类）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntryType {
    // === 现货 (0x1X) ===
    /// 现货资产转移（买入/卖出）
    SpotTransfer = 0x10,
    /// 现货手续费
    SpotFee = 0x11,

    // === 保证金 (0x2X) ===
    /// 锁定保证金到仓位
    MarginLock = 0x20,
    /// 从仓位释放保证金
    MarginRelease = 0x21,
    /// 保证金调整（逐仓追加/减少）
    MarginTransfer = 0x22,

    // === 盈亏 (0x3X) ===
    /// 已实现盈亏
    RealizedPnl = 0x30,
    /// 未实现盈亏（盯市结算）
    UnrealizedPnl = 0x31,
    /// 资金费用（支付/收取）
    FundingFee = 0x32,

    // === 手续费 (0x4X) ===
    /// 永续交易手续费
    TradingFee = 0x40,
    /// Maker 返佣
    MakerRebate = 0x41,

    // === 风控 (0x5X) ===
    /// 强平罚金
    LiquidationPenalty = 0x50,
    /// 保险基金注入
    InsuranceFund = 0x51,
    /// 社会化分摊损失
    SocializedLoss = 0x52,
    /// ADL 强制平仓
    ADLClose = 0x53,

    // === 管理 (0xFX) ===
    /// 人工调账
    Adjustment = 0xF0,
    /// 冲正分录
    Reversal = 0xF1,
}

impl EntryType {
    /// 获取该分录类型的默认方向
    pub fn default_direction(&self) -> Option<Direction> {
        use EntryType::*;
        use Direction::*;
        match self {
            // 固定借方（减少）
            SpotFee | TradingFee | LiquidationPenalty |
            MarginLock | SocializedLoss => Some(Debit),

            // 固定贷方（增加）
            MakerRebate | MarginRelease | InsuranceFund => Some(Credit),

            // 视情况而定
            SpotTransfer | MarginTransfer | RealizedPnl |
            UnrealizedPnl | FundingFee | ADLClose |
            Adjustment | Reversal => None,
        }
    }

    /// 是否允许产生负余额
    pub fn allows_negative_balance(&self) -> bool {
        // 只有未实现盈亏可能导致负余额（全仓模式）
        matches!(self, EntryType::UnrealizedPnl)
    }
}
```

#### 4.2.3 ClearingRecord (清算记录)

```rust
/// 清算记录 - 交易清算中间状态
///
/// 撮合成交后创建，结算前的中间态。
/// 链路: Trade → ClearingRecord → Settlement(s) → Entry(s)
#[repr(align(64))]
pub struct ClearingRecord {
    /// 清算ID
    pub id: ClearingId,

    /// 交易ID（来自撮合引擎）
    pub trade_id: TradeId,

    /// 交易品种类型
    pub instrument_type: InstrumentType,

    /// 交易对ID（如 BTC-USDT = 1）
    pub symbol_id: SymbolId,

    /// 买方信息
    pub buyer: ClearingParty,

    /// 卖方信息
    pub seller: ClearingParty,

    /// 成交价格
    pub price: Price,

    /// 成交数量
    pub quantity: Quantity,

    /// 名义价值 (price * quantity)
    pub notional: Amount,

    /// 清算状态
    pub status: ClearingStatus,

    /// 关联的结算ID数组
    /// - Spot: 1个结算（买卖双方在同一个结算中）
    /// - Prep: 1~2个结算（买方结算 + 卖方结算，可能独立）
    pub settlement_ids: [SettlementId; 2],

    /// 结算数量（1 或 2）
    pub settlement_count: u8,

    /// 创建时间
    pub created_at: Timestamp,

    /// 完成时间
    pub completed_at: Option<Timestamp>,
}

/// 交易品种类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InstrumentType {
    /// 现货
    Spot = 0,
    /// 永续合约
    Perpetual = 1,
    // 未来扩展
    // Futures = 2,
    // Options = 3,
}

/// 清算参与方信息
#[derive(Debug, Clone, Copy)]
pub struct ClearingParty {
    /// 交易者ID
    pub trader_id: TraderId,
    /// 账户ID
    pub account_id: AccountId,
    /// 仓位ID（永续合约用）
    pub position_id: PositionId,
    /// 手续费
    pub fee: Amount,
    /// 是否为 Maker
    pub is_maker: bool,
    /// 本方交易方向 (Buy/Sell)
    pub side: Side,
    /// 仓位方向（永续合约用）
    pub position_side: PositionSide,
    /// 是否为平仓（永续合约用）
    pub is_close: bool,
}

/// 清算状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClearingStatus {
    /// 刚创建
    Created = 0,
    /// 校验通过
    Validated = 1,
    /// 结算记录已创建
    SettlementCreated = 2,
    /// 全部结算完成
    Completed = 3,
    /// 被拒绝（校验失败）
    Rejected = 4,
}

impl ClearingStatus {
    /// 状态转换校验
    pub fn can_transition_to(&self, target: Self) -> bool {
        use ClearingStatus::*;
        matches!(
            (self, target),
            (Created, Validated) |
            (Created, Rejected) |
            (Validated, SettlementCreated) |
            (Validated, Rejected) |
            (SettlementCreated, Completed) |
            (SettlementCreated, Rejected)
        )
    }
}
```

### 4.3 Value Objects (值对象)

```rust
// ============================================================================
// 类型安全的 ID 包装（newtype 模式）
// ============================================================================

/// 结算ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct SettlementId(pub u64);

/// 清算ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ClearingId(pub u64);

/// 分录ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct EntryId(pub u64);

/// 账户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccountId(pub u64);

/// 交易ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TradeId(pub u64);

/// 交易者ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TraderId(pub u64);

/// 仓位ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PositionId(pub u64);

/// 交易对ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SymbolId(pub u32);

// ============================================================================
// 带精度的数值类型
// ============================================================================

/// 金额 - 有符号整数，8位小数精度
///
/// 示例:
/// - 1.00000000 USDT = 100_000_000
/// - -0.50000000 BTC = -50_000_000
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Amount(pub i64);

impl Amount {
    pub const PRECISION: u32 = 8;
    pub const SCALE: i64 = 100_000_000;
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline]
    pub fn saturating_add(&self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    #[inline]
    pub fn saturating_sub(&self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

/// 价格 - 无符号整数，8位小数精度
///
/// 示例:
/// - 50000.00000000 USDT = 5_000_000_000_000
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Price(pub u64);

impl Price {
    pub const PRECISION: u32 = 8;
    pub const SCALE: u64 = 100_000_000;
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

/// 数量 - 无符号整数，8位小数精度
///
/// 示例:
/// - 1.50000000 BTC = 150_000_000
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Quantity(pub u64);

impl Quantity {
    pub const PRECISION: u32 = 8;
    pub const SCALE: u64 = 100_000_000;
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

/// 时间戳 - 纳秒级 Unix 时间戳
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Timestamp(pub u64);

impl Timestamp {
    #[inline]
    pub fn now() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self(nanos)
    }

    #[inline]
    pub fn as_millis(&self) -> u64 {
        self.0 / 1_000_000
    }

    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.0 / 1_000_000_000
    }
}

// ============================================================================
// 资产标识（高性能）
// ============================================================================

/// 资产ID - 数值标识，替代 String
///
/// 性能优化: 用 u32 替代 String，避免内存分配和字符串比较。
/// 资产注册表负责 AssetId <-> 符号字符串 的映射。
///
/// 预留区间:
/// - 0: 无效
/// - 1-99: 主流币种 (BTC=1, ETH=2, BNB=3...)
/// - 100-199: 稳定币 (USDT=100, USDC=101, BUSD=102...)
/// - 200-999: 其他主流代币
/// - 1000+: 长尾代币
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AssetId(pub u32);

impl AssetId {
    pub const INVALID: Self = Self(0);
    pub const BTC: Self = Self(1);
    pub const ETH: Self = Self(2);
    pub const BNB: Self = Self(3);
    pub const USDT: Self = Self(100);
    pub const USDC: Self = Self(101);

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn is_stablecoin(&self) -> bool {
        self.0 >= 100 && self.0 < 200
    }
}

// ============================================================================
// 交易枚举
// ============================================================================

/// 交易方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Side {
    #[default]
    Buy = 0,
    Sell = 1,
}

impl Side {
    /// 获取对手方向
    #[inline]
    pub fn opposite(&self) -> Self {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

/// 仓位方向（永续合约对冲模式）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PositionSide {
    /// 单向持仓模式（净仓位）
    #[default]
    Both = 0,
    /// 对冲模式多头
    Long = 1,
    /// 对冲模式空头
    Short = 2,
}

// ============================================================================
// 领域计算辅助方法
// ============================================================================

impl Amount {
    /// 计算名义价值: price * quantity
    #[inline]
    pub fn notional(price: Price, quantity: Quantity) -> Self {
        // price (8 decimals) * quantity (8 decimals) = 16 decimals
        // 除以 SCALE 恢复到 8 decimals
        let raw = (price.0 as i128 * quantity.0 as i128) / Self::SCALE as i128;
        Self(raw as i64)
    }

    /// 计算保证金: notional / leverage
    #[inline]
    pub fn margin(notional: Amount, leverage: u32) -> Self {
        Self(notional.0 / leverage as i64)
    }

    /// 计算多头盈亏: (exit_price - entry_price) * quantity
    #[inline]
    pub fn long_pnl(entry: Price, exit: Price, qty: Quantity) -> Self {
        let price_diff = exit.0 as i64 - entry.0 as i64;
        let raw = (price_diff as i128 * qty.0 as i128) / Self::SCALE as i128;
        Self(raw as i64)
    }

    /// 计算空头盈亏: (entry_price - exit_price) * quantity
    #[inline]
    pub fn short_pnl(entry: Price, exit: Price, qty: Quantity) -> Self {
        let price_diff = entry.0 as i64 - exit.0 as i64;
        let raw = (price_diff as i128 * qty.0 as i128) / Self::SCALE as i128;
        Self(raw as i64)
    }

    /// 根据仓位方向计算盈亏
    #[inline]
    pub fn pnl(entry: Price, exit: Price, qty: Quantity, side: PositionSide) -> Self {
        match side {
            PositionSide::Long | PositionSide::Both => Self::long_pnl(entry, exit, qty),
            PositionSide::Short => Self::short_pnl(entry, exit, qty),
        }
    }
}

// ============================================================================
// 运算符重载
// ============================================================================

impl std::ops::Add for Amount {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Amount {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Neg for Amount {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
```

### 4.4 实体关系图

```
┌─────────────────────────────────────────────────────────────────┐
│                      Trade (来自撮合引擎)                         │
│  trade_id, buyer, seller, price, qty, timestamp                 │
└───────────────────────────────┬─────────────────────────────────┘
                                │ 1:1
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                       ClearingRecord                            │
│  clearing_id, trade_id, instrument_type, buyer, seller          │
│  price, quantity, notional, status, settlement_ids[]            │
└───────────────────────────────┬─────────────────────────────────┘
                                │ 1:N
                                │ (Spot: N=1, Prep: N=1~2)
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Settlement                               │
│  settlement_id, clearing_id, trade_id, idempotency_key          │
│  settlement_type, primary_account, asset_id, total_amount       │
│  entry_count, status, created_at, settled_at, version           │
└───────────────────────────────┬─────────────────────────────────┘
                                │ 1:N (N=2~6)
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      SettlementEntry                            │
│  entry_id, settlement_id, sequence, account_id, position_id     │
│  asset_id, entry_type, direction, amount                        │
│  balance_before, balance_after, timestamp, memo                 │
└─────────────────────────────────────────────────────────────────┘

关系说明:
┌────────────────────────────────────────────────────────────────┐
│ Trade : ClearingRecord = 1:1                                   │
│ ClearingRecord : Settlement = 1:N                              │
│   - Spot: N=1 (买卖双方在同一个 Settlement)                      │
│   - Prep 开仓: N=1~2 (买方开仓 + 卖方可能也开仓)                  │
│   - Prep 平仓: N=1~2 (各方独立结算)                              │
│ Settlement : SettlementEntry = 1:N (N=2~6)                     │
│   - Spot: 4~6 entries (双方资产转移 + 手续费)                    │
│   - Prep: 2~4 entries (保证金锁定/释放 + PnL + 手续费)           │
└────────────────────────────────────────────────────────────────┘
```

### 4.5 业务场景验证

| 业务场景 | 模型支持 | 关键字段 |
|---------|---------|---------|
| 现货买卖双方结算 | ✅ | Settlement.settlement_type=SpotInstant |
| 永续开仓锁定保证金 | ✅ | Entry.position_id, Entry.entry_type=MarginLock |
| 永续平仓释放保证金+PnL | ✅ | Entry.entry_type=MarginRelease/RealizedPnl |
| 资金费率批量结算 | ✅ | Settlement.settlement_type=PrepFundingRate |
| 强平结算 | ✅ | Settlement.settlement_type=PrepLiquidation |
| 防重复结算 | ✅ | Settlement.idempotency_key |
| 按仓位查流水 | ✅ | Entry.position_id |
| 按账户查流水 | ✅ | Entry.account_id |
| 分录顺序保证 | ✅ | Entry.sequence |
| 对账核算 | ✅ | Entry.balance_before/balance_after |
| 交易追溯 | ✅ | Settlement.trade_id, ClearingRecord.trade_id |

### 4.6 不变量与业务规则

```rust
impl Settlement {
    /// 校验结算不变量
    pub fn validate(&self) -> Result<(), DomainError> {
        // 1. ID 必须有效
        if self.id.0 == 0 {
            return Err(DomainError::InvalidSettlementId);
        }

        // 2. 必须有分录
        if self.entry_count == 0 {
            return Err(DomainError::NoSettlementEntries);
        }

        // 3. 完成状态必须有完成时间
        if self.status == SettlementStatus::Completed && self.settled_at.is_none() {
            return Err(DomainError::MissingSettledTimestamp);
        }

        // 4. 资产必须有效
        if !self.asset_id.is_valid() {
            return Err(DomainError::InvalidAssetId);
        }

        Ok(())
    }
}

impl SettlementEntry {
    /// 校验分录不变量
    pub fn validate(&self) -> Result<(), DomainError> {
        // 1. 金额必须为正（direction 表示方向）
        if self.amount.0 <= 0 {
            return Err(DomainError::InvalidAmount);
        }

        // 2. 余额一致性校验
        let expected_after = match self.direction {
            Direction::Credit => self.balance_before.saturating_add(self.amount),
            Direction::Debit => self.balance_before.saturating_sub(self.amount),
        };
        if self.balance_after != expected_after {
            return Err(DomainError::BalanceMismatch);
        }

        // 3. 余额不能为负（特定类型除外）
        if self.balance_after.is_negative() && !self.entry_type.allows_negative_balance() {
            return Err(DomainError::InsufficientBalance);
        }

        Ok(())
    }
}

/// 领域错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// 无效结算ID
    InvalidSettlementId,
    /// 无分录
    NoSettlementEntries,
    /// 缺少完成时间
    MissingSettledTimestamp,
    /// 无效资产ID
    InvalidAssetId,
    /// 无效金额
    InvalidAmount,
    /// 余额不一致
    BalanceMismatch,
    /// 余额不足
    InsufficientBalance,
    /// 无效状态转换
    InvalidStateTransition,
    /// 重复结算
    DuplicateSettlement,
}
```

## 5. 结算与资产账本交互

### 5.1 架构设计

结算模块通过 **AccountGateway** 接口与资产账本交互，遵循 Clean Architecture 的依赖倒置原则：

```
┌─────────────────────────────────────────────────────────────────┐
│                     Settlement Module                           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Domain Layer (核心业务逻辑)                  │   │
│  │  ┌───────────────┐    ┌───────────────┐                 │   │
│  │  │  Settlement   │    │ SettlementEntry│                │   │
│  │  │  (聚合根)      │    │  (分录)        │                 │   │
│  │  └───────────────┘    └───────────────┘                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Application Layer (用例层)                   │   │
│  │  ┌───────────────────────────────────────────────────┐  │   │
│  │  │           SettlementService                        │  │   │
│  │  │  - 编排结算流程                                     │  │   │
│  │  │  - 调用 AccountGateway 更新账本                     │  │   │
│  │  │  - 保证事务一致性                                   │  │   │
│  │  └───────────────────────────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Port Layer (端口接口)                        │   │
│  │  ┌─────────────────┐  ┌─────────────────┐               │   │
│  │  │ AccountGateway  │  │ PositionGateway │               │   │
│  │  │   (账本接口)     │  │   (仓位接口)    │                │   │
│  │  └─────────────────┘  └─────────────────┘               │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                               │
                               ▼ (依赖注入)
┌─────────────────────────────────────────────────────────────────┐
│                   Infrastructure Layer                          │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Account Module (资产账本模块)                │   │
│  │  ┌───────────────┐    ┌───────────────┐                 │   │
│  │  │    Account    │    │    Balance    │                 │   │
│  │  │   (账户)       │    │   (余额)       │                │   │
│  │  └───────────────┘    └───────────────┘                 │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 AccountCommandRepo 模式

#### 5.2.1 设计思路

Settlement 模块不直接操作 Account 余额，而是：
1. **生成 BalanceCommand**（余额变更指令）
2. **持久化到 AccountCommandRepo**

Account 模块作为消费者：
1. **消费 Command**
2. **执行余额变更**

```
┌─────────────────────────────────────────────────────────────────┐
│                  AccountCommandRepo 模式                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐    ┌───────────────────┐    ┌──────────────┐ │
│  │  Settlement  │───▶│ AccountCommandRepo │───▶│   Account    │ │
│  │   Service    │    │   (Command Log)    │    │   Consumer   │ │
│  └──────────────┘    └───────────────────┘    └──────────────┘ │
│         │                     │                      │          │
│    生成 Command          持久化 Command         消费并执行       │
│                               │                                 │
│                 ┌─────────────┴─────────────┐                  │
│                 │       部署灵活性           │                  │
│                 ├───────────────────────────┤                  │
│                 │ 同线程: 直接方法调用       │                  │
│                 │ 跨线程: Channel/Queue      │                  │
│                 │ 跨主机: Kafka/MQ/gRPC      │                  │
│                 └───────────────────────────┘                  │
│                                                                 │
│  优势：                                                         │
│  - Settlement 与 Account 解耦                                   │
│  - Settlement 只保证 Command 持久化成功即可                      │
│  - Command Log 天然形成审计日志                                  │
│  - 部署灵活：同一接口，不同消费策略                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### 5.2.2 BalanceCommand 定义

```rust
/// 余额变更命令
///
/// Settlement 生成，Account 消费执行。
/// 设计为不可变、可序列化，便于持久化和跨进程传输。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceCommand {
    /// 命令ID（全局唯一）
    pub id: CommandId,

    /// 幂等键（防止重复执行）
    pub idempotency_key: IdempotencyKey,

    /// 关联的结算ID
    pub settlement_id: SettlementId,

    /// 命令类型
    pub command_type: BalanceCommandType,

    /// 账户ID
    pub account_id: AccountId,

    /// 资产ID
    pub asset_id: AssetId,

    /// 金额（有符号，支持盈亏）
    /// 正=增加/冻结，负=减少/解冻
    pub amount: i64,

    /// 命令状态
    pub status: CommandStatus,

    /// 创建时间
    pub created_at: Timestamp,

    /// 执行时间
    pub executed_at: Option<Timestamp>,

    /// 执行结果（余额快照）
    pub result: Option<BalanceSnapshot>,
}

/// 命令ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct CommandId(pub u64);

/// 余额命令类型
///
/// 按业务分类，便于审计和统计
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum BalanceCommandType {
    // === 保证金操作 (0x1X) ===
    /// 冻结保证金（可用 → 冻结）
    Freeze = 0x10,
    /// 解冻保证金（冻结 → 可用）
    Unfreeze = 0x11,

    // === 通用余额操作 (0x2X) ===
    /// 增加可用余额
    Credit = 0x20,
    /// 扣减可用余额
    Debit = 0x21,
    /// 扣减冻结余额（强平时使用）
    DebitFrozen = 0x22,

    // === 现货转账 (0x3X) ===
    /// 现货转出（扣减可用）
    TransferOut = 0x30,
    /// 现货转入（增加可用）
    TransferIn = 0x31,

    // === 手续费 (0x4X) ===
    /// 交易手续费（扣减可用）
    TradingFee = 0x40,
    /// 资金费用（可正可负）
    FundingFee = 0x41,
    /// 强平罚金
    LiquidationFee = 0x42,
    /// Maker 返佣（增加可用）
    MakerRebate = 0x43,

    // === 盈亏结算 (0x5X) ===
    /// 已实现盈亏（amount 可正可负）
    RealizedPnl = 0x50,

    // === 保险基金 (0x6X) ===
    /// 注入保险基金
    InsuranceFundIn = 0x60,
    /// 保险基金赔付
    InsuranceFundOut = 0x61,
}

impl BalanceCommandType {
    /// 获取命令的默认余额影响方向
    /// Some(true) = 增加, Some(false) = 减少, None = 由 amount 符号决定
    pub fn default_effect(&self) -> Option<bool> {
        use BalanceCommandType::*;
        match self {
            // 固定增加
            Credit | TransferIn | MakerRebate | InsuranceFundOut => Some(true),
            // 固定减少
            Debit | DebitFrozen | TransferOut | TradingFee | LiquidationFee | InsuranceFundIn => Some(false),
            // 由 amount 符号决定
            Freeze | Unfreeze | FundingFee | RealizedPnl => None,
        }
    }

    /// 是否操作冻结余额
    pub fn affects_frozen(&self) -> bool {
        use BalanceCommandType::*;
        matches!(self, Freeze | Unfreeze | DebitFrozen)
    }
}

/// 命令状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CommandStatus {
    /// 待执行
    Pending = 0,
    /// 执行中
    Executing = 1,
    /// 已完成
    Completed = 2,
    /// 失败
    Failed = 3,
    /// 已跳过（幂等检查命中）
    Skipped = 4,
}

/// 余额快照（执行结果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSnapshot {
    pub available_before: Amount,
    pub frozen_before: Amount,
    pub available_after: Amount,
    pub frozen_after: Amount,
    pub version: u64,
}

/// 批量命令（原子执行）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceCommandBatch {
    /// 批次ID
    pub batch_id: u64,

    /// 关联的结算ID
    pub settlement_id: SettlementId,

    /// 命令列表（按顺序执行）
    pub commands: Vec<BalanceCommand>,

    /// 批次状态
    pub status: CommandStatus,

    /// 创建时间
    pub created_at: Timestamp,
}
```

#### 5.2.3 AccountCommandRepo 接口

```rust
/// 账户命令仓储
///
/// Settlement 模块调用此接口持久化命令，
/// Account 模块消费并执行。
#[async_trait]
pub trait AccountCommandRepo: Send + Sync {
    // =========================================================================
    // 写入操作（Settlement 调用）
    // =========================================================================

    /// 保存单个命令
    async fn save(&self, command: BalanceCommand) -> Result<(), RepoError>;

    /// 批量保存命令（同一结算的多个命令）
    async fn save_batch(&self, batch: BalanceCommandBatch) -> Result<(), RepoError>;

    // =========================================================================
    // 读取操作（Account Consumer 调用）
    // =========================================================================

    /// 获取待执行的命令
    async fn fetch_pending(
        &self,
        limit: usize,
    ) -> Result<Vec<BalanceCommand>, RepoError>;

    /// 获取待执行的批次
    async fn fetch_pending_batches(
        &self,
        limit: usize,
    ) -> Result<Vec<BalanceCommandBatch>, RepoError>;

    /// 根据结算ID查询命令
    async fn find_by_settlement(
        &self,
        settlement_id: SettlementId,
    ) -> Result<Vec<BalanceCommand>, RepoError>;

    // =========================================================================
    // 状态更新（Account Consumer 调用）
    // =========================================================================

    /// 更新命令状态和结果
    async fn update_status(
        &self,
        command_id: CommandId,
        status: CommandStatus,
        result: Option<BalanceSnapshot>,
    ) -> Result<(), RepoError>;

    /// 批量更新状态
    async fn update_batch_status(
        &self,
        batch_id: u64,
        status: CommandStatus,
        results: Vec<(CommandId, CommandStatus, Option<BalanceSnapshot>)>,
    ) -> Result<(), RepoError>;

    // =========================================================================
    // 幂等检查
    // =========================================================================

    /// 检查命令是否已执行（幂等）
    async fn exists_by_idempotency_key(
        &self,
        key: &IdempotencyKey,
    ) -> Result<bool, RepoError>;
}
```

#### 5.2.4 消费策略

```rust
/// 命令消费者 trait
///
/// 不同部署模式实现此 trait
pub trait BalanceCommandConsumer: Send + Sync {
    /// 消费并执行命令
    fn consume(&self, command: BalanceCommand) -> Result<BalanceSnapshot, ConsumerError>;

    /// 批量消费
    fn consume_batch(&self, batch: BalanceCommandBatch) -> Result<Vec<BalanceSnapshot>, ConsumerError>;
}

// ============================================================================
// 策略 1: 同线程直接调用
// ============================================================================

/// 同步消费者（同线程）
pub struct SyncBalanceConsumer {
    balance_store: Arc<dyn BalanceStore>,
}

impl BalanceCommandConsumer for SyncBalanceConsumer {
    fn consume(&self, command: BalanceCommand) -> Result<BalanceSnapshot, ConsumerError> {
        // 直接在当前线程执行
        self.execute_command(&command)
    }

    fn consume_batch(&self, batch: BalanceCommandBatch) -> Result<Vec<BalanceSnapshot>, ConsumerError> {
        // 事务中批量执行
        self.balance_store.transaction(|tx| {
            let mut results = Vec::with_capacity(batch.commands.len());
            for cmd in &batch.commands {
                let snapshot = self.execute_command_in_tx(tx, cmd)?;
                results.push(snapshot);
            }
            Ok(results)
        })
    }
}

// ============================================================================
// 策略 2: 跨线程 Channel
// ============================================================================

/// 异步消费者（跨线程）
pub struct AsyncBalanceConsumer {
    sender: mpsc::Sender<BalanceCommand>,
}

impl AsyncBalanceConsumer {
    pub fn new(balance_store: Arc<dyn BalanceStore>) -> (Self, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel(10000);

        let handle = std::thread::spawn(move || {
            let worker = BalanceWorker::new(balance_store, rx);
            worker.run();
        });

        (Self { sender: tx }, handle)
    }
}

impl BalanceCommandConsumer for AsyncBalanceConsumer {
    fn consume(&self, command: BalanceCommand) -> Result<BalanceSnapshot, ConsumerError> {
        // 投递到 Channel，异步执行
        self.sender.send(command)?;
        // 返回占位结果，实际结果通过回调或查询获取
        Ok(BalanceSnapshot::pending())
    }
}

/// 后台工作线程
struct BalanceWorker {
    balance_store: Arc<dyn BalanceStore>,
    receiver: mpsc::Receiver<BalanceCommand>,
}

impl BalanceWorker {
    fn run(&self) {
        while let Ok(command) = self.receiver.recv() {
            match self.execute(&command) {
                Ok(snapshot) => {
                    // 更新命令状态为完成
                    self.update_status(command.id, CommandStatus::Completed, Some(snapshot));
                }
                Err(e) => {
                    // 更新命令状态为失败
                    self.update_status(command.id, CommandStatus::Failed, None);
                }
            }
        }
    }
}

// ============================================================================
// 策略 3: 跨主机（Kafka/MQ）
// ============================================================================

/// 远程消费者（跨主机）
pub struct RemoteBalanceConsumer {
    /// Kafka producer
    producer: KafkaProducer,
    /// Topic
    topic: String,
}

impl BalanceCommandConsumer for RemoteBalanceConsumer {
    fn consume(&self, command: BalanceCommand) -> Result<BalanceSnapshot, ConsumerError> {
        // 序列化并发送到 Kafka
        let payload = bincode::serialize(&command)?;
        self.producer.send(&self.topic, &payload)?;

        // 返回占位结果
        Ok(BalanceSnapshot::pending())
    }
}

/// 远程 Account 服务的消费者
pub struct AccountServiceConsumer {
    consumer: KafkaConsumer,
    balance_store: Arc<dyn BalanceStore>,
    command_repo: Arc<dyn AccountCommandRepo>,
}

impl AccountServiceConsumer {
    pub async fn run(&self) {
        loop {
            // 从 Kafka 拉取消息
            let messages = self.consumer.poll().await;

            for msg in messages {
                let command: BalanceCommand = bincode::deserialize(&msg.payload)?;

                // 幂等检查
                if self.command_repo.exists_by_idempotency_key(&command.idempotency_key).await? {
                    continue;
                }

                // 执行命令
                match self.execute(&command).await {
                    Ok(snapshot) => {
                        self.command_repo.update_status(
                            command.id,
                            CommandStatus::Completed,
                            Some(snapshot),
                        ).await?;
                    }
                    Err(e) => {
                        self.command_repo.update_status(
                            command.id,
                            CommandStatus::Failed,
                            None,
                        ).await?;
                    }
                }

                // 提交 offset
                self.consumer.commit(&msg).await?;
            }
        }
    }
}
```

#### 5.2.5 Settlement 调用示例

```rust
/// 结算服务
pub struct SettlementService {
    settlement_repo: Arc<dyn SettlementRepository>,
    entry_repo: Arc<dyn SettlementEntryRepository>,
    command_repo: Arc<dyn AccountCommandRepo>,
    consumer: Arc<dyn BalanceCommandConsumer>,
}

impl SettlementService {
    /// 执行结算
    pub async fn execute_settlement(
        &self,
        settlement: Settlement,
        entries: Vec<SettlementEntry>,
    ) -> Result<SettlementResult, SettlementError> {

        // 1. 幂等检查
        if self.settlement_repo.exists_by_idempotency_key(&settlement.idempotency_key).await? {
            return Err(SettlementError::Duplicate);
        }

        // 2. 生成余额命令
        let commands = self.build_balance_commands(&settlement, &entries);
        let batch = BalanceCommandBatch {
            batch_id: settlement.id.0,
            settlement_id: settlement.id,
            commands,
            status: CommandStatus::Pending,
            created_at: Timestamp::now(),
        };

        // 3. 事务：保存 Settlement + Entry + Commands
        self.settlement_repo.transaction(|tx| async {
            tx.insert_settlement(&settlement).await?;
            tx.insert_entries(&entries).await?;
            tx.insert_command_batch(&batch).await?;
            Ok(())
        }).await?;

        // 4. 触发消费（根据策略同步或异步）
        let results = self.consumer.consume_batch(batch)?;

        // 5. 更新结算状态
        self.settlement_repo.update_status(
            settlement.id,
            SettlementStatus::Completed,
        ).await?;

        Ok(SettlementResult::Success {
            settlement_id: settlement.id,
            balance_results: results,
        })
    }

    /// 从 Entry 生成 BalanceCommand
    ///
    /// ## EntryType → BalanceCommandType 映射规则
    ///
    /// | EntryType | Direction | BalanceCommandType |
    /// |-----------|-----------|-------------------|
    /// | SpotTransfer | Debit | TransferOut |
    /// | SpotTransfer | Credit | TransferIn |
    /// | SpotFee | Debit | TradingFee |
    /// | MarginLock | Debit | Freeze |
    /// | MarginRelease | Credit | Unfreeze |
    /// | MarginTransfer | Debit | Freeze |
    /// | MarginTransfer | Credit | Unfreeze |
    /// | RealizedPnl | * | RealizedPnl |
    /// | FundingFee | * | FundingFee |
    /// | TradingFee | Debit | TradingFee |
    /// | MakerRebate | Credit | MakerRebate |
    /// | LiquidationPenalty | Debit | LiquidationFee |
    /// | InsuranceFund | Credit | InsuranceFundIn |
    /// | SocializedLoss | Debit | Debit |
    /// | ADLClose | * | RealizedPnl |
    fn build_balance_commands(
        &self,
        settlement: &Settlement,
        entries: &[SettlementEntry],
    ) -> Vec<BalanceCommand> {
        entries.iter().enumerate().map(|(seq, entry)| {
            let command_type = match (entry.entry_type, entry.direction) {
                // 现货转账
                (EntryType::SpotTransfer, Direction::Debit) => BalanceCommandType::TransferOut,
                (EntryType::SpotTransfer, Direction::Credit) => BalanceCommandType::TransferIn,
                (EntryType::SpotFee, Direction::Debit) => BalanceCommandType::TradingFee,

                // 保证金操作
                (EntryType::MarginLock, Direction::Debit) => BalanceCommandType::Freeze,
                (EntryType::MarginRelease, Direction::Credit) => BalanceCommandType::Unfreeze,
                (EntryType::MarginTransfer, Direction::Debit) => BalanceCommandType::Freeze,
                (EntryType::MarginTransfer, Direction::Credit) => BalanceCommandType::Unfreeze,

                // 盈亏结算 - 使用专用类型
                (EntryType::RealizedPnl, _) => BalanceCommandType::RealizedPnl,
                (EntryType::FundingFee, _) => BalanceCommandType::FundingFee,
                (EntryType::ADLClose, _) => BalanceCommandType::RealizedPnl,

                // 手续费
                (EntryType::TradingFee, Direction::Debit) => BalanceCommandType::TradingFee,
                (EntryType::MakerRebate, Direction::Credit) => BalanceCommandType::MakerRebate,

                // 风控操作
                (EntryType::LiquidationPenalty, Direction::Debit) => BalanceCommandType::LiquidationFee,
                (EntryType::InsuranceFund, Direction::Credit) => BalanceCommandType::InsuranceFundIn,
                (EntryType::SocializedLoss, Direction::Debit) => BalanceCommandType::Debit,

                // 管理操作 - 使用通用类型
                (EntryType::Adjustment, Direction::Credit) => BalanceCommandType::Credit,
                (EntryType::Adjustment, Direction::Debit) => BalanceCommandType::Debit,
                (EntryType::Reversal, Direction::Credit) => BalanceCommandType::Credit,
                (EntryType::Reversal, Direction::Debit) => BalanceCommandType::Debit,

                // 其他 fallback
                (_, Direction::Credit) => BalanceCommandType::Credit,
                (_, Direction::Debit) => BalanceCommandType::Debit,
            };

            BalanceCommand {
                id: CommandId(settlement.id.0 * 100 + seq as u64),
                idempotency_key: IdempotencyKey::from_settlement_entry(settlement.id, seq as u8),
                settlement_id: settlement.id,
                command_type,
                account_id: entry.account_id,
                asset_id: entry.asset_id,
                amount: entry.amount,
                status: CommandStatus::Pending,
                created_at: Timestamp::now(),
                executed_at: None,
                result: None,
            }
        }).collect()
    }
}
```

#### 5.2.6 数据库表设计

```sql
-- 余额命令表
CREATE TABLE balance_commands (
    id              BIGINT PRIMARY KEY,
    idempotency_key BYTEA NOT NULL UNIQUE,
    settlement_id   BIGINT NOT NULL,
    batch_id        BIGINT,
    command_type    SMALLINT NOT NULL,
    account_id      BIGINT NOT NULL,
    asset_id        INTEGER NOT NULL,
    amount          BIGINT NOT NULL,
    status          SMALLINT NOT NULL DEFAULT 0,
    created_at      BIGINT NOT NULL,
    executed_at     BIGINT,
    -- 执行结果快照
    available_before BIGINT,
    frozen_before    BIGINT,
    available_after  BIGINT,
    frozen_after     BIGINT,
    balance_version  BIGINT,

    INDEX idx_commands_status (status) WHERE status = 0,
    INDEX idx_commands_settlement (settlement_id),
    INDEX idx_commands_batch (batch_id) WHERE batch_id IS NOT NULL
);

-- 命令批次表
CREATE TABLE balance_command_batches (
    batch_id        BIGINT PRIMARY KEY,
    settlement_id   BIGINT NOT NULL,
    command_count   SMALLINT NOT NULL,
    status          SMALLINT NOT NULL DEFAULT 0,
    created_at      BIGINT NOT NULL,
    completed_at    BIGINT,

    INDEX idx_batches_status (status) WHERE status = 0
);
```

#### 5.2.7 Account 模块 Entity 定义

Account 模块负责消费 `BalanceCommand` 并执行余额变更。

##### 实体关系模型

```
┌─────────────────────────────────────────────────────────────────┐
│                           User                                   │
│                          (用户)                                  │
└─────────────────────────────┬───────────────────────────────────┘
                              │ 1:N (一个用户多个账户)
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Account                                  │
│                         (账户)                                   │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐       │
│  │  Spot 账户    │  │ Perp逐仓账户  │  │ Perp全仓账户  │        │
│  │  (现货交易)   │  │ (合约隔离)    │  │ (合约共享)    │        │
│  └───────┬───────┘  └───────┬───────┘  └───────┬───────┘       │
└──────────┼──────────────────┼──────────────────┼────────────────┘
           │ 1:N              │ 1:N              │ 1:N
           ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Balance                                  │
│                         (余额)                                   │
│                                                                  │
│  Spot账户:     [USDT] [BTC] [ETH] [SOL] ...  (多币种)           │
│  Perp逐仓账户: [USDT]                         (单一保证金)       │
│  Perp全仓账户: [USDT]                         (单一保证金)       │
└─────────────────────────────────────────────────────────────────┘
```

##### 关系说明

| 关系 | 基数 | 说明 |
|------|------|------|
| User → Account | 1:N | 一个用户可拥有多个不同类型的账户 |
| Account → Balance | 1:N | 一个账户下有多个资产余额 |
| Balance 主键 | - | `(account_id, asset_id)` 复合主键 |

##### 设计优势

| 优势 | 说明 |
|------|------|
| **风险隔离** | 现货、合约账户余额隔离，风险不传导 |
| **账户管控** | 可单独冻结/解冻某个账户（如风控、合规） |
| **灵活扩展** | 未来可扩展子账户、跟单账户、API 账户等 |
| **查询高效** | Balance 表按 `(account_id, asset_id)` 索引，O(1) |

##### 示例数据

```
用户 Alice (user_id=1001)
│
├── Account #1 (type=Spot, status=Active)
│   ├── Balance(USDT): available=10000, frozen=500
│   ├── Balance(BTC):  available=0.5,   frozen=0.1
│   └── Balance(ETH):  available=5.0,   frozen=0
│
├── Account #2 (type=PerpIsolated, status=Active)
│   └── Balance(USDT): available=5000,  frozen=2000  ← 逐仓保证金
│
└── Account #3 (type=PerpCross, status=Active)
    └── Balance(USDT): available=8000,  frozen=3000  ← 全仓保证金
```

##### 核心实体定义

```rust
//! Account Module Entity Definitions
//!
//! 账户模块核心实体，用于余额管理和命令消费。
//!
//! ## 实体关系
//!
//! - User 1:N Account (一个用户多个账户)
//! - Account 1:N Balance (一个账户多个资产余额)
//! - Balance 主键: (account_id, asset_id)

// ============================================================================
// Account Entity (账户实体)
// ============================================================================

/// 交易账户
///
/// 一个用户可以有多个账户（如现货账户、合约账户），
/// 每个账户下有多个资产余额。
#[repr(align(64))]
pub struct Account {
    /// 账户ID
    pub id: AccountId,

    /// 所属用户ID
    pub user_id: UserId,

    /// 账户类型
    pub account_type: AccountType,

    /// 账户状态
    pub status: AccountStatus,

    /// 创建时间
    pub created_at: Timestamp,

    /// 更新时间
    pub updated_at: Timestamp,
}

/// 用户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct UserId(pub u64);

/// 账户类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccountType {
    /// 现货账户
    Spot = 0,
    /// 合约账户（逐仓）
    PerpIsolated = 1,
    /// 合约账户（全仓）
    PerpCross = 2,
    /// 资金账户
    Funding = 3,
}

/// 账户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccountStatus {
    /// 正常
    Active = 0,
    /// 冻结（禁止交易）
    Frozen = 1,
    /// 注销
    Closed = 2,
}

// ============================================================================
// Balance Entity (余额实体)
// ============================================================================

/// 资产余额
///
/// 账户在某一资产上的余额，包含可用余额和冻结余额。
/// 使用乐观锁（version）保证并发安全。
#[repr(align(64))]
pub struct Balance {
    /// 账户ID
    pub account_id: AccountId,

    /// 资产ID
    pub asset_id: AssetId,

    /// 可用余额（可用于下单、提现）
    pub available: u64,

    /// 冻结余额（已锁定用于挂单、保证金）
    pub frozen: u64,

    /// 乐观锁版本号（每次修改 +1）
    pub version: u64,

    /// 最后更新时间
    pub updated_at: Timestamp,
}

impl Balance {
    /// 创建新余额
    pub fn new(account_id: AccountId, asset_id: AssetId) -> Self {
        Self {
            account_id,
            asset_id,
            available: 0,
            frozen: 0,
            version: 0,
            updated_at: Timestamp::now(),
        }
    }

    /// 总余额 = 可用 + 冻结
    #[inline]
    pub fn total(&self) -> u64 {
        self.available.saturating_add(self.frozen)
    }

    /// 冻结指定金额（可用 → 冻结）
    pub fn freeze(&mut self, amount: u64) -> Result<(), BalanceError> {
        if self.available < amount {
            return Err(BalanceError::InsufficientAvailable);
        }
        self.available -= amount;
        self.frozen += amount;
        self.version += 1;
        self.updated_at = Timestamp::now();
        Ok(())
    }

    /// 解冻指定金额（冻结 → 可用）
    pub fn unfreeze(&mut self, amount: u64) -> Result<(), BalanceError> {
        if self.frozen < amount {
            return Err(BalanceError::InsufficientFrozen);
        }
        self.frozen -= amount;
        self.available += amount;
        self.version += 1;
        self.updated_at = Timestamp::now();
        Ok(())
    }

    /// 增加可用余额（入金、收款）
    pub fn credit(&mut self, amount: u64) -> Result<(), BalanceError> {
        self.available = self.available.checked_add(amount)
            .ok_or(BalanceError::Overflow)?;
        self.version += 1;
        self.updated_at = Timestamp::now();
        Ok(())
    }

    /// 减少可用余额（出金、付款）
    pub fn debit(&mut self, amount: u64) -> Result<(), BalanceError> {
        if self.available < amount {
            return Err(BalanceError::InsufficientAvailable);
        }
        self.available -= amount;
        self.version += 1;
        self.updated_at = Timestamp::now();
        Ok(())
    }

    /// 直接扣减冻结余额（强平、成交扣款）
    pub fn debit_frozen(&mut self, amount: u64) -> Result<(), BalanceError> {
        if self.frozen < amount {
            return Err(BalanceError::InsufficientFrozen);
        }
        self.frozen -= amount;
        self.version += 1;
        self.updated_at = Timestamp::now();
        Ok(())
    }

    /// 创建快照
    pub fn snapshot(&self) -> BalanceSnapshot {
        BalanceSnapshot {
            available_before: Amount(self.available as i64),
            frozen_before: Amount(self.frozen as i64),
            available_after: Amount(self.available as i64),
            frozen_after: Amount(self.frozen as i64),
            version: self.version,
        }
    }
}

/// 余额错误
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceError {
    /// 可用余额不足
    InsufficientAvailable,
    /// 冻结余额不足
    InsufficientFrozen,
    /// 余额溢出
    Overflow,
    /// 账户不存在
    AccountNotFound,
    /// 版本冲突（乐观锁）
    VersionConflict,
}

// ============================================================================
// BalanceStore Trait (余额存储接口)
// ============================================================================

/// 余额存储接口
///
/// Account 模块的核心存储抽象，供 BalanceCommandConsumer 使用。
#[async_trait]
pub trait BalanceStore: Send + Sync {
    /// 获取余额（不存在则创建）
    async fn get_or_create(
        &self,
        account_id: AccountId,
        asset_id: AssetId,
    ) -> Result<Balance, BalanceError>;

    /// 更新余额（带乐观锁检查）
    async fn update(&self, balance: &Balance) -> Result<(), BalanceError>;

    /// 原子执行余额操作
    async fn execute(
        &self,
        account_id: AccountId,
        asset_id: AssetId,
        op: BalanceOp,
    ) -> Result<Balance, BalanceError>;

    /// 批量执行（事务）
    async fn execute_batch(
        &self,
        ops: Vec<(AccountId, AssetId, BalanceOp)>,
    ) -> Result<Vec<Balance>, BalanceError>;
}

/// 余额操作
#[derive(Debug, Clone, Copy)]
pub enum BalanceOp {
    /// 冻结
    Freeze(u64),
    /// 解冻
    Unfreeze(u64),
    /// 增加可用
    Credit(u64),
    /// 减少可用
    Debit(u64),
    /// 减少冻结
    DebitFrozen(u64),
    /// 盈亏结算（可正可负）
    SettlePnl(i64),
}

// ============================================================================
// Balance 数据库表
// ============================================================================

/*
CREATE TABLE balances (
    account_id      BIGINT NOT NULL,
    asset_id        INTEGER NOT NULL,
    available       BIGINT NOT NULL DEFAULT 0,
    frozen          BIGINT NOT NULL DEFAULT 0,
    version         BIGINT NOT NULL DEFAULT 0,
    updated_at      BIGINT NOT NULL,

    PRIMARY KEY (account_id, asset_id),
    CHECK (available >= 0),
    CHECK (frozen >= 0)
);

CREATE TABLE accounts (
    id              BIGINT PRIMARY KEY,
    user_id         BIGINT NOT NULL,
    account_type    SMALLINT NOT NULL,
    status          SMALLINT NOT NULL DEFAULT 0,
    created_at      BIGINT NOT NULL,
    updated_at      BIGINT NOT NULL,

    INDEX idx_accounts_user (user_id)
);
*/
```

### 5.3 结算流程中的账本操作

#### 5.3.1 现货结算流程

```
┌─────────────────────────────────────────────────────────────────┐
│           Spot Trade Settlement (现货结算)                       │
│                                                                 │
│  Trade: Buyer 买入 1 BTC @ 50000 USDT                           │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 1: 创建 Settlement + Entries                         │ │
│  │                                                           │ │
│  │ Settlement {                                              │ │
│  │   type: SpotInstant,                                      │ │
│  │   status: Pending,                                        │ │
│  │ }                                                         │ │
│  │                                                           │ │
│  │ Entries:                                                  │ │
│  │ [0] Buyer:  USDT -50000 (SpotTransfer, Debit)            │ │
│  │ [1] Buyer:  BTC  +1.0   (SpotTransfer, Credit)           │ │
│  │ [2] Seller: BTC  -1.0   (SpotTransfer, Debit)            │ │
│  │ [3] Seller: USDT +50000 (SpotTransfer, Credit)           │ │
│  │ [4] Buyer:  USDT -50    (SpotFee, Debit)                 │ │
│  │ [5] Seller: BTC  -0.001 (SpotFee, Debit)                 │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 2: 生成 BalanceCommandBatch 并持久化                 │ │
│  │                                                           │ │
│  │ commands = [                                              │ │
│  │   BalanceCommand { Buyer,  USDT, TransferOut, 50000 },   │ │
│  │   BalanceCommand { Buyer,  BTC,  TransferIn,  1.0 },     │ │
│  │   BalanceCommand { Seller, BTC,  TransferOut, 1.0 },     │ │
│  │   BalanceCommand { Seller, USDT, TransferIn,  50000 },   │ │
│  │   BalanceCommand { Buyer,  USDT, Debit,       50 },      │ │
│  │   BalanceCommand { Seller, BTC,  Debit,       0.001 },   │ │
│  │ ]                                                         │ │
│  │                                                           │ │
│  │ // 持久化到 AccountCommandRepo                            │ │
│  │ command_repo.save_batch(batch)                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 3: 更新 Settlement 状态                              │ │
│  │                                                           │ │
│  │ settlement.status = Completed                             │ │
│  │ settlement.settled_at = now()                             │ │
│  │                                                           │ │
│  │ // 回填 Entry 的 balance_before/after                     │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

#### 5.3.2 永续开仓结算流程

```
┌─────────────────────────────────────────────────────────────────┐
│           Prep Open Settlement (永续开仓结算)                    │
│                                                                 │
│  Trade: 开多 1 BTC @ 50000 USDT, 10x 杠杆                       │
│  margin = 50000 / 10 = 5000 USDT                               │
│  fee = 50000 * 0.0005 = 25 USDT                                │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 1: 创建 Settlement + Entries                         │ │
│  │                                                           │ │
│  │ Entries:                                                  │ │
│  │ [0] Trader: USDT -5000 (MarginLock, Debit)               │ │
│  │ [1] Trader: USDT -25   (TradingFee, Debit)               │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 2: 生成 BalanceCommandBatch 并持久化                 │ │
│  │                                                           │ │
│  │ commands = [                                              │ │
│  │   BalanceCommand { Trader, USDT, Freeze, 5000 },         │ │
│  │   BalanceCommand { Trader, USDT, Debit,  25 },           │ │
│  │ ]                                                         │ │
│  │                                                           │ │
│  │ // 持久化到 AccountCommandRepo                            │ │
│  │ command_repo.save_batch(batch)                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 3: 调用 PositionGateway 更新仓位                     │ │
│  │                                                           │ │
│  │ position.margin += 5000                                   │ │
│  │ position.quantity += 1.0                                  │ │
│  │ position.avg_price = weighted_avg(...)                    │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ 账本变化:                                                 │ │
│  │                                                           │ │
│  │ Trader Balance:                                           │ │
│  │   available: 100000 → 94975 (-5025)                      │ │
│  │   frozen:    0      → 5000  (+5000)                      │ │
│  │   total:     100000 → 99975 (-25, 手续费)                │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

#### 5.3.3 永续平仓结算流程

```
┌─────────────────────────────────────────────────────────────────┐
│           Prep Close Settlement (永续平仓结算)                   │
│                                                                 │
│  平仓: 卖出 1 BTC @ 52000 USDT (盈利)                           │
│  entry_price = 50000, exit_price = 52000                       │
│  pnl = (52000 - 50000) * 1.0 = +2000 USDT                      │
│  released_margin = 5000 USDT                                   │
│  fee = 52000 * 0.0005 = 26 USDT                                │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 1: 创建 Settlement + Entries                         │ │
│  │                                                           │ │
│  │ Entries:                                                  │ │
│  │ [0] Trader: USDT +5000 (MarginRelease, Credit)           │ │
│  │ [1] Trader: USDT +2000 (RealizedPnl, Credit)             │ │
│  │ [2] Trader: USDT -26   (TradingFee, Debit)               │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Step 2: 生成 BalanceCommandBatch 并持久化                 │ │
│  │                                                           │ │
│  │ commands = [                                              │ │
│  │   BalanceCommand { Trader, USDT, Unfreeze, 5000 },       │ │
│  │   BalanceCommand { Trader, USDT, Credit,   2000 },       │ │
│  │   BalanceCommand { Trader, USDT, Debit,    26 },         │ │
│  │ ]                                                         │ │
│  │                                                           │ │
│  │ // 持久化到 AccountCommandRepo                            │ │
│  │ command_repo.save_batch(batch)                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ 账本变化:                                                 │ │
│  │                                                           │ │
│  │ Trader Balance:                                           │ │
│  │   frozen:    5000  → 0      (-5000, 释放)                │ │
│  │   available: 94975 → 101949 (+5000+2000-26)              │ │
│  │   total:     99975 → 101949 (+1974, 净盈利)              │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 5.4 事务一致性保证

基于 AccountCommandRepo 模式的一致性保证：

```
┌─────────────────────────────────────────────────────────────────┐
│              AccountCommandRepo 一致性模型                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Step 1: 单事务持久化                                            │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ BEGIN TRANSACTION                                       │   │
│  │   INSERT settlement                                     │   │
│  │   INSERT settlement_entries                             │   │
│  │   INSERT balance_commands (batch)                       │   │
│  │ COMMIT                                                  │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│  Step 2: 消费执行（根据策略同步/异步）                            │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ consumer.consume_batch(batch)                           │   │
│  │ UPDATE balance_commands SET status = 'Completed'        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  一致性保证:                                                     │
│  ✓ Settlement + Entry + Command 原子持久化                       │
│  ✓ Command 幂等执行（idempotency_key 去重）                      │
│  ✓ 故障恢复：扫描 Pending 状态重试                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### 5.4.1 消费策略对比

| 消费策略 | 一致性 | 性能 | 适用场景 |
|----------|--------|------|---------|
| **SyncBalanceConsumer** | 强一致 | 最高 | **推荐：单体部署** |
| AsyncBalanceConsumer | 最终一致 | 高 | 同进程高吞吐 |
| RemoteBalanceConsumer | 最终一致 | 中 | 微服务分布式 |

> 详细实现见 Section 5.2.4 消费策略

#### 5.4.2 故障恢复

```rust
/// 恢复未完成的命令（启动时调用）
pub async fn recover_pending_commands(
    command_repo: &dyn AccountCommandRepo,
    consumer: &dyn BalanceCommandConsumer,
) -> Result<usize, RecoveryError> {
    let pending_batches = command_repo.fetch_pending_batches(100).await?;
    let mut recovered = 0;

    for batch in pending_batches {
        // 幂等执行，重复调用安全
        if let Ok(results) = consumer.consume_batch(batch.clone()) {
            command_repo.update_batch_status(
                batch.batch_id,
                CommandStatus::Completed,
                results.into_iter()
                    .enumerate()
                    .map(|(i, r)| (batch.commands[i].id, CommandStatus::Completed, Some(r)))
                    .collect(),
            ).await?;
            recovered += 1;
        }
    }
    Ok(recovered)
}
```

## 6. Spot Settlement

### 6.1 Business Rules

1. **T+0 Instant Settlement**: Transfer funds immediately after matching
2. **Bilateral Deduction**: Buyer deducts quote, seller deducts base
3. **Atomicity**: Either party insufficient balance fails entire trade
4. **Fees**: Deducted from trading parties (configurable Maker/Taker rates)

### 6.2 Clearing Flow

```
Trade(buyer, seller, BTC/USDT, 50000, 1.0)
    |
    v
+------------------------------------------------+
| Clearing Phase                                  |
+------------------------------------------------+
| 1. Validate: buyer has 50000 USDT              |
| 2. Validate: seller has 1.0 BTC                |
| 3. Calculate fees:                              |
|    - buyer_fee = 50000 * 0.001 = 50 USDT       |
|    - seller_fee = 1.0 * 0.001 = 0.001 BTC      |
| 4. Create ClearingRecord                        |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Settlement Phase                                |
+------------------------------------------------+
| SettlementEntry[]:                              |
| [1] buyer:  USDT -50050  (SpotDebit)           |
| [2] buyer:  BTC  +1.0    (SpotCredit)          |
| [3] seller: BTC  -1.001  (SpotDebit)           |
| [4] seller: USDT +50000  (SpotCredit)          |
| [5] system: USDT +50     (TradingFee)          |
| [6] system: BTC  +0.001  (TradingFee)          |
+------------------------------------------------+
```

## 6. Perpetual Settlement

### 6.1 Business Rules

1. **Margin Mechanism**: Lock margin on open, release on close
2. **Per-Trade PnL**: Settle realized PnL on close
3. **Funding Rate**: Settle funding fee every N hours
4. **Liquidation**: Trigger when margin ratio below maintenance margin
5. **ADL**: Auto-deleverage when insurance fund insufficient

### 6.2 Open Position Settlement (SettlePrepOpen)

```
Command::SettlePrepOpen {
    side: Buy, position_side: Long,
    price: 50000, qty: 1.0, leverage: 10
}
    |
    v
+------------------------------------------------+
| Pre-conditions                                  |
+------------------------------------------------+
| - available_balance >= margin + fee            |
| - position is new OR same direction (adding)   |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Calculation                                     |
+------------------------------------------------+
| notional = 50000 * 1.0 = 50000 USDT            |
| margin = notional / leverage = 5000 USDT       |
| fee = notional * 0.0005 = 25 USDT              |
| liquidation_price = calc_liq_price(...)        |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Settlement Entries                              |
+------------------------------------------------+
| [1] trader: USDT -5000 (MarginLock)            |
| [2] trader: USDT -25   (TradingFee)            |
| [3] system: USDT +25   (TradingFee)            |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Result: SettlePrepOpen                          |
+------------------------------------------------+
| available_balance: 94975 (was 100000)          |
| position_margin: 5000                           |
| position_avg_price: 50000                       |
| position_quantity: 1.0                          |
| liquidation_price: 45500 (example)              |
+------------------------------------------------+
```

### 6.3 Close Position Settlement (SettlePrepClose)

```
Command::SettlePrepClose {
    side: Sell, position_side: Long,
    price: 52000, qty: 1.0,
    entry_price: 50000, released_margin: 5000
}
    |
    v
+------------------------------------------------+
| Pre-conditions                                  |
+------------------------------------------------+
| - position exists and is open                  |
| - quantity == position.quantity (full close)   |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| PnL Calculation (Long)                          |
+------------------------------------------------+
| realized_pnl = (exit - entry) * qty            |
|              = (52000 - 50000) * 1.0 = +2000   |
| fee = 52000 * 1.0 * 0.0005 = 26 USDT          |
| net_credit = margin + pnl - fee                |
|            = 5000 + 2000 - 26 = 6974 USDT      |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Settlement Entries                              |
+------------------------------------------------+
| [1] trader: USDT +5000 (MarginRelease)         |
| [2] trader: USDT +2000 (RealizedPnl)           |
| [3] trader: USDT -26   (TradingFee)            |
| [4] system: USDT +26   (TradingFee)            |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Result: SettlePrepClose                         |
+------------------------------------------------+
| available_balance: 101974 (was 95000)          |
| released_margin: 5000                           |
| realized_pnl: +2000                             |
| net_amount: +6974                               |
| position: CLOSED                                |
+------------------------------------------------+
```

### 6.4 Reduce Position Settlement (SettlePrepReduce)

```
Command::SettlePrepReduce {
    side: Sell, position_side: Long,
    price: 52000, reduce_qty: 0.5,
    remaining_qty: 0.5, entry_price: 50000
}
    |
    v
+------------------------------------------------+
| Pre-conditions                                  |
+------------------------------------------------+
| - position exists and is open                  |
| - reduce_qty < position.quantity               |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Proportional Calculation                        |
+------------------------------------------------+
| reduce_ratio = 0.5 / 1.0 = 50%                 |
| released_margin = 5000 * 50% = 2500 USDT       |
| remaining_margin = 5000 * 50% = 2500 USDT      |
| realized_pnl = (52000 - 50000) * 0.5 = +1000   |
| fee = 52000 * 0.5 * 0.0005 = 13 USDT          |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Settlement Entries                              |
+------------------------------------------------+
| [1] trader: USDT +2500 (MarginRelease)         |
| [2] trader: USDT +1000 (RealizedPnl)           |
| [3] trader: USDT -13   (TradingFee)            |
| [4] system: USDT +13   (TradingFee)            |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Result: SettlePrepReduce                        |
+------------------------------------------------+
| available_balance: 98487 (was 95000)           |
| released_margin: 2500                           |
| remaining_margin: 2500                          |
| remaining_quantity: 0.5                         |
| realized_pnl: +1000                             |
| liquidation_price: recalculated                 |
+------------------------------------------------+
```

### 6.5 Funding Rate Settlement

```
FundingSettlement(funding_rate=0.0001, settlement_time=T)
    |
    v
+------------------------------------------------+
| For each position:                              |
+------------------------------------------------+
| position_value = mark_price * quantity         |
| funding_fee = position_value * funding_rate    |
|                                                 |
| if LONG:  pay funding_fee (fee > 0)            |
| if SHORT: receive funding_fee (fee < 0)        |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Settlement Entries (per position)               |
+------------------------------------------------+
| LONG:  -funding_fee (FundingFee)               |
| SHORT: +funding_fee (FundingFee)               |
+------------------------------------------------+
```

### 6.5 Liquidation Settlement

```
Liquidation(position, liquidation_price, bankruptcy_price)
    |
    v
+------------------------------------------------+
| Liquidation Calculation                         |
+------------------------------------------------+
| liquidation_value = liquidation_price * qty    |
| bankruptcy_value = bankruptcy_price * qty      |
| surplus = liquidation_value - bankruptcy_value |
| liquidation_fee = surplus * 0.5 (example)      |
| insurance_contribution = surplus - fee         |
+------------------------------------------------+
    |
    v
+------------------------------------------------+
| Settlement Entries                              |
+------------------------------------------------+
| [1] trader: -margin        (MarginRelease)     |
| [2] trader: -loss          (RealizedPnl)       |
| [3] trader: -fee           (LiquidationPenalty)|
| [4] insurance: +surplus    (InsuranceFund)     |
+------------------------------------------------+
```

## 7. Repository Interfaces

```rust
/// Clearing Record Repository
#[async_trait]
pub trait ClearingRepository: Send + Sync {
    async fn save(&self, record: ClearingRecord) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: ClearingId) -> Result<Option<ClearingRecord>, RepositoryError>;
    async fn find_by_trade(&self, trade_id: TradeId) -> Result<Option<ClearingRecord>, RepositoryError>;
    async fn find_pending(&self) -> Result<Vec<ClearingRecord>, RepositoryError>;
}

/// Settlement Repository
#[async_trait]
pub trait SettlementRepository: Send + Sync {
    async fn save(&self, settlement: Settlement) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: SettlementId) -> Result<Option<Settlement>, RepositoryError>;
    async fn find_by_account(
        &self,
        account_id: AccountId,
        from: Timestamp,
        to: Timestamp,
    ) -> Result<Vec<Settlement>, RepositoryError>;
    async fn find_pending(&self) -> Result<Vec<Settlement>, RepositoryError>;
    async fn update_status(&self, id: SettlementId, status: SettlementStatus, version: u64) -> Result<bool, RepositoryError>;
}

/// Settlement Entry Repository
#[async_trait]
pub trait SettlementEntryRepository: Send + Sync {
    async fn save_batch(&self, entries: Vec<SettlementEntry>) -> Result<(), RepositoryError>;
    async fn find_by_settlement(&self, settlement_id: SettlementId) -> Result<Vec<SettlementEntry>, RepositoryError>;
    async fn find_by_account(
        &self,
        account_id: AccountId,
        entry_type: Option<EntryType>,
        from: Timestamp,
        to: Timestamp,
    ) -> Result<Vec<SettlementEntry>, RepositoryError>;
}
```

## 8. External Ports

### 8.1 Outbound Ports

> **设计变更**: 原 `AccountGateway` 已替换为 `AccountCommandRepo` 模式，
> 详见 Section 5.2 AccountCommandRepo 模式。
> Settlement 模块不再直接操作 Account 余额，而是生成 BalanceCommand 由 Account 模块消费。

```rust
/// Position Gateway (仓位网关)
///
/// 用于查询和更新仓位信息
#[async_trait]
pub trait PositionGateway: Send + Sync {
    /// 获取仓位
    async fn get_position(&self, position_id: PositionId) -> Result<Option<Position>, GatewayError>;

    /// 获取交易者所有仓位
    async fn get_positions_by_trader(&self, trader_id: TraderId) -> Result<Vec<Position>, GatewayError>;

    /// 更新仓位
    async fn update_position(&self, position: Position) -> Result<(), GatewayError>;

    /// 关闭仓位
    async fn close_position(&self, position_id: PositionId) -> Result<(), GatewayError>;
}

/// Settlement Event Publisher (结算事件发布)
///
/// 用于发布结算相关事件
#[async_trait]
pub trait SettlementEventPublisher: Send + Sync {
    /// 发布结算完成事件
    async fn publish_settlement_completed(&self, settlement: &Settlement) -> Result<(), GatewayError>;

    /// 发布强平事件
    async fn publish_liquidation(&self, position_id: PositionId, price: Price) -> Result<(), GatewayError>;

    /// 发布资金费率结算事件
    async fn publish_funding_settled(&self, settlement_time: Timestamp, rate: i64) -> Result<(), GatewayError>;
}
```

### 8.2 Account 交互模式

Settlement 与 Account 的交互采用 **命令仓储模式** (Command Repository Pattern):

```
┌─────────────────────────────────────────────────────────────────┐
│                    Settlement → Account 交互                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Settlement Module              Account Module                 │
│   ┌──────────────┐              ┌──────────────┐               │
│   │ Settlement   │              │   Balance    │               │
│   │   Service    │              │   Consumer   │               │
│   └──────┬───────┘              └──────▲───────┘               │
│          │                             │                        │
│          │ save()                      │ consume()              │
│          ▼                             │                        │
│   ┌──────────────────────────────────────────┐                 │
│   │          AccountCommandRepo              │                 │
│   │     (BalanceCommand 命令仓储)              │                 │
│   └──────────────────────────────────────────┘                 │
│                                                                 │
│   部署方式:                                                      │
│   ├─ 同线程: SyncBalanceConsumer (直接调用)                      │
│   ├─ 跨线程: AsyncBalanceConsumer (Channel)                     │
│   └─ 跨主机: RemoteBalanceConsumer (Kafka/MQ)                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

详细接口定义见 Section 5.2.3 AccountCommandRepo 接口。

## 9. Directory Structure

```
lib/core/settlement/
├── Cargo.toml
├── design.md
└── src/
    ├── lib.rs
    │
    ├── domain/                          # 领域层（核心）
    │   ├── mod.rs
    │   ├── entity/                      # 实体和值对象
    │   │   ├── mod.rs                   # Settlement, Entry, ClearingRecord
    │   │   └── balance_command.rs       # BalanceCommand, BalanceCommandBatch
    │   ├── service/                     # 领域服务
    │   │   ├── mod.rs
    │   │   ├── command.rs               # Settlement Command definitions
    │   │   ├── spot_settlement.rs       # 现货结算逻辑
    │   │   └── prep_settlement.rs       # 永续结算逻辑
    │   └── repository/                  # 仓储接口 (trait)
    │       ├── mod.rs
    │       ├── settlement_repo.rs       # SettlementRepository
    │       ├── entry_repo.rs            # SettlementEntryRepository
    │       └── account_command_repo.rs  # AccountCommandRepo ★
    │
    ├── application/                     # 应用层（用例）
    │   ├── mod.rs
    │   ├── ports/                       # 端口接口
    │   │   ├── mod.rs
    │   │   ├── position_gateway.rs      # PositionGateway
    │   │   └── event_publisher.rs       # SettlementEventPublisher
    │   ├── usecases/                    # 用例实现
    │   │   ├── mod.rs
    │   │   ├── settle_spot_trade.rs     # 现货结算用例
    │   │   ├── settle_prep_open.rs      # 永续开仓结算
    │   │   ├── settle_prep_close.rs     # 永续平仓结算
    │   │   ├── settle_funding.rs        # 资金费率结算
    │   │   └── settle_liquidation.rs    # 强平结算
    │   └── settlement_service.rs        # SettlementService (编排)
    │
    └── adaptor/                         # 适配器层（基础设施）
        ├── mod.rs
        ├── inbound/                     # 入站适配器
        │   ├── mod.rs
        │   └── trade_handler.rs         # 接收撮合结果
        └── outbound/                    # 出站适配器
            ├── mod.rs
            ├── postgres/                # PostgreSQL 实现
            │   ├── mod.rs
            │   ├── settlement_repo.rs
            │   ├── entry_repo.rs
            │   └── account_command_repo.rs
            ├── in_memory/               # 内存实现（测试用）
            │   ├── mod.rs
            │   ├── settlement_repo.rs
            │   └── account_command_repo.rs
            ├── position_gateway.rs      # PositionGateway 实现
            └── balance_consumer/        # BalanceCommand 消费者 ★
                ├── mod.rs
                ├── sync_consumer.rs     # 同线程消费
                ├── async_consumer.rs    # 跨线程消费
                └── remote_consumer.rs   # 跨主机消费
```

### 9.1 模块职责说明

| 目录 | 职责 | 依赖方向 |
|------|------|----------|
| `domain/entity` | 值对象、实体、聚合根定义 | 无外部依赖 |
| `domain/service` | 领域服务、Command 定义 | 仅依赖 entity |
| `domain/repository` | 仓储接口 (trait) | 仅依赖 entity |
| `application/ports` | 外部系统端口接口 | 仅依赖 domain |
| `application/usecases` | 用例编排 | 依赖 domain + ports |
| `adaptor/outbound` | 仓储/网关具体实现 | 实现 domain/application 接口 |

### 9.2 AccountCommandRepo 核心文件

```
关键文件:
├── domain/entity/balance_command.rs      # BalanceCommand 定义
├── domain/repository/account_command_repo.rs  # AccountCommandRepo trait
└── adaptor/outbound/balance_consumer/    # 消费者实现
    ├── sync_consumer.rs                  # 同进程直接执行
    ├── async_consumer.rs                 # Channel 异步执行
    └── remote_consumer.rs                # Kafka/MQ 远程执行
```

## 10. Performance Optimization

### 10.1 Low Latency Design

1. **Cache Line Alignment**: Settlement and SettlementEntry use 64-byte alignment
2. **Pre-allocation**: Use object pool to avoid runtime allocation
3. **Batch Processing**: Merge settlements from same matching batch
4. **Lock-free Design**: Settlement entries use append-only writes

### 10.2 Concurrency Control

```rust
/// Optimistic lock version control
pub struct VersionedSettlement {
    pub settlement: Settlement,
    pub version: u64,
}

/// Atomic balance update
pub struct AtomicBalance {
    value: AtomicI64,
}

impl AtomicBalance {
    pub fn add(&self, delta: i64) -> i64 {
        self.value.fetch_add(delta, Ordering::AcqRel)
    }
}
```

## 11. Test Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spot_settlement_command() {
        let cmd = Command::SettleSpotTrade {
            trade_id: 1,
            buyer_account: 100,
            seller_account: 101,
            base_asset: Asset::Base("BTC".into()),
            quote_asset: Asset::Quote("USDT".into()),
            price: 50000_00,
            quantity: 100,  // 0.01 BTC
            buyer_fee: 50,
            seller_fee: 1,
            buyer_is_maker: false,
        };

        match cmd {
            Command::SettleSpotTrade { price, quantity, .. } => {
                assert_eq!(price, 50000_00);
                assert_eq!(quantity, 100);
            }
            _ => panic!("Expected SettleSpotTrade"),
        }
    }

    #[test]
    fn test_prep_open_settlement() {
        // Open Long: Buy 1 BTC at 50000 USDT with 10x leverage
        // notional = 50000, margin = 5000, fee = 25
        let cmd = Command::SettlePrepOpen {
            trade_id: 1,
            account: 100,
            position_id: 1,
            margin_asset: Asset::Margin("USDT".into()),
            side: Side::Buy,
            position_side: PositionSide::Long,
            price: 50000_00,
            quantity: 100,         // 1.0 BTC (scaled by 100)
            notional: 50000_00,    // 50000 USDT
            margin: 5000_00,       // 5000 USDT (10x leverage)
            leverage: 10,
            fee: 25_00,            // 25 USDT
            is_maker: false,
        };

        match cmd {
            Command::SettlePrepOpen { margin, leverage, notional, .. } => {
                assert_eq!(notional / leverage as i64, margin);
                assert_eq!(leverage, 10);
            }
            _ => panic!("Expected SettlePrepOpen"),
        }
    }

    #[test]
    fn test_prep_close_settlement() {
        // Close Long: Sell 1 BTC at 52000 USDT
        // entry = 50000, exit = 52000, pnl = +2000
        let cmd = Command::SettlePrepClose {
            trade_id: 2,
            account: 100,
            position_id: 1,
            margin_asset: Asset::Margin("USDT".into()),
            side: Side::Sell,
            position_side: PositionSide::Long,
            price: 52000_00,
            quantity: 100,
            entry_price: 50000_00,
            released_margin: 5000_00,
            realized_pnl: 2000_00,  // (52000 - 50000) * 1.0 = 2000
            fee: 26_00,
            is_maker: false,
        };

        match cmd {
            Command::SettlePrepClose { realized_pnl, released_margin, entry_price, price, .. } => {
                // Verify PnL calculation: (exit - entry) * quantity
                let expected_pnl = (price as i64 - entry_price as i64) * 100 / 100;
                assert_eq!(realized_pnl, expected_pnl);
                assert_eq!(released_margin, 5000_00);
            }
            _ => panic!("Expected SettlePrepClose"),
        }
    }

    #[test]
    fn test_prep_reduce_settlement() {
        // Partial Close: Reduce Long by 0.5 BTC at 52000 USDT
        // Original: 1 BTC at 50000, margin = 5000
        // After: 0.5 BTC, margin = 2500
        let cmd = Command::SettlePrepReduce {
            trade_id: 3,
            account: 100,
            position_id: 1,
            margin_asset: Asset::Margin("USDT".into()),
            side: Side::Sell,
            position_side: PositionSide::Long,
            price: 52000_00,
            reduce_quantity: 50,         // 0.5 BTC
            remaining_quantity: 50,      // 0.5 BTC remaining
            entry_price: 50000_00,
            released_margin: 2500_00,    // 50% of 5000
            remaining_margin: 2500_00,   // 50% of 5000
            realized_pnl: 1000_00,       // (52000 - 50000) * 0.5 = 1000
            fee: 13_00,
            is_maker: false,
        };

        match cmd {
            Command::SettlePrepReduce {
                reduce_quantity,
                remaining_quantity,
                released_margin,
                remaining_margin,
                realized_pnl,
                ..
            } => {
                // Verify proportional release
                assert_eq!(reduce_quantity, remaining_quantity);  // 50/50 split
                assert_eq!(released_margin, remaining_margin);    // Equal split
                // Verify partial PnL
                assert_eq!(realized_pnl, 1000_00);
            }
            _ => panic!("Expected SettlePrepReduce"),
        }
    }

    #[test]
    fn test_prep_open_short_settlement() {
        // Open Short: Sell 1 BTC at 50000 USDT with 10x leverage
        let cmd = Command::SettlePrepOpen {
            trade_id: 4,
            account: 200,
            position_id: 2,
            margin_asset: Asset::Margin("USDT".into()),
            side: Side::Sell,
            position_side: PositionSide::Short,
            price: 50000_00,
            quantity: 100,
            notional: 50000_00,
            margin: 5000_00,
            leverage: 10,
            fee: 25_00,
            is_maker: true,
        };

        match cmd {
            Command::SettlePrepOpen { side, position_side, .. } => {
                assert_eq!(side, Side::Sell);
                assert_eq!(position_side, PositionSide::Short);
            }
            _ => panic!("Expected SettlePrepOpen"),
        }
    }

    #[test]
    fn test_prep_close_short_profit() {
        // Close Short at profit: Buy 1 BTC at 48000 USDT (entry was 50000)
        // pnl = (50000 - 48000) * 1.0 = +2000 (short profits when price drops)
        let cmd = Command::SettlePrepClose {
            trade_id: 5,
            account: 200,
            position_id: 2,
            margin_asset: Asset::Margin("USDT".into()),
            side: Side::Buy,
            position_side: PositionSide::Short,
            price: 48000_00,
            quantity: 100,
            entry_price: 50000_00,
            released_margin: 5000_00,
            realized_pnl: 2000_00,  // (50000 - 48000) * 1.0 = 2000 (short direction)
            fee: 24_00,
            is_maker: false,
        };

        match cmd {
            Command::SettlePrepClose { realized_pnl, entry_price, price, .. } => {
                // Short PnL = (entry - exit) * quantity
                let expected_pnl = (entry_price as i64 - price as i64) * 100 / 100;
                assert_eq!(realized_pnl, expected_pnl);
            }
            _ => panic!("Expected SettlePrepClose"),
        }
    }

    #[test]
    fn test_funding_settlement() {
        let entries = vec![
            FundingEntry {
                trader_id: 1,
                account: 100,
                position_id: 1,
                position_side: PositionSide::Long,
                position_value: 50000_00,
                funding_fee: 50,  // pay
            },
            FundingEntry {
                trader_id: 2,
                account: 101,
                position_id: 2,
                position_side: PositionSide::Short,
                funding_fee: -50,  // receive
            },
        ];

        let cmd = Command::SettleFunding {
            settlement_time: 1700000000,
            funding_rate: 100,  // 0.0001 = 100 / 1e6
            entries,
        };

        match cmd {
            Command::SettleFunding { entries, .. } => {
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0].funding_fee + entries[1].funding_fee, 0);  // zero-sum
            }
            _ => panic!("Expected SettleFunding"),
        }
    }
}
```

---

**Document Version**: v1.0.0
**Last Updated**: 2025-12-01
