use std::collections::HashSet;

use common_entity::{EntityError, Fsm, LogEntry, UseCase};
use example_core_entity::SpotOrderStatus;
use example_core_entity::common::CommonOrderExecution;
use example_core_entity::spot::spot_order_v7::{
    MatchSpotOrderV7Input, MatchSpotOrderV7Outcome, SpotOrderV7, SpotOrderV7Event,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchSpotOrderV3Cmd {
    pub order_id: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderV3Error {
    #[error("order oid must be greater than zero")]
    InvalidOrderId,
    #[error("taker order id does not match on_command")]
    TakerOrderIdMismatch,
    #[error("maker order id must be greater than zero")]
    InvalidMakerOrderId,
    #[error("maker order ids must be unique")]
    DuplicateMakerOrderId,
    #[error("taker order is not matchable")]
    TakerOrderNotMatchable,
    #[error("maker order is not matchable")]
    MakerOrderNotMatchable,
    #[error("maker order does not belong to the expected market or side")]
    MakerOrderMismatch,
    #[error(transparent)]
    Entity(#[from] EntityError),
}
pub type MatchSpotOrderV3AfterChanges = MatchSpotOrderV7Outcome;
pub type MatchSpotOrderV3Changes = MatchSpotOrderV7Outcome;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderV3State {
    pub taker_order: SpotOrderV7,
    pub maker_orders: Vec<SpotOrderV7>,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderV3UseCase;
impl UseCase for MatchSpotOrderV3UseCase {
    type Command = LogEntry<MatchSpotOrderV3Cmd>;
    type StateGiven = MatchSpotOrderV3State;
    type Error = MatchSpotOrderV3Error;
    type StateChanged = MatchSpotOrderV3AfterChanges;
    fn check_command(&self, c: &Self::Command) -> Result<(), Self::Error> {
        if c.payload().order_id == 0 { Err(MatchSpotOrderV3Error::InvalidOrderId) } else { Ok(()) }
    }
    fn validate_state_given(
        &self,
        c: &Self::Command,
        s: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let t = s.taker_order.order_base();
        if t.order_id != c.payload().order_id {
            return Err(MatchSpotOrderV3Error::TakerOrderIdMismatch);
        }
        if t.size <= Decimal::ZERO
            || t.size > t.orig_size
            || t.limit_px <= Decimal::ZERO
            || !t.has_consistent_execution_facts()
            || t.status != SpotOrderStatus::Pending
        {
            return Err(MatchSpotOrderV3Error::TakerOrderNotMatchable);
        }
        let mut ids = HashSet::new();
        for x in &s.maker_orders {
            let m = x.order_base();
            if m.order_id == 0 {
                return Err(MatchSpotOrderV3Error::InvalidMakerOrderId);
            }
            if !ids.insert(m.order_id) {
                return Err(MatchSpotOrderV3Error::DuplicateMakerOrderId);
            }
            if m.size <= Decimal::ZERO
                || m.size > m.orig_size
                || m.limit_px <= Decimal::ZERO
                || m.execution != CommonOrderExecution::Limit
                || !m.has_consistent_execution_facts()
                || !matches!(m.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
            {
                return Err(MatchSpotOrderV3Error::MakerOrderNotMatchable);
            }
            if m.order_id == t.order_id || m.symbol != t.symbol || m.side == t.side {
                return Err(MatchSpotOrderV3Error::MakerOrderMismatch);
            }
        }
        Ok(())
    }
    fn compute_state_changed_only(
        &self,
        c: &Self::Command,
        s: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut t = s.taker_order.clone();
        let (_, o) = t.transition(SpotOrderV7Event::Match {
            makers: s.maker_orders.clone(),
            input: MatchSpotOrderV7Input {
                trade_id_start: s.trade_id_start,
                maker_fee_bps: s.maker_fee_bps,
                taker_fee_bps: s.taker_fee_bps,
                timestamp: c.logical_ts(),
            },
            timestamp: c.logical_ts(),
        })?;
        o.ok_or_else(|| {
            MatchSpotOrderV3Error::Entity(EntityError::Custom(
                "V7 match produced no outcome".into(),
            ))
        })
    }
}
