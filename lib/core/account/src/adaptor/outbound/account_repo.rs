//! 内存实现的账户仓储

use std::collections::HashMap;

use crate::domain::{
    entity::{Account, AccountId, AccountStatus},
    repo::AccountRepo
};

/// 内存账户仓储实现
pub struct MemoryAccountRepo {
    /// 账户缓存 (account_id -> Account)
    accounts: HashMap<AccountId, Account>
}

impl MemoryAccountRepo {
    /// 创建新的内存账户仓储
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new()
        }
    }
}

impl Default for MemoryAccountRepo {
    fn default() -> Self { Self::new() }
}

impl AccountRepo for MemoryAccountRepo {
    fn get(&self, account_id: AccountId) -> Option<&Account> { self.accounts.get(&account_id) }

    fn get_mut(&mut self, account_id: AccountId) -> Option<&mut Account> { self.accounts.get_mut(&account_id) }

    fn save(&mut self, account: Account) { self.accounts.insert(account.id, account); }

    fn exists(&self, account_id: AccountId) -> bool { self.accounts.contains_key(&account_id) }

    fn get_status(&self, account_id: AccountId) -> Option<AccountStatus> {
        self.accounts.get(&account_id).map(|a| a.status)
    }
}
