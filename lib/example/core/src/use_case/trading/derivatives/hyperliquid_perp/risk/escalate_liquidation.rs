use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus};

/// 将一个 Hyperliquid perp 强平会话升级到更高风险处置路径。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscalateHyperliquidPerpLiquidationCmd {
    /// 发起升级动作的业务主体。
    pub party_id: String,
    /// 要升级的强平会话 ID。
    pub liquidation_id: String,
}

impl IssuedByParty for EscalateHyperliquidPerpLiquidationCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 升级强平会话时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscalateHyperliquidPerpLiquidationState {
    /// 当前强平会话。
    pub liquidation: HyperliquidPerpLiquidation,
}

/// Escalate 强平会话可能出现的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EscalateHyperliquidPerpLiquidationError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("liquidation does not match command liquidation id")]
    LiquidationMismatch,
    #[error("liquidation status does not allow escalate")]
    LiquidationNotEscalatable,
    #[error("arithmetic overflow while escalating liquidation")]
    ArithmeticOverflow,
}

/// Use case that escalates one Hyperliquid perp liquidation session.
#[derive(Debug, Clone, Copy, Default)]
pub struct EscalateHyperliquidPerpLiquidationUseCase;

/// Escalate 后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscalateHyperliquidPerpLiquidationChanges {
    /// 强平会话的 before/after 对。
    pub changed_liquidation: UpdatedEntityPair<HyperliquidPerpLiquidation>,
}

impl ReplayableChanges for EscalateHyperliquidPerpLiquidationChanges {
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

impl CommandUseCase4 for EscalateHyperliquidPerpLiquidationUseCase {
    type Command = EscalateHyperliquidPerpLiquidationCmd;
    type GivenState = EscalateHyperliquidPerpLiquidationState;
    type Error = EscalateHyperliquidPerpLiquidationError;
    type Changes = EscalateHyperliquidPerpLiquidationChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(EscalateHyperliquidPerpLiquidationError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(EscalateHyperliquidPerpLiquidationError::InvalidLiquidationId);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.liquidation.liquidation_id != cmd.liquidation_id {
            return Err(EscalateHyperliquidPerpLiquidationError::LiquidationMismatch);
        }
        if !matches!(
            state.liquidation.status,
            HyperliquidPerpLiquidationStatus::Started
                | HyperliquidPerpLiquidationStatus::OrderPlaced
        ) {
            return Err(EscalateHyperliquidPerpLiquidationError::LiquidationNotEscalatable);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let before = state.liquidation;
        let mut after = before.clone();
        let next_version = after
            .version
            .checked_add(1)
            .ok_or(EscalateHyperliquidPerpLiquidationError::ArithmeticOverflow)?;
        after
            .apply_escalated(next_version)
            .ok_or(EscalateHyperliquidPerpLiquidationError::ArithmeticOverflow)?;

        Ok(EscalateHyperliquidPerpLiquidationChanges {
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

    fn liquidation(status: HyperliquidPerpLiquidationStatus) -> HyperliquidPerpLiquidation {
        HyperliquidPerpLiquidation::new(
            "liq-1".to_string(),
            "liq-batch-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            10,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            status,
        )
    }

    fn cmd() -> EscalateHyperliquidPerpLiquidationCmd {
        EscalateHyperliquidPerpLiquidationCmd {
            party_id: "risk-engine".to_string(),
            liquidation_id: "liq-1".to_string(),
        }
    }

    #[test]
    fn role_returns_risk_engine() {
        assert_eq!(EscalateHyperliquidPerpLiquidationUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_blank_liquidation_id() {
        let mut cmd = cmd();
        cmd.liquidation_id.clear();

        assert_eq!(
            EscalateHyperliquidPerpLiquidationUseCase.pre_check_command(&cmd),
            Err(EscalateHyperliquidPerpLiquidationError::InvalidLiquidationId)
        );
    }

    #[test]
    fn validate_rejects_terminal_status() {
        let state = EscalateHyperliquidPerpLiquidationState {
            liquidation: liquidation(HyperliquidPerpLiquidationStatus::Resolved),
        };

        assert_eq!(
            EscalateHyperliquidPerpLiquidationUseCase.validate_against_state(&cmd(), &state),
            Err(EscalateHyperliquidPerpLiquidationError::LiquidationNotEscalatable)
        );
    }

    #[test]
    fn risk_engine_escalate_moves_started_session_to_escalated_and_projects_single_update_event() {
        // Rule:
        // - 风险引擎可以把仍处于处理中状态的强平会话升级到更高风险处置路径。
        //
        // Given:
        // - 一个 `Started` 状态的强平会话。
        //
        // When:
        // - 调用 `compute_changes()` 并再投影 `to_replayable_events()`。
        //
        // Then:
        // - before/after pair 显示状态从 `Started` 推进到 `Escalated`。
        // - 版本号自增 1。
        // - 最终只投影出 1 条 liquidation update event。

        // arrange
        let use_case = EscalateHyperliquidPerpLiquidationUseCase;
        let state = EscalateHyperliquidPerpLiquidationState {
            liquidation: liquidation(HyperliquidPerpLiquidationStatus::Started),
        };

        // act
        let changes = use_case.compute_changes(&cmd(), state).unwrap();
        let events = changes.to_replayable_events().unwrap();

        // assert
        assert_eq!(
            changes.changed_liquidation.before.status,
            HyperliquidPerpLiquidationStatus::Started
        );
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::Escalated
        );
        assert_eq!(changes.changed_liquidation.before.version, 1);
        assert_eq!(changes.changed_liquidation.after.version, 2);

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"escalated"
        }));
    }
}
