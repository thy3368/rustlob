//! Balance变更追溯模型 - POD版本
//!
//! Plain Old Data类型，适合：
//! - 零拷贝序列化/反序列化
//! - 直接内存映射
//! - 网络传输
//! - 共享内存
//! - SIMD批量处理
//!
//! 设计原则：
//! - 只使用基础类型（u8, u64, i64）
//! - 无Option、enum等复杂类型
//! - 可以直接memcpy
//! - 128字节对齐（2个缓存行）

use crate::account::balance_change::{BalanceChange, BalanceChangeReason, BalanceChangeType};
use crate::{AccountId, AssetId, Quantity, Timestamp};

/// Balance变更事件（POD版本）
///
/// 设计要点：
/// - 128字节对齐（2个缓存行）
/// - 只使用基础类型（u8, u64, i64）
/// - 无Option、enum等复杂类型
/// - 可以直接memcpy
/// - 支持零拷贝序列化
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(128))]
pub struct BalanceChangePod {
    // ===== 第一缓存行（64字节）=====
    /// 全局唯一序列号
    pub sequence_id: u64,
    /// 账户ID
    pub account_id: u64,
    /// 资产ID
    pub asset_id: u64,
    /// 变更类型（u8编码）
    /// 1=Deposit, 2=Withdraw, 3=Freeze, 4=Unfreeze, 5=Trade, 6=Fee, 7=Settlement, 8=Adjustment
    pub change_type: u8,
    /// 变更原因（u8编码）
    /// 1=UserDeposit, 2=UserWithdraw, 3=OrderPlace, 4=OrderCancel, 5=OrderFilled,
    /// 6=TradingFee, 7=FundingRate, 8=Liquidation, 9=SystemAdjustment
    pub reason: u8,
    /// 保留字段（对齐）
    pub _padding1: [u8; 6],
    /// 变更金额（i64，8位小数精度）
    pub amount: i64,
    /// 变更前可用余额
    pub available_before: i64,
    /// 变更后可用余额
    pub available_after: i64,

    // ===== 第二缓存行（64字节）=====
    /// 变更前冻结余额
    pub frozen_before: i64,
    /// 变更后冻结余额
    pub frozen_after: i64,
    /// 关联订单ID（u64::MAX表示None）
    pub order_id: u64,
    /// 变更时间戳（纳秒）
    pub timestamp: u64,
    /// Balance版本号
    pub balance_version: u64,
    /// 保留字段（未来扩展）
    pub _reserved: [u64; 3],
}

impl BalanceChangePod {
    /// 创建充值变更
    #[inline]
    pub const fn deposit(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: i64,
        available_before: i64,
        timestamp: u64,
        balance_version: u64,
    ) -> Self {
        Self {
            sequence_id,
            account_id,
            asset_id,
            change_type: BalanceChangeType::Deposit as u8,
            reason: BalanceChangeReason::UserDeposit as u8,
            _padding1: [0; 6],
            amount,
            available_before,
            available_after: available_before + amount,
            frozen_before: 0,
            frozen_after: 0,
            order_id: u64::MAX, // None
            timestamp,
            balance_version,
            _reserved: [0; 3],
        }
    }

