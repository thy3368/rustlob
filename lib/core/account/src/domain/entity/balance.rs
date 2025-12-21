//! 余额实体定义

use base_types::{AccountId, AssetId, Timestamp};
use std::fmt;
use entity_derive::Entity;

/// 余额ID（复合键：account_id:asset_id）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BalanceId {
    pub account_id: AccountId,
    pub asset_id: AssetId,
}

impl BalanceId {
    pub fn new(account_id: AccountId, asset_id: AssetId) -> Self {
        Self { account_id, asset_id }
    }
}

impl fmt::Display for BalanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.account_id.0, self.asset_id.0)
    }
}

impl Default for BalanceId {
    fn default() -> Self {
        Self {
            account_id: AccountId(0),
            asset_id: AssetId(0),
        }
    }
}

/// 资产余额（统一资产模型）
///
/// 示例：
/// - Balance(account, USDT, 100_000_000) = 100 USDT
/// - Balance(account, BTC, 100_000_000)  = 1 BTC
/// - Balance(account, AAPL, 1000)        = 1000 股苹果
#[derive(Debug, Clone, Entity)]
#[repr(align(64))]
pub struct Balance {
    /// 余额ID（复合键）
    pub id: BalanceId,
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
    pub updated_at: Timestamp
}

impl Balance {
    /// 创建新余额记录
    pub fn new(account_id: AccountId, asset_id: AssetId, now: Timestamp) -> Self {
        Self {
            id: BalanceId::new(account_id, asset_id),
            account_id,
            asset_id,
            available: 0,
            frozen: 0,
            version: 0,
            updated_at: now
        }
    }

    /// 创建带初始余额的记录
    pub fn with_available(account_id: AccountId, asset_id: AssetId, available: u64, now: Timestamp) -> Self {
        Self {
            id: BalanceId::new(account_id, asset_id),
            account_id,
            asset_id,
            available,
            frozen: 0,
            version: 0,
            updated_at: now
        }
    }

    /// 总余额 = 可用 + 冻结
    #[inline]
    pub fn total(&self) -> u64 { self.available.saturating_add(self.frozen) }

    /// 检查是否有足够的可用余额
    #[inline]
    pub fn has_available(&self, amount: u64) -> bool { self.available >= amount }

    /// 检查是否有足够的冻结余额
    #[inline]
    pub fn has_frozen(&self, amount: u64) -> bool { self.frozen >= amount }

    #[inline]
    pub fn add_balance(&mut self, amount: u64, now: Timestamp) {
        self.available += amount;
        self.version += 1;
        self.updated_at = now;
    }

    #[inline]
    pub fn deduct_balance(&mut self, amount: u64, now: Timestamp) {
        // if self.available < amount {
        //     return Err(PrepCommandError::InsufficientBalance);
        // }
        self.available -= amount;

        self.version += 1;
        self.updated_at = now;
    }

    /// 检查余额是否为空
    #[inline]
    pub fn is_empty(&self) -> bool { self.available == 0 && self.frozen == 0 }
}

/// 余额操作（用于 BalanceStore）
#[derive(Debug, Clone, Copy)]
pub enum BalanceOp {
    /// 冻结（可用 → 冻结）
    Freeze(u64),
    /// 解冻（冻结 → 可用）
    Unfreeze(u64),
    /// 入账（增加可用）
    Credit(u64),
    /// 扣款（减少可用）
    Debit(u64),
    /// 扣减冻结余额
    DebitFrozen(u64),
    /// 结算盈亏（可正可负）
    SettlePnl(i64)
}
