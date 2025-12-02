# Account Module Design Document

## 1. Overview

Account 模块负责账户和余额管理，通过统一的 `AccountCommand` 处理来自 LOB（下单/撤单）和 Settlement（结算）的所有余额操作。

### 1.1 核心职责

| 职责 | 说明 |
|------|------|
| **余额管理** | 管理用户各资产的可用/冻结余额 |
| **余额检查** | 为下单提供余额充足性验证 |
| **统一命令处理** | 处理来自 LOB 和 Settlement 的 AccountCommand |
| **乐观锁** | 使用 version 字段保证并发安全 |

### 1.2 设计原则

- **统一资产模型（方案B）**：所有资产（现金、股票、加密货币）使用同一 Balance 结构
- **Clean Architecture**：领域层不依赖外部框架
- **高性能**：使用 u32/u64 替代 String，缓存行对齐

---

## 2. 实体关系模型

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

### 2.1 关系说明

| 关系 | 基数 | 说明 |
|------|------|------|
| User → Account | 1:N | 一个用户可拥有多个不同类型的账户 |
| Account → Balance | 1:N | 一个账户下有多个资产余额 |
| Balance 主键 | - | `(account_id, asset_id)` 复合主键 |

### 2.2 示例数据

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

---

## 3. Entity 定义

### 3.1 基础类型

```rust
/// 时间戳（纳秒）
pub type Timestamp = u64;

/// 用户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UserId(pub u64);

/// 账户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AccountId(pub u64);

/// 资产ID（使用 u32 高性能）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AssetId(pub u32);

impl AssetId {
    pub const USDT: AssetId = AssetId(1);
    pub const BTC: AssetId = AssetId(2);
    pub const ETH: AssetId = AssetId(3);
}
```

### 3.2 Account (账户实体)

```rust
/// 交易账户
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

/// 账户类型
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
#[repr(u8)]
pub enum AccountStatus {
    /// 正常
    Active = 0,
    /// 冻结（禁止交易）
    Frozen = 1,
    /// 注销
    Closed = 2,
}
```

### 3.3 Balance (余额实体) - 统一资产模型

```rust
/// 资产余额（统一资产模型）
///
/// 示例：
/// - Balance(account, USDT, 100_000_000) = 100 USDT
/// - Balance(account, BTC, 100_000_000)  = 1 BTC
/// - Balance(account, AAPL, 1000)        = 1000 股苹果
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
```

### 3.4 Balance 方法

```rust
impl Balance {
    /// 总余额 = 可用 + 冻结
    pub fn total(&self) -> u64;

    /// 检查是否有足够的可用余额
    pub fn has_available(&self, amount: u64) -> bool;

    /// 冻结指定金额（可用 → 冻结）
    pub fn freeze(&mut self, amount: u64) -> Result<(), BalanceError>;

    /// 解冻指定金额（冻结 → 可用）
    pub fn unfreeze(&mut self, amount: u64) -> Result<(), BalanceError>;

    /// 增加可用余额（入金、收款）
    pub fn credit(&mut self, amount: u64) -> Result<(), BalanceError>;

    /// 减少可用余额（出金、付款）
    pub fn debit(&mut self, amount: u64) -> Result<(), BalanceError>;

    /// 直接扣减冻结余额（强平、成交扣款）
    pub fn debit_frozen(&mut self, amount: u64) -> Result<(), BalanceError>;
}
```

### 3.5 BalanceError

```rust
/// 余额错误
pub enum BalanceError {
    /// 可用余额不足
    InsufficientAvailable { required: u64, available: u64 },
    /// 冻结余额不足
    InsufficientFrozen { required: u64, frozen: u64 },
    /// 余额溢出（price * quantity 超出 u64）
    Overflow,
    /// 账户不存在
    AccountNotFound { account_id: AccountId },
    /// 余额记录不存在
    BalanceNotFound { account_id: AccountId, asset_id: AssetId },
    /// 账户已冻结（禁止交易）
    AccountFrozen { account_id: AccountId },
    /// 账户已注销
    AccountClosed { account_id: AccountId },
    /// 版本冲突（乐观锁）
    VersionConflict { expected: u64, actual: u64 },
}
```

---

## 4. Service 定义

### 4.1 TradingPair (交易对)

