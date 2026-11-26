use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

const SUPPORTED: &str = "`agentEnableDexAbstraction`, `agentSendAsset`, `agentSetAbstraction`, `approveAgent`, `approveBuilderFee`, `authorizeAqav2Role`, `batchModify`, `cDeposit`, `cWithdraw`, `cancel`, `cancelByCloid`, `claimRewards`, `hip3LiquidatorTransfer`, `modify`, `noop`, `order`, `reserveRequestWeight`, `scheduleCancel`, `sendAsset`, `sendToEvmWithData`, `spotSend`, `tokenDelegate`, `topUpIsolatedOnlyMargin`, `twapCancel`, `twapOrder`, `updateIsolatedMargin`, `updateLeverage`, `usdClassTransfer`, `usdSend`, `userDexAbstraction`, `userOutcome`, `userSetAbstraction`, `validatorL1Stream`, `vaultTransfer`, `withdraw3`";

#[derive(Debug, Error)]
pub enum ExchangeV2Error {
    #[error("Malformed JSON body.")]
    MalformedJson,
    #[error("Missing or invalid request fields: {0}")]
    InvalidJsonShape(String),
    #[error("Unsupported action.type `{0}`. Supported actions: {SUPPORTED}.")]
    UnsupportedActionType(String),
    #[error("Invalid `signature` shape. Expected hex `r`/`s` and numeric `v`.")]
    InvalidSignature,
    #[error("Invalid `vaultAddress`. Expected a 42-character hexadecimal address.")]
    InvalidVaultAddress,
    #[error("Invalid `expiresAfter`. Expected a positive millisecond timestamp.")]
    InvalidExpiresAfter,
    #[error("Invalid `nonce`. Expected a positive millisecond timestamp.")]
    InvalidNonce,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    status: &'static str,
    error: &'a str,
}

impl ExchangeV2Error {
    pub fn json(error: serde_json::Error) -> Self {
        if error.is_syntax() || error.is_eof() {
            Self::MalformedJson
        } else {
            Self::InvalidJsonShape(error.to_string())
        }
    }
}

impl ResponseError for ExchangeV2Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        HttpResponse::BadRequest().json(ErrorBody {
            status: "err",
            error: &message,
        })
    }
}
