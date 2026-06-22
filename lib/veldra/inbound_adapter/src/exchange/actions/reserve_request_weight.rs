use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_default_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum ReserveRequestWeightContractError {
    #[error("Unexpected `action.type` for reserveRequestWeight handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.weight`. Expected a positive integer.")]
    InvalidWeight,
    #[error("`vaultAddress` is not supported for `reserveRequestWeight`.")]
    VaultAddressNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as ReserveRequestWeightResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    weight: u64,
}

pub(crate) struct ReserveRequestWeightAction;

impl ExchangeActionHandler for ReserveRequestWeightAction {
    type Request = RequestWire;
    type Reply = reply::ReserveRequestWeightResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "reserveRequestWeight" {
        return Err(ExchangeHttpError::contract(
            ReserveRequestWeightContractError::UnexpectedActionType(request.action.type_.clone()),
        ));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.common.vault_address.is_some() {
        return Err(ExchangeHttpError::contract(
            ReserveRequestWeightContractError::VaultAddressNotSupported,
        ));
    }
    if request.action.weight == 0 {
        return Err(ExchangeHttpError::contract(ReserveRequestWeightContractError::InvalidWeight));
    }
    Ok(())
}

async fn execute() -> Result<reply::ReserveRequestWeightResponseWire, ExchangeHttpError> {
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request =
            parse_json_request::<RequestWire>(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.weight, 1);
    }

    #[test]
    fn rejects_zero_weight() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": { "type": "reserveRequestWeight", "weight": 0 },
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
        assert_eq!(error.to_string(), "Invalid `action.weight`. Expected a positive integer.");
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
            "action": { "type": "reserveRequestWeight", "weight": 1 },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
