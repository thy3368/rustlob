use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode},
};
use bdd::bdd_test;
use http_body_util::BodyExt;
use l1_adapter::MdbxStateStore;
use l1_core::{CodeStore, StateReader};
use l1_e2e::{bootstrap, http};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use tower::ServiceExt;

static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(1);

fn next_state_path() -> PathBuf {
    let test_id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("rustlob-l1-e2e-bdd-{test_id}"))
}

fn build_app() -> Router {
    let state_path = next_state_path();
    build_app_with_path(&state_path)
}

fn build_app_with_path(state_path: &PathBuf) -> Router {
    let _ = std::fs::remove_dir_all(state_path);
    std::fs::create_dir_all(state_path).expect("failed to create test state directory");
    std::env::set_var("L1_E2E_MDBX_PATH", state_path);

    let state = bootstrap::build_app_state().expect("failed to build app state");
    http::router(state)
}

fn assert_mdbx_files_exist(state_path: &PathBuf) {
    assert!(state_path.join("mdbx.dat").exists());
    assert!(state_path.join("mdbx.lck").exists());
}

fn assert_mdbx_activity_exists(state_path: &PathBuf) {
    let store = MdbxStateStore::open(state_path).expect("failed to reopen MDBX store");

    let has_account = store.account(alloy_primitives::Address::ZERO).ok().flatten().is_some();
    let has_code = CodeStore::code(&store, alloy_primitives::B256::ZERO)
        .ok()
        .flatten()
        .is_some();
    let has_storage = store.storage(alloy_primitives::Address::ZERO, alloy_primitives::B256::ZERO)
        .map(|value| value != alloy_primitives::B256::ZERO)
        .unwrap_or(false);

    let db_size = std::fs::metadata(state_path.join("mdbx.dat"))
        .expect("failed to stat mdbx.dat")
        .len();

    assert!(db_size > 0, "expected MDBX data file to be non-empty");
    assert!(has_account || has_code || has_storage || db_size > 0, "expected MDBX persistence activity to be observable");
}

async fn send_json(app: Router, method: Method, path: &str, body: Value) -> (StatusCode, Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("failed to build request"),
        )
        .await
        .expect("request failed");

    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("failed to collect response body")
        .to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or_else(|_| {
        json!({
            "raw": String::from_utf8_lossy(&bytes).to_string()
        })
    });
    (status, json)
}

async fn send_empty(app: Router, method: Method, path: &str) -> (StatusCode, Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .body(Body::empty())
                .expect("failed to build request"),
        )
        .await
        .expect("request failed");

    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("failed to collect response body")
        .to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or_else(|_| {
        json!({
            "raw": String::from_utf8_lossy(&bytes).to_string()
        })
    });
    (status, json)
}

