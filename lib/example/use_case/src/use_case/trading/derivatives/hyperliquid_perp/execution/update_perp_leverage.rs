use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, MiStateMachineV2Unchecked};
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpLeverageSetting, HyperliquidPerpLeverageSettingError, HyperliquidPerpMarginMode,
    HyperliquidPerpPosition, HyperliquidPerpPositionError,
};

/// 更新 Hyperliquid perp 杠杆配置的命令。
///
/// 字段语义对齐 Hyperliquid `updateLeverage`：`asset + isCross + leverage`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHyperliquidPerpLeverageCmd {
    /// 发起本次配置更新的账户 ID。
    pub party_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// `true` 表示 Cross，`false` 表示 Isolated。
    pub is_cross: bool,
    /// 目标杠杆值。
    pub leverage: u64,
}

impl IssuedByParty for UpdateHyperliquidPerpLeverageCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 更新 Hyperliquid perp 杠杆配置所需的已加载状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHyperliquidPerpLeverageState {
    /// 当前业务账户 ID。
    pub account_id: String,
    /// 当前 asset + margin_mode 对应的杠杆配置快照。
    pub leverage_setting: HyperliquidPerpLeverageSetting,
    /// 若 adapter 已加载同账户同 asset 的仓位快照，本用例会同步应用杠杆配置变化。
    pub position: Option<HyperliquidPerpPosition>,
}

/// 更新 Hyperliquid perp 杠杆配置可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UpdateHyperliquidPerpLeverageError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 杠杆必须大于等于 1。
    #[error("leverage must be greater than or equal to 1")]
    InvalidLeverage,
    /// 命令账户和已加载账户不一致。
    #[error("state account does not match command party")]
    AccountMismatch,
    /// 已加载配置不属于当前账户，或 asset 不匹配。
    #[error("loaded leverage setting does not match requested account or asset")]
    LeverageSettingMismatch,
    /// 命令的保证金模式和已加载配置不一致。
    #[error("requested margin mode does not match loaded leverage setting")]
    MarginModeMismatch,
    /// 推导版本时发生整数溢出。
    #[error("arithmetic overflow while deriving leverage update")]
    ArithmeticOverflow,
}

/// Use case that updates the authoritative Hyperliquid perp leverage setting.
///
/// 该用例以 `HyperliquidPerpLeverageSetting` 为配置真相；若 state 同时携带对应仓位，则同步更新仓位快照。
#[derive(Debug, Clone, Copy, Default)]
pub struct UpdateHyperliquidPerpLeverageUseCase;

/// 本次杠杆配置更新后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHyperliquidPerpLeverageChanges {
    /// 权威杠杆配置的 before/after。
    pub changed_leverage_setting: UpdatedEntityPair<HyperliquidPerpLeverageSetting>,
    /// 可选仓位快照的 before/after；只有 state 已加载对应仓位时产生。
    pub changed_position: Option<UpdatedEntityPair<HyperliquidPerpPosition>>,
}

impl ReplayableChanges for UpdateHyperliquidPerpLeverageChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = vec![
            self.changed_leverage_setting
                .after
                .track_update_event_from(&self.changed_leverage_setting.before)?,
        ];
        if let Some(position) = &self.changed_position {
            events.push(position.after.track_update_event_from(&position.before)?);
        }
        Ok(events)
    }
}

