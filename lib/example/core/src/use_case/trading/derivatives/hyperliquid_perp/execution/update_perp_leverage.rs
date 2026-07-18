use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, MiStateMachineV2Unchecked};
use thiserror::Error;

use crate::entity::{HyperliquidPerpLeverageSetting, HyperliquidPerpMarginMode};

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
    /// 当前版本暂不支持 Isolated 杠杆配置更新。
    #[error("margin mode is not supported yet")]
    UnsupportedMarginMode,
    /// 推导版本时发生整数溢出。
    #[error("arithmetic overflow while deriving leverage update")]
    ArithmeticOverflow,
}

/// Use case that updates the authoritative Hyperliquid perp leverage setting.
///
/// 该用例只更新配置实体 `HyperliquidPerpLeverageSetting`，不修改仓位事实，也不重算任何仓位保证金。
#[derive(Debug, Clone, Copy, Default)]
pub struct UpdateHyperliquidPerpLeverageUseCase;

/// 本次杠杆配置更新后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHyperliquidPerpLeverageChanges {
    /// 权威杠杆配置的 before/after。
    pub changed_leverage_setting: UpdatedEntityPair<HyperliquidPerpLeverageSetting>,
}

impl ReplayableChanges for UpdateHyperliquidPerpLeverageChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        Ok(vec![
            self.changed_leverage_setting
                .after
                .track_update_event_from(&self.changed_leverage_setting.before)?,
        ])
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

        Ok(())
    }

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        match margin_mode_from_is_cross(cmd.is_cross) {
            HyperliquidPerpMarginMode::Cross => {
                let before = state.leverage_setting.clone();
                let next_version = before
                    .version
                    .checked_add(1)
                    .ok_or(UpdateHyperliquidPerpLeverageError::ArithmeticOverflow)?;
                let mut after = before.clone();
                after.apply_new_leverage(cmd.leverage, next_version);
                Ok(UpdateHyperliquidPerpLeverageChanges {
                    changed_leverage_setting: UpdatedEntityPair { before, after },
                })
            }
            HyperliquidPerpMarginMode::Isolated => {
                Err(UpdateHyperliquidPerpLeverageError::UnsupportedMarginMode)
            }
        }
    }
}

fn margin_mode_from_is_cross(is_cross: bool) -> HyperliquidPerpMarginMode {
    if is_cross { HyperliquidPerpMarginMode::Cross } else { HyperliquidPerpMarginMode::Isolated }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cross_setting() -> HyperliquidPerpLeverageSetting {
        HyperliquidPerpLeverageSetting::new(
            "trader-1".to_string(),
            7,
            HyperliquidPerpMarginMode::Cross,
            5,
            3,
        )
    }

    fn cross_state() -> UpdateHyperliquidPerpLeverageState {
        UpdateHyperliquidPerpLeverageState {
            account_id: "trader-1".to_string(),
            leverage_setting: cross_setting(),
        }
    }

    fn cross_cmd() -> UpdateHyperliquidPerpLeverageCmd {
        UpdateHyperliquidPerpLeverageCmd {
            party_id: "trader-1".to_string(),
            asset: 7,
            is_cross: true,
            leverage: 10,
        }
    }

    #[test]
    fn pre_check_rejects_empty_party_id() {
        let cmd = UpdateHyperliquidPerpLeverageCmd { party_id: String::new(), ..cross_cmd() };

        let result = UpdateHyperliquidPerpLeverageUseCase.pre_check_command(&cmd);

        assert_eq!(result, Err(UpdateHyperliquidPerpLeverageError::InvalidPartyId));
    }

    #[test]
    fn pre_check_rejects_zero_leverage() {
        let cmd = UpdateHyperliquidPerpLeverageCmd { leverage: 0, ..cross_cmd() };

        let result = UpdateHyperliquidPerpLeverageUseCase.pre_check_command(&cmd);

        assert_eq!(result, Err(UpdateHyperliquidPerpLeverageError::InvalidLeverage));
    }

    #[test]
    fn validate_rejects_account_mismatch() {
        let cmd = UpdateHyperliquidPerpLeverageCmd {
            party_id: "other-trader".to_string(),
            ..cross_cmd()
        };

        let result =
            UpdateHyperliquidPerpLeverageUseCase.validate_against_given_state(&cmd, &cross_state());

        assert_eq!(result, Err(UpdateHyperliquidPerpLeverageError::AccountMismatch));
    }

    #[test]
    fn validate_rejects_asset_mismatch() {
        let cmd = UpdateHyperliquidPerpLeverageCmd { asset: 9, ..cross_cmd() };

        let result =
            UpdateHyperliquidPerpLeverageUseCase.validate_against_given_state(&cmd, &cross_state());

        assert_eq!(result, Err(UpdateHyperliquidPerpLeverageError::LeverageSettingMismatch));
    }

    #[test]
    fn validate_rejects_margin_mode_mismatch() {
        let cmd = UpdateHyperliquidPerpLeverageCmd { is_cross: false, ..cross_cmd() };

        let result =
            UpdateHyperliquidPerpLeverageUseCase.validate_against_given_state(&cmd, &cross_state());

        assert_eq!(result, Err(UpdateHyperliquidPerpLeverageError::MarginModeMismatch));
    }

    #[test]
    fn compute_changes_updates_cross_leverage_setting() {
        let changes = UpdateHyperliquidPerpLeverageUseCase
            .compute_after_changes_unchecked(&cross_cmd(), &cross_state())
            .unwrap();

        assert_eq!(changes.changed_leverage_setting.before.leverage, 5);
        assert_eq!(changes.changed_leverage_setting.after.leverage, 10);
        assert_eq!(changes.changed_leverage_setting.before.version, 3);
        assert_eq!(changes.changed_leverage_setting.after.version, 4);
    }

    #[test]
    fn compute_changes_rejects_isolated_path_for_now() {
        let cmd = UpdateHyperliquidPerpLeverageCmd { is_cross: false, ..cross_cmd() };
        let state = UpdateHyperliquidPerpLeverageState {
            account_id: "trader-1".to_string(),
            leverage_setting: HyperliquidPerpLeverageSetting::new(
                "trader-1".to_string(),
                7,
                HyperliquidPerpMarginMode::Isolated,
                3,
                2,
            ),
        };

        let result =
            UpdateHyperliquidPerpLeverageUseCase.compute_after_changes_unchecked(&cmd, &state);

        assert_eq!(result, Err(UpdateHyperliquidPerpLeverageError::UnsupportedMarginMode));
    }

    #[test]
    fn replayable_events_project_updated_leverage_field() {
        let changes = UpdateHyperliquidPerpLeverageUseCase
            .compute_after_changes_unchecked(&cross_cmd(), &cross_state())
            .unwrap();

        let events = changes.to_replayable_events().unwrap();

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert_eq!(events[0].old_version, 3);
        assert_eq!(events[0].new_version, 4);
        assert_eq!(events[0].field_changes.len(), 1);
        assert_eq!(events[0].field_changes[0].field_name_as_str().ok(), Some("leverage"));
        assert_eq!(events[0].field_changes[0].old_value_bytes(), b"5");
        assert_eq!(events[0].field_changes[0].new_value_bytes(), b"10");
    }
}
