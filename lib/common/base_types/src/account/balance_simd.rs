//! SIMD友好的余额结构（使用原始类型）
//!
//! 设计目标：
//! - 使用纯原始类型（i64, u64, u32）实现零拷贝
//! - 64字节缓存行对齐，避免false sharing
//! - 符合CLAUDE.md中的低时延和Clean Architecture要求

use crate::account::error::BalanceError;
use crate::{AccountId, AssetId, Quantity, Timestamp};

/// SIMD友好的资产余额（使用原始类型）
///
/// # 内存布局（64字节 = 1个缓存行）
///
/// ```text
/// Offset | Field        | Size | Description
/// -------|--------------|------|---------------------------
/// 0      | account_id   | 8    | 账户ID (u64)
/// 8      | asset_id     | 4    | 资产ID (u32)
/// 12     | _pad1        | 4    | 对齐padding
/// 16     | available    | 8    | 可用余额 (i64)
/// 24     | frozen       | 8    | 冻结余额 (i64)
/// 32     | version      | 8    | 版本号 (u64)
/// 40     | updated_at   | 8    | 更新时间 (u64)
/// 48     | _pad2        | 16   | 填充到64字节
/// ```
///
/// # 精度说明
///
/// 所有金额字段使用8位小数精度（与Decimal一致）：
/// - 最小单位：0.00000001
/// - 示例：10000000000 = 100.00000000 USDT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct Balance {
    /// 账户ID（原始u64）
    pub account_id: u64,
    /// 资产ID（原始u32，对应AssetId枚举值）
    pub asset_id: u32,
    /// 对齐padding
    _pad1: u32,
    /// 可用余额（原始i64，8位小数精度）
    pub available: i64,
    /// 冻结余额（原始i64，8位小数精度）
    pub frozen: i64,
    /// 乐观锁版本号（每次修改 +1）
    pub version: u64,
    /// 最后更新时间（纳秒时间戳）
    pub updated_at: u64,
    /// 填充到64字节
    _pad2: [u64; 2],
}

impl Balance {
    /// 创建新余额记录
    #[inline]
    pub const fn new(account_id: u64, asset_id: u32, now: u64) -> Self {
        Self {
            account_id,
            asset_id,
            _pad1: 0,
            available: 0,
            frozen: 0,
            version: 0,
            updated_at: now,
            _pad2: [0; 2],
        }
    }

    /// 创建带初始可用余额的记录
    #[inline]
    pub const fn with_available(account_id: u64, asset_id: u32, available: i64, now: u64) -> Self {
        Self {
            account_id,
            asset_id,
            _pad1: 0,
            available,
            frozen: 0,
            version: 0,
            updated_at: now,
            _pad2: [0; 2],
        }
    }

    /// 从包装类型创建（兼容性方法）
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
            _pad1: 0,
            available: available.raw(),
            frozen: frozen.raw(),
            version,
            updated_at: updated_at.0,
            _pad2: [0; 2],
        }
    }

    /// 获取总余额（可用 + 冻结）
    #[inline]
    pub const fn total(&self) -> i64 {
        self.available + self.frozen
    }

    /// 检查余额是否为空
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.available == 0 && self.frozen == 0
    }

    /// 检查是否有足够的可用余额
    #[inline]
    pub const fn has_available(&self, amount: i64) -> bool {
        self.available >= amount
    }

    /// 检查是否有足够的冻结余额
    #[inline]
    pub const fn has_frozen(&self, amount: i64) -> bool {
        self.frozen >= amount
    }

    /// 添加可用余额
    #[inline]
    pub fn add_available(&mut self, amount: i64, now: u64) {
        self.available += amount;
        self.version += 1;
        self.updated_at = now;
    }

    /// 冻结余额（可用 → 冻结）
    #[inline]
    pub fn freeze(&mut self, amount: i64, now: u64) -> Result<(), BalanceError> {
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

    /// 解冻余额（冻结 → 可用）
    #[inline]
    pub fn unfreeze(&mut self, amount: i64, now: u64) -> Result<(), BalanceError> {
        if self.frozen < amount {
            return Err(BalanceError::InsufficientFrozen { required: amount, frozen: self.frozen });
        }
        self.frozen -= amount;
        self.available += amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 从冻结余额中扣款（冻结 → 扣除）
    #[inline]
    pub fn debit_frozen(&mut self, amount: i64, now: u64) -> Result<(), BalanceError> {
        if self.frozen < amount {
            return Err(BalanceError::InsufficientFrozen { required: amount, frozen: self.frozen });
        }
        self.frozen -= amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 结算盈亏（可正可负）
    #[inline]
    pub fn settle_pnl(&mut self, pnl: i64, now: u64) -> Result<(), BalanceError> {
        if pnl < 0 && self.available < -pnl {
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
    use super::*;

    #[test]
    fn test_balance_size() {
        assert_eq!(std::mem::size_of::<Balance>(), 64);
        assert_eq!(std::mem::align_of::<Balance>(), 64);
    }

    #[test]
    fn test_balance_creation() {
        let balance = Balance::new(1, 1, 1234567890);
        assert_eq!(balance.account_id, 1);
        assert_eq!(balance.asset_id, 1);
        assert_eq!(balance.available, 0);
        assert_eq!(balance.frozen, 0);
    }

    #[test]
    fn test_freeze() {
        let mut balance = Balance::with_available(1, 1, 10000000000, 1234567890);
        balance.freeze(5000000000, 1234567891).unwrap();
        assert_eq!(balance.available, 5000000000);
        assert_eq!(balance.frozen, 5000000000);
    }
}
