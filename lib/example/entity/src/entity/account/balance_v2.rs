use std::marker::PhantomData;

use common_entity::{Entity, EntityFieldChange, FieldDiff};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::support::concat3;

const BALANCE_V2_ENTITY_TYPE: u8 = 12;

/// `BalanceV2` 的余额行为错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BalanceV2Error {
    /// 金额必须大于零。
    #[error("balance amount must be greater than zero")]
    InvalidAmount,
    /// 可用余额不足。
    #[error("available balance is insufficient")]
    InsufficientAvailableBalance,
    /// 冻结余额不足。
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    /// 余额属于已关闭的账户资产。
    #[error("balance is closed")]
    BalanceClosed,
    /// 余额或版本计算发生溢出。
    #[error("arithmetic overflow while deriving balance transition")]
    ArithmeticOverflow,
    /// 可用余额或冻结余额尚未清零。
    #[error("balance cannot be closed while available or frozen funds remain")]
    CannotCloseNonZeroBalance,
}

/// 余额实体的持久化生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum BalanceV2Status {
    /// 余额仍可执行资金变更。
    #[default]
    Open,
    /// 余额已经关闭，不能再执行资金变更。
    Closed,
}

impl BalanceV2Status {
    /// 返回稳定的 replay 状态值。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

/// 开放余额 typestate 标记。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Open;

/// 已关闭余额 typestate 标记。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Closed;

/// 可序列化、可比较并参与 replay 的稳定余额记录。
///
/// `version` 是实体 LSN，不作为独立字段写入字段级 diff；更新事件通过 LSN 表达版本递增。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceV2Record {
    party_id: u64,
    account_id: u64,
    asset_id: u64,

    available: Decimal,
    frozen: Decimal,
    version: u64,
    #[serde(default)]
    status: BalanceV2Status,
}

impl BalanceV2Record {
    /// 返回余额对应的业务主体 ID。
    pub fn party_id(&self) -> u64 {
        self.party_id
    }

    /// 返回余额对应的账户 ID。
    pub fn account_id(&self) -> u64 {
        self.account_id
    }

    /// 返回余额对应的资产 ID。
    pub fn asset_id(&self) -> u64 {
        self.asset_id
    }

    /// 返回可用余额。
    pub fn available(&self) -> Decimal {
        self.available
    }

    /// 返回冻结余额。
    pub fn frozen(&self) -> Decimal {
        self.frozen
    }

    /// 返回实体版本。
    pub fn version(&self) -> u64 {
        self.version
    }

    /// 返回持久化生命周期状态。
    pub fn status(&self) -> BalanceV2Status {
        self.status
    }
}

/// 以 typestate 限制余额生命周期操作的实体包装。
///
/// 默认阶段是 [`Open`]。该包装使用 [`BalanceV2Record`] 的平坦序列化形状；解码时会检查
/// 持久化状态与目标 Rust 阶段一致。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceV2<S = Open> {
    record: BalanceV2Record,
    stage: PhantomData<S>,
}

impl<S> BalanceV2<S> {
    /// 返回该余额的稳定持久化记录。
    pub fn record(&self) -> &BalanceV2Record {
        &self.record
    }

    /// 消耗 typestate 包装并返回稳定持久化记录。
    pub fn into_record(self) -> BalanceV2Record {
        self.record
    }

    /// 返回余额对应的业务主体 ID。
    pub fn party_id(&self) -> u64 {
        self.record.party_id()
    }

    /// 返回余额对应的账户 ID。
    pub fn account_id(&self) -> u64 {
        self.record.account_id()
    }

    /// 返回余额对应的资产 ID。
    pub fn asset_id(&self) -> u64 {
        self.record.asset_id()
    }

    /// 返回可用余额。
    pub fn available(&self) -> Decimal {
        self.record.available()
    }

    /// 返回冻结余额。
    pub fn frozen(&self) -> Decimal {
        self.record.frozen()
    }

