use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode},
};
use bdd::bdd_test;
use http_body_util::BodyExt;
use l1_adapter::MdbxStateStore;
use l1_core::{Account, CodeBlob, CodeStore, StateReader, VmKind};
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

fn assert_mdbx_state_exists(
    state_path: &PathBuf,
    address: alloy_primitives::Address,
    storage_key: alloy_primitives::B256,
    expected_account: Account,
    expected_storage: alloy_primitives::B256,
    expected_code: CodeBlob,
) {
    let store = MdbxStateStore::open(state_path).expect("failed to reopen MDBX store");

    assert_eq!(store.account(address).expect("failed to load account"), Some(expected_account));
    assert_eq!(
        store.storage(address, storage_key).expect("failed to load storage"),
        expected_storage
    );
    assert_eq!(
        CodeStore::code(&store, expected_code.code_hash).expect("failed to load code"),
        Some(expected_code)
    );
}

fn hash_to_b256(input: &str) -> alloy_primitives::B256 {
    let digest = md5::compute(input.as_bytes());
    let mut bytes = [0u8; 32];
    bytes[..16].copy_from_slice(&digest.0);
    bytes[16..].copy_from_slice(&digest.0);
    alloy_primitives::B256::from(bytes)
}

fn rustvm_address(performer: &str, gas_used: u64) -> alloy_primitives::Address {
    let hash = hash_to_b256(&format!("{}:{}", performer, gas_used));
    alloy_primitives::Address::from_slice(&hash.as_slice()[..20])
}

fn rustvm_expected_state(
    performer: &str,
    request_id: &str,
    action_type: &str,
    capability: &str,
    payload_hash: &str,
) -> (
    alloy_primitives::Address,
    alloy_primitives::B256,
    Account,
    alloy_primitives::B256,
    CodeBlob,
) {
    let gas_used = 1;
    let address = rustvm_address(performer, gas_used);
    let code_hash = hash_to_b256(&format!("{}:{}:{}", capability, payload_hash, action_type));
    let storage_key = hash_to_b256(request_id);
    let storage_value = hash_to_b256(&format!("{}:{}:{}", performer, action_type, payload_hash));
    let account = Account {
        nonce: gas_used,
        balance: alloy_primitives::U256::from(gas_used),
        code_hash,
        storage_root: storage_value,
        vm_kind: VmKind::RustVm,
    };
    let code = CodeBlob {
        code_hash,
        vm_kind: VmKind::RustVm,
        bytes: format!("rustvm:{}:{}:{}", capability, action_type, payload_hash).into_bytes(),
    };

    (address, storage_key, account, storage_value, code)
}

fn evm_encode_amount(payload_hash: &str) -> u64 {
    payload_hash
        .as_bytes()
        .iter()
        .fold(0u64, |acc, byte| acc.wrapping_mul(131).wrapping_add(*byte as u64))
        % 10_000
        + 1
}

fn evm_expected_state(
    performer: &str,
    request_id: &str,
    payload_hash: &str,
) -> (
    alloy_primitives::Address,
    alloy_primitives::B256,
    Account,
    alloy_primitives::B256,
    CodeBlob,
) {
    let gas_used = 2;
    let address = l1_adapter::contracts::address_from_performer(performer);
    let contract_name = format!("settlement-{performer}");
    let code_hash = hash_to_b256(&format!("create:{contract_name}"));
    let storage_key = hash_to_b256(&l1_adapter::contracts::settlement_id_from_seed(request_id));
    let storage_value = hash_to_b256(&format!(
        "create:{}:{}:{}",
        request_id,
        payload_hash,
        evm_encode_amount(payload_hash)
    ));
    let account = Account {
        nonce: gas_used,
        balance: alloy_primitives::U256::from(evm_encode_amount(payload_hash)),
        code_hash,
        storage_root: storage_value,
        vm_kind: VmKind::Evm,
    };
    let code = CodeBlob {
        code_hash,
        vm_kind: VmKind::Evm,
        bytes: format!("evm:create:{contract_name}").into_bytes(),
    };

    (address, storage_key, account, storage_value, code)
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

fn spot_match_request(request_id: &str, account: &str, side: &str) -> Value {
    transaction_request(request_id, account, side, "rust_vm", "dex.spot.place_order")
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
    let (address, storage_key, account, storage_value, code) = rustvm_expected_state(
        "acct-bdd-spot-1",
        "req-bdd-spot-1",
        "spot_order",
        "dex.spot.place_order",
        "payload-req-bdd-spot-1",
    );
    assert_mdbx_state_exists(&state_path, address, storage_key, account, storage_value, code);
}

#[tokio::test]
#[bdd_test(
    feature = "L1 HTTP E2E",
    scenario = "spot orders match in a single executed block",
    given(empty_demo_state),
    when = "the client submits crossing Rust VM spot orders and executes a block",
    then(transaction_is_admitted, spot_matching_execution_succeeds),
    tags(http, rust_vm, spot, matching, e2e),
    priority = "5"
)]
async fn spot_orders_match_in_a_single_executed_block() {
    let app = build_app();
    let (sell_submit_status, sell_submit_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/transactions",
        spot_match_request("req-bdd-spot-sell-1", "acct-bdd-spot-maker-1", "sell"),
    )
    .await;
    let (buy_submit_status, buy_submit_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/transactions",
        spot_match_request("req-bdd-spot-buy-1", "acct-bdd-spot-taker-1", "buy"),
    )
    .await;
    let (execute_status, execute_body) = send_json(
        app.clone(),
        Method::POST,
        "/api/l1/blocks/execute",
        json!({ "block_height": 1011 }),
    )
    .await;
    let (health_status, health_body) = send_empty(app, Method::GET, "/api/l1/health").await;

    assert_eq!(sell_submit_status, StatusCode::OK);
    assert_eq!(sell_submit_body["admitted_count"], json!(1));
    assert_eq!(sell_submit_body["rejected_count"], json!(0));
    assert_eq!(buy_submit_status, StatusCode::OK);
    assert_eq!(buy_submit_body["admitted_count"], json!(1));
    assert_eq!(buy_submit_body["rejected_count"], json!(0));
    assert_eq!(execute_status, StatusCode::OK);
    assert_eq!(execute_body["block_height"], json!(1011));
    assert_eq!(execute_body["matched_trade_count"], json!(1));
    assert!(execute_body["block_event_count"].as_u64().unwrap_or_default() >= 2);
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body["mempool_len"], json!(0));
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
    let (address, storage_key, account, storage_value, code) = evm_expected_state(
        "acct-bdd-evm-1",
        "req-bdd-evm-1",
        "payload-req-bdd-evm-1",
    );
    assert_mdbx_state_exists(&state_path, address, storage_key, account, storage_value, code);
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
