use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus};

/// 关闭一个已完成处置的 Hyperliquid perp 强平会话。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveHyperliquidPerpLiquidationCmd {
    /// 发起关闭动作的业务主体。
    pub party_id: String,
    /// 要关闭的强平会话 ID。
    pub liquidation_id: String,
}

impl IssuedByParty for ResolveHyperliquidPerpLiquidationCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 关闭强平会话时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveHyperliquidPerpLiquidationState {
    /// 当前强平会话。
    pub liquidation: HyperliquidPerpLiquidation,
}

/// Resolve 强平会话可能出现的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ResolveHyperliquidPerpLiquidationError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("liquidation does not match command liquidation id")]
    LiquidationMismatch,
    #[error("liquidation status does not allow resolve")]
    LiquidationNotResolvable,
    #[error("liquidation remaining qty must be zero")]
    RemainingQtyNotZero,
    #[error("arithmetic overflow while resolving liquidation")]
    ArithmeticOverflow,
}

/// Use case that closes a Hyperliquid perp liquidation session as resolved.
#[derive(Debug, Clone, Copy, Default)]
pub struct ResolveHyperliquidPerpLiquidationUseCase;

/// Resolve 后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveHyperliquidPerpLiquidationChanges {
    /// 强平会话的 before/after 对。
    pub changed_liquidation: UpdatedEntityPair<HyperliquidPerpLiquidation>,
}

impl ReplayableChanges for ResolveHyperliquidPerpLiquidationChanges {
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

impl CommandUseCase4 for ResolveHyperliquidPerpLiquidationUseCase {
    type Command = ResolveHyperliquidPerpLiquidationCmd;
    type GivenState = ResolveHyperliquidPerpLiquidationState;
    type Error = ResolveHyperliquidPerpLiquidationError;
    type Changes = ResolveHyperliquidPerpLiquidationChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(ResolveHyperliquidPerpLiquidationError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(ResolveHyperliquidPerpLiquidationError::InvalidLiquidationId);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.liquidation.liquidation_id != cmd.liquidation_id {
            return Err(ResolveHyperliquidPerpLiquidationError::LiquidationMismatch);
        }
        if !matches!(
            state.liquidation.status,
            HyperliquidPerpLiquidationStatus::Started
                | HyperliquidPerpLiquidationStatus::OrderPlaced
        ) {
            return Err(ResolveHyperliquidPerpLiquidationError::LiquidationNotResolvable);
        }
        if state.liquidation.remaining_qty != 0 {
            return Err(ResolveHyperliquidPerpLiquidationError::RemainingQtyNotZero);
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
            .ok_or(ResolveHyperliquidPerpLiquidationError::ArithmeticOverflow)?;
        after
            .apply_resolved(next_version)
            .ok_or(ResolveHyperliquidPerpLiquidationError::ArithmeticOverflow)?;

        Ok(ResolveHyperliquidPerpLiquidationChanges {
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
            HyperliquidPerpPositionSide::Long,
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

    fn cmd() -> ResolveHyperliquidPerpLiquidationCmd {
        ResolveHyperliquidPerpLiquidationCmd {
            party_id: "risk-engine".to_string(),
            liquidation_id: "liq-1".to_string(),
        }
    }

    #[test]
    fn role_returns_risk_engine() {
        assert_eq!(ResolveHyperliquidPerpLiquidationUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_blank_liquidation_id() {
        let mut cmd = cmd();
        cmd.liquidation_id.clear();

        assert_eq!(
            ResolveHyperliquidPerpLiquidationUseCase.pre_check_command(&cmd),
            Err(ResolveHyperliquidPerpLiquidationError::InvalidLiquidationId)
        );
    }

    #[test]
    fn validate_rejects_when_remaining_qty_is_not_zero() {
        let state = ResolveHyperliquidPerpLiquidationState {
            liquidation: liquidation(HyperliquidPerpLiquidationStatus::OrderPlaced, 3),
        };

        assert_eq!(
            ResolveHyperliquidPerpLiquidationUseCase.validate_against_state(&cmd(), &state),
            Err(ResolveHyperliquidPerpLiquidationError::RemainingQtyNotZero)
        );
    }

    #[test]
    fn risk_engine_resolve_closes_zero_remaining_session_and_projects_single_update_event() {
        // Rule:
        // - 强平会话只有在剩余待处置数量已经归零时，才能进入 `Resolved`。
        //
        // Given:
        // - 一个 `OrderPlaced` 状态且 `remaining_qty == 0` 的强平会话。
        //
        // When:
        // - 调用 `compute_changes()` 并再投影 `to_replayable_events()`。
        //
        // Then:
        // - before/after pair 显示状态从 `OrderPlaced` 推进到 `Resolved`。
        // - 版本号自增 1。
        // - 最终只投影出 1 条 liquidation update event。

        // arrange
        let use_case = ResolveHyperliquidPerpLiquidationUseCase;
        let state = ResolveHyperliquidPerpLiquidationState {
            liquidation: liquidation(HyperliquidPerpLiquidationStatus::OrderPlaced, 0),
        };

        // act
        let changes = use_case.compute_changes(&cmd(), state).unwrap();
        let events = changes.to_replayable_events().unwrap();

        // assert
        assert_eq!(
            changes.changed_liquidation.before.status,
            HyperliquidPerpLiquidationStatus::OrderPlaced
        );
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::Resolved
        );
        assert_eq!(changes.changed_liquidation.before.version, 1);
        assert_eq!(changes.changed_liquidation.after.version, 2);

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"resolved"
        }));
    }
}
