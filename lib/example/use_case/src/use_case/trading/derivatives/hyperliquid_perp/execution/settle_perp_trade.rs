use std::collections::BTreeMap;

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges,
    UpdatedEntityPair, UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, PerpPositionV3, PerpPositionV3Error,
    PerpPositionV3Event, PerpPositionV3Output, PerpTradeV3, PerpTradeV3Error, PerpTradeV3Event,
    PerpTradeV3Output, PerpTradeV3State, UserId,
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
    #[error("trade settlement failed: {0}")]
    TradeSettlement(PerpTradeV3Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SettleHyperliquidPerpTradeUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeChanges {
    pub changed_positions: Vec<UpdatedEntityPair<PerpPositionV3>>,
    pub changed_margin_balances: Vec<UpdatedEntityPair<BalanceV2>>,
    pub changed_trades: Vec<UpdatedEntityPair<PerpTradeV3>>,
}

impl ReplayableChanges for SettleHyperliquidPerpTradeChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(
            self.changed_positions.len()
                + self.changed_margin_balances.len()
                + self.changed_trades.len(),
        );
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
        for pair in &self.changed_trades {
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
        if state.trades.iter().any(|trade| trade.state == PerpTradeV3State::Settled) {
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
    let mut changed_trades = Vec::with_capacity(state.trades.len());

    for trade in &state.trades {
        let mut start_positions = [Decimal::ZERO; 2];
        let mut closed_pnls = [Decimal::ZERO; 2];
        for (index, (user, fee)) in [
            (trade.trade_base.taker_user, trade.trade_base.taker_fee),
            (trade.trade_base.maker_user, trade.trade_base.maker_fee),
        ]
        .into_iter()
        .enumerate()
        {
            let key = (user.0, trade.trade_base.symbol.base.index());
            let before = positions
                .get(&key)
                .cloned()
                .ok_or(SettleHyperliquidPerpTradeError::PositionMismatch)?;
            start_positions[index] = before.signed_size();
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
            closed_pnls[index] = closed_pnl;
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

        let before = trade.clone();
        let mut after = before.clone();
        let (_, output) = after
            .transition(PerpTradeV3Event::Settle {
                taker_start_position: start_positions[0],
                maker_start_position: start_positions[1],
                taker_closed_pnl: closed_pnls[0],
                maker_closed_pnl: closed_pnls[1],
                timestamp: cmd.logical_ts(),
            })
            .map_err(map_trade_error)?;
        let PerpTradeV3Output::Settled { .. } = output;
        changed_trades.push(UpdatedEntityPair { before, after });
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

    Ok(SettleHyperliquidPerpTradeChanges {
        changed_positions,
        changed_margin_balances,
        changed_trades,
    })
}

fn map_position_error(_: PerpPositionV3Error) -> SettleHyperliquidPerpTradeError {
    SettleHyperliquidPerpTradeError::PositionMismatch
}

fn map_trade_error(error: PerpTradeV3Error) -> SettleHyperliquidPerpTradeError {
    match error {
        PerpTradeV3Error::AlreadySettled => SettleHyperliquidPerpTradeError::TradeAlreadySettled,
        other => SettleHyperliquidPerpTradeError::TradeSettlement(other),
    }
}

fn map_balance_error(error: BalanceV2Error) -> SettleHyperliquidPerpTradeError {
    match error {
        BalanceV2Error::InsufficientAvailableBalance => {
            SettleHyperliquidPerpTradeError::InsufficientAvailableMargin
        }
        _ => SettleHyperliquidPerpTradeError::ArithmeticOverflow,
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, ReplayableChanges, UseCase};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::{
        FundingAccumulatorV3, MarginModeV3, OrderSide, PerpTradeKind, TradeBase,
    };

    use super::*;

    fn position(id: u64, user: u64, size: i64, price: i64) -> PerpPositionV3 {
        PerpPositionV3::new(
            id,
            UserId(user),
            Coin::BTC,
            Decimal::from(size),
            if size == 0 { Decimal::ZERO } else { Decimal::from(price) },
            Decimal::from(10),
            MarginModeV3::Cross,
            None,
            Decimal::ZERO,
            FundingAccumulatorV3::default(),
            Decimal::ZERO,
            3,
        )
        .unwrap()
    }

    fn trade(kind: PerpTradeKind) -> PerpTradeV3 {
        PerpTradeV3 {
            trade_base: TradeBase {
                trade_id: 42,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDC),
                taker_user: UserId(1),
                maker_user: UserId(2),
                taker_order_id: 10,
                maker_order_id: 11,
                taker_side: OrderSide::Sell,
                price: Decimal::from(110),
                size: Decimal::ONE,
                taker_fee: Decimal::ZERO,
                maker_fee: Decimal::ZERO,
                created_at: 5,
                updated_at: 5,
                version: 0,
            },
            state: PerpTradeV3State::Unsettled,
            kind,
            reference_id: None,
            taker_start_position: Decimal::ZERO,
            maker_start_position: Decimal::ZERO,
            taker_closed_pnl: Decimal::ZERO,
            maker_closed_pnl: Decimal::ZERO,
            liquidation: None,
        }
    }

    fn command() -> LogEntry<SettleHyperliquidPerpTradeCmd> {
        LogEntrySequencer::from_metadata(
            9,
            8,
            7,
            6,
            SettleHyperliquidPerpTradeCmd {
                operator: UserId(99),
                settlement_batch_id: 1,
                trade_ids: vec![42],
            },
        )
    }

    fn state(kind: PerpTradeKind) -> SettleHyperliquidPerpTradeState {
        SettleHyperliquidPerpTradeState {
            trades: vec![trade(kind)],
            positions: vec![position(101, 1, 2, 100), position(102, 2, -2, 100)],
            margin_balances: vec![
                BalanceV2::new(
                    1,
                    1,
                    u64::from(Coin::USDC.index()),
                    Decimal::from(100),
                    Decimal::ZERO,
                    1,
                    0,
                ),
                BalanceV2::new(
                    2,
                    2,
                    u64::from(Coin::USDC.index()),
                    Decimal::from(100),
                    Decimal::ZERO,
                    1,
                    0,
                ),
            ],
        }
    }

    #[test]
    fn settles_positions_balances_and_trade_then_replays_trade_last() {
        let changes = SettleHyperliquidPerpTradeUseCase
            .compute_state_changed(&command(), &state(PerpTradeKind::Normal))
            .unwrap();

        assert_eq!(changes.changed_positions.len(), 2);
        assert_eq!(changes.changed_margin_balances.len(), 2);
        assert_eq!(changes.changed_trades.len(), 1);
        let trade = &changes.changed_trades[0];
        assert_eq!(trade.before.state, PerpTradeV3State::Unsettled);
        assert_eq!(trade.after.state, PerpTradeV3State::Settled);
        assert_eq!(trade.after.taker_start_position, Decimal::from(2));
        assert_eq!(trade.after.maker_start_position, Decimal::from(-2));
        assert_eq!(trade.after.taker_closed_pnl, Decimal::from(10));
        assert_eq!(trade.after.maker_closed_pnl, Decimal::from(-10));
        assert_eq!(trade.after.trade_base.updated_at, 8);
        assert_eq!(trade.after.trade_base.version, 1);

        let events = changes.to_replayable_events().unwrap();
        assert_eq!(events.len(), 5);
        assert_eq!(events.last().unwrap().entity_id, 42);
    }

    #[test]
    fn rejects_settled_trade_from_entity_state() {
        let mut state = state(PerpTradeKind::Normal);
        state.trades[0].state = PerpTradeV3State::Settled;

        assert_eq!(
            SettleHyperliquidPerpTradeUseCase.validate_state_given(&command(), &state),
            Err(SettleHyperliquidPerpTradeError::TradeAlreadySettled)
        );
    }

    #[test]
    fn insurance_fund_transfer_keeps_position_restriction() {
        assert_eq!(
            SettleHyperliquidPerpTradeUseCase
                .compute_state_changed(&command(), &state(PerpTradeKind::InsuranceFundTransfer)),
            Err(SettleHyperliquidPerpTradeError::PositionMismatch)
        );
    }
}
