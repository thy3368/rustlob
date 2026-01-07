# Funds Module Design Document

## 1. Overview

Funds 模块负责资产管理业务，包括出入金（充值/提现）、内部转账、资产冻结等核心资金操作。该模块与 Account 模块协同，通过 BalanceCommand 驱动账户余额变更。

### 1.1 模块职责

| 模块 | 职责 |
|------|------|
| **Funds** | 出入金业务逻辑、转账路由、风控检查、审批流程 |
| **Account** | 余额维护、冻结/解冻、余额快照 |
| **Settlement** | 交易结算、盈亏计算、保证金管理 |

### 1.2 业务流转

```
[External]              [Funds]                [Account]
    |                      |                      |
 Deposit Request  ->   DepositCmd          ->  BalanceCmd(Credit)
 Withdraw Request ->   WithdrawCmd         ->  BalanceCmd(Debit)
 Transfer Request ->   TransferCmd         ->  BalanceCmd(Debit/Credit)
    |                      |                      |
    +--- Audit Trail ------+--- Balance Update ---+
```

## 2. Command Design

### 2.1 Priority Hierarchy

| Priority | Category | Commands |
|----------|----------|----------|
| P0 | Core Funds | Deposit, Withdraw, InternalTransfer |
| P1 | Asset Freeze | FreezeAsset, UnfreezeAsset |
| P2 | Admin Ops | AdjustBalance, ReverseTransaction |
| P3 | Batch Ops | BatchDeposit, BatchWithdraw |

### 2.2 Command Definition

```rust
//! Funds Command Definitions
//!
//! ## Priority Hierarchy
//!
//! - P0: Core Funds (Deposit, Withdraw, InternalTransfer)
//! - P1: Asset Freeze (FreezeAsset, UnfreezeAsset)
//! - P2: Admin Operations (AdjustBalance, ReverseTransaction)
//! - P3: Batch Operations (BatchDeposit, BatchWithdraw)
//!
//! ## 设计原则
//!
//! - 使用 AssetId(u32) 高性能资产标识
//! - 幂等性: 每个命令携带 IdempotencyKey
//! - 审计追踪: 完整记录出入金流水
//! - 风控前置: 命令执行前进行风控检查

use crate::domain::entity::{
    FundsId, TransactionId, AccountId, AssetId, Amount, Timestamp,
    IdempotencyKey, FundsType, FundsStatus, FreezeReason,
    ChannelId, NetworkId, ChainId,
};

/// Funds Commands
#[derive(Debug, Clone)]
pub enum Command {
    // ==================== P0: Core Funds ====================

    /// 充值/入金
    ///
    /// 外部资产转入平台账户
    ///
    /// 生成 BalanceCommand:
    /// - Credit(asset_id, amount)
    ///
    /// 触发 Event:
    /// - DepositCompleted { account, asset, amount, tx_hash }
    Deposit {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 目标账户
        account_id: AccountId,
        /// 资产ID
        asset_id: AssetId,
        /// 充值金额
        amount: Amount,
        /// 充值渠道
        channel_id: ChannelId,
        /// 外部交易哈希 (区块链 tx_hash 或银行流水号)
        external_tx_id: [u8; 32],
        /// 来源地址/账号
        source_address: Option<[u8; 64]>,
        /// 区块链网络 (crypto only)
        network_id: Option<NetworkId>,
        /// 确认数 (crypto only)
        confirmations: Option<u32>,
    },

    /// 提现/出金
    ///
    /// 平台资产转出到外部
    ///
    /// 生成 BalanceCommand:
    /// - Debit(asset_id, amount + fee)
    /// - Fee(asset_id, fee)
    ///
    /// 状态机:
    /// Pending -> Processing -> Completed/Failed
    Withdraw {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 来源账户
        account_id: AccountId,
        /// 资产ID
        asset_id: AssetId,
        /// 提现金额
        amount: Amount,
        /// 提现手续费
        fee: Amount,
        /// 提现渠道
        channel_id: ChannelId,
        /// 目标地址/账号
        target_address: [u8; 64],
        /// 区块链网络 (crypto only)
        network_id: Option<NetworkId>,
        /// 链ID (crypto only)
        chain_id: Option<ChainId>,
        /// 备注
        memo: Option<u64>,
    },

    /// 内部转账
    ///
    /// 平台内账户间转账（零手续费）
    ///
    /// 生成 BalanceCommand:
    /// - from: Debit(asset_id, amount)
    /// - to: Credit(asset_id, amount)
    InternalTransfer {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 来源账户
        from_account: AccountId,
        /// 目标账户
        to_account: AccountId,
        /// 资产ID
        asset_id: AssetId,
        /// 转账金额
        amount: Amount,
        /// 转账类型
        transfer_type: TransferType,
        /// 关联业务ID
        reference_id: Option<u64>,
    },

    // ==================== P1: Asset Freeze ====================

    /// 冻结资产
    ///
    /// 将账户可用余额转为冻结状态
    ///
    /// 生成 BalanceCommand:
    /// - Freeze(asset_id, amount)
    FreezeAsset {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 账户ID
        account_id: AccountId,
        /// 资产ID
        asset_id: AssetId,
        /// 冻结金额
        amount: Amount,
        /// 冻结原因
        reason: FreezeReason,
        /// 关联业务ID
        reference_id: u64,
        /// 过期时间 (None = 永久)
        expires_at: Option<Timestamp>,
    },

    /// 解冻资产
    ///
    /// 将冻结余额转回可用状态
    ///
    /// 生成 BalanceCommand:
    /// - Unfreeze(asset_id, amount)
    UnfreezeAsset {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 账户ID
        account_id: AccountId,
        /// 资产ID
        asset_id: AssetId,
        /// 解冻金额
        amount: Amount,
        /// 关联冻结记录ID
        freeze_id: FundsId,
    },

    /// 扣除冻结资产
    ///
    /// 直接从冻结余额扣款（如强平、违约）
    ///
    /// 生成 BalanceCommand:
    /// - DebitFrozen(asset_id, amount)
    DebitFrozen {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 账户ID
        account_id: AccountId,
        /// 资产ID
        asset_id: AssetId,
        /// 扣除金额
        amount: Amount,
        /// 关联冻结记录ID
        freeze_id: FundsId,
        /// 扣款原因
        reason: DebitReason,
    },

    // ==================== P2: Admin Operations ====================

    /// 调账
    ///
    /// 管理员人工调整账户余额
    ///
    /// 生成 BalanceCommand:
    /// - amount > 0: Credit
    /// - amount < 0: Debit
    AdjustBalance {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 账户ID
        account_id: AccountId,
        /// 资产ID
        asset_id: AssetId,
        /// 调账金额 (正=增加, 负=减少)
        amount: i64,
        /// 调账原因
        reason: AdjustReason,
        /// 操作员ID
        operator_id: u64,
        /// 审批单号
        approval_id: u64,
    },

    /// 冲正交易
    ///
    /// 撤销已完成的出入金交易
    ReverseTransaction {
        /// 幂等键
        idempotency_key: IdempotencyKey,
        /// 原交易ID
        original_tx_id: TransactionId,
        /// 冲正原因
        reason: String,
        /// 操作员ID
        operator_id: u64,
        /// 审批单号
        approval_id: u64,
    },

    // ==================== P3: Batch Operations ====================

    /// 批量充值
    BatchDeposit {
        /// 批次ID
        batch_id: u64,
        /// 充值列表
        deposits: Vec<DepositItem>,
    },

    /// 批量提现
    BatchWithdraw {
        /// 批次ID
        batch_id: u64,
        /// 提现列表
        withdrawals: Vec<WithdrawItem>,
    },

    /// 确认提现
    ///
    /// 运营确认提现请求（人工审核后）
    ConfirmWithdraw {
        /// 交易ID
        transaction_id: TransactionId,
        /// 操作员ID
        operator_id: u64,
        /// 外部交易哈希
        external_tx_id: Option<[u8; 32]>,
    },

    /// 拒绝提现
    RejectWithdraw {
        /// 交易ID
        transaction_id: TransactionId,
        /// 拒绝原因
        reason: RejectReason,
        /// 操作员ID
        operator_id: u64,
    },
}

/// 转账类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransferType {
    /// 用户间转账
    UserToUser = 0,
    /// 现货账户到合约账户
    SpotToContract = 1,
    /// 合约账户到现货账户
    ContractToSpot = 2,
    /// 主账户到子账户
    MainToSub = 3,
    /// 子账户到主账户
    SubToMain = 4,
    /// 系统转账
    System = 5,
}

/// 冻结原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FreezeReason {
    /// 提现冻结
    Withdraw = 0,
    /// 订单冻结
    Order = 1,
    /// 风控冻结
    RiskControl = 2,
    /// 法务冻结
    Legal = 3,
    /// 清算冻结
    Liquidation = 4,
    /// 其他
    Other = 255,
}

/// 扣款原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DebitReason {
    /// 提现成功
    WithdrawSuccess = 0,
    /// 强制平仓
    Liquidation = 1,
    /// 违约扣款
    Default = 2,
    /// 手续费
    Fee = 3,
    /// 罚款
    Penalty = 4,
}

/// 调账原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdjustReason {
    /// 系统补偿
    Compensation = 0,
    /// 活动奖励
    Promotion = 1,
    /// 交易对账差异
    Reconciliation = 2,
    /// 测试用途
    Testing = 3,
    /// 其他
    Other = 255,
}

/// 拒绝原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RejectReason {
    /// 风控拒绝
    RiskControl = 0,
    /// 地址无效
    InvalidAddress = 1,
    /// 金额异常
    AbnormalAmount = 2,
    /// 用户请求取消
    UserCancelled = 3,
    /// 其他
    Other = 255,
}

/// 充值条目（批量）
#[derive(Debug, Clone)]
pub struct DepositItem {
    pub idempotency_key: IdempotencyKey,
    pub account_id: AccountId,
    pub asset_id: AssetId,
    pub amount: Amount,
    pub external_tx_id: [u8; 32],
}

/// 提现条目（批量）
#[derive(Debug, Clone)]
pub struct WithdrawItem {
    pub idempotency_key: IdempotencyKey,
    pub account_id: AccountId,
    pub asset_id: AssetId,
    pub amount: Amount,
    pub fee: Amount,
    pub target_address: [u8; 64],
}
```

