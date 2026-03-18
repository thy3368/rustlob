//! Balance变更追溯模型
//!
//! 设计原则：
//! - 不可变事件流：每次balance变更生成一个不可变的BalanceChange事件
//! - 时序保证：使用sequence_id保证全局顺序
//! - 审计追溯：记录变更原因、操作类型、关联订单等
//! - 缓存行对齐：关键数据结构64字节对齐
//! - 零分配：使用固定大小类型，避免堆分配

use crate::{AccountId, AssetId, OrderId, Quantity, Timestamp};

/// Balance变更类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BalanceChangeType {
    /// 充值
    Deposit = 1,
    /// 提现
    Withdraw = 2,
    /// 冻结（下单）
    Freeze = 3,
    /// 解冻（撤单）
    Unfreeze = 4,
    /// 成交扣款（从冻结余额扣除）
    Trade = 5,
    /// 手续费
    Fee = 6,
    /// 结算盈亏
    Settlement = 7,
    /// 系统调整
    Adjustment = 8,
}

/// Balance变更原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BalanceChangeReason {
    /// 用户充值
    UserDeposit = 1,
    /// 用户提现
    UserWithdraw = 2,
    /// 下单冻结
    OrderPlace = 3,
    /// 撤单解冻
    OrderCancel = 4,
    /// 订单成交
    OrderFilled = 5,
    /// 交易手续费
    TradingFee = 6,
    /// 资金费率结算
    FundingRate = 7,
    /// 强平
    Liquidation = 8,
    /// 系统调整
    SystemAdjustment = 9,
}

/// Balance变更事件（不可变）
///
/// 设计要点：
/// - 64字节缓存行对齐
/// - 固定大小，无堆分配
/// - 包含完整的审计信息
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
//todo 写个POD 类型版本
pub struct BalanceChange {
    /// 全局唯一序列号（保证顺序性）
    pub sequence_id: u64,
    /// 账户ID
    pub account_id: AccountId,
    /// 资产ID
    pub asset_id: AssetId,
    /// 变更类型
    pub change_type: BalanceChangeType,
    /// 变更原因
    pub reason: BalanceChangeReason,
    /// 变更金额（正数=增加，负数=减少）
    pub amount: Quantity,
    /// 变更前可用余额
    pub available_before: Quantity,
    /// 变更后可用余额
    pub available_after: Quantity,
    /// 变更前冻结余额
    pub frozen_before: Quantity,
    /// 变更后冻结余额
    pub frozen_after: Quantity,
    /// 关联订单ID（如果有）
    pub order_id: Option<OrderId>,
    /// 变更时间戳
    pub timestamp: Timestamp,
    /// Balance版本号（乐观锁）
    pub balance_version: u64,
}

