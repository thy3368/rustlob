//! 余额仓储接口定义

use crate::domain::entity::{AccountId, AssetId, Balance, BalanceError, Timestamp};

/// 余额仓储接口
///
/// 提供余额的持久化操作
pub trait BalanceRepository: Send + Sync {
    /// 获取余额
    fn get(&self, account_id: AccountId, asset_id: AssetId) -> Option<&Balance>;

    /// 获取余额（可变引用）
    fn get_mut(&mut self, account_id: AccountId, asset_id: AssetId) -> Option<&mut Balance>;

    /// 获取或创建余额
    fn get_or_create(&mut self, account_id: AccountId, asset_id: AssetId, now: Timestamp)
        -> &mut Balance;

    /// 保存余额
    fn save(&mut self, balance: Balance);

    /// 检查余额是否存在
    fn exists(&self, account_id: AccountId, asset_id: AssetId) -> bool;

    /// 获取账户的所有余额
    fn get_all_by_account(&self, account_id: AccountId) -> Vec<&Balance>;

    /// 冻结操作（原子：检查+冻结）
    fn freeze(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError>;

    /// 解冻操作
    fn unfreeze(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError>;

    /// 入账操作
    fn credit(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError>;

    /// 扣款操作（从可用余额）
    fn debit(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError>;

    /// 扣款操作（从冻结余额）
    fn debit_frozen(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError>;

    /// 转账操作（原子）
    fn transfer(
        &mut self,
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<(), BalanceError>;

    /// 结算盈亏
    fn settle_pnl(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        pnl: i64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError>;
}
