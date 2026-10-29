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

#[cfg(test)]
mod tests {
    use base_types::UserId;
    use common_entity::{LogEntry, LogEntrySequencer, StateMachineV2};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::{CommonOrderExecution, CommonOrderTrigger, OrderBase};
    use example_core_entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
    use example_core_entity::spot::spot_order_v2::OrderGroupRelation;
    use example_core_entity::spot::spot_order_v3::{MatchResult, SpotOrderV3Any};
    use rust_decimal::Decimal;

    use super::{MatchSpotOrderV3Cmd, MatchSpotOrderV3State, MatchSpotOrderV3UseCase};

    fn order(id: u64, side: OrderSide, px: i64, size: i64) -> SpotOrderV3Any {
        order_with(id, side, px, size, size, OrderTif::Gtc, SpotOrderStatus::Open)
    }

    fn pending_order(id: u64, side: OrderSide, px: i64, size: i64) -> SpotOrderV3Any {
        order_with(id, side, px, size, size, OrderTif::Gtc, SpotOrderStatus::Pending)
    }

    fn order_with(
        id: u64,
        side: OrderSide,
        px: i64,
        size: i64,
        orig_size: i64,
        tif: OrderTif,
        status: SpotOrderStatus,
    ) -> SpotOrderV3Any {
        SpotOrderV3Any::from_record(OrderBase {
            owner: UserId(id),
            order_id: id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(px),
            size: Decimal::from(size),
            orig_size: Decimal::from(orig_size),
            execution: CommonOrderExecution::Limit,
            tif,
            trigger: CommonOrderTrigger::None,
            trigger_px: None,
            group_relation: OrderGroupRelation::Standalone,
            is_position_tpsl: false,
            reduce_only: false,
            status,
            taker_max_fee: Decimal::ZERO,
            fee_consumed: Decimal::ZERO,
            cloid: None,
            created_at: 1,
            updated_at: 1,
            version: 0,
        })
        .unwrap()
    }

    fn entry(order_id: u64, logical_ts: u64) -> LogEntry<MatchSpotOrderV3Cmd> {
        LogEntrySequencer::from_metadata(1, logical_ts, 1, 1, MatchSpotOrderV3Cmd { order_id })
    }

    fn state(taker: SpotOrderV3Any, makers: Vec<SpotOrderV3Any>) -> MatchSpotOrderV3State {
        let SpotOrderV3Any::Pending(taker_order) = taker else {
            panic!("test taker must be pending");
        };
        MatchSpotOrderV3State {
            taker_order,
            maker_orders: makers,
            trade_id_start: 100,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        }
    }

    #[test]
    fn matches_multiple_makers_in_input_order_with_stable_trade_ids() {
        let taker = pending_order(1, OrderSide::Buy, 100, 3);
        let first = order(2, OrderSide::Sell, 99, 1);
        let second = order(3, OrderSide::Sell, 100, 2);
        let mut given_state = state(taker, vec![first, second]);
        given_state.maker_fee_bps = 100;
        given_state.taker_fee_bps = 200;
        let result =
            MatchSpotOrderV3UseCase.compute_state_changed(&entry(1, 77), &given_state).unwrap();
        let MatchResult::Filled { created_trades, .. } = result else {
            panic!("expected filled result");
        };
        assert_eq!(created_trades.len(), 2);
        assert_eq!(created_trades[0].comon_trade.maker_order_id, 2);
        assert_eq!(created_trades[1].comon_trade.maker_order_id, 3);
        assert_eq!(created_trades[0].comon_trade.trade_id, 100);
        assert_eq!(created_trades[1].comon_trade.trade_id, 101);
        assert_eq!(created_trades[0].comon_trade.created_at, 77);
        assert_eq!(created_trades[0].comon_trade.taker_fee, Decimal::new(198, 2));
        assert_eq!(created_trades[0].comon_trade.maker_fee, Decimal::new(99, 2));
    }

    #[test]
    fn preserves_resting_result_without_liquidity() {
        let result = MatchSpotOrderV3UseCase.unwrap();
        assert!(matches!(result, MatchResult::Resting { .. }));
    }

    #[test]
    fn returns_partial_fill_for_gtc_with_remaining_quantity() {
        let result = MatchSpotOrderV3UseCase
            .compute_state_changed(
                &entry(1, 77),
                &state(
                    pending_order(1, OrderSide::Buy, 100, 3),
                    vec![order(2, OrderSide::Sell, 99, 1)],
                ),
            )
            .unwrap();
        let MatchResult::PartiallyFilled { taker_order_after, created_trades, .. } = result else {
            panic!("expected partial fill");
        };
        assert_eq!(taker_order_after.order_base().size, Decimal::from(2));
        assert_eq!(created_trades.len(), 1);
    }

    #[test]
    fn returns_ioc_cancellation_after_partial_fill() {
        let taker = pending_order(1, OrderSide::Buy, 100, 3);
        let result = MatchSpotOrderV3UseCase
            .compute_state_changed(
                &entry(1, 77),
                &state(taker, vec![order(2, OrderSide::Sell, 99, 1)]),
            )
            .unwrap();
        let MatchResult::CanceledAfterPartialFill { canceled_taker_order_after, .. } = result
        else {
            panic!("expected IOC cancellation");
        };
        assert_eq!(canceled_taker_order_after.order_base().status, SpotOrderStatus::Canceled);
    }

    #[test]
    fn rejects_alo_order_that_would_cross() {
        let taker =
            order_with(1, OrderSide::Buy, 100, 1, 1, OrderTif::Alo, SpotOrderStatus::Pending);
        let result = MatchSpotOrderV3UseCase
            .compute_state_changed(
                &entry(1, 77),
                &state(taker, vec![order(2, OrderSide::Sell, 99, 1)]),
            )
            .unwrap();
        assert!(matches!(result, MatchResult::Rejected { .. }));
    }

    #[test]
    fn rejects_duplicate_maker_ids_in_given_state() {
        let use_case = MatchSpotOrderV3UseCase;
        let duplicate = order(2, OrderSide::Sell, 99, 1);
        assert!(matches!(
            use_case.validate_state_given(
                &entry(1, 77),
                &state(
                    pending_order(1, OrderSide::Buy, 100, 2),
                    vec![duplicate.clone(), duplicate]
                ),
            ),
            Err(super::MatchSpotOrderV3Error::DuplicateMakerOrderId)
        ));
    }

    #[test]
    fn rejects_command_and_state_mismatch() {
        let use_case = MatchSpotOrderV3UseCase;
        assert!(matches!(
            use_case.check_command(&entry(0, 1)),
            Err(super::MatchSpotOrderV3Error::InvalidOrderId)
        ));
        assert!(matches!(
            use_case.validate_state_given(
                &entry(9, 1),
                &state(pending_order(1, OrderSide::Buy, 100, 1), vec![])
            ),
            Err(super::MatchSpotOrderV3Error::TakerOrderIdMismatch)
        ));
    }

    #[test]
    fn does_not_mutate_given_state_and_is_deterministic() {
        let state =
            state(pending_order(1, OrderSide::Buy, 100, 1), vec![order(2, OrderSide::Sell, 99, 1)]);
        let command = entry(1, 77);
        let first = MatchSpotOrderV3UseCase.compute_state_changed(&command, &state).unwrap();
        let second = MatchSpotOrderV3UseCase.compute_state_changed(&command, &state).unwrap();
        assert_eq!(first, second);
        assert_eq!(state.taker_order.order_base().size, Decimal::ONE);
        assert_eq!(state.maker_orders[0].order_base().size, Decimal::ONE);
    }
}
