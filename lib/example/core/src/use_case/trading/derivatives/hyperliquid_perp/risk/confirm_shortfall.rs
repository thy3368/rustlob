use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use super::close_liquidation::HyperliquidPerpLiquidationCloseAs;
use crate::entity::{
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationFill, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpShortfall,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmHyperliquidPerpShortfallCmd {
    pub party_id: String,
    pub liquidation_id: String,
    pub shortfall_id: String,
    pub required_quote: u64,
    pub recovered_quote: u64,
}

impl IssuedByParty for ConfirmHyperliquidPerpShortfallCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmHyperliquidPerpShortfallState {
    pub liquidation: HyperliquidPerpLiquidation,
    pub fills: Vec<HyperliquidPerpLiquidationFill>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfirmHyperliquidPerpShortfallError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("shortfall_id must not be empty")]
    InvalidShortfallId,
    #[error("required_quote must be greater than zero")]
    InvalidRequiredQuote,
    #[error("liquidation does not match command liquidation id")]
    LiquidationMismatch,
    #[error("liquidation is not ready for shortfall confirmation")]
    LiquidationNotReady,
    #[error("fill recovered quote does not match command")]
    RecoveredQuoteMismatch,
    #[error("fill does not belong to liquidation")]
    FillMismatch,
    #[error("arithmetic overflow while confirming shortfall")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ConfirmHyperliquidPerpShortfallUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmHyperliquidPerpShortfallChanges {
    pub created_shortfall: Option<HyperliquidPerpShortfall>,
    pub changed_liquidation: Option<UpdatedEntityPair<HyperliquidPerpLiquidation>>,
    pub close_as: Option<HyperliquidPerpLiquidationCloseAs>,
}

impl ReplayableChanges for ConfirmHyperliquidPerpShortfallChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::new();
        if let Some(shortfall) = &self.created_shortfall {
            events.push(shortfall.track_create_event()?);
        }
        if let Some(liquidation) = &self.changed_liquidation {
            events.push(liquidation.after.track_update_event_from(&liquidation.before)?);
        }
        Ok(events)
    }
}

impl CommandUseCase4 for ConfirmHyperliquidPerpShortfallUseCase {
    type Command = ConfirmHyperliquidPerpShortfallCmd;
    type GivenState = ConfirmHyperliquidPerpShortfallState;
    type Error = ConfirmHyperliquidPerpShortfallError;
    type Changes = ConfirmHyperliquidPerpShortfallChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(ConfirmHyperliquidPerpShortfallError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(ConfirmHyperliquidPerpShortfallError::InvalidLiquidationId);
        }
        if cmd.shortfall_id.is_empty() {
            return Err(ConfirmHyperliquidPerpShortfallError::InvalidShortfallId);
        }
        if cmd.required_quote == 0 {
            return Err(ConfirmHyperliquidPerpShortfallError::InvalidRequiredQuote);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.liquidation.liquidation_id != cmd.liquidation_id {
            return Err(ConfirmHyperliquidPerpShortfallError::LiquidationMismatch);
        }
        if !matches!(
            state.liquidation.status,
            HyperliquidPerpLiquidationStatus::Started | HyperliquidPerpLiquidationStatus::Executing
        ) || state.liquidation.remaining_qty != 0
        {
            return Err(ConfirmHyperliquidPerpShortfallError::LiquidationNotReady);
        }
        let recovered_sum = state.fills.iter().try_fold(0_u64, |acc, fill| {
            if fill.liquidation_id != cmd.liquidation_id {
                return None;
            }
            acc.checked_add(fill.recovered_quote)
        });
        let Some(recovered_sum) = recovered_sum else {
            return Err(ConfirmHyperliquidPerpShortfallError::FillMismatch);
        };
        if recovered_sum != cmd.recovered_quote {
            return Err(ConfirmHyperliquidPerpShortfallError::RecoveredQuoteMismatch);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        if cmd.recovered_quote >= cmd.required_quote {
            return Ok(ConfirmHyperliquidPerpShortfallChanges {
                created_shortfall: None,
                changed_liquidation: None,
                close_as: Some(HyperliquidPerpLiquidationCloseAs::Closed),
            });
        }

        let shortfall_quote = cmd
            .required_quote
            .checked_sub(cmd.recovered_quote)
            .ok_or(ConfirmHyperliquidPerpShortfallError::ArithmeticOverflow)?;
        let created_shortfall = HyperliquidPerpShortfall::new(
            cmd.shortfall_id.clone(),
            cmd.liquidation_id.clone(),
            state.liquidation.account_id.clone(),
            state.liquidation.position_id.clone(),
            state.liquidation.asset,
            state.liquidation.symbol.clone(),
            state.liquidation.bankruptcy_price,
            cmd.required_quote,
            cmd.recovered_quote,
            shortfall_quote,
        );
        let before = state.liquidation;
        let mut after = before.clone();
        let next_version = after
            .version
            .checked_add(1)
            .ok_or(ConfirmHyperliquidPerpShortfallError::ArithmeticOverflow)?;
        after
            .mark_shortfall_assessed(next_version)
            .ok_or(ConfirmHyperliquidPerpShortfallError::ArithmeticOverflow)?;

        Ok(ConfirmHyperliquidPerpShortfallChanges {
            created_shortfall: Some(created_shortfall),
            changed_liquidation: Some(UpdatedEntityPair { before, after }),
            close_as: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::CommandUseCase4;

    use super::*;
    use crate::entity::{
        HyperliquidPerpLiquidationFill, HyperliquidPerpLiquidationTriggerReason,
        HyperliquidPerpMarginMode,
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
            2,
            2,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::Executing,
        );
        liquidation.remaining_qty = 0;
        liquidation
    }

    fn fill() -> HyperliquidPerpLiquidationFill {
        HyperliquidPerpLiquidationFill::new(
            "liq-1-trade-1".to_string(),
            "liq-1".to_string(),
            "order-1".to_string(),
            "trade-1".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            2,
            2,
            48_000,
            50_000,
            96_000,
        )
    }

    #[test]
    fn no_shortfall_path_only_returns_close_signal() {
        let changes = ConfirmHyperliquidPerpShortfallUseCase
            .compute_changes(
                &ConfirmHyperliquidPerpShortfallCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    shortfall_id: "shortfall-1".to_string(),
                    required_quote: 96_000,
                    recovered_quote: 96_000,
                },
                ConfirmHyperliquidPerpShortfallState {
                    liquidation: liquidation(),
                    fills: vec![fill()],
                },
            )
            .unwrap();

        assert_eq!(changes.close_as, Some(HyperliquidPerpLiquidationCloseAs::Closed));
        assert!(changes.created_shortfall.is_none());
    }
}
