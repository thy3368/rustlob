use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpAdlBatch, HyperliquidPerpAdlBatchStatus, HyperliquidPerpAdlExecution,
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus, HyperliquidPerpShortfall,
    HyperliquidPerpShortfallStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpAdlExecutionCmd {
    pub party_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub adl_batch_id: String,
    pub adl_execution_id: String,
    pub deleveraged_account_id: String,
    pub deleveraged_position_id: String,
}

impl IssuedByParty for StartHyperliquidPerpAdlExecutionCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpAdlExecutionState {
    pub liquidation: HyperliquidPerpLiquidation,
    pub shortfall: HyperliquidPerpShortfall,
    pub adl_batch: HyperliquidPerpAdlBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StartHyperliquidPerpAdlExecutionError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("shortfall_id must not be empty")]
    InvalidShortfallId,
    #[error("adl_batch_id must not be empty")]
    InvalidAdlBatchId,
    #[error("adl_execution_id must not be empty")]
    InvalidAdlExecutionId,
    #[error("deleveraged_account_id must not be empty")]
    InvalidDeleveragedAccountId,
    #[error("deleveraged_position_id must not be empty")]
    InvalidDeleveragedPositionId,
    #[error("liquidation, shortfall, or adl batch identity mismatch")]
    StateMismatch,
    #[error("liquidation is not in ADL covering stage")]
    LiquidationNotAdlCovering,
    #[error("adl batch or shortfall is not open for ADL execution")]
    AdlNotStartable,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StartHyperliquidPerpAdlExecutionUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpAdlExecutionChanges {
    pub created_execution: HyperliquidPerpAdlExecution,
}

impl ReplayableChanges for StartHyperliquidPerpAdlExecutionChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        Ok(vec![self.created_execution.track_create_event()?])
    }
}

impl CommandUseCase4 for StartHyperliquidPerpAdlExecutionUseCase {
    type Command = StartHyperliquidPerpAdlExecutionCmd;
    type GivenState = StartHyperliquidPerpAdlExecutionState;
    type Error = StartHyperliquidPerpAdlExecutionError;
    type Changes = StartHyperliquidPerpAdlExecutionChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(StartHyperliquidPerpAdlExecutionError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(StartHyperliquidPerpAdlExecutionError::InvalidLiquidationId);
        }
        if cmd.shortfall_id.is_empty() {
            return Err(StartHyperliquidPerpAdlExecutionError::InvalidShortfallId);
        }
        if cmd.adl_batch_id.is_empty() {
            return Err(StartHyperliquidPerpAdlExecutionError::InvalidAdlBatchId);
        }
        if cmd.adl_execution_id.is_empty() {
            return Err(StartHyperliquidPerpAdlExecutionError::InvalidAdlExecutionId);
        }
        if cmd.deleveraged_account_id.is_empty() {
            return Err(StartHyperliquidPerpAdlExecutionError::InvalidDeleveragedAccountId);
        }
        if cmd.deleveraged_position_id.is_empty() {
            return Err(StartHyperliquidPerpAdlExecutionError::InvalidDeleveragedPositionId);
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
            || state.adl_batch.adl_batch_id != cmd.adl_batch_id
            || state.shortfall.liquidation_id != cmd.liquidation_id
            || state.adl_batch.liquidation_id != cmd.liquidation_id
            || state.adl_batch.shortfall_id != cmd.shortfall_id
        {
            return Err(StartHyperliquidPerpAdlExecutionError::StateMismatch);
        }
        if state.liquidation.status != HyperliquidPerpLiquidationStatus::AdlCovering {
            return Err(StartHyperliquidPerpAdlExecutionError::LiquidationNotAdlCovering);
        }
        if !matches!(
            state.shortfall.status,
            HyperliquidPerpShortfallStatus::Open | HyperliquidPerpShortfallStatus::PartiallyCovered
        ) || !matches!(
            state.adl_batch.status,
            HyperliquidPerpAdlBatchStatus::Open | HyperliquidPerpAdlBatchStatus::PartiallyCovered
        ) {
            return Err(StartHyperliquidPerpAdlExecutionError::AdlNotStartable);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        Ok(StartHyperliquidPerpAdlExecutionChanges {
            created_execution: HyperliquidPerpAdlExecution::new(
                cmd.adl_execution_id.clone(),
                cmd.adl_batch_id.clone(),
                cmd.liquidation_id.clone(),
                cmd.shortfall_id.clone(),
                cmd.deleveraged_account_id.clone(),
                cmd.deleveraged_position_id.clone(),
                state.shortfall.asset,
                state.shortfall.symbol.clone(),
                state.shortfall.bankruptcy_price,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::entity::{HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode};

    fn cmd() -> StartHyperliquidPerpAdlExecutionCmd {
        StartHyperliquidPerpAdlExecutionCmd {
            party_id: "risk-engine".to_string(),
            liquidation_id: "liq-1".to_string(),
            shortfall_id: "shortfall-1".to_string(),
            adl_batch_id: "adl-batch-1".to_string(),
            adl_execution_id: "adl-exec-1".to_string(),
            deleveraged_account_id: "winner-1".to_string(),
            deleveraged_position_id: "position-9".to_string(),
        }
    }

    fn liquidation() -> HyperliquidPerpLiquidation {
        let mut liquidation = HyperliquidPerpLiquidation::new(
            "liq-1".to_string(),
            "batch-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            2,
            2,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::AdlCovering,
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

    fn adl_batch() -> HyperliquidPerpAdlBatch {
        let mut batch = HyperliquidPerpAdlBatch::new(
            "adl-batch-1".to_string(),
            "liq-1".to_string(),
            "shortfall-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            25_000,
        );
        batch.version = 1;
        batch
    }

    #[test]
    fn role_is_risk_engine() {
        assert_eq!(StartHyperliquidPerpAdlExecutionUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_empty_party_id() {
        let err = StartHyperliquidPerpAdlExecutionUseCase
            .pre_check_command(&StartHyperliquidPerpAdlExecutionCmd {
                party_id: String::new(),
                ..cmd()
            })
            .unwrap_err();

        assert_eq!(err, StartHyperliquidPerpAdlExecutionError::InvalidPartyId);
    }

    #[test]
    fn validate_rejects_when_liquidation_is_not_in_adl_covering() {
        let mut liquidation = liquidation();
        liquidation.status = HyperliquidPerpLiquidationStatus::Executing;

        let err = StartHyperliquidPerpAdlExecutionUseCase
            .validate_against_state(
                &cmd(),
                &StartHyperliquidPerpAdlExecutionState {
                    liquidation,
                    shortfall: shortfall(),
                    adl_batch: adl_batch(),
                },
            )
            .unwrap_err();

        assert_eq!(err, StartHyperliquidPerpAdlExecutionError::LiquidationNotAdlCovering);
    }

    #[test]
    fn compute_changes_creates_started_execution_without_touching_batch_or_shortfall() {
        let changes = StartHyperliquidPerpAdlExecutionUseCase
            .compute_changes(
                &cmd(),
                StartHyperliquidPerpAdlExecutionState {
                    liquidation: liquidation(),
                    shortfall: shortfall(),
                    adl_batch: adl_batch(),
                },
            )
            .unwrap();

        assert_eq!(
            changes.created_execution.status,
            crate::entity::HyperliquidPerpAdlExecutionStatus::Started
        );
        assert_eq!(changes.created_execution.reference_bankruptcy_price, 50_000);
        assert_eq!(changes.created_execution.execution_price, None);
        assert_eq!(changes.to_replayable_events().unwrap().len(), 1);
    }
}
