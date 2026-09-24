use actix_web::http::StatusCode;
use actix_web::{App, test as actix_test};
use example_veldra_inbound_adapter::command::exchange::CancelStatusWire;
use example_veldra_inbound_adapter::command::exchange::http::build_exchange_scope;
use rstest::rstest;
use serde_json::{Value, json};

#[rstest]
#[case::valid_cancel(
    valid_cancel_request_value(),
    StatusCode::OK,
    json!({
        "status": "ok",
        "response": {
            "type": "cancel",
            "data": {
                "statuses": [
                    {
                        "error": "load_state failed: spot order v2 cancel state is not wired for default HTTP path"
                    }
                ]
            }
        }
    })
)]
#[case::empty_cancels(
    empty_cancels_request_value(),
    StatusCode::BAD_REQUEST,
    json!({
        "status": "err",
        "error": "`action.cancels` must contain at least one cancel request."
    })
)]
#[case::invalid_order_id(
    invalid_order_id_cancel_request_value(),
    StatusCode::BAD_REQUEST,
    json!({
        "status": "err",
        "error": "Invalid `action.cancels[].o`. Expected a positive numeric oid."
    })
)]
#[case::false_fast_flag(
    false_fast_flag_cancel_request_value(),
    StatusCode::BAD_REQUEST,
    json!({
        "status": "err",
        "error": "Invalid `action.f`. Omit `f` unless fast cancel is enabled."
    })
)]
#[actix_web::test]
async fn cancel_http_contract_matches_public_wire_shape(
    #[case] payload: Value,
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) {
    let (status, body) = post_exchange_status_and_json(payload).await;

    assert_eq!(status, expected_status);
    assert_eq!(body, expected_body);
}

#[actix_web::test]
async fn cancel_http_contract_returns_status_for_each_cancel_in_request_order() {
    let (status, body) = post_exchange_status_and_json(multiple_cancels_request_value()).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!({
            "status": "ok",
            "response": {
                "type": "cancel",
                "data": {
                    "statuses": [
                        { "error": default_cancel_load_state_error() },
                        { "error": default_cancel_load_state_error() }
                    ]
                }
            }
        })
    );
}

#[rstest]
fn cancel_error_status_wire_serializes_as_error_object() {
    let status = CancelStatusWire::Error {
        error: "Order was never placed, already canceled, or filled.".to_string(),
    };

    let actual = serde_json::to_value(status).expect("cancel status serializes");

    assert_eq!(actual, json!({ "error": "Order was never placed, already canceled, or filled." }));
}

async fn post_exchange_status_and_json(payload: Value) -> (StatusCode, Value) {
    let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
    let request = actix_test::TestRequest::post().uri("/exchange").set_json(payload).to_request();
    let response = actix_test::call_service(&app, request).await;
    let status = response.status();
    let body = actix_test::read_body_json(response).await;
    (status, body)
}

fn default_cancel_load_state_error() -> &'static str {
    "load_state failed: spot order v2 cancel state is not wired for default HTTP path"
}

fn valid_cancel_request_value() -> Value {
    json!({
        "action": {
            "type": "cancel",
            "cancels": [{ "a": 10000, "o": 77738308 }]
        },
        "nonce": 1710000000000u64,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    })
}

fn empty_cancels_request_value() -> Value {
    json!({
        "action": {
            "type": "cancel",
            "cancels": []
        },
        "nonce": 1710000000000u64,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    })
}

fn invalid_order_id_cancel_request_value() -> Value {
    json!({
        "action": {
            "type": "cancel",
            "cancels": [{ "a": 10000, "o": 0 }]
        },
        "nonce": 1710000000000u64,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    })
}

fn false_fast_flag_cancel_request_value() -> Value {
    json!({
        "action": {
            "type": "cancel",
            "cancels": [{ "a": 10000, "o": 77738308 }],
            "f": false
        },
        "nonce": 1710000000000u64,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    })
}

fn multiple_cancels_request_value() -> Value {
    json!({
        "action": {
            "type": "cancel",
            "cancels": [
                { "a": 10000, "o": 77738308 },
                { "a": 10001, "o": 77738309 }
            ]
        },
        "nonce": 1710000000000u64,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    })
}
