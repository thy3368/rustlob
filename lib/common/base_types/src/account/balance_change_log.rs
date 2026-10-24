//! Balance 变更日志。
//!
//! 文件名和主要类型名保留以减少引用面；金额字段已经改为 `Decimal` 语义类型，
//! 不再承诺 POD、零拷贝、SIMD 友好或固定 128 字节布局。

use crate::account::balance_change::{BalanceChange, BalanceChangeReason, BalanceChangeType};
use crate::{AccountId, AssetId, Quantity, Timestamp};

/// Balance 变更事件日志记录。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BalanceChangeLog {
    /// 全局唯一序列号
    pub sequence_id: u64,
    /// 账户ID
    pub account_id: u64,
    /// 资产ID
    pub asset_id: u64,
    /// 变更类型（u8编码）
    pub change_type: u8,
    /// 变更原因（u8编码）
    pub reason: u8,
    /// 变更金额
    pub amount: Quantity,
    /// 变更前可用余额
    pub available_before: Quantity,
    /// 变更后可用余额
    pub available_after: Quantity,
    /// 变更前冻结余额
    pub frozen_before: Quantity,
    /// 变更后冻结余额
    pub frozen_after: Quantity,
    /// 关联订单ID（u64::MAX表示None）
    pub order_id: u64,
    /// 变更时间戳（纳秒）
    pub timestamp: u64,
    /// Balance版本号
    pub balance_version: u64,
}