    /// 创建冻结变更（下单）
    #[inline]
    pub const fn freeze(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: i64,
        available_before: i64,
        frozen_before: i64,
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
            _padding1: [0; 6],
            amount,
            available_before,
            available_after: available_before - amount,
            frozen_before,
            frozen_after: frozen_before + amount,
            order_id,
            timestamp,
            balance_version,
            _reserved: [0; 3],
        }
    }

    /// 创建解冻变更（撤单）
    #[inline]
    pub const fn unfreeze(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: i64,
        available_before: i64,
        frozen_before: i64,
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
            _padding1: [0; 6],
            amount,
            available_before,
            available_after: available_before + amount,
            frozen_before,
            frozen_after: frozen_before - amount,
            order_id,
            timestamp,
            balance_version,
            _reserved: [0; 3],
        }
    }

    /// 创建成交扣款变更
    #[inline]
    pub const fn trade(
        sequence_id: u64,
        account_id: u64,
        asset_id: u64,
        amount: i64,
        available_before: i64,
        frozen_before: i64,
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
            _padding1: [0; 6],
            amount,
            available_before,
            available_after: available_before,
            frozen_before,
            frozen_after: frozen_before - amount,
            order_id,
            timestamp,
            balance_version,
            _reserved: [0; 3],
        }
    }

    /// 验证变更的一致性
    #[inline]
    pub const fn validate(&self) -> bool {
        match self.change_type {
            1 => {
                // Deposit
                self.available_after == self.available_before + self.amount
                    && self.frozen_after == self.frozen_before
            }
            3 => {
                // Freeze
                self.available_after == self.available_before - self.amount
                    && self.frozen_after == self.frozen_before + self.amount
            }
            4 => {
                // Unfreeze
                self.available_after == self.available_before + self.amount
                    && self.frozen_after == self.frozen_before - self.amount
            }
            5 => {
                // Trade
                self.available_after == self.available_before
                    && self.frozen_after == self.frozen_before - self.amount
            }
            _ => true,
        }
    }

    /// 计算总余额变化
    #[inline]
    pub const fn total_balance_delta(&self) -> i64 {
        let total_before = self.available_before + self.frozen_before;
        let total_after = self.available_after + self.frozen_after;
        total_after - total_before
    }

    /// 检查是否有关联订单
    #[inline]
    pub const fn has_order(&self) -> bool {
        self.order_id != u64::MAX
    }

    /// 获取变更类型
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

    /// 获取变更原因
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

    /// 从BalanceChange转换
    #[inline]
    pub fn from_balance_change(change: &BalanceChange) -> Self {
        Self {
            sequence_id: change.sequence_id,
            account_id: change.account_id.0,
            asset_id: change.asset_id.as_u32() as u64,
            change_type: change.change_type as u8,
            reason: change.reason as u8,
            _padding1: [0; 6],
            amount: change.amount.raw(),
            available_before: change.available_before.raw(),
            available_after: change.available_after.raw(),
            frozen_before: change.frozen_before.raw(),
            frozen_after: change.frozen_after.raw(),
            order_id: change.order_id.unwrap_or(u64::MAX),
            timestamp: change.timestamp.0,
            balance_version: change.balance_version,
            _reserved: [0; 3],
        }
    }

    /// 转换为BalanceChange
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
            amount: Quantity::from_raw(self.amount),
            available_before: Quantity::from_raw(self.available_before),
            available_after: Quantity::from_raw(self.available_after),
            frozen_before: Quantity::from_raw(self.frozen_before),
            frozen_after: Quantity::from_raw(self.frozen_after),
            order_id: if self.order_id == u64::MAX {
                None
            } else {
                Some(self.order_id)
            },
            timestamp: Timestamp(self.timestamp),
            balance_version: self.balance_version,
        })
    }

    /// 零拷贝：从字节数组创建
    ///
    /// # Safety
    /// 调用者必须确保字节数组是有效的BalanceChangePod表示
    #[inline]
    pub unsafe fn from_bytes(bytes: &[u8; 128]) -> &Self {
        &*(bytes.as_ptr() as *const Self)
    }

    /// 零拷贝：转换为字节数组
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 128] {
        unsafe { &*(self as *const Self as *const [u8; 128]) }
    }

    /// 零拷贝：从字节切片批量创建
    ///
    /// # Safety
    /// 调用者必须确保：
    /// - 字节切片长度是128的倍数
    /// - 字节切片对齐到128字节
    /// - 字节内容是有效的BalanceChangePod表示
    #[inline]
    pub unsafe fn from_bytes_slice(bytes: &[u8]) -> &[Self] {
        assert_eq!(bytes.len() % 128, 0, "Bytes length must be multiple of 128");
        std::slice::from_raw_parts(bytes.as_ptr() as *const Self, bytes.len() / 128)
    }

    /// 零拷贝：转换为字节切片
    #[inline]
    pub fn as_bytes_slice(slice: &[Self]) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * 128)
        }
    }
}

