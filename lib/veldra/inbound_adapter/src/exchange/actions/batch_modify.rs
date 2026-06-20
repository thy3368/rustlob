use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::order::reply::{
    OrderResponseDataWire, OrderResponseEnvelopeWire, OrderResponseWire, OrderStatusWire,
    RestingOrderStatusWire,
};
#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::exchange::common::validate::{validate_cloid, validate_common_fields};
use crate::exchange::common::wire::ExchangeRequestEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

const STUB_BATCH_MODIFIED_OID_BASE: u64 = 77738400;

#[derive(Debug, thiserror::Error)]
pub enum BatchModifyContractError {
    #[error("Unexpected `action.type` for batchModify handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.modifies`. Expected at least one modify entry.")]
    EmptyModifies,
    #[error(
        "Invalid `action.modifies[].oid`. Expected a positive order id or 128-bit hex client order id."
    )]
    InvalidOid,
    #[error("Invalid `action.modifies[].order.p`. Expected a non-empty decimal string.")]
    InvalidPrice,
    #[error("Invalid `action.modifies[].order.s`. Expected a non-empty decimal string.")]
    InvalidSize,
    #[error("Invalid `action.modifies[].order.c`. Expected a 128-bit hex client order id.")]
    InvalidOrderCloid,
    #[error("Invalid `action.modifies[].order.t.limit.tif`. Expected `Alo`, `Ioc`, or `Gtc`.")]
    InvalidTimeInForce,
    #[error(
        "Invalid `action.modifies[].order.t.trigger.triggerPx`. Expected a non-empty decimal string."
    )]
    InvalidTriggerPrice,
    #[error("Invalid `action.modifies[].order.t.trigger.tpsl`. Expected `tp` or `sl`.")]
    InvalidTriggerKind,
    #[error("Invalid `action.modifies[].order.t`. Expected exactly one of `limit` or `trigger`.")]
    InvalidOrderType,
    #[error("Invalid `action.a`. `a` must be omitted when false.")]
    InvalidAlwaysPlaceFlag,
}

pub mod reply {
    pub use crate::exchange::actions::order::reply::OrderResponseWire as BatchModifyResponseWire;
}

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    modifies: Vec<ModifyWire>,
    #[serde(rename = "a")]
    always_place: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModifyWire {
    oid: Value,
    order: OrderWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderWire {
    a: u32,
    b: bool,
    p: String,
    s: String,
    r: bool,
    t: OrderTypeWire,
    c: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderTypeWire {
    limit: Option<LimitWire>,
    trigger: Option<TriggerWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LimitWire {
    tif: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TriggerWire {
    #[serde(rename = "isMarket")]
    is_market: bool,
    #[serde(rename = "triggerPx")]
    trigger_px: String,
    tpsl: String,
}

struct BatchModifyAction;

impl ExchangeActionHandler for BatchModifyAction {
    type Request = RequestWire;
    type Reply = reply::BatchModifyResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute<'a>(
        request: Self::Request,
        deps: &'a ExchangeActionDeps,
    ) -> ExchangeActionFuture<'a, Self::Reply> {
        Box::pin(execute(request, deps))
    }
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::BatchModifyResponseWire, ExchangeHttpError> {
    run_exchange_action::<BatchModifyAction>(body, deps).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "batchModify" {
        return Err(
            BatchModifyContractError::UnexpectedActionType(request.action.type_.clone()).into()
        );
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
    if request.action.modifies.is_empty() {
        return Err(BatchModifyContractError::EmptyModifies.into());
    }
    if matches!(request.action.always_place, Some(false)) {
        return Err(BatchModifyContractError::InvalidAlwaysPlaceFlag.into());
    }
    for modify in &request.action.modifies {
        validate_oid(&modify.oid)?;
        validate_order(&modify.order)?;
    }
    Ok(())
}

fn validate_oid(oid: &Value) -> Result<(), ExchangeHttpError> {
    if oid.as_u64().is_some_and(|value| value > 0) {
        return Ok(());
    }
    if let Some(cloid) = oid.as_str() {
        validate_cloid(cloid).map_err(|_| BatchModifyContractError::InvalidOid)?;
        return Ok(());
    }
    Err(BatchModifyContractError::InvalidOid.into())
}

fn validate_order(order: &OrderWire) -> Result<(), ExchangeHttpError> {
    if order.p.trim().is_empty() {
        return Err(BatchModifyContractError::InvalidPrice.into());
    }
    if order.s.trim().is_empty() {
        return Err(BatchModifyContractError::InvalidSize.into());
    }
    if let Some(cloid) = &order.c {
        validate_cloid(cloid).map_err(|_| BatchModifyContractError::InvalidOrderCloid)?;
    }
    match (&order.t.limit, &order.t.trigger) {
        (Some(limit), None) => {
            if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                return Err(BatchModifyContractError::InvalidTimeInForce.into());
            }
        }
        (None, Some(trigger)) => {
            if trigger.trigger_px.trim().is_empty() {
                return Err(BatchModifyContractError::InvalidTriggerPrice.into());
            }
            if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                return Err(BatchModifyContractError::InvalidTriggerKind.into());
            }
        }
        _ => return Err(BatchModifyContractError::InvalidOrderType.into()),
    }
    Ok(())
}

async fn execute(
    request: RequestWire,
    _deps: &ExchangeActionDeps,
) -> Result<reply::BatchModifyResponseWire, ExchangeHttpError> {
    // 官方文档未给出 batchModify 成功响应示例。
    // 官方 Python SDK 的 modify_order() 实际组装的是 batchModify action，但示例仅打印返回值。
    let statuses = request
        .action
        .modifies
        .iter()
        .enumerate()
        .map(|(index, _)| OrderStatusWire::Resting {
            resting: RestingOrderStatusWire { oid: STUB_BATCH_MODIFIED_OID_BASE + index as u64 },
        })
        .collect();
    Ok(OrderResponseWire {
        status: "ok",
        response: OrderResponseEnvelopeWire {
            type_: "order",
            data: OrderResponseDataWire { statuses },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.modifies.len(), 2);
    }

    #[test]
    fn rejects_empty_modifies() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": { "type": "batchModify", "modifies": [] },
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
            "Invalid `action.modifies`. Expected at least one modify entry."
        );
    }

    #[actix_web::test]
    async fn reply_snapshot_is_stable() {
        let response = execute(
            parse_json_request::<RequestWire>(valid_request_json()).expect("request parses"),
            &ExchangeActionDeps::default(),
        )
        .await
        .expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"order\",\n    \"data\": {\n      \"statuses\": [\n        {\n          \"resting\": {\n            \"oid\": 77738400\n          }\n        },\n        {\n          \"resting\": {\n            \"oid\": 77738401\n          }\n        }\n      ]\n    }\n  }\n}"
        );
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "batchModify",
                "modifies": [
                    {
                        "oid": 77738308,
                        "order": {
                            "a": 10000,
                            "b": true,
                            "p": "1891.4",
                            "s": "0.02",
                            "r": false,
                            "t": { "limit": { "tif": "Gtc" } }
                        }
                    },
                    {
                        "oid": "0x1234567890abcdef1234567890abcdef",
                        "order": {
                            "a": 10001,
                            "b": false,
                            "p": "1890.0",
                            "s": "0.04",
                            "r": false,
                            "t": { "limit": { "tif": "Ioc" } }
                        }
                    }
                ]
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
