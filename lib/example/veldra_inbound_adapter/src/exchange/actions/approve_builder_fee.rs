use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::{
    validate_common_fields, validate_hex_address, validate_hyperliquid_chain,
    validate_signature_chain_id,
};
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_default_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum ApproveBuilderFeeContractError {
    #[error("Unexpected `action.type` for approveBuilderFee handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.maxFeeRate`. Expected a non-empty percent string.")]
    InvalidMaxFeeRate,
    #[error("Invalid `action.builder`. Expected a 42-character hexadecimal address.")]
    InvalidBuilder,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`expiresAfter` is not supported for `approveBuilderFee`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as ApproveBuilderFeeResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "hyperliquidChain")]
    hyperliquid_chain: String,
    #[serde(rename = "signatureChainId")]
    signature_chain_id: String,
    #[serde(rename = "maxFeeRate")]
    max_fee_rate: String,
    builder: String,
    nonce: u64,
}

pub(crate) struct ApproveBuilderFeeAction;

impl ExchangeActionHandler for ApproveBuilderFeeAction {
    type Request = RequestWire;
    type Reply = reply::ApproveBuilderFeeResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "approveBuilderFee" {
        return Err(ExchangeHttpError::contract(
            ApproveBuilderFeeContractError::UnexpectedActionType(request.action.type_.clone()),
        ));
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
        return Err(ExchangeHttpError::contract(
            ApproveBuilderFeeContractError::ExpiresAfterNotSupported,
        ));
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain).map_err(|_| {
        ExchangeHttpError::contract(ApproveBuilderFeeContractError::InvalidHyperliquidChain)
    })?;
    validate_signature_chain_id(&request.action.signature_chain_id).map_err(|_| {
        ExchangeHttpError::contract(ApproveBuilderFeeContractError::InvalidSignatureChainId)
    })?;
    if request.action.max_fee_rate.trim().is_empty() {
        return Err(ExchangeHttpError::contract(ApproveBuilderFeeContractError::InvalidMaxFeeRate));
    }
    validate_hex_address(&request.action.builder)
        .map_err(|_| ExchangeHttpError::contract(ApproveBuilderFeeContractError::InvalidBuilder))?;
    if request.action.nonce != request.common.nonce {
        return Err(ExchangeHttpError::contract(ApproveBuilderFeeContractError::NonceMismatch));
    }
    Ok(())
}

async fn execute() -> Result<reply::ApproveBuilderFeeResponseWire, ExchangeHttpError> {
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.max_fee_rate, "0.001%");
    }

    #[test]
    fn rejects_nonce_mismatch() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "approveBuilderFee",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "maxFeeRate": "0.001%",
                    "builder": "0x7777777777777777777777777777777777777777",
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

    #[test]
    fn allows_vault_address_like_sdk_post_payload() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "approveBuilderFee",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "maxFeeRate": "0.001%",
                    "builder": "0x7777777777777777777777777777777777777777",
                    "nonce": 1710000000000
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
                "type": "approveBuilderFee",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "maxFeeRate": "0.001%",
                "builder": "0x7777777777777777777777777777777777777777",
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
