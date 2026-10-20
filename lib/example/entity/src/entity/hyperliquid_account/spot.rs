use std::collections::BTreeMap;

use crate::entity::SpotBalance;

/// 子账户在现货清算域上的余额状态快照。
///
/// 该对象属于 `Snapshot` 语义，主要支持余额查询与可用资金计算。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotClearinghouseState {
    account_id: u64,
    balances: BTreeMap<u64, SpotBalance>,
}

impl SpotClearinghouseState {
    /// 从已校验事实装配子账户现货清算状态，映射键为资产 ID。
    pub fn new(account_id: u64, balances: BTreeMap<u64, SpotBalance>) -> Self {
        Self { account_id, balances }
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &u64 {
        &self.account_id
    }

    /// 返回资产 ID 到余额快照的映射。
    pub fn balances(&self) -> &BTreeMap<u64, SpotBalance> {
        &self.balances
    }

    /// 按资产 ID 查找余额快照。
    pub fn balance_of(&self, asset_id: u64) -> Option<&SpotBalance> {
        self.balances.get(&asset_id)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn balance(asset_id: u64, available: u64, frozen: u64) -> SpotBalance {
        SpotBalance::new(1, 7, asset_id, available, frozen, 1)
    }

    #[test]
    fn queries_balances_by_asset_id_and_returns_zero_for_missing_assets() {
        let mut balances = BTreeMap::new();
        balances.insert(42, balance(42, 100, 25));
        let state = SpotClearinghouseState::new(7, balances);

        assert_eq!(state.balance_of(42).map(|balance| balance.asset_id), Some(42));
        assert_eq!(state.total_balance_of(42), 125);
        assert_eq!(state.available_balance_of(42), 100);
        assert!(state.balance_of(99).is_none());
        assert_eq!(state.total_balance_of(99), 0);
        assert_eq!(state.available_balance_of(99), 0);
        assert_eq!(state.balances().keys().copied().collect::<Vec<_>>(), vec![42]);
    }
}
