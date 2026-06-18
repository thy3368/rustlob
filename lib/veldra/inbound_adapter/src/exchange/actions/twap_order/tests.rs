use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::twap_order::reply::{
    TwapOrderResponseDataWire, TwapOrderResponseEnvelopeWire, TwapOrderResponseWire,
    TwapOrderStatusWire,
};
use crate::exchange::actions::twap_order::{parse, validate};

#[test]
fn parses_twap_order_request() {
    let request = parse(valid_request_json()).expect("request should parse");
    assert_eq!(request.action.type_, "twapOrder");
    assert_eq!(request.action.twap.m, 15);
}

#[test]
fn rejects_zero_duration_minutes() {
    let request = parse(
        br#"{
            "action": {
                "type": "twapOrder",
                "twap": { "a": 7, "b": true, "s": "1.25", "r": false, "m": 0, "t": true }
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
    assert_eq!(
        error.to_string(),
        "Invalid `action.twap.m`. Expected a duration greater than zero minutes."
    );
}

#[test]
fn twap_order_reply_snapshot_is_stable() {
    let response =
        super::service::execute(&ExchangeActionDeps::default()).expect("response should build");
    let actual = serde_json::to_string_pretty(&response).expect("response serializes");
    let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapOrder",
    "data": {
      "status": {
        "running": {
          "twapId": 77738308
        }
      }
    }
  }
}"#;

    assert_eq!(actual, expected);
}

#[test]
fn twap_order_error_snapshot_is_stable() {
    let response = TwapOrderResponseWire {
        status: "ok",
        response: TwapOrderResponseEnvelopeWire {
            type_: "twapOrder",
            data: TwapOrderResponseDataWire {
                status: TwapOrderStatusWire::Error {
                    error: "Invalid TWAP duration: 1 min(s)".to_string(),
                },
            },
        },
    };

    let actual = serde_json::to_string_pretty(&response).expect("response serializes");
    let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapOrder",
    "data": {
      "status": {
        "error": "Invalid TWAP duration: 1 min(s)"
      }
    }
  }
}"#;

    assert_eq!(actual, expected);
}

fn valid_request_json() -> &'static [u8] {
    br#"{
        "action": {
            "type": "twapOrder",
            "twap": { "a": 7, "b": true, "s": "1.25", "r": false, "m": 15, "t": true }
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