impl BalanceChange {
    /// 创建充值变更
    #[inline]
    pub fn deposit(
        sequence_id: u64,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Quantity,
        available_before: Quantity,
        timestamp: Timestamp,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Deposit,
            reason: BalanceChangeReason::UserDeposit,
            amount,
            available_before,
            available_after: available_before + amount,
            frozen_before: Quantity::default(),
            frozen_after: Quantity::default(),
            order_id: None,
            timestamp,
            balance_version,
        }
    }

    /// 创建冻结变更（下单）
    #[inline]
    pub fn freeze(
        sequence_id: u64,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Quantity,
        available_before: Quantity,
        frozen_before: Quantity,
        order_id: OrderId,
        timestamp: Timestamp,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Freeze,
            reason: BalanceChangeReason::OrderPlace,
            amount,
            available_before,
            available_after: available_before - amount,
            frozen_before,
            frozen_after: frozen_before + amount,
            order_id: Some(order_id),
            timestamp,
            balance_version,
        }
    }

    /// 创建解冻变更（撤单）
    #[inline]
    pub fn unfreeze(
        sequence_id: u64,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Quantity,
        available_before: Quantity,
        frozen_before: Quantity,
        order_id: OrderId,
        timestamp: Timestamp,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Unfreeze,
            reason: BalanceChangeReason::OrderCancel,
            amount,
            available_before,
            available_after: available_before + amount,
            frozen_before,
            frozen_after: frozen_before - amount,
            order_id: Some(order_id),
            timestamp,
            balance_version,
        }
    }

    /// 创建成交扣款变更
    #[inline]
    pub fn trade(
        sequence_id: u64,
        account_id: AccountId,
        asset_id: AssetId,
        amount: Quantity,
        available_before: Quantity,
        frozen_before: Quantity,
        order_id: OrderId,
        timestamp: Timestamp,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Trade,
            reason: BalanceChangeReason::OrderFilled,
            amount,
            available_before,
            available_after: available_before,
            frozen_before,
            frozen_after: frozen_before - amount,
            order_id: Some(order_id),
            timestamp,
            balance_version,
        }
    }

    /// 验证变更的一致性
    #[inline]
    pub fn validate(&self) -> bool {
        // 验证余额变化的数学一致性
        match self.change_type {
            BalanceChangeType::Deposit => {
                self.available_after == self.available_before + self.amount
                    && self.frozen_after == self.frozen_before
            }
            BalanceChangeType::Freeze => {
                self.available_after == self.available_before - self.amount
                    && self.frozen_after == self.frozen_before + self.amount
            }
            BalanceChangeType::Unfreeze => {
                self.available_after == self.available_before + self.amount
                    && self.frozen_after == self.frozen_before - self.amount
            }
            BalanceChangeType::Trade => {
                self.available_after == self.available_before
                    && self.frozen_after == self.frozen_before - self.amount
            }
            _ => true,
        }
    }

    /// 计算总余额变化
    #[inline]
    pub fn total_balance_delta(&self) -> Quantity {
        let total_before = self.available_before + self.frozen_before;
        let total_after = self.available_after + self.frozen_after;
        total_after - total_before
    }
}

/// Balance变更日志（SoA结构，SIMD友好）
///
/// 用于批量处理和查询
#[derive(Debug, Clone)]
pub struct BalanceChangeLog {
    /// 序列号数组
    pub sequence_ids: Vec<u64>,
    /// 账户ID数组
    pub account_ids: Vec<AccountId>,
    /// 资产ID数组
    pub asset_ids: Vec<AssetId>,
    /// 变更类型数组
    pub change_types: Vec<BalanceChangeType>,
    /// 变更金额数组
    pub amounts: Vec<Quantity>,
    /// 时间戳数组
    pub timestamps: Vec<Timestamp>,
    /// 订单ID数组（Option通过u64::MAX表示None）
    pub order_ids: Vec<u64>,
}

impl BalanceChangeLog {
    /// 创建空日志
    pub fn new() -> Self {
        Self {
            sequence_ids: Vec::new(),
            account_ids: Vec::new(),
            asset_ids: Vec::new(),
            change_types: Vec::new(),
            amounts: Vec::new(),
            timestamps: Vec::new(),
            order_ids: Vec::new(),
        }
    }

