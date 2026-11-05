use std::collections::{BTreeMap, HashSet};

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges,
    UpdatedEntityPair, UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, PerpPositionV3, PerpPositionV3Error,
    PerpPositionV3Event, PerpPositionV3Output, PerpTradeV3, UserId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeCmd {
    pub operator: UserId,
    pub settlement_batch_id: u64,
    pub trade_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeState {
    pub trades: Vec<PerpTradeV3>,
    pub positions: Vec<PerpPositionV3>,
    pub margin_balances: Vec<BalanceV2>,
    pub settled_trade_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleHyperliquidPerpTradeError {
    #[error("settlement batch id must be non-zero")]
    InvalidSettlementBatchId,
    #[error("trade ids must not be empty")]
    EmptyTradeIds,
    #[error("trade ids do not match loaded trades")]
    TradeIdsMismatch,
    #[error("trade was already settled")]
    TradeAlreadySettled,
    #[error("position slot was not found or mismatched")]
    PositionMismatch,
    #[error("margin balance was not found")]
    MarginBalanceNotFound,
    #[error("insufficient available margin")]
    InsufficientAvailableMargin,
    #[error("arithmetic overflow while settling trades")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SettleHyperliquidPerpTradeUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeChanges {
    pub changed_positions: Vec<UpdatedEntityPair<PerpPositionV3>>,
    pub changed_margin_balances: Vec<UpdatedEntityPair<BalanceV2>>,
}

impl ReplayableChanges for SettleHyperliquidPerpTradeChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events =
            Vec::with_capacity(self.changed_positions.len() + self.changed_margin_balances.len());
        for pair in &self.changed_positions {
            if pair.before.version() == 0 && pair.before.is_flat() {
                events.push(pair.after.track_create_event()?);
            } else {
                events.push(pair.after.track_update_event_from(&pair.before)?);
            }
        }
        for pair in &self.changed_margin_balances {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        Ok(events)
    }
}

impl UseCase for SettleHyperliquidPerpTradeUseCase {
    type Command = LogEntry<SettleHyperliquidPerpTradeCmd>;
    type StateGiven = SettleHyperliquidPerpTradeState;
    type Error = SettleHyperliquidPerpTradeError;
    type StateChanged = SettleHyperliquidPerpTradeChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.settlement_batch_id == 0 {
            return Err(SettleHyperliquidPerpTradeError::InvalidSettlementBatchId);
        }
        if cmd.trade_ids.is_empty() {
            return Err(SettleHyperliquidPerpTradeError::EmptyTradeIds);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if cmd.trade_ids.len() != state.trades.len()
            || cmd
                .trade_ids
                .iter()
                .zip(&state.trades)
                .any(|(id, trade)| *id != trade.trade_base.trade_id)
        {
            return Err(SettleHyperliquidPerpTradeError::TradeIdsMismatch);
        }
        let settled: HashSet<u64> = state.settled_trade_ids.iter().copied().collect();
        if cmd.trade_ids.iter().any(|id| settled.contains(id)) {
            return Err(SettleHyperliquidPerpTradeError::TradeAlreadySettled);
        }
        derive_changes(cmd, state).map(|_| ())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        derive_changes(cmd, state)
    }
}

fn derive_changes(
    cmd: &LogEntry<SettleHyperliquidPerpTradeCmd>,
    state: &SettleHyperliquidPerpTradeState,
) -> Result<SettleHyperliquidPerpTradeChanges, SettleHyperliquidPerpTradeError> {
    let mut positions: BTreeMap<(u64, u32), PerpPositionV3> = state
        .positions
        .iter()
        .cloned()
        .map(|position| ((position.user_id().0, position.coin().index()), position))
        .collect();
    let mut balance_deltas: BTreeMap<u64, Decimal> = BTreeMap::new();
    let mut changed_positions = Vec::with_capacity(state.trades.len() * 2);

    for trade in &state.trades {
        for (user, fee) in [
            (trade.trade_base.taker_user, trade.trade_base.taker_fee),
            (trade.trade_base.maker_user, trade.trade_base.maker_fee),
        ] {
            let key = (user.0, trade.trade_base.symbol.base.index());
            let before = positions
                .get(&key)
                .cloned()
                .ok_or(SettleHyperliquidPerpTradeError::PositionMismatch)?;
            let mut after = before.clone();
            let (_, output) = after
                .transition(PerpPositionV3Event::TradeFilled {
                    trade: trade.clone(),
                    logical_ts: cmd.logical_ts(),
                })
                .map_err(map_position_error)?;
            let PerpPositionV3Output::TradeApplied { closed_pnl } = output else {
                return Err(SettleHyperliquidPerpTradeError::PositionMismatch);
            };
            positions.insert(key, after.clone());
            changed_positions.push(UpdatedEntityPair { before, after });

            let delta = closed_pnl
                .checked_sub(fee)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            let total = balance_deltas.entry(user.0).or_default();
            *total = total
                .checked_add(delta)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
        }
    }

    let mut changed_margin_balances = Vec::with_capacity(balance_deltas.len());
    for (account_id, delta) in balance_deltas {
        if delta.is_zero() {
            continue;
        }
        let before = state
            .margin_balances
            .iter()
            .find(|balance| balance.account_id() == account_id)
            .cloned()
            .ok_or(SettleHyperliquidPerpTradeError::MarginBalanceNotFound)?;
        let mut after = before.clone();
        let event = if delta.is_sign_positive() {
            BalanceV2Event::CreditAvailable { amount: delta, timestamp: cmd.logical_ts() }
        } else {
            BalanceV2Event::DebitAvailable { amount: delta.abs(), timestamp: cmd.logical_ts() }
        };
        after.apply(event).map_err(map_balance_error)?;
        changed_margin_balances.push(UpdatedEntityPair { before, after });
    }

    Ok(SettleHyperliquidPerpTradeChanges { changed_positions, changed_margin_balances })
}

fn map_position_error(_: PerpPositionV3Error) -> SettleHyperliquidPerpTradeError {
    SettleHyperliquidPerpTradeError::PositionMismatch
}

fn map_balance_error(error: BalanceV2Error) -> SettleHyperliquidPerpTradeError {
    match error {
        BalanceV2Error::InsufficientAvailableBalance => {
            SettleHyperliquidPerpTradeError::InsufficientAvailableMargin
        }
        _ => SettleHyperliquidPerpTradeError::ArithmeticOverflow,
    }
}
