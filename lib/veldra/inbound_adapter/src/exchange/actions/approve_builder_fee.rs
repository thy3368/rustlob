use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::{
    validate_common_fields, validate_hex_address, validate_hyperliquid_chain,
    validate_signature_chain_id,
};
use crate::exchange::common::wire::{CommonExchangeFields, DefaultExchangeResponseEnvelopeWire};
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
    pub use crate::exchange::common::wire::DefaultExchangeResponseWire as ApproveBuilderFeeResponseWire;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestWire {
    action: ActionWire,
    #[serde(flatten)]
    common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
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

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::ApproveBuilderFeeResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "approveBuilderFee" {
        return Err(ApproveBuilderFeeContractError::UnexpectedActionType(
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
        return Err(ApproveBuilderFeeContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| ApproveBuilderFeeContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| ApproveBuilderFeeContractError::InvalidSignatureChainId)?;
    if request.action.max_fee_rate.trim().is_empty() {
        return Err(ApproveBuilderFeeContractError::InvalidMaxFeeRate.into());
    }
    validate_hex_address(&request.action.builder)
        .map_err(|_| ApproveBuilderFeeContractError::InvalidBuilder)?;
    if request.action.nonce != request.common.nonce {
        return Err(ApproveBuilderFeeContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::ApproveBuilderFeeResponseWire, ExchangeHttpError> {
    Ok(reply::ApproveBuilderFeeResponseWire {
        status: "ok",
        response: DefaultExchangeResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.max_fee_rate, "0.001%");
    }

    #[test]
    fn rejects_nonce_mismatch() {
        let request = parse(
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
        let request = parse(
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