```rust
/// 交易对
///
/// 定义基础资产和计价资产的关系
/// 例如：BTC/USDT 交易对
/// - base_asset = BTC (基础资产，卖出时检查)
/// - quote_asset = USDT (计价资产，买入时检查)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TradingPair {
    /// 基础资产（如 BTC）
    pub base_asset: AssetId,
    /// 计价资产（如 USDT）
    pub quote_asset: AssetId,
}

impl TradingPair {
    pub fn new(base_asset: AssetId, quote_asset: AssetId) -> Self {
        Self { base_asset, quote_asset }
    }

    /// BTC/USDT 交易对
    pub const BTC_USDT: TradingPair = TradingPair {
        base_asset: AssetId::BTC,
        quote_asset: AssetId::USDT,
    };

    /// ETH/USDT 交易对
    pub const ETH_USDT: TradingPair = TradingPair {
        base_asset: AssetId::ETH,
        quote_asset: AssetId::USDT,
    };
}
```

### 4.2 Side (买卖方向)

```rust
/// 买卖方向
///
/// 设计选择：定义在 Account 模块，供 LOB 模块引用
///
/// 替代方案：
/// - 方案A（当前）：Account 模块定义 Side，LOB 模块依赖 Account
/// - 方案B：创建共享 `common` 模块定义 Side，Account 和 LOB 都依赖 common
///
/// 如果选择方案B，需要创建 lib/core/common 模块：
/// ```
/// lib/core/common/src/types.rs:
///   pub enum Side { Buy, Sell }
///   pub type OrderId = u64;
///   pub type Price = u64;
///   pub type Quantity = u64;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    Buy = 0,
    Sell = 1,
}

impl Side {
    /// 获取相反方向
    #[inline]
    pub fn opposite(&self) -> Side {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}
```

### 4.3 AccountCommand (统一账户命令)

```rust
/// 账户命令（统一）
///
/// 合并 LOB 和 Settlement 的所有账户操作：
/// - LOB 调用：CheckAndFreeze, Unfreeze（基于交易对）
/// - Settlement 调用：Credit, Debit, DebitFrozen, Transfer 等（直接操作资产）
#[derive(Debug, Clone)]
pub enum AccountCommand {
    // ==================== LOB 调用（基于交易对） ====================

    /// 检查并冻结（原子操作，防止 TOCTOU 竞态）
    /// 下单时使用，一步完成检查+冻结
    CheckAndFreeze {
        account_id: AccountId,
        order_id: OrderId,
        pair: TradingPair,
        side: Side,
        price: u64,
        quantity: u64,
    },

    /// 解冻资金（撤单时释放）
    Unfreeze {
        account_id: AccountId,
        order_id: OrderId,
        pair: TradingPair,
        side: Side,
        price: u64,
        quantity: u64,
    },

    // ==================== Settlement 调用（直接操作资产） ====================

    /// 冻结指定资产（可用 → 冻结）
    Freeze {
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        reference_id: u64,  // 关联的交易/结算ID
    },

    /// 解冻指定资产（冻结 → 可用）
    UnfreezeAsset {
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        reference_id: u64,
    },

    /// 增加可用余额（入金、收款、成交收入）
    Credit {
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        reference_id: u64,
    },

    /// 扣减可用余额（出金、付款）
    Debit {
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        reference_id: u64,
    },

    /// 扣减冻结余额（成交扣款、强平）
    DebitFrozen {
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        reference_id: u64,
    },

    /// 转账（同用户不同账户间）
    Transfer {
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        reference_id: u64,
    },

    /// 结算盈亏（可正可负）
    SettlePnl {
        account_id: AccountId,
        asset_id: AssetId,
        pnl: i64,  // 正=盈利，负=亏损
        reference_id: u64,
    },

    // ==================== 查询 ====================

    /// 查询余额
    GetBalance {
        account_id: AccountId,
        asset_id: AssetId,
    },
}

/// 订单ID
pub type OrderId = u64;

impl AccountCommand {
    /// 获取账户ID
    pub fn account_id(&self) -> AccountId {
        match self {
            AccountCommand::CheckAndFreeze { account_id, .. }
            | AccountCommand::Unfreeze { account_id, .. }
            | AccountCommand::Freeze { account_id, .. }
            | AccountCommand::UnfreezeAsset { account_id, .. }
            | AccountCommand::Credit { account_id, .. }
            | AccountCommand::Debit { account_id, .. }
            | AccountCommand::DebitFrozen { account_id, .. }
            | AccountCommand::SettlePnl { account_id, .. }
            | AccountCommand::GetBalance { account_id, .. } => *account_id,
            AccountCommand::Transfer { from_account_id, .. } => *from_account_id,
        }
    }