impl MiStateMachineV2Unchecked for UpdateHyperliquidPerpLeverageUseCase {
    type Command = UpdateHyperliquidPerpLeverageCmd;
    type GivenState = UpdateHyperliquidPerpLeverageState;
    type Error = UpdateHyperliquidPerpLeverageError;
    type AfterChanges = UpdateHyperliquidPerpLeverageChanges;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(UpdateHyperliquidPerpLeverageError::InvalidPartyId);
        }
        if cmd.leverage < 1 {
            return Err(UpdateHyperliquidPerpLeverageError::InvalidLeverage);
        }
        Ok(())
    }

    fn validate_against_given_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.account_id != cmd.party_id {
            return Err(UpdateHyperliquidPerpLeverageError::AccountMismatch);
        }
        if !state.leverage_setting.belongs_to_account(state.account_id.as_str())
            || !state.leverage_setting.trades_asset(cmd.asset)
        {
            return Err(UpdateHyperliquidPerpLeverageError::LeverageSettingMismatch);
        }

        let requested_margin_mode = margin_mode_from_is_cross(cmd.is_cross);
        if !state.leverage_setting.matches_margin_mode(requested_margin_mode) {
            return Err(UpdateHyperliquidPerpLeverageError::MarginModeMismatch);
        }
        if let Some(position) = &state.position {
            if !position.belongs_to_account(state.account_id.as_str())
                || !position.trades_asset(cmd.asset)
            {
                return Err(UpdateHyperliquidPerpLeverageError::LeverageSettingMismatch);
            }
            if position.margin_mode != requested_margin_mode {
                return Err(UpdateHyperliquidPerpLeverageError::MarginModeMismatch);
            }
            if !position.has_consistent_state() {
                return Err(UpdateHyperliquidPerpLeverageError::LeverageSettingMismatch);
            }
        }

        Ok(())
    }

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        let before = state.leverage_setting.clone();
        let after = before.update_leverage(cmd.leverage).map_err(map_leverage_setting_error)?;
        let changed_position = if let Some(position) = &state.position {
            let before_position = position.clone();
            let mut after_position = position.clone();
            after_position
                .apply_leverage_setting(
                    after.account_id.as_str(),
                    after.asset,
                    after.margin_mode,
                    after.leverage,
                )
                .map_err(map_position_error)?;
            Some(UpdatedEntityPair { before: before_position, after: after_position })
        } else {
            None
        };

        Ok(UpdateHyperliquidPerpLeverageChanges {
            changed_leverage_setting: UpdatedEntityPair { before, after },
            changed_position,
        })
    }
}

fn map_position_error(error: HyperliquidPerpPositionError) -> UpdateHyperliquidPerpLeverageError {
    match error {
        HyperliquidPerpPositionError::InvalidLeverage => {
            UpdateHyperliquidPerpLeverageError::InvalidLeverage
        }
        HyperliquidPerpPositionError::MarginModeMismatch => {
            UpdateHyperliquidPerpLeverageError::MarginModeMismatch
        }
        HyperliquidPerpPositionError::ArithmeticOverflow => {
            UpdateHyperliquidPerpLeverageError::ArithmeticOverflow
        }
        HyperliquidPerpPositionError::InvalidTradeQty
        | HyperliquidPerpPositionError::InvalidTradePrice
        | HyperliquidPerpPositionError::InconsistentState => {
            UpdateHyperliquidPerpLeverageError::LeverageSettingMismatch
        }
    }
}

fn margin_mode_from_is_cross(is_cross: bool) -> HyperliquidPerpMarginMode {
    if is_cross { HyperliquidPerpMarginMode::Cross } else { HyperliquidPerpMarginMode::Isolated }
}

