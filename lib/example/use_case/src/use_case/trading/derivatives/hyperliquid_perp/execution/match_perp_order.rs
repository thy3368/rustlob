use common_entity::{
    Entity, EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, UpdatedEntityPair,
    UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::common::common_trade::TradeBase;
use crate::entity::{OrderSide, PerpOrderV3, PerpTradeKind, PerpTradeV3, SpotOrderStatus, UserId};

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
        for maker in &state.maker_orders {
            validate_matchable(maker)?;
            if maker.comon_order.order_id == cmd.taker_order_id
                || maker.comon_order.side == state.taker_order.comon_order.side
                || maker.comon_order.symbol != state.taker_order.comon_order.symbol
                || maker.comon_order.execution != crate::entity::common::CommonOrderExecution::Limit
            {
                return Err(MatchHyperliquidPerpOrderError::IncompatibleMaker);
            }
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut taker = state.taker_order.clone();
        let taker_before = taker.clone();
        let mut created_trades = Vec::new();
        let mut updated_maker_orders = Vec::new();

        for (index, maker_before) in state.maker_orders.iter().enumerate() {
            if taker.comon_order.size.is_zero() {
                break;
            }
            if !crosses(
                taker.comon_order.side,
                taker.comon_order.limit_px,
                maker_before.comon_order.limit_px,
            ) {
                break;
            }
            let fill = taker.comon_order.size.min(maker_before.comon_order.size);
            if fill <= Decimal::ZERO {
                continue;
            }
            let mut maker_after = maker_before.clone();
            apply_fill(&mut maker_after, fill, cmd.executed_at)?;
            apply_fill(&mut taker, fill, cmd.executed_at)?;
            let trade_id = cmd
                .match_id
                .checked_mul(1_000_000)
                .and_then(|base| base.checked_add(index as u64 + 1))
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            let notional = fill
                .checked_mul(maker_before.comon_order.limit_px)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            let taker_fee = notional
                .checked_mul(state.taker_fee_rate)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            let maker_fee = notional
                .checked_mul(state.maker_fee_rate)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            created_trades.push(PerpTradeV3 {
                trade_base: TradeBase {
                    trade_id,
                    symbol: taker.comon_order.symbol,
                    taker_user: taker.comon_order.owner,
                    maker_user: maker_before.comon_order.owner,
                    taker_order_id: taker.comon_order.order_id,
                    maker_order_id: maker_before.comon_order.order_id,
                    taker_side: taker.comon_order.side,
                    price: maker_before.comon_order.limit_px,
                    size: fill,
                    taker_fee,
                    maker_fee,
                    created_at: cmd.executed_at,
                    updated_at: cmd.executed_at,
                    version: 0,
                },
                kind: if taker.is_liquidation {
                    PerpTradeKind::Liquidation
                } else {
                    PerpTradeKind::Normal
                },
                reference_id: Some(cmd.match_id.to_string()),
                taker_start_position: Decimal::ZERO,
                maker_start_position: Decimal::ZERO,
                taker_closed_pnl: Decimal::ZERO,
                maker_closed_pnl: Decimal::ZERO,
            });
            updated_maker_orders
                .push(UpdatedEntityPair { before: maker_before.clone(), after: maker_after });
        }
        if created_trades.is_empty() {
            return Err(MatchHyperliquidPerpOrderError::NoTradesMatched);
        }
        taker.comon_order.version = taker_before
            .comon_order
            .version
            .checked_add(1)
            .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
        Ok(MatchHyperliquidPerpOrderChanges {
            created_trades,
            updated_maker_orders,
            updated_taker_order: UpdatedEntityPair { before: taker_before, after: taker },
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

fn crosses(side: OrderSide, taker_price: Decimal, maker_price: Decimal) -> bool {
    match side {
        OrderSide::Buy => taker_price >= maker_price,
        OrderSide::Sell => taker_price <= maker_price,
    }
}

fn apply_fill(
    order: &mut PerpOrderV3,
    fill: Decimal,
    timestamp: u64,
) -> Result<(), MatchHyperliquidPerpOrderError> {
    order.comon_order.size = order
        .comon_order
        .size
        .checked_sub(fill)
        .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
    order.comon_order.status = if order.comon_order.size.is_zero() {
        SpotOrderStatus::Filled
    } else {
        SpotOrderStatus::PartiallyFilled
    };
    order.comon_order.version = order
        .comon_order
        .version
        .checked_add(1)
        .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
    order.comon_order.updated_at = timestamp;
    Ok(())
}