    /// 获取资产ID（如果适用）
    pub fn asset_id(&self) -> Option<AssetId> {
        match self {
            AccountCommand::Freeze { asset_id, .. }
            | AccountCommand::UnfreezeAsset { asset_id, .. }
            | AccountCommand::Credit { asset_id, .. }
            | AccountCommand::Debit { asset_id, .. }
            | AccountCommand::DebitFrozen { asset_id, .. }
            | AccountCommand::Transfer { asset_id, .. }
            | AccountCommand::SettlePnl { asset_id, .. }
            | AccountCommand::GetBalance { asset_id, .. } => Some(*asset_id),
            AccountCommand::CheckAndFreeze { .. }
            | AccountCommand::Unfreeze { .. } => None,  // 由交易对决定
        }
    }

    /// 获取关联ID（订单ID或结算ID）
    pub fn reference_id(&self) -> Option<u64> {
        match self {
            AccountCommand::CheckAndFreeze { order_id, .. }
            | AccountCommand::Unfreeze { order_id, .. } => Some(*order_id),
            AccountCommand::Freeze { reference_id, .. }
            | AccountCommand::UnfreezeAsset { reference_id, .. }
            | AccountCommand::Credit { reference_id, .. }
            | AccountCommand::Debit { reference_id, .. }
            | AccountCommand::DebitFrozen { reference_id, .. }
            | AccountCommand::Transfer { reference_id, .. }
            | AccountCommand::SettlePnl { reference_id, .. } => Some(*reference_id),
            AccountCommand::GetBalance { .. } => None,
        }
    }

    /// 计算交易对命令的冻结资产和金额
    /// 仅适用于 CheckAndFreeze 和 Unfreeze
    #[inline]
    pub fn trading_pair_amount(&self) -> Option<(AssetId, u64)> {
        match self {
            AccountCommand::CheckAndFreeze { pair, side, price, quantity, .. }
            | AccountCommand::Unfreeze { pair, side, price, quantity, .. } => {
                match side {
                    Side::Buy => {
                        let amount = price.checked_mul(*quantity)?;
                        Some((pair.quote_asset, amount))
                    }
                    Side::Sell => Some((pair.base_asset, *quantity)),
                }
            }
            _ => None,
        }
    }
}
```

### 4.4 AccountCommandResult (命令结果)

```rust
/// 账户命令执行结果
#[derive(Debug, Clone)]
pub enum AccountCommandResult {
    /// 冻结成功（CheckAndFreeze, Freeze）
    Frozen {
        reference_id: u64,
        asset_id: AssetId,
        amount: u64,
        new_available: u64,
        new_frozen: u64,
    },

    /// 解冻成功（Unfreeze, UnfreezeAsset）
    Unfrozen {
        reference_id: u64,
        asset_id: AssetId,
        amount: u64,
        new_available: u64,
        new_frozen: u64,
    },

    /// 入账成功（Credit）
    Credited {
        reference_id: u64,
        asset_id: AssetId,
        amount: u64,
        new_available: u64,
    },

    /// 扣款成功（Debit, DebitFrozen）
    Debited {
        reference_id: u64,
        asset_id: AssetId,
        amount: u64,
        from_frozen: bool,  // true=扣冻结，false=扣可用
        new_available: u64,
        new_frozen: u64,
    },

    /// 转账成功（Transfer）
    Transferred {
        reference_id: u64,
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    },

    /// 盈亏结算成功（SettlePnl）
    PnlSettled {
        reference_id: u64,
        asset_id: AssetId,
        pnl: i64,
        new_available: u64,
    },

    /// 余额查询结果
    Balance(Option<Balance>),

    /// 错误
    Error(BalanceError),
}
```

### 4.5 AccountService (账户服务接口)

```rust
/// 账户服务接口
///
/// 统一处理来自 LOB 和 Settlement 的所有账户操作：
/// - LOB: CheckAndFreeze, Unfreeze（下单/撤单）
/// - Settlement: Credit, Debit, DebitFrozen, Transfer, SettlePnl（结算）
pub trait AccountService: Send + Sync {
    /// 执行账户命令
    fn execute(&mut self, cmd: AccountCommand) -> AccountCommandResult;

