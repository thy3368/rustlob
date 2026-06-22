use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_status_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum TwapCancelContractError {
    #[error("Unexpected `action.type` for twapCancel handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.t`. Expected a positive twap id.")]
    InvalidTwapId,
}

pub mod reply {
    use serde::Serialize;

    use crate::exchange::common::wire::{ExchangeResponseWire, ExchangeStatusDataWire};

    pub type TwapCancelResponseWire = ExchangeResponseWire<TwapCancelResponseDataWire>;
    pub type TwapCancelResponseDataWire = ExchangeStatusDataWire<TwapCancelStatusWire>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum TwapCancelStatusWire {
        Success(&'static str),
        Error { error: String },
    }
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    a: u32,
    t: u64,
}

pub(crate) struct TwapCancelAction;

impl ExchangeActionHandler for TwapCancelAction {
    type Request = RequestWire;
    type Reply = reply::TwapCancelResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "twapCancel" {
        return Err(ExchangeHttpError::contract(TwapCancelContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.t == 0 {
        return Err(ExchangeHttpError::contract(TwapCancelContractError::InvalidTwapId));
    }
    Ok(())
}

async fn execute() -> Result<reply::TwapCancelResponseWire, ExchangeHttpError> {
    Ok(ok_status_response("twapCancel", reply::TwapCancelStatusWire::Success("success")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_twap_cancel_request() {
        let request =
            parse_json_request::<RequestWire>(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.type_, "twapCancel");
        assert_eq!(request.action.t, 77738308);
    }

    #[test]
    fn rejects_zero_twap_id() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": { "type": "twapCancel", "a": 7, "t": 0 },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");

        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(error.to_string(), "Invalid `action.t`. Expected a positive twap id.");
    }

    #[actix_web::test]
    async fn twap_cancel_reply_snapshot_is_stable() {
        let response = execute().await.expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapCancel",
    "data": {
      "status": "success"
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn twap_cancel_error_snapshot_is_stable() {
        let response = ok_status_response(
            "twapCancel",
            reply::TwapCancelStatusWire::Error {
                error: "TWAP was never placed, already canceled, or filled.".to_string(),
            },
        );

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapCancel",
    "data": {
      "status": {
        "error": "TWAP was never placed, already canceled, or filled."
      }
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": { "type": "twapCancel", "a": 7, "t": 77738308 },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
