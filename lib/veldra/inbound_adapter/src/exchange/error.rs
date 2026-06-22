use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::exchange::action_registry::SUPPORTED_ACTION_TYPES_DISPLAY;
use crate::exchange::actions::agent_enable_dex_abstraction::AgentEnableDexAbstractionContractError;
use crate::exchange::actions::agent_send_asset::AgentSendAssetContractError;
use crate::exchange::actions::agent_set_abstraction::AgentSetAbstractionContractError;
use crate::exchange::actions::approve_agent::ApproveAgentContractError;
use crate::exchange::actions::approve_builder_fee::ApproveBuilderFeeContractError;
use crate::exchange::actions::authorize_aqav2_role::AuthorizeAqav2RoleContractError;
use crate::exchange::actions::batch_modify::BatchModifyContractError;
use crate::exchange::actions::c_deposit::CDepositContractError;
use crate::exchange::actions::c_withdraw::CWithdrawContractError;
use crate::exchange::actions::cancel::CancelContractError;
use crate::exchange::actions::cancel_by_cloid::CancelByCloidContractError;
use crate::exchange::actions::claim_rewards::ClaimRewardsContractError;
use crate::exchange::actions::hip3_liquidator_transfer::Hip3LiquidatorTransferContractError;
use crate::exchange::actions::modify::ModifyContractError;
use crate::exchange::actions::noop::NoopContractError;
use crate::exchange::actions::order::OrderContractError;
use crate::exchange::actions::reserve_request_weight::ReserveRequestWeightContractError;
use crate::exchange::actions::schedule_cancel::ScheduleCancelContractError;
use crate::exchange::actions::send_asset::SendAssetContractError;
use crate::exchange::actions::send_to_evm_with_data::SendToEvmWithDataContractError;
use crate::exchange::actions::spot_send::SpotSendContractError;
use crate::exchange::actions::token_delegate::TokenDelegateContractError;
use crate::exchange::actions::top_up_isolated_only_margin::TopUpIsolatedOnlyMarginContractError;
use crate::exchange::actions::twap_cancel::TwapCancelContractError;
use crate::exchange::actions::twap_order::TwapOrderContractError;
use crate::exchange::actions::update_isolated_margin::UpdateIsolatedMarginContractError;
use crate::exchange::actions::update_leverage::UpdateLeverageContractError;
use crate::exchange::actions::usd_class_transfer::UsdClassTransferContractError;
use crate::exchange::actions::usd_send::UsdSendContractError;
use crate::exchange::actions::user_dex_abstraction::UserDexAbstractionContractError;
use crate::exchange::actions::user_outcome::UserOutcomeContractError;
use crate::exchange::actions::user_set_abstraction::UserSetAbstractionContractError;
use crate::exchange::actions::validator_l1_stream::ValidatorL1StreamContractError;
use crate::exchange::actions::vault_transfer::VaultTransferContractError;
use crate::exchange::actions::withdraw3::Withdraw3ContractError;
use crate::exchange::common::wire::ExchangeErrorResponseWire;