### 2.3 Command Result Definition

```rust
/// Error Code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Balance related (1xxx)
    InsufficientBalance = 1001,
    InsufficientFrozen = 1002,
    BalanceOverflow = 1003,
    AmountTooSmall = 1004,
    AmountTooLarge = 1005,

    // Account related (2xxx)
    AccountNotFound = 2001,
    AccountFrozen = 2002,
    AccountDisabled = 2003,
    SubAccountNotAllowed = 2004,

    // Transaction related (3xxx)
    TransactionNotFound = 3001,
    TransactionAlreadyCompleted = 3002,
    TransactionAlreadyReversed = 3003,
    DuplicateTransaction = 3004,
    InvalidTransactionStatus = 3005,

    // Address related (4xxx)
    InvalidAddress = 4001,
    AddressBlacklisted = 4002,
    AddressNotWhitelisted = 4003,
    UnsupportedNetwork = 4004,

    // Risk Control (5xxx)
    WithdrawLimitExceeded = 5001,
    DailyLimitExceeded = 5002,
    RiskControlBlocked = 5003,
    KYCRequired = 5004,
    CoolingPeriod = 5005,

    // System (9xxx)
    ChannelUnavailable = 9001,
    NetworkCongestion = 9002,
    MaintenanceMode = 9003,
    SystemError = 9999,
}

/// Command Result
#[derive(Debug, Clone)]
pub enum CommandResult {
    // ==================== P0: Core Funds Results ====================

    /// 充值结果
    Deposit {
        /// 交易ID
        transaction_id: TransactionId,
        /// 资金记录ID
        funds_id: FundsId,
        /// 账户新余额
        new_balance: Amount,
        /// 状态
        status: FundsStatus,
        /// 到账时间
        completed_at: Timestamp,
    },

    /// 提现结果
    Withdraw {
        /// 交易ID
        transaction_id: TransactionId,
        /// 资金记录ID
        funds_id: FundsId,
        /// 扣除金额 (amount + fee)
        deducted_amount: Amount,
        /// 账户新余额
        new_balance: Amount,
        /// 状态
        status: FundsStatus,
        /// 预计到账时间
        estimated_arrival: Option<Timestamp>,
    },

    /// 内部转账结果
    InternalTransfer {
        /// 交易ID
        transaction_id: TransactionId,
        /// 资金记录ID
        funds_id: FundsId,
        /// 转出账户新余额
        from_new_balance: Amount,
        /// 转入账户新余额
        to_new_balance: Amount,
        /// 状态
        status: FundsStatus,
    },

    // ==================== P1: Asset Freeze Results ====================

    /// 冻结结果
    FreezeAsset {
        /// 冻结记录ID
        freeze_id: FundsId,
        /// 可用余额
        available_balance: Amount,
        /// 冻结余额
        frozen_balance: Amount,
    },

    /// 解冻结果
    UnfreezeAsset {
        /// 资金记录ID
        funds_id: FundsId,
        /// 可用余额
        available_balance: Amount,
        /// 冻结余额
        frozen_balance: Amount,
    },

    /// 扣除冻结结果
    DebitFrozen {
        /// 资金记录ID
        funds_id: FundsId,
        /// 冻结余额
        frozen_balance: Amount,
        /// 扣除金额
        debited_amount: Amount,
    },

    // ==================== P2: Admin Results ====================

    /// 调账结果
    AdjustBalance {
        /// 资金记录ID
        funds_id: FundsId,
        /// 账户新余额
        new_balance: Amount,
        /// 调整金额
        adjusted_amount: i64,
    },

    /// 冲正结果
    ReverseTransaction {
        /// 冲正交易ID
        reverse_tx_id: TransactionId,
        /// 原交易ID
        original_tx_id: TransactionId,
        /// 账户新余额
        new_balance: Amount,
    },

    // ==================== P3: Batch Results ====================

    /// 批量充值结果
    BatchDeposit {
        /// 批次ID
        batch_id: u64,
        /// 成功数量
        success_count: usize,
        /// 失败数量
        failure_count: usize,
        /// 交易ID列表
        transaction_ids: Vec<TransactionId>,
    },

    /// 批量提现结果
    BatchWithdraw {
        /// 批次ID
        batch_id: u64,
        /// 成功数量
        success_count: usize,
        /// 失败数量
        failure_count: usize,
        /// 交易ID列表
        transaction_ids: Vec<TransactionId>,
    },

    /// 确认提现结果
    ConfirmWithdraw {
        /// 交易ID
        transaction_id: TransactionId,
        /// 新状态
        new_status: FundsStatus,
        /// 外部交易哈希
        external_tx_id: Option<[u8; 32]>,
    },

    /// 拒绝提现结果
    RejectWithdraw {
        /// 交易ID
        transaction_id: TransactionId,
        /// 解冻金额
        unfrozen_amount: Amount,
        /// 账户新可用余额
        new_available_balance: Amount,
    },

    /// 错误结果
    Error {
        /// 错误码
        code: ErrorCode,
        /// 错误信息
        message: String,
        /// 关联实体ID
        related_id: Option<u64>,
    },
}
```

