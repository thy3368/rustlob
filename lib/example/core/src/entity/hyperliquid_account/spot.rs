use super::{AccountId, AssetId};
use crate::entity::Balance;

/// 子账户在现货清算域上的余额状态快照。
///
/// 该对象属于 `Snapshot` 语义，主要支持余额查询与可用资金计算。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotClearinghouseState {
    account_id: AccountId,
    balances: Vec<Balance>,
}

impl SpotClearinghouseState {
    /// 从已校验事实装配子账户现货清算状态。
    pub fn new(account_id: AccountId, balances: Vec<Balance>) -> Self {
        Self { account_id, balances }
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回当前余额快照集合。
    pub fn balances(&self) -> &[Balance] {
        &self.balances
    }

    /// 按资产查找余额快照。
    pub fn balance_of(&self, asset_id: &AssetId) -> Option<&Balance> {
        self.balances.iter().find(|balance| balance.is_asset(asset_id.as_str()))
    }

    /// 返回指定资产的总余额；若不存在该资产则返回零。
    pub fn total_balance_of(&self, asset_id: &AssetId) -> u64 {
        self.balance_of(asset_id).and_then(Balance::total).unwrap_or_default()
    }

    /// 返回指定资产的可用余额；若不存在该资产则返回零。
    pub fn available_balance_of(&self, asset_id: &AssetId) -> u64 {
        self.balance_of(asset_id).map(|balance| balance.available).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_and_available_balance_follow_balance_fields() {
        let state = SpotClearinghouseState::new(
            AccountId::from("sub-1"),
            vec![Balance::new_with_snapshot_facts(
                "sub-1".to_string(),
                "USDC".to_string(),
                7,
                3,
                None,
                None,
                1,
            )],
        );

        assert_eq!(state.available_balance_of(&AssetId::from("USDC")), 7);
        assert_eq!(state.total_balance_of(&AssetId::from("USDC")), 10);
    }

    #[test]
    fn missing_asset_balance_returns_zero() {
        let state = SpotClearinghouseState::new(
            AccountId::from("sub-1"),
            vec![Balance::new_with_snapshot_facts(
                "sub-1".to_string(),
                "USDC".to_string(),
                7,
                3,
                None,
                None,
                1,
            )],
        );

        assert_eq!(state.total_balance_of(&AssetId::from("BTC")), 0);
        assert_eq!(state.available_balance_of(&AssetId::from("BTC")), 0);
        assert!(state.balance_of(&AssetId::from("BTC")).is_none());
    }

    #[test]
    fn balance_preserves_optional_snapshot_facts() {
        let balance = Balance::new_with_snapshot_facts(
            "sub-1".to_string(),
            "HYPE".to_string(),
            4,
            1,
            Some(42),
            Some("0xspot".to_owned()),
            1,
        );

        assert_eq!(balance.asset_id, "HYPE");
        assert_eq!(balance.total(), Some(5));
        assert_eq!(balance.frozen, 1);
        assert_eq!(balance.entry_notional(), Some(42));
        assert_eq!(balance.identifier(), Some("0xspot"));
    }
}
