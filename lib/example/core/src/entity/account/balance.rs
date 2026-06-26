use common_entity::{Entity, EntityError, EntityFieldChange};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod balance_scenarios;

const BALANCE_ENTITY_TYPE: u8 = 7;

/// 某个账户在单一资产上的余额快照。
///
/// 一个账户可以拥有多条 `Balance`：例如 BTC、USDT、USDC 各一条。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Balance {
    /// 账户 ID。
    pub account_id: String,
    /// 资产 ID，例如 `BTC`、`USDT` 或 Hyperliquid token id 字符串。
    pub asset_id: String,
    /// 可用余额。
    pub available: u64,
    /// 冻结或占用余额。
    pub frozen: u64,
    /// 当前余额实体版本。
    pub version: u64,
}

impl Balance {
    /// 从已经校验过的业务事实或回放事件构造资产余额。
    pub fn new(
        account_id: String,
        asset_id: String,
        available: u64,
        frozen: u64,
        version: u64,
    ) -> Self {
        Self { account_id, asset_id, available, frozen, version }
    }

    /// 返回余额是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回余额是否对应指定资产。
    pub fn is_asset(&self, asset_id: &str) -> bool {
        self.asset_id == asset_id
    }

    /// 返回两个余额是否表达同一条业务快照。
    pub fn matches_business_snapshot(&self, other: &Self) -> bool {
        self.account_id == other.account_id
            && self.asset_id == other.asset_id
            && self.available == other.available
            && self.frozen == other.frozen
    }

    /// 返回是否能冻结指定数量。
    pub fn can_reserve(&self, amount: u64) -> bool {
        self.available >= amount
    }

    /// 返回冻结后的 `(available, frozen)`。
    pub fn reserve_after(&self, amount: u64) -> Option<(u64, u64)> {
        let next_available = self.available.checked_sub(amount)?;
        let next_frozen = self.frozen.checked_add(amount)?;
        Some((next_available, next_frozen))
    }

    /// 返回释放冻结后的 `(available, frozen)`。
    pub fn release_after(&self, amount: u64) -> Option<(u64, u64)> {
        let next_available = self.available.checked_add(amount)?;
        let next_frozen = self.frozen.checked_sub(amount)?;
        Some((next_available, next_frozen))
    }

    /// 返回交割收入后的可用余额。
    pub fn credit_available_after(&self, amount: u64) -> Option<u64> {
        self.available.checked_add(amount)
    }

    /// 返回当前可用余额是否足以直接支付指定金额。
    pub fn can_debit_available(&self, amount: u64) -> bool {
        self.available >= amount
    }

    /// 返回直接扣减可用余额后的新可用余额。
    pub fn debit_available_after(&self, amount: u64) -> Option<u64> {
        self.available.checked_sub(amount)
    }

    /// 返回应用 signed 可用余额增量后的新可用余额。
    pub fn available_after_signed_delta(&self, delta: i128) -> Option<u64> {
        if delta >= 0 {
            let amount = u64::try_from(delta).ok()?;
            self.credit_available_after(amount)
        } else {
            let amount = delta.checked_neg().and_then(|value| u64::try_from(value).ok())?;
            self.debit_available_after(amount)
        }
    }

    /// 返回扣减冻结后的冻结余额。
    pub fn debit_frozen_after(&self, amount: u64) -> Option<u64> {
        self.frozen.checked_sub(amount)
    }

    /// 应用已计算好的余额字段和版本。
    pub fn apply_after(&mut self, available: u64, frozen: u64, version: u64) {
        self.available = available;
        self.frozen = frozen;
        self.version = version;
    }
}

impl Entity for Balance {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        format!("{}:{}", self.account_id, self.asset_id)
    }

    fn entity_type() -> u8 {
        BALANCE_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("available", "", self.available.to_string()),
            EntityFieldChange::new("frozen", "", self.frozen.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = vec![
            EntityFieldChange::new("account_id", self.account_id.clone(), other.account_id.clone()),
            EntityFieldChange::new("asset_id", self.asset_id.clone(), other.asset_id.clone()),
        ];
        if self.available != other.available {
            changes.push(EntityFieldChange::new(
                "available",
                self.available.to_string(),
                other.available.to_string(),
            ));
        }
        if self.frozen != other.frozen {
            changes.push(EntityFieldChange::new(
                "frozen",
                self.frozen.to_string(),
                other.frozen.to_string(),
            ));
        }
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "account_id" | "asset_id" => 0,
            "available" | "frozen" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_balance_entity_id(&self.entity_id()))
    }
}

fn stable_balance_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserve_and_release_are_asset_local() {
        let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3);

        assert!(balance.belongs_to_account("trader-1"));
        assert!(balance.is_asset("USDT"));
        assert!(balance.can_debit_available(200));
        assert!(!balance.can_debit_available(2_000));
        assert_eq!(balance.debit_available_after(300), Some(700));
        assert_eq!(balance.debit_available_after(2_000), None);
        assert_eq!(balance.reserve_after(200), Some((800, 200)));
        assert_eq!(
            Balance::new("trader-1".to_string(), "USDT".to_string(), 800, 200, 4).release_after(50),
            Some((850, 150))
        );
    }

    #[test]
    fn available_after_signed_delta_handles_credit_and_debit() {
        let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3);

        assert_eq!(balance.available_after_signed_delta(200), Some(1_200));
        assert_eq!(balance.available_after_signed_delta(-300), Some(700));
        assert_eq!(balance.available_after_signed_delta(-2_000), None);
    }
}
