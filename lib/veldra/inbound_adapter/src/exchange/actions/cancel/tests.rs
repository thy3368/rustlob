use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::cancel::reply::{
    CancelResponseDataWire, CancelResponseEnvelopeWire, CancelResponseWire, CancelStatusWire,
};
use crate::exchange::actions::cancel::{parse, validate};
#[test]
fn parses_cancel_request() {
    let request = parse(valid_cancel_request_json()).expect("cancel request should parse");

    assert_eq!(request.action.type_, "cancel");
    assert_eq!(request.action.cancels.len(), 1);
    assert_eq!(request.action.cancels[0].o, 77738308);
}

#[test]
fn rejects_false_fast_flag() {
    let request = parse(
        br#"{
            "action": {
                "type": "cancel",
                "cancels": [{ "a": 10000, "o": 77738308 }],
                "f": false
            },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#,
    )
    .expect("cancel request parses");

    let error = validate(&request).expect_err("validation should fail");
    assert_eq!(error.to_string(), "Invalid `action.f`. Omit `f` unless fast cancel is enabled.");
}

#[test]
fn cancel_reply_snapshot_is_stable() {
    let request = parse(valid_cancel_request_json()).expect("cancel request parses");
    let response = super::service::execute(request, &ExchangeActionDeps::default())
        .expect("cancel response builds");

    let actual = serde_json::to_string_pretty(&response).expect("cancel response serializes");
    let expected = r#"{
  "status": "ok",
  "response": {
    "type": "cancel",
    "data": {
      "statuses": [
        "success"
      ]
    }
  }
}"#;

    assert_eq!(actual, expected);
}

#[test]
fn cancel_error_shape_snapshot_is_stable() {
    let response = CancelResponseWire {
        status: "ok",
        response: CancelResponseEnvelopeWire {
            type_: "cancel",
            data: CancelResponseDataWire {
                statuses: vec![CancelStatusWire::Error {
                    error: "Order was never placed, already canceled, or filled.".to_string(),
                }],
            },
        },
    };

    let actual = serde_json::to_string_pretty(&response).expect("cancel response serializes");
    let expected = r#"{
  "status": "ok",
  "response": {
    "type": "cancel",
    "data": {
      "statuses": [
        {
          "error": "Order was never placed, already canceled, or filled."
        }
      ]
    }
  }
}"#;

    assert_eq!(actual, expected);
}

fn valid_cancel_request_json() -> &'static [u8] {
    br#"{
        "action": {
            "type": "cancel",
            "cancels": [{ "a": 10000, "o": 77738308 }]
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