## 3. Domain Entities

### 3.1 设计原则

1. **聚合边界清晰**: FundsRecord 为聚合根
2. **类型安全**: newtype 包装基础类型
3. **高性能**: 64 字节缓存行对齐
4. **幂等性**: IdempotencyKey 防重复
5. **完整审计**: 所有操作可追溯

### 3.2 Value Objects

```rust
//! Funds Domain Entities
//!
//! 资金模块领域实体，遵循 DDD 设计原则

// ============================================================================
// Value Objects (值对象)
// ============================================================================

/// 资金记录ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FundsId(pub u64);

/// 交易ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransactionId(pub u64);

/// 账户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AccountId(pub u64);

/// 资产ID (u32 高性能)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AssetId(pub u32);

/// 渠道ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ChannelId(pub u16);

/// 网络ID (区块链网络)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NetworkId(pub u16);

/// 链ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ChainId(pub u64);

/// 金额 - 有符号整数
/// 8位小数精度 (1e8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

    /// 转换为无符号金额 (用于余额检查)
    #[inline]
    pub fn as_unsigned(&self) -> Option<u64> {
        if self.0 >= 0 {
            Some(self.0 as u64)
        } else {
            None
        }
    }
}

/// 时间戳 (纳秒)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Timestamp(pub u64);

impl Timestamp {
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

/// 幂等键 (固定 32 字节)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdempotencyKey(pub [u8; 32]);

impl IdempotencyKey {
    /// 从充值创建
    /// 格式: "DEP:{channel}:{external_tx_id}"
    pub fn from_deposit(channel_id: ChannelId, external_tx_id: &[u8; 32]) -> Self {
        let mut key = [0u8; 32];
        key[0..4].copy_from_slice(b"DEP:");
        key[4..6].copy_from_slice(&channel_id.0.to_be_bytes());
        key[6..22].copy_from_slice(&external_tx_id[0..16]);
        Self(key)
    }

    /// 从提现创建
    /// 格式: "WDR:{account}:{seq}"
    pub fn from_withdraw(account_id: AccountId, seq: u64) -> Self {
        let mut key = [0u8; 32];
        key[0..4].copy_from_slice(b"WDR:");
        key[4..12].copy_from_slice(&account_id.0.to_be_bytes());
        key[12..20].copy_from_slice(&seq.to_be_bytes());
        Self(key)
    }

    /// 从转账创建
    /// 格式: "TRF:{from}:{to}:{seq}"
    pub fn from_transfer(from: AccountId, to: AccountId, seq: u64) -> Self {
        let mut key = [0u8; 32];
        key[0..4].copy_from_slice(b"TRF:");
        key[4..12].copy_from_slice(&from.0.to_be_bytes());
        key[12..20].copy_from_slice(&to.0.to_be_bytes());
        key[20..28].copy_from_slice(&seq.to_be_bytes());
        Self(key)
    }
}

impl std::fmt::Debug for IdempotencyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = self.0.iter().position(|&b| b == 0).unwrap_or(32);
        let s = std::str::from_utf8(&self.0[..end]).unwrap_or("<binary>");
        write!(f, "IdempotencyKey({})", s)
    }
}
```

