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
pub enum Withdraw3ContractError {
    #[error("Unexpected `action.type` for withdraw3 handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.destination`. Expected a 42-character hexadecimal address.")]
    InvalidDestination,
    #[error("Invalid `action.amount`. Expected a non-empty decimal string.")]
    InvalidAmount,
    #[error("Invalid `action.time`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`expiresAfter` is not supported for `withdraw3`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as Withdraw3ResponseWire;
}

pub(crate) type Withdraw3RequestWire = ExchangeRequestEnvelopeWire<Withdraw3ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Withdraw3ActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "hyperliquidChain")]
    hyperliquid_chain: String,
    #[serde(rename = "signatureChainId")]
    signature_chain_id: String,
    amount: String,
    time: u64,
    destination: String,
}

pub(crate) struct Withdraw3Action;

impl ExchangeActionHandler for Withdraw3Action {
    type Request = Withdraw3RequestWire;
    type Reply = reply::Withdraw3ResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &Withdraw3RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "withdraw3" {
        return Err(
            Withdraw3ContractError::UnexpectedActionType(request.action.type_.clone()).into()
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
        return Err(Withdraw3ContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| Withdraw3ContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| Withdraw3ContractError::InvalidSignatureChainId)?;
    validate_hex_address(&request.action.destination)
        .map_err(|_| Withdraw3ContractError::InvalidDestination)?;
    if request.action.amount.trim().is_empty() {
        return Err(Withdraw3ContractError::InvalidAmount.into());
    }
    if request.action.time != request.common.nonce {
        return Err(Withdraw3ContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute() -> Result<reply::Withdraw3ResponseWire, ExchangeHttpError> {
    Ok(reply::Withdraw3ResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<Withdraw3RequestWire>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.amount, "1");
    }

    #[test]
    fn rejects_empty_amount() {
        let request = parse_json_request::<Withdraw3RequestWire>(
            br#"{
                "action": {
                    "type": "withdraw3",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "amount": "",
                    "time": 1710000000000,
                    "destination": "0x6666666666666666666666666666666666666666"
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
            "Invalid `action.amount`. Expected a non-empty decimal string."
        );
    }

    #[test]
    fn allows_vault_address_like_sdk_post_payload() {
        let request = parse_json_request::<Withdraw3RequestWire>(
            br#"{
                "action": {
                    "type": "withdraw3",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "amount": "1",
                    "time": 1710000000000,
                    "destination": "0x6666666666666666666666666666666666666666"
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
                "type": "withdraw3",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "amount": "1",
                "time": 1710000000000,
                "destination": "0x6666666666666666666666666666666666666666"
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
