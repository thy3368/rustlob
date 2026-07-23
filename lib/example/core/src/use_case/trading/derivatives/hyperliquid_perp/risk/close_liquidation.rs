use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus};

/// 关闭强平会话时允许写入的终态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpLiquidationCloseAs {
    /// 当前强平流程已经正常闭环。
    Closed,
    /// 当前强平流程在本组边界内已经穷尽，需要转交更高风险处置路径。
    Exhausted,
}

/// 显式关闭一个 Hyperliquid perp 强平会话。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseHyperliquidPerpLiquidationCmd {
    /// 发起关闭动作的业务主体。
    pub party_id: String,
    /// 要关闭的强平会话 ID。
    pub liquidation_id: String,
    /// 期望写入的终态。
    pub close_as: HyperliquidPerpLiquidationCloseAs,
}

impl IssuedByParty for CloseHyperliquidPerpLiquidationCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 关闭强平会话时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseHyperliquidPerpLiquidationState {
    /// 当前强平会话。
    pub liquidation: HyperliquidPerpLiquidation,
}

/// Close 强平会话可能出现的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CloseHyperliquidPerpLiquidationError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("liquidation does not match command liquidation id")]
    LiquidationMismatch,
    #[error("liquidation status does not allow close")]
    LiquidationNotClosable,
    #[error("liquidation remaining qty must be zero when closing as closed")]
    RemainingQtyNotZero,
    #[error("arithmetic overflow while closing liquidation")]
    ArithmeticOverflow,
}

/// 将强平会话推进到 `Closed | Exhausted` 终态的 use case。
#[derive(Debug, Clone, Copy, Default)]
pub struct CloseHyperliquidPerpLiquidationUseCase;

/// Close 后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseHyperliquidPerpLiquidationChanges {
    /// 强平会话的 before/after 对。
    pub changed_liquidation: UpdatedEntityPair<HyperliquidPerpLiquidation>,
}

impl ReplayableChanges for CloseHyperliquidPerpLiquidationChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        Ok(vec![
            self.changed_liquidation
                .after
                .track_update_event_from(&self.changed_liquidation.before)?,
        ])
    }
}

