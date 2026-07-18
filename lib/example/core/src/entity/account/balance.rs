use common_entity::{Entity, EntityError, EntityFieldChange};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const BALANCE_ENTITY_TYPE: u8 = 7;

/// 余额实体行为错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BalanceError {
    /// 金额必须大于零。
    #[error("balance amount must be greater than zero")]
    InvalidAmount,
    /// 可用余额不足。
    #[error("available balance is insufficient")]
    InsufficientAvailableBalance,
    /// 冻结余额不足。
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    /// 余额状态计算发生整数溢出。
    #[error("arithmetic overflow while deriving balance transition")]
    ArithmeticOverflow,
}

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
    /// 上游快照附带的入场名义值；并非所有余额都提供该事实。
    #[serde(default)]
    pub entry_notional: Option<u64>,
    /// 上游快照附带的标识信息；并非所有余额都提供该事实。
    #[serde(default)]
    pub identifier: Option<String>,
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
        Self {
            account_id,
            asset_id,
            available,
            frozen,
            entry_notional: None,
            identifier: None,
            version,
        }
    }

    /// 从快照事实装配资产余额，并保留上游附带的补充信息。
    pub fn new_with_snapshot_facts(
        account_id: String,
        asset_id: String,
        available: u64,
        frozen: u64,
        entry_notional: Option<u64>,
        identifier: Option<String>,
        version: u64,
    ) -> Self {
        Self { account_id, asset_id, available, frozen, entry_notional, identifier, version }
    }

    /// 返回余额是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回余额是否对应指定资产。
    pub fn is_asset(&self, asset_id: &str) -> bool {
        self.asset_id == asset_id
    }

    /// 返回余额总额，即 `available + frozen`；若溢出则返回 `None`。
    pub fn total(&self) -> Option<u64> {
        self.available.checked_add(self.frozen)
    }

    /// 返回快照里的入场名义值。
    pub fn entry_notional(&self) -> Option<u64> {
        self.entry_notional
    }

    /// 返回快照里的附带标识。
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }

    /// 返回两个余额是否表达同一条业务快照。
    pub fn matches_business_snapshot(&self, other: &Self) -> bool {
        self.account_id == other.account_id
            && self.asset_id == other.asset_id
            && self.available == other.available
            && self.frozen == other.frozen
            && self.entry_notional == other.entry_notional
            && self.identifier == other.identifier
    }

    /// 返回是否能冻结指定数量。
    pub fn can_reserve(&self, amount: u64) -> bool {
        self.available >= amount
    }

    /// 可 BDD 规格化的聚合根行为：冻结可用余额。
    pub fn reserve(&mut self, amount: u64) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }

        let available =
            self.available.checked_sub(amount).ok_or(BalanceError::InsufficientAvailableBalance)?;
        let frozen = self.frozen.checked_add(amount).ok_or(BalanceError::ArithmeticOverflow)?;
        self.apply_amounts(available, frozen)
    }

    /// 可 BDD 规格化的聚合根行为：释放冻结余额。
    pub fn release(&mut self, amount: u64) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }

        let frozen =
            self.frozen.checked_sub(amount).ok_or(BalanceError::InsufficientFrozenBalance)?;
        let available =
            self.available.checked_add(amount).ok_or(BalanceError::ArithmeticOverflow)?;
        self.apply_amounts(available, frozen)
    }

    /// 可 BDD 规格化的聚合根行为：入账可用余额。
    pub fn credit_available(&mut self, amount: u64) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }

        let available =
            self.available.checked_add(amount).ok_or(BalanceError::ArithmeticOverflow)?;
        self.apply_amounts(available, self.frozen)
    }

    /// 返回当前可用余额是否足以直接支付指定金额。
    pub fn can_debit_available(&self, amount: u64) -> bool {
        self.available >= amount
    }

    /// 可 BDD 规格化的聚合根行为：扣减可用余额。
    pub fn debit_available(&mut self, amount: u64) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }

        let available =
            self.available.checked_sub(amount).ok_or(BalanceError::InsufficientAvailableBalance)?;
        self.apply_amounts(available, self.frozen)
    }

    /// 可 BDD 规格化的聚合根行为：成交消耗冻结余额。
    pub fn debit_frozen(&mut self, amount: u64) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }

        let frozen =
            self.frozen.checked_sub(amount).ok_or(BalanceError::InsufficientFrozenBalance)?;
        self.apply_amounts(self.available, frozen)
    }

    fn apply_amounts(&mut self, available: u64, frozen: u64) -> Result<(), BalanceError> {
        let version = self.version.checked_add(1).ok_or(BalanceError::ArithmeticOverflow)?;
        self.available = available;
        self.frozen = frozen;
        self.version = version;
        Ok(())
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
            EntityFieldChange::new("entry_notional", "", option_u64_value(self.entry_notional)),
            EntityFieldChange::new("identifier", "", self.identifier.clone().unwrap_or_default()),
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
        if self.entry_notional != other.entry_notional {
            changes.push(EntityFieldChange::new(
                "entry_notional",
                option_u64_value(self.entry_notional),
                option_u64_value(other.entry_notional),
            ));
        }
        if self.identifier != other.identifier {
            changes.push(EntityFieldChange::new(
                "identifier",
                self.identifier.clone().unwrap_or_default(),
                other.identifier.clone().unwrap_or_default(),
            ));
        }
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "account_id" | "asset_id" | "identifier" => 0,
            "available" | "frozen" | "entry_notional" => 1,
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

fn option_u64_value(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}
