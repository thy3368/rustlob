use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::{
    validate_cloid, validate_common_fields, validate_hex_address,
};
use crate::exchange::common::wire::CommonExchangeFields;
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum OrderContractError {
    #[error("Unexpected `action.type` for order handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`action.orders` must contain at least one order.")]
    EmptyOrders,
    #[error("Invalid `action.grouping`. Expected one of `na`, `normalTpsl`, `positionTpsl`.")]
    InvalidGrouping,
    #[error("Invalid `action.builder.b`. Expected a 42-character hexadecimal address.")]
    InvalidBuilderAddress,
    #[error("Invalid `action.orders[].c`. Expected a 128-bit hexadecimal cloid.")]
    InvalidCloid,
    #[error("Invalid `action.orders[].t`. Expected exactly one of `limit` or `trigger`.")]
    InvalidOrderType,
    #[error("Invalid `action.orders[].t.limit.tif`. Expected one of `Alo`, `Ioc`, `Gtc`.")]
    InvalidTimeInForce,
    #[error("Invalid `action.orders[].t.trigger.tpsl`. Expected `tp` or `sl`.")]
    InvalidTriggerKind,
    #[error("Invalid `action.orders[].p`. Expected a non-empty decimal string.")]
    InvalidPrice,
    #[error("Invalid `action.orders[].s`. Expected a non-empty decimal string.")]
    InvalidSize,
    #[error("Invalid `action.orders[].t.trigger.triggerPx`. Expected a non-empty decimal string.")]
    InvalidTriggerPrice,
}