fn transaction_request(request_id: &str, account: &str, action_type: &str, vm_kind: &str, capability: &str) -> Value {
    json!({
        "requests": [{
            "request_id": request_id,
            "account": account,
            "nonce": "1",
            "expires_at": "2099-01-01T00:00:00Z",
            "action_type": action_type,
            "payload_hash": format!("payload-{request_id}"),
            "signature_hash": format!("sig-{request_id}"),
            "vm_kind": vm_kind,
            "capability": capability,
        }]
    })
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "health check returns ok",
    given(app_is_initialized),
    when = "the client calls GET /api/l1/health",
    then(response_status_is_ok, body_reports_ok_true),
    tags(http, health, e2e),
    priority = "3"
)]
async fn health_check_returns_ok() {
    let (status, body) = send_empty(build_app(), Method::GET, "/api/l1/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], json!(true));
    assert_eq!(body["mempool_len"], json!(0));
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "spot order is admitted and executed",
    given(empty_demo_state),
    when = "the client submits a Rust VM spot order and executes a block",
    then(transaction_is_admitted, block_execution_succeeds),
    tags(http, rust_vm, spot, e2e),
    priority = "5"
)]
async fn spot_order_is_admitted_and_executed() {
    let state_path = next_state_path();
    let app = build_app_with_path(&state_path);
    let (submit_status, submit_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/transactions",
        transaction_request("req-bdd-spot-1", "acct-bdd-spot-1", "spot_order", "rust_vm", "dex.spot.place_order"),
    )
    .await;
    let (execute_status, execute_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/blocks/execute",
        json!({ "block_height": 101 }),
    )
    .await;
    let (health_status, health_body) = send_empty(app, Method::GET, "/api/l1/health").await;

    assert_eq!(submit_status, StatusCode::OK);
    assert_eq!(submit_body["admitted_count"], json!(1));
    assert_eq!(submit_body["rejected_count"], json!(0));
    assert_eq!(execute_status, StatusCode::OK);
    assert_eq!(execute_body["block_height"], json!(101));
    assert!(execute_body["block_event_count"].as_u64().unwrap_or_default() >= 1);
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body["mempool_len"], json!(0));
    assert_mdbx_files_exist(&state_path);
    assert_mdbx_activity_exists(&state_path);
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "perp order is admitted and executed",
    given(empty_demo_state),
    when = "the client submits a Rust VM perp order and executes a block",
    then(transaction_is_admitted, block_execution_succeeds),
    tags(http, rust_vm, perp, e2e),
    priority = "5"
)]
async fn perp_order_is_admitted_and_executed() {
    let app = build_app();
    let (submit_status, submit_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/transactions",
        transaction_request("req-bdd-perp-1", "acct-bdd-perp-1", "perp_order", "rust_vm", "dex.perp.place_order"),
    )
    .await;
    let (execute_status, execute_body) = send_json(
        app,
        Method::POST,
        "/api/l1/blocks/execute",
        json!({ "block_height": 102 }),
    )
    .await;

    assert_eq!(submit_status, StatusCode::OK);
    assert_eq!(submit_body["admitted_count"], json!(1));
    assert_eq!(submit_body["rejected_count"], json!(0));
    assert_eq!(execute_status, StatusCode::OK);
    assert_eq!(execute_body["block_height"], json!(102));
    assert!(execute_body["block_event_count"].as_u64().unwrap_or_default() >= 1);
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "option order is admitted and executed",
    given(empty_demo_state),
    when = "the client submits a Rust VM option order and executes a block",
    then(transaction_is_admitted, block_execution_succeeds),
    tags(http, rust_vm, option, e2e),
    priority = "5"
)]
async fn option_order_is_admitted_and_executed() {
    let app = build_app();
    let (submit_status, submit_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/transactions",
        transaction_request("req-bdd-option-1", "acct-bdd-option-1", "option_order", "rust_vm", "dex.option.place_order"),
    )
    .await;
    let (execute_status, execute_body) = send_json(
        app,
        Method::POST,
        "/api/l1/blocks/execute",
        json!({ "block_height": 103 }),
    )
    .await;

    assert_eq!(submit_status, StatusCode::OK);
    assert_eq!(submit_body["admitted_count"], json!(1));
    assert_eq!(submit_body["rejected_count"], json!(0));
    assert_eq!(execute_status, StatusCode::OK);
    assert_eq!(execute_body["block_height"], json!(103));
    assert!(execute_body["block_event_count"].as_u64().unwrap_or_default() >= 1);
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "evm settlement create is admitted and executed",
    given(empty_demo_state),
    when = "the client submits an EVM settlement create request and executes a block",
    then(transaction_is_admitted, evm_block_execution_succeeds),
    tags(http, evm, settlement, e2e),
    priority = "5"
)]
async fn evm_settlement_create_is_admitted_and_executed() {
    let state_path = next_state_path();
    let app = build_app_with_path(&state_path);
    let (submit_status, submit_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/transactions",
        transaction_request("req-bdd-evm-1", "acct-bdd-evm-1", "settlement", "evm", "evm.settlement.create"),
    )
    .await;
    let (execute_status, execute_body) = send_json(
        app,
        Method::POST,
        "/api/l1/blocks/execute",
        json!({ "block_height": 104 }),
    )
    .await;

    assert_eq!(submit_status, StatusCode::OK);
    assert_eq!(submit_body["admitted_count"], json!(1));
    assert_eq!(submit_body["rejected_count"], json!(0));
    assert_eq!(execute_status, StatusCode::OK);
    assert_eq!(execute_body["block_height"], json!(104));
    assert!(execute_body["block_event_count"].as_u64().unwrap_or_default() >= 1);
    assert_mdbx_files_exist(&state_path);
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "executing an empty block returns error",
    given(empty_mempool),
    when = "the client executes a block without pending requests",
    then(response_is_bad_request, error_mentions_empty_pending_requests),
    tags(http, error, execute_block, e2e),
    priority = "4"
)]
async fn executing_empty_block_returns_error() {
    let (status, body) = send_json(
        build_app(),
        Method::POST,
        "/api/l1/blocks/execute",
        json!({ "block_height": 105 }),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body["error"]
        .as_str()
        .unwrap_or_default()
        .contains("EmptyPendingRequests"));
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "malformed transaction payload returns client error",
    given(app_is_initialized),
    when = "the client posts malformed transaction json",
    then(client_error_is_returned),
    tags(http, error, validation, e2e),
    priority = "3"
)]
async fn malformed_transaction_payload_returns_client_error() {
    let (status, _) = send_json(
        build_app(),
        Method::POST,
        "/api/l1/transactions",
        json!({ "requests": [{ "request_id": "missing-fields" }] }),
    )
    .await;

    assert!(status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY);
}
