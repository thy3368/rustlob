use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
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
pub enum CDepositContractError {
    #[error("Unexpected `action.type` for cDeposit handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`vaultAddress` is not supported for `cDeposit`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `cDeposit`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as CDepositResponseWire;
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
    wei: u64,
    nonce: u64,
}

struct CDepositAction;

impl ExchangeActionHandler for CDepositAction {
    type Request = RequestWire;
    type Reply = reply::CDepositResponseWire;

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
) -> Result<reply::CDepositResponseWire, ExchangeHttpError> {
    run_exchange_action::<CDepositAction>(body, deps).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "cDeposit" {
        return Err(
            CDepositContractError::UnexpectedActionType(request.action.type_.clone()).into()
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
        return Err(CDepositContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(CDepositContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| CDepositContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| CDepositContractError::InvalidSignatureChainId)?;
    if request.action.nonce != request.common.nonce {
        return Err(CDepositContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::CDepositResponseWire, ExchangeHttpError> {
    Ok(reply::CDepositResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.wei, 1000);
    }

    #[test]
    fn rejects_nonce_mismatch() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "cDeposit",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
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
                "type": "cDeposit",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
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
