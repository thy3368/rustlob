use common_entity::{Fsm, UpdatedEntityPair};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::balance_v2::BalanceV2;
use crate::account::coin::Coin;

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
}

/// 一条一次性的余额账务流水，先记录操作，成功校验余额变化后补齐快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceLedgerV3 {
    pub entry_id: u64,
    pub party_id: u64,
    pub account_id: u64,
    pub coin: Coin,
    pub operation: LedgerOperation,
    pub amount: Decimal,
    pub before_available: Option<Decimal>,
    pub before_frozen: Option<Decimal>,
    pub after_available: Option<Decimal>,
    pub after_frozen: Option<Decimal>,
    pub created_at: u64,
    pub version: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerOperation {
    Freeze,
    UnfreezePay,
}

impl LedgerOperation {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Freeze => "freeze",
            Self::UnfreezePay => "unfreeze_pay",
        }
    }
}

impl BalanceLedgerV3 {
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
            || before.coin_idx() != self.coin.index() as u64
            || after.coin_idx() != self.coin.index() as u64
        {
            return Err(BalanceLedgerV3Error::BalanceIdentityMismatch);
        }
        if after.version()
            != before.version().checked_add(1).ok_or(BalanceLedgerV3Error::VersionOverflow)?
        {
            return Err(BalanceLedgerV3Error::BalanceVersionMismatch);
        }
        if applied_at < self.created_at
            || applied_at < before.updated_at()
            || after.updated_at() != applied_at
        {
            return Err(BalanceLedgerV3Error::TimeWentBackwards);
        }
        let (ba, bf, aa, af) =
            (before.available(), before.frozen(), after.available(), after.frozen());
        match self.operation {
            LedgerOperation::Freeze => {
                let expected_a =
                    ba.checked_sub(self.amount).ok_or(BalanceLedgerV3Error::ArithmeticOverflow)?;
                let expected_f =
                    bf.checked_add(self.amount).ok_or(BalanceLedgerV3Error::ArithmeticOverflow)?;
                if expected_a != aa || expected_f != af {
                    return Err(BalanceLedgerV3Error::OperationMismatch);
                }
            }
            LedgerOperation::UnfreezePay => {
                let delta = aa.checked_sub(ba).ok_or(BalanceLedgerV3Error::ArithmeticOverflow)?;
                let expected_f =
                    bf.checked_sub(self.amount).ok_or(BalanceLedgerV3Error::ArithmeticOverflow)?;
                if delta < Decimal::ZERO || expected_f != af || af < Decimal::ZERO {
                    return Err(BalanceLedgerV3Error::OperationMismatch);
                }
            }
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