### 3.3 Enums

```rust
// ============================================================================
// Enums (枚举类型)
// ============================================================================

/// 资金类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FundsType {
    // === 充值 (0x1X) ===
    /// 链上充值
    DepositOnChain = 0x10,
    /// 法币充值
    DepositFiat = 0x11,
    /// 内部转入
    DepositInternal = 0x12,

    // === 提现 (0x2X) ===
    /// 链上提现
    WithdrawOnChain = 0x20,
    /// 法币提现
    WithdrawFiat = 0x21,
    /// 内部转出
    WithdrawInternal = 0x22,

    // === 转账 (0x3X) ===
    /// 现货到合约
    TransferSpotToContract = 0x30,
    /// 合约到现货
    TransferContractToSpot = 0x31,
    /// 主账户到子账户
    TransferMainToSub = 0x32,
    /// 子账户到主账户
    TransferSubToMain = 0x33,

    // === 冻结 (0x4X) ===
    /// 资产冻结
    Freeze = 0x40,
    /// 资产解冻
    Unfreeze = 0x41,
    /// 扣除冻结
    DebitFrozen = 0x42,

    // === 管理 (0xFX) ===
    /// 调账
    Adjustment = 0xF0,
    /// 冲正
    Reversal = 0xF1,
}

/// 资金状态（状态机）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FundsStatus {
    /// 待处理
    Pending = 0,
    /// 处理中
    Processing = 1,
    /// 等待确认 (区块链确认中)
    Confirming = 2,
    /// 待审核
    PendingReview = 3,
    /// 已完成
    Completed = 4,
    /// 失败
    Failed = 5,
    /// 已取消
    Cancelled = 6,
    /// 已冲正
    Reversed = 7,
}

impl FundsStatus {
    /// 状态转换校验
    pub fn can_transition_to(&self, target: Self) -> bool {
        use FundsStatus::*;
        matches!(
            (self, target),
            // 正常流程
            (Pending, Processing)
                | (Pending, PendingReview)
                | (Processing, Confirming)
                | (Processing, Completed)
                | (Processing, Failed)
                | (Confirming, Completed)
                | (Confirming, Failed)
                | (PendingReview, Processing)
                | (PendingReview, Cancelled)
                // 取消
                | (Pending, Cancelled)
                // 冲正
                | (Completed, Reversed)
                // 重试
                | (Failed, Pending)
        )
    }

    /// 是否为终态
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            FundsStatus::Completed | FundsStatus::Cancelled | FundsStatus::Reversed
        )
    }
}

/// 渠道类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelType {
    /// 区块链
    Blockchain = 0,
    /// 银行转账
    BankTransfer = 1,
    /// 第三方支付
    ThirdParty = 2,
    /// 内部
    Internal = 3,
}
```

### 3.4 Aggregate Root: FundsRecord

```rust
// ============================================================================
// Aggregate Root: FundsRecord (资金记录聚合根)
// ============================================================================

/// 资金记录 - 聚合根
///
/// 代表一次完整的资金操作（充值/提现/转账/冻结）
#[repr(align(64))]
pub struct FundsRecord {
    /// 资金记录ID
    pub id: FundsId,

    /// 幂等键
    pub idempotency_key: IdempotencyKey,

    /// 交易ID
    pub transaction_id: TransactionId,

    /// 账户ID
    pub account_id: AccountId,

    /// 资产ID
    pub asset_id: AssetId,

    /// 资金类型
    pub funds_type: FundsType,

    /// 金额（正=入账，负=出账）
    pub amount: Amount,

    /// 手续费
    pub fee: Amount,

    /// 状态
    pub status: FundsStatus,

    /// 渠道ID
    pub channel_id: ChannelId,

    /// 外部交易ID
    pub external_tx_id: Option<[u8; 32]>,

    /// 目标地址/来源地址
    pub address: Option<[u8; 64]>,

    /// 网络ID
    pub network_id: Option<NetworkId>,

    /// 确认数
    pub confirmations: u32,

    /// 关联业务ID
    pub reference_id: Option<u64>,

    /// 创建时间
    pub created_at: Timestamp,

    /// 更新时间
    pub updated_at: Timestamp,

    /// 完成时间
    pub completed_at: Option<Timestamp>,

    /// 乐观锁版本
    pub version: u64,
}

impl FundsRecord {
    /// 创建充值记录
    pub fn new_deposit(
        id: FundsId,
        idempotency_key: IdempotencyKey,
        transaction_id: TransactionId,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        channel_id: ChannelId,
        external_tx_id: [u8; 32],
        network_id: Option<NetworkId>,
    ) -> Self {
        let now = Timestamp::now();
        Self {
            id,
            idempotency_key,
            transaction_id,
            account_id,
            asset_id,
            funds_type: if network_id.is_some() {
                FundsType::DepositOnChain
            } else {
                FundsType::DepositFiat
            },
            amount,
            fee: Amount::ZERO,
            status: FundsStatus::Pending,
            channel_id,
            external_tx_id: Some(external_tx_id),
            address: None,
            network_id,
            confirmations: 0,
            reference_id: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
            version: 0,
        }
    }

    /// 创建提现记录
    pub fn new_withdraw(
        id: FundsId,
        idempotency_key: IdempotencyKey,
        transaction_id: TransactionId,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        fee: Amount,
        channel_id: ChannelId,
        target_address: [u8; 64],
        network_id: Option<NetworkId>,
    ) -> Self {
        let now = Timestamp::now();
        Self {
            id,
            idempotency_key,
            transaction_id,
            account_id,
            asset_id,
            funds_type: if network_id.is_some() {
                FundsType::WithdrawOnChain
            } else {
                FundsType::WithdrawFiat
            },
            amount: Amount(-amount.0), // 出账为负
            fee,
            status: FundsStatus::Pending,
            channel_id,
            external_tx_id: None,
            address: Some(target_address),
            network_id,
            confirmations: 0,
            reference_id: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
            version: 0,
        }
    }

    /// 状态转换
    pub fn transition_to(&mut self, status: FundsStatus) -> Result<(), &'static str> {
        if self.status.can_transition_to(status) {
            self.status = status;
            self.updated_at = Timestamp::now();
            self.version += 1;
            if status.is_final() {
                self.completed_at = Some(Timestamp::now());
            }
            Ok(())
        } else {
            Err("Invalid status transition")
        }
    }

    /// 更新确认数
    pub fn update_confirmations(&mut self, confirmations: u32) {
        self.confirmations = confirmations;
        self.updated_at = Timestamp::now();
        self.version += 1;
    }

    /// 设置外部交易ID
    pub fn set_external_tx_id(&mut self, tx_id: [u8; 32]) {
        self.external_tx_id = Some(tx_id);
        self.updated_at = Timestamp::now();
        self.version += 1;
    }
}
```

