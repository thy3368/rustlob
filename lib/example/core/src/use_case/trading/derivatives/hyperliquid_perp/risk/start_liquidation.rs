use cmd_handler::command_use_case_def2::{CommandUseCase3, IssuedByParty, UseCaseOutput};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    Balance, HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode, HyperliquidPerpPosition,
};

/// 启动单个 Hyperliquid perp 仓位强平流程的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpLiquidationCmd {
    /// 发起强平的业务主体。
    pub party_id: String,
    /// 强平批次 ID。
    pub liquidation_batch_id: String,
    /// 被强平仓位 ID。
    pub position_id: String,
    /// 触发判定使用的标记价格。
    pub mark_price: u64,
    /// 当前仓位的破产价格。
    pub bankruptcy_price: u64,
    /// 触发原因。
    pub trigger_reason: HyperliquidPerpLiquidationTriggerReason,
    /// 当前仓位保证金模式。
    pub margin_mode: HyperliquidPerpMarginMode,
}

impl IssuedByParty for StartHyperliquidPerpLiquidationCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 启动强平所需的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpLiquidationState {
    /// 被检查的仓位快照。
    pub position: HyperliquidPerpPosition,
    /// 对应账户在保证金币种上的余额快照。
    pub margin_balance: Balance,
    /// 保证金币种，例如 `USDC`。
    pub margin_asset_id: String,
    /// 当前已经进入强平流程的仓位 ID。
    pub existing_liquidation_position_ids: Vec<String>,
}

/// 启动 Hyperliquid perp 强平的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StartHyperliquidPerpLiquidationError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_batch_id must not be empty")]
    InvalidLiquidationBatchId,
    #[error("position_id must not be empty")]
    InvalidPositionId,
    #[error("mark_price must be greater than zero")]
    InvalidMarkPrice,
    #[error("bankruptcy_price must be greater than zero")]
    InvalidBankruptcyPrice,
    #[error("position does not match command position id")]
    PositionMismatch,
    #[error("position is not liquidatable")]
    PositionNotLiquidatable,
    #[error("position state is inconsistent")]
    InconsistentPositionState,
    #[error("margin balance does not belong to position account")]
    MarginBalanceAccountMismatch,
    #[error("margin balance asset does not match state margin asset")]
    InvalidMarginBalance,
    #[error("position is already in liquidation")]
    PositionAlreadyInLiquidation,
    #[error("liquidation trigger condition is not met")]
    LiquidationConditionNotMet,
    #[error("arithmetic overflow while deriving liquidation fact")]
    ArithmeticOverflow,
}

/// 启动单个 Hyperliquid perp 强平流程。
///
/// 该用例只负责创建一条强平开始事实，不修改仓位和余额，也不创建自动减仓订单。
#[derive(Debug, Clone, Copy, Default)]
pub struct StartHyperliquidPerpLiquidationUseCase;

/// 启动强平后的业务产出。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperliquidPerpLiquidationOutput {
    /// 新创建的强平事实。
    pub liquidation: HyperliquidPerpLiquidation,
}

impl CommandUseCase3 for StartHyperliquidPerpLiquidationUseCase {
    type Command = StartHyperliquidPerpLiquidationCmd;
    type GivenState = StartHyperliquidPerpLiquidationState;
    type Error = StartHyperliquidPerpLiquidationError;
    type Output = StartHyperliquidPerpLiquidationOutput;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(StartHyperliquidPerpLiquidationError::InvalidPartyId);
        }
        if cmd.liquidation_batch_id.is_empty() {
            return Err(StartHyperliquidPerpLiquidationError::InvalidLiquidationBatchId);
        }
        if cmd.position_id.is_empty() {
            return Err(StartHyperliquidPerpLiquidationError::InvalidPositionId);
        }
        if cmd.mark_price == 0 {
            return Err(StartHyperliquidPerpLiquidationError::InvalidMarkPrice);
        }
        if cmd.bankruptcy_price == 0 {
            return Err(StartHyperliquidPerpLiquidationError::InvalidBankruptcyPrice);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        validate_state(cmd, state)
    }

    fn compute_output_and_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {
        validate_state(cmd, &state)?;

        let liquidation = HyperliquidPerpLiquidation::new(
            liquidation_id(cmd.liquidation_batch_id.as_str(), cmd.position_id.as_str()),
            cmd.liquidation_batch_id.clone(),
            cmd.party_id.clone(),
            state.position.account_id.clone(),
            state.position.position_id.clone(),
            state.position.asset,
            state.position.symbol.clone(),
            state.position.side,
            state.position.qty,
            cmd.margin_mode,
            cmd.mark_price,
            cmd.bankruptcy_price,
            cmd.trigger_reason,
            HyperliquidPerpLiquidationStatus::Started,
        );

        Ok(UseCaseOutput {
            output: StartHyperliquidPerpLiquidationOutput { liquidation: liquidation.clone() },
            events: vec![
                liquidation
                    .track_create_event()
                    .map_err(|_| StartHyperliquidPerpLiquidationError::ArithmeticOverflow)?,
            ],
        })
    }
}

