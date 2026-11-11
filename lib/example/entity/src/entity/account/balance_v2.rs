use common_entity::{Entity, EntityFieldChange, FieldDiff, Fsm};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::support::concat3;

const BALANCE_V2_ENTITY_TYPE: u8 = 12;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BalanceV2Error {
    #[error("balance amount must be greater than zero")]
    InvalidAmount,
    #[error("available balance is insufficient")]
    InsufficientAvailableBalance,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("balance is closed")]
    BalanceClosed,
    #[error("arithmetic overflow while deriving balance transition")]
    ArithmeticOverflow,
    #[error("balance cannot be closed while available or frozen funds remain")]
    CannotCloseNonZeroBalance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BalanceV2Status {
    #[default]
    Open,
    Closed,
}
impl BalanceV2Status {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BalanceV2Event {
    Freeze { amount: Decimal, timestamp: u64 },
    UnfreezeAndPay { unfreeze_amount: Decimal, payment_amount: Decimal, timestamp: u64 },
    ///todo DebitFrozen应该没有用
    DebitFrozen { amount: Decimal, timestamp: u64 },
    CreditAvailable { amount: Decimal, timestamp: u64 },
    DebitAvailable { amount: Decimal, timestamp: u64 },
    Close { timestamp: u64 },
}

/// 可变余额实体，同时承载稳定身份、生命周期和 replay 字段。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceV2 {
    party_id: u64,
    account_id: u64,
    coin_idx: u64,
    available: Decimal,
    frozen: Decimal,
    status: BalanceV2Status,
    version: u64,
    created_at: u64,
    updated_at: u64,
}
impl BalanceV2 {
    /// 返回账户与币种组成的稳定业务身份。
    pub fn business_entity_id(&self) -> String {
        concat3(&self.account_id.to_string(), ":", &self.coin_idx.to_string())
    }
    pub fn belongs_to_account(&self, account_id: u64) -> bool {
        self.account_id == account_id
    }
    pub fn is_coin(&self, coin_idx: u64) -> bool {
        self.coin_idx == coin_idx
    }
    pub fn total(&self) -> Option<Decimal> {
        self.available.checked_add(self.frozen)
    }
    pub fn party_id(&self) -> u64 {
        self.party_id
    }
    pub fn account_id(&self) -> u64 {
        self.account_id
    }
    pub fn coin_idx(&self) -> u64 {
        self.coin_idx
    }
    pub fn available(&self) -> Decimal {
        self.available
    }
    pub fn frozen(&self) -> Decimal {
        self.frozen
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    pub fn updated_at(&self) -> u64 {
        self.updated_at
    }
    pub fn status(&self) -> BalanceV2Status {
        self.status
    }
}

impl BalanceV2 {
    pub fn new<A, F>(
        party_id: u64,
        account_id: u64,
        coin_idx: u64,
        available: A,
        frozen: F,
        version: u64,
        created_at: u64,
    ) -> Self
    where
        A: Into<Decimal>,
        F: Into<Decimal>,
    {
        Self {
            party_id,
            account_id,
            coin_idx,
            available: available.into(),
            frozen: frozen.into(),
            status: BalanceV2Status::Open,
            version,
            created_at,
            updated_at: created_at,
        }
    }
}

impl Fsm for BalanceV2 {
    type State = BalanceV2Status;
    type Event = BalanceV2Event;
    type Output = ();
    type Error = BalanceV2Error;
    fn transition(
        &mut self,
        event: Self::Event,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        if self.status() == BalanceV2Status::Closed {
            return Err(BalanceV2Error::BalanceClosed);
        }
        let (available, frozen, status, timestamp) = match event {
            BalanceV2Event::Freeze { amount, timestamp } => {
                positive(amount)?;
                if self.available() < amount {
                    return Err(BalanceV2Error::InsufficientAvailableBalance);
                };
                (
                    checked_sub(self.available(), amount)?,
                    checked_add(self.frozen(), amount)?,
                    self.status(),
                    timestamp,
                )
            }
            BalanceV2Event::UnfreezeAndPay { unfreeze_amount, payment_amount, timestamp } => {
                positive(unfreeze_amount)?;
                if payment_amount < Decimal::ZERO {
                    return Err(BalanceV2Error::InvalidAmount);
                }
                let consumed = checked_add(unfreeze_amount, payment_amount)?;
                if self.frozen() < consumed {
                    return Err(BalanceV2Error::InsufficientFrozenBalance);
                };
                (
                    checked_add(self.available(), unfreeze_amount)?,
                    checked_sub(self.frozen(), consumed)?,
                    self.status(),
                    timestamp,
                )
            }
            BalanceV2Event::DebitFrozen { amount, timestamp } => {
                positive(amount)?;
                if self.frozen() < amount {
                    return Err(BalanceV2Error::InsufficientFrozenBalance);
                };
                (self.available(), checked_sub(self.frozen(), amount)?, self.status(), timestamp)
            }
            BalanceV2Event::CreditAvailable { amount, timestamp } => {
                positive(amount)?;
                (checked_add(self.available(), amount)?, self.frozen(), self.status(), timestamp)
            }
            BalanceV2Event::DebitAvailable { amount, timestamp } => {
                positive(amount)?;
                if self.available() < amount {
                    return Err(BalanceV2Error::InsufficientAvailableBalance);
                };
                (checked_sub(self.available(), amount)?, self.frozen(), self.status(), timestamp)
            }
            BalanceV2Event::Close { timestamp } => {
                if self.available() != Decimal::ZERO || self.frozen() != Decimal::ZERO {
                    return Err(BalanceV2Error::CannotCloseNonZeroBalance);
                };
                (self.available(), self.frozen(), BalanceV2Status::Closed, timestamp)
            }
        };
        let version = self.version().checked_add(1).ok_or(BalanceV2Error::ArithmeticOverflow)?;
        self.available = available;
        self.frozen = frozen;
        self.status = status;
        self.version = version;
        self.updated_at = timestamp;
        Ok((status, ()))
    }
}
fn positive(amount: Decimal) -> Result<(), BalanceV2Error> {
    if amount <= Decimal::ZERO { Err(BalanceV2Error::InvalidAmount) } else { Ok(()) }
}
fn checked_add(a: Decimal, b: Decimal) -> Result<Decimal, BalanceV2Error> {
    a.checked_add(b).ok_or(BalanceV2Error::ArithmeticOverflow)
}
fn checked_sub(a: Decimal, b: Decimal) -> Result<Decimal, BalanceV2Error> {
    a.checked_sub(b).ok_or(BalanceV2Error::ArithmeticOverflow)
}

impl FieldDiff for BalanceV2 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            change("party_id", "", self.party_id),
            change("account_id", "", self.account_id),
            change("coin_idx", "", self.coin_idx),
            EntityFieldChange::new("available", "", self.available.to_string()),
            EntityFieldChange::new("frozen", "", self.frozen.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            change("created_at", "", self.created_at),
            change("updated_at", "", self.updated_at),
        ]
    }
    fn diff(&self, o: &Self) -> Vec<EntityFieldChange> {
        let f = [
            ("party_id", self.party_id.to_string(), o.party_id.to_string()),
            ("account_id", self.account_id.to_string(), o.account_id.to_string()),
            ("coin_idx", self.coin_idx.to_string(), o.coin_idx.to_string()),
            ("available", self.available.to_string(), o.available.to_string()),
            ("frozen", self.frozen.to_string(), o.frozen.to_string()),
            ("status", self.status.as_str().into(), o.status.as_str().into()),
            ("created_at", self.created_at.to_string(), o.created_at.to_string()),
            ("updated_at", self.updated_at.to_string(), o.updated_at.to_string()),
        ];
        f.into_iter()
            .filter(|(_, a, b)| a != b)
            .map(|(n, a, b)| EntityFieldChange::new(n, a, b))
            .collect()
    }
}
fn change(name: &'static str, before: &str, after: u64) -> EntityFieldChange {
    EntityFieldChange::new(name, before, after.to_string())
}
impl common_entity::Shard for BalanceV2 {
    fn partition(&self) -> u8 {
        (self.account_id % 256) as u8
    }
}
impl Entity for BalanceV2 {
    fn entity_id(&self) -> u64 {
        common_entity::stable_string_entity_id(&self.business_entity_id())
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
    fn replay_field_type(f: &str) -> u8 {
        match f {
            "available" | "frozen" => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Fsm;
    use rust_decimal::Decimal;

    use super::{BalanceV2, BalanceV2Error, BalanceV2Event, BalanceV2Status};

    const TIMESTAMP: u64 = 42;

    fn decimal(value: i64) -> Decimal {
        Decimal::from(value)
    }

    fn balance(available: i64, frozen: i64, version: u64) -> BalanceV2 {
        BalanceV2::new(1, 2, 3, decimal(available), decimal(frozen), version, 7)
    }

    fn assert_rejected_without_mutation(
        mut balance: BalanceV2,
        event: BalanceV2Event,
        expected: BalanceV2Error,
    ) {
        let before = balance.clone();
        assert_eq!(balance.transition(event), Err(expected));
        assert_eq!(balance, before);
    }

    #[test]
    fn open_state_accepts_every_event_when_its_business_preconditions_hold() {
        let cases = [
            (
                balance(10, 2, 3),
                BalanceV2Event::Freeze { amount: decimal(4), timestamp: TIMESTAMP },
                decimal(6),
                decimal(6),
                BalanceV2Status::Open,
            ),
            (
                balance(1, 10, 3),
                BalanceV2Event::UnfreezeAndPay {
                    unfreeze_amount: decimal(4),
                    payment_amount: decimal(3),
                    timestamp: TIMESTAMP,
                },
                decimal(5),
                decimal(3),
                BalanceV2Status::Open,
            ),
            (
                balance(1, 10, 3),
                BalanceV2Event::DebitFrozen { amount: decimal(4), timestamp: TIMESTAMP },
                decimal(1),
                decimal(6),
                BalanceV2Status::Open,
            ),
            (
                balance(1, 2, 3),
                BalanceV2Event::CreditAvailable { amount: decimal(4), timestamp: TIMESTAMP },
                decimal(5),
                decimal(2),
                BalanceV2Status::Open,
            ),
            (
                balance(10, 2, 3),
                BalanceV2Event::DebitAvailable { amount: decimal(4), timestamp: TIMESTAMP },
                decimal(6),
                decimal(2),
                BalanceV2Status::Open,
            ),
            (
                balance(0, 0, 3),
                BalanceV2Event::Close { timestamp: TIMESTAMP },
                Decimal::ZERO,
                Decimal::ZERO,
                BalanceV2Status::Closed,
            ),
        ];

        for (mut balance, event, expected_available, expected_frozen, expected_status) in cases {
            let (state, output) = balance.transition(event).unwrap();

            assert_eq!(state, expected_status);
            assert_eq!(output, ());
            assert_eq!(balance.status(), expected_status);
            assert_eq!(balance.available(), expected_available);
            assert_eq!(balance.frozen(), expected_frozen);
            assert_eq!(balance.version(), 4);
            assert_eq!(balance.updated_at(), TIMESTAMP);
        }
    }

    #[test]
    fn open_state_rejects_non_positive_amounts_without_mutation() {
        for amount in [decimal(-1), Decimal::ZERO] {
            for event in [
                BalanceV2Event::Freeze { amount, timestamp: TIMESTAMP },
                BalanceV2Event::UnfreezeAndPay {
                    unfreeze_amount: amount,
                    payment_amount: Decimal::ZERO,
                    timestamp: TIMESTAMP,
                },
                BalanceV2Event::DebitFrozen { amount, timestamp: TIMESTAMP },
                BalanceV2Event::CreditAvailable { amount, timestamp: TIMESTAMP },
                BalanceV2Event::DebitAvailable { amount, timestamp: TIMESTAMP },
            ] {
                assert_rejected_without_mutation(
                    balance(10, 10, 3),
                    event,
                    BalanceV2Error::InvalidAmount,
                );
            }
        }

        assert_rejected_without_mutation(
            balance(10, 10, 3),
            BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: Decimal::ONE,
                payment_amount: decimal(-1),
                timestamp: TIMESTAMP,
            },
            BalanceV2Error::InvalidAmount,
        );
    }

    #[test]
    fn open_state_rejects_insufficient_funds_and_non_zero_close_without_mutation() {
        let cases = [
            (
                balance(1, 10, 3),
                BalanceV2Event::Freeze { amount: decimal(2), timestamp: TIMESTAMP },
                BalanceV2Error::InsufficientAvailableBalance,
            ),
            (
                balance(1, 10, 3),
                BalanceV2Event::DebitAvailable { amount: decimal(2), timestamp: TIMESTAMP },
                BalanceV2Error::InsufficientAvailableBalance,
            ),
            (
                balance(10, 1, 3),
                BalanceV2Event::DebitFrozen { amount: decimal(2), timestamp: TIMESTAMP },
                BalanceV2Error::InsufficientFrozenBalance,
            ),
            (
                balance(10, 2, 3),
                BalanceV2Event::UnfreezeAndPay {
                    unfreeze_amount: decimal(2),
                    payment_amount: Decimal::ONE,
                    timestamp: TIMESTAMP,
                },
                BalanceV2Error::InsufficientFrozenBalance,
            ),
            (
                balance(1, 0, 3),
                BalanceV2Event::Close { timestamp: TIMESTAMP },
                BalanceV2Error::CannotCloseNonZeroBalance,
            ),
            (
                balance(0, 1, 3),
                BalanceV2Event::Close { timestamp: TIMESTAMP },
                BalanceV2Error::CannotCloseNonZeroBalance,
            ),
        ];

        for (balance, event, expected) in cases {
            assert_rejected_without_mutation(balance, event, expected);
        }
    }

    #[test]
    fn closed_state_rejects_every_event_without_mutation() {
        let mut closed = balance(0, 0, 3);
        closed.transition(BalanceV2Event::Close { timestamp: 8 }).unwrap();

        for event in [
            BalanceV2Event::Close { timestamp: TIMESTAMP },
            BalanceV2Event::Freeze { amount: Decimal::ONE, timestamp: TIMESTAMP },
            BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: Decimal::ONE,
                payment_amount: Decimal::ZERO,
                timestamp: TIMESTAMP,
            },
            BalanceV2Event::DebitFrozen { amount: Decimal::ONE, timestamp: TIMESTAMP },
            BalanceV2Event::CreditAvailable { amount: Decimal::ONE, timestamp: TIMESTAMP },
            BalanceV2Event::DebitAvailable { amount: Decimal::ONE, timestamp: TIMESTAMP },
        ] {
            assert_rejected_without_mutation(closed.clone(), event, BalanceV2Error::BalanceClosed);
        }
    }

    #[test]
    fn arithmetic_overflow_rejects_transition_without_mutation() {
        assert_rejected_without_mutation(
            balance(1, 0, u64::MAX),
            BalanceV2Event::CreditAvailable { amount: Decimal::ONE, timestamp: TIMESTAMP },
            BalanceV2Error::ArithmeticOverflow,
        );

        let mut decimal_max = balance(0, 0, 3);
        decimal_max.available = Decimal::MAX;
        assert_rejected_without_mutation(
            decimal_max,
            BalanceV2Event::CreditAvailable { amount: Decimal::ONE, timestamp: TIMESTAMP },
            BalanceV2Error::ArithmeticOverflow,
        );
    }

    #[test]
    fn apply_commits_the_same_transition_while_discarding_output() {
        let event = BalanceV2Event::Freeze { amount: decimal(4), timestamp: TIMESTAMP };
        let mut transitioned = balance(10, 2, 3);
        let mut applied = transitioned.clone();

        transitioned.transition(event.clone()).unwrap();
        applied.apply(event).unwrap();

        assert_eq!(applied, transitioned);
    }
}
