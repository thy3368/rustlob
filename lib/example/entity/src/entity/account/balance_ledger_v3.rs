use common_entity::{
    AggregateRole, Entity, EntityFieldChange, EntityMutationModel, FieldDiff,
    FinancialClassification, FourColorArchetype, Fsm, UpdatedEntityPair,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::balance_v2::BalanceV2;

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
    #[error("ledger snapshots are incomplete or inconsistent")]
    IncompleteSnapshots,
    #[error("ledger has already been applied")]
    AlreadyApplied,
    #[error("application time moves backwards")]
    TimeWentBackwards,
    #[error("ledger version overflow")]
    VersionOverflow,
}

/// 一条 create-only 余额账务事实；余额先完成 FSM 迁移，再由本实体校验并记录快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceLedgerV3 {
    pub entry_id: u64,
    pub party_id: u64,
    pub account_id: u64,
    pub coin_idx: u64,
    pub amount: Decimal,
    pub before_available: Option<Decimal>,
    pub before_frozen: Option<Decimal>,
    pub after_available: Option<Decimal>,
    pub after_frozen: Option<Decimal>,
    pub created_at: u64,
    pub version: u64,
    pub updated_at: u64,
}

impl BalanceLedgerV3 {
    pub fn new(
        entry_id: u64,
        balance: &BalanceV2,
        amount: Decimal,
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
            amount,
            before_available: None,
            before_frozen: None,
            after_available: None,
            after_frozen: None,
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
}

impl Fsm for BalanceLedgerV3 {
    type State = BalanceLedgerV3State;
    type BizEvent = BalanceLedgerV3Event;
    type Output = BalanceLedgerV3Output;
    type Error = BalanceLedgerV3Error;

