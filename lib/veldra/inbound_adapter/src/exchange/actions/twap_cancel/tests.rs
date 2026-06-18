use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::twap_cancel::reply::{
    TwapCancelResponseDataWire, TwapCancelResponseEnvelopeWire, TwapCancelResponseWire,
    TwapCancelStatusWire,
};
use crate::exchange::actions::twap_cancel::{parse, validate};

#[test]
fn parses_twap_cancel_request() {
    let request = parse(valid_request_json()).expect("request should parse");
    assert_eq!(request.action.type_, "twapCancel");
    assert_eq!(request.action.t, 77738308);
}

#[test]
fn rejects_zero_twap_id() {
    let request = parse(
        br#"{
            "action": { "type": "twapCancel", "a": 7, "t": 0 },
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
    assert_eq!(error.to_string(), "Invalid `action.t`. Expected a positive twap id.");
}

#[test]
fn twap_cancel_reply_snapshot_is_stable() {
    let response =
        super::service::execute(&ExchangeActionDeps::default()).expect("response should build");
    let actual = serde_json::to_string_pretty(&response).expect("response serializes");
    let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapCancel",
    "data": {
      "status": "success"
    }
  }
}"#;

    assert_eq!(actual, expected);
}

#[test]
fn twap_cancel_error_snapshot_is_stable() {
    let response = TwapCancelResponseWire {
        status: "ok",
        response: TwapCancelResponseEnvelopeWire {
            type_: "twapCancel",
            data: TwapCancelResponseDataWire {
                status: TwapCancelStatusWire::Error {
                    error: "TWAP was never placed, already canceled, or filled.".to_string(),
                },
            },
        },
    };

    let actual = serde_json::to_string_pretty(&response).expect("response serializes");
    let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapCancel",
    "data": {
      "status": {
        "error": "TWAP was never placed, already canceled, or filled."
      }
    }
  }
}"#;

    assert_eq!(actual, expected);
}

fn valid_request_json() -> &'static [u8] {
    br#"{
        "action": { "type": "twapCancel", "a": 7, "t": 77738308 },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
