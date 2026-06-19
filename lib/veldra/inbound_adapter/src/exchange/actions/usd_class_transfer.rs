use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::{
    validate_common_fields, validate_hyperliquid_chain, validate_signature_chain_id,
};
use crate::exchange::common::wire::{CommonExchangeFields, DefaultExchangeResponseEnvelopeWire};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum UsdClassTransferContractError {
    #[error("Unexpected `action.type` for usdClassTransfer handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.amount`. Expected a non-empty amount string.")]
    InvalidAmount,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`vaultAddress` is not supported for `usdClassTransfer`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `usdClassTransfer`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::DefaultExchangeResponseWire as UsdClassTransferResponseWire;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UsdClassTransferRequestWire {
    action: UsdClassTransferActionWire,
    #[serde(flatten)]
    common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UsdClassTransferActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "hyperliquidChain")]
    hyperliquid_chain: String,
    #[serde(rename = "signatureChainId")]
    signature_chain_id: String,
    amount: String,
    #[serde(rename = "toPerp")]
    to_perp: bool,
    nonce: u64,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::UsdClassTransferResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<UsdClassTransferRequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &UsdClassTransferRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "usdClassTransfer" {
        return Err(UsdClassTransferContractError::UnexpectedActionType(
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
        return Err(UsdClassTransferContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(UsdClassTransferContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| UsdClassTransferContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| UsdClassTransferContractError::InvalidSignatureChainId)?;
    if request.action.amount.trim().is_empty() {
        return Err(UsdClassTransferContractError::InvalidAmount.into());
    }
    if request.action.nonce != request.common.nonce {
        return Err(UsdClassTransferContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::UsdClassTransferResponseWire, ExchangeHttpError> {
    Ok(reply::UsdClassTransferResponseWire {
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
        assert!(request.action.to_perp);
    }

    #[test]
    fn rejects_empty_amount() {
        let request = parse(
            br#"{
                "action": {
                    "type": "usdClassTransfer",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "amount": "",
                    "toPerp": true,
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
            "Invalid `action.amount`. Expected a non-empty amount string."
        );
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
                "type": "usdClassTransfer",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "amount": "1",
                "toPerp": true,
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
