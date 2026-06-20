use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{
    ExchangeEmptyResponseEnvelopeWire, ExchangeRequestEnvelopeWire,
};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum ScheduleCancelContractError {
    #[error("Unexpected `action.type` for scheduleCancel handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.time`. Expected it to be at least 5 seconds after `nonce`.")]
    InvalidTime,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as ScheduleCancelResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    time: Option<u64>,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::ScheduleCancelResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    parse_json_request(body)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "scheduleCancel" {
        return Err(ScheduleCancelContractError::UnexpectedActionType(
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
    if let Some(time) = request.action.time {
        if time < request.common.nonce.saturating_add(5_000) {
            return Err(ScheduleCancelContractError::InvalidTime.into());
        }
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::ScheduleCancelResponseWire, ExchangeHttpError> {
    // 官方文档未展示 scheduleCancel 成功响应。
    // 官方 Python SDK 的 basic_schedule_cancel.py 仅原样打印结果，没有约束 response shape。
    Ok(reply::ScheduleCancelResponseWire {
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
        assert_eq!(request.action.time, Some(1710000006000));
    }

    #[test]
    fn rejects_too_soon_time() {
        let request = parse(
            br#"{
                "action": { "type": "scheduleCancel", "time": 1710000004000 },
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
            "Invalid `action.time`. Expected it to be at least 5 seconds after `nonce`."
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
            "action": { "type": "scheduleCancel", "time": 1710000006000 },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
