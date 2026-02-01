//! 余额实体定义

use std::fmt;

use entity_derive::Entity;
use crate::{AccountId, AssetId, Price, Quantity, Timestamp};
use crate::exchange::spot::spot_types::SpotOrder;

/// 余额ID（复合键：account_id:asset_id）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BalanceId {
    pub account_id: AccountId,
    pub asset_id: AssetId
}

impl BalanceId {
    pub fn new(account_id: AccountId, asset_id: AssetId) -> Self {
        Self {
            account_id,
            asset_id
        }
    }
}

impl fmt::Display for BalanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}:{}", self.account_id.0, u32::from(self.asset_id)) }
}

impl Default for BalanceId {
    fn default() -> Self {
        Self {
            account_id: AccountId(0),
            asset_id: AssetId::default()
        }
    }
}

/// 资产余额（统一资产模型，支持8位小数精度）
///
/// 使用 Price (i64) 存储，确保与持仓、PnL 等计算精度一致
///
/// 示例：
/// - Balance(account, USDT, 100.00000000) = 100 USDT
/// - Balance(account, BTC, 1.00000000)    = 1 BTC
/// - Balance(account, AAPL, 1000.00000000) = 1000 股苹果
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
    /// 使用 Price 类型保证 8 位小数精度
    pub available: Quantity,
    /// 冻结余额（已锁定用于挂单、保证金）
    /// 使用 Price 类型保证 8 位小数精度
    pub frozen: Quantity,
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
            available: Quantity::default(),
            frozen: Quantity::default(),
            version: 0,
            updated_at: now
        }
    }

    /// 创建带初始余额的记录
    pub fn with_available(account_id: AccountId, asset_id: AssetId, available: i64, now: Timestamp) -> Self {
        Self {
            id: BalanceId::new(account_id, asset_id),
            account_id,
            asset_id,
            available: Quantity::from_raw(available),
            frozen: Quantity::default(),
            version: 0,
            updated_at: now
        }
    }



    /// 检查是否有足够的冻结余额
    #[inline]
    pub fn has_frozen(&self, amount: Quantity) -> bool { self.frozen >= amount }

    #[inline]
    pub fn add_balance(&mut self, amount: Quantity, now: Timestamp) {
        self.available = self.available + amount;
        self.version += 1;
        self.updated_at = now;
    }

    #[inline]
    pub fn frozen(&mut self, amount: Quantity, now: Timestamp) {
        self.available = self.available - amount;
        self.frozen = self.frozen + amount;
        self.version += 1;
        self.updated_at = now;
    }

    #[inline]
    pub fn frozen2pay(&mut self, amount: Quantity, now: Timestamp) {
        self.frozen = self.frozen - amount;
        self.version += 1;
        self.updated_at = now;
    }


    #[inline]
    pub fn un_frozen(&mut self, amount: Quantity, now: Timestamp) {
        self.available = self.available + amount;
        self.frozen = self.frozen - amount;
        self.version += 1;
        self.updated_at = now;
    }

    /// 检查余额是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.available.is_zero() && self.frozen.is_zero()
    }
}

/// 余额操作（用于 BalanceStore）
#[derive(Debug, Clone, Copy)]
pub enum BalanceOp {
    /// 冻结（可用 → 冻结）
    Freeze(Price),
    /// 解冻（冻结 → 可用）
    Unfreeze(Price),
    /// 入账（增加可用）
    Credit(Price),
    /// 扣款（减少可用）
    Debit(Price),
    /// 扣减冻结余额
    DebitFrozen(Price),
    /// 结算盈亏（可正可负）
    SettlePnl(Price)
}
