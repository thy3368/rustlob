use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::order::error::OrderContractError;
use crate::exchange::actions::order::reply::{
    FilledOrderStatusWire, OrderResponseDataWire, OrderResponseEnvelopeWire, OrderResponseWire,
    OrderStatusWire, RestingOrderStatusWire,
};
use crate::exchange::actions::order::service::STUB_ERROR_MESSAGE;
use crate::exchange::actions::order::{parse, validate};
use crate::exchange::error::ExchangeHttpError;
use crate::exchange::test_support::{valid_order_request_json, valid_request_with_grouping_json};

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
    assert_eq!(request.action.orders[0].c.as_deref(), Some("0x1234567890abcdef1234567890abcdef"));
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
    assert!(matches!(error, ExchangeHttpError::OrderContract(OrderContractError::InvalidGrouping)));
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

#[test]
fn success_json_snapshot_is_stable() {
    let request = parse(valid_order_request_json()).expect("request parses");
    let response =
        super::service::execute(request, &ExchangeActionDeps::default()).expect("response builds");

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
    let response = OrderResponseWire {
        status: "ok",
        response: OrderResponseEnvelopeWire {
            type_: "order",
            data: OrderResponseDataWire {
                statuses: vec![
                    OrderStatusWire::Resting { resting: RestingOrderStatusWire { oid: 1 } },
                    OrderStatusWire::Filled {
                        filled: FilledOrderStatusWire {
                            total_sz: "0.02".to_string(),
                            avg_px: "1891.4".to_string(),
                            oid: 2,
                        },
                    },
                    OrderStatusWire::Error { error: STUB_ERROR_MESSAGE.to_string() },
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
  },
  "vaultAddress": null,
  "expiresAfter": null
}"#;

    assert_eq!(actual, expected);
}
