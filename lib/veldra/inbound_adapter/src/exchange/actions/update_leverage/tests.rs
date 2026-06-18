use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::update_leverage::{parse, validate};

#[test]
fn parses_update_leverage_request() {
    let request = parse(valid_update_leverage_json()).expect("request should parse");
    assert_eq!(request.action.type_, "updateLeverage");
    assert_eq!(request.action.leverage, 5);
}

#[test]
fn rejects_zero_leverage() {
    let request = parse(
        br#"{
            "action": {
                "type": "updateLeverage",
                "asset": 7,
                "isCross": true,
                "leverage": 0
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
        "Invalid `action.leverage`. Expected an integer greater than or equal to 1."
    );
}

#[test]
fn update_leverage_reply_snapshot_is_stable() {
    let response =
        super::service::execute(&ExchangeActionDeps::default()).expect("response should build");
    let actual = serde_json::to_string_pretty(&response).expect("response serializes");
    assert_eq!(
        actual,
        "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
    );
}

fn valid_update_leverage_json() -> &'static [u8] {
    br#"{
        "action": {
            "type": "updateLeverage",
            "asset": 7,
            "isCross": true,
            "leverage": 5
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
