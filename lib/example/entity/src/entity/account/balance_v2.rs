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
    coin_idx: u64,
    available: Decimal,
    frozen: Decimal,
    status: BalanceV2Status,
    version: u64,
    created_at: u64,
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
