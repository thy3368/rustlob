//! 内存实现的余额仓储
//!
//! 遵循 Clean Architecture：Repository 只负责纯粹的 CRUD 操作

use std::collections::HashMap;

use crate::domain::{
    entity::{AccountId, AssetId, Balance, Timestamp},
    repo::BalanceRepo
};

/// 内存余额仓储实现
pub struct MemoryBalanceRepo {
    /// 余额缓存 ((account_id, asset_id) -> Balance)
    balances: HashMap<(AccountId, AssetId), Balance>,
    /// 时间戳生成器
    timestamp_fn: fn() -> Timestamp
}

impl MemoryBalanceRepo {
    /// 创建新的内存余额仓储
    pub fn new(timestamp_fn: fn() -> Timestamp) -> Self {
        Self {
            balances: HashMap::new(),
            timestamp_fn
        }
    }

    /// 使用默认时间戳函数创建
    pub fn with_default_timestamp() -> Self {
        Self::new(|| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64)
    }

    /// 获取当前时间戳
    #[inline]
    fn now(&self) -> Timestamp { (self.timestamp_fn)() }

    /// 设置余额（测试用）
    pub fn set_balance(&mut self, account_id: AccountId, asset_id: AssetId, available: u64) {
        let now = self.now();
        self.balances.insert((account_id, asset_id), Balance::with_available(account_id, asset_id, available, now));
    }
}

impl BalanceRepo for MemoryBalanceRepo {
    fn get(&self, account_id: AccountId, asset_id: AssetId) -> Option<&Balance> {
        self.balances.get(&(account_id, asset_id))
    }

    fn get_mut(&mut self, account_id: AccountId, asset_id: AssetId) -> Option<&mut Balance> {
        self.balances.get_mut(&(account_id, asset_id))
    }

    fn get_or_create(&mut self, account_id: AccountId, asset_id: AssetId, now: Timestamp) -> &mut Balance {
        self.balances.entry((account_id, asset_id)).or_insert_with(|| Balance::new(account_id, asset_id, now))
    }

    fn save(&mut self, balance: Balance) { self.balances.insert((balance.account_id, balance.asset_id), balance); }

    fn exists(&self, account_id: AccountId, asset_id: AssetId) -> bool {
        self.balances.contains_key(&(account_id, asset_id))
    }

    fn get_all_by_account(&self, account_id: AccountId) -> Vec<&Balance> {
        self.balances.iter().filter(|((acc_id, _), _)| *acc_id == account_id).map(|(_, balance)| balance).collect()
    }
}
