//! 余额实体定义

use std::fmt;

use entity_derive::Entity;

use crate::account::balance_change::BalanceChange;
use crate::account::error::BalanceError;
use crate::{AccountId, AssetId, OrderId, Price, Quantity, Timestamp};

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
        write!(f, "{}:{}", self.account_id.0, u32::from(self.asset_id))
    }
}

impl Default for BalanceId {
    fn default() -> Self {
        Self { account_id: AccountId(0), asset_id: AssetId::default() }
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
///
/// 变更追溯：
/// - 每次余额变更都会生成BalanceChange事件
/// - 通过sequence_id保证全局顺序
/// - 支持审计和回溯
#[derive(Debug, Clone, Entity)]
#[repr(align(64))]
//todo 用基础类型生成Balance，simd友好

pub struct Balance {
    //todo add     pub trader_id; key(trader_id,asset_id)
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
    pub updated_at: Timestamp,
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
            updated_at: now,
        }
    }

    /// 创建带初始余额的记录
    pub fn with_available(
        account_id: AccountId,
        asset_id: AssetId,
        available: i64,
        now: Timestamp,
    ) -> Self {
        Self {
            id: BalanceId::new(account_id, asset_id),
            account_id,
            asset_id,
            available: Quantity::from_raw(available),
            frozen: Quantity::default(),
            version: 0,
            updated_at: now,
        }
    }

    /// 检查是否有足够的冻结余额
    #[inline]
    pub fn has_frozen(&self, amount: Quantity) -> bool {
        self.frozen >= amount
    }

    #[inline]
    pub fn add_balance(&mut self, amount: Quantity, now: Timestamp) {
        self.available = self.available + amount;
        self.version += 1;
        self.updated_at = now;
    }

    /// 添加余额并生成变更事件
    #[inline]
    pub fn add_balance_with_change(
        &mut self,
        amount: Quantity,
        now: Timestamp,
        sequence_id: u64,
    ) -> BalanceChange {
        let available_before = self.available;
        self.available = self.available + amount;
        self.version += 1;
        self.updated_at = now;

        BalanceChange::deposit(
            sequence_id,
            self.account_id,
            self.asset_id,
            amount,
            available_before,
            now,
            self.version,
        )
    }

    #[inline]
    /// 冻结余额（可用 → 冻结）
    ///
    /// # 错误
    /// 当可用余额不足时返回 `BalanceError::InsufficientAvailable`
    pub fn frozen(&mut self, amount: Quantity, now: Timestamp) -> Result<(), BalanceError> {
        let available_raw = self.available.raw();
        let amount_raw = amount.raw();

        if available_raw < amount_raw {
            return Err(BalanceError::InsufficientAvailable {
                required: amount_raw,
                available: available_raw,
            });
        }

        self.available = self.available - amount;
        self.frozen = self.frozen + amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 冻结余额并生成变更事件
    #[inline]
    pub fn frozen_with_change(
        &mut self,
        amount: Quantity,
        order_id: OrderId,
        now: Timestamp,
        sequence_id: u64,
    ) -> Result<BalanceChange, BalanceError> {
        let available_before = self.available;
        let frozen_before = self.frozen;

        let available_raw = self.available.raw();
        let amount_raw = amount.raw();

        if available_raw < amount_raw {
            return Err(BalanceError::InsufficientAvailable {
                required: amount_raw,
                available: available_raw,
            });
        }

        self.available = self.available - amount;
        self.frozen = self.frozen + amount;
        self.version += 1;
        self.updated_at = now;

        Ok(BalanceChange::freeze(
            sequence_id,
            self.account_id,
            self.asset_id,
            amount,
            available_before,
            frozen_before,
            order_id,
            now,
            self.version,
        ))
    }

    #[inline]
    /// 从冻结余额中扣款（冻结 → 扣除）
    ///
    /// # 错误
    /// 当冻结余额不足时返回 `BalanceError::InsufficientFrozen`
    pub fn frozen2pay(&mut self, amount: Quantity, now: Timestamp) -> Result<(), BalanceError> {
        let frozen_raw = self.frozen.raw();
        let amount_raw = amount.raw();

        if frozen_raw < amount_raw {
            return Err(BalanceError::InsufficientFrozen {
                required: amount_raw,
                frozen: frozen_raw,
            });
        }

        self.frozen = self.frozen - amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 从冻结余额中扣款并生成变更事件
    #[inline]
    pub fn frozen2pay_with_change(
        &mut self,
        amount: Quantity,
        order_id: OrderId,
        now: Timestamp,
        sequence_id: u64,
    ) -> Result<BalanceChange, BalanceError> {
        let available_before = self.available;
        let frozen_before = self.frozen;

        let frozen_raw = self.frozen.raw();
        let amount_raw = amount.raw();

        if frozen_raw < amount_raw {
            return Err(BalanceError::InsufficientFrozen {
                required: amount_raw,
                frozen: frozen_raw,
            });
        }

        self.frozen = self.frozen - amount;
        self.version += 1;
        self.updated_at = now;

        Ok(BalanceChange::trade(
            sequence_id,
            self.account_id,
            self.asset_id,
            amount,
            available_before,
            frozen_before,
            order_id,
            now,
            self.version,
        ))
    }

    #[inline]
    /// 解冻余额（冻结 → 可用）
    ///
    /// # 错误
    /// 当冻结余额不足时返回 `BalanceError::InsufficientFrozen`
    pub fn un_frozen(&mut self, amount: Quantity, now: Timestamp) -> Result<(), BalanceError> {
        let frozen_raw = self.frozen.raw();
        let amount_raw = amount.raw();

        if frozen_raw < amount_raw {
            return Err(BalanceError::InsufficientFrozen {
                required: amount_raw,
                frozen: frozen_raw,
            });
        }

        self.available = self.available + amount;
        self.frozen = self.frozen - amount;
        self.version += 1;
        self.updated_at = now;
        Ok(())
    }

    /// 解冻余额并生成变更事件
    #[inline]
    pub fn un_frozen_with_change(
        &mut self,
        amount: Quantity,
        order_id: OrderId,
        now: Timestamp,
        sequence_id: u64,
    ) -> Result<BalanceChange, BalanceError> {
        let available_before = self.available;
        let frozen_before = self.frozen;

        let frozen_raw = self.frozen.raw();
        let amount_raw = amount.raw();

        if frozen_raw < amount_raw {
            return Err(BalanceError::InsufficientFrozen {
                required: amount_raw,
                frozen: frozen_raw,
            });
        }

        self.available = self.available + amount;
        self.frozen = self.frozen - amount;
        self.version += 1;
        self.updated_at = now;

        Ok(BalanceChange::unfreeze(
            sequence_id,
            self.account_id,
            self.asset_id,
            amount,
            available_before,
            frozen_before,
            order_id,
            now,
            self.version,
        ))
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
    SettlePnl(Price),
}
