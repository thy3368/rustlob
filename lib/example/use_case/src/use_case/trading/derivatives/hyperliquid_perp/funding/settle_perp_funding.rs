use std::collections::{BTreeMap, HashSet};

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges,
    UpdatedEntityPair, UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::perp::fund::user_funding::{
    UserFunding, UserFundingError, UserFundingEvent, UserFundingOutput,
};
use crate::entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, MarginModeV3, PerpPositionV3, PerpPositionV3Error,
    PerpPositionV3Event, UserId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingCmd {
    pub operator: UserId,
    pub funding_batch_id: u64,
    pub funding_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingState {
    pub fundings: Vec<UserFunding>,
    pub positions: Vec<PerpPositionV3>,
    pub margin_balances: Vec<BalanceV2>,
    pub settled_funding_ids: Vec<u64>,
    pub margin_mode: MarginModeV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleHyperliquidPerpFundingError {
    #[error("funding batch id must be non-zero")]
    InvalidFundingBatchId,
    #[error("funding ids must not be empty")]
    EmptyFundingIds,
    #[error("funding ids do not match loaded facts")]
    FundingIdsMismatch,
    #[error("funding fact was already settled")]
    FundingAlreadySettled,
    #[error("position was not found or mismatched")]
    PositionMismatch,
    #[error("margin balance was not found")]
    MarginBalanceNotFound,
    #[error("insufficient available margin")]
    InsufficientAvailableMargin,
    #[error("arithmetic overflow while settling funding")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SettleHyperliquidPerpFundingUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingChanges {
    pub changed_fundings: Vec<UpdatedEntityPair<UserFunding>>,
    pub changed_positions: Vec<UpdatedEntityPair<PerpPositionV3>>,
    pub changed_margin_balances: Vec<UpdatedEntityPair<BalanceV2>>,
}

impl ReplayableChanges for SettleHyperliquidPerpFundingChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(
            self.changed_fundings.len()
                + self.changed_positions.len()
                + self.changed_margin_balances.len(),
        );
        for pair in &self.changed_fundings {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        for pair in &self.changed_positions {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        for pair in &self.changed_margin_balances {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        Ok(events)
    }
}

impl UseCase for SettleHyperliquidPerpFundingUseCase {
    type Command = LogEntry<SettleHyperliquidPerpFundingCmd>;
    type StateGiven = SettleHyperliquidPerpFundingState;
    type Error = SettleHyperliquidPerpFundingError;
    type StateChanged = SettleHyperliquidPerpFundingChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.funding_batch_id == 0 {
            return Err(SettleHyperliquidPerpFundingError::InvalidFundingBatchId);
        }
        if cmd.funding_ids.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::EmptyFundingIds);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if cmd.funding_ids.len() != state.fundings.len()
            || cmd
                .funding_ids
                .iter()
                .zip(&state.fundings)
                .any(|(id, funding)| *id != funding.funding_id())
        {
            return Err(SettleHyperliquidPerpFundingError::FundingIdsMismatch);
        }
        let settled: HashSet<u64> = state.settled_funding_ids.iter().copied().collect();
        if cmd.funding_ids.iter().any(|id| settled.contains(id)) {
            return Err(SettleHyperliquidPerpFundingError::FundingAlreadySettled);
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
    cmd: &LogEntry<SettleHyperliquidPerpFundingCmd>,
    state: &SettleHyperliquidPerpFundingState,
) -> Result<SettleHyperliquidPerpFundingChanges, SettleHyperliquidPerpFundingError> {
    let mut positions: BTreeMap<(u64, u32), PerpPositionV3> = state
        .positions
        .iter()
        .cloned()
        .map(|position| ((position.user_id().0, position.coin().index()), position))
        .collect();
    let mut balance_deltas: BTreeMap<(u64, u64), Decimal> = BTreeMap::new();
    let mut changed_fundings = Vec::with_capacity(state.fundings.len());
    let mut changed_positions = Vec::with_capacity(state.fundings.len());

    for funding in &state.fundings {
        let before_funding = funding.clone();
        let mut after_funding = before_funding.clone();
        let (_, output) = after_funding
            .transition(UserFundingEvent::Settle { settled_at: cmd.logical_ts() })
            .map_err(map_funding_error)?;
        let UserFundingOutput::BalanceImpact { party_id, quote_coin_idx, signed_usdc_delta } =
            output;
        changed_fundings.push(UpdatedEntityPair { before: before_funding, after: after_funding });

        let key = (party_id, funding.coin().base.index());
        let before_position = positions
            .get(&key)
            .cloned()
            .ok_or(SettleHyperliquidPerpFundingError::PositionMismatch)?;
        if before_position.margin_mode() != state.margin_mode {
            return Err(SettleHyperliquidPerpFundingError::PositionMismatch);
        }
        let mut after_position = before_position.clone();
        after_position
            .apply(PerpPositionV3Event::FundingAmountSettled {
                user_id: UserId(party_id),
                coin: funding.coin().base,
                signed_amount: signed_usdc_delta,
                logical_ts: cmd.logical_ts(),
            })
            .map_err(map_position_error)?;
        positions.insert(key, after_position.clone());
        changed_positions
            .push(UpdatedEntityPair { before: before_position, after: after_position });

        let delta = balance_deltas.entry((party_id, quote_coin_idx)).or_default();
        *delta = delta
            .checked_add(signed_usdc_delta)
            .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
    }

    let mut changed_margin_balances = Vec::with_capacity(balance_deltas.len());
    for ((account_id, coin_idx), delta) in balance_deltas {
        if delta.is_zero() {
            continue;
        }
        let before = state
            .margin_balances
            .iter()
            .find(|balance| balance.account_id() == account_id && balance.coin_idx() == coin_idx)
            .cloned()
            .ok_or(SettleHyperliquidPerpFundingError::MarginBalanceNotFound)?;
        let mut after = before.clone();
        let event = if delta.is_sign_positive() {
            BalanceV2Event::CreditAvailable { amount: delta, timestamp: cmd.logical_ts() }
        } else {
            BalanceV2Event::DebitAvailable { amount: delta.abs(), timestamp: cmd.logical_ts() }
        };
        after.apply(event).map_err(map_balance_error)?;
        changed_margin_balances.push(UpdatedEntityPair { before, after });
    }
    Ok(SettleHyperliquidPerpFundingChanges {
        changed_fundings,
        changed_positions,
        changed_margin_balances,
    })
}

fn map_funding_error(_: UserFundingError) -> SettleHyperliquidPerpFundingError {
    SettleHyperliquidPerpFundingError::FundingAlreadySettled
}

fn map_position_error(_: PerpPositionV3Error) -> SettleHyperliquidPerpFundingError {
    SettleHyperliquidPerpFundingError::PositionMismatch
}

fn map_balance_error(error: BalanceV2Error) -> SettleHyperliquidPerpFundingError {
    match error {
        BalanceV2Error::InsufficientAvailableBalance => {
            SettleHyperliquidPerpFundingError::InsufficientAvailableMargin
        }
        _ => SettleHyperliquidPerpFundingError::ArithmeticOverflow,
    }
}
