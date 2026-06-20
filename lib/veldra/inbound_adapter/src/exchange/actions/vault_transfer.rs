use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::{validate_common_fields, validate_hex_address};
use crate::exchange::common::wire::{CommonExchangeFields, ExchangeEmptyResponseEnvelopeWire};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum VaultTransferContractError {
    #[error("Unexpected `action.type` for vaultTransfer handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.vaultAddress`. Expected a 42-character hexadecimal address.")]
    InvalidVaultAddress,
    #[error("Invalid `action.usd`. Expected a numeric amount.")]
    InvalidUsd,
    #[error("`vaultAddress` is not supported at the outer request level for `vaultTransfer`.")]
    OuterVaultAddressNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as VaultTransferResponseWire;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct VaultTransferRequestWire {
    action: VaultTransferActionWire,
    #[serde(flatten)]
    common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct VaultTransferActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "vaultAddress")]
    vault_address: String,
    #[serde(rename = "isDeposit")]
    is_deposit: bool,
    usd: Number,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::VaultTransferResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<VaultTransferRequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &VaultTransferRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "vaultTransfer" {
        return Err(
            VaultTransferContractError::UnexpectedActionType(request.action.type_.clone()).into()
        );
    }
    validate_common_fields(
        request.common.nonce,
        request.common.expires_after,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        None,
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.common.vault_address.is_some() {
        return Err(VaultTransferContractError::OuterVaultAddressNotSupported.into());
    }
    validate_hex_address(&request.action.vault_address)
        .map_err(|_| VaultTransferContractError::InvalidVaultAddress)?;
    if request.action.usd.as_i64().is_none()
        && request.action.usd.as_u64().is_none()
        && request.action.usd.as_f64().is_none()
    {
        return Err(VaultTransferContractError::InvalidUsd.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::VaultTransferResponseWire, ExchangeHttpError> {
    Ok(reply::VaultTransferResponseWire {
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
        assert!(request.action.is_deposit);
    }

    #[test]
    fn rejects_invalid_action_vault_address() {
        let request = parse(
            br#"{
                "action": {
                    "type": "vaultTransfer",
                    "vaultAddress": "0x1234",
                    "isDeposit": true,
                    "usd": 10
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
            "Invalid `action.vaultAddress`. Expected a 42-character hexadecimal address."
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
                "type": "vaultTransfer",
                "vaultAddress": "0x8888888888888888888888888888888888888888",
                "isDeposit": true,
                "usd": 10
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