### 3.5 Entity: FreezeRecord

```rust
// ============================================================================
// Entity: FreezeRecord (冻结记录)
// ============================================================================

/// 冻结记录
///
/// 跟踪资产冻结状态
#[repr(align(64))]
pub struct FreezeRecord {
    /// 冻结ID
    pub id: FundsId,

    /// 账户ID
    pub account_id: AccountId,

    /// 资产ID
    pub asset_id: AssetId,

    /// 冻结金额
    pub amount: Amount,

    /// 已解冻/扣除金额
    pub released_amount: Amount,

    /// 冻结原因
    pub reason: FreezeReason,

    /// 关联业务ID
    pub reference_id: u64,

    /// 过期时间
    pub expires_at: Option<Timestamp>,

    /// 创建时间
    pub created_at: Timestamp,

    /// 是否活跃
    pub is_active: bool,

    /// 版本号
    pub version: u64,
}

impl FreezeRecord {
    pub fn new(
        id: FundsId,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        reason: FreezeReason,
        reference_id: u64,
        expires_at: Option<Timestamp>,
    ) -> Self {
        Self {
            id,
            account_id,
            asset_id,
            amount,
            released_amount: Amount::ZERO,
            reason,
            reference_id,
            expires_at,
            created_at: Timestamp::now(),
            is_active: true,
            version: 0,
        }
    }

    /// 剩余冻结金额
    pub fn remaining_amount(&self) -> Amount {
        self.amount.saturating_sub(self.released_amount)
    }

    /// 释放金额
    pub fn release(&mut self, amount: Amount) -> Result<(), &'static str> {
        let remaining = self.remaining_amount();
        if amount.0 > remaining.0 {
            return Err("Release amount exceeds remaining");
        }
        self.released_amount = self.released_amount.saturating_add(amount);
        self.version += 1;
        if self.released_amount.0 >= self.amount.0 {
            self.is_active = false;
        }
        Ok(())
    }

    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Timestamp::now().0 > expires_at.0
        } else {
            false
        }
    }
}
```

### 3.6 Entity: Channel

```rust
// ============================================================================
// Entity: Channel (渠道配置)
// ============================================================================

/// 渠道配置
pub struct Channel {
    /// 渠道ID
    pub id: ChannelId,

    /// 渠道名称
    pub name: [u8; 32],

    /// 渠道类型
    pub channel_type: ChannelType,

    /// 支持的资产列表
    pub supported_assets: Vec<AssetId>,

    /// 最小金额
    pub min_amount: Amount,

    /// 最大金额
    pub max_amount: Amount,

    /// 手续费率 (万分比)
    pub fee_rate: u32,

    /// 固定手续费
    pub fixed_fee: Amount,

    /// 是否启用
    pub is_enabled: bool,

    /// 预计到账时间（秒）
    pub estimated_arrival_secs: u32,
}

impl Channel {
    /// 计算手续费
    pub fn calculate_fee(&self, amount: Amount) -> Amount {
        let rate_fee = (amount.0 as i128 * self.fee_rate as i128 / 10000) as i64;
        Amount(rate_fee).saturating_add(self.fixed_fee)
    }

    /// 验证金额
    pub fn validate_amount(&self, amount: Amount) -> Result<(), &'static str> {
        if amount.0 < self.min_amount.0 {
            return Err("Amount below minimum");
        }
        if amount.0 > self.max_amount.0 {
            return Err("Amount exceeds maximum");
        }
        Ok(())
    }
}
```

## 4. Account Integration

### 4.1 BalanceCommand

Funds 模块通过 BalanceCommand 与 Account 模块协同：

```rust
/// Balance Command - 余额操作指令
///
/// Funds -> Account 的通信协议
#[derive(Debug, Clone)]
pub enum BalanceCommand {
    /// 增加可用余额（充值入账）
    Credit {
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        reference_id: TransactionId,
    },

    /// 减少可用余额（提现扣款）
    Debit {
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        reference_id: TransactionId,
    },

    /// 冻结余额（可用 -> 冻结）
    Freeze {
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        reference_id: FundsId,
    },

    /// 解冻余额（冻结 -> 可用）
    Unfreeze {
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        reference_id: FundsId,
    },

    /// 扣除冻结余额
    DebitFrozen {
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        reference_id: FundsId,
    },
}

/// Balance Command Result
#[derive(Debug, Clone)]
pub struct BalanceCommandResult {
    /// 是否成功
    pub success: bool,
    /// 新可用余额
    pub available_balance: Amount,
    /// 新冻结余额
    pub frozen_balance: Amount,
    /// 错误信息
    pub error: Option<String>,
}
```

