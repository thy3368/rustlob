use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges,
    UpdatedEntityPair, UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::{
    MatchPerpOrderV3Input, PerpOrderV3, PerpOrderV3Error, PerpOrderV3Event, PerpTradeV3,
    SpotOrderStatus, UserId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchHyperliquidPerpOrderCmd {
    pub operator: UserId,
    pub taker_order_id: u64,
    pub match_id: u64,
    pub executed_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchHyperliquidPerpOrderState {
    pub taker_order: PerpOrderV3,
    pub maker_orders: Vec<PerpOrderV3>,
    pub taker_fee_rate: Decimal,
    pub maker_fee_rate: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchHyperliquidPerpOrderError {
    #[error("taker order id and match id must be non-zero")]
    InvalidCommand,
    #[error("taker order does not match command")]
    TakerOrderMismatch,
    #[error("order is not matchable")]
    OrderNotMatchable,
    #[error("maker order is incompatible with taker")]
    IncompatibleMaker,
    #[error("no trades were matched")]
    NoTradesMatched,
    #[error("arithmetic overflow while matching")]
    ArithmeticOverflow,
    #[error(transparent)]
    Order(#[from] PerpOrderV3Error),
    #[error("order FSM did not return a match outcome")]
    MissingOutcome,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MatchHyperliquidPerpOrderUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchHyperliquidPerpOrderChanges {
    pub created_trades: Vec<PerpTradeV3>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<PerpOrderV3>>,
    pub updated_taker_order: UpdatedEntityPair<PerpOrderV3>,
}

impl ReplayableChanges for MatchHyperliquidPerpOrderChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(self.created_trades.len() * 2 + 1);
        for (trade, maker) in self.created_trades.iter().zip(&self.updated_maker_orders) {
            events.push(trade.track_create_event()?);
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        events.push(
            self.updated_taker_order
                .after
                .track_update_event_from(&self.updated_taker_order.before)?,
        );
        Ok(events)
    }
}

impl UseCase for MatchHyperliquidPerpOrderUseCase {
    type Command = LogEntry<MatchHyperliquidPerpOrderCmd>;
    type StateGiven = MatchHyperliquidPerpOrderState;
    type Error = MatchHyperliquidPerpOrderError;
    type StateChanged = MatchHyperliquidPerpOrderChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.taker_order_id == 0 || cmd.match_id == 0 {
            return Err(MatchHyperliquidPerpOrderError::InvalidCommand);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.taker_order.comon_order.order_id != cmd.taker_order_id {
            return Err(MatchHyperliquidPerpOrderError::TakerOrderMismatch);
        }
        validate_matchable(&state.taker_order)?;
        if state.taker_fee_rate < Decimal::ZERO || state.maker_fee_rate < Decimal::ZERO {
            return Err(MatchHyperliquidPerpOrderError::InvalidCommand);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let taker_before = state.taker_order.clone();
        let mut taker = taker_before.clone();
        let (_, outcome) = taker.transition(PerpOrderV3Event::Match {
            makers: state.maker_orders.clone(),
            input: MatchPerpOrderV3Input {
                match_id: cmd.match_id,
                maker_fee_rate: state.maker_fee_rate,
                taker_fee_rate: state.taker_fee_rate,
                timestamp: cmd.executed_at,
            },
            timestamp: cmd.executed_at,
        })?;
        let (taker_after, maker_orders_after, created_trades) =
            outcome.ok_or(MatchHyperliquidPerpOrderError::MissingOutcome)?.into_parts();
        let updated_maker_orders = state
            .maker_orders
            .iter()
            .zip(maker_orders_after)
            .filter(|(before, after)| *before != after)
            .map(|(before, after)| UpdatedEntityPair { before: before.clone(), after })
            .collect();
        Ok(MatchHyperliquidPerpOrderChanges {
            created_trades,
            updated_maker_orders,
            updated_taker_order: UpdatedEntityPair { before: taker_before, after: taker_after },
        })
    }
}

fn validate_matchable(order: &PerpOrderV3) -> Result<(), MatchHyperliquidPerpOrderError> {
    if order.comon_order.size <= Decimal::ZERO
        || order.comon_order.limit_px <= Decimal::ZERO
        || !matches!(
            order.comon_order.status,
            SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        )
    {
        return Err(MatchHyperliquidPerpOrderError::OrderNotMatchable);
    }
    Ok(())
}