impl BalanceChangeLog {
    /// 创建充值变更。
    #[inline]
    pub fn deposit(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: Quantity,
        available_before: Quantity,
        timestamp: u64,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Deposit as u8,
            reason: BalanceChangeReason::UserDeposit as u8,
            amount,
            available_before,
            available_after: available_before + amount,
            frozen_before: Quantity::ZERO,
            frozen_after: Quantity::ZERO,
            order_id: u64::MAX,
            timestamp,
            balance_version,
        }
    }

    /// 创建冻结变更（下单）。
    #[inline]
    pub fn freeze(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: Quantity,
        available_before: Quantity,
        frozen_before: Quantity,
        order_id: u64,
        timestamp: u64,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Freeze as u8,
            reason: BalanceChangeReason::OrderPlace as u8,
            amount,
            available_before,
            available_after: available_before - amount,
            frozen_before,
            frozen_after: frozen_before + amount,
            order_id,
            timestamp,
            balance_version,
        }
    }

    /// 创建解冻变更（撤单）。
    #[inline]
    pub fn unfreeze(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: Quantity,
        available_before: Quantity,
        frozen_before: Quantity,
        order_id: u64,
        timestamp: u64,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Unfreeze as u8,
            reason: BalanceChangeReason::OrderCancel as u8,
            amount,
            available_before,
            available_after: available_before + amount,
            frozen_before,
            frozen_after: frozen_before - amount,
            order_id,
            timestamp,
            balance_version,
        }
    }

    /// 创建成交扣款变更。
    #[inline]
    pub fn trade(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: Quantity,
        available_before: Quantity,
        frozen_before: Quantity,
        order_id: u64,
        timestamp: u64,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Trade as u8,
            reason: BalanceChangeReason::OrderFilled as u8,
            amount,
            available_before,
            available_after: available_before,
            frozen_before,
            frozen_after: frozen_before - amount,
            order_id,
            timestamp,
            balance_version,
        }
    }

    /// 验证变更的一致性。
    #[inline]
    pub fn validate(&self) -> bool {
        match self.change_type {
            1 => {
                self.available_after == self.available_before + self.amount
                    && self.frozen_after == self.frozen_before
            }
            3 => {
                self.available_after == self.available_before - self.amount
                    && self.frozen_after == self.frozen_before + self.amount
            }
            4 => {
                self.available_after == self.available_before + self.amount
                    && self.frozen_after == self.frozen_before - self.amount
            }
            5 => {
                self.available_after == self.available_before
                    && self.frozen_after == self.frozen_before - self.amount
            }
            _ => true,
        }
    }

    /// 计算总余额变化。
    #[inline]
    pub fn total_balance_delta(&self) -> Quantity {
        let total_before = self.available_before + self.frozen_before;
        let total_after = self.available_after + self.frozen_after;
        total_after - total_before
    }

    /// 检查是否有关联订单。
    #[inline]
    pub const fn has_order(&self) -> bool {
        self.order_id != u64::MAX
    }

    /// 获取变更类型。
    #[inline]
    pub const fn get_change_type(&self) -> Option<BalanceChangeType> {
        match self.change_type {
            1 => Some(BalanceChangeType::Deposit),
            2 => Some(BalanceChangeType::Withdraw),
            3 => Some(BalanceChangeType::Freeze),
            4 => Some(BalanceChangeType::Unfreeze),
            5 => Some(BalanceChangeType::Trade),
            6 => Some(BalanceChangeType::Fee),
            7 => Some(BalanceChangeType::Settlement),
            8 => Some(BalanceChangeType::Adjustment),
            _ => None,
        }
    }

    /// 获取变更原因。
    #[inline]
    pub const fn get_reason(&self) -> Option<BalanceChangeReason> {
        match self.reason {
            1 => Some(BalanceChangeReason::UserDeposit),
            2 => Some(BalanceChangeReason::UserWithdraw),
            3 => Some(BalanceChangeReason::OrderPlace),
            4 => Some(BalanceChangeReason::OrderCancel),
            5 => Some(BalanceChangeReason::OrderFilled),
            6 => Some(BalanceChangeReason::TradingFee),
            7 => Some(BalanceChangeReason::FundingRate),
            8 => Some(BalanceChangeReason::Liquidation),
            9 => Some(BalanceChangeReason::SystemAdjustment),
            _ => None,
        }
    }

    /// 从 BalanceChange 转换。
    #[inline]
    pub fn from_balance_change(change: &BalanceChange) -> Self {
        Self {
            sequence_id: change.sequence_id,
            account_id: change.account_id.0,
            asset_id: change.asset_id.as_u32() as u64,
            change_type: change.change_type as u8,
            reason: change.reason as u8,
            amount: change.amount,
            available_before: change.available_before,
            available_after: change.available_after,
            frozen_before: change.frozen_before,
            frozen_after: change.frozen_after,
            order_id: change.order_id.unwrap_or(u64::MAX),
            timestamp: change.timestamp.0,
            balance_version: change.balance_version,
        }
    }

    /// 转换为 BalanceChange。
    #[inline]
    pub fn to_balance_change(&self) -> Option<BalanceChange> {
        let asset_id = match self.asset_id as u32 {
            1 => AssetId::Usdt,
            2 => AssetId::Btc,
            3 => AssetId::Eth,
            _ => return None,
        };

        Some(BalanceChange {
            sequence_id: self.sequence_id,
            account_id: AccountId(self.account_id),
            asset_id,
            change_type: self.get_change_type()?,
            reason: self.get_reason()?,
            amount: self.amount,
            available_before: self.available_before,
            available_after: self.available_after,
            frozen_before: self.frozen_before,
            frozen_after: self.frozen_after,
            order_id: if self.order_id == u64::MAX { None } else { Some(self.order_id) },
            timestamp: Timestamp(self.timestamp),
            balance_version: self.balance_version,
        })
    }
}

/// Balance变更日志容量常量。
pub const BALANCE_CHANGE_LOG_CAPACITY: usize = 64;