### 4.2 协同流程

#### 4.2.1 充值流程

```
[Blockchain/Bank]
       |
       v
+-------------+    Deposit Cmd    +-------------+
|   Gateway   | ----------------> |    Funds    |
+-------------+                   +-------------+
                                        |
                                        | BalanceCommand::Credit
                                        v
                                 +-------------+
                                 |   Account   |
                                 +-------------+
                                        |
                                        | Update Balance
                                        v
                                 +--------------+
                                 | Balance DB   |
                                 +--------------+
```

```rust
// 充值处理流程 (伪代码)
async fn process_deposit(cmd: DepositCommand) -> Result<CommandResult, Error> {
    // 1. 幂等性检查
    if self.idempotency_store.exists(&cmd.idempotency_key)? {
        return Ok(self.get_cached_result(&cmd.idempotency_key)?);
    }

    // 2. 创建资金记录
    let funds_record = FundsRecord::new_deposit(
        self.id_generator.next(),
        cmd.idempotency_key,
        self.tx_id_generator.next(),
        cmd.account_id,
        cmd.asset_id,
        cmd.amount,
        cmd.channel_id,
        cmd.external_tx_id,
        cmd.network_id,
    );

    // 3. 保存资金记录 (Pending 状态)
    self.funds_repo.save(&funds_record)?;

    // 4. 发送 BalanceCommand 到 Account
    let balance_cmd = BalanceCommand::Credit {
        account_id: cmd.account_id,
        asset_id: cmd.asset_id,
        amount: cmd.amount,
        reference_id: funds_record.transaction_id,
    };

    let balance_result = self.account_gateway.execute(balance_cmd).await?;

    // 5. 更新资金记录状态
    if balance_result.success {
        funds_record.transition_to(FundsStatus::Completed)?;
    } else {
        funds_record.transition_to(FundsStatus::Failed)?;
    }
    self.funds_repo.save(&funds_record)?;

    // 6. 发布事件
    self.event_publisher.publish(DepositCompletedEvent {
        transaction_id: funds_record.transaction_id,
        account_id: cmd.account_id,
        asset_id: cmd.asset_id,
        amount: cmd.amount,
    }).await?;

    Ok(CommandResult::Deposit {
        transaction_id: funds_record.transaction_id,
        funds_id: funds_record.id,
        new_balance: balance_result.available_balance,
        status: funds_record.status,
        completed_at: funds_record.completed_at.unwrap(),
    })
}
```

#### 4.2.2 提现流程

```
[User Request]
       |
       v
+-------------+   Withdraw Cmd    +-------------+
|     API     | ----------------> |    Funds    |
+-------------+                   +-------------+
                                        |
                              +---------+---------+
                              |                   |
                              v                   v
                    Freeze Balance       Risk Check
                              |                   |
                              +--------+----------+
                                       |
                                       v (if approved)
                              +-------------------+
                              |    Gateway        |
                              +-------------------+
                                       |
                                       v
                              +-------------------+
                              | Blockchain/Bank   |
                              +-------------------+
                                       |
                                       v (callback)
                              +-------------------+
                              | DebitFrozen       |
                              +-------------------+
```

```rust
// 提现处理流程 (伪代码)
async fn process_withdraw(cmd: WithdrawCommand) -> Result<CommandResult, Error> {
    // 1. 幂等性检查
    if self.idempotency_store.exists(&cmd.idempotency_key)? {
        return Ok(self.get_cached_result(&cmd.idempotency_key)?);
    }

    // 2. 风控检查
    self.risk_checker.check_withdraw(&cmd)?;

    // 3. 创建资金记录
    let funds_record = FundsRecord::new_withdraw(
        self.id_generator.next(),
        cmd.idempotency_key,
        self.tx_id_generator.next(),
        cmd.account_id,
        cmd.asset_id,
        cmd.amount,
        cmd.fee,
        cmd.channel_id,
        cmd.target_address,
        cmd.network_id,
    );

    // 4. 冻结余额
    let freeze_cmd = BalanceCommand::Freeze {
        account_id: cmd.account_id,
        asset_id: cmd.asset_id,
        amount: cmd.amount.saturating_add(cmd.fee),
        reference_id: funds_record.id,
    };

    let freeze_result = self.account_gateway.execute(freeze_cmd).await?;
    if !freeze_result.success {
        return Err(Error::InsufficientBalance);
    }

    // 5. 保存资金记录 (Processing 状态)
    funds_record.transition_to(FundsStatus::Processing)?;
    self.funds_repo.save(&funds_record)?;

    // 6. 提交到渠道
    let channel_result = self.channel_gateway.submit_withdraw(&funds_record).await;

    match channel_result {
        Ok(external_tx_id) => {
            funds_record.set_external_tx_id(external_tx_id);
            funds_record.transition_to(FundsStatus::Confirming)?;
        }
        Err(e) => {
            // 提交失败，解冻
            let unfreeze_cmd = BalanceCommand::Unfreeze {
                account_id: cmd.account_id,
                asset_id: cmd.asset_id,
                amount: cmd.amount.saturating_add(cmd.fee),
                reference_id: funds_record.id,
            };
            self.account_gateway.execute(unfreeze_cmd).await?;
            funds_record.transition_to(FundsStatus::Failed)?;
        }
    }

    self.funds_repo.save(&funds_record)?;

    Ok(CommandResult::Withdraw {
        transaction_id: funds_record.transaction_id,
        funds_id: funds_record.id,
        deducted_amount: cmd.amount.saturating_add(cmd.fee),
        new_balance: freeze_result.available_balance,
        status: funds_record.status,
        estimated_arrival: None,
    })
}

// 提现回调处理
async fn handle_withdraw_callback(
    tx_id: TransactionId,
    success: bool,
    external_tx_id: Option<[u8; 32]>,
) -> Result<(), Error> {
    let mut funds_record = self.funds_repo.find_by_tx_id(tx_id)?;

    if success {
        // 扣除冻结余额
        let debit_cmd = BalanceCommand::DebitFrozen {
            account_id: funds_record.account_id,
            asset_id: funds_record.asset_id,
            amount: funds_record.amount.abs().saturating_add(funds_record.fee),
            reference_id: funds_record.id,
        };
        self.account_gateway.execute(debit_cmd).await?;

        if let Some(tx_id) = external_tx_id {
            funds_record.set_external_tx_id(tx_id);
        }
        funds_record.transition_to(FundsStatus::Completed)?;
    } else {
        // 解冻
        let unfreeze_cmd = BalanceCommand::Unfreeze {
            account_id: funds_record.account_id,
            asset_id: funds_record.asset_id,
            amount: funds_record.amount.abs().saturating_add(funds_record.fee),
            reference_id: funds_record.id,
        };
        self.account_gateway.execute(unfreeze_cmd).await?;
        funds_record.transition_to(FundsStatus::Failed)?;
    }

    self.funds_repo.save(&funds_record)?;

    // 发布事件
    self.event_publisher.publish(WithdrawCompletedEvent {
        transaction_id: funds_record.transaction_id,
        account_id: funds_record.account_id,
        success,
    }).await?;

    Ok(())
}
```

