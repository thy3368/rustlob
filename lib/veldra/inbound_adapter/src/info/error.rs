use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::info::common::wire::InfoErrorResponseWire;

#[derive(Debug, Error)]
pub enum InfoHttpError {
    #[error("Malformed JSON body.")]
    MalformedJson,
    #[error("Missing or invalid request fields: {0}")]
    InvalidJsonShape(String),
    #[error(
        "Unsupported type `{0}`. Supported queries: `allMids`, `openOrders`, `frontendOpenOrders`, `userFills`, `userFillsByTime`, `userRateLimit`, `orderStatus`, `l2Book`, `candleSnapshot`, `maxBuilderFee`, `historicalOrders`, `userTwapSliceFills`, `subAccounts`, `vaultDetails`, `userVaultEquities`, `userRole`, `portfolio`, `referral`, `userFees`, `delegations`, `delegatorSummary`, `delegatorHistory`, `delegatorRewards`, `userDexAbstraction`, `userAbstraction`, `alignedQuoteTokenInfo`, `borrowLendUserState`, `borrowLendReserveState`, `allBorrowLendReserveStates`, `approvedBuilders`."
    )]
    UnsupportedQueryType(String),
    #[error("{0}")]
    Validation(String),
}

impl InfoHttpError {
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

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }
}

impl ResponseError for InfoHttpError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(InfoErrorResponseWire { status: "err", error: self.to_string() })
    }
}
