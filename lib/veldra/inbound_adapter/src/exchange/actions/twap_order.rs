use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_status_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum TwapOrderContractError {
    #[error("Unexpected `action.type` for twapOrder handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.twap.s`. Expected a non-empty decimal string.")]
    InvalidSize,
    #[error("Invalid `action.twap.m`. Expected a duration greater than zero minutes.")]
    InvalidMinutes,
}

pub mod reply {
    use serde::Serialize;

    use crate::exchange::common::wire::{ExchangeResponseWire, ExchangeStatusDataWire};

    pub type TwapOrderResponseWire = ExchangeResponseWire<TwapOrderResponseDataWire>;
    pub type TwapOrderResponseDataWire = ExchangeStatusDataWire<TwapOrderStatusWire>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum TwapOrderStatusWire {
        Running { running: TwapRunningStatusWire },
        Error { error: String },
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct TwapRunningStatusWire {
        #[serde(rename = "twapId")]
        pub twap_id: u64,
    }
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    twap: TwapSpecWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TwapSpecWire {
    a: u32,
    b: bool,
    s: String,
    r: bool,
    m: u64,
    t: bool,
}

pub(crate) struct TwapOrderAction;

impl ExchangeActionHandler for TwapOrderAction {
    type Request = RequestWire;
    type Reply = reply::TwapOrderResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "twapOrder" {
        return Err(ExchangeHttpError::contract(TwapOrderContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.twap.s.trim().is_empty() {
        return Err(ExchangeHttpError::contract(TwapOrderContractError::InvalidSize));
    }
    if request.action.twap.m == 0 {
        return Err(ExchangeHttpError::contract(TwapOrderContractError::InvalidMinutes));
    }
    Ok(())
}

const STUB_TWAP_ID: u64 = 77738308;

async fn execute() -> Result<reply::TwapOrderResponseWire, ExchangeHttpError> {
    Ok(ok_status_response(
        "twapOrder",
        reply::TwapOrderStatusWire::Running {
            running: reply::TwapRunningStatusWire { twap_id: STUB_TWAP_ID },
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_twap_order_request() {
        let request =
            parse_json_request::<RequestWire>(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.type_, "twapOrder");
        assert_eq!(request.action.twap.m, 15);
    }

    #[test]
    fn rejects_zero_duration_minutes() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "twapOrder",
                    "twap": { "a": 7, "b": true, "s": "1.25", "r": false, "m": 0, "t": true }
                },
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
        assert_eq!(
            error.to_string(),
            "Invalid `action.twap.m`. Expected a duration greater than zero minutes."
        );
    }

    #[actix_web::test]
    async fn twap_order_reply_snapshot_is_stable() {
        let response = execute().await.expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapOrder",
    "data": {
      "status": {
        "running": {
          "twapId": 77738308
        }
      }
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn twap_order_error_snapshot_is_stable() {
        let response = ok_status_response(
            "twapOrder",
            reply::TwapOrderStatusWire::Error {
                error: "Invalid TWAP duration: 1 min(s)".to_string(),
            },
        );

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapOrder",
    "data": {
      "status": {
        "error": "Invalid TWAP duration: 1 min(s)"
      }
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "twapOrder",
                "twap": { "a": 7, "b": true, "s": "1.25", "r": false, "m": 15, "t": true }
            },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