    /// 返回实体版本。
    pub fn version(&self) -> u64 {
        self.record.version()
    }

    /// 返回持久化生命周期状态。
    pub fn status(&self) -> BalanceV2Status {
        self.record.status()
    }
}

impl BalanceV2<Open> {
    /// 创建开放余额；参数用于装配已校验或回放的初始业务事实。
    pub fn new(
        party_id: u64,
        account_id: u64,
        asset_id: u64,
        available: Decimal,
        frozen: Decimal,
        version: u64,
    ) -> Self {
        Self {
            record: BalanceV2Record {
                party_id,
                account_id,
                asset_id,
                available,
                frozen,
                version,
                status: BalanceV2Status::Open,
            },
            stage: PhantomData,
        }
    }

    /// 可 BDD 规格化的聚合根行为：冻结可用余额。
    ///
    /// 将指定金额从可用余额转入冻结余额；校验失败时余额和版本保持不变。
    pub fn freeze(&mut self, amount: Decimal) -> Result<(), BalanceV2Error> {
        if self.record.status != BalanceV2Status::Open {
            return Err(BalanceV2Error::BalanceClosed);
        }
        if amount <= Decimal::ZERO {
            return Err(BalanceV2Error::InvalidAmount);
        }

        if self.record.available < amount {
            return Err(BalanceV2Error::InsufficientAvailableBalance);
        }
        let available =
            self.record.available.checked_sub(amount).ok_or(BalanceV2Error::ArithmeticOverflow)?;
        let frozen =
            self.record.frozen.checked_add(amount).ok_or(BalanceV2Error::ArithmeticOverflow)?;
        let version =
            self.record.version.checked_add(1).ok_or(BalanceV2Error::ArithmeticOverflow)?;

        self.record.available = available;
        self.record.frozen = frozen;
        self.record.version = version;
        Ok(())
    }

    /// 可 BDD 规格化的聚合根行为：原子解冻并支付冻结余额。
    ///
    /// 例如 `unfreeze_and_pay(20, 80)` 会解冻 20、支付 80，使可用余额增加 20、冻结余额减少
    /// 100，总余额减少 80。校验失败时余额和版本保持不变。
    pub fn unfreeze_and_pay(
        &mut self,
        unfreeze_amount: Decimal,
        payment_amount: Decimal,
    ) -> Result<(), BalanceV2Error> {
        if self.record.status != BalanceV2Status::Open {
            return Err(BalanceV2Error::BalanceClosed);
        }
        if unfreeze_amount <= Decimal::ZERO || payment_amount <= Decimal::ZERO {
            return Err(BalanceV2Error::InvalidAmount);
        }

        let consumed_frozen = unfreeze_amount
            .checked_add(payment_amount)
            .ok_or(BalanceV2Error::ArithmeticOverflow)?;
        if self.record.frozen < consumed_frozen {
            return Err(BalanceV2Error::InsufficientFrozenBalance);
        }
        let frozen = self
            .record
            .frozen
            .checked_sub(consumed_frozen)
            .ok_or(BalanceV2Error::ArithmeticOverflow)?;
        let available = self
            .record
            .available
            .checked_add(unfreeze_amount)
            .ok_or(BalanceV2Error::ArithmeticOverflow)?;
        let version =
            self.record.version.checked_add(1).ok_or(BalanceV2Error::ArithmeticOverflow)?;

        self.record.available = available;
        self.record.frozen = frozen;
        self.record.version = version;
        Ok(())
    }

    /// 将成交消耗额从冻结余额中扣除，不释放订单的剩余冻结额。
    pub fn debit_frozen(&mut self, amount: Decimal) -> Result<(), BalanceV2Error> {
        if self.record.status != BalanceV2Status::Open {
            return Err(BalanceV2Error::BalanceClosed);
        }
        if amount <= Decimal::ZERO {
            return Err(BalanceV2Error::InvalidAmount);
        }
        if self.record.frozen < amount {
            return Err(BalanceV2Error::InsufficientFrozenBalance);
        }

        let frozen =
            self.record.frozen.checked_sub(amount).ok_or(BalanceV2Error::ArithmeticOverflow)?;
        let version =
            self.record.version.checked_add(1).ok_or(BalanceV2Error::ArithmeticOverflow)?;
        self.record.frozen = frozen;
        self.record.version = version;
        Ok(())
    }

