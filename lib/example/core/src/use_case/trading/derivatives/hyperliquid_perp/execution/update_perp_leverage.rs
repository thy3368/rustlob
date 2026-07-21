use cmd_handler::command_use_case_def2::{
    EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use cmd_handler::EntityReplayableEvent;
use common_entity::{Entity, MiStateMachineV2Unchecked};
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpLeverageSetting, HyperliquidPerpLeverageSettingError, HyperliquidPerpMarginMode,
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
                let after =
                    before.update_leverage(cmd.leverage).map_err(map_leverage_setting_error)?;
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
