use std::collections::BTreeMap;

use crate::entity::SpotBalance;

/// 子账户在现货清算域上的余额状态快照。
///
/// 该对象属于 `Snapshot` 语义，主要支持余额查询与可用资金计算。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotClearinghouseState {
    party_id: u64,
    account_id: u64,
    balances: BTreeMap<u64, SpotBalance>, // asset → 余额
}

impl SpotClearinghouseState {
    /// 从已校验事实装配子账户现货清算状态。
    pub fn new(account_id: u64, balances: Vec<SpotBalance>) -> Self {
        Self { account_id, balances }
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &u64 {
        &self.account_id
    }

    /// 返回当前余额快照集合。
    pub fn balances(&self) -> &[SpotBalance] {
        &self.balances
    }

    /// 按资产查找余额快照。
    pub fn balance_of(&self, asset_id: u64) -> Option<&SpotBalance> {
        self.balances.iter().find(|balance| balance.is_asset(asset_id.as_str()))
    }

    /// 返回指定资产的总余额；若不存在该资产则返回零。
    pub fn total_balance_of(&self, asset_id: u64) -> u64 {
        self.balance_of(asset_id).and_then(SpotBalance::total).unwrap_or_default()
    }

    /// 返回指定资产的可用余额；若不存在该资产则返回零。
    pub fn available_balance_of(&self, asset_id: u64) -> u64 {
        self.balance_of(asset_id).map(|balance| balance.available).unwrap_or_default()
    }
}
