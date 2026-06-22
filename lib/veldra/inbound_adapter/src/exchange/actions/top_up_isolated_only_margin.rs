use serde::{Deserialize, Serialize};

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
pub enum TopUpIsolatedOnlyMarginContractError {
    #[error("Unexpected `action.type` for topUpIsolatedOnlyMargin handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.leverage`. Expected a non-empty decimal string.")]
    InvalidLeverage,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as TopUpIsolatedOnlyMarginResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    asset: u32,
    leverage: String,
}

struct TopUpIsolatedOnlyMarginAction;

impl ExchangeActionHandler for TopUpIsolatedOnlyMarginAction {
    type Request = RequestWire;
    type Reply = reply::TopUpIsolatedOnlyMarginResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

pub async fn handle(
    body: &[u8],
) -> Result<reply::TopUpIsolatedOnlyMarginResponseWire, ExchangeHttpError> {
    run_exchange_action::<TopUpIsolatedOnlyMarginAction>(body).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "topUpIsolatedOnlyMargin" {
        return Err(TopUpIsolatedOnlyMarginContractError::UnexpectedActionType(
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
    if request.action.leverage.trim().is_empty() {
        return Err(TopUpIsolatedOnlyMarginContractError::InvalidLeverage.into());
    }
    Ok(())
}

async fn execute() -> Result<reply::TopUpIsolatedOnlyMarginResponseWire, ExchangeHttpError> {
    Ok(reply::TopUpIsolatedOnlyMarginResponseWire {
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
        assert_eq!(request.action.leverage, "2.5");
    }

    #[test]
    fn rejects_empty_leverage() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": { "type": "topUpIsolatedOnlyMargin", "asset": 12, "leverage": "" },
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
            "Invalid `action.leverage`. Expected a non-empty decimal string."
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
            "action": { "type": "topUpIsolatedOnlyMargin", "asset": 12, "leverage": "2.5" },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
