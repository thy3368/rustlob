use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use super::close_liquidation::HyperliquidPerpLiquidationCloseAs;
use crate::entity::{
    HyperliquidPerpInsuranceFundAllocation, HyperliquidPerpLiquidation,
    HyperliquidPerpLiquidationStatus, HyperliquidPerpShortfall, HyperliquidPerpShortfallStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocateHyperliquidPerpInsuranceFundCmd {
    pub party_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub insurance_fund_allocation_id: String,
    pub insurance_fund_account_id: String,
    pub allocated_quote: u64,
}

impl IssuedByParty for AllocateHyperliquidPerpInsuranceFundCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocateHyperliquidPerpInsuranceFundState {
    pub liquidation: HyperliquidPerpLiquidation,
    pub shortfall: HyperliquidPerpShortfall,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AllocateHyperliquidPerpInsuranceFundError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("shortfall_id must not be empty")]
    InvalidShortfallId,
    #[error("insurance_fund_allocation_id must not be empty")]
    InvalidAllocationId,
    #[error("insurance_fund_account_id must not be empty")]
    InvalidInsuranceFundAccountId,
    #[error("allocated_quote must be greater than zero")]
    InvalidAllocatedQuote,
    #[error("liquidation or shortfall identity mismatch")]
    StateMismatch,
    #[error("liquidation is not in shortfall covering stage")]
    LiquidationNotCoveringShortfall,
    #[error("shortfall is not open for insurance coverage")]
    ShortfallNotCoverable,
    #[error("allocated quote exceeds remaining shortfall")]
    AllocationTooLarge,
    #[error("arithmetic overflow while allocating insurance fund")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AllocateHyperliquidPerpInsuranceFundUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocateHyperliquidPerpInsuranceFundChanges {
    pub created_allocation: HyperliquidPerpInsuranceFundAllocation,
    pub changed_shortfall: UpdatedEntityPair<HyperliquidPerpShortfall>,
    pub changed_liquidation: UpdatedEntityPair<HyperliquidPerpLiquidation>,
    pub close_as: Option<HyperliquidPerpLiquidationCloseAs>,
}

impl ReplayableChanges for AllocateHyperliquidPerpInsuranceFundChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        Ok(vec![
            self.created_allocation.track_create_event()?,
            self.changed_shortfall.after.track_update_event_from(&self.changed_shortfall.before)?,
            self.changed_liquidation
                .after
                .track_update_event_from(&self.changed_liquidation.before)?,
        ])
    }
}

