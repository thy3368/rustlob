use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::{validate_common_fields, validate_hex_address};
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum AgentSendAssetContractError {
    #[error("Unexpected `action.type` for agentSendAsset handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.destination`. Expected a 42-character hexadecimal address.")]
    InvalidDestination,
    #[error("Invalid `action.token`. Expected a non-empty token identifier string.")]
    InvalidToken,
    #[error("Invalid `action.amount`. Expected a non-empty decimal string.")]
    InvalidAmount,
    #[error("Invalid `action.sourceDex`. Expected a non-empty dex identifier string.")]
    InvalidSourceDex,
    #[error("Invalid `action.destinationDex`. Expected a non-empty dex identifier string.")]
    InvalidDestinationDex,
    #[error(
        "Invalid `action.fromSubAccount`. Expected an empty string or a 42-character hexadecimal address."
    )]
    InvalidFromSubAccount,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`vaultAddress` is not supported for `agentSendAsset`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `agentSendAsset`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as AgentSendAssetResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    destination: String,
    #[serde(rename = "sourceDex")]
    source_dex: String,
    #[serde(rename = "destinationDex")]
    destination_dex: String,
    token: String,
    amount: String,
    #[serde(rename = "fromSubAccount")]
    from_sub_account: String,
    nonce: u64,
}

pub(crate) struct AgentSendAssetAction;

impl ExchangeActionHandler for AgentSendAssetAction {
    type Request = RequestWire;
    type Reply = reply::AgentSendAssetResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "agentSendAsset" {
        return Err(AgentSendAssetContractError::UnexpectedActionType(
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
        None,
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.common.vault_address.is_some() {
        return Err(AgentSendAssetContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(AgentSendAssetContractError::ExpiresAfterNotSupported.into());
    }
    validate_hex_address(&request.action.destination)
        .map_err(|_| AgentSendAssetContractError::InvalidDestination)?;
    if request.action.source_dex.trim().is_empty() {
        return Err(AgentSendAssetContractError::InvalidSourceDex.into());
    }
    if request.action.destination_dex.trim().is_empty() {
        return Err(AgentSendAssetContractError::InvalidDestinationDex.into());
    }
    if request.action.token.trim().is_empty() {
        return Err(AgentSendAssetContractError::InvalidToken.into());
    }
    if request.action.amount.trim().is_empty() {
        return Err(AgentSendAssetContractError::InvalidAmount.into());
    }
    if !request.action.from_sub_account.is_empty() {
        validate_hex_address(&request.action.from_sub_account)
            .map_err(|_| AgentSendAssetContractError::InvalidFromSubAccount)?;
    }
    if request.action.nonce != request.common.nonce {
        return Err(AgentSendAssetContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute() -> Result<reply::AgentSendAssetResponseWire, ExchangeHttpError> {
    Ok(reply::AgentSendAssetResponseWire {
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
        assert_eq!(request.action.source_dex, "spot");
    }

    #[test]
    fn rejects_invalid_from_sub_account() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "agentSendAsset",
                    "destination": "0x5555555555555555555555555555555555555555",
                    "sourceDex": "spot",
                    "destinationDex": "spot",
                    "token": "USDC:0x0000000000000000000000000000000000000000",
                    "amount": "1",
                    "fromSubAccount": "bad",
                    "nonce": 1710000000000
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
            "Invalid `action.fromSubAccount`. Expected an empty string or a 42-character hexadecimal address."
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
            "action": {
                "type": "agentSendAsset",
                "destination": "0x5555555555555555555555555555555555555555",
                "sourceDex": "spot",
                "destinationDex": "spot",
                "token": "USDC:0x0000000000000000000000000000000000000000",
                "amount": "1",
                "fromSubAccount": "",
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
