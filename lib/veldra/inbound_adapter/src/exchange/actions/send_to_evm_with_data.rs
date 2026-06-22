use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::exchange::common::validate::{
    validate_common_fields, validate_hyperliquid_chain, validate_signature_chain_id,
};
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum SendToEvmWithDataContractError {
    #[error("Unexpected `action.type` for sendToEvmWithData handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.token`. Expected a non-empty token identifier string.")]
    InvalidToken,
    #[error("Invalid `action.amount`. Expected a non-empty decimal string.")]
    InvalidAmount,
    #[error("Invalid `action.sourceDex`. Expected a non-empty dex identifier string.")]
    InvalidSourceDex,
    #[error("Invalid `action.destinationRecipient`. Expected a non-empty encoded recipient.")]
    InvalidDestinationRecipient,
    #[error("Invalid `action.addressEncoding`. Expected `hex` or `base58`.")]
    InvalidAddressEncoding,
    #[error("Invalid `action.data`. Expected a non-empty bytes payload string.")]
    InvalidData,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`vaultAddress` is not supported for `sendToEvmWithData`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `sendToEvmWithData`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as SendToEvmWithDataResponseWire;
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
    token: String,
    amount: String,
    #[serde(rename = "sourceDex")]
    source_dex: String,
    #[serde(rename = "destinationRecipient")]
    destination_recipient: String,
    #[serde(rename = "addressEncoding")]
    address_encoding: String,
    #[serde(rename = "destinationChainId")]
    destination_chain_id: u64,
    #[serde(rename = "gasLimit")]
    gas_limit: u64,
    data: String,
    nonce: u64,
}

struct SendToEvmWithDataAction;

impl ExchangeActionHandler for SendToEvmWithDataAction {
    type Request = RequestWire;
    type Reply = reply::SendToEvmWithDataResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

pub async fn handle(
    body: &[u8],
) -> Result<reply::SendToEvmWithDataResponseWire, ExchangeHttpError> {
    run_exchange_action::<SendToEvmWithDataAction>(body).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "sendToEvmWithData" {
        return Err(SendToEvmWithDataContractError::UnexpectedActionType(
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
        return Err(SendToEvmWithDataContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(SendToEvmWithDataContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| SendToEvmWithDataContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| SendToEvmWithDataContractError::InvalidSignatureChainId)?;
    if request.action.token.trim().is_empty() {
        return Err(SendToEvmWithDataContractError::InvalidToken.into());
    }
    if request.action.amount.trim().is_empty() {
        return Err(SendToEvmWithDataContractError::InvalidAmount.into());
    }
    if request.action.source_dex.trim().is_empty() {
        return Err(SendToEvmWithDataContractError::InvalidSourceDex.into());
    }
    if request.action.destination_recipient.trim().is_empty() {
        return Err(SendToEvmWithDataContractError::InvalidDestinationRecipient.into());
    }
    if !matches!(request.action.address_encoding.as_str(), "hex" | "base58") {
        return Err(SendToEvmWithDataContractError::InvalidAddressEncoding.into());
    }
    if request.action.data.trim().is_empty() {
        return Err(SendToEvmWithDataContractError::InvalidData.into());
    }
    if request.action.nonce != request.common.nonce {
        return Err(SendToEvmWithDataContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute() -> Result<reply::SendToEvmWithDataResponseWire, ExchangeHttpError> {
    Ok(reply::SendToEvmWithDataResponseWire {
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
        assert_eq!(request.action.address_encoding, "hex");
    }

    #[test]
    fn rejects_invalid_address_encoding() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "sendToEvmWithData",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                    "amount": "0.01",
                    "sourceDex": "spot",
                    "destinationRecipient": "0x3333",
                    "addressEncoding": "bech32",
                    "destinationChainId": 42161,
                    "gasLimit": 250000,
                    "data": "0x1234",
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
            "Invalid `action.addressEncoding`. Expected `hex` or `base58`."
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
                "type": "sendToEvmWithData",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "amount": "0.01",
                "sourceDex": "spot",
                "destinationRecipient": "0x3333333333333333333333333333333333333333",
                "addressEncoding": "hex",
                "destinationChainId": 42161,
                "gasLimit": 250000,
                "data": "0x1234",
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
