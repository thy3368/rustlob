use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::ExchangeRequestEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum CancelContractError {
    #[error("Unexpected `action.type` for cancel handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`action.cancels` must contain at least one cancel request.")]
    EmptyCancels,
    #[error("Invalid `action.cancels[].o`. Expected a positive order id.")]
    InvalidOid,
    #[error("Invalid `action.f`. Omit `f` unless fast cancel is enabled.")]
    InvalidFastFlag,
}

pub mod reply {
    use serde::Serialize;

    use crate::exchange::common::wire::{
        ExchangeResponseEnvelopeWire, ExchangeResponseWire, ExchangeStatusesDataWire,
    };

    pub type CancelResponseWire = ExchangeResponseWire<CancelResponseDataWire>;
    pub type CancelResponseEnvelopeWire = ExchangeResponseEnvelopeWire<CancelResponseDataWire>;
    pub type CancelResponseDataWire = ExchangeStatusesDataWire<CancelStatusWire>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum CancelStatusWire {
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
    cancels: Vec<CancelItemWire>,
    f: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CancelItemWire {
    a: u32,
    o: u64,
}

pub(crate) struct CancelAction;

impl ExchangeActionHandler for CancelAction {
    type Request = RequestWire;
    type Reply = reply::CancelResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "cancel" {
        return Err(CancelContractError::UnexpectedActionType(request.action.type_.clone()).into());
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
    if request.action.cancels.is_empty() {
        return Err(CancelContractError::EmptyCancels.into());
    }
    if matches!(request.action.f, Some(false)) {
        return Err(CancelContractError::InvalidFastFlag.into());
    }
    if request.action.cancels.iter().any(|cancel| cancel.o == 0) {
        return Err(CancelContractError::InvalidOid.into());
    }
    Ok(())
}

async fn execute(request: RequestWire) -> Result<reply::CancelResponseWire, ExchangeHttpError> {
    let statuses = request
        .action
        .cancels
        .iter()
        .map(|_| reply::CancelStatusWire::Success("success"))
        .collect();
    Ok(reply::CancelResponseWire {
        status: "ok",
        response: reply::CancelResponseEnvelopeWire {
            type_: "cancel",
            data: reply::CancelResponseDataWire { statuses },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cancel_request() {
        let request = parse_json_request::<RequestWire>(valid_cancel_request_json())
            .expect("cancel request should parse");

        assert_eq!(request.action.type_, "cancel");
        assert_eq!(request.action.cancels.len(), 1);
        assert_eq!(request.action.cancels[0].o, 77738308);
    }

    #[test]
    fn rejects_false_fast_flag() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "cancel",
                    "cancels": [{ "a": 10000, "o": 77738308 }],
                    "f": false
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("cancel request parses");

        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(
            error.to_string(),
            "Invalid `action.f`. Omit `f` unless fast cancel is enabled."
        );
    }

    #[actix_web::test]
    async fn cancel_reply_snapshot_is_stable() {
        let request = parse_json_request::<RequestWire>(valid_cancel_request_json())
            .expect("cancel request parses");
        let response = execute(request).await.expect("cancel response builds");

        let actual = serde_json::to_string_pretty(&response).expect("cancel response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "cancel",
    "data": {
      "statuses": [
        "success"
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn cancel_error_shape_snapshot_is_stable() {
        let response = reply::CancelResponseWire {
            status: "ok",
            response: reply::CancelResponseEnvelopeWire {
                type_: "cancel",
                data: reply::CancelResponseDataWire {
                    statuses: vec![reply::CancelStatusWire::Error {
                        error: "Order was never placed, already canceled, or filled.".to_string(),
                    }],
                },
            },
        };

        let actual = serde_json::to_string_pretty(&response).expect("cancel response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "cancel",
    "data": {
      "statuses": [
        {
          "error": "Order was never placed, already canceled, or filled."
        }
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    fn valid_cancel_request_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "cancel",
                "cancels": [{ "a": 10000, "o": 77738308 }]
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
