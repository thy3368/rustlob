use cmd_handler::command_use_case_def2::{ExecutionError, MiFamilyExecutionSpec};
use example_core_use_case::{ModifySpotOrderV2OrderType, OrderId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::command::exchange::actions::cancel::DEFAULT_EXCHANGE_PARTY_ID;
use crate::command::exchange::actions::modify::{
    ModifySpotOrderV2Request, SpotOrderV2ModifyExecutionSpec, execute_modify_spot_order_v2,
};
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
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());
    let statuses = request
        .action
        .modifies
        .iter()
        .map(|modify| execute_single_modify(&party_id, modify))
        .collect();

    Ok(OrderResponseWire {
        status: "ok",
        response: OrderResponseEnvelopeWire {
            type_: "order",
            data: OrderResponseDataWire { statuses },
        },
    })
}

fn execute_single_modify(party_id: &str, modify: &ModifyWire) -> OrderStatusWire {
    let modify_request = match from_wire_batch_modify(party_id, modify) {
        Ok(request) => request,
        Err(error) => return OrderStatusWire::Error { error: error.to_string() },
    };
    let command = SpotOrderV2ModifyExecutionSpec::command(&modify_request);

    match execute_modify_spot_order_v2(&command) {
        Ok(result) => OrderStatusWire::Resting {
            resting: RestingOrderStatusWire {
                oid: result.changes.updated_order.after.exchange_oid().unwrap_or(0),
            },
        },
        Err(error) => OrderStatusWire::Error { error: modify_execution_error_message(error) },
    }
}

fn from_wire_batch_modify(
    party_id: &str,
    modify: &ModifyWire,
) -> Result<ModifySpotOrderV2Request, BatchModifyContractError> {
    Ok(ModifySpotOrderV2Request {
        party_id: party_id.to_string(),
        asset: modify.order.a,
        order_id: order_id_from_wire_oid(&modify.oid)?,
        is_buy: modify.order.b,
        price: decimal_wire_to_core_units(&modify.order.p),
        size: decimal_wire_to_core_units(&modify.order.s),
        order_type: modify_order_type_from_wire(&modify.order.t)?,
        cloid: modify.order.c.clone(),
    })
}

fn order_id_from_wire_oid(oid: &Value) -> Result<OrderId, BatchModifyContractError> {
    if let Some(oid) = oid.as_u64() {
        if oid > 0 {
            return Ok(OrderId::Oid(oid));
        }
    }
    if let Some(cloid) = oid.as_str() {
        if validate_cloid(cloid).is_ok() {
            return Ok(OrderId::Cloid(cloid.to_string()));
        }
    }
    Err(BatchModifyContractError::InvalidOid)
}

fn modify_order_type_from_wire(
    order_type: &OrderTypeWire,
) -> Result<ModifySpotOrderV2OrderType, BatchModifyContractError> {
    match (&order_type.limit, &order_type.trigger) {
        (Some(limit), None) => Ok(ModifySpotOrderV2OrderType::Limit { tif: limit.tif.clone() }),
        (None, Some(trigger)) => Ok(ModifySpotOrderV2OrderType::Trigger {
            is_market: trigger.is_market,
            trigger_price: decimal_wire_to_core_units(&trigger.trigger_px),
            trigger_role: trigger.tpsl.clone(),
        }),
        _ => Err(BatchModifyContractError::InvalidOrderType),
    }
}

fn decimal_wire_to_core_units(raw: &str) -> String {
    let trimmed = raw.trim();
    let mut seen_dot = false;
    let mut normalized = String::with_capacity(trimmed.len());

    for ch in trimmed.chars() {
        if ch == '.' {
            if seen_dot {
                return raw.to_string();
            }
            seen_dot = true;
            continue;
        }
        if !ch.is_ascii_digit() {
            return raw.to_string();
        }
        normalized.push(ch);
    }

    if normalized.is_empty() { raw.to_string() } else { normalized }
}

fn modify_execution_error_message<BE, OE>(error: ExecutionError<BE, OE>) -> String
where
    BE: std::fmt::Display,
    OE: std::fmt::Display,
{
    match error {
        ExecutionError::Business(error) => error.to_string(),
        ExecutionError::ProjectEvents(error) => {
            format!("project replayable events failed: {error}")
        }
        ExecutionError::LoadState(error) => format!("load_state failed: {error}"),
        ExecutionError::Persist(error) => format!("persist failed: {error}"),
        ExecutionError::Replay(error) => format!("replay failed: {error}"),
        ExecutionError::Publish(error) => format!("publish failed: {error}"),
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

    #[test]
    fn maps_batch_items_to_modify_commands() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request parses");

        let first = from_wire_batch_modify("trader", &request.action.modifies[0])
            .expect("first modify request should build");
        let first_command = SpotOrderV2ModifyExecutionSpec::command(&first);
        assert_eq!(first_command.party_id, "trader");
        assert_eq!(first_command.asset, 10_000);
        assert_eq!(first_command.order_id, OrderId::Oid(77738308));
        assert!(first_command.is_buy);
        assert_eq!(first_command.price, "18914");
        assert_eq!(first_command.size, "002");
        assert_eq!(
            first_command.order_type,
            ModifySpotOrderV2OrderType::Limit { tif: "Gtc".to_string() }
        );

        let second = from_wire_batch_modify("trader", &request.action.modifies[1])
            .expect("second modify request should build");
        let second_command = SpotOrderV2ModifyExecutionSpec::command(&second);
        assert_eq!(
            second_command.order_id,
            OrderId::Cloid("0x1234567890abcdef1234567890abcdef".to_string())
        );
        assert_eq!(
            second_command.order_type,
            ModifySpotOrderV2OrderType::Limit { tif: "Ioc".to_string() }
        );
    }

    #[test]
    fn maps_trigger_batch_item_to_modify_command() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "batchModify",
                    "modifies": [
                        {
                            "oid": "0x1234567890abcdef1234567890abcdef",
                            "order": {
                                "a": 10000,
                                "b": false,
                                "p": "1891.4",
                                "s": "0.02",
                                "r": false,
                                "t": {
                                    "trigger": {
                                        "isMarket": true,
                                        "triggerPx": "1900.5",
                                        "tpsl": "tp"
                                    }
                                },
                                "c": "0xfedcba0987654321fedcba0987654321"
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
            }"#,
        )
        .expect("request parses");

        let modify_request = from_wire_batch_modify("seller", &request.action.modifies[0])
            .expect("modify request should build");
        let command = SpotOrderV2ModifyExecutionSpec::command(&modify_request);

        assert_eq!(command.party_id, "seller");
        assert_eq!(command.cloid.as_deref(), Some("0xfedcba0987654321fedcba0987654321"));
        assert_eq!(
            command.order_type,
            ModifySpotOrderV2OrderType::Trigger {
                is_market: true,
                trigger_price: "19005".to_string(),
                trigger_role: "tp".to_string(),
            }
        );
    }

    #[test]
    fn missing_vault_address_uses_default_party_id() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request parses");
        let party_id =
            request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

        let modify_request = from_wire_batch_modify(&party_id, &request.action.modifies[0])
            .expect("modify request should build");

        assert_eq!(modify_request.party_id, DEFAULT_EXCHANGE_PARTY_ID);
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
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"order\",\n    \"data\": {\n      \"statuses\": [\n        {\n          \"error\": \"load_state failed: spot order v2 modify state is not wired for default HTTP path\"\n        },\n        {\n          \"error\": \"load_state failed: spot order v2 modify state is not wired for default HTTP path\"\n        }\n      ]\n    }\n  }\n}"
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