fn map_leverage_setting_error(
    error: HyperliquidPerpLeverageSettingError,
) -> UpdateHyperliquidPerpLeverageError {
    match error {
        HyperliquidPerpLeverageSettingError::InvalidLeverage => {
            UpdateHyperliquidPerpLeverageError::InvalidLeverage
        }
        HyperliquidPerpLeverageSettingError::ArithmeticOverflow => {
            UpdateHyperliquidPerpLeverageError::ArithmeticOverflow
        }
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::ReplayableChanges;
    use common_entity::MiStateMachineV2Unchecked;

    use super::*;
    use crate::entity::HyperliquidPerpPositionStatus;

    fn cmd(leverage: u64, is_cross: bool) -> UpdateHyperliquidPerpLeverageCmd {
        UpdateHyperliquidPerpLeverageCmd {
            party_id: "trader-1".to_string(),
            asset: 7,
            is_cross,
            leverage,
        }
    }

    fn setting(margin_mode: HyperliquidPerpMarginMode) -> HyperliquidPerpLeverageSetting {
        HyperliquidPerpLeverageSetting::new("trader-1".to_string(), 7, margin_mode, 5, 3)
    }

    fn open_position(margin_mode: HyperliquidPerpMarginMode) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            "trader-1:BTC-PERP".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            3,
            100,
            5,
            margin_mode,
            0,
            2,
        )
    }

    fn state(
        margin_mode: HyperliquidPerpMarginMode,
        position: Option<HyperliquidPerpPosition>,
    ) -> UpdateHyperliquidPerpLeverageState {
        UpdateHyperliquidPerpLeverageState {
            account_id: "trader-1".to_string(),
            leverage_setting: setting(margin_mode),
            position,
        }
    }

    #[test]
    fn pre_check_rejects_empty_party_and_zero_leverage() {
        let mut invalid_party = cmd(10, true);
        invalid_party.party_id.clear();
        assert_eq!(
            UpdateHyperliquidPerpLeverageUseCase.pre_check_command(&invalid_party),
            Err(UpdateHyperliquidPerpLeverageError::InvalidPartyId)
        );
        assert_eq!(
            UpdateHyperliquidPerpLeverageUseCase.pre_check_command(&cmd(0, true)),
            Err(UpdateHyperliquidPerpLeverageError::InvalidLeverage)
        );
    }

    #[test]
    fn compute_updates_setting_without_loaded_position() {
        let state = state(HyperliquidPerpMarginMode::Cross, None);

        let changes = UpdateHyperliquidPerpLeverageUseCase
            .compute_after_changes_unchecked(&cmd(10, true), &state)
            .unwrap();
        let events = changes.to_replayable_events().unwrap();

        assert_eq!(changes.changed_leverage_setting.before.leverage, 5);
        assert_eq!(changes.changed_leverage_setting.after.leverage, 10);
        assert!(changes.changed_position.is_none());
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn compute_applies_setting_to_loaded_open_position_margin() {
        let state = state(
            HyperliquidPerpMarginMode::Cross,
            Some(open_position(HyperliquidPerpMarginMode::Cross)),
        );

        let changes = UpdateHyperliquidPerpLeverageUseCase
            .compute_after_changes_unchecked(&cmd(10, true), &state)
            .unwrap();
        let position = changes.changed_position.as_ref().unwrap();
        let events = changes.to_replayable_events().unwrap();

        assert_eq!(position.before.required_margin(), Some(60));
        assert_eq!(position.after.leverage_value, 10);
        assert_eq!(position.after.required_margin(), Some(30));
        assert_eq!(position.after.version, 3);
        assert_eq!(position.after.lifecycle_status(), HyperliquidPerpPositionStatus::Open);
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn compute_supports_isolated_setting_when_state_matches() {
        let state = state(
            HyperliquidPerpMarginMode::Isolated,
            Some(open_position(HyperliquidPerpMarginMode::Isolated)),
        );

        let changes = UpdateHyperliquidPerpLeverageUseCase
            .compute_after_changes_unchecked(&cmd(10, false), &state)
            .unwrap();

        assert_eq!(
            changes.changed_leverage_setting.after.margin_mode,
            HyperliquidPerpMarginMode::Isolated
        );
        assert_eq!(changes.changed_position.as_ref().unwrap().after.required_margin(), Some(30));
    }

    #[test]
    fn validate_rejects_position_margin_mode_mismatch() {
        let state = state(
            HyperliquidPerpMarginMode::Cross,
            Some(open_position(HyperliquidPerpMarginMode::Isolated)),
        );

        assert_eq!(
            UpdateHyperliquidPerpLeverageUseCase
                .validate_against_given_state(&cmd(10, true), &state),
            Err(UpdateHyperliquidPerpLeverageError::MarginModeMismatch)
        );
    }
}
