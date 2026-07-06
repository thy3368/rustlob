use decimal::Decimal;

use super::{AccountId, AssetId};

/// 子账户在现货清算域上的单资产余额快照。
///
/// 该值对象属于 `Snapshot` 语义，不表达挂单来源或 adapter wire 字段名，
/// 只保留 core 需要承接的现货余额核心事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotBalance {
    /// core 中稳定的现货资产标识，不直接等同于上游接口里的 `coin` 原字段名。
    asset_id: AssetId,
    /// 当前资产总余额。
    total: Decimal,
    /// 当前资产被占用的余额。
    hold: Decimal,
    /// 当前资产入场名义值，对应上游 `entry_ntl` 语义；上游可缺省。
    entry_notional: Option<Decimal>,
    /// 上游附带的标识信息，对应 `identifier` 语义；上游可缺省。
    identifier: Option<String>,
}

impl SpotBalance {
    /// 从已校验事实装配单资产余额快照。
    ///
    /// 该构造器只负责装配已被 adapter 或上层流程确认的业务事实，
    /// 不承担解析、字段映射或业务校验职责。
    pub fn new(
        asset_id: AssetId,
        total: Decimal,
        hold: Decimal,
        entry_notional: Option<Decimal>,
        identifier: Option<String>,
    ) -> Self {
        Self { asset_id, total, hold, entry_notional, identifier }
    }

    /// 返回资产标识。
    pub fn asset_id(&self) -> &AssetId {
        &self.asset_id
    }

    /// 返回该资产总余额。
    pub fn total(&self) -> Decimal {
        self.total
    }

    /// 返回该资产当前占用余额。
    pub fn hold(&self) -> Decimal {
        self.hold
    }

    /// 返回可用余额，即 `total - hold`。
    pub fn available(&self) -> Decimal {
        self.total - self.hold
    }

    /// 返回该资产的入场名义值；若上游未提供则返回 `None`。
    pub fn entry_notional(&self) -> Option<Decimal> {
        self.entry_notional
    }

    /// 返回上游附带的标识信息；若上游未提供则返回 `None`。
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }
}

/// 子账户在现货清算域上的余额状态快照。
///
/// 该对象属于 `Snapshot` 语义，主要支持余额查询与可用资金计算。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotClearinghouseState {
    account_id: AccountId,
    balances: Vec<SpotBalance>,
}

impl SpotClearinghouseState {
    /// 从已校验事实装配子账户现货清算状态。
    pub fn new(account_id: AccountId, balances: Vec<SpotBalance>) -> Self {
        Self { account_id, balances }
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回当前余额快照集合。
    pub fn balances(&self) -> &[SpotBalance] {
        &self.balances
    }

    /// 按资产查找余额快照。
    pub fn balance_of(&self, asset_id: &AssetId) -> Option<&SpotBalance> {
        self.balances.iter().find(|balance| balance.asset_id() == asset_id)
    }

    /// 返回指定资产的总余额；若不存在该资产则返回零。
    pub fn total_balance_of(&self, asset_id: &AssetId) -> Decimal {
        self.balance_of(asset_id).map(SpotBalance::total).unwrap_or_default()
    }

    /// 返回指定资产的可用余额；若不存在该资产则返回零。
    pub fn available_balance_of(&self, asset_id: &AssetId) -> Decimal {
        self.balance_of(asset_id).map(SpotBalance::available).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use decimal::Decimal;

    use super::*;

    fn dec(units: i64) -> Decimal {
        Decimal::from_raw(units * 100_000_000)
    }

    #[test]
    fn spot_balance_available_returns_total_minus_hold() {
        let balance = SpotBalance::new(AssetId::from("USDC"), dec(10), dec(3), None, None);

        assert_eq!(balance.available(), dec(7));
    }

    #[test]
    fn missing_asset_balance_returns_zero() {
        let state = SpotClearinghouseState::new(
            AccountId::from("sub-1"),
            vec![SpotBalance::new(AssetId::from("USDC"), dec(10), dec(3), None, None)],
        );

        assert_eq!(state.total_balance_of(&AssetId::from("BTC")), Decimal::default());
        assert_eq!(state.available_balance_of(&AssetId::from("BTC")), Decimal::default());
        assert!(state.balance_of(&AssetId::from("BTC")).is_none());
    }

    #[test]
    fn spot_balance_constructor_preserves_optional_snapshot_facts() {
        let balance = SpotBalance::new(
            AssetId::from("HYPE"),
            dec(5),
            dec(1),
            Some(dec(42)),
            Some("0xspot".to_owned()),
        );

        assert_eq!(balance.asset_id(), &AssetId::from("HYPE"));
        assert_eq!(balance.total(), dec(5));
        assert_eq!(balance.hold(), dec(1));
        assert_eq!(balance.entry_notional(), Some(dec(42)));
        assert_eq!(balance.identifier(), Some("0xspot"));
    }
}
