use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::exchange::actions::order::reply::{
    OrderResponseDataWire, OrderResponseEnvelopeWire, OrderResponseWire, OrderStatusWire,
    RestingOrderStatusWire,
};
#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::{validate_cloid, validate_common_fields};
use crate::exchange::common::wire::ExchangeRequestEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

const STUB_MODIFIED_OID: u64 = 77738309;

#[derive(Debug, thiserror::Error)]
pub enum ModifyContractError {
    #[error("Unexpected `action.type` for modify handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.oid`. Expected a positive order id or 128-bit hex client order id.")]
    InvalidOid,
    #[error("Invalid `action.order.p`. Expected a non-empty decimal string.")]
    InvalidPrice,
    #[error("Invalid `action.order.s`. Expected a non-empty decimal string.")]
    InvalidSize,
    #[error("Invalid `action.order.c`. Expected a 128-bit hex client order id.")]
    InvalidOrderCloid,
    #[error("Invalid `action.order.t.limit.tif`. Expected `Alo`, `Ioc`, or `Gtc`.")]
    InvalidTimeInForce,
    #[error("Invalid `action.order.t.trigger.triggerPx`. Expected a non-empty decimal string.")]
    InvalidTriggerPrice,
    #[error("Invalid `action.order.t.trigger.tpsl`. Expected `tp` or `sl`.")]
    InvalidTriggerKind,
    #[error("Invalid `action.order.t`. Expected exactly one of `limit` or `trigger`.")]
    InvalidOrderType,
    #[error("Invalid `action.a`. `a` must be omitted when false.")]
    InvalidAlwaysPlaceFlag,
    #[error("Invalid `action.order.builder`. Builder is not supported for `modify`.")]
    BuilderNotSupported,
}

pub mod reply {
    pub use crate::exchange::actions::order::reply::OrderResponseWire as ModifyResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    oid: Value,
    order: OrderWire,
    #[serde(rename = "a")]
    always_place: Option<bool>,
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
    builder: Option<Value>,
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

pub(crate) struct ModifyAction;

impl ExchangeActionHandler for ModifyAction {
    type Request = RequestWire;
    type Reply = reply::ModifyResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "modify" {
        return Err(ModifyContractError::UnexpectedActionType(request.action.type_.clone()).into());
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
    validate_oid(&request.action.oid)?;
    if matches!(request.action.always_place, Some(false)) {
        return Err(ModifyContractError::InvalidAlwaysPlaceFlag.into());
    }
    validate_order(&request.action.order)?;
    Ok(())
}

fn validate_oid(oid: &Value) -> Result<(), ExchangeHttpError> {
    if oid.as_u64().is_some_and(|value| value > 0) {
        return Ok(());
    }
    if let Some(cloid) = oid.as_str() {
        validate_cloid(cloid).map_err(|_| ModifyContractError::InvalidOid)?;
        return Ok(());
    }
    Err(ModifyContractError::InvalidOid.into())
}

fn validate_order(order: &OrderWire) -> Result<(), ExchangeHttpError> {
    if order.builder.is_some() {
        return Err(ModifyContractError::BuilderNotSupported.into());
    }
    if order.p.trim().is_empty() {
        return Err(ModifyContractError::InvalidPrice.into());
    }
    if order.s.trim().is_empty() {
        return Err(ModifyContractError::InvalidSize.into());
    }
    if let Some(cloid) = &order.c {
        validate_cloid(cloid).map_err(|_| ModifyContractError::InvalidOrderCloid)?;
    }
    match (&order.t.limit, &order.t.trigger) {
        (Some(limit), None) => {
            if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                return Err(ModifyContractError::InvalidTimeInForce.into());
            }
        }
        (None, Some(trigger)) => {
            if trigger.trigger_px.trim().is_empty() {
                return Err(ModifyContractError::InvalidTriggerPrice.into());
            }
            if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                return Err(ModifyContractError::InvalidTriggerKind.into());
            }
        }
        _ => return Err(ModifyContractError::InvalidOrderType.into()),
    }
    Ok(())
}

async fn execute(_request: RequestWire) -> Result<reply::ModifyResponseWire, ExchangeHttpError> {
    // 官方文档未给出 modify 成功响应示例。
    // 官方 Python SDK 的 basic_order_modify.py 只打印 modify_result，没有对 shape 做任何断言。
    // 这里先采用与 order 一致的最小成功形状并固定在测试中。
    Ok(OrderResponseWire {
        status: "ok",
        response: OrderResponseEnvelopeWire {
            type_: "order",
            data: OrderResponseDataWire {
                statuses: vec![OrderStatusWire::Resting {
                    resting: RestingOrderStatusWire { oid: STUB_MODIFIED_OID },
                }],
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request =
            parse_json_request::<RequestWire>(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.order.p, "1891.4");
    }

    #[test]
    fn rejects_false_always_place() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": {
                    "type": "modify",
                    "oid": 77738308,
                    "a": false,
                    "order": {
                        "a": 10000,
                        "b": true,
                        "p": "1891.4",
                        "s": "0.02",
                        "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }
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
        assert_eq!(error.to_string(), "Invalid `action.a`. `a` must be omitted when false.");
    }

    #[actix_web::test]
    async fn reply_snapshot_is_stable() {
        let response = execute(
            parse_json_request::<RequestWire>(valid_request_json()).expect("request parses"),
        )
        .await
        .expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"order\",\n    \"data\": {\n      \"statuses\": [\n        {\n          \"resting\": {\n            \"oid\": 77738309\n          }\n        }\n      ]\n    }\n  }\n}"
        );
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "modify",
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
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
