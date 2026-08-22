//! Decimal 余额结构。
//!
//! 文件名保留以减少引用面；金额字段已经改为 `Decimal` 语义类型，不再承诺
//! POD、零拷贝、SIMD 友好或固定缓存行布局。

use crate::account::error::BalanceError;
use crate::{AccountId, AssetId, Quantity, Timestamp};

/// 使用 Decimal 金额的资产余额。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Balance {
    /// 账户ID
    pub account_id: u64,
    /// 资产ID（对应AssetId枚举值）
    pub asset_id: u32,
    /// 可用余额
    pub available: Quantity,
    /// 冻结余额
    pub frozen: Quantity,
    /// 乐观锁版本号（每次修改 +1）
    pub version: u64,
    /// 最后更新时间（纳秒时间戳）
    pub updated_at: u64,
}

impl Balance {
    /// 创建新余额记录。
    #[inline]
    pub fn new(account_id: u64, asset_id: u32, now: u64) -> Self {
        Self {
            account_id,
            asset_id,
            available: Quantity::ZERO,
            frozen: Quantity::ZERO,
            version: 0,
            updated_at: now,
        }
    }

    /// 创建带初始可用余额的记录。
    #[inline]
    pub fn with_available(account_id: u64, asset_id: u32, available: Quantity, now: u64) -> Self {
        Self {
            account_id,
            asset_id,
            available,
            frozen: Quantity::ZERO,
            version: 0,
            updated_at: now,
        }
    }

    /// 从包装类型创建。
    #[inline]
    pub fn from_wrapped(
        account_id: AccountId,
        asset_id: AssetId,
        available: Quantity,
        frozen: Quantity,
        version: u64,
        updated_at: Timestamp,
    ) -> Self {
        Self {
            account_id: account_id.0,
            asset_id: asset_id as u32,
            available,
            frozen,
            version,
            updated_at: updated_at.0,
        }
    }

    /// 获取总余额（可用 + 冻结）。
    #[inline]
    pub fn total(&self) -> Quantity {
        self.available + self.frozen
    }

    /// 检查余额是否为空。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.available.is_zero() && self.frozen.is_zero()
    }

    /// 检查是否有足够的可用余额。
    #[inline]
    pub fn has_available(&self, amount: Quantity) -> bool {
        self.available >= amount
    }

    /// 检查是否有足够的冻结余额。
    #[inline]
    pub fn has_frozen(&self, amount: Quantity) -> bool {
        self.frozen >= amount
    }

    /// 添加可用余额。
    #[inline]
    pub fn add_available(&mut self, amount: Quantity, now: u64) {
        self.available += amount;
        self.version += 1;
        self.updated_at = now;
    }

    /// 冻结余额（可用 -> 冻结）。
    #[inline]
    pub fn freeze(&mut self, amount: Quantity, now: u64) -> Result<(), BalanceError> {
        if self.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount,
                available: self.available,
            });
        }
        self.available -= amount;
        self.frozen += amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 解冻余额（冻结 -> 可用）。
    #[inline]
    pub fn unfreeze(&mut self, amount: Quantity, now: u64) -> Result<(), BalanceError> {
        if self.frozen < amount {
            return Err(BalanceError::InsufficientFrozen { required: amount, frozen: self.frozen });
        }
        self.frozen -= amount;
        self.available += amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 从冻结余额中扣款（冻结 -> 扣除）。
    #[inline]
    pub fn debit_frozen(&mut self, amount: Quantity, now: u64) -> Result<(), BalanceError> {
        if self.frozen < amount {
            return Err(BalanceError::InsufficientFrozen { required: amount, frozen: self.frozen });
        }
        self.frozen -= amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 结算盈亏（可正可负）。
    #[inline]
    pub fn settle_pnl(&mut self, pnl: Quantity, now: u64) -> Result<(), BalanceError> {
        if pnl.is_sign_negative() && self.available < -pnl {
            return Err(BalanceError::InsufficientAvailable {
                required: -pnl,
                available: self.available,
            });
        }
        self.available += pnl;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }
}

impl Default for Balance {
    fn default() -> Self {
        Self::new(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_balance_creation() {
        let balance = Balance::new(1, 1, 1234567890);
        assert_eq!(balance.account_id, 1);
        assert_eq!(balance.asset_id, 1);
        assert_eq!(balance.available, Decimal::ZERO);
        assert_eq!(balance.frozen, Decimal::ZERO);
    }

    #[test]
    fn test_freeze() {
        let mut balance = Balance::with_available(1, 1, Decimal::new(100, 0), 1234567890);
        balance.freeze(Decimal::new(50, 0), 1234567891).unwrap();
        assert_eq!(balance.available, Decimal::new(50, 0));
        assert_eq!(balance.frozen, Decimal::new(50, 0));
    }
}