    /// 将成交所得加入可用余额。
    pub fn credit_available(&mut self, amount: Decimal) -> Result<(), BalanceV2Error> {
        if self.record.status != BalanceV2Status::Open {
            return Err(BalanceV2Error::BalanceClosed);
        }
        if amount <= Decimal::ZERO {
            return Err(BalanceV2Error::InvalidAmount);
        }

        let available =
            self.record.available.checked_add(amount).ok_or(BalanceV2Error::ArithmeticOverflow)?;
        let version =
            self.record.version.checked_add(1).ok_or(BalanceV2Error::ArithmeticOverflow)?;
        self.record.available = available;
        self.record.version = version;
        Ok(())
    }

    /// 将余额关闭。仅可用余额和冻结余额都为零时成功；失败会返回未改变的开放余额。
    pub fn close(self) -> Result<BalanceV2<Closed>, (Self, BalanceV2Error)> {
        if self.record.available != Decimal::ZERO || self.record.frozen != Decimal::ZERO {
            return Err((self, BalanceV2Error::CannotCloseNonZeroBalance));
        }

        let Some(version) = self.record.version.checked_add(1) else {
            return Err((self, BalanceV2Error::ArithmeticOverflow));
        };
        let mut record = self.record;
        record.status = BalanceV2Status::Closed;
        record.version = version;

        Ok(BalanceV2 { record, stage: PhantomData })
    }
}

impl Serialize for BalanceV2<Open> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.record.serialize(serializer)
    }
}

impl Serialize for BalanceV2<Closed> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.record.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BalanceV2<Open> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let record = BalanceV2Record::deserialize(deserializer)?;
        if record.status != BalanceV2Status::Open {
            return Err(serde::de::Error::custom("closed balance cannot be decoded as open"));
        }
        Ok(Self { record, stage: PhantomData })
    }
}

impl<'de> Deserialize<'de> for BalanceV2<Closed> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let record = BalanceV2Record::deserialize(deserializer)?;
        if record.status != BalanceV2Status::Closed {
            return Err(serde::de::Error::custom("open balance cannot be decoded as closed"));
        }
        Ok(Self { record, stage: PhantomData })
    }
}

impl FieldDiff for BalanceV2Record {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("party_id", "", self.party_id.to_string()),
            EntityFieldChange::new("account_id", "", self.account_id.to_string()),
            EntityFieldChange::new("asset_id", "", self.asset_id.to_string()),
            EntityFieldChange::new("available", "", self.available.to_string()),
            EntityFieldChange::new("frozen", "", self.frozen.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::with_capacity(6);
        if self.party_id != other.party_id {
            changes.push(EntityFieldChange::new(
                "party_id",
                self.party_id.to_string(),
                other.party_id.to_string(),
            ));
        }
        if self.account_id != other.account_id {
            changes.push(EntityFieldChange::new(
                "account_id",
                self.account_id.to_string(),
                other.account_id.to_string(),
            ));
        }
        if self.asset_id != other.asset_id {
            changes.push(EntityFieldChange::new(
                "asset_id",
                self.asset_id.to_string(),
                other.asset_id.to_string(),
            ));
        }
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
        if self.status != other.status {
            changes.push(EntityFieldChange::new(
                "status",
                self.status.as_str(),
                other.status.as_str(),
            ));
        }
        changes
    }
}

impl common_entity::Shard for BalanceV2Record {
    fn partition(&self) -> u8 {
        (self.account_id % 256) as u8
    }
}

