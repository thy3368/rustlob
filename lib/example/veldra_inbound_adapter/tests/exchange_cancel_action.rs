use example_veldra_inbound_adapter::command::exchange::actions::cancel::CancelAction;
use example_veldra_inbound_adapter::command::exchange::actions::cancel::reply::CancelStatusWire;
use example_veldra_inbound_adapter::command::exchange::error::ExchangeHttpError;
use rstest::rstest;

#[actix_web::test]
async fn cancel_action_run_json_returns_default_outbound_error_status() {
    let response =
        CancelAction::run_json(valid_cancel_request_json()).await.expect("cancel action runs");

    assert_eq!(response.response.type_, "cancel");
    assert_eq!(
        response.response.data.statuses,
        vec![CancelStatusWire::Error {
            error:
                "load_state failed: spot order v2 cancel state is not wired for default HTTP path"
                    .to_string(),
        }]
    );
}

#[rstest]
#[case::empty_cancels(
    empty_cancels_request_json(),
    "`action.cancels` must contain at least one cancel request."
)]
#[case::invalid_order_id(
    invalid_order_id_cancel_request_json(),
    "Invalid `action.cancels[].o`. Expected a positive numeric oid."
)]
#[case::false_fast_flag(
    false_fast_flag_cancel_request_json(),
    "Invalid `action.f`. Omit `f` unless fast cancel is enabled."
)]
#[actix_web::test]
async fn cancel_action_run_json_rejects_contract_errors(
    #[case] body: &'static [u8],
    #[case] expected_error: &'static str,
) {
    let error = CancelAction::run_json(body).await.expect_err("cancel request should fail");

    assert!(matches!(error, ExchangeHttpError::ActionContract(_)));
    assert_eq!(error.to_string(), expected_error);
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

fn empty_cancels_request_json() -> &'static [u8] {
    br#"{
        "action": {
            "type": "cancel",
            "cancels": []
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}

fn invalid_order_id_cancel_request_json() -> &'static [u8] {
    br#"{
        "action": {
            "type": "cancel",
            "cancels": [{ "a": 10000, "o": 0 }]
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}

fn false_fast_flag_cancel_request_json() -> &'static [u8] {
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
    }"#
}