// 静态断言：确保POD类型大小和对齐
const _: () = assert!(std::mem::size_of::<BalanceChangePod>() == 128);
const _: () = assert!(std::mem::align_of::<BalanceChangePod>() == 128);

/// Balance变更日志（POD版本，SoA结构）
///
/// 用于批量处理和SIMD优化
#[derive(Debug, Clone)]
pub struct BalanceChangePodLog {
    /// 序列号数组
    pub sequence_ids: Vec<u64>,
    /// 账户ID数组
    pub account_ids: Vec<u64>,
    /// 资产ID数组
    pub asset_ids: Vec<u64>,
    /// 变更类型数组
    pub change_types: Vec<u8>,
    /// 变更原因数组
    pub reasons: Vec<u8>,
    /// 变更金额数组
    pub amounts: Vec<i64>,
    /// 变更前可用余额数组
    pub available_befores: Vec<i64>,
    /// 变更后可用余额数组
    pub available_afters: Vec<i64>,
    /// 变更前冻结余额数组
    pub frozen_befores: Vec<i64>,
    /// 变更后冻结余额数组
    pub frozen_afters: Vec<i64>,
    /// 订单ID数组
    pub order_ids: Vec<u64>,
    /// 时间戳数组
    pub timestamps: Vec<u64>,
    /// 版本号数组
    pub balance_versions: Vec<u64>,
}

impl BalanceChangePodLog {
    /// 创建空日志
    pub fn new() -> Self {
        Self {
            sequence_ids: Vec::new(),
            account_ids: Vec::new(),
            asset_ids: Vec::new(),
            change_types: Vec::new(),
            reasons: Vec::new(),
            amounts: Vec::new(),
            available_befores: Vec::new(),
            available_afters: Vec::new(),
            frozen_befores: Vec::new(),
            frozen_afters: Vec::new(),
            order_ids: Vec::new(),
            timestamps: Vec::new(),
            balance_versions: Vec::new(),
        }
    }

