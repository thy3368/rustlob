#![allow(
    clippy::arithmetic_side_effects,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::indexing_slicing
)]

use std::error::Error;
use std::process::Command;

use hyperliquid_node::replica_rpc::{ReplicaBlock, RpcError, get_block};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const FIXTURE: &str = include_str!("fixtures/block_zero_rpc.json");

async fn fetch_from_mock(
    status: &'static str,
    body: &'static str,
) -> Result<
    (
        Result<hyperliquid_node::replica_rpc::FetchedBlock, RpcError>,
        Vec<u8>,
    ),
    Box<dyn Error>,
> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await?;
        let mut request = Vec::new();
        let mut buffer = [0_u8; 8192];
        let count = stream.read(&mut buffer).await?;
        request.extend_from_slice(&buffer[..count]);
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).await?;
        Ok::<_, std::io::Error>(request)
    });
    let result = get_block(&format!("http://{address}"), 0).await;
    Ok((result, server.await??))
}

#[tokio::test]
async fn unwraps_result_and_preserves_unknown_fields() -> Result<(), Box<dyn Error>> {
    let (result, request) = fetch_from_mock("200 OK", FIXTURE).await?;
    let fetched = result?;
    assert_eq!(fetched.block().height, Some(0));
    assert_eq!(fetched.raw_response().get("jsonrpc"), Some(&json!("2.0")));
    assert_eq!(
        fetched.raw_block()["future_block_field"]["enabled"],
        json!(true)
    );
    assert!(serde_json::to_vec(fetched.raw_response())?.len() > 1);

    let request = String::from_utf8(request)?;
    let body = request
        .split("\r\n\r\n")
        .nth(1)
        .ok_or("missing request body")?;
    let value: Value = serde_json::from_str(body)?;
    assert_eq!(
        value["params"][0],
        json!({"stream": "replica_cmds", "block_number": 0})
    );
    Ok(())
}

#[test]
fn parses_bundle_tuple_and_multiple_string_orders() -> Result<(), Box<dyn Error>> {
    let envelope: Value = serde_json::from_str(FIXTURE)?;
    let block: ReplicaBlock = serde_json::from_value(envelope["result"].clone())?;
    let (hash, bundle) = block
        .signed_action_bundles
        .first()
        .ok_or("missing bundle")?;
    assert_eq!(hash, "0xbundle");
    let action = bundle
        .signed_actions
        .first()
        .and_then(|signed| signed.action.as_ref())
        .ok_or("missing action")?;
    assert_eq!(action.orders.len(), 2);
    assert_eq!(action.orders[0].p, "12345.6789");
    assert_eq!(action.orders[1].s, "999999.9");
    Ok(())
}

#[test]
fn missing_optional_fields_parse() -> Result<(), serde_json::Error> {
    let block: ReplicaBlock = serde_json::from_value(json!({}))?;
    assert_eq!(block.height, None);
    assert!(block.signed_action_bundles.is_empty());
    Ok(())
}

#[tokio::test]
async fn reports_json_rpc_error() -> Result<(), Box<dyn Error>> {
    let (result, _) = fetch_from_mock(
        "200 OK",
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"block unavailable"}}"#,
    )
    .await?;
    let error = result.err().ok_or("expected JSON-RPC error")?;
    assert!(matches!(&error, RpcError::JsonRpc(rpc_error) if rpc_error.code == -32000));
    assert_eq!(
        error.to_string(),
        "JSON-RPC error -32000: block unavailable"
    );
    Ok(())
}

#[tokio::test]
async fn reports_invalid_json() -> Result<(), Box<dyn Error>> {
    let (result, _) = fetch_from_mock("200 OK", "not-json").await?;
    let error = result.err().ok_or("expected invalid JSON error")?;
    assert!(matches!(&error, RpcError::InvalidJson(_)));
    assert!(error.source().is_some());
    Ok(())
}

#[tokio::test]
async fn reports_http_error() -> Result<(), Box<dyn Error>> {
    let (result, _) = fetch_from_mock("503 Service Unavailable", r#"{"message":"down"}"#).await?;
    assert!(matches!(result, Err(RpcError::HttpStatus { status, .. }) if status.as_u16() == 503));
    Ok(())
}

#[test]
fn fetch_binary_requires_rpc_url() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_fetch_genesis_block")).output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr)?;
    assert!(stderr.contains("UsageError"));
    Ok(())
}