    /// 预分配容量
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sequence_ids: Vec::with_capacity(capacity),
            account_ids: Vec::with_capacity(capacity),
            asset_ids: Vec::with_capacity(capacity),
            change_types: Vec::with_capacity(capacity),
            amounts: Vec::with_capacity(capacity),
            timestamps: Vec::with_capacity(capacity),
            order_ids: Vec::with_capacity(capacity),
        }
    }

    /// 添加变更记录
    pub fn push(&mut self, change: &BalanceChange) {
        self.sequence_ids.push(change.sequence_id);
        self.account_ids.push(change.account_id);
        self.asset_ids.push(change.asset_id);
        self.change_types.push(change.change_type);
        self.amounts.push(change.amount);
        self.timestamps.push(change.timestamp);
        self.order_ids.push(change.order_id.unwrap_or(u64::MAX));
    }

    /// 获取记录数量
    #[inline]
    pub fn len(&self) -> usize {
        self.sequence_ids.len()
    }

    /// 是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sequence_ids.is_empty()
    }

    /// 按账户ID过滤（SIMD优化潜力）
    pub fn filter_by_account(&self, account_id: AccountId) -> Vec<usize> {
        self.account_ids
            .iter()
            .enumerate()
            .filter_map(|(idx, &id)| if id == account_id { Some(idx) } else { None })
            .collect()
    }

    /// 按时间范围过滤
    pub fn filter_by_time_range(&self, start: Timestamp, end: Timestamp) -> Vec<usize> {
        self.timestamps
            .iter()
            .enumerate()
            .filter_map(|(idx, &ts)| {
                if ts.0 >= start.0 && ts.0 <= end.0 {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for BalanceChangeLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_change_deposit() {
        let change = BalanceChange::deposit(
            1,
            AccountId(100),
            AssetId::default(),
            Quantity::from_raw(1000_00000000),
            Quantity::from_raw(500_00000000),
            Timestamp::now(),
            1,
        );

        assert_eq!(change.change_type, BalanceChangeType::Deposit);
        assert_eq!(change.available_after.raw(), 1500_00000000);
        assert!(change.validate());
    }

    #[test]
    fn test_balance_change_freeze() {
        let change = BalanceChange::freeze(
            2,
            AccountId(100),
            AssetId::default(),
            Quantity::from_raw(100_00000000),
            Quantity::from_raw(500_00000000),
            Quantity::from_raw(0),
            1, // OrderId is u64
            Timestamp::now(),
            2,
        );

        assert_eq!(change.change_type, BalanceChangeType::Freeze);
        assert_eq!(change.available_after.raw(), 400_00000000);
        assert_eq!(change.frozen_after.raw(), 100_00000000);
        assert!(change.validate());
    }

    #[test]
    fn test_balance_change_log() {
        let mut log = BalanceChangeLog::with_capacity(10);

        let change1 = BalanceChange::deposit(
            1,
            AccountId(100),
            AssetId::default(),
            Quantity::from_raw(1000_00000000),
            Quantity::from_raw(0),
            Timestamp::now(),
            1,
        );

        let change2 = BalanceChange::freeze(
            2,
            AccountId(100),
            AssetId::default(),
            Quantity::from_raw(100_00000000),
            Quantity::from_raw(1000_00000000),
            Quantity::from_raw(0),
            1, // OrderId is u64
            Timestamp::now(),
            2,
        );

        log.push(&change1);
        log.push(&change2);

        assert_eq!(log.len(), 2);
        assert_eq!(log.filter_by_account(AccountId(100)).len(), 2);
    }

    #[test]
    fn test_total_balance_delta() {
        // 充值：总余额增加
        let deposit = BalanceChange::deposit(
            1,
            AccountId(100),
            AssetId::default(),
            Quantity::from_raw(1000_00000000),
            Quantity::from_raw(500_00000000),
            Timestamp::now(),
            1,
        );
        assert_eq!(deposit.total_balance_delta().raw(), 1000_00000000);

        // 冻结：总余额不变
        let freeze = BalanceChange::freeze(
            2,
            AccountId(100),
            AssetId::default(),
            Quantity::from_raw(100_00000000),
            Quantity::from_raw(500_00000000),
            Quantity::from_raw(0),
            1, // OrderId is u64
            Timestamp::now(),
            2,
        );
        assert_eq!(freeze.total_balance_delta().raw(), 0);

        // 成交：总余额减少
        let trade = BalanceChange::trade(
            3,
            AccountId(100),
            AssetId::default(),
            Quantity::from_raw(100_00000000),
            Quantity::from_raw(400_00000000),
            Quantity::from_raw(100_00000000),
            1, // OrderId is u64
            Timestamp::now(),
            3,
        );
        assert_eq!(trade.total_balance_delta().raw(), -100_00000000);
    }
}
