use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum Hip3LiquidatorTransferContractError {
    #[error("Unexpected `action.type` for hip3LiquidatorTransfer handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.dex`. Expected a non-empty DEX identifier.")]
    InvalidDex,
    #[error("`vaultAddress` is not supported for `hip3LiquidatorTransfer`.")]
    VaultAddressNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as Hip3LiquidatorTransferResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    dex: String,
    ntl: i64,
    #[serde(rename = "isDeposit")]
    is_deposit: bool,
}

struct Hip3LiquidatorTransferAction;

impl ExchangeActionHandler for Hip3LiquidatorTransferAction {
    type Request = RequestWire;
    type Reply = reply::Hip3LiquidatorTransferResponseWire;

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
) -> Result<reply::Hip3LiquidatorTransferResponseWire, ExchangeHttpError> {
    run_exchange_action::<Hip3LiquidatorTransferAction>(body, deps).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "hip3LiquidatorTransfer" {
        return Err(Hip3LiquidatorTransferContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )
        .into());
    }
    validate_common_fields(
        request.common.nonce,
        request.common.expires_after,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        request.common.vault_address.as_deref(),
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.common.vault_address.is_some() {
        return Err(Hip3LiquidatorTransferContractError::VaultAddressNotSupported.into());
    }
    if request.action.dex.trim().is_empty() {
        return Err(Hip3LiquidatorTransferContractError::InvalidDex.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::Hip3LiquidatorTransferResponseWire, ExchangeHttpError> {
    Ok(reply::Hip3LiquidatorTransferResponseWire {
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
        assert_eq!(request.action.dex, "xyz");
    }

    #[test]
    fn rejects_empty_dex() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "hip3LiquidatorTransfer",
                    "dex": "",
                    "ntl": 1000000000,
                    "isDeposit": true
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
        assert_eq!(error.to_string(), "Invalid `action.dex`. Expected a non-empty DEX identifier.");
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
                "type": "hip3LiquidatorTransfer",
                "dex": "xyz",
                "ntl": 1000000000,
                "isDeposit": true
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