    /// 预分配容量
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sequence_ids: Vec::with_capacity(capacity),
            account_ids: Vec::with_capacity(capacity),
            asset_ids: Vec::with_capacity(capacity),
            change_types: Vec::with_capacity(capacity),
            reasons: Vec::with_capacity(capacity),
            amounts: Vec::with_capacity(capacity),
            available_befores: Vec::with_capacity(capacity),
            available_afters: Vec::with_capacity(capacity),
            frozen_befores: Vec::with_capacity(capacity),
            frozen_afters: Vec::with_capacity(capacity),
            order_ids: Vec::with_capacity(capacity),
            timestamps: Vec::with_capacity(capacity),
            balance_versions: Vec::with_capacity(capacity),
        }
    }

    /// 添加变更记录
    pub fn push(&mut self, change: &BalanceChangePod) {
        self.sequence_ids.push(change.sequence_id);
        self.account_ids.push(change.account_id);
        self.asset_ids.push(change.asset_id);
        self.change_types.push(change.change_type);
        self.reasons.push(change.reason);
        self.amounts.push(change.amount);
        self.available_befores.push(change.available_before);
        self.available_afters.push(change.available_after);
        self.frozen_befores.push(change.frozen_before);
        self.frozen_afters.push(change.frozen_after);
        self.order_ids.push(change.order_id);
        self.timestamps.push(change.timestamp);
        self.balance_versions.push(change.balance_version);
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
    pub fn filter_by_account(&self, account_id: u64) -> Vec<usize> {
        self.account_ids
            .iter()
            .enumerate()
            .filter_map(|(idx, &id)| if id == account_id { Some(idx) } else { None })
            .collect()
    }

    /// 按时间范围过滤
    pub fn filter_by_time_range(&self, start: u64, end: u64) -> Vec<usize> {
        self.timestamps
            .iter()
            .enumerate()
            .filter_map(|(idx, &ts)| {
                if ts >= start && ts <= end {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }

    /// 按变更类型过滤
    pub fn filter_by_type(&self, change_type: u8) -> Vec<usize> {
        self.change_types
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
    use super::*;

    #[test]
    fn test_pod_size_and_alignment() {
        assert_eq!(std::mem::size_of::<BalanceChangePod>(), 128);
        assert_eq!(std::mem::align_of::<BalanceChangePod>(), 128);
    }

    #[test]
    fn test_pod_deposit() {
        let change = BalanceChangePod::deposit(
            1,
            100,
            1,
            1000_00000000,
            500_00000000,
            1234567890,
            1,
        );

        assert_eq!(change.sequence_id, 1);
        assert_eq!(change.account_id, 100);
        assert_eq!(change.change_type, BalanceChangeType::Deposit as u8);
        assert_eq!(change.available_after, 1500_00000000);
        assert!(change.validate());
    }

    #[test]
    fn test_pod_freeze() {
        let change = BalanceChangePod::freeze(
            2,
            100,
            1,
            100_00000000,
            500_00000000,
            0,
            1,
            1234567890,
            2,
        );

        assert_eq!(change.change_type, BalanceChangeType::Freeze as u8);
        assert_eq!(change.available_after, 400_00000000);
        assert_eq!(change.frozen_after, 100_00000000);
        assert!(change.validate());
        assert!(change.has_order());
    }

    #[test]
    fn test_pod_total_balance_delta() {
        // 充值：总余额增加
        let deposit = BalanceChangePod::deposit(
            1,
            100,
            1,
            1000_00000000,
            500_00000000,
            1234567890,
            1,
        );
        assert_eq!(deposit.total_balance_delta(), 1000_00000000);

        // 冻结：总余额不变
        let freeze = BalanceChangePod::freeze(
            2,
            100,
            1,
            100_00000000,
            500_00000000,
            0,
            1,
            1234567890,
            2,
        );
        assert_eq!(freeze.total_balance_delta(), 0);

        // 成交：总余额减少
        let trade = BalanceChangePod::trade(
            3,
            100,
            1,
            100_00000000,
            400_00000000,
            100_00000000,
            1,
            1234567890,
            3,
        );
        assert_eq!(trade.total_balance_delta(), -100_00000000);
    }

    #[test]
    fn test_pod_zero_copy() {
        let change = BalanceChangePod::deposit(
            1,
            100,
            1,
            1000_00000000,
            500_00000000,
            1234567890,
            1,
        );

        // 转换为字节数组
        let bytes = change.as_bytes();
        assert_eq!(bytes.len(), 128);

        // 从字节数组恢复
        let recovered = unsafe { BalanceChangePod::from_bytes(bytes) };
        assert_eq!(*recovered, change);
    }

    #[test]
    fn test_pod_log() {
        let mut log = BalanceChangePodLog::with_capacity(10);

        let change1 = BalanceChangePod::deposit(
            1,
            100,
            1,
            1000_00000000,
            0,
            1234567890,
            1,
        );

        let change2 = BalanceChangePod::freeze(
            2,
            100,
            1,
            100_00000000,
            1000_00000000,
            0,
            1,
            1234567891,
            2,
        );

        log.push(&change1);
        log.push(&change2);

        assert_eq!(log.len(), 2);
        assert_eq!(log.filter_by_account(100).len(), 2);
        assert_eq!(log.filter_by_type(BalanceChangeType::Deposit as u8).len(), 1);
    }
}