impl Entity for BalanceV2Record {
    fn entity_id(&self) -> u64 {
        let business_id = concat3(&self.account_id.to_string(), ":", &self.asset_id.to_string());
        common_entity::stable_string_entity_id(&business_id)
    }

    fn is_derived() -> bool {
        true
    }

    fn entity_type() -> u8 {
        BALANCE_V2_ENTITY_TYPE
    }

    fn lsn(&self) -> u64 {
        self.version
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "account_id" | "asset_id" => 0,
            "available" | "frozen" => 1,
            "status" => 0,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, EntityFieldChange, FieldDiff, Shard};
    use rust_decimal::Decimal;
    use serde_json::json;

    use super::{BalanceV2, BalanceV2Error, BalanceV2Record, BalanceV2Status, Closed, Open};

    fn amount(value: i64) -> Decimal {
        Decimal::from(value)
    }

    fn balance() -> BalanceV2<Open> {
        BalanceV2::new(1, 2, 3, amount(1_000), Decimal::ZERO, 3)
    }

    #[test]
    fn new_balance_is_open_and_preserves_initial_facts() {
        let balance = balance();

        assert_eq!(balance.status(), BalanceV2Status::Open);
        assert_eq!(
            (
                balance.party_id(),
                balance.account_id(),
                balance.asset_id(),
                balance.available(),
                balance.frozen(),
                balance.version(),
            ),
            (1, 2, 3, amount(1_000), Decimal::ZERO, 3)
        );
    }

    #[test]
    fn freeze_then_unfreeze_and_pay_updates_amounts_and_version() {
        let mut balance = balance();

        balance.freeze(amount(100)).unwrap();
        assert_eq!(
            (balance.available(), balance.frozen(), balance.version()),
            (amount(900), amount(100), 4)
        );

        balance.unfreeze_and_pay(amount(20), amount(80)).unwrap();
        assert_eq!(
            (balance.available(), balance.frozen(), balance.version()),
            (amount(920), Decimal::ZERO, 5)
        );
    }

    #[test]
    fn fractional_amounts_are_preserved_across_transitions() {
        let mut balance = BalanceV2::new(1, 2, 3, Decimal::new(125, 2), Decimal::ZERO, 3);

        balance.freeze(Decimal::new(125, 3)).unwrap();
        assert_eq!(balance.available(), Decimal::new(1_125, 3));
        assert_eq!(balance.frozen(), Decimal::new(125, 3));

        balance.unfreeze_and_pay(Decimal::new(25, 3), Decimal::new(1, 1)).unwrap();
        assert_eq!(balance.available(), Decimal::new(115, 2));
        assert_eq!(balance.frozen(), Decimal::ZERO);
    }

    #[test]
    fn failed_amount_transitions_leave_balance_unchanged() {
        let mut balance = balance();
        balance.freeze(amount(100)).unwrap();
        let before = balance.clone();

        assert_eq!(balance.freeze(Decimal::ZERO), Err(BalanceV2Error::InvalidAmount));
        assert_eq!(balance.freeze(amount(-1)), Err(BalanceV2Error::InvalidAmount));
        assert_eq!(balance.freeze(amount(901)), Err(BalanceV2Error::InsufficientAvailableBalance));
        assert_eq!(balance, before);
        assert_eq!(
            balance.unfreeze_and_pay(amount(20), amount(81)),
            Err(BalanceV2Error::InsufficientFrozenBalance)
        );
        assert_eq!(
            balance.unfreeze_and_pay(Decimal::ZERO, amount(80)),
            Err(BalanceV2Error::InvalidAmount)
        );
        assert_eq!(
            balance.unfreeze_and_pay(amount(-1), amount(80)),
            Err(BalanceV2Error::InvalidAmount)
        );
        assert_eq!(
            balance.unfreeze_and_pay(amount(20), Decimal::ZERO),
            Err(BalanceV2Error::InvalidAmount)
        );
        assert_eq!(balance, before);
    }

