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
pub enum TokenDelegateContractError {
    #[error("Unexpected `action.type` for tokenDelegate handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.validator`. Expected a 42-character hexadecimal address.")]
    InvalidValidator,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`expiresAfter` is not supported for `tokenDelegate`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as TokenDelegateResponseWire;
}

pub(crate) type TokenDelegateRequestWire = ExchangeRequestEnvelopeWire<TokenDelegateActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TokenDelegateActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "hyperliquidChain")]
    hyperliquid_chain: String,
    #[serde(rename = "signatureChainId")]
    signature_chain_id: String,
    validator: String,
    #[serde(rename = "isUndelegate")]
    is_undelegate: bool,
    wei: u64,
    nonce: u64,
}

pub(crate) struct TokenDelegateAction;

impl ExchangeActionHandler for TokenDelegateAction {
    type Request = TokenDelegateRequestWire;
    type Reply = reply::TokenDelegateResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &TokenDelegateRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "tokenDelegate" {
        return Err(ExchangeHttpError::contract(TokenDelegateContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
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
            TokenDelegateContractError::ExpiresAfterNotSupported,
        ));
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain).map_err(|_| {
        ExchangeHttpError::contract(TokenDelegateContractError::InvalidHyperliquidChain)
    })?;
    validate_signature_chain_id(&request.action.signature_chain_id).map_err(|_| {
        ExchangeHttpError::contract(TokenDelegateContractError::InvalidSignatureChainId)
    })?;
    validate_hex_address(&request.action.validator)
        .map_err(|_| ExchangeHttpError::contract(TokenDelegateContractError::InvalidValidator))?;
    if request.action.nonce != request.common.nonce {
        return Err(ExchangeHttpError::contract(TokenDelegateContractError::NonceMismatch));
    }
    Ok(())
}

async fn execute() -> Result<reply::TokenDelegateResponseWire, ExchangeHttpError> {
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request =
            parse_json_request::<TokenDelegateRequestWire, ExchangeHttpError>(valid_request_json())
                .expect("request should parse");
        assert!(!request.action.is_undelegate);
    }

    #[test]
    fn rejects_nonce_mismatch() {
        let request = parse_json_request::<TokenDelegateRequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "tokenDelegate",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "validator": "0x7777777777777777777777777777777777777777",
                    "isUndelegate": false,
                    "wei": 1000,
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
        let request = parse_json_request::<TokenDelegateRequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "tokenDelegate",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "validator": "0x7777777777777777777777777777777777777777",
                    "isUndelegate": false,
                    "wei": 1000,
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
                "type": "tokenDelegate",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "validator": "0x7777777777777777777777777777777777777777",
                "isUndelegate": false,
                "wei": 1000,
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
