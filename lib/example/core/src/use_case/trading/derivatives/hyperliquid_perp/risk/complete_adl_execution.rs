use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use super::close_liquidation::HyperliquidPerpLiquidationCloseAs;
use crate::entity::{
    HyperliquidPerpAdlBatch, HyperliquidPerpAdlBatchStatus, HyperliquidPerpAdlDeleveragingRecord,
    HyperliquidPerpAdlExecution, HyperliquidPerpAdlExecutionStatus, HyperliquidPerpLiquidation,
    HyperliquidPerpLiquidationStatus, HyperliquidPerpShortfall, HyperliquidPerpShortfallStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteHyperliquidPerpAdlExecutionCmd {
    pub party_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub adl_batch_id: String,
    pub adl_execution_id: String,
    pub adl_deleveraging_record_id: String,
    pub qty: u64,
    pub execution_price: u64,
    pub covered_quote: u64,
    pub no_more_progress: bool,
}

impl IssuedByParty for CompleteHyperliquidPerpAdlExecutionCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteHyperliquidPerpAdlExecutionState {
    pub liquidation: HyperliquidPerpLiquidation,
    pub shortfall: HyperliquidPerpShortfall,
    pub adl_batch: HyperliquidPerpAdlBatch,
    pub adl_execution: HyperliquidPerpAdlExecution,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CompleteHyperliquidPerpAdlExecutionError {
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
    #[error("adl_deleveraging_record_id must not be empty when execution completes")]
    InvalidAdlRecordId,
    #[error("qty, execution_price, and covered_quote must be greater than zero unless exhausting")]
    InvalidCompletionPayload,
    #[error("exhausted execution must not carry completion payload")]
    InvalidExhaustedPayload,
    #[error("liquidation, shortfall, adl batch, or execution identity mismatch")]
    StateMismatch,
    #[error("liquidation is not in ADL covering stage")]
    LiquidationNotAdlCovering,
    #[error("adl batch or shortfall is not open for ADL progress")]
    AdlNotProgressable,
    #[error("adl execution is not in started status")]
    AdlExecutionNotStarted,
    #[error("covered quote exceeds remaining shortfall")]
    CoveredQuoteTooLarge,
    #[error("arithmetic overflow while completing ADL execution")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CompleteHyperliquidPerpAdlExecutionUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteHyperliquidPerpAdlExecutionChanges {
    pub changed_execution: UpdatedEntityPair<HyperliquidPerpAdlExecution>,
    pub created_record: Option<HyperliquidPerpAdlDeleveragingRecord>,
    pub changed_adl_batch: UpdatedEntityPair<HyperliquidPerpAdlBatch>,
    pub changed_shortfall: UpdatedEntityPair<HyperliquidPerpShortfall>,
    pub close_as: Option<HyperliquidPerpLiquidationCloseAs>,
}

impl ReplayableChanges for CompleteHyperliquidPerpAdlExecutionChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        let mut events = vec![
            self.changed_execution.after.track_update_event_from(&self.changed_execution.before)?,
        ];
        if let Some(record) = &self.created_record {
            events.push(record.track_create_event()?);
        }
        events.push(
            self.changed_adl_batch.after.track_update_event_from(&self.changed_adl_batch.before)?,
        );
        events.push(
            self.changed_shortfall.after.track_update_event_from(&self.changed_shortfall.before)?,
        );
        Ok(events)
    }
}