    #[test]
    fn amount_and_version_overflow_leave_balance_unchanged() {
        let mut balance = BalanceV2::new(1, 2, 3, amount(1_000), Decimal::MAX, 1);
        let before = balance.clone();
        assert_eq!(balance.freeze(amount(100)), Err(BalanceV2Error::ArithmeticOverflow));
        assert_eq!(balance, before);

        let mut balance = BalanceV2::new(1, 2, 3, amount(1_000), Decimal::ZERO, u64::MAX);
        let before = balance.clone();
        assert_eq!(balance.freeze(amount(100)), Err(BalanceV2Error::ArithmeticOverflow));
        assert_eq!(balance, before);

        let mut balance = BalanceV2::new(1, 2, 3, Decimal::ZERO, Decimal::MAX, 1);
        let before = balance.clone();
        assert_eq!(
            balance.unfreeze_and_pay(Decimal::MAX, amount(1)),
            Err(BalanceV2Error::ArithmeticOverflow)
        );
        assert_eq!(balance, before);

        let mut balance = BalanceV2::new(1, 2, 3, Decimal::MAX, amount(2), 1);
        let before = balance.clone();
        assert_eq!(
            balance.unfreeze_and_pay(amount(1), amount(1)),
            Err(BalanceV2Error::ArithmeticOverflow)
        );
        assert_eq!(balance, before);
    }

    #[test]
    fn debit_frozen_and_credit_available_apply_checked_transitions() {
        let mut balance = BalanceV2::new(1, 2, 3, amount(10), amount(50), 4);
        balance.debit_frozen(amount(20)).unwrap();
        assert_eq!(
            (balance.available(), balance.frozen(), balance.version()),
            (amount(10), amount(30), 5)
        );
        balance.credit_available(amount(7)).unwrap();
        assert_eq!(
            (balance.available(), balance.frozen(), balance.version()),
            (amount(17), amount(30), 6)
        );
    }

    #[test]
    fn debit_and_credit_failures_leave_balance_unchanged() {
        let mut balance = BalanceV2::new(1, 2, 3, Decimal::MAX, amount(5), 4);
        let before = balance.clone();
        assert_eq!(balance.debit_frozen(Decimal::ZERO), Err(BalanceV2Error::InvalidAmount));
        assert_eq!(balance.debit_frozen(amount(6)), Err(BalanceV2Error::InsufficientFrozenBalance));
        assert_eq!(balance.credit_available(amount(1)), Err(BalanceV2Error::ArithmeticOverflow));
        assert_eq!(balance.credit_available(amount(-1)), Err(BalanceV2Error::InvalidAmount));
        assert_eq!(balance, before);

        let mut balance = BalanceV2::new(1, 2, 3, Decimal::ZERO, amount(5), u64::MAX);
        let before = balance.clone();
        assert_eq!(balance.debit_frozen(amount(1)), Err(BalanceV2Error::ArithmeticOverflow));
        assert_eq!(balance, before);
    }

    #[test]
    fn close_requires_zero_balance_and_returns_original_on_failure() {
        let available_nonzero = balance();
        let before = available_nonzero.clone();
        let (returned, error) = available_nonzero.close().unwrap_err();
        assert_eq!(error, BalanceV2Error::CannotCloseNonZeroBalance);
        assert_eq!(returned, before);

        let frozen_nonzero = BalanceV2::new(1, 2, 3, Decimal::ZERO, amount(10), 4);
        let (returned, error) = frozen_nonzero.close().unwrap_err();
        assert_eq!(error, BalanceV2Error::CannotCloseNonZeroBalance);
        assert_eq!(
            (returned.available(), returned.frozen(), returned.version()),
            (Decimal::ZERO, amount(10), 4)
        );
    }

