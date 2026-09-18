use cmd_handler::command_use_case_def2::{MiFamilyExecutionError, MiFamilyExecutionSpec};
use example_core_use_case::{
    ModifySpotOrderV2Cmd, ModifySpotOrderV2OrderType, ModifySpotOrderV2UseCase, OrderId,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use use_case_executor::trading::spot::modify_spot_order_v2_executor::execute_modify_spot_order_v2;

use crate::command::exchange::actions::cancel::DEFAULT_EXCHANGE_PARTY_ID;
use crate::command::exchange::actions::order::reply::{OrderStatusWire, RestingOrderStatusWire};
use crate::command::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::command::exchange::common::validate::{validate_cloid, validate_envelope_common};
use crate::command::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_statuses_response};
use crate::command::exchange::error::ExchangeHttpError;
#[cfg(test)]
use crate::common::parse::parse_json_request;

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
    pub use crate::command::exchange::actions::order::reply::OrderResponseWire as ModifyResponseWire;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV2Request {
    pub party_id: String,
    pub asset: u32,
    pub order_id: OrderId,
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub order_type: ModifySpotOrderV2OrderType,
    pub cloid: Option<String>,
}

impl ModifySpotOrderV2Request {
    fn from_wire_modify(
        party_id: String,
        action: &ActionWire,
    ) -> Result<Self, ModifyContractError> {
        Ok(Self {
            party_id,
            asset: action.order.a,
            order_id: order_id_from_wire_oid(&action.oid)?,
            is_buy: action.order.b,
            price: decimal_wire_to_core_units(&action.order.p),
            size: decimal_wire_to_core_units(&action.order.s),
            order_type: modify_order_type_from_wire(&action.order.t)?,
            cloid: action.order.c.clone(),
        })
    }
}

pub struct SpotOrderV2ModifyExecutionSpec;

impl MiFamilyExecutionSpec<ModifySpotOrderV2UseCase> for SpotOrderV2ModifyExecutionSpec {
    type Request = ModifySpotOrderV2Request;

    fn command(request: &Self::Request) -> ModifySpotOrderV2Cmd {
        ModifySpotOrderV2Cmd {
            party_id: request.party_id.clone(),
            asset: request.asset,
            order_id: request.order_id.clone(),
            is_buy: request.is_buy,
            price: request.price.clone(),
            size: request.size.clone(),
            order_type: request.order_type.clone(),
            cloid: request.cloid.clone(),
        }
    }
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
        return Err(ExchangeHttpError::contract(ModifyContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    validate_oid(&request.action.oid)?;
    if matches!(request.action.always_place, Some(false)) {
        return Err(ExchangeHttpError::contract(ModifyContractError::InvalidAlwaysPlaceFlag));
    }
    validate_order(&request.action.order)?;
    Ok(())
}

fn validate_oid(oid: &Value) -> Result<(), ExchangeHttpError> {
    if oid.as_u64().is_some_and(|value| value > 0) {
        return Ok(());
    }
    if let Some(cloid) = oid.as_str() {
        validate_cloid(cloid)
            .map_err(|_| ExchangeHttpError::contract(ModifyContractError::InvalidOid))?;
        return Ok(());
    }
    Err(ExchangeHttpError::contract(ModifyContractError::InvalidOid))
}

fn validate_order(order: &OrderWire) -> Result<(), ExchangeHttpError> {
    if order.builder.is_some() {
        return Err(ExchangeHttpError::contract(ModifyContractError::BuilderNotSupported));
    }
    if order.p.trim().is_empty() {
        return Err(ExchangeHttpError::contract(ModifyContractError::InvalidPrice));
    }
    if order.s.trim().is_empty() {
        return Err(ExchangeHttpError::contract(ModifyContractError::InvalidSize));
    }
    if let Some(cloid) = &order.c {
        validate_cloid(cloid)
            .map_err(|_| ExchangeHttpError::contract(ModifyContractError::InvalidOrderCloid))?;
    }
    match (&order.t.limit, &order.t.trigger) {
        (Some(limit), None) => {
            if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                return Err(ExchangeHttpError::contract(ModifyContractError::InvalidTimeInForce));
            }
        }
        (None, Some(trigger)) => {
            if trigger.trigger_px.trim().is_empty() {
                return Err(ExchangeHttpError::contract(ModifyContractError::InvalidTriggerPrice));
            }
            if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                return Err(ExchangeHttpError::contract(ModifyContractError::InvalidTriggerKind));
            }
        }
        _ => return Err(ExchangeHttpError::contract(ModifyContractError::InvalidOrderType)),
    }
    Ok(())
}

