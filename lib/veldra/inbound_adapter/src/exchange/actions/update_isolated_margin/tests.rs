use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::update_isolated_margin::parse;

#[test]
fn parses_update_isolated_margin_request() {
    let request = parse(valid_request_json()).expect("request should parse");
    assert_eq!(request.action.type_, "updateIsolatedMargin");
    assert_eq!(request.action.ntli, 1_000_000);
}

#[test]
fn update_isolated_margin_reply_snapshot_is_stable() {
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
            "type": "updateIsolatedMargin",
            "asset": 7,
            "isBuy": true,
            "ntli": 1000000
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
