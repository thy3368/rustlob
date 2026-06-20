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
pub enum ValidatorL1StreamContractError {
    #[error("Unexpected `action.type` for validatorL1Stream handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.riskFreeRate`. Expected a non-empty decimal string.")]
    InvalidRiskFreeRate,
    #[error("`vaultAddress` is not supported for `validatorL1Stream`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `validatorL1Stream`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as ValidatorL1StreamResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "riskFreeRate")]
    risk_free_rate: String,
}

struct ValidatorL1StreamAction;

impl ExchangeActionHandler for ValidatorL1StreamAction {
    type Request = RequestWire;
    type Reply = reply::ValidatorL1StreamResponseWire;

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
) -> Result<reply::ValidatorL1StreamResponseWire, ExchangeHttpError> {
    run_exchange_action::<ValidatorL1StreamAction>(body, deps).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "validatorL1Stream" {
        return Err(ValidatorL1StreamContractError::UnexpectedActionType(
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
        return Err(ValidatorL1StreamContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(ValidatorL1StreamContractError::ExpiresAfterNotSupported.into());
    }
    if request.action.risk_free_rate.trim().is_empty() {
        return Err(ValidatorL1StreamContractError::InvalidRiskFreeRate.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::ValidatorL1StreamResponseWire, ExchangeHttpError> {
    Ok(reply::ValidatorL1StreamResponseWire {
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
        assert_eq!(request.action.risk_free_rate, "0.04");
    }

    #[test]
    fn rejects_empty_rate() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": { "type": "validatorL1Stream", "riskFreeRate": "" },
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
            "Invalid `action.riskFreeRate`. Expected a non-empty decimal string."
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
            "action": { "type": "validatorL1Stream", "riskFreeRate": "0.04" },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
