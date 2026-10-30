use std::collections::HashSet;

use common_entity::{EntityError, LogEntry, StateMachineV2};
use example_core_entity::SpotOrderV3;
use example_core_entity::spot::spot_order_v3::{
    MatchResult, MatchSpotOrderV3Input, Pending, SpotOrderV3Any,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderV3State {
    pub taker_order: SpotOrderV3<Pending>,
    pub maker_orders: Vec<SpotOrderV3Any>,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderV3UseCase;

impl StateMachineV2 for MatchSpotOrderV3UseCase {
    type Command = LogEntry<MatchSpotOrderV3Cmd>;
    type StateGiven = MatchSpotOrderV3State;
    type Error = MatchSpotOrderV3Error;
    type StateChanged = MatchResult;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.order_id == 0 {
            return Err(MatchSpotOrderV3Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.taker_order.order_base().order_id != cmd.order_id {
            return Err(MatchSpotOrderV3Error::TakerOrderIdMismatch);
        }
        let taker = state.taker_order.order_base();
        if taker.order_id == 0
            || taker.size <= Decimal::ZERO
            || taker.size > taker.orig_size
            || taker.limit_px <= Decimal::ZERO
            || (taker.status == example_core_entity::SpotOrderStatus::PartiallyFilled
                && taker.size >= taker.orig_size)
            || !taker.has_consistent_execution_facts()
            || taker.status != example_core_entity::SpotOrderStatus::Pending
        {
            return Err(MatchSpotOrderV3Error::TakerOrderNotMatchable);
        }

        let mut maker_ids = HashSet::with_capacity(state.maker_orders.len());
        for maker in &state.maker_orders {
            let maker_base = maker.order_base();
            if maker_base.order_id == 0 {
                return Err(MatchSpotOrderV3Error::InvalidMakerOrderId);
            }
            if !maker_ids.insert(maker_base.order_id) {
                return Err(MatchSpotOrderV3Error::DuplicateMakerOrderId);
            }
            if maker_base.size <= Decimal::ZERO
                || maker_base.size > maker_base.orig_size
                || maker_base.limit_px <= Decimal::ZERO
                || maker_base.execution != example_core_entity::common::CommonOrderExecution::Limit
                || (maker_base.status == example_core_entity::SpotOrderStatus::PartiallyFilled
                    && maker_base.size >= maker_base.orig_size)
                || !maker_base.has_consistent_execution_facts()
                || !matches!(
                    maker_base.status,
                    example_core_entity::SpotOrderStatus::Open
                        | example_core_entity::SpotOrderStatus::PartiallyFilled
                )
            {
                return Err(MatchSpotOrderV3Error::MakerOrderNotMatchable);
            }
            if maker_base.order_id == taker.order_id
                || maker_base.symbol != taker.symbol
                || maker_base.side == taker.side
            {
                return Err(MatchSpotOrderV3Error::MakerOrderMismatch);
            }
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        state
            .taker_order
            .match_with_input(
                state.maker_orders.clone(),
                MatchSpotOrderV3Input {
                    trade_id_start: state.trade_id_start,
                    maker_fee_bps: state.maker_fee_bps,
                    taker_fee_bps: state.taker_fee_bps,
                    timestamp: cmd.logical_ts(),
                },
            )
            .map_err(MatchSpotOrderV3Error::from)
    }
}
