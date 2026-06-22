use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum AgentSetAbstractionContractError {
    #[error("Unexpected `action.type` for agentSetAbstraction handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.abstraction`. Expected one of `i`, `u`, `p`.")]
    InvalidAbstraction,
    #[error("`expiresAfter` is not supported for `agentSetAbstraction`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as AgentSetAbstractionResponseWire;
}

pub(crate) type AgentSetAbstractionRequestWire =
    ExchangeRequestEnvelopeWire<AgentSetAbstractionActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AgentSetAbstractionActionWire {
    #[serde(rename = "type")]
    type_: String,
    abstraction: String,
}

pub(crate) struct AgentSetAbstractionAction;

impl ExchangeActionHandler for AgentSetAbstractionAction {
    type Request = AgentSetAbstractionRequestWire;
    type Reply = reply::AgentSetAbstractionResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &AgentSetAbstractionRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "agentSetAbstraction" {
        return Err(AgentSetAbstractionContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )
        .into());
    }
    validate_common_fields(
        request.common.nonce,
        None,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        request.common.vault_address.as_deref(),
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.common.expires_after.is_some() {
        return Err(AgentSetAbstractionContractError::ExpiresAfterNotSupported.into());
    }
    if !matches!(request.action.abstraction.as_str(), "i" | "u" | "p") {
        return Err(AgentSetAbstractionContractError::InvalidAbstraction.into());
    }
    Ok(())
}

async fn execute() -> Result<reply::AgentSetAbstractionResponseWire, ExchangeHttpError> {
    Ok(reply::AgentSetAbstractionResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<AgentSetAbstractionRequestWire>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.abstraction, "u");
    }

    #[test]
    fn rejects_invalid_abstraction() {
        let request = parse_json_request::<AgentSetAbstractionRequestWire>(
            br#"{
                "action": { "type": "agentSetAbstraction", "abstraction": "x" },
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
            "Invalid `action.abstraction`. Expected one of `i`, `u`, `p`."
        );
    }

    #[actix_web::test]
    async fn reply_snapshot_is_stable() {
        let response = execute().await.expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
        );
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": { "type": "agentSetAbstraction", "abstraction": "u" },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
