use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::ExchangeRequestEnvelopeWire;
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

    use crate::exchange::common::wire::{
        ExchangeResponseEnvelopeWire, ExchangeResponseWire, ExchangeStatusDataWire,
    };

    pub type TwapCancelResponseWire = ExchangeResponseWire<TwapCancelResponseDataWire>;
    pub type TwapCancelResponseEnvelopeWire =
        ExchangeResponseEnvelopeWire<TwapCancelResponseDataWire>;
    pub type TwapCancelResponseDataWire = ExchangeStatusDataWire<TwapCancelStatusWire>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum TwapCancelStatusWire {
        Success(&'static str),
        Error { error: String },
    }
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    a: u32,
    t: u64,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::TwapCancelResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    parse_json_request(body)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "twapCancel" {
        return Err(
            TwapCancelContractError::UnexpectedActionType(request.action.type_.clone()).into()
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
    if request.action.t == 0 {
        return Err(TwapCancelContractError::InvalidTwapId.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::TwapCancelResponseWire, ExchangeHttpError> {
    Ok(reply::TwapCancelResponseWire {
        status: "ok",
        response: reply::TwapCancelResponseEnvelopeWire {
            type_: "twapCancel",
            data: reply::TwapCancelResponseDataWire {
                status: reply::TwapCancelStatusWire::Success("success"),
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_twap_cancel_request() {
        let request = parse(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.type_, "twapCancel");
        assert_eq!(request.action.t, 77738308);
    }

    #[test]
    fn rejects_zero_twap_id() {
        let request = parse(
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
        let response =
            execute(&ExchangeActionDeps::default()).await.expect("response should build");
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
        let response = reply::TwapCancelResponseWire {
            status: "ok",
            response: reply::TwapCancelResponseEnvelopeWire {
                type_: "twapCancel",
                data: reply::TwapCancelResponseDataWire {
                    status: reply::TwapCancelStatusWire::Error {
                        error: "TWAP was never placed, already canceled, or filled.".to_string(),
                    },
                },
            },
        };

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
