use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::ExchangeRequestEnvelopeWire;
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

    use crate::exchange::common::wire::{
        ExchangeResponseEnvelopeWire, ExchangeResponseWire, ExchangeStatusDataWire,
    };

    pub type TwapOrderResponseWire = ExchangeResponseWire<TwapOrderResponseDataWire>;
    pub type TwapOrderResponseEnvelopeWire =
        ExchangeResponseEnvelopeWire<TwapOrderResponseDataWire>;
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

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
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

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::TwapOrderResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    parse_json_request(body)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "twapOrder" {
        return Err(
            TwapOrderContractError::UnexpectedActionType(request.action.type_.clone()).into()
        );
    }
    validate_common_fields(
        request.common.nonce,
        request.common.expires_after,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        request.common.vault_address.as_deref(),
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.action.twap.s.trim().is_empty() {
        return Err(TwapOrderContractError::InvalidSize.into());
    }
    if request.action.twap.m == 0 {
        return Err(TwapOrderContractError::InvalidMinutes.into());
    }
    Ok(())
}

const STUB_TWAP_ID: u64 = 77738308;

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::TwapOrderResponseWire, ExchangeHttpError> {
    Ok(reply::TwapOrderResponseWire {
        status: "ok",
        response: reply::TwapOrderResponseEnvelopeWire {
            type_: "twapOrder",
            data: reply::TwapOrderResponseDataWire {
                status: reply::TwapOrderStatusWire::Running {
                    running: reply::TwapRunningStatusWire { twap_id: STUB_TWAP_ID },
                },
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_twap_order_request() {
        let request = parse(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.type_, "twapOrder");
        assert_eq!(request.action.twap.m, 15);
    }

    #[test]
    fn rejects_zero_duration_minutes() {
        let request = parse(
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
        let response =
            execute(&ExchangeActionDeps::default()).await.expect("response should build");
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
        let response = reply::TwapOrderResponseWire {
            status: "ok",
            response: reply::TwapOrderResponseEnvelopeWire {
                type_: "twapOrder",
                data: reply::TwapOrderResponseDataWire {
                    status: reply::TwapOrderStatusWire::Error {
                        error: "Invalid TWAP duration: 1 min(s)".to_string(),
                    },
                },
            },
        };

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
