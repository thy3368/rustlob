//! 账户实体定义

use crate::{AccountId, Timestamp, UserId};

/// 交易账户
#[derive(Debug, Clone)]
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
    pub updated_at: Timestamp
}

impl Account {
    /// 创建新账户
    pub fn new(id: AccountId, user_id: UserId, account_type: AccountType, now: Timestamp) -> Self {
        Self {
            id,
            user_id,
            account_type,
            status: AccountStatus::Active,
            created_at: now,
            updated_at: now
        }
    }

    /// 检查账户是否可用于交易
    #[inline]
    pub fn is_active(&self) -> bool { matches!(self.status, AccountStatus::Active) }

    /// 冻结账户
    pub fn freeze(&mut self, now: Timestamp) {
        self.status = AccountStatus::Frozen;
        self.updated_at = now;
    }

    /// 解冻账户
    pub fn unfreeze(&mut self, now: Timestamp) {
        self.status = AccountStatus::Active;
        self.updated_at = now;
    }

    /// 关闭账户
    pub fn close(&mut self, now: Timestamp) {
        self.status = AccountStatus::Closed;
        self.updated_at = now;
    }
}

/// 账户类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccountType {
    /// 现货账户
    Spot = 0,
    /// 合约账户（逐仓）
    PerpIsolated = 1,
    /// 合约账户（全仓）
    PerpCross = 2,
    /// 资金账户
    Funding = 3
}

/// 账户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccountStatus {
    /// 正常
    Active = 0,
    /// 冻结（禁止交易）
    Frozen = 1,
    /// 注销
    Closed = 2
}
