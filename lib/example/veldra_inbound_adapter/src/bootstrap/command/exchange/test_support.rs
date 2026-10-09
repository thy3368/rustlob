use serde_json::{Value, json};

pub fn valid_signature_value() -> Value {
    json!({
        "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
        "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
        "v": 27
    })
}

pub fn valid_order_request_value() -> Value {
    json!({
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
        "nonce": 1710000000000u64,
        "signature": valid_signature_value()
    })
}

pub fn valid_order_request_json() -> &'static [u8] {
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
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}

pub fn valid_request_with_grouping_json(grouping: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({
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
            "grouping": grouping
        },
        "nonce": 1710000000000u64,
        "signature": valid_signature_value()
    }))
    .expect("fixture serializes")
}
