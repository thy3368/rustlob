use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::command::exchange::actions::cancel::reply::CancelStatusWire;
use crate::command::exchange::actions::cancel::{
    CancelSpotOrderV2Lookup, CancelSpotOrderV2Request, DEFAULT_EXCHANGE_PARTY_ID,
};
use crate::command::exchange::actions::order::PlaceSpotOrderV2Request;
use crate::command::exchange::actions::order::reply::{
    OrderResponseDataWire, OrderResponseEnvelopeWire, OrderResponseWire, OrderStatusWire,
    RestingOrderStatusWire,
};
use crate::command::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::command::exchange::common::validate::{validate_cloid, validate_envelope_common};
use crate::command::exchange::common::wire::ExchangeRequestEnvelopeWire;
use crate::command::exchange::error::ExchangeHttpError;
#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::common::parse::parse_json_request as parse_exchange_json_request;

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
    pub use crate::command::exchange::actions::order::reply::OrderResponseWire as BatchModifyResponseWire;
}

pub trait BatchModifyCancelPlaceExecutor {
    fn cancel(
        &self,
        request: CancelSpotOrderV2Request,
    ) -> Result<CancelStatusWire, ExchangeHttpError>;

    fn place(&self, request: PlaceSpotOrderV2Request)
    -> Result<OrderStatusWire, ExchangeHttpError>;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
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

pub(crate) struct BatchModifyAction;

impl ExchangeActionHandler for BatchModifyAction {
    type Request = RequestWire;
    type Reply = reply::BatchModifyResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "batchModify" {
        return Err(ExchangeHttpError::contract(BatchModifyContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.modifies.is_empty() {
        return Err(ExchangeHttpError::contract(BatchModifyContractError::EmptyModifies));
    }
    if matches!(request.action.always_place, Some(false)) {
        return Err(ExchangeHttpError::contract(BatchModifyContractError::InvalidAlwaysPlaceFlag));
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
        validate_cloid(cloid)
            .map_err(|_| ExchangeHttpError::contract(BatchModifyContractError::InvalidOid))?;
        return Ok(());
    }
    Err(ExchangeHttpError::contract(BatchModifyContractError::InvalidOid))
}

fn validate_order(order: &OrderWire) -> Result<(), ExchangeHttpError> {
    if order.p.trim().is_empty() {
        return Err(ExchangeHttpError::contract(BatchModifyContractError::InvalidPrice));
    }
    if order.s.trim().is_empty() {
        return Err(ExchangeHttpError::contract(BatchModifyContractError::InvalidSize));
    }
    if let Some(cloid) = &order.c {
        validate_cloid(cloid).map_err(|_| {
            ExchangeHttpError::contract(BatchModifyContractError::InvalidOrderCloid)
        })?;
    }
    match (&order.t.limit, &order.t.trigger) {
        (Some(limit), None) => {
            if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                return Err(ExchangeHttpError::contract(
                    BatchModifyContractError::InvalidTimeInForce,
                ));
            }
        }
        (None, Some(trigger)) => {
            if trigger.trigger_px.trim().is_empty() {
                return Err(ExchangeHttpError::contract(
                    BatchModifyContractError::InvalidTriggerPrice,
                ));
            }
            if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                return Err(ExchangeHttpError::contract(
                    BatchModifyContractError::InvalidTriggerKind,
                ));
            }
        }
        _ => return Err(ExchangeHttpError::contract(BatchModifyContractError::InvalidOrderType)),
    }
    Ok(())
}

async fn execute(
    request: RequestWire,
) -> Result<reply::BatchModifyResponseWire, ExchangeHttpError> {
    let executor = DefaultBatchModifyCancelPlaceExecutor::new();
    let statuses = execute_with_cancel_place_executor(request, &executor);
    Ok(OrderResponseWire {
        status: "ok",
        response: OrderResponseEnvelopeWire {
            type_: "order",
            data: OrderResponseDataWire { statuses },
        },
    })
}

pub fn run_batch_modify_cancel_replace_with_executor(
    body: &[u8],
    executor: &impl BatchModifyCancelPlaceExecutor,
) -> Result<reply::BatchModifyResponseWire, ExchangeHttpError> {
    let request = parse_exchange_json_request::<RequestWire, ExchangeHttpError>(body)?;
    validate(&request)?;
    let statuses = execute_with_cancel_place_executor(request, executor);
    Ok(OrderResponseWire {
        status: "ok",
        response: OrderResponseEnvelopeWire {
            type_: "order",
            data: OrderResponseDataWire { statuses },
        },
    })
}

fn execute_with_cancel_place_executor(
    request: RequestWire,
    executor: &impl BatchModifyCancelPlaceExecutor,
) -> Vec<OrderStatusWire> {
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    request
        .action
        .modifies
        .iter()
        .map(|modify| execute_single_cancel_replace(&party_id, modify, executor))
        .collect()
}

fn execute_single_cancel_replace(
    party_id: &str,
    modify: &ModifyWire,
    executor: &impl BatchModifyCancelPlaceExecutor,
) -> OrderStatusWire {
    let cancel_request = match cancel_request_from_modify(party_id, modify) {
        Ok(request) => request,
        Err(error) => return OrderStatusWire::Error { error: error.to_string() },
    };
    if let Err(error) = executor.cancel(cancel_request) {
        return OrderStatusWire::Error { error: error.to_string() };
    }

    let place_request = match place_request_from_modify(party_id, modify) {
        Ok(request) => request,
        Err(error) => return OrderStatusWire::Error { error: error.to_string() },
    };
    match executor.place(place_request) {
        Ok(status) => status,
        Err(error) => OrderStatusWire::Error { error: error.to_string() },
    }
}

fn cancel_request_from_modify(
    party_id: &str,
    modify: &ModifyWire,
) -> Result<CancelSpotOrderV2Request, ExchangeHttpError> {
    let lookup = if let Some(oid) = modify.oid.as_u64() {
        CancelSpotOrderV2Lookup::Oid(oid)
    } else if let Some(cloid) = modify.oid.as_str() {
        CancelSpotOrderV2Lookup::Cloid(cloid.to_string())
    } else {
        return Err(ExchangeHttpError::contract(BatchModifyContractError::InvalidOid));
    };

    Ok(CancelSpotOrderV2Request { party_id: party_id.to_string(), asset: modify.order.a, lookup })
}

fn place_request_from_modify(
    party_id: &str,
    modify: &ModifyWire,
) -> Result<PlaceSpotOrderV2Request, ExchangeHttpError> {
    let Some(limit) = &modify.order.t.limit else {
        return Err(ExchangeHttpError::contract(BatchModifyContractError::InvalidOrderType));
    };

    Ok(PlaceSpotOrderV2Request {
        party_id: party_id.to_string(),
        asset: modify.order.a,
        is_buy: modify.order.b,
        price: modify.order.p.clone(),
        size: modify.order.s.clone(),
        tif: limit.tif.to_ascii_lowercase(),
        cloid: modify.order.c.clone(),
    })
}

struct DefaultBatchModifyCancelPlaceExecutor {
    next_oid: std::cell::Cell<u64>,
}

impl DefaultBatchModifyCancelPlaceExecutor {
    fn new() -> Self {
        Self { next_oid: std::cell::Cell::new(STUB_BATCH_MODIFIED_OID_BASE) }
    }
}

impl BatchModifyCancelPlaceExecutor for DefaultBatchModifyCancelPlaceExecutor {
    fn cancel(
        &self,
        _request: CancelSpotOrderV2Request,
    ) -> Result<CancelStatusWire, ExchangeHttpError> {
        Ok(CancelStatusWire::Success("success"))
    }

    fn place(
        &self,
        _request: PlaceSpotOrderV2Request,
    ) -> Result<OrderStatusWire, ExchangeHttpError> {
        let oid = self.next_oid.get();
        self.next_oid.set(oid + 1);
        Ok(OrderStatusWire::Resting { resting: RestingOrderStatusWire { oid } })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.modifies.len(), 2);
    }

    #[test]
    fn rejects_empty_modifies() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
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
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
                .expect("request parses"),
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