    fn transition(
        &mut self,
        event: Self::BizEvent,
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
        let expected_balance_version =
            before.version().checked_add(1).ok_or(BalanceLedgerV3Error::BalanceVersionMismatch)?;
        if after.version() != expected_balance_version {
            return Err(BalanceLedgerV3Error::BalanceVersionMismatch);
        }
        if applied_at < self.created_at || after.updated_at() != applied_at {
            return Err(BalanceLedgerV3Error::TimeWentBackwards);
        }

        let next_version =
            self.version.checked_add(1).ok_or(BalanceLedgerV3Error::VersionOverflow)?;
        let before_available = before.available();
        let before_frozen = before.frozen();
        let after_available = after.available();
        let after_frozen = after.frozen();

        self.before_available = Some(before_available);
        self.before_frozen = Some(before_frozen);
        self.after_available = Some(after_available);
        self.after_frozen = Some(after_frozen);
        self.version = next_version;
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
            EntityFieldChange::new("amount", "", self.amount.to_string()),
            EntityFieldChange::new("before_available", "", value(self.before_available)),
            EntityFieldChange::new("before_frozen", "", value(self.before_frozen)),
            EntityFieldChange::new("after_available", "", value(self.after_available)),
            EntityFieldChange::new("after_frozen", "", value(self.after_frozen)),
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

    use super::{BalanceLedgerV3, BalanceLedgerV3Error, BalanceLedgerV3Event};
    use crate::entity::{BalanceV2, BalanceV2Event};

    fn balance() -> BalanceV2 {
        BalanceV2::new(7, 11, 42, Decimal::from(100), Decimal::from(20), 3, 5)
    }

    fn update(balance: &BalanceV2, timestamp: u64) -> common_entity::UpdatedEntityPair<BalanceV2> {
        let before = balance.clone();
        let mut after = before.clone();
        after.apply(BalanceV2Event::Freeze { amount: Decimal::TEN, timestamp }).unwrap();
        common_entity::UpdatedEntityPair { before, after }
    }

    fn ledger(balance: &BalanceV2) -> BalanceLedgerV3 {
        BalanceLedgerV3::new(9, balance, Decimal::TEN, 10).unwrap()
    }

    fn event(
        update: common_entity::UpdatedEntityPair<BalanceV2>,
        applied_at: u64,
    ) -> BalanceLedgerV3Event {
        BalanceLedgerV3Event::BalanceApplied { balance_update: update, applied_at }
    }

    #[test]
    fn transition_records_authoritative_snapshots_and_returns_original_pair() {
        let balance = balance();
        let update = update(&balance, 12);
        let mut ledger = ledger(&balance);

        let result = ledger.transition(event(update.clone(), 12)).unwrap();

        assert_eq!(result.0, super::BalanceLedgerV3State::Applied);
        assert_eq!(result.1, super::BalanceLedgerV3Output::Applied { balance_update: update });
        assert_eq!(ledger.before_available, Some(Decimal::from(100)));
        assert_eq!(ledger.before_frozen, Some(Decimal::from(20)));
        assert_eq!(ledger.after_available, Some(Decimal::from(90)));
        assert_eq!(ledger.after_frozen, Some(Decimal::from(30)));
        assert_eq!(ledger.version, 1);
        assert_eq!(ledger.updated_at, 12);
    }

    #[test]
    fn rejects_already_applied_and_incomplete_snapshots_atomically() {
        let balance = balance();
        let update = update(&balance, 12);
        let mut applied = ledger(&balance);
        applied.apply(event(update.clone(), 12)).unwrap();
        let before = applied.clone();
        assert_eq!(
            applied.apply(event(update.clone(), 12)),
            Err(BalanceLedgerV3Error::AlreadyApplied)
        );
        assert_eq!(applied, before);

        let mut incomplete = ledger(&balance);
        incomplete.before_available = Some(balance.available());
        let before = incomplete.clone();
        assert_eq!(
            incomplete.apply(event(update, 12)),
            Err(BalanceLedgerV3Error::IncompleteSnapshots)
        );
        assert_eq!(incomplete, before);
    }

    #[test]
    fn rejects_invalid_pair_metadata_atomically() {
        let balance = balance();

        let other = BalanceV2::new(7, 12, 42, 100_u64, 20_u64, 3, 5);
        assert_rejected_unchanged(
            &balance,
            update(&other, 12),
            12,
            BalanceLedgerV3Error::BalanceIdentityMismatch,
        );

        let invalid_version = common_entity::UpdatedEntityPair {
            before: balance.clone(),
            after: BalanceV2::new(7, 11, 42, 90_u64, 30_u64, 5, 12),
        };
        assert_rejected_unchanged(
            &balance,
            invalid_version,
            12,
            BalanceLedgerV3Error::BalanceVersionMismatch,
        );

        assert_rejected_unchanged(
            &balance,
            update(&balance, 9),
            9,
            BalanceLedgerV3Error::TimeWentBackwards,
        );
        assert_rejected_unchanged(
            &balance,
            update(&balance, 12),
            11,
            BalanceLedgerV3Error::TimeWentBackwards,
        );
    }

    #[test]
    fn rejects_invalid_amount_and_ledger_version_overflow_atomically() {
        let balance = balance();
        let update = update(&balance, 12);
        let mut invalid_amount = ledger(&balance);
        invalid_amount.amount = Decimal::ZERO;
        let before = invalid_amount.clone();
        assert_eq!(
            invalid_amount.apply(event(update.clone(), 12)),
            Err(BalanceLedgerV3Error::InvalidAmount)
        );
        assert_eq!(invalid_amount, before);

        let mut overflow = ledger(&balance);
        overflow.version = u64::MAX;
        let before = overflow.clone();
        assert_eq!(overflow.apply(event(update, 12)), Err(BalanceLedgerV3Error::VersionOverflow));
        assert_eq!(overflow, before);
    }

    #[test]
    fn apply_uses_the_same_rules_as_transition() {
        let balance = balance();
        let update = update(&balance, 12);
        let mut via_transition = ledger(&balance);
        let mut via_apply = via_transition.clone();

        via_transition.transition(event(update.clone(), 12)).unwrap();
        via_apply.apply(event(update, 12)).unwrap();

        assert_eq!(via_apply, via_transition);
    }

    fn assert_rejected_unchanged(
        balance: &BalanceV2,
        update: common_entity::UpdatedEntityPair<BalanceV2>,
        applied_at: u64,
        expected: BalanceLedgerV3Error,
    ) {
        let mut ledger = ledger(balance);
        let before = ledger.clone();
        assert_eq!(ledger.apply(event(update, applied_at)), Err(expected));
        assert_eq!(ledger, before);
    }
}
