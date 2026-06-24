use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpAdlBatch, HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpShortfall, HyperliquidPerpShortfallStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpAdlBatchCmd {
    pub party_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub adl_batch_id: String,
}

impl IssuedByParty for StartHyperliquidPerpAdlBatchCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpAdlBatchState {
    pub liquidation: HyperliquidPerpLiquidation,
    pub shortfall: HyperliquidPerpShortfall,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StartHyperliquidPerpAdlBatchError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("shortfall_id must not be empty")]
    InvalidShortfallId,
    #[error("adl_batch_id must not be empty")]
    InvalidAdlBatchId,
    #[error("liquidation or shortfall identity mismatch")]
    StateMismatch,
    #[error("liquidation does not allow ADL covering")]
    LiquidationNotReady,
    #[error("shortfall has no remaining quote for ADL")]
    ShortfallNotReady,
    #[error("arithmetic overflow while starting ADL batch")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StartHyperliquidPerpAdlBatchUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpAdlBatchChanges {
    pub created_adl_batch: HyperliquidPerpAdlBatch,
    pub changed_liquidation: UpdatedEntityPair<HyperliquidPerpLiquidation>,
}

impl ReplayableChanges for StartHyperliquidPerpAdlBatchChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        Ok(vec![
            self.created_adl_batch.track_create_event()?,
            self.changed_liquidation
                .after
                .track_update_event_from(&self.changed_liquidation.before)?,
        ])
    }
}

impl CommandUseCase4 for StartHyperliquidPerpAdlBatchUseCase {
    type Command = StartHyperliquidPerpAdlBatchCmd;
    type GivenState = StartHyperliquidPerpAdlBatchState;
    type Error = StartHyperliquidPerpAdlBatchError;
    type Changes = StartHyperliquidPerpAdlBatchChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(StartHyperliquidPerpAdlBatchError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(StartHyperliquidPerpAdlBatchError::InvalidLiquidationId);
        }
        if cmd.shortfall_id.is_empty() {
            return Err(StartHyperliquidPerpAdlBatchError::InvalidShortfallId);
        }
        if cmd.adl_batch_id.is_empty() {
            return Err(StartHyperliquidPerpAdlBatchError::InvalidAdlBatchId);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.liquidation.liquidation_id != cmd.liquidation_id
            || state.shortfall.shortfall_id != cmd.shortfall_id
            || state.shortfall.liquidation_id != cmd.liquidation_id
        {
            return Err(StartHyperliquidPerpAdlBatchError::StateMismatch);
        }
        if !matches!(
            state.liquidation.status,
            HyperliquidPerpLiquidationStatus::ShortfallAssessed
                | HyperliquidPerpLiquidationStatus::FundCovering
                | HyperliquidPerpLiquidationStatus::AdlCovering
        ) {
            return Err(StartHyperliquidPerpAdlBatchError::LiquidationNotReady);
        }
        if !matches!(
            state.shortfall.status,
            HyperliquidPerpShortfallStatus::Open | HyperliquidPerpShortfallStatus::PartiallyCovered
        ) || state.shortfall.remaining_quote == 0
        {
            return Err(StartHyperliquidPerpAdlBatchError::ShortfallNotReady);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let created_adl_batch = HyperliquidPerpAdlBatch::new(
            cmd.adl_batch_id.clone(),
            cmd.liquidation_id.clone(),
            cmd.shortfall_id.clone(),
            state.shortfall.asset,
            state.shortfall.symbol.clone(),
            state.shortfall.remaining_quote,
        );
        let before = state.liquidation;
        let mut after = before.clone();
        let next_version = after
            .version
            .checked_add(1)
            .ok_or(StartHyperliquidPerpAdlBatchError::ArithmeticOverflow)?;
        after
            .mark_adl_covering(next_version)
            .ok_or(StartHyperliquidPerpAdlBatchError::ArithmeticOverflow)?;

        Ok(StartHyperliquidPerpAdlBatchChanges {
            created_adl_batch,
            changed_liquidation: UpdatedEntityPair { before, after },
        })
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::entity::{
        HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode,
        HyperliquidPerpPositionSide,
    };

    fn liquidation() -> HyperliquidPerpLiquidation {
        let mut liquidation = HyperliquidPerpLiquidation::new(
            "liq-1".to_string(),
            "batch-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            2,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::FundCovering,
        );
        liquidation.remaining_qty = 0;
        liquidation
    }

    fn shortfall() -> HyperliquidPerpShortfall {
        let mut shortfall = HyperliquidPerpShortfall::new(
            "shortfall-1".to_string(),
            "liq-1".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            50_000,
            100_000,
            70_000,
            30_000,
        );
        shortfall.apply_insurance_coverage(5_000, 2).unwrap();
        shortfall
    }

    #[test]
    fn start_adl_batch_uses_remaining_shortfall_and_moves_liquidation_to_adl_covering() {
        let changes = StartHyperliquidPerpAdlBatchUseCase
            .compute_changes(
                &StartHyperliquidPerpAdlBatchCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    shortfall_id: "shortfall-1".to_string(),
                    adl_batch_id: "adl-batch-1".to_string(),
                },
                StartHyperliquidPerpAdlBatchState {
                    liquidation: liquidation(),
                    shortfall: shortfall(),
                },
            )
            .unwrap();

        assert_eq!(changes.created_adl_batch.target_cover_quote, 25_000);
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::AdlCovering
        );
        assert_eq!(changes.to_replayable_events().unwrap().len(), 2);
    }
}
