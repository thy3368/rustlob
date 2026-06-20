use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::exchange::common::validate::{
    validate_common_fields, validate_hex_address, validate_hyperliquid_chain,
    validate_signature_chain_id,
};
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeEmptyResponseWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum ApproveAgentContractError {
    #[error("Unexpected `action.type` for approveAgent handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.agentAddress`. Expected a 42-character hexadecimal address.")]
    InvalidAgentAddress,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`expiresAfter` is not supported for `approveAgent`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as ApproveAgentResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "hyperliquidChain")]
    hyperliquid_chain: String,
    #[serde(rename = "signatureChainId")]
    signature_chain_id: String,
    #[serde(rename = "agentAddress")]
    agent_address: String,
    #[serde(rename = "agentName")]
    agent_name: Option<String>,
    nonce: u64,
}

struct ApproveAgentAction;

impl ExchangeActionHandler for ApproveAgentAction {
    type Request = RequestWire;
    type Reply = reply::ApproveAgentResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute<'a>(
        _request: Self::Request,
        deps: &'a ExchangeActionDeps,
    ) -> ExchangeActionFuture<'a, Self::Reply> {
        Box::pin(execute(deps))
    }
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::ApproveAgentResponseWire, ExchangeHttpError> {
    run_exchange_action::<ApproveAgentAction>(body, deps).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "approveAgent" {
        return Err(
            ApproveAgentContractError::UnexpectedActionType(request.action.type_.clone()).into()
        );
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
        return Err(ApproveAgentContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| ApproveAgentContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| ApproveAgentContractError::InvalidSignatureChainId)?;
    validate_hex_address(&request.action.agent_address)
        .map_err(|_| ApproveAgentContractError::InvalidAgentAddress)?;
    if request.action.nonce != request.common.nonce {
        return Err(ApproveAgentContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::ApproveAgentResponseWire, ExchangeHttpError> {
    Ok(ExchangeEmptyResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_approve_agent_request() {
        let request = parse_json_request::<RequestWire>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.type_, "approveAgent");
        assert_eq!(request.action.agent_name.as_deref(), Some("desk-bot"));
    }

    #[test]
    fn rejects_nonce_mismatch() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "approveAgent",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "agentAddress": "0x3333333333333333333333333333333333333333",
                    "nonce": 1710000001000
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
            "Invalid `action.nonce`. Expected it to match the outer `nonce`."
        );
    }

    #[actix_web::test]
    async fn approve_agent_reply_snapshot_is_stable() {
        let response =
            execute(&ExchangeActionDeps::default()).await.expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
        );
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "approveAgent",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "agentAddress": "0x3333333333333333333333333333333333333333",
                "agentName": "desk-bot",
                "nonce": 1710000000000
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
