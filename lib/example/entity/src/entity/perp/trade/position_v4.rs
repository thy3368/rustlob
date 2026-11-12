use common_entity::{
    AggregateRole, Entity, EntityFieldChange, EntityMutationModel, FieldDiff, FourColorArchetype,
    Fsm, Shard,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::Signed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::account::coin::Coin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginMode {
    Cross,
    Isolated,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leverage {
    pub leverage_type: MarginMode,
    pub value: u8,
    pub raw_usd: Option<Decimal>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FundingAccum {
    pub all_time: Decimal,
    pub since_open: Decimal,
    pub since_change: Decimal,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionState {
    Empty,
    Open,
    Flat,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillSource {
    Regular,
    Liquidation { liquidation_id: u64 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionFill {
    pub signed_quantity: Decimal,
    pub price: Decimal,
    pub source: FillSource,
}
#[derive(Debug, Clone)]
pub enum PositionEvent {
    TradeFilled { fill: PositionFill, logical_ts: u64 },
    FundingSettled { signed_amount: Decimal, logical_ts: u64 },
    LeverageUpdated { leverage: Leverage, logical_ts: u64 },
    IsolatedMarginAdjusted { delta: Decimal, logical_ts: u64 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeChange {
    Opened,
    Increased,
    Reduced,
    Closed,
    Flipped,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionOutput {
    TradeApplied {
        change: TradeChange,
        realized_pnl: Decimal,
        size_before: Decimal,
        size_after: Decimal,
        source: FillSource,
    },
    FundingApplied {
        signed_amount: Decimal,
    },
    LeverageApplied,
    IsolatedMarginApplied {
        margin_after: Decimal,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PositionError {
    #[error("quantity must be non-zero")]
    InvalidQuantity,
    #[error("price must be greater than zero")]
    InvalidPrice,
    #[error("leverage must be greater than zero")]
    InvalidLeverage,
    #[error("invalid margin configuration")]
    InvalidMarginConfiguration,
    #[error("event is not valid for this state")]
    InvalidState,
    #[error("isolated margin cannot be negative")]
    InvalidIsolatedMargin,
    #[error("arithmetic overflow")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositionV4 {
    position_id: u64,
    account_id: u64,
    coin: Coin,
    state: PositionState,
    szi: Decimal,
    entry_px: Decimal,
    leverage: Leverage,
    cum_funding: FundingAccum,
    version: u64,
    created_at: u64,
    updated_at: u64,
}
impl PositionV4 {
    pub fn empty(position_id: u64, account_id: u64, coin: Coin) -> Self {
        Self {
            position_id,
            account_id,
            coin,
            state: PositionState::Empty,
            szi: Decimal::ZERO,
            entry_px: Decimal::ZERO,
            leverage: Leverage { leverage_type: MarginMode::Cross, value: 1, raw_usd: None },
            cum_funding: FundingAccum::default(),
            version: 0,
            created_at: 0,
            updated_at: 0,
        }
    }
    pub fn new(
        position_id: u64,
        account_id: u64,
        coin: Coin,
        leverage: Leverage,
        created_at: u64,
    ) -> Result<Self, PositionError> {
        validate_leverage(leverage)?;
        let mut p = Self::empty(position_id, account_id, coin);
        p.leverage = leverage;
        p.created_at = created_at;
        p.updated_at = created_at;
        Ok(p)
    }
    pub fn position_id(&self) -> u64 {
        self.position_id
    }
    pub fn account_id(&self) -> u64 {
        self.account_id
    }
    pub fn coin(&self) -> Coin {
        self.coin
    }
    pub fn state(&self) -> PositionState {
        self.state
    }
    pub fn signed_size(&self) -> Decimal {
        self.szi
    }
    pub fn entry_price(&self) -> Decimal {
        self.entry_px
    }
    pub fn leverage(&self) -> Leverage {
        self.leverage
    }
    pub fn cum_funding(&self) -> FundingAccum {
        self.cum_funding
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
    fn next_version(&self) -> Result<u64, PositionError> {
        self.version.checked_add(1).ok_or(PositionError::ArithmeticOverflow)
    }
}
fn validate_leverage(l: Leverage) -> Result<(), PositionError> {
    if l.value == 0 {
        return Err(PositionError::InvalidLeverage);
    }
    match l.leverage_type {
        MarginMode::Cross if l.raw_usd.is_some() => Err(PositionError::InvalidMarginConfiguration),
        MarginMode::Isolated if l.raw_usd.is_none_or(|v| v < Decimal::ZERO) => {
            Err(PositionError::InvalidMarginConfiguration)
        }
        _ => Ok(()),
    }
}

impl Fsm for PositionV4 {
    type State = PositionState;
    type BizEvent = PositionEvent;
    type Output = PositionOutput;
    type Error = PositionError;
    fn transition(
        &mut self,
        event: Self::BizEvent,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        let mut n = self.clone();
        let out = match event {
            PositionEvent::TradeFilled { fill, logical_ts } => {
                if fill.signed_quantity.is_zero() {
                    return Err(PositionError::InvalidQuantity);
                }
                if fill.price <= Decimal::ZERO {
                    return Err(PositionError::InvalidPrice);
                }
                let before = n.szi;
                let same = !before.is_zero() && before.signum() == fill.signed_quantity.signum();
                let total = before
                    .checked_add(fill.signed_quantity)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                let closing = if same || before.is_zero() {
                    Decimal::ZERO
                } else {
                    before.abs().min(fill.signed_quantity.abs())
                };
                let d =
                    fill.price.checked_sub(n.entry_px).ok_or(PositionError::ArithmeticOverflow)?;
                let mut pnl = closing.checked_mul(d).ok_or(PositionError::ArithmeticOverflow)?;
                if before.is_sign_negative() {
                    pnl = -pnl;
                }
                let entry = if total.is_zero() {
                    Decimal::ZERO
                } else if before.is_zero() || (!same && total.signum() != before.signum()) {
                    fill.price
                } else if same {
                    before
                        .abs()
                        .checked_mul(n.entry_px)
                        .and_then(|v| {
                            fill.signed_quantity
                                .abs()
                                .checked_mul(fill.price)
                                .and_then(|w| v.checked_add(w))
                        })
                        .and_then(|v| v.checked_div(total.abs()))
                        .ok_or(PositionError::ArithmeticOverflow)?
                } else {
                    n.entry_px
                };
                n.szi = total;
                n.entry_px = entry;
                n.state = if total.is_zero() { PositionState::Flat } else { PositionState::Open };
                n.version = n.next_version()?;
                n.updated_at = logical_ts;
                n.cum_funding = if total.is_zero() {
                    FundingAccum {
                        since_open: Decimal::ZERO,
                        since_change: Decimal::ZERO,
                        ..n.cum_funding
                    }
                } else if before.is_zero() || (!same && total.signum() != before.signum()) {
                    FundingAccum {
                        since_open: Decimal::ZERO,
                        since_change: Decimal::ZERO,
                        ..n.cum_funding
                    }
                } else {
                    FundingAccum { since_change: Decimal::ZERO, ..n.cum_funding }
                };
                let change = if before.is_zero() {
                    TradeChange::Opened
                } else if total.is_zero() {
                    TradeChange::Closed
                } else if same {
                    TradeChange::Increased
                } else if total.signum() == before.signum() {
                    TradeChange::Reduced
                } else {
                    TradeChange::Flipped
                };
                PositionOutput::TradeApplied {
                    change,
                    realized_pnl: pnl,
                    size_before: before,
                    size_after: total,
                    source: fill.source,
                }
            }
            PositionEvent::FundingSettled { signed_amount, logical_ts } => {
                if n.state != PositionState::Open {
                    return Err(PositionError::InvalidState);
                }
                n.cum_funding.all_time = n
                    .cum_funding
                    .all_time
                    .checked_add(signed_amount)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                n.cum_funding.since_open = n
                    .cum_funding
                    .since_open
                    .checked_add(signed_amount)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                n.cum_funding.since_change = n
                    .cum_funding
                    .since_change
                    .checked_add(signed_amount)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                n.version = n.next_version()?;
                n.updated_at = logical_ts;
                PositionOutput::FundingApplied { signed_amount }
            }
            PositionEvent::LeverageUpdated { leverage, logical_ts } => {
                validate_leverage(leverage)?;
                n.leverage = leverage;
                n.cum_funding.since_change = Decimal::ZERO;
                n.version = n.next_version()?;
                n.updated_at = logical_ts;
                PositionOutput::LeverageApplied
            }
            PositionEvent::IsolatedMarginAdjusted { delta, logical_ts } => {
                if n.state != PositionState::Open
                    || n.leverage.leverage_type != MarginMode::Isolated
                {
                    return Err(PositionError::InvalidMarginConfiguration);
                }
                let cur = n.leverage.raw_usd.ok_or(PositionError::InvalidMarginConfiguration)?;
                let after = cur.checked_add(delta).ok_or(PositionError::ArithmeticOverflow)?;
                if after < Decimal::ZERO {
                    return Err(PositionError::InvalidIsolatedMargin);
                }
                n.leverage.raw_usd = Some(after);
                n.cum_funding.since_change = Decimal::ZERO;
                n.version = n.next_version()?;
                n.updated_at = logical_ts;
                PositionOutput::IsolatedMarginApplied { margin_after: after }
            }
        };
        *self = n;
        Ok((self.state, out))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionRiskSnapshot {
    pub position_value: Decimal,
    pub unrealized_pnl: Decimal,
    pub return_on_equity: Decimal,
    pub liquidation_px: Option<Decimal>,
    pub margin_used: Decimal,
    pub max_leverage: u8,
}
impl PositionV4 {
    pub fn risk_snapshot(
        &self,
        mark: Decimal,
        equity: Decimal,
        max_leverage: u8,
    ) -> Result<PositionRiskSnapshot, PositionError> {
        if mark <= Decimal::ZERO {
            return Err(PositionError::InvalidPrice);
        }
        let value = self.szi.abs().checked_mul(mark).ok_or(PositionError::ArithmeticOverflow)?;
        let pnl = mark
            .checked_sub(self.entry_px)
            .and_then(|v| v.checked_mul(self.szi))
            .ok_or(PositionError::ArithmeticOverflow)?;
        let roe = if equity.is_zero() {
            Decimal::ZERO
        } else {
            pnl.checked_div(equity).ok_or(PositionError::ArithmeticOverflow)?
        };
        let margin = value
            .checked_div(Decimal::from(self.leverage.value))
            .ok_or(PositionError::ArithmeticOverflow)?;
        Ok(PositionRiskSnapshot {
            position_value: value,
            unrealized_pnl: pnl,
            return_on_equity: roe,
            liquidation_px: None,
            margin_used: margin,
            max_leverage,
        })
    }
}
impl FieldDiff for PositionV4 {
    fn diff(&self, o: &Self) -> Vec<EntityFieldChange> {
        [
            ("state", format!("{:?}", self.state), format!("{:?}", o.state)),
            ("szi", self.szi.to_string(), o.szi.to_string()),
            ("entry_px", self.entry_px.to_string(), o.entry_px.to_string()),
            ("version", self.version.to_string(), o.version.to_string()),
        ]
        .into_iter()
        .filter(|(_, a, b)| a != b)
        .map(|(n, a, b)| EntityFieldChange::new(n, a, b))
        .collect()
    }
}
impl Shard for PositionV4 {
    fn partition(&self) -> u8 {
        (self.account_id % 256) as u8
    }
}
impl Entity for PositionV4 {
    fn entity_id(&self) -> u64 {
        self.position_id
    }
    fn is_derived() -> bool {
        false
    }
    fn entity_type() -> u8 {
        0xB4
    }
    fn lsn(&self) -> u64 {
        self.version
    }
    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::PartyPlaceThing
    }
    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::VersionedMutable
    }
    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }
    fn replay_field_type(_: &str) -> u8 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::{Leverage, MarginMode, PositionV4};
    use crate::account::coin::Coin;

    #[test]
    fn constructors_preserve_coin() {
        let empty = PositionV4::empty(1, 2, Coin::BTC);
        assert_eq!(empty.coin(), Coin::BTC);

        let position = PositionV4::new(
            3,
            4,
            Coin::ETH,
            Leverage { leverage_type: MarginMode::Cross, value: 5, raw_usd: None },
            10,
        )
        .expect("valid leverage should construct a position");
        assert_eq!(position.coin(), Coin::ETH);
    }
}