impl CommandUseCase4 for AllocateHyperliquidPerpInsuranceFundUseCase {
    type Command = AllocateHyperliquidPerpInsuranceFundCmd;
    type GivenState = AllocateHyperliquidPerpInsuranceFundState;
    type Error = AllocateHyperliquidPerpInsuranceFundError;
    type Changes = AllocateHyperliquidPerpInsuranceFundChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(AllocateHyperliquidPerpInsuranceFundError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(AllocateHyperliquidPerpInsuranceFundError::InvalidLiquidationId);
        }
        if cmd.shortfall_id.is_empty() {
            return Err(AllocateHyperliquidPerpInsuranceFundError::InvalidShortfallId);
        }
        if cmd.insurance_fund_allocation_id.is_empty() {
            return Err(AllocateHyperliquidPerpInsuranceFundError::InvalidAllocationId);
        }
        if cmd.insurance_fund_account_id.is_empty() {
            return Err(AllocateHyperliquidPerpInsuranceFundError::InvalidInsuranceFundAccountId);
        }
        if cmd.allocated_quote == 0 {
            return Err(AllocateHyperliquidPerpInsuranceFundError::InvalidAllocatedQuote);
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
            return Err(AllocateHyperliquidPerpInsuranceFundError::StateMismatch);
        }
        if !matches!(
            state.liquidation.status,
            HyperliquidPerpLiquidationStatus::ShortfallAssessed
                | HyperliquidPerpLiquidationStatus::FundCovering
        ) {
            return Err(AllocateHyperliquidPerpInsuranceFundError::LiquidationNotCoveringShortfall);
        }
        if !matches!(
            state.shortfall.status,
            HyperliquidPerpShortfallStatus::Open | HyperliquidPerpShortfallStatus::PartiallyCovered
        ) {
            return Err(AllocateHyperliquidPerpInsuranceFundError::ShortfallNotCoverable);
        }
        if cmd.allocated_quote > state.shortfall.remaining_quote {
            return Err(AllocateHyperliquidPerpInsuranceFundError::AllocationTooLarge);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let mut shortfall_after = state.shortfall.clone();
        let shortfall_version = shortfall_after
            .version
            .checked_add(1)
            .ok_or(AllocateHyperliquidPerpInsuranceFundError::ArithmeticOverflow)?;
        shortfall_after
            .apply_insurance_coverage(cmd.allocated_quote, shortfall_version)
            .ok_or(AllocateHyperliquidPerpInsuranceFundError::ArithmeticOverflow)?;

        let created_allocation = HyperliquidPerpInsuranceFundAllocation::new(
            cmd.insurance_fund_allocation_id.clone(),
            cmd.liquidation_id.clone(),
            cmd.shortfall_id.clone(),
            shortfall_after.asset,
            cmd.allocated_quote,
            shortfall_after.remaining_quote,
            cmd.insurance_fund_account_id.clone(),
        );

        let mut liquidation_after = state.liquidation.clone();
        let liquidation_version = liquidation_after
            .version
            .checked_add(1)
            .ok_or(AllocateHyperliquidPerpInsuranceFundError::ArithmeticOverflow)?;
        liquidation_after
            .mark_fund_covering(liquidation_version)
            .ok_or(AllocateHyperliquidPerpInsuranceFundError::ArithmeticOverflow)?;

        let close_as = if shortfall_after.status == HyperliquidPerpShortfallStatus::Covered {
            Some(HyperliquidPerpLiquidationCloseAs::Closed)
        } else {
            None
        };

        Ok(AllocateHyperliquidPerpInsuranceFundChanges {
            created_allocation,
            changed_shortfall: UpdatedEntityPair {
                before: state.shortfall,
                after: shortfall_after,
            },
            changed_liquidation: UpdatedEntityPair {
                before: state.liquidation,
                after: liquidation_after,
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
            HyperliquidPerpLiquidationStatus::ShortfallAssessed,
        );
        liquidation.remaining_qty = 0;
        liquidation
    }

    fn shortfall() -> HyperliquidPerpShortfall {
        HyperliquidPerpShortfall::new(
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
        )
    }

    #[test]
    fn insurance_fully_covers_shortfall_and_then_liquidation_can_close() {
        let changes = AllocateHyperliquidPerpInsuranceFundUseCase
            .compute_changes(
                &AllocateHyperliquidPerpInsuranceFundCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    shortfall_id: "shortfall-1".to_string(),
                    insurance_fund_allocation_id: "ifa-1".to_string(),
                    insurance_fund_account_id: "insurance-fund".to_string(),
                    allocated_quote: 30_000,
                },
                AllocateHyperliquidPerpInsuranceFundState {
                    liquidation: liquidation(),
                    shortfall: shortfall(),
                },
            )
            .unwrap();

        assert_eq!(changes.changed_shortfall.after.remaining_quote, 0);
        assert_eq!(changes.changed_shortfall.after.status, HyperliquidPerpShortfallStatus::Covered);
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::FundCovering
        );
        assert_eq!(changes.close_as, Some(HyperliquidPerpLiquidationCloseAs::Closed));
        assert_eq!(changes.to_replayable_events().unwrap().len(), 3);

        let close_changes = CloseHyperliquidPerpLiquidationUseCase
            .compute_changes(
                &CloseHyperliquidPerpLiquidationCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    close_as: HyperliquidPerpLiquidationCloseAs::Closed,
                },
                CloseHyperliquidPerpLiquidationState {
                    liquidation: changes.changed_liquidation.after.clone(),
                },
            )
            .unwrap();
        assert_eq!(
            close_changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::Closed
        );
    }
}
