use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeEmptyResponseWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateIsolatedMarginContractError {
    #[error("Unexpected `action.type` for updateIsolatedMargin handler: `{0}`.")]
    UnexpectedActionType(String),
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as UpdateIsolatedMarginResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    asset: u32,
    #[serde(rename = "isBuy")]
    is_buy: bool,
    ntli: i64,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::UpdateIsolatedMarginResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    parse_json_request(body)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "updateIsolatedMargin" {
        return Err(UpdateIsolatedMarginContractError::UnexpectedActionType(
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
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::UpdateIsolatedMarginResponseWire, ExchangeHttpError> {
    Ok(ExchangeEmptyResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_update_isolated_margin_request() {
        let request = parse(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.type_, "updateIsolatedMargin");
        assert_eq!(request.action.ntli, 1_000_000);
    }

    #[actix_web::test]
    async fn update_isolated_margin_reply_snapshot_is_stable() {
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
                "type": "updateIsolatedMargin",
                "asset": 7,
                "isBuy": true,
                "ntli": 1000000
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