fn order_id_from_wire_oid(oid: &Value) -> Result<OrderId, ModifyContractError> {
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
    Err(ModifyContractError::InvalidOid)
}

fn modify_order_type_from_wire(
    order_type: &OrderTypeWire,
) -> Result<ModifySpotOrderV2OrderType, ModifyContractError> {
    match (&order_type.limit, &order_type.trigger) {
        (Some(limit), None) => Ok(ModifySpotOrderV2OrderType::Limit { tif: limit.tif.clone() }),
        (None, Some(trigger)) => Ok(ModifySpotOrderV2OrderType::Trigger {
            is_market: trigger.is_market,
            trigger_price: decimal_wire_to_core_units(&trigger.trigger_px),
            trigger_role: trigger.tpsl.clone(),
        }),
        _ => Err(ModifyContractError::InvalidOrderType),
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

async fn execute(request: RequestWire) -> Result<reply::ModifyResponseWire, ExchangeHttpError> {
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());
    let modify_request = ModifySpotOrderV2Request::from_wire_modify(party_id, &request.action)
        .map_err(ExchangeHttpError::contract)?;
    let command = SpotOrderV2ModifyExecutionSpec::command(&modify_request);
    let status = match execute_modify_spot_order_v2(&command) {
        Ok(result) => OrderStatusWire::Resting {
            resting: RestingOrderStatusWire {
                oid: result.changes.updated_order.after.exchange_oid().unwrap_or(0),
            },
        },
        Err(error) => OrderStatusWire::Error { error: modify_execution_error_message(error) },
    };

    Ok(ok_statuses_response("order", vec![status]))
}

fn modify_execution_error_message<BE, OE>(error: MiFamilyExecutionError<BE, OE>) -> String
where
    BE: std::fmt::Display,
    OE: std::fmt::Display,
{
    match error {
        MiFamilyExecutionError::Business(error) => error.to_string(),
        MiFamilyExecutionError::ProjectEvents(error) => {
            format!("project replayable events failed: {error}")
        }
        MiFamilyExecutionError::LoadState(error) => format!("load_state failed: {error}"),
        MiFamilyExecutionError::Persist(error) => format!("persist failed: {error}"),
        MiFamilyExecutionError::Replay(error) => format!("replay failed: {error}"),
        MiFamilyExecutionError::Publish(error) => format!("publish failed: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.order.p, "1891.4");
    }

    #[test]
    fn rejects_false_always_place() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
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

    #[test]
    fn maps_oid_request_to_modify_command() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request parses");
        let modify_request =
            ModifySpotOrderV2Request::from_wire_modify("buyer".to_string(), &request.action)
                .expect("modify request should build");

        let command = SpotOrderV2ModifyExecutionSpec::command(&modify_request);

        assert_eq!(command.party_id, "buyer");
        assert_eq!(command.asset, 10_000);
        assert_eq!(command.order_id, OrderId::Oid(77738308));
        assert!(command.is_buy);
        assert_eq!(command.price, "18914");
        assert_eq!(command.size, "002");
        assert_eq!(
            command.order_type,
            ModifySpotOrderV2OrderType::Limit { tif: "Gtc".to_string() }
        );
    }

    #[test]
    fn maps_cloid_lookup_request_to_modify_command() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "modify",
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
        let modify_request =
            ModifySpotOrderV2Request::from_wire_modify("seller".to_string(), &request.action)
                .expect("modify request should build");

        let command = SpotOrderV2ModifyExecutionSpec::command(&modify_request);

        assert_eq!(
            command.order_id,
            OrderId::Cloid("0x1234567890abcdef1234567890abcdef".to_string())
        );
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
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"order\",\n    \"data\": {\n      \"statuses\": [\n        {\n          \"error\": \"load_state failed: spot order v2 modify state is not wired for default HTTP path\"\n        }\n      ]\n    }\n  }\n}"
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
