use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_default_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum NoopContractError {
    #[error("Unexpected `action.type` for noop handler: `{0}`.")]
    UnexpectedActionType(String),
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as NoopResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
}

pub(crate) struct NoopAction;

impl ExchangeActionHandler for NoopAction {
    type Request = RequestWire;
    type Reply = reply::NoopResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "noop" {
        return Err(ExchangeHttpError::contract(NoopContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    Ok(())
}

async fn execute() -> Result<reply::NoopResponseWire, ExchangeHttpError> {
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_noop_request() {
        let request = parse_json_request::<RequestWire>(valid_noop_request_json())
            .expect("noop request should parse");
        assert_eq!(request.action.type_, "noop");
    }

    #[actix_web::test]
    async fn noop_reply_snapshot_is_stable() {
        let response = execute().await.expect("noop reply builds");
        let actual = serde_json::to_string_pretty(&response).expect("noop response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
        );
    }

    #[test]
    fn validates_noop_request() {
        let request = parse_json_request::<RequestWire>(valid_noop_request_json())
            .expect("noop request should parse");
        validate(&request).expect("noop validation should pass");
    }

    fn valid_noop_request_json() -> &'static [u8] {
        br#"{
            "action": { "type": "noop" },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