#### 4.2.3 内部转账流程

```rust
// 内部转账处理 (伪代码)
async fn process_internal_transfer(cmd: InternalTransferCommand) -> Result<CommandResult, Error> {
    // 1. 幂等性检查
    if self.idempotency_store.exists(&cmd.idempotency_key)? {
        return Ok(self.get_cached_result(&cmd.idempotency_key)?);
    }

    // 2. 验证账户
    self.validate_transfer_accounts(&cmd)?;

    // 3. 创建资金记录
    let funds_record = FundsRecord::new_transfer(
        self.id_generator.next(),
        cmd.idempotency_key,
        self.tx_id_generator.next(),
        cmd.from_account,
        cmd.to_account,
        cmd.asset_id,
        cmd.amount,
        cmd.transfer_type,
    );

    // 4. 原子执行：扣款 + 入账
    // 使用事务保证原子性
    let tx = self.db.begin_transaction().await?;

    // 扣款
    let debit_cmd = BalanceCommand::Debit {
        account_id: cmd.from_account,
        asset_id: cmd.asset_id,
        amount: cmd.amount,
        reference_id: funds_record.transaction_id,
    };
    let debit_result = self.account_gateway.execute_in_tx(&tx, debit_cmd).await?;

    if !debit_result.success {
        tx.rollback().await?;
        return Err(Error::InsufficientBalance);
    }

    // 入账
    let credit_cmd = BalanceCommand::Credit {
        account_id: cmd.to_account,
        asset_id: cmd.asset_id,
        amount: cmd.amount,
        reference_id: funds_record.transaction_id,
    };
    let credit_result = self.account_gateway.execute_in_tx(&tx, credit_cmd).await?;

    if !credit_result.success {
        tx.rollback().await?;
        return Err(Error::TransferFailed);
    }

    // 5. 保存并提交
    funds_record.transition_to(FundsStatus::Completed)?;
    self.funds_repo.save_in_tx(&tx, &funds_record)?;
    tx.commit().await?;

    Ok(CommandResult::InternalTransfer {
        transaction_id: funds_record.transaction_id,
        funds_id: funds_record.id,
        from_new_balance: debit_result.available_balance,
        to_new_balance: credit_result.available_balance,
        status: FundsStatus::Completed,
    })
}
```

## 5. Repository Interfaces

```rust
/// 资金记录仓储接口
#[async_trait]
pub trait FundsRepository: Send + Sync {
    /// 保存资金记录
    async fn save(&self, record: &FundsRecord) -> Result<(), RepositoryError>;

    /// 根据ID查询
    async fn find_by_id(&self, id: FundsId) -> Result<Option<FundsRecord>, RepositoryError>;

    /// 根据交易ID查询
    async fn find_by_tx_id(&self, tx_id: TransactionId) -> Result<Option<FundsRecord>, RepositoryError>;

    /// 根据幂等键查询
    async fn find_by_idempotency_key(&self, key: &IdempotencyKey) -> Result<Option<FundsRecord>, RepositoryError>;

    /// 查询账户资金记录
    async fn find_by_account(
        &self,
        account_id: AccountId,
        funds_type: Option<FundsType>,
        status: Option<FundsStatus>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<FundsRecord>, RepositoryError>;

    /// 查询待处理提现
    async fn find_pending_withdrawals(&self, limit: usize) -> Result<Vec<FundsRecord>, RepositoryError>;
}

/// 冻结记录仓储接口
#[async_trait]
pub trait FreezeRepository: Send + Sync {
    /// 保存冻结记录
    async fn save(&self, record: &FreezeRecord) -> Result<(), RepositoryError>;

    /// 根据ID查询
    async fn find_by_id(&self, id: FundsId) -> Result<Option<FreezeRecord>, RepositoryError>;

    /// 查询账户活跃冻结
    async fn find_active_by_account(
        &self,
        account_id: AccountId,
        asset_id: AssetId,
    ) -> Result<Vec<FreezeRecord>, RepositoryError>;

    /// 查询过期冻结
    async fn find_expired(&self, limit: usize) -> Result<Vec<FreezeRecord>, RepositoryError>;
}

/// Account 网关接口
#[async_trait]
pub trait AccountGateway: Send + Sync {
    /// 执行余额命令
    async fn execute(&self, cmd: BalanceCommand) -> Result<BalanceCommandResult, GatewayError>;

    /// 查询余额
    async fn get_balance(&self, account_id: AccountId, asset_id: AssetId) -> Result<(Amount, Amount), GatewayError>;
}

/// 渠道网关接口
#[async_trait]
pub trait ChannelGateway: Send + Sync {
    /// 提交提现
    async fn submit_withdraw(&self, record: &FundsRecord) -> Result<[u8; 32], GatewayError>;

    /// 查询交易状态
    async fn query_status(&self, external_tx_id: &[u8; 32]) -> Result<ChannelTxStatus, GatewayError>;
}
```