/// 固定容量 Balance 变更日志。
#[derive(Debug, Clone, Copy)]
pub struct BalanceChangePodLog {
    /// 当前记录数量
    pub len: u64,
    /// 序列号数组
    pub sequence_ids: [u64; BALANCE_CHANGE_LOG_CAPACITY],
    /// 账户ID数组
    pub account_ids: [u64; BALANCE_CHANGE_LOG_CAPACITY],
    /// 资产ID数组
    pub asset_ids: [u64; BALANCE_CHANGE_LOG_CAPACITY],
    /// 变更类型数组
    pub change_types: [u8; BALANCE_CHANGE_LOG_CAPACITY],
    /// 变更原因数组
    pub reasons: [u8; BALANCE_CHANGE_LOG_CAPACITY],
    /// 变更金额数组
    pub amounts: [Quantity; BALANCE_CHANGE_LOG_CAPACITY],
    /// 变更前可用余额数组
    pub available_befores: [Quantity; BALANCE_CHANGE_LOG_CAPACITY],
    /// 变更后可用余额数组
    pub available_afters: [Quantity; BALANCE_CHANGE_LOG_CAPACITY],
    /// 变更前冻结余额数组
    pub frozen_befores: [Quantity; BALANCE_CHANGE_LOG_CAPACITY],
    /// 变更后冻结余额数组
    pub frozen_afters: [Quantity; BALANCE_CHANGE_LOG_CAPACITY],
    /// 订单ID数组
    pub order_ids: [u64; BALANCE_CHANGE_LOG_CAPACITY],
    /// 时间戳数组
    pub timestamps: [u64; BALANCE_CHANGE_LOG_CAPACITY],
    /// 版本号数组
    pub balance_versions: [u64; BALANCE_CHANGE_LOG_CAPACITY],
}

impl BalanceChangePodLog {
    /// 创建空日志。
    pub const fn new() -> Self {
        Self {
            len: 0,
            sequence_ids: [0; BALANCE_CHANGE_LOG_CAPACITY],
            account_ids: [0; BALANCE_CHANGE_LOG_CAPACITY],
            asset_ids: [0; BALANCE_CHANGE_LOG_CAPACITY],
            change_types: [0; BALANCE_CHANGE_LOG_CAPACITY],
            reasons: [0; BALANCE_CHANGE_LOG_CAPACITY],
            amounts: [Quantity::ZERO; BALANCE_CHANGE_LOG_CAPACITY],
            available_befores: [Quantity::ZERO; BALANCE_CHANGE_LOG_CAPACITY],
            available_afters: [Quantity::ZERO; BALANCE_CHANGE_LOG_CAPACITY],
            frozen_befores: [Quantity::ZERO; BALANCE_CHANGE_LOG_CAPACITY],
            frozen_afters: [Quantity::ZERO; BALANCE_CHANGE_LOG_CAPACITY],
            order_ids: [0; BALANCE_CHANGE_LOG_CAPACITY],
            timestamps: [0; BALANCE_CHANGE_LOG_CAPACITY],
            balance_versions: [0; BALANCE_CHANGE_LOG_CAPACITY],
        }
    }

    /// 添加变更记录。
    #[inline]
    pub fn push(&mut self, change: &BalanceChangeLog) -> Result<(), &'static str> {
        let idx = self.len as usize;
        if idx >= BALANCE_CHANGE_LOG_CAPACITY {
            return Err("BalanceChangePodLog is full");
        }

        self.sequence_ids[idx] = change.sequence_id;
        self.account_ids[idx] = change.account_id;
        self.asset_ids[idx] = change.asset_id;
        self.change_types[idx] = change.change_type;
        self.reasons[idx] = change.reason;
        self.amounts[idx] = change.amount;
        self.available_befores[idx] = change.available_before;
        self.available_afters[idx] = change.available_after;
        self.frozen_befores[idx] = change.frozen_before;
        self.frozen_afters[idx] = change.frozen_after;
        self.order_ids[idx] = change.order_id;
        self.timestamps[idx] = change.timestamp;
        self.balance_versions[idx] = change.balance_version;

