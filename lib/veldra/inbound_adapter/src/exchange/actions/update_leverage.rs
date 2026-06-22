use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_default_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateLeverageContractError {
    #[error("Unexpected `action.type` for updateLeverage handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.leverage`. Expected an integer greater than or equal to 1.")]
    InvalidLeverage,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as UpdateLeverageResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    asset: u32,
    #[serde(rename = "isCross")]
    is_cross: bool,
    leverage: u64,
}

pub(crate) struct UpdateLeverageAction;

impl ExchangeActionHandler for UpdateLeverageAction {
    type Request = RequestWire;
    type Reply = reply::UpdateLeverageResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "updateLeverage" {
        return Err(ExchangeHttpError::contract(
            UpdateLeverageContractError::UnexpectedActionType(request.action.type_.clone()),
        ));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.leverage < 1 {
        return Err(ExchangeHttpError::contract(UpdateLeverageContractError::InvalidLeverage));
    }
    Ok(())
}

async fn execute() -> Result<reply::UpdateLeverageResponseWire, ExchangeHttpError> {
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_update_leverage_request() {
        let request = parse_json_request::<RequestWire>(valid_update_leverage_json())
            .expect("request should parse");
        assert_eq!(request.action.type_, "updateLeverage");
        assert_eq!(request.action.leverage, 5);
    }

    #[test]
    fn rejects_zero_leverage() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "updateLeverage",
                    "asset": 7,
                    "isCross": true,
                    "leverage": 0
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
            "Invalid `action.leverage`. Expected an integer greater than or equal to 1."
        );
    }

    #[actix_web::test]
    async fn update_leverage_reply_snapshot_is_stable() {
        let response = execute().await.expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
        );
    }

    fn valid_update_leverage_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "updateLeverage",
                "asset": 7,
                "isCross": true,
                "leverage": 5
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
