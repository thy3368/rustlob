//! 余额仓储接口定义
//!
//! 遵循 Clean Architecture：Repository 只负责纯粹的 CRUD 操作
//! 业务逻辑（freeze/unfreeze/credit/debit）在 Service 层实现

use crate::domain::entity::{AccountId, AssetId, Timestamp};
use crate::domain::position::PositionInfo;

/// 余额仓储接口
///
/// 提供余额的持久化操作（纯 CRUD）
pub trait PositionRepo: Send + Sync {
    /// 获取余额
    fn get(&self, account_id: AccountId, asset_id: AssetId) -> Option<&PositionInfo>;

    /// 获取余额（可变引用）
    fn get_mut(&mut self, account_id: AccountId, asset_id: AssetId) -> Option<&mut PositionInfo>;

    /// 获取或创建余额
    fn get_or_create(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        now: Timestamp,
    ) -> &mut PositionInfo;

    /// 保存余额
    fn save(&mut self, balance: PositionInfo);

    /// 检查余额是否存在
    fn exists(&self, account_id: AccountId, asset_id: AssetId) -> bool;

    /// 获取账户的所有余额
    fn get_all_by_account(&self, account_id: AccountId) -> Vec<&PositionInfo>;
}
