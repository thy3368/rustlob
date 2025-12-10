//! 账户仓储接口定义

use crate::domain::entity::{Account, AccountId, AccountStatus, BalanceError};

/// 账户仓储接口
///
/// 提供账户的持久化操作
pub trait AccountRepo: Send + Sync {
    /// 获取账户
    fn get(&self, account_id: AccountId) -> Option<&Account>;

    /// 获取账户（可变引用）
    fn get_mut(&mut self, account_id: AccountId) -> Option<&mut Account>;

    /// 保存账户
    fn save(&mut self, account: Account);

    /// 检查账户是否存在
    fn exists(&self, account_id: AccountId) -> bool;

    /// 获取账户状态
    fn get_status(&self, account_id: AccountId) -> Option<AccountStatus>;

    /// 检查账户状态是否可用
    fn check_status(&self, account_id: AccountId) -> Result<(), BalanceError> {
        match self.get(account_id) {
            Some(account) => match account.status {
                AccountStatus::Active => Ok(()),
                AccountStatus::Frozen => Err(BalanceError::AccountFrozen { account_id }),
                AccountStatus::Closed => Err(BalanceError::AccountClosed { account_id }),
            },
            None => Err(BalanceError::AccountNotFound { account_id }),
        }
    }
}
