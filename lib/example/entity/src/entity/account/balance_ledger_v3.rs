use common_entity::{
    AggregateRole, Entity, EntityFieldChange, EntityMutationModel, FieldDiff,
    FinancialClassification, FourColorArchetype, Fsm, UpdatedEntityPair,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::balance_ledger_reason::BalanceLedgerReason;
use super::balance_v2::{BalanceV2, BalanceV2Error, BalanceV2Event};

const BALANCE_LEDGER_V3_ENTITY_TYPE: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceLedgerV3State {
    Pending,
    Applied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BalanceLedgerV3Event {
    BalanceApplied { balance_update: UpdatedEntityPair<BalanceV2>, applied_at: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BalanceLedgerV3Output {
    Applied { balance_update: UpdatedEntityPair<BalanceV2> },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BalanceLedgerV3Error {
    #[error("ledger amount must be greater than zero")]
    InvalidAmount,
    #[error("balance identity does not match ledger")]
    BalanceIdentityMismatch,
    #[error("balance versions must increase by one")]
    BalanceVersionMismatch,
    #[error("balance change does not match ledger operation")]
    OperationMismatch,
    #[error("ledger snapshots are incomplete or inconsistent")]
    IncompleteSnapshots,
    #[error("ledger has already been applied")]
    AlreadyApplied,
    #[error("application time moves backwards")]
    TimeWentBackwards,
    #[error("ledger version overflow")]
    VersionOverflow,
    #[error("decimal arithmetic overflow")]
    ArithmeticOverflow,
    #[error(transparent)]
    BalanceTransition(#[from] BalanceV2Error),
}

/// 一条 create-only 余额账务事实；余额先完成 FSM 迁移，再由本实体校验并记录快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceLedgerV3 {
    pub entry_id: u64,
    pub party_id: u64,
    pub account_id: u64,
    pub coin_idx: u64,
    pub operation: LedgerOperation,
    pub amount: Decimal,
    pub before_available: Option<Decimal>,
    pub before_frozen: Option<Decimal>,
    pub after_available: Option<Decimal>,
    pub after_frozen: Option<Decimal>,
    pub reason: BalanceLedgerReason,
    pub created_at: u64,
    pub version: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerOperation {
    Freeze,
    Unfreeze,
    UnfreezeAndPay,
    CreditAvailable,
    DebitAvailable,
    DebitFrozen,
}

impl LedgerOperation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Freeze => "freeze",
            Self::Unfreeze => "unfreeze",
            Self::UnfreezeAndPay => "unfreeze_and_pay",
            Self::CreditAvailable => "credit_available",
            Self::DebitAvailable => "debit_available",
            Self::DebitFrozen => "debit_frozen",
        }
    }
}

impl BalanceLedgerV3 {
    pub fn new(
        entry_id: u64,
        balance: &BalanceV2,
        operation: LedgerOperation,
        amount: Decimal,
        reason: BalanceLedgerReason,
        created_at: u64,
    ) -> Result<Self, BalanceLedgerV3Error> {
        if amount <= Decimal::ZERO {
            return Err(BalanceLedgerV3Error::InvalidAmount);
        }
        Ok(Self {
            entry_id,
            party_id: balance.party_id(),
            account_id: balance.account_id(),
            coin_idx: balance.coin_idx(),
            operation,
            amount,
            before_available: None,
            before_frozen: None,
            after_available: None,
            after_frozen: None,
            reason,
            created_at,
            version: 0,
            updated_at: created_at,
        })
    }

    pub fn state(&self) -> Result<BalanceLedgerV3State, BalanceLedgerV3Error> {
        match (self.before_available, self.before_frozen, self.after_available, self.after_frozen) {
            (None, None, None, None) => Ok(BalanceLedgerV3State::Pending),
            (Some(_), Some(_), Some(_), Some(_)) => Ok(BalanceLedgerV3State::Applied),
            _ => Err(BalanceLedgerV3Error::IncompleteSnapshots),
        }
    }

    pub fn is_applied(&self) -> bool {
        matches!(self.state(), Ok(BalanceLedgerV3State::Applied))
    }

    pub fn matches_balance_update(&self, pair: &UpdatedEntityPair<BalanceV2>) -> bool {
        self.is_applied()
            && self.account_id == pair.after.account_id()
            && self.coin_idx == pair.after.coin_idx()
            && self.party_id == pair.after.party_id()
            && self.before_available == Some(pair.before.available())
            && self.before_frozen == Some(pair.before.frozen())
            && self.after_available == Some(pair.after.available())
            && self.after_frozen == Some(pair.after.frozen())
    }

    /// 原子地应用余额 FSM，并以产生的 pair 完成本流水。
    pub fn apply_to(
        &mut self,
        balance: &mut BalanceV2,
        timestamp: u64,
    ) -> Result<UpdatedEntityPair<BalanceV2>, BalanceLedgerV3Error> {
        let before = balance.clone();
        let event = match self.operation {
            LedgerOperation::Freeze => BalanceV2Event::Freeze { amount: self.amount, timestamp },
            LedgerOperation::Unfreeze => BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: self.amount,
                payment_amount: Decimal::ZERO,
                timestamp,
            },
            LedgerOperation::UnfreezeAndPay => BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: Decimal::ZERO,
                payment_amount: self.amount,
                timestamp,
            },
            LedgerOperation::CreditAvailable => {
                BalanceV2Event::CreditAvailable { amount: self.amount, timestamp }
            }
            LedgerOperation::DebitAvailable => {
                BalanceV2Event::DebitAvailable { amount: self.amount, timestamp }
            }
            LedgerOperation::DebitFrozen => {
                BalanceV2Event::DebitFrozen { amount: self.amount, timestamp }
            }
        };
        let mut after = before.clone();
        after.apply(event)?;
        let pair = UpdatedEntityPair { before, after };
        self.apply(BalanceLedgerV3Event::BalanceApplied {
            balance_update: pair.clone(),
            applied_at: timestamp,
        })?;
        *balance = pair.after.clone();
        Ok(pair)
    }
}

impl Fsm for BalanceLedgerV3 {
    type State = BalanceLedgerV3State;
    type Event = BalanceLedgerV3Event;
    type Output = BalanceLedgerV3Output;
    type Error = BalanceLedgerV3Error;

    fn transition(
        &mut self,
        event: Self::Event,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        if self.state()? == BalanceLedgerV3State::Applied {
            return Err(BalanceLedgerV3Error::AlreadyApplied);
        }
        if self.amount <= Decimal::ZERO {
            return Err(BalanceLedgerV3Error::InvalidAmount);
        }
        let BalanceLedgerV3Event::BalanceApplied { balance_update, applied_at } = event;
        let before = &balance_update.before;
        let after = &balance_update.after;
        if before.party_id() != self.party_id
            || after.party_id() != self.party_id
            || before.account_id() != self.account_id
            || after.account_id() != self.account_id
            || before.coin_idx() != self.coin_idx
            || after.coin_idx() != self.coin_idx
        {
            return Err(BalanceLedgerV3Error::BalanceIdentityMismatch);
        }
        if after.version()
            != before.version().checked_add(1).ok_or(BalanceLedgerV3Error::VersionOverflow)?
        {
            return Err(BalanceLedgerV3Error::BalanceVersionMismatch);
        }
        if applied_at < self.created_at || after.updated_at() != applied_at {
            return Err(BalanceLedgerV3Error::TimeWentBackwards);
        }
        let (ba, bf, aa, af) =
            (before.available(), before.frozen(), after.available(), after.frozen());
        let add = |a: Decimal, b: Decimal| {
            a.checked_add(b).ok_or(BalanceLedgerV3Error::ArithmeticOverflow)
        };
        let sub = |a: Decimal, b: Decimal| {
            a.checked_sub(b).ok_or(BalanceLedgerV3Error::ArithmeticOverflow)
        };
        let matches = match self.operation {
            LedgerOperation::Freeze => sub(ba, self.amount)? == aa && add(bf, self.amount)? == af,
            LedgerOperation::Unfreeze => add(ba, self.amount)? == aa && sub(bf, self.amount)? == af,
            LedgerOperation::UnfreezeAndPay => {
                aa >= ba
                    && sub(bf, self.amount)? == af
                    && sub(self.amount, sub(aa, ba)?)? >= Decimal::ZERO
            }
            LedgerOperation::CreditAvailable => add(ba, self.amount)? == aa && bf == af,
            LedgerOperation::DebitAvailable => sub(ba, self.amount)? == aa && bf == af,
            LedgerOperation::DebitFrozen => ba == aa && sub(bf, self.amount)? == af,
        };
        if !matches {
            return Err(BalanceLedgerV3Error::OperationMismatch);
        }
        let version = self.version.checked_add(1).ok_or(BalanceLedgerV3Error::VersionOverflow)?;
        self.before_available = Some(ba);
        self.before_frozen = Some(bf);
        self.after_available = Some(aa);
        self.after_frozen = Some(af);
        self.version = version;
        self.updated_at = applied_at;
        Ok((BalanceLedgerV3State::Applied, BalanceLedgerV3Output::Applied { balance_update }))
    }
}

impl FieldDiff for BalanceLedgerV3 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let value = |v: Option<Decimal>| v.map(|n| n.to_string()).unwrap_or_default();
        vec![
            EntityFieldChange::new("entry_id", "", self.entry_id.to_string()),
            EntityFieldChange::new("party_id", "", self.party_id.to_string()),
            EntityFieldChange::new("account_id", "", self.account_id.to_string()),
            EntityFieldChange::new("coin_idx", "", self.coin_idx.to_string()),
            EntityFieldChange::new("operation", "", self.operation.as_str()),
            EntityFieldChange::new("amount", "", self.amount.to_string()),
            EntityFieldChange::new("before_available", "", value(self.before_available)),
            EntityFieldChange::new("before_frozen", "", value(self.before_frozen)),
            EntityFieldChange::new("after_available", "", value(self.after_available)),
            EntityFieldChange::new("after_frozen", "", value(self.after_frozen)),
            EntityFieldChange::new("reason", "", self.reason.as_str()),
            EntityFieldChange::new("created_at", "", self.created_at.to_string()),
            EntityFieldChange::new("updated_at", "", self.updated_at.to_string()),
        ]
    }
    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl common_entity::Shard for BalanceLedgerV3 {
    fn partition(&self) -> u8 {
        (self.account_id % 256) as u8
    }
}
impl Entity for BalanceLedgerV3 {
    fn entity_id(&self) -> u64 {
        self.entry_id
    }
    fn entity_type() -> u8 {
        BALANCE_LEDGER_V3_ENTITY_TYPE
    }
    fn is_derived() -> bool {
        true
    }
    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }
    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::AppendOnlyRecord
    }
    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }
    fn financial_classification() -> FinancialClassification {
        FinancialClassification::LedgerEntry
    }
    fn lsn(&self) -> u64 {
        self.version
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "amount" | "before_available" | "before_frozen" | "after_available"
            | "after_frozen" => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Fsm;
    use rust_decimal::Decimal;

    use super::{
        BalanceLedgerReason, BalanceLedgerV3, BalanceLedgerV3Error, BalanceLedgerV3Event,
        LedgerOperation,
    };
    use crate::entity::BalanceV2;

    fn balance() -> BalanceV2 {
        BalanceV2::new(7, 11, 42, Decimal::from(100), Decimal::from(20), 3, 5)
    }

    #[test]
    fn apply_to_records_decimal_snapshots_and_single_step_balance_version() {
        let mut balance = balance();
        let mut ledger = BalanceLedgerV3::new(
            9,
            &balance,
            LedgerOperation::Freeze,
            Decimal::new(125, 1),
            BalanceLedgerReason::FreezeForOrder { order_id: 99 },
            10,
        )
        .unwrap();

        let update = ledger.apply_to(&mut balance, 10).unwrap();

        assert_eq!(balance.available(), Decimal::new(875, 1));
        assert_eq!(balance.frozen(), Decimal::new(325, 1));
        assert_eq!(balance.version(), 4);
        assert_eq!(balance.business_entity_id(), "11:42");
        assert!(ledger.matches_balance_update(&update));
    }

    #[test]
    fn rejected_application_preserves_ledger_and_balance() {
        let mut balance = balance();
        let mut ledger = BalanceLedgerV3::new(
            9,
            &balance,
            LedgerOperation::DebitAvailable,
            Decimal::from(101),
            BalanceLedgerReason::FreezeForOrder { order_id: 99 },
            10,
        )
        .unwrap();
        let before_balance = balance.clone();
        let before_ledger = ledger.clone();

        assert_eq!(
            ledger.apply_to(&mut balance, 10),
            Err(BalanceLedgerV3Error::BalanceTransition(
                crate::entity::BalanceV2Error::InsufficientAvailableBalance,
            ))
        );
        assert_eq!(balance, before_balance);
        assert_eq!(ledger, before_ledger);
    }

    #[test]
    fn applying_an_unrelated_balance_update_is_rejected_atomically() {
        let balance = balance();
        let mut ledger = BalanceLedgerV3::new(
            9,
            &balance,
            LedgerOperation::CreditAvailable,
            Decimal::ONE,
            BalanceLedgerReason::FreezeForOrder { order_id: 99 },
            10,
        )
        .unwrap();
        let mut other = BalanceV2::new(7, 12, 42, 1_u64, 0_u64, 3, 5);
        let before_ledger = ledger.clone();
        let before_other = other.clone();

        let before = other.clone();
        other
            .apply(crate::entity::BalanceV2Event::CreditAvailable {
                amount: Decimal::ONE,
                timestamp: 10,
            })
            .unwrap();
        let update = common_entity::UpdatedEntityPair { before, after: other.clone() };

        assert_eq!(
            ledger.apply(BalanceLedgerV3Event::BalanceApplied {
                balance_update: update,
                applied_at: 10,
            }),
            Err(BalanceLedgerV3Error::BalanceIdentityMismatch)
        );
        assert_eq!(ledger, before_ledger);
        assert_ne!(other, before_other);
    }
}
