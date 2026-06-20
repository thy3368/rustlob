use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::{
    validate_common_fields, validate_hex_address, validate_hyperliquid_chain,
    validate_signature_chain_id,
};
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum SendAssetContractError {
    #[error("Unexpected `action.type` for sendAsset handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.destination`. Expected a 42-character hexadecimal address.")]
    InvalidDestination,
    #[error(
        "Invalid `action.sourceDex`. Expected a dex identifier string; use empty string for the default perp dex."
    )]
    InvalidSourceDex,
    #[error(
        "Invalid `action.destinationDex`. Expected a dex identifier string; use empty string for the default perp dex."
    )]
    InvalidDestinationDex,
    #[error("Invalid `action.token`. Expected a non-empty token identifier string.")]
    InvalidToken,
    #[error("Invalid `action.amount`. Expected a non-empty decimal string.")]
    InvalidAmount,
    #[error(
        "Invalid `action.fromSubAccount`. Expected an empty string or a 42-character hexadecimal address."
    )]
    InvalidFromSubAccount,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`vaultAddress` is not supported for `sendAsset`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `sendAsset`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as SendAssetResponseWire;
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

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::SendAssetResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    parse_json_request(body)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "sendAsset" {
        return Err(
            SendAssetContractError::UnexpectedActionType(request.action.type_.clone()).into()
        );
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
        return Err(SendAssetContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(SendAssetContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| SendAssetContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| SendAssetContractError::InvalidSignatureChainId)?;
    validate_hex_address(&request.action.destination)
        .map_err(|_| SendAssetContractError::InvalidDestination)?;
    if request.action.source_dex.trim() != request.action.source_dex {
        return Err(SendAssetContractError::InvalidSourceDex.into());
    }
    if request.action.destination_dex.trim() != request.action.destination_dex {
        return Err(SendAssetContractError::InvalidDestinationDex.into());
    }
    if request.action.token.trim().is_empty() {
        return Err(SendAssetContractError::InvalidToken.into());
    }
    if request.action.amount.trim().is_empty() {
        return Err(SendAssetContractError::InvalidAmount.into());
    }
    if !request.action.from_sub_account.is_empty() {
        validate_hex_address(&request.action.from_sub_account)
            .map_err(|_| SendAssetContractError::InvalidFromSubAccount)?;
    }
    if request.action.nonce != request.common.nonce {
        return Err(SendAssetContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::SendAssetResponseWire, ExchangeHttpError> {
    Ok(reply::SendAssetResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.destination_dex, "spot");
    }

    #[test]
    fn rejects_nonce_mismatch() {
        let request = parse(
            br#"{
                "action": {
                    "type": "sendAsset",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "destination": "0x5555555555555555555555555555555555555555",
                    "sourceDex": "spot",
                    "destinationDex": "spot",
                    "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                    "amount": "0.01",
                    "fromSubAccount": "",
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
    fn allows_empty_dex_for_default_perp_pool() {
        let request = parse(
            br#"{
                "action": {
                    "type": "sendAsset",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "destination": "0x5555555555555555555555555555555555555555",
                    "sourceDex": "",
                    "destinationDex": "",
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
            }"#,
        )
        .expect("request parses");
        validate(&request).expect("empty dex should be accepted");
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
                "type": "sendAsset",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "destination": "0x5555555555555555555555555555555555555555",
                "sourceDex": "spot",
                "destinationDex": "spot",
                "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "amount": "0.01",
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
