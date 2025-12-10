//! 余额仓储接口定义
//!
//! 遵循 Clean Architecture：Repository 只负责纯粹的 CRUD 操作
//! 业务逻辑（freeze/unfreeze/credit/debit）在 Service 层实现

use crate::domain::entity::{AccountId, AssetId, Balance, Timestamp};

/// 余额仓储接口
///
/// 提供余额的持久化操作（纯 CRUD）
pub trait BalanceRepo: Send + Sync {
    /// 获取余额
    fn get(&self, account_id: AccountId, asset_id: AssetId) -> Option<&Balance>;

    /// 获取余额（可变引用）
    fn get_mut(&mut self, account_id: AccountId, asset_id: AssetId) -> Option<&mut Balance>;

    /// 获取或创建余额
    fn get_or_create(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        now: Timestamp,
    ) -> &mut Balance;

    /// 保存余额
    fn save(&mut self, balance: Balance);

    /// 检查余额是否存在
    fn exists(&self, account_id: AccountId, asset_id: AssetId) -> bool;

    /// 获取账户的所有余额
    fn get_all_by_account(&self, account_id: AccountId) -> Vec<&Balance>;
}
