use common_entity::fsm_v2::{Fsm, SequencedCommand};
use common_entity::{Entity, EntityError, EntityFieldChange, TrackDiff};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::support::concat3;

const BALANCE_V2_ENTITY_TYPE: u8 = 12;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BalanceV2Error {
    #[error("replay projection failed: {0}")]
    Replay(#[from] EntityError),
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
    #[error("incoming lsn {incoming_lsn} is older than current lsn {current_lsn}")]
    StaleLsn { incoming_lsn: u64, current_lsn: u64 },
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
pub enum BalanceV2Command {
    Freeze {
        amount: Decimal,
    },
    UnfreezeAndPay {
        unfreeze_amount: Decimal,
        payment_amount: Decimal,
    },
    ///todo DebitFrozen应该没有用
    DebitFrozen {
        amount: Decimal,
    },
    CreditAvailable {
        amount: Decimal,
    },
    DebitAvailable {
        amount: Decimal,
    },
    Close,
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
    lsn: u64,
    command_index: u32,
    created_at: u64,
    updated_at: u64,
}
impl BalanceV2 {
    pub fn with_position(mut self, lsn: u64, command_index: u32) -> Self {
        self.lsn = lsn;
        self.command_index = command_index;
        self
    }
    /// 返回账户与币种组成的稳定业务身份。
    pub fn business_entity_id(&self) -> String {
        concat3(
            &self.account_id.to_string(),
            ":",
            &self.coin_idx.to_string(),
        )
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
    pub fn position(&self) -> (u64, u32) {
        (self.lsn, self.command_index)
    }
    pub fn version(&self) -> u64 {
        self.lsn
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
        lsn: u64,
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
            lsn,
            command_index: 0,
            created_at,
            updated_at: created_at,
        }
    }
}

impl Fsm for BalanceV2 {
    type Command = BalanceV2Command;
    type Error = BalanceV2Error;
    fn derive_transition(
        &self,
        command: SequencedCommand<Self::Command>,
    ) -> Result<Self, Self::Error> {
        let (metadata, command) = command.into_parts();
        if self.status() == BalanceV2Status::Closed {
            return Err(BalanceV2Error::BalanceClosed);
        }
        let position = (metadata.lsn, metadata.command_index);
        if position < (self.lsn, self.command_index) {
            return Err(BalanceV2Error::StaleLsn {
                incoming_lsn: position.0,
                current_lsn: self.lsn,
            });
        }
        let (available, frozen, status) = match command {
            BalanceV2Command::Freeze { amount } => {
                positive(amount)?;
                if self.available() < amount {
                    return Err(BalanceV2Error::InsufficientAvailableBalance);
                };
                (
                    checked_sub(self.available(), amount)?,
                    checked_add(self.frozen(), amount)?,
                    self.status(),
                )
            }
            BalanceV2Command::UnfreezeAndPay {
                unfreeze_amount,
                payment_amount,
            } => {
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
                )
            }
            BalanceV2Command::DebitFrozen { amount } => {
                positive(amount)?;
                if self.frozen() < amount {
                    return Err(BalanceV2Error::InsufficientFrozenBalance);
                };
                (
                    self.available(),
                    checked_sub(self.frozen(), amount)?,
                    self.status(),
                )
            }
            BalanceV2Command::CreditAvailable { amount } => {
                positive(amount)?;
                (
                    checked_add(self.available(), amount)?,
                    self.frozen(),
                    self.status(),
                )
            }
            BalanceV2Command::DebitAvailable { amount } => {
                positive(amount)?;
                if self.available() < amount {
                    return Err(BalanceV2Error::InsufficientAvailableBalance);
                };
                (
                    checked_sub(self.available(), amount)?,
                    self.frozen(),
                    self.status(),
                )
            }
            BalanceV2Command::Close => {
                if self.available() != Decimal::ZERO || self.frozen() != Decimal::ZERO {
                    return Err(BalanceV2Error::CannotCloseNonZeroBalance);
                };
                (self.available(), self.frozen(), BalanceV2Status::Closed)
            }
        };
        let after = Self {
            party_id: self.party_id,
            account_id: self.account_id,
            coin_idx: self.coin_idx,
            available,
            frozen,
            status,
            lsn: position.0,
            command_index: position.1,
            created_at: self.created_at,
            updated_at: metadata.logical_ts,
        };
        Ok(after)
    }

    fn event_type(command: &Self::Command) -> u8 {
        match command {
            BalanceV2Command::Freeze { .. } => 1,
            BalanceV2Command::UnfreezeAndPay { .. } => 2,
            BalanceV2Command::DebitFrozen { .. } => 3,
            BalanceV2Command::CreditAvailable { .. } => 4,
            BalanceV2Command::DebitAvailable { .. } => 5,
            BalanceV2Command::Close => 6,
        }
    }
}
fn positive(amount: Decimal) -> Result<(), BalanceV2Error> {
    if amount <= Decimal::ZERO {
        Err(BalanceV2Error::InvalidAmount)
    } else {
        Ok(())
    }
}
fn checked_add(a: Decimal, b: Decimal) -> Result<Decimal, BalanceV2Error> {
    a.checked_add(b).ok_or(BalanceV2Error::ArithmeticOverflow)
}
fn checked_sub(a: Decimal, b: Decimal) -> Result<Decimal, BalanceV2Error> {
    a.checked_sub(b).ok_or(BalanceV2Error::ArithmeticOverflow)
}

impl TrackDiff for BalanceV2 {
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
            (
                "party_id",
                self.party_id.to_string(),
                o.party_id.to_string(),
            ),
            (
                "account_id",
                self.account_id.to_string(),
                o.account_id.to_string(),
            ),
            (
                "coin_idx",
                self.coin_idx.to_string(),
                o.coin_idx.to_string(),
            ),
            (
                "available",
                self.available.to_string(),
                o.available.to_string(),
            ),
            ("frozen", self.frozen.to_string(), o.frozen.to_string()),
            (
                "status",
                self.status.as_str().into(),
                o.status.as_str().into(),
            ),
            (
                "created_at",
                self.created_at.to_string(),
                o.created_at.to_string(),
            ),
            (
                "updated_at",
                self.updated_at.to_string(),
                o.updated_at.to_string(),
            ),
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

    fn owner_id(&self) -> String {
        todo!()
    }

    fn op_id(&self) -> String {
        todo!()
    }

    fn created_at(&self) -> u64 {
        todo!()
    }

    fn updated_at(&self) -> u64 {
        todo!()
    }

    fn is_derived() -> bool {
        true
    }
    fn entity_type() -> u8 {
        BALANCE_V2_ENTITY_TYPE
    }
    fn lsn(&self) -> u64 {
        self.lsn
    }
    fn command_index(&self) -> u32 {
        self.command_index
    }
    fn replay_field_type(f: &str) -> u8 {
        match f {
            "available" | "frozen" => 1,
            _ => 0,
        }
    }
}

impl common_entity::PositionedEntity for BalanceV2 {
    fn entity_id(&self) -> u64 {
        <Self as Entity>::entity_id(self)
    }

    fn entity_type(&self) -> u8 {
        <Self as Entity>::entity_type()
    }

    fn lsn(&self) -> u64 {
        self.lsn
    }

    fn command_index(&self) -> u32 {
        self.command_index
    }
}

#[cfg(test)]
mod tests {
    use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
    use rust_decimal::Decimal;

    use super::{BalanceV2, BalanceV2Command, BalanceV2Error, BalanceV2Status};

    const LOGICAL_TS: u64 = 42;
    const LSN: u64 = 4;

    fn decimal(value: i64) -> Decimal {
        Decimal::from(value)
    }

    fn balance(available: i64, frozen: i64, version: u64) -> BalanceV2 {
        BalanceV2::new(1, 2, 3, decimal(available), decimal(frozen), version, 7)
    }

    fn sequenced(command: BalanceV2Command) -> SequencedCommand<BalanceV2Command> {
        SequencedCommand::new(
            CommandMetadata::new(LSN, LOGICAL_TS, 0, 2, "1".to_owned()),
            command,
        )
    }

    fn assert_rejected_without_mutation(
        mut balance: BalanceV2,
        command: BalanceV2Command,
        expected: BalanceV2Error,
    ) {
        let before = balance.clone();
        assert_eq!(
            FsmV2::transition(&mut balance, sequenced(command)),
            Err(expected)
        );
        assert_eq!(balance, before);
    }

    #[test]
    fn open_state_accepts_every_command_and_projects_stable_event_types() {
        let cases = [
            (
                balance(10, 2, 3),
                BalanceV2Command::Freeze { amount: decimal(4) },
                decimal(6),
                decimal(6),
                BalanceV2Status::Open,
                1,
            ),
            (
                balance(1, 10, 3),
                BalanceV2Command::UnfreezeAndPay {
                    unfreeze_amount: decimal(4),
                    payment_amount: decimal(3),
                },
                decimal(5),
                decimal(3),
                BalanceV2Status::Open,
                2,
            ),
            (
                balance(1, 10, 3),
                BalanceV2Command::DebitFrozen { amount: decimal(4) },
                decimal(1),
                decimal(6),
                BalanceV2Status::Open,
                3,
            ),
            (
                balance(1, 2, 3),
                BalanceV2Command::CreditAvailable { amount: decimal(4) },
                decimal(5),
                decimal(2),
                BalanceV2Status::Open,
                4,
            ),
            (
                balance(10, 2, 3),
                BalanceV2Command::DebitAvailable { amount: decimal(4) },
                decimal(6),
                decimal(2),
                BalanceV2Status::Open,
                5,
            ),
            (
                balance(0, 0, 3),
                BalanceV2Command::Close,
                Decimal::ZERO,
                Decimal::ZERO,
                BalanceV2Status::Closed,
                6,
            ),
        ];

        for (balance, command, available, frozen, status, event_type) in cases {
            let before = balance.clone();
            let (after, event) =
                FsmV2::transition(&balance, sequenced(command)).expect("valid balance transition");

            assert_eq!(balance, before);
            assert_eq!(
                (after.available(), after.frozen(), after.status()),
                (available, frozen, status)
            );
            assert_eq!((after.version(), after.updated_at()), (LSN, LOGICAL_TS));
            assert_eq!(event.domain_event_type, event_type);
            assert_eq!(event.previous_lsn, 3);
            assert_eq!(event.lsn, LSN);
            assert_eq!(event.logical_ts, LOGICAL_TS);
            assert!(!event.field_changes.is_empty());
        }
    }

    #[test]
    fn command_payload_has_no_independent_sequence_metadata() {
        assert_eq!(
            BalanceV2Command::Freeze {
                amount: Decimal::ONE
            },
            BalanceV2Command::Freeze {
                amount: Decimal::ONE
            }
        );
        let command = sequenced(BalanceV2Command::Close);
        assert_eq!(
            (command.metadata.lsn, command.metadata.logical_ts),
            (LSN, LOGICAL_TS)
        );
    }

    #[test]
    fn rejects_invalid_amounts_insufficient_funds_and_non_zero_close_atomically() {
        for amount in [decimal(-1), Decimal::ZERO] {
            for command in [
                BalanceV2Command::Freeze { amount },
                BalanceV2Command::UnfreezeAndPay {
                    unfreeze_amount: amount,
                    payment_amount: Decimal::ZERO,
                },
                BalanceV2Command::DebitFrozen { amount },
                BalanceV2Command::CreditAvailable { amount },
                BalanceV2Command::DebitAvailable { amount },
            ] {
                assert_rejected_without_mutation(
                    balance(10, 10, 3),
                    command,
                    BalanceV2Error::InvalidAmount,
                );
            }
        }

        let cases = [
            (
                balance(1, 10, 3),
                BalanceV2Command::Freeze { amount: decimal(2) },
                BalanceV2Error::InsufficientAvailableBalance,
            ),
            (
                balance(1, 10, 3),
                BalanceV2Command::DebitAvailable { amount: decimal(2) },
                BalanceV2Error::InsufficientAvailableBalance,
            ),
            (
                balance(10, 1, 3),
                BalanceV2Command::DebitFrozen { amount: decimal(2) },
                BalanceV2Error::InsufficientFrozenBalance,
            ),
            (
                balance(10, 2, 3),
                BalanceV2Command::UnfreezeAndPay {
                    unfreeze_amount: decimal(2),
                    payment_amount: Decimal::ONE,
                },
                BalanceV2Error::InsufficientFrozenBalance,
            ),
            (
                balance(1, 0, 3),
                BalanceV2Command::Close,
                BalanceV2Error::CannotCloseNonZeroBalance,
            ),
        ];
        for (balance, command, error) in cases {
            assert_rejected_without_mutation(balance, command, error);
        }
    }

    #[test]
    fn rejects_closed_state_stale_lsn_and_overflow_atomically() {
        let open = balance(0, 0, 3);
        let (closed, _) =
            FsmV2::transition(&open, sequenced(BalanceV2Command::Close)).expect("close balance");

        assert_rejected_without_mutation(
            closed,
            BalanceV2Command::CreditAvailable {
                amount: Decimal::ONE,
            },
            BalanceV2Error::BalanceClosed,
        );

        let mut stale = balance(1, 0, 5);
        let before = stale.clone();
        let result = FsmV2::transition(
            &mut stale,
            sequenced(BalanceV2Command::DebitAvailable {
                amount: Decimal::ONE,
            }),
        );
        assert_eq!(
            result,
            Err(BalanceV2Error::StaleLsn {
                incoming_lsn: LSN,
                current_lsn: 5,
            })
        );
        assert_eq!(stale, before);

        let mut decimal_max = balance(0, 0, 3);
        decimal_max.available = Decimal::MAX;
        assert_rejected_without_mutation(
            decimal_max,
            BalanceV2Command::CreditAvailable {
                amount: Decimal::ONE,
            },
            BalanceV2Error::ArithmeticOverflow,
        );
    }
}
