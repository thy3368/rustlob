use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::{
    validate_common_fields, validate_hex_address, validate_hyperliquid_chain,
    validate_signature_chain_id,
};
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum SpotSendContractError {
    #[error("Unexpected `action.type` for spotSend handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.destination`. Expected a 42-character hexadecimal address.")]
    InvalidDestination,
    #[error("Invalid `action.token`. Expected a non-empty token identifier string.")]
    InvalidToken,
    #[error("Invalid `action.amount`. Expected a non-empty decimal string.")]
    InvalidAmount,
    #[error("Invalid `action.time`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`expiresAfter` is not supported for `spotSend`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as SpotSendResponseWire;
}

pub(crate) type SpotSendRequestWire = ExchangeRequestEnvelopeWire<SpotSendActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SpotSendActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "hyperliquidChain")]
    hyperliquid_chain: String,
    #[serde(rename = "signatureChainId")]
    signature_chain_id: String,
    destination: String,
    token: String,
    amount: String,
    time: u64,
}

pub(crate) struct SpotSendAction;

impl ExchangeActionHandler for SpotSendAction {
    type Request = SpotSendRequestWire;
    type Reply = reply::SpotSendResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &SpotSendRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "spotSend" {
        return Err(
            SpotSendContractError::UnexpectedActionType(request.action.type_.clone()).into()
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
        return Err(SpotSendContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| SpotSendContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| SpotSendContractError::InvalidSignatureChainId)?;
    validate_hex_address(&request.action.destination)
        .map_err(|_| SpotSendContractError::InvalidDestination)?;
    if request.action.token.trim().is_empty() {
        return Err(SpotSendContractError::InvalidToken.into());
    }
    if request.action.amount.trim().is_empty() {
        return Err(SpotSendContractError::InvalidAmount.into());
    }
    if request.action.time != request.common.nonce {
        return Err(SpotSendContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute() -> Result<reply::SpotSendResponseWire, ExchangeHttpError> {
    Ok(reply::SpotSendResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<SpotSendRequestWire>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.token, "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2");
    }

    #[test]
    fn rejects_empty_token() {
        let request = parse_json_request::<SpotSendRequestWire>(
            br#"{
                "action": {
                    "type": "spotSend",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "destination": "0x5555555555555555555555555555555555555555",
                    "token": "",
                    "amount": "0.01",
                    "time": 1710000000000
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
            "Invalid `action.token`. Expected a non-empty token identifier string."
        );
    }

    #[test]
    fn allows_vault_address_like_sdk_post_payload() {
        let request = parse_json_request::<SpotSendRequestWire>(
            br#"{
                "action": {
                    "type": "spotSend",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "destination": "0x5555555555555555555555555555555555555555",
                    "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                    "amount": "0.01",
                    "time": 1710000000000
                },
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
            "action": {
                "type": "spotSend",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "destination": "0x5555555555555555555555555555555555555555",
                "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "amount": "0.01",
                "time": 1710000000000
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
