use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::user_set_abstraction::{parse, validate};

#[test]
fn parses_user_set_abstraction_request() {
    let request = parse(valid_request_json()).expect("request should parse");
    assert_eq!(request.action.type_, "userSetAbstraction");
    assert_eq!(request.action.abstraction, "unifiedAccount");
}

#[test]
fn rejects_invalid_abstraction() {
    let request = parse(
        br#"{
            "action": {
                "type": "userSetAbstraction",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "user": "0x4444444444444444444444444444444444444444",
                "abstraction": "invalid",
                "nonce": 1710000000000
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
        "Invalid `action.abstraction`. Expected one of `disabled`, `unifiedAccount`, `portfolioMargin`."
    );
}

#[test]
fn user_set_abstraction_reply_snapshot_is_stable() {
    let response =
        super::service::execute(&ExchangeActionDeps::default()).expect("response should build");
    let actual = serde_json::to_string_pretty(&response).expect("response serializes");
    assert_eq!(
        actual,
        "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
    );
}

fn valid_request_json() -> &'static [u8] {
    br#"{
        "action": {
            "type": "userSetAbstraction",
            "hyperliquidChain": "Mainnet",
            "signatureChainId": "0xa4b1",
            "user": "0x4444444444444444444444444444444444444444",
            "abstraction": "unifiedAccount",
            "nonce": 1710000000000
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
