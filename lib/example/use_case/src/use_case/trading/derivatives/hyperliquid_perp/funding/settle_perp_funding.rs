use std::collections::HashSet;

use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, TrackDiff, UpdatedEntityPair,
    UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::account::symbol::Symbol;
use crate::entity::perp::fund::user_funding::UserFunding;
use crate::entity::perp::fund::user_funding_domain_service::{
    SettleHourError, SettleUserFundingBatchError, settle_hour, settle_user_funding_batch,
};
use crate::entity::{BalanceV2, FundingRate, MarginModeV3, PerpPositionV3, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingCmd {
    pub operator: UserId,
    pub funding_batch_id: u64,
    pub funding_time: u64,
    pub settled_at: u64,
    pub symbol: Symbol,
    pub oracle_price: Decimal,
    pub funding_rate: FundingRate,
    pub position_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingState {
    pub positions: Vec<PerpPositionV3>,
    pub margin_balances: Vec<BalanceV2>,
    pub settled_position_ids: Vec<u64>,
    pub margin_mode: MarginModeV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleHyperliquidPerpFundingError {
    #[error("funding batch id must be non-zero")]
    InvalidFundingBatchId,
    #[error("position ids must not be empty")]
    EmptyPositionIds,
    #[error("oracle price must be greater than zero")]
    InvalidOraclePrice,
    #[error("settlement time must not precede funding time")]
    InvalidSettlementTime,
    #[error("position ids do not match loaded positions")]
    PositionIdsMismatch,
    #[error("funding settlement only supports cross margin")]
    UnsupportedMarginMode,
    #[error("position was already settled in this batch")]
    PositionAlreadySettled,
    #[error(transparent)]
    Generate(#[from] SettleHourError),
    #[error(transparent)]
    Settlement(#[from] SettleUserFundingBatchError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SettleHyperliquidPerpFundingUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingChanges {
    pub created_fundings: Vec<UserFunding>,
    pub changed_positions: Vec<UpdatedEntityPair<PerpPositionV3>>,
    pub changed_margin_balances: Vec<UpdatedEntityPair<BalanceV2>>,
}

impl ReplayableChanges for SettleHyperliquidPerpFundingChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(
            self.created_fundings.len()
                + self.changed_positions.len()
                + self.changed_margin_balances.len(),
        );
        for funding in &self.created_fundings {
            events.push(funding.track_create_event()?);
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
        if cmd.position_ids.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::EmptyPositionIds);
        }
        if cmd.oracle_price <= Decimal::ZERO {
            return Err(SettleHyperliquidPerpFundingError::InvalidOraclePrice);
        }
        if cmd.settled_at < cmd.funding_time {
            return Err(SettleHyperliquidPerpFundingError::InvalidSettlementTime);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if cmd.position_ids.len() != state.positions.len()
            || cmd
                .position_ids
                .iter()
                .zip(&state.positions)
                .any(|(id, position)| *id != position.position_id())
        {
            return Err(SettleHyperliquidPerpFundingError::PositionIdsMismatch);
        }
        if state.margin_mode != MarginModeV3::Cross
            || state.positions.iter().any(|position| position.margin_mode() != MarginModeV3::Cross)
        {
            return Err(SettleHyperliquidPerpFundingError::UnsupportedMarginMode);
        }
        let settled: HashSet<u64> = state.settled_position_ids.iter().copied().collect();
        if cmd.position_ids.iter().any(|id| settled.contains(id)) {
            return Err(SettleHyperliquidPerpFundingError::PositionAlreadySettled);
        }
        let fundings = self.generate_fundings(cmd, state)?;
        settle_user_funding_batch(
            &fundings,
            &state.positions,
            &state.margin_balances,
            cmd.settled_at,
            cmd.lsn(),
        )?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let fundings = self.generate_fundings(cmd, state)?;
        let output = settle_user_funding_batch(
            &fundings,
            &state.positions,
            &state.margin_balances,
            cmd.settled_at,
            cmd.lsn(),
        )?;
        Ok(SettleHyperliquidPerpFundingChanges {
            created_fundings: output.changed_fundings.into_iter().map(|pair| pair.after).collect(),
            changed_positions: output.changed_positions,
            changed_margin_balances: output.changed_balances,
        })
    }
}

impl SettleHyperliquidPerpFundingUseCase {
    fn generate_fundings(
        self,
        cmd: &LogEntry<SettleHyperliquidPerpFundingCmd>,
        state: &SettleHyperliquidPerpFundingState,
    ) -> Result<Vec<UserFunding>, SettleHyperliquidPerpFundingError> {
        Ok(settle_hour(
            cmd.symbol,
            cmd.funding_batch_id,
            cmd.funding_time,
            cmd.funding_rate,
            cmd.oracle_price,
            &state.positions,
            cmd.lsn(),
        )?)
    }
}
