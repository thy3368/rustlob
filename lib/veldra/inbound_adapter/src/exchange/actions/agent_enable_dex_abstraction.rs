use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum AgentEnableDexAbstractionContractError {
    #[error("Unexpected `action.type` for agentEnableDexAbstraction handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`expiresAfter` is not supported for `agentEnableDexAbstraction`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as AgentEnableDexAbstractionResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
}

struct AgentEnableDexAbstractionAction;

impl ExchangeActionHandler for AgentEnableDexAbstractionAction {
    type Request = RequestWire;
    type Reply = reply::AgentEnableDexAbstractionResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

pub async fn handle(
    body: &[u8],
) -> Result<reply::AgentEnableDexAbstractionResponseWire, ExchangeHttpError> {
    run_exchange_action::<AgentEnableDexAbstractionAction>(body).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "agentEnableDexAbstraction" {
        return Err(AgentEnableDexAbstractionContractError::UnexpectedActionType(
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
        return Err(AgentEnableDexAbstractionContractError::ExpiresAfterNotSupported.into());
    }
    Ok(())
}

async fn execute() -> Result<reply::AgentEnableDexAbstractionResponseWire, ExchangeHttpError> {
    Ok(reply::AgentEnableDexAbstractionResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request =
            parse_json_request::<RequestWire>(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.type_, "agentEnableDexAbstraction");
    }

    #[test]
    fn allows_vault_address_like_sdk_post_payload() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": { "type": "agentEnableDexAbstraction" },
                "nonce": 1710000000000,
                "vaultAddress": "0x1111111111111111111111111111111111111111",
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");
        validate(&request).expect("vault address should be accepted");
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
            "action": { "type": "agentEnableDexAbstraction" },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
