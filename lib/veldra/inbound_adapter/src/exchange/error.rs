use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::exchange::actions::approve_agent::error::ApproveAgentContractError;
use crate::exchange::actions::cancel::error::CancelContractError;
use crate::exchange::actions::noop::error::NoopContractError;
use crate::exchange::actions::order::error::OrderContractError;
use crate::exchange::actions::twap_cancel::error::TwapCancelContractError;
use crate::exchange::actions::twap_order::error::TwapOrderContractError;
use crate::exchange::actions::update_isolated_margin::error::UpdateIsolatedMarginContractError;
use crate::exchange::actions::update_leverage::error::UpdateLeverageContractError;
use crate::exchange::actions::user_set_abstraction::error::UserSetAbstractionContractError;
use crate::exchange::common::wire::ExchangeErrorResponseWire;

#[derive(Debug, Error)]
pub enum ExchangeHttpError {
    #[error("Malformed JSON body.")]
    MalformedJson,
    #[error("Missing or invalid request fields: {0}")]
    InvalidJsonShape(String),
    #[error(
        "Unsupported action.type `{0}`. Supported actions: `approveAgent`, `cancel`, `noop`, `order`, `twapCancel`, `twapOrder`, `updateIsolatedMargin`, `updateLeverage`, `userSetAbstraction`."
    )]
    UnsupportedActionType(String),
    #[error(transparent)]
    SharedFields(#[from] SharedFieldError),
    #[error(transparent)]
    ApproveAgentContract(#[from] ApproveAgentContractError),
    #[error(transparent)]
    CancelContract(#[from] CancelContractError),
    #[error(transparent)]
    NoopContract(#[from] NoopContractError),
    #[error(transparent)]
    OrderContract(#[from] OrderContractError),
    #[error(transparent)]
    TwapCancelContract(#[from] TwapCancelContractError),
    #[error(transparent)]
    TwapOrderContract(#[from] TwapOrderContractError),
    #[error(transparent)]
    UpdateIsolatedMarginContract(#[from] UpdateIsolatedMarginContractError),
    #[error(transparent)]
    UpdateLeverageContract(#[from] UpdateLeverageContractError),
    #[error(transparent)]
    UserSetAbstractionContract(#[from] UserSetAbstractionContractError),
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
            | Self::ApproveAgentContract(_)
            | Self::CancelContract(_)
            | Self::NoopContract(_)
            | Self::OrderContract(_)
            | Self::TwapCancelContract(_)
            | Self::TwapOrderContract(_)
            | Self::UpdateIsolatedMarginContract(_)
            | Self::UpdateLeverageContract(_)
            | Self::UserSetAbstractionContract(_) => StatusCode::BAD_REQUEST,
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