impl CommandUseCase4 for CompleteHyperliquidPerpAdlExecutionUseCase {
    type Command = CompleteHyperliquidPerpAdlExecutionCmd;
    type GivenState = CompleteHyperliquidPerpAdlExecutionState;
    type Error = CompleteHyperliquidPerpAdlExecutionError;
    type Changes = CompleteHyperliquidPerpAdlExecutionChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidLiquidationId);
        }
        if cmd.shortfall_id.is_empty() {
            return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidShortfallId);
        }
        if cmd.adl_batch_id.is_empty() {
            return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidAdlBatchId);
        }
        if cmd.adl_execution_id.is_empty() {
            return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidAdlExecutionId);
        }
        if cmd.no_more_progress {
            if !cmd.adl_deleveraging_record_id.is_empty()
                || cmd.qty != 0
                || cmd.execution_price != 0
                || cmd.covered_quote != 0
            {
                return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidExhaustedPayload);
            }
            return Ok(());
        }
        if cmd.adl_deleveraging_record_id.is_empty() {
            return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidAdlRecordId);
        }
        if cmd.qty == 0 || cmd.execution_price == 0 || cmd.covered_quote == 0 {
            return Err(CompleteHyperliquidPerpAdlExecutionError::InvalidCompletionPayload);
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
            || state.adl_execution.adl_execution_id != cmd.adl_execution_id
            || state.shortfall.liquidation_id != cmd.liquidation_id
            || state.adl_batch.liquidation_id != cmd.liquidation_id
            || state.adl_batch.shortfall_id != cmd.shortfall_id
            || state.adl_execution.liquidation_id != cmd.liquidation_id
            || state.adl_execution.shortfall_id != cmd.shortfall_id
            || state.adl_execution.adl_batch_id != cmd.adl_batch_id
        {
            return Err(CompleteHyperliquidPerpAdlExecutionError::StateMismatch);
        }
        if state.liquidation.status != HyperliquidPerpLiquidationStatus::AdlCovering {
            return Err(CompleteHyperliquidPerpAdlExecutionError::LiquidationNotAdlCovering);
        }
        if !matches!(
            state.shortfall.status,
            HyperliquidPerpShortfallStatus::Open | HyperliquidPerpShortfallStatus::PartiallyCovered
        ) || !matches!(
            state.adl_batch.status,
            HyperliquidPerpAdlBatchStatus::Open | HyperliquidPerpAdlBatchStatus::PartiallyCovered
        ) {
            return Err(CompleteHyperliquidPerpAdlExecutionError::AdlNotProgressable);
        }
        if state.adl_execution.status != HyperliquidPerpAdlExecutionStatus::Started {
            return Err(CompleteHyperliquidPerpAdlExecutionError::AdlExecutionNotStarted);
        }
        if !cmd.no_more_progress
            && (cmd.covered_quote > state.shortfall.remaining_quote
                || cmd.covered_quote > state.adl_batch.remaining_quote)
        {
            return Err(CompleteHyperliquidPerpAdlExecutionError::CoveredQuoteTooLarge);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let mut execution_after = state.adl_execution.clone();
        let mut batch_after = state.adl_batch.clone();
        let mut shortfall_after = state.shortfall.clone();
        let created_record;
        let close_as;

        if cmd.no_more_progress {
            let execution_version = execution_after
                .version
                .checked_add(1)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;
            execution_after
                .apply_exhausted(execution_version)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;

            let batch_version = batch_after
                .version
                .checked_add(1)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;
            batch_after
                .apply_exhausted(batch_version)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;

            let shortfall_version = shortfall_after
                .version
                .checked_add(1)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;
            shortfall_after
                .apply_exhausted(shortfall_version)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;

            created_record = None;
            close_as = Some(HyperliquidPerpLiquidationCloseAs::Exhausted);
        } else {
            let execution_version = execution_after
                .version
                .checked_add(1)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;
            execution_after
                .apply_completed(cmd.execution_price, cmd.qty, cmd.covered_quote, execution_version)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;

            let batch_version = batch_after
                .version
                .checked_add(1)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;
            batch_after
                .apply_entry(cmd.covered_quote, batch_version)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;

            let shortfall_version = shortfall_after
                .version
                .checked_add(1)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;
            shortfall_after
                .apply_adl_coverage(cmd.covered_quote, shortfall_version)
                .ok_or(CompleteHyperliquidPerpAdlExecutionError::ArithmeticOverflow)?;

            created_record = Some(HyperliquidPerpAdlDeleveragingRecord::new(
                cmd.adl_deleveraging_record_id.clone(),
                cmd.adl_execution_id.clone(),
                cmd.adl_batch_id.clone(),
                cmd.liquidation_id.clone(),
                cmd.shortfall_id.clone(),
                execution_after.deleveraged_account_id.clone(),
                execution_after.deleveraged_position_id.clone(),
                execution_after.asset,
                execution_after.symbol.clone(),
                cmd.qty,
                cmd.execution_price,
                cmd.covered_quote,
            ));
            close_as = if shortfall_after.remaining_quote == 0 {
                Some(HyperliquidPerpLiquidationCloseAs::Closed)
            } else {
                None
            };
        }

        Ok(CompleteHyperliquidPerpAdlExecutionChanges {
            changed_execution: UpdatedEntityPair {
                before: state.adl_execution,
                after: execution_after,
            },
            created_record,
            changed_adl_batch: UpdatedEntityPair { before: state.adl_batch, after: batch_after },
            changed_shortfall: UpdatedEntityPair {
                before: state.shortfall,
                after: shortfall_after,
            },
            close_as,
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
    use crate::{
        CloseHyperliquidPerpLiquidationCmd, CloseHyperliquidPerpLiquidationState,
        CloseHyperliquidPerpLiquidationUseCase,
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

    fn adl_execution() -> HyperliquidPerpAdlExecution {
        HyperliquidPerpAdlExecution::new(
            "adl-exec-1".to_string(),
            "adl-batch-1".to_string(),
            "liq-1".to_string(),
            "shortfall-1".to_string(),
            "winner-1".to_string(),
            "position-9".to_string(),
            7,
            "BTC-PERP".to_string(),
            50_000,
        )
    }

    #[test]
    fn role_is_risk_engine() {
        assert_eq!(CompleteHyperliquidPerpAdlExecutionUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_exhausted_payload_with_progress_fields() {
        let err = CompleteHyperliquidPerpAdlExecutionUseCase
            .pre_check_command(&CompleteHyperliquidPerpAdlExecutionCmd {
                party_id: "risk-engine".to_string(),
                liquidation_id: "liq-1".to_string(),
                shortfall_id: "shortfall-1".to_string(),
                adl_batch_id: "adl-batch-1".to_string(),
                adl_execution_id: "adl-exec-1".to_string(),
                adl_deleveraging_record_id: String::new(),
                qty: 1,
                execution_price: 25_000,
                covered_quote: 25_000,
                no_more_progress: true,
            })
            .unwrap_err();

        assert_eq!(err, CompleteHyperliquidPerpAdlExecutionError::InvalidExhaustedPayload);
    }

    #[test]
    fn validate_rejects_when_execution_is_not_started() {
        let mut execution = adl_execution();
        execution.apply_completed(25_000, 1, 25_000, 2).unwrap();

        let err = CompleteHyperliquidPerpAdlExecutionUseCase
            .validate_against_state(
                &CompleteHyperliquidPerpAdlExecutionCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    shortfall_id: "shortfall-1".to_string(),
                    adl_batch_id: "adl-batch-1".to_string(),
                    adl_execution_id: "adl-exec-1".to_string(),
                    adl_deleveraging_record_id: "adl-record-1".to_string(),
                    qty: 1,
                    execution_price: 25_000,
                    covered_quote: 25_000,
                    no_more_progress: false,
                },
                &CompleteHyperliquidPerpAdlExecutionState {
                    liquidation: liquidation(),
                    shortfall: shortfall(),
                    adl_batch: adl_batch(),
                    adl_execution: execution,
                },
            )
            .unwrap_err();

        assert_eq!(err, CompleteHyperliquidPerpAdlExecutionError::AdlExecutionNotStarted);
    }

    #[test]
    fn completed_path_updates_execution_creates_record_and_returns_close_signal() {
        let changes = CompleteHyperliquidPerpAdlExecutionUseCase
            .compute_changes(
                &CompleteHyperliquidPerpAdlExecutionCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    shortfall_id: "shortfall-1".to_string(),
                    adl_batch_id: "adl-batch-1".to_string(),
                    adl_execution_id: "adl-exec-1".to_string(),
                    adl_deleveraging_record_id: "adl-record-1".to_string(),
                    qty: 1,
                    execution_price: 25_000,
                    covered_quote: 25_000,
                    no_more_progress: false,
                },
                CompleteHyperliquidPerpAdlExecutionState {
                    liquidation: liquidation(),
                    shortfall: shortfall(),
                    adl_batch: adl_batch(),
                    adl_execution: adl_execution(),
                },
            )
            .unwrap();

        assert_eq!(
            changes.changed_execution.after.status,
            HyperliquidPerpAdlExecutionStatus::Completed
        );
        assert_eq!(
            changes.created_record.as_ref().map(|record| record.adl_execution_id.as_str()),
            Some("adl-exec-1")
        );
        assert_eq!(changes.changed_shortfall.after.remaining_quote, 0);
        assert_eq!(changes.changed_adl_batch.after.status, HyperliquidPerpAdlBatchStatus::Covered);
        assert_eq!(changes.close_as, Some(HyperliquidPerpLiquidationCloseAs::Closed));
        assert_eq!(changes.to_replayable_events().unwrap().len(), 4);

        let close_changes = CloseHyperliquidPerpLiquidationUseCase
            .compute_changes(
                &CloseHyperliquidPerpLiquidationCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    close_as: HyperliquidPerpLiquidationCloseAs::Closed,
                },
                CloseHyperliquidPerpLiquidationState { liquidation: liquidation() },
            )
            .unwrap();
        assert_eq!(
            close_changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::Closed
        );
    }

    #[test]
    fn exhausted_path_marks_execution_batch_and_shortfall_exhausted() {
        let changes = CompleteHyperliquidPerpAdlExecutionUseCase
            .compute_changes(
                &CompleteHyperliquidPerpAdlExecutionCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    shortfall_id: "shortfall-1".to_string(),
                    adl_batch_id: "adl-batch-1".to_string(),
                    adl_execution_id: "adl-exec-1".to_string(),
                    adl_deleveraging_record_id: String::new(),
                    qty: 0,
                    execution_price: 0,
                    covered_quote: 0,
                    no_more_progress: true,
                },
                CompleteHyperliquidPerpAdlExecutionState {
                    liquidation: liquidation(),
                    shortfall: shortfall(),
                    adl_batch: adl_batch(),
                    adl_execution: adl_execution(),
                },
            )
            .unwrap();

        assert_eq!(
            changes.changed_execution.after.status,
            HyperliquidPerpAdlExecutionStatus::Exhausted
        );
        assert_eq!(changes.created_record, None);
        assert_eq!(
            changes.changed_adl_batch.after.status,
            HyperliquidPerpAdlBatchStatus::Exhausted
        );
        assert_eq!(
            changes.changed_shortfall.after.status,
            HyperliquidPerpShortfallStatus::Exhausted
        );
        assert_eq!(changes.close_as, Some(HyperliquidPerpLiquidationCloseAs::Exhausted));
    }
}
