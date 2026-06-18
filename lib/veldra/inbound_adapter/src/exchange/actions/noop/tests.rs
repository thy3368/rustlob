use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::noop::{parse, validate};

#[test]
fn parses_noop_request() {
    let request = parse(valid_noop_request_json()).expect("noop request should parse");
    assert_eq!(request.action.type_, "noop");
}

#[test]
fn noop_reply_snapshot_is_stable() {
    let response =
        super::service::execute(&ExchangeActionDeps::default()).expect("noop reply builds");
    let actual = serde_json::to_string_pretty(&response).expect("noop response serializes");
    assert_eq!(
        actual,
        "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
    );
}

#[test]
fn validates_noop_request() {
    let request = parse(valid_noop_request_json()).expect("noop request should parse");
    validate(&request).expect("noop validation should pass");
}

fn valid_noop_request_json() -> &'static [u8] {
    br#"{
        "action": { "type": "noop" },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
