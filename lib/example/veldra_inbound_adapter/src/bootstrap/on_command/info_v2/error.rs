use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InfoV2Error {
    #[error("Malformed JSON body.")]
    MalformedJson,
    #[error("Missing or invalid request fields: {0}")]
    InvalidJsonShape(String),
    #[error("Unsupported info type `{0}`.")]
    UnsupportedType(String),
    #[error("Invalid `{0}`. Expected a 42-character hexadecimal address.")]
    InvalidAddress(&'static str),
    #[error("Invalid `{0}`. Expected a non-empty string.")]
    EmptyString(&'static str),
    #[error("Invalid `{0}`. Expected a positive integer.")]
    NonPositive(&'static str),
    #[error("Invalid time range. Expected `startTime <= endTime`.")]
    InvalidTimeRange,
    #[error("Invalid `oid`. Expected a positive integer or a 16-byte hexadecimal cloid.")]
    InvalidOid,
    #[error("Invalid `nSigFigs`. Expected one of 2, 3, 4, or 5.")]
    InvalidSigFigs,
    #[error("Invalid `mantissa`. It is only valid with `nSigFigs = 5` and must be 1, 2, or 5.")]
    InvalidMantissa,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    status: &'static str,
    error: &'a str,
}

impl InfoV2Error {
    pub(crate) fn json(error: serde_json::Error) -> Self {
        if error.is_syntax() || error.is_eof() {
            Self::MalformedJson
        } else {
            Self::InvalidJsonShape(error.to_string())
        }
    }
}

impl ResponseError for InfoV2Error {
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
