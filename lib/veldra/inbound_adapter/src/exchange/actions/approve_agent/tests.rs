use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::approve_agent::{parse, validate};

#[test]
fn parses_approve_agent_request() {
    let request = parse(valid_request_json()).expect("request should parse");
    assert_eq!(request.action.type_, "approveAgent");
    assert_eq!(request.action.agent_name.as_deref(), Some("desk-bot"));
}

#[test]
fn rejects_nonce_mismatch() {
    let request = parse(
        br#"{
            "action": {
                "type": "approveAgent",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0xa4b1",
                "agentAddress": "0x3333333333333333333333333333333333333333",
                "nonce": 1710000001000
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
        "Invalid `action.nonce`. Expected it to match the outer `nonce`."
    );
}

#[test]
fn approve_agent_reply_snapshot_is_stable() {
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
            "type": "approveAgent",
            "hyperliquidChain": "Mainnet",
            "signatureChainId": "0xa4b1",
            "agentAddress": "0x3333333333333333333333333333333333333333",
            "agentName": "desk-bot",
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