#[derive(Debug, Error)]
pub enum ExchangeHttpError {
    #[error("Malformed JSON body.")]
    MalformedJson,
    #[error("Missing or invalid request fields: {0}")]
    InvalidJsonShape(String),
    #[error("Unsupported action.type `{0}`. Supported actions: {SUPPORTED_ACTION_TYPES_DISPLAY}")]
    UnsupportedActionType(String),
    #[error(transparent)]
    SharedFields(#[from] SharedFieldError),
    #[error(transparent)]
    AgentEnableDexAbstractionContract(#[from] AgentEnableDexAbstractionContractError),
    #[error(transparent)]
    AgentSendAssetContract(#[from] AgentSendAssetContractError),
    #[error(transparent)]
    AgentSetAbstractionContract(#[from] AgentSetAbstractionContractError),
    #[error(transparent)]
    ApproveAgentContract(#[from] ApproveAgentContractError),
    #[error(transparent)]
    ApproveBuilderFeeContract(#[from] ApproveBuilderFeeContractError),
    #[error(transparent)]
    AuthorizeAqav2RoleContract(#[from] AuthorizeAqav2RoleContractError),
    #[error(transparent)]
    BatchModifyContract(#[from] BatchModifyContractError),
    #[error(transparent)]
    CancelContract(#[from] CancelContractError),
    #[error(transparent)]
    CancelByCloidContract(#[from] CancelByCloidContractError),
    #[error(transparent)]
    CDepositContract(#[from] CDepositContractError),
    #[error(transparent)]
    CWithdrawContract(#[from] CWithdrawContractError),
    #[error(transparent)]
    ClaimRewardsContract(#[from] ClaimRewardsContractError),
    #[error(transparent)]
    Hip3LiquidatorTransferContract(#[from] Hip3LiquidatorTransferContractError),
    #[error(transparent)]
    ModifyContract(#[from] ModifyContractError),
    #[error(transparent)]
    NoopContract(#[from] NoopContractError),
    #[error(transparent)]
    OrderContract(#[from] OrderContractError),
    #[error(transparent)]
    ReserveRequestWeightContract(#[from] ReserveRequestWeightContractError),
    #[error(transparent)]
    ScheduleCancelContract(#[from] ScheduleCancelContractError),
    #[error(transparent)]
    SendAssetContract(#[from] SendAssetContractError),
    #[error(transparent)]
    SendToEvmWithDataContract(#[from] SendToEvmWithDataContractError),
    #[error(transparent)]
    SpotSendContract(#[from] SpotSendContractError),
    #[error(transparent)]
    TopUpIsolatedOnlyMarginContract(#[from] TopUpIsolatedOnlyMarginContractError),
    #[error(transparent)]
    TwapCancelContract(#[from] TwapCancelContractError),
    #[error(transparent)]
    TwapOrderContract(#[from] TwapOrderContractError),
    #[error(transparent)]
    TokenDelegateContract(#[from] TokenDelegateContractError),
    #[error(transparent)]
    UpdateIsolatedMarginContract(#[from] UpdateIsolatedMarginContractError),
    #[error(transparent)]
    UpdateLeverageContract(#[from] UpdateLeverageContractError),
    #[error(transparent)]
    UsdClassTransferContract(#[from] UsdClassTransferContractError),
    #[error(transparent)]
    UsdSendContract(#[from] UsdSendContractError),
    #[error(transparent)]
    UserOutcomeContract(#[from] UserOutcomeContractError),
    #[error(transparent)]
    UserDexAbstractionContract(#[from] UserDexAbstractionContractError),
    #[error(transparent)]
    UserSetAbstractionContract(#[from] UserSetAbstractionContractError),
    #[error(transparent)]
    ValidatorL1StreamContract(#[from] ValidatorL1StreamContractError),
    #[error(transparent)]
    VaultTransferContract(#[from] VaultTransferContractError),
    #[error(transparent)]
    Withdraw3Contract(#[from] Withdraw3ContractError),
}

#[derive(Debug, Error)]
pub enum SharedFieldError {
    #[error("Invalid `signature` shape. Expected hex `r`/`s` and numeric `v`.")]
    InvalidSignature,
    #[error("Invalid `vaultAddress`. Expected a 42-character hexadecimal address.")]
    InvalidVaultAddress,
    #[error("Invalid `expiresAfter`. Expected a positive millisecond timestamp.")]
    InvalidExpiresAfter,
    #[error("Invalid `nonce`. Expected a positive millisecond timestamp.")]
    InvalidNonce,
}

impl ExchangeHttpError {
    pub fn from_json_error(error: serde_json::Error) -> Self {
        match error.classify() {
            serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                Self::MalformedJson
            }
            serde_json::error::Category::Data | serde_json::error::Category::Io => {
                Self::InvalidJsonShape(error.to_string())
            }
        }
    }

    fn status_code_value(&self) -> StatusCode {
        match self {
            Self::MalformedJson
            | Self::InvalidJsonShape(_)
            | Self::UnsupportedActionType(_)
            | Self::SharedFields(_)
            | Self::AgentEnableDexAbstractionContract(_)
            | Self::AgentSendAssetContract(_)
            | Self::AgentSetAbstractionContract(_)
            | Self::ApproveAgentContract(_)
            | Self::ApproveBuilderFeeContract(_)
            | Self::AuthorizeAqav2RoleContract(_)
            | Self::BatchModifyContract(_)
            | Self::CancelContract(_)
            | Self::CancelByCloidContract(_)
            | Self::CDepositContract(_)
            | Self::CWithdrawContract(_)
            | Self::ClaimRewardsContract(_)
            | Self::Hip3LiquidatorTransferContract(_)
            | Self::ModifyContract(_)
            | Self::NoopContract(_)
            | Self::OrderContract(_)
            | Self::ReserveRequestWeightContract(_)
            | Self::ScheduleCancelContract(_)
            | Self::SendAssetContract(_)
            | Self::SendToEvmWithDataContract(_)
            | Self::SpotSendContract(_)
            | Self::TopUpIsolatedOnlyMarginContract(_)
            | Self::TwapCancelContract(_)
            | Self::TwapOrderContract(_)
            | Self::TokenDelegateContract(_)
            | Self::UpdateIsolatedMarginContract(_)
            | Self::UpdateLeverageContract(_)
            | Self::UsdClassTransferContract(_)
            | Self::UsdSendContract(_)
            | Self::UserOutcomeContract(_)
            | Self::UserDexAbstractionContract(_)
            | Self::UserSetAbstractionContract(_)
            | Self::ValidatorL1StreamContract(_)
            | Self::VaultTransferContract(_)
            | Self::Withdraw3Contract(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl ResponseError for ExchangeHttpError {
    fn status_code(&self) -> StatusCode {
        self.status_code_value()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(ExchangeErrorResponseWire { status: "err", error: self.to_string() })
    }
}

#[cfg(test)]
mod tests {
    use super::ExchangeHttpError;

    #[test]
    fn unsupported_action_error_message_stays_stable() {
        assert_eq!(
            ExchangeHttpError::UnsupportedActionType("doesNotExist".to_string()).to_string(),
            "Unsupported action.type `doesNotExist`. Supported actions: `agentEnableDexAbstraction`, `agentSendAsset`, `agentSetAbstraction`, `approveAgent`, `approveBuilderFee`, `authorizeAqav2Role`, `batchModify`, `cDeposit`, `cWithdraw`, `cancel`, `cancelByCloid`, `claimRewards`, `hip3LiquidatorTransfer`, `modify`, `noop`, `order`, `reserveRequestWeight`, `scheduleCancel`, `sendAsset`, `sendToEvmWithData`, `spotSend`, `tokenDelegate`, `topUpIsolatedOnlyMargin`, `twapCancel`, `twapOrder`, `updateIsolatedMargin`, `updateLeverage`, `usdClassTransfer`, `usdSend`, `userDexAbstraction`, `userOutcome`, `userSetAbstraction`, `validatorL1Stream`, `vaultTransfer`, `withdraw3`."
        );
    }
}