fn validate_state(
    cmd: &StartHyperliquidPerpLiquidationCmd,
    state: &StartHyperliquidPerpLiquidationState,
) -> Result<(), StartHyperliquidPerpLiquidationError> {
    if state.position.position_id != cmd.position_id {
        return Err(StartHyperliquidPerpLiquidationError::PositionMismatch);
    }
    if !state.position.has_consistent_state() {
        return Err(StartHyperliquidPerpLiquidationError::InconsistentPositionState);
    }
    if !state.position.is_liquidatable() {
        return Err(StartHyperliquidPerpLiquidationError::PositionNotLiquidatable);
    }
    if !state.margin_balance.belongs_to_account(state.position.account_id.as_str()) {
        return Err(StartHyperliquidPerpLiquidationError::MarginBalanceAccountMismatch);
    }
    if !state.margin_balance.is_asset(state.margin_asset_id.as_str()) {
        return Err(StartHyperliquidPerpLiquidationError::InvalidMarginBalance);
    }
    if state
        .existing_liquidation_position_ids
        .iter()
        .any(|position_id| position_id == &cmd.position_id)
    {
        return Err(StartHyperliquidPerpLiquidationError::PositionAlreadyInLiquidation);
    }
    if !state.position.liquidation_triggered_by_mark_price(cmd.mark_price, cmd.bankruptcy_price) {
        return Err(StartHyperliquidPerpLiquidationError::LiquidationConditionNotMet);
    }

    Ok(())
}

fn liquidation_id(batch_id: &str, position_id: &str) -> String {
    format!("{batch_id}-{position_id}")
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::CommandUseCase3;

    use super::*;
    use crate::entity::HyperliquidPerpPositionSide;

    fn position() -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            2,
            60_000,
            5,
            24_000,
            0,
            3,
        )
    }

    fn balance() -> Balance {
        Balance::new("trader-1".to_string(), "USDC".to_string(), 100_000, 0, 5)
    }

    fn state() -> StartHyperliquidPerpLiquidationState {
        StartHyperliquidPerpLiquidationState {
            position: position(),
            margin_balance: balance(),
            margin_asset_id: "USDC".to_string(),
            existing_liquidation_position_ids: Vec::new(),
        }
    }

    fn cmd() -> StartHyperliquidPerpLiquidationCmd {
        StartHyperliquidPerpLiquidationCmd {
            party_id: "risk-engine".to_string(),
            liquidation_batch_id: "liq-2026-06-10T00".to_string(),
            position_id: "position-1".to_string(),
            mark_price: 49_000,
            bankruptcy_price: 50_000,
            trigger_reason: HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            margin_mode: HyperliquidPerpMarginMode::Cross,
        }
    }

    #[test]
    fn role_returns_risk_engine() {
        assert_eq!(StartHyperliquidPerpLiquidationUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_blank_batch_id() {
        let mut cmd = cmd();
        cmd.liquidation_batch_id.clear();

        assert_eq!(
            StartHyperliquidPerpLiquidationUseCase.pre_check_command(&cmd),
            Err(StartHyperliquidPerpLiquidationError::InvalidLiquidationBatchId)
        );
    }

    #[test]
    fn validate_rejects_when_already_in_liquidation() {
        let mut state = state();
        state.existing_liquidation_position_ids.push("position-1".to_string());

        assert_eq!(
            StartHyperliquidPerpLiquidationUseCase.validate_against_state(&cmd(), &state),
            Err(StartHyperliquidPerpLiquidationError::PositionAlreadyInLiquidation)
        );
    }

    #[test]
    fn validate_rejects_when_mark_does_not_trigger() {
        let mut cmd = cmd();
        cmd.mark_price = 50_001;

        assert_eq!(
            StartHyperliquidPerpLiquidationUseCase.validate_against_state(&cmd, &state()),
            Err(StartHyperliquidPerpLiquidationError::LiquidationConditionNotMet)
        );
    }

    #[test]
    fn compute_creates_single_liquidation_event() {
        let result = StartHyperliquidPerpLiquidationUseCase
            .compute_output_and_events(&cmd(), state())
            .unwrap();
        let events = result.events;

        assert_eq!(events.len(), 1);
        assert_eq!(result.output.liquidation.status, HyperliquidPerpLiquidationStatus::Started);
        assert!(events[0].is_created());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("liquidation_id")
                && change.new_value_bytes() == b"liq-2026-06-10T00-position-1"
        }));
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"started"
        }));
    }
}