impl CommandUseCase4 for CloseHyperliquidPerpLiquidationUseCase {
    type Command = CloseHyperliquidPerpLiquidationCmd;
    type GivenState = CloseHyperliquidPerpLiquidationState;
    type Error = CloseHyperliquidPerpLiquidationError;
    type Changes = CloseHyperliquidPerpLiquidationChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(CloseHyperliquidPerpLiquidationError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(CloseHyperliquidPerpLiquidationError::InvalidLiquidationId);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.liquidation.liquidation_id != cmd.liquidation_id {
            return Err(CloseHyperliquidPerpLiquidationError::LiquidationMismatch);
        }
        if !matches!(
            state.liquidation.status,
            HyperliquidPerpLiquidationStatus::Started
                | HyperliquidPerpLiquidationStatus::Executing
                | HyperliquidPerpLiquidationStatus::ShortfallAssessed
                | HyperliquidPerpLiquidationStatus::FundCovering
                | HyperliquidPerpLiquidationStatus::AdlCovering
        ) {
            return Err(CloseHyperliquidPerpLiquidationError::LiquidationNotClosable);
        }
        if matches!(cmd.close_as, HyperliquidPerpLiquidationCloseAs::Closed)
            && state.liquidation.remaining_qty != 0
        {
            return Err(CloseHyperliquidPerpLiquidationError::RemainingQtyNotZero);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let before = state.liquidation;
        let mut after = before.clone();
        let next_version = after
            .version
            .checked_add(1)
            .ok_or(CloseHyperliquidPerpLiquidationError::ArithmeticOverflow)?;

        let applied = match cmd.close_as {
            HyperliquidPerpLiquidationCloseAs::Closed => after.apply_closed(next_version),
            HyperliquidPerpLiquidationCloseAs::Exhausted => after.apply_exhausted(next_version),
        };
        applied.ok_or(CloseHyperliquidPerpLiquidationError::ArithmeticOverflow)?;

        Ok(CloseHyperliquidPerpLiquidationChanges {
            changed_liquidation: UpdatedEntityPair { before, after },
        })
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::entity::{HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode};

    fn liquidation(
        status: HyperliquidPerpLiquidationStatus,
        remaining_qty: u64,
    ) -> HyperliquidPerpLiquidation {
        let mut liquidation = HyperliquidPerpLiquidation::new(
            "liq-1".to_string(),
            "liq-batch-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            10,
            10,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            status,
        );
        liquidation.remaining_qty = remaining_qty;
        liquidation
    }

    fn cmd(close_as: HyperliquidPerpLiquidationCloseAs) -> CloseHyperliquidPerpLiquidationCmd {
        CloseHyperliquidPerpLiquidationCmd {
            party_id: "risk-engine".to_string(),
            liquidation_id: "liq-1".to_string(),
            close_as,
        }
    }

    #[test]
    fn role_returns_risk_engine() {
        assert_eq!(CloseHyperliquidPerpLiquidationUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_blank_liquidation_id() {
        let mut cmd = cmd(HyperliquidPerpLiquidationCloseAs::Closed);
        cmd.liquidation_id.clear();

        assert_eq!(
            CloseHyperliquidPerpLiquidationUseCase.pre_check_command(&cmd),
            Err(CloseHyperliquidPerpLiquidationError::InvalidLiquidationId)
        );
    }

    #[test]
    fn validate_rejects_close_as_closed_when_remaining_qty_is_not_zero() {
        let state = CloseHyperliquidPerpLiquidationState {
            liquidation: liquidation(HyperliquidPerpLiquidationStatus::Executing, 3),
        };

        assert_eq!(
            CloseHyperliquidPerpLiquidationUseCase
                .validate_against_state(&cmd(HyperliquidPerpLiquidationCloseAs::Closed), &state),
            Err(CloseHyperliquidPerpLiquidationError::RemainingQtyNotZero)
        );
    }

    #[test]
    fn risk_engine_close_moves_zero_remaining_session_to_closed_and_projects_single_update_event() {
        let use_case = CloseHyperliquidPerpLiquidationUseCase;
        let state = CloseHyperliquidPerpLiquidationState {
            liquidation: liquidation(HyperliquidPerpLiquidationStatus::Executing, 0),
        };

        let changes = use_case
            .compute_changes(&cmd(HyperliquidPerpLiquidationCloseAs::Closed), state)
            .unwrap();
        let events = changes.to_replayable_events().unwrap();

        assert_eq!(
            changes.changed_liquidation.before.status,
            HyperliquidPerpLiquidationStatus::Executing
        );
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::Closed
        );
        assert_eq!(changes.changed_liquidation.before.version, 1);
        assert_eq!(changes.changed_liquidation.after.version, 2);

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"closed"
        }));
    }

    #[test]
    fn risk_engine_close_moves_started_session_to_exhausted_and_projects_single_update_event() {
        let use_case = CloseHyperliquidPerpLiquidationUseCase;
        let state = CloseHyperliquidPerpLiquidationState {
            liquidation: liquidation(HyperliquidPerpLiquidationStatus::Started, 7),
        };

        let changes = use_case
            .compute_changes(&cmd(HyperliquidPerpLiquidationCloseAs::Exhausted), state)
            .unwrap();
        let events = changes.to_replayable_events().unwrap();

        assert_eq!(
            changes.changed_liquidation.before.status,
            HyperliquidPerpLiquidationStatus::Started
        );
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::Exhausted
        );
        assert_eq!(changes.changed_liquidation.before.version, 1);
        assert_eq!(changes.changed_liquidation.after.version, 2);

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"exhausted"
        }));
    }
}