pub mod reply {
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct OrderResponseWire {
        pub status: &'static str,
        pub response: OrderResponseEnvelopeWire,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct OrderResponseEnvelopeWire {
        #[serde(rename = "type")]
        pub type_: &'static str,
        pub data: OrderResponseDataWire,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct OrderResponseDataWire {
        pub statuses: Vec<OrderStatusWire>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum OrderStatusWire {
        Resting { resting: RestingOrderStatusWire },
        Filled { filled: FilledOrderStatusWire },
        Error { error: String },
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct RestingOrderStatusWire {
        pub oid: u64,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct FilledOrderStatusWire {
        #[serde(rename = "totalSz")]
        pub total_sz: String,
        #[serde(rename = "avgPx")]
        pub avg_px: String,
        pub oid: u64,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestWire {
    action: ActionWire,
    #[serde(flatten)]
    common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    orders: Vec<OrderWire>,
    grouping: String,
    builder: Option<BuilderWire>,
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
    limit: Option<LimitOrderTypeWire>,
    trigger: Option<TriggerOrderTypeWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LimitOrderTypeWire {
    tif: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TriggerOrderTypeWire {
    #[serde(rename = "isMarket")]
    is_market: bool,
    #[serde(rename = "triggerPx")]
    trigger_px: String,
    tpsl: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuilderWire {
    b: String,
    f: u64,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |request, deps| Box::pin(execute(request, deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "order" {
        return Err(OrderContractError::UnexpectedActionType(request.action.type_.clone()).into());
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
    if request.action.orders.is_empty() {
        return Err(OrderContractError::EmptyOrders.into());
    }
    if !matches!(request.action.grouping.as_str(), "na" | "normalTpsl" | "positionTpsl") {
        return Err(OrderContractError::InvalidGrouping.into());
    }
    if let Some(builder) = &request.action.builder {
        validate_hex_address(&builder.b).map_err(|_| {
            ExchangeHttpError::OrderContract(OrderContractError::InvalidBuilderAddress)
        })?;
    }
    for order in &request.action.orders {
        if order.p.trim().is_empty() {
            return Err(OrderContractError::InvalidPrice.into());
        }
        if order.s.trim().is_empty() {
            return Err(OrderContractError::InvalidSize.into());
        }
        if let Some(cloid) = &order.c {
            validate_cloid(cloid)
                .map_err(|_| ExchangeHttpError::OrderContract(OrderContractError::InvalidCloid))?;
        }
        match (&order.t.limit, &order.t.trigger) {
            (Some(limit), None) => {
                if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                    return Err(OrderContractError::InvalidTimeInForce.into());
                }
            }
            (None, Some(trigger)) => {
                if trigger.trigger_px.trim().is_empty() {
                    return Err(OrderContractError::InvalidTriggerPrice.into());
                }
                if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                    return Err(OrderContractError::InvalidTriggerKind.into());
                }
            }
            _ => return Err(OrderContractError::InvalidOrderType.into()),
        }
    }
    Ok(())
}

const STUB_FILLED_PREFIX: &str = "stub-filled";
const STUB_ERROR_PREFIX: &str = "stub-error";
const STUB_ERROR_MESSAGE: &str = "Order must have minimum value of $10.";
const STUB_OID_BASE: u64 = 77738308;

async fn execute(
    request: RequestWire,
    _deps: &ExchangeActionDeps,
) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    let statuses = request
        .action
        .orders
        .iter()
        .enumerate()
        .map(|(index, order)| {
            let oid = STUB_OID_BASE + index as u64;
            match order.c.as_deref() {
                Some(cloid) if cloid.starts_with(STUB_FILLED_PREFIX) => {
                    reply::OrderStatusWire::Filled {
                        filled: reply::FilledOrderStatusWire {
                            total_sz: order.s.clone(),
                            avg_px: order.p.clone(),
                            oid,
                        },
                    }
                }
                Some(cloid) if cloid.starts_with(STUB_ERROR_PREFIX) => {
                    reply::OrderStatusWire::Error { error: STUB_ERROR_MESSAGE.to_string() }
                }
                _ => reply::OrderStatusWire::Resting {
                    resting: reply::RestingOrderStatusWire { oid },
                },
            }
        })
        .collect();

    Ok(reply::OrderResponseWire {
        status: "ok",
        response: reply::OrderResponseEnvelopeWire {
            type_: "order",
            data: reply::OrderResponseDataWire { statuses },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchange::test_support::{
        valid_order_request_json, valid_request_with_grouping_json,
    };

    #[test]
    fn parses_minimal_order_request() {
        let request = parse(valid_order_request_json()).expect("request should parse");

        assert_eq!(request.action.type_, "order");
        assert_eq!(request.action.orders.len(), 1);
        assert_eq!(request.action.orders[0].p, "1891.4");
        assert_eq!(request.common.expires_after, None);
        assert_eq!(request.common.vault_address, None);
    }

    #[test]
    fn parses_order_request_with_optional_fields() {
        let request = parse(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000,
                        "b": false,
                        "p": "1800",
                        "s": "0.5",
                        "r": true,
                        "t": {
                            "trigger": {
                                "isMarket": true,
                                "triggerPx": "1790",
                                "tpsl": "sl"
                            }
                        },
                        "c": "0x1234567890abcdef1234567890abcdef"
                    }],
                    "grouping": "positionTpsl",
                    "builder": {
                        "b": "0x1111111111111111111111111111111111111111",
                        "f": 10
                    }
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 28
                },
                "vaultAddress": "0x2222222222222222222222222222222222222222",
                "expiresAfter": 1710000001000
            }"#,
        )
        .expect("request should parse");

        assert_eq!(request.action.grouping, "positionTpsl");
        assert!(request.action.builder.is_some());
        assert_eq!(
            request.action.orders[0].c.as_deref(),
            Some("0x1234567890abcdef1234567890abcdef")
        );
        assert_eq!(
            request.common.vault_address.as_deref(),
            Some("0x2222222222222222222222222222222222222222")
        );
        assert_eq!(request.common.expires_after, Some(1710000001000));
    }

    #[test]
    fn rejects_empty_orders() {
        let request = parse(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [],
                    "grouping": "na"
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
        assert!(matches!(error, ExchangeHttpError::OrderContract(OrderContractError::EmptyOrders)));
    }

    #[test]
    fn rejects_invalid_grouping() {
        let request = parse(&valid_request_with_grouping_json("unknown")).expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert!(matches!(
            error,
            ExchangeHttpError::OrderContract(OrderContractError::InvalidGrouping)
        ));
    }

    #[test]
    fn rejects_invalid_signature_shape() {
        let request = parse(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000,
                        "b": true,
                        "p": "1891.4",
                        "s": "0.02",
                        "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }],
                    "grouping": "na"
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1234",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(
            error.to_string(),
            "Invalid `signature` shape. Expected hex `r`/`s` and numeric `v`."
        );
    }

    #[test]
    fn rejects_invalid_order_type_shape() {
        let request = parse(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000,
                        "b": true,
                        "p": "1891.4",
                        "s": "0.02",
                        "r": false,
                        "t": {
                            "limit": { "tif": "Gtc" },
                            "trigger": {
                                "isMarket": false,
                                "triggerPx": "1900",
                                "tpsl": "tp"
                            }
                        }
                    }],
                    "grouping": "na"
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
        assert!(matches!(
            error,
            ExchangeHttpError::OrderContract(OrderContractError::InvalidOrderType)
        ));
    }

    #[actix_web::test]
    async fn success_json_snapshot_is_stable() {
        let request = parse(valid_order_request_json()).expect("request parses");
        let response =
            execute(request, &ExchangeActionDeps::default()).await.expect("response builds");

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "order",
    "data": {
      "statuses": [
        {
          "resting": {
            "oid": 77738308
          }
        }
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn resting_filled_and_error_shapes_are_stable() {
        let response = reply::OrderResponseWire {
            status: "ok",
            response: reply::OrderResponseEnvelopeWire {
                type_: "order",
                data: reply::OrderResponseDataWire {
                    statuses: vec![
                        reply::OrderStatusWire::Resting {
                            resting: reply::RestingOrderStatusWire { oid: 1 },
                        },
                        reply::OrderStatusWire::Filled {
                            filled: reply::FilledOrderStatusWire {
                                total_sz: "0.02".to_string(),
                                avg_px: "1891.4".to_string(),
                                oid: 2,
                            },
                        },
                        reply::OrderStatusWire::Error { error: STUB_ERROR_MESSAGE.to_string() },
                    ],
                },
            },
        };

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "order",
    "data": {
      "statuses": [
        {
          "resting": {
            "oid": 1
          }
        },
        {
          "filled": {
            "totalSz": "0.02",
            "avgPx": "1891.4",
            "oid": 2
          }
        },
        {
          "error": "Order must have minimum value of $10."
        }
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn request_parse_snapshot_is_stable() {
        let request = parse(valid_order_request_json()).expect("request parses");
        let actual = serde_json::to_string_pretty(&request).expect("request serializes");
        let expected = r#"{
  "action": {
    "type": "order",
    "orders": [
      {
        "a": 10000,
        "b": true,
        "p": "1891.4",
        "s": "0.02",
        "r": false,
        "t": {
          "limit": {
            "tif": "Gtc"
          },
          "trigger": null
        },
        "c": null
      }
    ],
    "grouping": "na",
    "builder": null
  },
  "nonce": 1710000000000,
  "signature": {
    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
    "v": 27
  }
}"#;

        assert_eq!(actual, expected);
    }
}