## 6. Command Handler

```rust
/// Funds Command Handler Trait
#[async_trait]
pub trait FundsCommandHandler: Send + Sync {
    /// 处理单个命令
    async fn handle(&self, command: Command) -> CommandResult;

    /// 处理批量命令
    async fn handle_batch(&self, commands: Vec<Command>) -> Vec<CommandResult>;

    /// Handler 名称
    fn handler_name(&self) -> &'static str;
}

/// Funds Command Handler 实现
pub struct FundsCommandHandlerImpl {
    funds_repo: Arc<dyn FundsRepository>,
    freeze_repo: Arc<dyn FreezeRepository>,
    account_gateway: Arc<dyn AccountGateway>,
    channel_gateway: Arc<dyn ChannelGateway>,
    id_generator: Arc<dyn IdGenerator>,
    risk_checker: Arc<dyn RiskChecker>,
    event_publisher: Arc<dyn EventPublisher>,
}

#[async_trait]
impl FundsCommandHandler for FundsCommandHandlerImpl {
    async fn handle(&self, command: Command) -> CommandResult {
        match command {
            Command::Deposit { .. } => self.handle_deposit(command).await,
            Command::Withdraw { .. } => self.handle_withdraw(command).await,
            Command::InternalTransfer { .. } => self.handle_transfer(command).await,
            Command::FreezeAsset { .. } => self.handle_freeze(command).await,
            Command::UnfreezeAsset { .. } => self.handle_unfreeze(command).await,
            Command::DebitFrozen { .. } => self.handle_debit_frozen(command).await,
            Command::AdjustBalance { .. } => self.handle_adjust(command).await,
            Command::ReverseTransaction { .. } => self.handle_reverse(command).await,
            Command::BatchDeposit { .. } => self.handle_batch_deposit(command).await,
            Command::BatchWithdraw { .. } => self.handle_batch_withdraw(command).await,
            Command::ConfirmWithdraw { .. } => self.handle_confirm_withdraw(command).await,
            Command::RejectWithdraw { .. } => self.handle_reject_withdraw(command).await,
        }
    }

    async fn handle_batch(&self, commands: Vec<Command>) -> Vec<CommandResult> {
        let mut results = Vec::with_capacity(commands.len());
        for cmd in commands {
            results.push(self.handle(cmd).await);
        }
        results
    }

    fn handler_name(&self) -> &'static str {
        "FundsCommandHandler"
    }
}
```

## 7. Events

```rust
/// Funds Domain Events
#[derive(Debug, Clone)]
pub enum FundsEvent {
    /// 充值完成
    DepositCompleted {
        transaction_id: TransactionId,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        external_tx_id: [u8; 32],
        completed_at: Timestamp,
    },

    /// 提现请求创建
    WithdrawRequested {
        transaction_id: TransactionId,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        fee: Amount,
        target_address: [u8; 64],
    },

    /// 提现完成
    WithdrawCompleted {
        transaction_id: TransactionId,
        account_id: AccountId,
        success: bool,
        external_tx_id: Option<[u8; 32]>,
    },

    /// 内部转账完成
    TransferCompleted {
        transaction_id: TransactionId,
        from_account: AccountId,
        to_account: AccountId,
        asset_id: AssetId,
        amount: Amount,
    },

    /// 资产冻结
    AssetFrozen {
        freeze_id: FundsId,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
        reason: FreezeReason,
    },

    /// 资产解冻
    AssetUnfrozen {
        freeze_id: FundsId,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Amount,
    },

    /// 交易冲正
    TransactionReversed {
        reverse_tx_id: TransactionId,
        original_tx_id: TransactionId,
        amount: Amount,
    },
}
```

## 8. Directory Structure

```
lib/core/funds/
├── Cargo.toml
├── design.md
├── readme.md
└── src/
    ├── lib.rs
    ├── domain/
    │   ├── mod.rs
    │   ├── entity/
    │   │   ├── mod.rs              # FundsRecord, FreezeRecord, Channel
    │   │   └── types.rs            # Value Objects
    │   ├── service/
    │   │   ├── mod.rs
    │   │   └── command.rs          # Command & CommandResult
    │   └── repository.rs           # Repository Traits
    ├── adaptor/
    │   ├── mod.rs
    │   ├── inbound/
    │   │   └── mod.rs              # Command Handler
    │   └── outbound/
    │       ├── mod.rs
    │       ├── account_gateway.rs  # Account Gateway Impl
    │       └── channel_gateway.rs  # Channel Gateway Impl
    └── event.rs                    # Domain Events
```

## 9. Performance Considerations

### 9.1 High-Performance Design

1. **缓存行对齐**: 所有核心实体 64 字节对齐
2. **零分配 ID**: 使用 newtype u64/u32 替代 String
3. **固定长度键**: IdempotencyKey 32 字节固定长度
4. **批量处理**: 支持 BatchDeposit/BatchWithdraw

### 9.2 Latency Targets

| Operation | Target Latency |
|-----------|---------------|
| Deposit (内部处理) | < 1ms |
| Withdraw (冻结) | < 1ms |
| Internal Transfer | < 2ms |
| Freeze/Unfreeze | < 500μs |

### 9.3 Idempotency

- 所有命令携带 `IdempotencyKey`
- 使用 Redis/内存存储幂等键
- 幂等键 TTL: 24h

## 10. Security Considerations

### 10.1 风控检查

- 提现前进行风控检查
- 大额提现需人工审核
- 地址黑白名单校验
- 异常行为检测

### 10.2 审计追踪

- 所有操作记录完整流水
- 支持交易冲正
- 事件溯源支持

---

**Document Version**: v1.0.0
**Last Updated**: 2024-12-01
**Next Review**: 2025-02-01
