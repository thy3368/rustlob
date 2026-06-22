use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::exchange::action_registry::SUPPORTED_ACTION_TYPES_DISPLAY;
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
    #[error("{0}")]
    ActionContract(String),
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

    pub fn contract(error: impl std::fmt::Display) -> Self {
        Self::ActionContract(error.to_string())
    }

    fn status_code_value(&self) -> StatusCode {
        match self {
            Self::MalformedJson
            | Self::InvalidJsonShape(_)
            | Self::UnsupportedActionType(_)
            | Self::SharedFields(_)
            | Self::ActionContract(_) => StatusCode::BAD_REQUEST,
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
