use std::collections::HashSet;

use common_entity::{EntityError, LogEntry, UseCase};
use example_core_entity::spot::spot_order_v7::SpotOrderV7;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::spot_order_domain_service::{
    MatchSpotOrderV3Input, MatchSpotOrderV3Outcome, SpotOrderDomainService,
};
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
pub type MatchSpotOrderV3AfterChanges = MatchSpotOrderV3Outcome;
pub type MatchSpotOrderV3Changes = MatchSpotOrderV3Outcome;
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
        if s.taker_order.validate_matchable_as_taker().is_err() {
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
            if x.validate_matchable_as_maker().is_err() {
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
        SpotOrderDomainService::match_order(
            s.taker_order.clone(),
            s.maker_orders.clone(),
            MatchSpotOrderV3Input {
                trade_id_start: s.trade_id_start,
                maker_fee_bps: s.maker_fee_bps,
                taker_fee_bps: s.taker_fee_bps,
                timestamp: c.logical_ts(),
            },
        )
        .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntry, LogEntrySequencer, UseCase};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::{OrderBase, PriceType, UserId};
    use example_core_entity::{OrderSide, OrderTif, SpotOrderStatus};
    use rust_decimal::Decimal;

    use super::*;

    fn order(id: u64, side: OrderSide, status: SpotOrderStatus, size: i64) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(id + 10),
            order_id: id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(100),
            size: Decimal::from(size),
            orig_size: Decimal::from(size),
            tif: OrderTif::Gtc,
            status,
            execution: PriceType::Limit,
            version: 3,
            ..OrderBase::default()
        })
        .unwrap()
    }

    fn command() -> LogEntry<MatchSpotOrderV3Cmd> {
        LogEntrySequencer::from_metadata(1, 500, 77, 1, MatchSpotOrderV3Cmd { order_id: 1 })
    }

    #[test]
    fn open_and_partially_filled_takers_use_domain_service_outcome() {
        for status in [SpotOrderStatus::Open, SpotOrderStatus::PartiallyFilled] {
            let state = MatchSpotOrderV3State {
                taker_order: order(1, OrderSide::Buy, status, 2),
                maker_orders: vec![order(2, OrderSide::Sell, SpotOrderStatus::Open, 1)],
                trade_id_start: 900,
                maker_fee_bps: 0,
                taker_fee_bps: 0,
            };
            let outcome =
                MatchSpotOrderV3UseCase.compute_state_changed(&command(), &state).unwrap();
            assert!(matches!(outcome, MatchSpotOrderV3Outcome::PartiallyFilled { .. }));
        }
    }

    #[test]
    fn pending_taker_is_rejected_by_state_validation() {
        let state = MatchSpotOrderV3State {
            taker_order: order(1, OrderSide::Buy, SpotOrderStatus::Pending, 2),
            maker_orders: vec![],
            trade_id_start: 900,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        };
        assert_eq!(
            MatchSpotOrderV3UseCase.validate_state_given(&command(), &state),
            Err(MatchSpotOrderV3Error::TakerOrderNotMatchable)
        );
    }
}