        self.len += 1;
        Ok(())
    }

    /// 获取记录数量。
    #[inline]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// 是否为空。
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 是否已满。
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len as usize >= BALANCE_CHANGE_LOG_CAPACITY
    }

    /// 清空日志。
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// 按账户ID过滤。
    pub fn filter_by_account(&self, account_id: u64) -> Vec<usize> {
        let len = self.len();
        self.account_ids[..len]
            .iter()
            .enumerate()
            .filter_map(|(idx, &id)| if id == account_id { Some(idx) } else { None })
            .collect()
    }

    /// 按时间范围过滤。
    pub fn filter_by_time_range(&self, start: u64, end: u64) -> Vec<usize> {
        let len = self.len();
        self.timestamps[..len]
            .iter()
            .enumerate()
            .filter_map(|(idx, &ts)| if ts >= start && ts <= end { Some(idx) } else { None })
            .collect()
    }

    /// 按变更类型过滤。
    pub fn filter_by_type(&self, change_type: u8) -> Vec<usize> {
        let len = self.len();
        self.change_types[..len]
            .iter()
            .enumerate()
            .filter_map(|(idx, &t)| if t == change_type { Some(idx) } else { None })
            .collect()
    }
}

impl Default for BalanceChangePodLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_log_deposit() {
        let change = BalanceChangeLog::deposit(
            1,
            100,
            1,
            Decimal::new(1000, 0),
            Decimal::new(500, 0),
            1234567890,
            1,
        );

        assert_eq!(change.sequence_id, 1);
        assert_eq!(change.account_id, 100);
        assert_eq!(change.change_type, BalanceChangeType::Deposit as u8);
        assert_eq!(change.available_after, Decimal::new(1500, 0));
        assert!(change.validate());
    }

    #[test]
    fn test_log_freeze() {
        let change = BalanceChangeLog::freeze(
            2,
            100,
            1,
            Decimal::new(100, 0),
            Decimal::new(500, 0),
            Decimal::ZERO,
            1,
            1234567890,
            2,
        );

        assert_eq!(change.change_type, BalanceChangeType::Freeze as u8);
        assert_eq!(change.available_after, Decimal::new(400, 0));
        assert_eq!(change.frozen_after, Decimal::new(100, 0));
        assert!(change.validate());
        assert!(change.has_order());
    }

    #[test]
    fn test_log_total_balance_delta() {
        let deposit = BalanceChangeLog::deposit(
            1,
            100,
            1,
            Decimal::new(1000, 0),
            Decimal::new(500, 0),
            1234567890,
            1,
        );
        assert_eq!(deposit.total_balance_delta(), Decimal::new(1000, 0));

        let freeze = BalanceChangeLog::freeze(
            2,
            100,
            1,
            Decimal::new(100, 0),
            Decimal::new(500, 0),
            Decimal::ZERO,
            1,
            1234567890,
            2,
        );
        assert_eq!(freeze.total_balance_delta(), Decimal::ZERO);

        let trade = BalanceChangeLog::trade(
            3,
            100,
            1,
            Decimal::new(100, 0),
            Decimal::new(400, 0),
            Decimal::new(100, 0),
            1,
            1234567890,
            3,
        );
        assert_eq!(trade.total_balance_delta(), Decimal::new(-100, 0));
    }

    #[test]
    fn test_log_collection() {
        let mut log = BalanceChangePodLog::new();

        let change1 = BalanceChangeLog::deposit(
            1,
            100,
            1,
            Decimal::new(1000, 0),
            Decimal::ZERO,
            1234567890,
            1,
        );

        let change2 = BalanceChangeLog::freeze(
            2,
            100,
            1,
            Decimal::new(100, 0),
            Decimal::new(1000, 0),
            Decimal::ZERO,
            1,
            1234567891,
            2,
        );

        log.push(&change1).unwrap();
        log.push(&change2).unwrap();

        assert_eq!(log.len(), 2);
        assert_eq!(log.filter_by_account(100).len(), 2);
        assert_eq!(log.filter_by_type(BalanceChangeType::Deposit as u8).len(), 1);
    }
}
