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
    #[serde(rename = "asset_id", alias = "coin_idx")]
    coin_idx: u64,
    available: Decimal,
    frozen: Decimal,
    #[serde(default)]
    status: BalanceV2Status,
    version: u64,
    #[serde(default)]
    created_at: u64,
    #[serde(default)]
    updated_at: u64,
}
impl BalanceV2 {
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
    pub fn new(
        party_id: u64,
        account_id: u64,
        coin_idx: u64,
        available: Decimal,
        frozen: Decimal,
        version: u64,
        created_at: u64,
    ) -> Self {
        Self {
            party_id,
            account_id,
            coin_idx,
            available,
            frozen,
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
            change("asset_id", "", self.coin_idx),
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
            ("asset_id", self.coin_idx.to_string(), o.coin_idx.to_string()),
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
        common_entity::stable_string_entity_id(&concat3(
            &self.account_id.to_string(),
            ":",
            &self.coin_idx.to_string(),
        ))
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
    use common_entity::{Entity, FieldDiff, Shard};

    use super::*;

    fn d(value: i64) -> Decimal {
        Decimal::from(value)
    }
    fn balance(available: i64, frozen: i64, version: u64) -> BalanceV2 {
        BalanceV2::new(1, 258, 3, d(available), d(frozen), version, 100)
    }

    #[test]
    fn all_fund_events_transition_and_increment_version() {
        let mut value = balance(100, 20, 1);
        value.apply(BalanceV2Event::Freeze { amount: d(10), timestamp: 101 }).unwrap();
        value
            .apply(BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: d(5),
                payment_amount: d(5),
                timestamp: 102,
            })
            .unwrap();
        value.apply(BalanceV2Event::DebitFrozen { amount: d(10), timestamp: 103 }).unwrap();
        value.apply(BalanceV2Event::CreditAvailable { amount: d(7), timestamp: 104 }).unwrap();
        value.apply(BalanceV2Event::DebitAvailable { amount: d(2), timestamp: 105 }).unwrap();
        assert_eq!((value.available(), value.frozen(), value.version()), (d(100), d(10), 6));
        assert_eq!(value.updated_at(), 105);
    }

    #[test]
    fn errors_are_atomic() {
        let events = [
            BalanceV2Event::Freeze { amount: Decimal::ZERO, timestamp: 200 },
            BalanceV2Event::Freeze { amount: d(101), timestamp: 200 },
            BalanceV2Event::DebitFrozen { amount: d(21), timestamp: 200 },
            BalanceV2Event::DebitAvailable { amount: d(101), timestamp: 200 },
            BalanceV2Event::Close { timestamp: 200 },
        ];
        for event in events {
            let mut value = balance(100, 20, 4);
            let before = value.clone();
            assert!(value.apply(event).is_err());
            assert_eq!(value, before);
        }
        let mut overflow = balance(100, 0, u64::MAX);
        let before = overflow.clone();
        assert_eq!(
            overflow.apply(BalanceV2Event::CreditAvailable { amount: d(1), timestamp: 200 }),
            Err(BalanceV2Error::ArithmeticOverflow)
        );
        assert_eq!(overflow, before);

        let mut decimal_overflow = balance(0, 0, 4);
        decimal_overflow.available = Decimal::MAX;
        let before = decimal_overflow.clone();
        assert_eq!(
            decimal_overflow
                .apply(BalanceV2Event::CreditAvailable { amount: d(1), timestamp: 200 }),
            Err(BalanceV2Error::ArithmeticOverflow)
        );
        assert_eq!(decimal_overflow, before);
    }

    #[test]
    fn close_is_a_terminal_transition() {
        let mut value = balance(0, 0, 7);
        assert_eq!(
            value.transition(BalanceV2Event::Close { timestamp: 200 }),
            Ok((BalanceV2Status::Closed, ()))
        );
        let closed = value.clone();
        let events = [
            BalanceV2Event::Close { timestamp: 201 },
            BalanceV2Event::CreditAvailable { amount: d(1), timestamp: 201 },
        ];
        for event in events {
            assert_eq!(value.apply(event), Err(BalanceV2Error::BalanceClosed));
            assert_eq!(value, closed);
        }
    }

    #[test]
    fn serde_is_flat_and_accepts_open_and_closed_records() {
        let old = serde_json::json!({"party_id":1,"account_id":258,"asset_id":3,"available":"0","frozen":"0","version":7,"created_at":100,"updated_at":100});
        let decoded: BalanceV2 = serde_json::from_value(old).unwrap();
        assert_eq!(decoded.status(), BalanceV2Status::Open);
        let legacy_alias = serde_json::json!({"party_id":1,"account_id":258,"coin_idx":3,"available":"0","frozen":"0","version":7});
        let decoded_alias: BalanceV2 = serde_json::from_value(legacy_alias).unwrap();
        assert_eq!(decoded_alias.coin_idx(), 3);
        let mut closed = decoded;
        closed.apply(BalanceV2Event::Close { timestamp: 200 }).unwrap();
        let json = serde_json::to_value(&closed).unwrap();
        assert_eq!(json["status"], "closed");
        assert!(json.get("record").is_none());
        let roundtrip: BalanceV2 = serde_json::from_value(json).unwrap();
        assert_eq!(roundtrip, closed);
    }

    #[test]
    fn close_diff_and_entity_metadata_remain_stable() {
        let before = balance(0, 0, 7);
        let mut after = before.clone();
        after.apply(BalanceV2Event::Close { timestamp: 200 }).unwrap();
        let changes = before.diff(&after);
        assert!(changes.iter().any(|c| c.field_name == "status"));
        assert_eq!(after.lsn(), 8);
        assert_eq!(after.partition(), 2);
        assert_eq!(BalanceV2::entity_type(), 12);
        assert_eq!(BalanceV2::replay_field_type("available"), 1);
        assert_eq!(before.entity_id(), after.entity_id());
    }
}