    #[test]
    fn zero_balance_closes_once_and_advances_version() {
        let closed: BalanceV2<Closed> =
            BalanceV2::new(1, 2, 3, Decimal::ZERO, Decimal::ZERO, 7).close().unwrap();
        assert_eq!(closed.status(), BalanceV2Status::Closed);
        assert_eq!(closed.version(), 8);
        assert_eq!(closed.record().available(), Decimal::ZERO);
        assert_eq!(closed.record().frozen(), Decimal::ZERO);

        let open = BalanceV2::new(1, 2, 3, Decimal::ZERO, Decimal::ZERO, u64::MAX);
        let (returned, error) = open.close().unwrap_err();
        assert_eq!(error, BalanceV2Error::ArithmeticOverflow);
        assert_eq!(returned.version(), u64::MAX);
        assert_eq!(returned.status(), BalanceV2Status::Open);
    }

    #[test]
    fn serialization_is_flat_and_old_records_default_to_open() {
        let open = balance();
        let encoded = serde_json::to_value(&open).unwrap();
        assert_eq!(encoded["status"], json!("open"));

        let fractional = BalanceV2::new(1, 2, 3, Decimal::new(125, 2), Decimal::new(25, 2), 3);
        let fractional_json = serde_json::to_value(&fractional).unwrap();
        let decoded_fractional: BalanceV2<Open> = serde_json::from_value(fractional_json).unwrap();
        assert_eq!(decoded_fractional.available(), Decimal::new(125, 2));
        assert_eq!(decoded_fractional.frozen(), Decimal::new(25, 2));

        let old_json = json!({
            "party_id": 1,
            "account_id": 2,
            "asset_id": 3,
            "available": 1_000,
            "frozen": 0,
            "version": 3
        });
        let decoded: BalanceV2<Open> = serde_json::from_value(old_json).unwrap();
        assert_eq!(decoded.status(), BalanceV2Status::Open);

        let closed: BalanceV2<Closed> =
            BalanceV2::new(1, 2, 3, Decimal::ZERO, Decimal::ZERO, 7).close().unwrap();
        let closed_json = serde_json::to_value(&closed).unwrap();
        assert!(serde_json::from_value::<BalanceV2<Open>>(closed_json.clone()).is_err());
        assert!(serde_json::from_value::<BalanceV2<Closed>>(encoded).is_err());
        let decoded_closed: BalanceV2<Closed> = serde_json::from_value(closed_json).unwrap();
        assert_eq!(decoded_closed.status(), BalanceV2Status::Closed);
    }

    #[test]
    fn close_record_diff_and_replay_keep_stable_entity_metadata() {
        let open = BalanceV2::new(1, 2, 3, Decimal::ZERO, Decimal::ZERO, 7);
        let before = open.into_record();
        let after =
            BalanceV2::new(1, 2, 3, Decimal::ZERO, Decimal::ZERO, 7).close().unwrap().into_record();

        assert_eq!(before.diff(&after), vec![EntityFieldChange::new("status", "open", "closed")]);
        assert!(
            before.created_field_changes().contains(&EntityFieldChange::new("status", "", "open"))
        );

        let event = after.track_update_event_from(&before).unwrap();
        assert_eq!(event.previous_lsn, 7);
        assert_eq!(event.lsn, 8);
        assert_eq!(event.entity_id, before.entity_id());
        assert_eq!(event.entity_type, 12);
        assert_eq!(event.p, 2);
        assert_eq!(event.field_value("status"), Some(b"closed".as_slice()));
        assert_eq!(BalanceV2Record::replay_field_type("status"), 0);
        assert_eq!(BalanceV2Record::replay_field_type("available"), 1);
        assert!(BalanceV2Record::is_derived());
    }

    #[test]
    fn balance_record_entity_identity_partition_and_lsn_are_stable() {
        let record = balance().into_record();
        assert_eq!(record.entity_id(), common_entity::stable_string_entity_id("2:3"));
        assert_eq!(BalanceV2Record::entity_type(), 12);
        assert_eq!(record.partition(), 2);
        assert_eq!(record.lsn(), 3);
    }
}