    /// 批量执行（原子操作，全部成功或全部回滚）
    fn execute_batch(&mut self, cmds: Vec<AccountCommand>) -> Result<Vec<AccountCommandResult>, BalanceError>;
}
```

### 4.6 AccountServiceImpl 结构体

```rust
use std::collections::HashMap;

/// 账户服务实现
pub struct AccountServiceImpl<T: BalanceStore> {
    /// 余额存储
    balance_store: T,
    /// 账户缓存 (account_id -> Account)
    accounts: HashMap<AccountId, Account>,
    /// 余额缓存 ((account_id, asset_id) -> Balance)
    balances: HashMap<(AccountId, AssetId), Balance>,
    /// 时间戳生成器
    timestamp_fn: fn() -> Timestamp,
}

impl<T: BalanceStore> AccountServiceImpl<T> {
    pub fn new(balance_store: T, timestamp_fn: fn() -> Timestamp) -> Self {
        Self {
            balance_store,
            accounts: HashMap::new(),
            balances: HashMap::new(),
            timestamp_fn,
        }
    }

    /// 获取当前时间戳
    #[inline]
    fn now(&self) -> Timestamp {
        (self.timestamp_fn)()
    }
}
```

### 4.7 AccountService 实现逻辑

```rust
impl<T: BalanceStore> AccountService for AccountServiceImpl<T> {
    fn execute(&mut self, cmd: AccountCommand) -> AccountCommandResult {
        // 1. 检查账户状态（查询操作跳过检查）
        if !matches!(cmd, AccountCommand::GetBalance { .. }) {
            let account_id = cmd.account_id();
            if let Err(e) = self.check_account_status(account_id) {
                return AccountCommandResult::Error(e);
            }
        }

        match cmd {
            // ==================== LOB 调用（基于交易对） ====================

            AccountCommand::CheckAndFreeze { account_id, order_id, pair, side, price, quantity } => {
                // 计算冻结金额（带溢出检查）
                let (asset_id, amount) = match side {
                    Side::Buy => {
                        match price.checked_mul(quantity) {
                            Some(amt) => (pair.quote_asset, amt),
                            None => return AccountCommandResult::Error(BalanceError::Overflow),
                        }
                    }
                    Side::Sell => (pair.base_asset, quantity),
                };

                // 原子检查并冻结
                match self.check_and_freeze_balance(account_id, asset_id, amount) {
                    Ok(balance) => AccountCommandResult::Frozen {
                        reference_id: order_id,
                        asset_id,
                        amount,
                        new_available: balance.available,
                        new_frozen: balance.frozen,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            AccountCommand::Unfreeze { account_id, order_id, pair, side, price, quantity } => {
                let (asset_id, amount) = match side {
                    Side::Buy => {
                        match price.checked_mul(quantity) {
                            Some(amt) => (pair.quote_asset, amt),
                            None => return AccountCommandResult::Error(BalanceError::Overflow),
                        }
                    }
                    Side::Sell => (pair.base_asset, quantity),
                };

                match self.unfreeze_balance(account_id, asset_id, amount) {
                    Ok(balance) => AccountCommandResult::Unfrozen {
                        reference_id: order_id,
                        asset_id,
                        amount,
                        new_available: balance.available,
                        new_frozen: balance.frozen,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            // ==================== Settlement 调用（直接操作资产） ====================

            AccountCommand::Freeze { account_id, asset_id, amount, reference_id } => {
                match self.freeze_balance(account_id, asset_id, amount) {
                    Ok(balance) => AccountCommandResult::Frozen {
                        reference_id,
                        asset_id,
                        amount,
                        new_available: balance.available,
                        new_frozen: balance.frozen,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            AccountCommand::UnfreezeAsset { account_id, asset_id, amount, reference_id } => {
                match self.unfreeze_balance(account_id, asset_id, amount) {
                    Ok(balance) => AccountCommandResult::Unfrozen {
                        reference_id,
                        asset_id,
                        amount,
                        new_available: balance.available,
                        new_frozen: balance.frozen,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            AccountCommand::Credit { account_id, asset_id, amount, reference_id } => {
                match self.credit_balance(account_id, asset_id, amount) {
                    Ok(balance) => AccountCommandResult::Credited {
                        reference_id,
                        asset_id,
                        amount,
                        new_available: balance.available,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            AccountCommand::Debit { account_id, asset_id, amount, reference_id } => {
                match self.debit_balance(account_id, asset_id, amount) {
                    Ok(balance) => AccountCommandResult::Debited {
                        reference_id,
                        asset_id,
                        amount,
                        from_frozen: false,
                        new_available: balance.available,
                        new_frozen: balance.frozen,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            AccountCommand::DebitFrozen { account_id, asset_id, amount, reference_id } => {
                match self.debit_frozen_balance(account_id, asset_id, amount) {
                    Ok(balance) => AccountCommandResult::Debited {
                        reference_id,
                        asset_id,
                        amount,
                        from_frozen: true,
                        new_available: balance.available,
                        new_frozen: balance.frozen,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            AccountCommand::Transfer { from_account_id, to_account_id, asset_id, amount, reference_id } => {
                // 转账需要检查目标账户状态
                if let Err(e) = self.check_account_status(to_account_id) {
                    return AccountCommandResult::Error(e);
                }

                match self.transfer_balance(from_account_id, to_account_id, asset_id, amount) {
                    Ok(_) => AccountCommandResult::Transferred {
                        reference_id,
                        from_account_id,
                        to_account_id,
                        asset_id,
                        amount,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            AccountCommand::SettlePnl { account_id, asset_id, pnl, reference_id } => {
                match self.settle_pnl(account_id, asset_id, pnl) {
                    Ok(balance) => AccountCommandResult::PnlSettled {
                        reference_id,
                        asset_id,
                        pnl,
                        new_available: balance.available,
                    },
                    Err(e) => AccountCommandResult::Error(e),
                }
            }

            // ==================== 查询 ====================

            AccountCommand::GetBalance { account_id, asset_id } => {
                AccountCommandResult::Balance(
                    self.get_balance(account_id, asset_id).cloned()
                )
            }
        }
    }

    fn execute_batch(&mut self, cmds: Vec<AccountCommand>) -> Result<Vec<AccountCommandResult>, BalanceError> {
        // 1. 预检查：验证所有命令是否可执行
        for cmd in &cmds {
            self.validate_command(cmd)?;
        }

        // 2. 执行所有命令（预检查通过后不会失败）
        let results = cmds.into_iter()
            .map(|cmd| self.execute(cmd))
            .collect();

        Ok(results)
    }
}

impl<T: BalanceStore> AccountServiceImpl<T> {
    /// 检查账户状态
    fn check_account_status(&self, account_id: AccountId) -> Result<(), BalanceError> {
        match self.accounts.get(&account_id) {
            Some(account) => match account.status {
                AccountStatus::Active => Ok(()),
                AccountStatus::Frozen => Err(BalanceError::AccountFrozen { account_id }),
                AccountStatus::Closed => Err(BalanceError::AccountClosed { account_id }),
            },
            None => Err(BalanceError::AccountNotFound { account_id }),
        }
    }

    /// 预验证命令（用于批量执行的预检查）
    fn validate_command(&self, cmd: &AccountCommand) -> Result<(), BalanceError> {
        let account_id = cmd.account_id();
        self.check_account_status(account_id)?;

        match cmd {
            AccountCommand::Transfer { to_account_id, asset_id, amount, .. } => {
                // 检查目标账户
                self.check_account_status(*to_account_id)?;
                // 检查源账户余额
                let balance = self.get_balance(account_id, *asset_id)
                    .ok_or(BalanceError::BalanceNotFound { account_id, asset_id: *asset_id })?;
                if balance.available < *amount {
                    return Err(BalanceError::InsufficientAvailable {
                        required: *amount,
                        available: balance.available,
                    });
                }
                // 检查目标账户是否会溢出
                if let Some(to_balance) = self.get_balance(*to_account_id, *asset_id) {
                    to_balance.available.checked_add(*amount)
                        .ok_or(BalanceError::Overflow)?;
                }
                Ok(())
            }
            AccountCommand::Credit { account_id, asset_id, amount, .. } => {
                // 检查是否会溢出
                if let Some(balance) = self.get_balance(*account_id, *asset_id) {
                    balance.available.checked_add(*amount)
                        .ok_or(BalanceError::Overflow)?;
                }
                Ok(())
            }
            // 其他命令的预检查...
            _ => Ok(()),
        }
    }

    /// 获取余额引用
    fn get_balance(&self, account_id: AccountId, asset_id: AssetId) -> Option<&Balance> {
        self.balances.get(&(account_id, asset_id))
    }

    /// 获取余额可变引用
    fn get_balance_mut(&mut self, account_id: AccountId, asset_id: AssetId) -> Option<&mut Balance> {
        self.balances.get_mut(&(account_id, asset_id))
    }

    /// 获取或创建余额（用于 Credit 等可能创建新余额的操作）
    fn get_or_create_balance_mut(&mut self, account_id: AccountId, asset_id: AssetId) -> &mut Balance {
        let now = self.now();
        self.balances.entry((account_id, asset_id)).or_insert_with(|| Balance {
            account_id,
            asset_id,
            available: 0,
            frozen: 0,
            version: 0,
            updated_at: now,
        })
    }

    /// 原子检查并冻结（防止 TOCTOU）
    fn check_and_freeze_balance(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<Balance, BalanceError> {
        let now = self.now();
        let balance = self.get_balance_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound { account_id, asset_id })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount,
                available: balance.available,
            });
        }

        // 检查冻结是否会溢出
        let new_frozen = balance.frozen.checked_add(amount)
            .ok_or(BalanceError::Overflow)?;

        // 原子操作：检查通过后立即冻结
        balance.available -= amount;
        balance.frozen = new_frozen;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 冻结（可用 → 冻结）
    fn freeze_balance(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<Balance, BalanceError> {
        self.check_and_freeze_balance(account_id, asset_id, amount)
    }

    /// 解冻（冻结 → 可用）
    fn unfreeze_balance(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<Balance, BalanceError> {
        let now = self.now();
        let balance = self.get_balance_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound { account_id, asset_id })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount,
                frozen: balance.frozen,
            });
        }

        // 检查可用余额是否会溢出
        let new_available = balance.available.checked_add(amount)
            .ok_or(BalanceError::Overflow)?;

        balance.frozen -= amount;
        balance.available = new_available;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 入账（增加可用）
    fn credit_balance(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<Balance, BalanceError> {
        let now = self.now();
        let balance = self.get_or_create_balance_mut(account_id, asset_id);

        balance.available = balance.available.checked_add(amount)
            .ok_or(BalanceError::Overflow)?;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 扣款（减少可用）
    fn debit_balance(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<Balance, BalanceError> {
        let now = self.now();
        let balance = self.get_balance_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound { account_id, asset_id })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount,
                available: balance.available,
            });
        }

        balance.available -= amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 扣减冻结余额
    fn debit_frozen_balance(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<Balance, BalanceError> {
        let now = self.now();
        let balance = self.get_balance_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound { account_id, asset_id })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount,
                frozen: balance.frozen,
            });
        }

        balance.frozen -= amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 转账（跨账户，原子操作）
    ///
    /// 采用预检查策略：先验证所有条件，再执行操作
    fn transfer_balance(
        &mut self,
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<(), BalanceError> {
        let now = self.now();

        // 1. 预检查：源账户余额
        {
            let from_balance = self.get_balance(from_account_id, asset_id)
                .ok_or(BalanceError::BalanceNotFound {
                    account_id: from_account_id,
                    asset_id
                })?;

            if from_balance.available < amount {
                return Err(BalanceError::InsufficientAvailable {
                    required: amount,
                    available: from_balance.available,
                });
            }
        }

        // 2. 预检查：目标账户是否会溢出
        {
            if let Some(to_balance) = self.get_balance(to_account_id, asset_id) {
                to_balance.available.checked_add(amount)
                    .ok_or(BalanceError::Overflow)?;
            }
        }

        // 3. 执行转账（预检查通过后不会失败）
        // 扣减源账户
        {
            let from_balance = self.get_balance_mut(from_account_id, asset_id).unwrap();
            from_balance.available -= amount;
            from_balance.version += 1;
            from_balance.updated_at = now;
        }

        // 增加目标账户
        {
            let to_balance = self.get_or_create_balance_mut(to_account_id, asset_id);
            to_balance.available += amount;  // 已预检查，不会溢出
            to_balance.version += 1;
            to_balance.updated_at = now;
        }

        Ok(())
    }

    /// 结算盈亏
    fn settle_pnl(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        pnl: i64,
    ) -> Result<Balance, BalanceError> {
        let now = self.now();
        let balance = self.get_or_create_balance_mut(account_id, asset_id);

        if pnl >= 0 {
            // 盈利：增加可用
            balance.available = balance.available.checked_add(pnl as u64)
                .ok_or(BalanceError::Overflow)?;
        } else {
            // 亏损：减少可用
            let loss = (-pnl) as u64;
            if balance.available < loss {
                return Err(BalanceError::InsufficientAvailable {
                    required: loss,
                    available: balance.available,
                });
            }
            balance.available -= loss;
        }
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }
}

---

## 5. Repository 定义

### 5.1 BalanceStore (余额存储接口)

```rust
/// 余额存储接口
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
pub enum BalanceOp {
    Freeze(u64),
    Unfreeze(u64),
    Credit(u64),
    Debit(u64),
    DebitFrozen(u64),
    SettlePnl(i64),
}
```

---

## 6. 数据库 Schema

```sql
-- 账户表
CREATE TABLE accounts (
    id              BIGINT PRIMARY KEY,
    user_id         BIGINT NOT NULL,
    account_type    SMALLINT NOT NULL,
    status          SMALLINT NOT NULL DEFAULT 0,
    created_at      BIGINT NOT NULL,
    updated_at      BIGINT NOT NULL,

    INDEX idx_accounts_user (user_id)
);

-- 余额表
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
```

---

## 7. 与 LOB 模块集成

### 7.1 下单流程中的余额检查

```
┌─────────────────────────────────────────────────────────────────┐
│                    Limit Order Flow                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. MatchingService.limit_order(command)                        │
│     │                                                            │
│     ▼                                                            │
│  2. 分配 order_id                                                │
│     │                                                            │
│     ▼                                                            │
│  3. 构造 AccountCommand::CheckAndFreeze（原子操作）              │
│     └── account_service.execute(CheckAndFreeze { ... })         │
│         ├── 检查账户状态 (Active/Frozen/Closed)                 │
│         ├── 计算金额（带溢出检查）                               │
│         ├── Buy:  检查并冻结 quote_asset (USDT)                 │
│         └── Sell: 检查并冻结 base_asset (BTC)                   │
│     │                                                            │
│     ▼                                                            │
│  4. 执行撮合                                                     │
│     └── matching_service.match_orders(...)                       │
│     │                                                            │
│     ▼                                                            │
│  5. 生成结算命令 (由 Settlement 处理)                            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 7.2 MatchingService 集成示例

```rust
pub struct MatchingService<R, A>
where
    R: OrderRepository + RepositoryAccessor,
    A: AccountService,
{
    lob_repo: R,
    account_service: A,
    trading_pair: TradingPair,  // 当前交易对
    // ...
}

impl<R, A> MatchingService<R, A>
where
    R: OrderRepository + RepositoryAccessor + Send + Sync,
    A: AccountService,
{
    fn limit_order(&mut self, command: Command) -> CommandResult {
        if let Command::LimitOrder { trader, side, price, quantity } = command {
            let account_id = AccountId::from(trader);
            let order_id = self.lob_repo.allocate_order_id();

            // 原子操作：检查并冻结（防止 TOCTOU 竞态）
            let cmd = AccountCommand::CheckAndFreeze {
                account_id,
                order_id,
                pair: self.trading_pair,
                side,
                price: price as u64,
                quantity: quantity as u64,
            };

            match self.account_service.execute(cmd) {
                AccountCommandResult::Frozen { .. } => {
                    // 冻结成功，继续执行撮合...
                }
                AccountCommandResult::Error(e) => {
                    return CommandResult::Error(e.into());
                }
                _ => unreachable!(),
            }

            // 继续执行撮合...
        }
    }

    fn cancel_order(&mut self, order_id: OrderId) -> CommandResult {
        // 获取订单信息
        let order = self.lob_repo.get_order(order_id)?;

        // 解冻资金
        let cmd = AccountCommand::Unfreeze {
            account_id: AccountId::from(order.trader),
            order_id,
            pair: self.trading_pair,
            side: order.side,
            price: order.price as u64,
            quantity: order.unfilled_quantity as u64,
        };

        match self.account_service.execute(cmd) {
            AccountCommandResult::Unfrozen { .. } => {
                // 解冻成功，继续取消订单...
            }
            AccountCommandResult::Error(e) => {
                // 记录错误但继续取消（订单状态优先）
                log::warn!("Unfreeze failed for order {}: {:?}", order_id, e);
            }
            _ => unreachable!(),
        }

        // 继续取消订单...
    }
}
```

### 7.3 Settlement 集成示例

```rust
/// Settlement 服务使用 AccountCommand 进行结算
pub struct SettlementService<A: AccountService> {
    account_service: A,
}

impl<A: AccountService> SettlementService<A> {
    /// 处理成交结算（原子操作）
    ///
    /// 买方：扣减冻结的 quote_asset，入账 base_asset
    /// 卖方：扣减冻结的 base_asset，入账 quote_asset
    ///
    /// 使用 execute_batch 保证 4 个操作的原子性：
    /// - 全部成功：结算完成
    /// - 任意失败：全部回滚，无状态变更
    pub fn settle_trade(
        &mut self,
        trade_id: u64,
        buyer_account_id: AccountId,
        seller_account_id: AccountId,
        pair: TradingPair,
        price: u64,
        quantity: u64,
    ) -> Result<(), SettlementError> {
        let quote_amount = price.checked_mul(quantity)
            .ok_or(SettlementError::Overflow)?;

        // 构造批量命令（原子执行）
        let cmds = vec![
            // 1. 扣减买方冻结的 quote_asset (USDT)
            AccountCommand::DebitFrozen {
                account_id: buyer_account_id,
                asset_id: pair.quote_asset,
                amount: quote_amount,
                reference_id: trade_id,
            },
            // 2. 入账买方的 base_asset (BTC)
            AccountCommand::Credit {
                account_id: buyer_account_id,
                asset_id: pair.base_asset,
                amount: quantity,
                reference_id: trade_id,
            },
            // 3. 扣减卖方冻结的 base_asset (BTC)
            AccountCommand::DebitFrozen {
                account_id: seller_account_id,
                asset_id: pair.base_asset,
                amount: quantity,
                reference_id: trade_id,
            },
            // 4. 入账卖方的 quote_asset (USDT)
            AccountCommand::Credit {
                account_id: seller_account_id,
                asset_id: pair.quote_asset,
                amount: quote_amount,
                reference_id: trade_id,
            },
        ];

        // 原子执行：全部成功或全部失败
        self.account_service.execute_batch(cmds)
            .map_err(SettlementError::BalanceError)?;

        Ok(())
    }

    /// 转账（如划转资产）
    pub fn transfer(
        &mut self,
        transfer_id: u64,
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<(), SettlementError> {
        let cmd = AccountCommand::Transfer {
            from_account_id,
            to_account_id,
            asset_id,
            amount,
            reference_id: transfer_id,
        };
        self.execute_single(cmd)
    }

    /// 结算合约盈亏
    pub fn settle_pnl(
        &mut self,
        settlement_id: u64,
        account_id: AccountId,
        asset_id: AssetId,
        pnl: i64,
    ) -> Result<(), SettlementError> {
        let cmd = AccountCommand::SettlePnl {
            account_id,
            asset_id,
            pnl,
            reference_id: settlement_id,
        };
        self.execute_single(cmd)
    }

    /// 执行单个命令
    fn execute_single(&mut self, cmd: AccountCommand) -> Result<(), SettlementError> {
        match self.account_service.execute(cmd) {
            AccountCommandResult::Error(e) => Err(SettlementError::BalanceError(e)),
            _ => Ok(()),
        }
    }
}
```

---

## 8. 目录结构

```
lib/core/account/
├── Cargo.toml
├── design.md                          # 本文档
├── src/
│   ├── lib.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── entity/
│   │   │   ├── mod.rs
│   │   │   ├── account.rs            # Account, AccountType, AccountStatus
│   │   │   ├── balance.rs            # Balance, BalanceOp
│   │   │   ├── command.rs            # AccountCommand, AccountCommandResult
│   │   │   ├── trading_pair.rs       # TradingPair, Side
│   │   │   └── error.rs              # BalanceError
│   │   ├── service/
│   │   │   ├── mod.rs
│   │   │   └── account_service.rs    # AccountService trait + impl
│   │   └── repository/
│   │       ├── mod.rs
│   │       └── balance_store.rs      # BalanceStore trait
│   └── adaptor/
│       ├── mod.rs
│       ├── inbound/
│       │   ├── mod.rs
│       │   └── in_memory.rs          # InMemoryAccountService
│       └── outbound/
│           ├── mod.rs
│           └── postgres.rs           # PostgresBalanceStore
```

---

## 9. 下一步

1. **实现 Entity**：创建 domain/entity 下的实体定义
2. **实现 Service**：创建 AccountService trait 及 InMemory 实现
3. **集成 LOB**：在 MatchingService 中注入 AccountService
4. **实现持久化**：PostgresBalanceStore 实现
