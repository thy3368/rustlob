#![allow(clippy::disallowed_methods, clippy::indexing_slicing)]

use std::error::Error;

use hyperliquid_node::explorer_block::{
    Action, ExplorerError, OrderId, fetch_explorer_block, parse_explorer_block,
};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const HEIGHT: u64 = 932_387_680;
const FIXTURE: &[u8] = include_bytes!("fixtures/explorer_block_932387680.json");

async fn fetch_from_mock(
    status: &'static str,
    body: &'static [u8],
) -> Result<
    (
        Result<hyperliquid_node::explorer_block::FetchedExplorerBlock, ExplorerError>,
        Vec<u8>,
    ),
    Box<dyn Error>,
> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await?;
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let count = stream.read(&mut buffer).await?;
            if count == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..count]);
            if let Some(header_end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&request[..header_end]);
                let length = headers
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(str::trim)
                            .and_then(|value| value.parse::<usize>().ok())
                    })
                    .unwrap_or(0);
                if request.len() >= header_end + 4 + length {
                    break;
                }
            }
        }
        let headers = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(headers.as_bytes()).await?;
        stream.write_all(body).await?;
        Ok::<_, std::io::Error>(request)
    });
    let result = fetch_explorer_block(&format!("http://{address}/explorer"), HEIGHT).await;
    Ok((result, server.await??))
}

#[tokio::test]
async fn posts_exact_height_and_preserves_raw_fixture() -> Result<(), Box<dyn Error>> {
    let (result, request) = fetch_from_mock("200 OK", FIXTURE).await?;
    let fetched = result?;
    assert_eq!(fetched.raw_response(), FIXTURE);
    assert_eq!(fetched.block().height, HEIGHT);
    let request = String::from_utf8(request)?;
    assert!(request.starts_with("POST /explorer HTTP/1.1\r\n"));
    let body = request.split("\r\n\r\n").nth(1).ok_or("missing body")?;
    assert_eq!(
        serde_json::from_str::<Value>(body)?,
        json!({"type":"blockDetails","height":HEIGHT})
    );
    Ok(())
}

#[test]
fn fixture_covers_strict_contract_and_action_payloads() -> Result<(), Box<dyn Error>> {
    let fetched = parse_explorer_block(FIXTURE.to_vec(), HEIGHT)?;
    let block = fetched.block();
    assert_eq!(block.num_txs, 5868);
    assert_eq!(block.txs.len(), 5868);
    assert_eq!(
        block.hash,
        "0x9b454a5f2072a6c2e1c45389f917b1730a4b5d8d85033d88b010553e738ea10e"
    );
    assert_eq!(block.proposer, "0xa82fe73bbd768bc15d1ef2f6142a21ff8bd762ad");
    assert!(
        block
            .txs
            .iter()
            .all(|transaction| transaction.block == HEIGHT)
    );
    assert!(
        block
            .txs
            .iter()
            .any(|transaction| transaction.error.is_none())
    );
    assert!(
        block
            .txs
            .iter()
            .any(|transaction| transaction.error.is_some())
    );

    let mut kinds = [false; 8];
    let mut numeric_oid = false;
    let mut cloid = false;
    for transaction in &block.txs {
        match &transaction.action {
            Action::Order(action) => {
                kinds[0] = true;
                assert!(
                    action
                        .orders
                        .iter()
                        .all(|order| !order.p.is_empty() && !order.s.is_empty())
                );
                if action.builder.is_some() {
                    assert!(
                        action
                            .builder
                            .as_ref()
                            .is_some_and(|value| !value.b.is_empty())
                    );
                }
            }
            Action::Cancel(action) => {
                kinds[1] = true;
                numeric_oid |= !action.cancels.is_empty();
            }
            Action::CancelByCloid(action) => {
                kinds[2] = true;
                cloid |= action
                    .cancels
                    .iter()
                    .any(|cancel| cancel.asset > 0 && cancel.cloid.starts_with("0x"));
            }
            Action::Modify(action) => {
                kinds[3] = true;
                cloid |= matches!(action.oid, OrderId::Cloid(_));
            }
            Action::BatchModify(action) => {
                kinds[4] = true;
                numeric_oid |= action
                    .modifies
                    .iter()
                    .any(|modify| matches!(modify.oid, OrderId::Number(_)));
                cloid |= action
                    .modifies
                    .iter()
                    .any(|modify| matches!(modify.oid, OrderId::Cloid(_)));
            }
            Action::Noop(_) => kinds[5] = true,
            Action::ScheduleCancel(_) => kinds[6] = true,
            Action::UpdateLeverage(_) => kinds[7] = true,
        }
    }
    assert!(kinds.into_iter().all(|present| present));
    assert!(numeric_oid && cloid);
    Ok(())
}

fn minimal(action: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({"type":"blockDetails","blockDetails":{"height":HEIGHT,"blockTime":1,"hash":"0xh","proposer":"0xp","numTxs":1,"txs":[{"time":1,"user":"0xu","action":action,"block":HEIGHT,"hash":"0xt","error":null}]}})).expect("static JSON serializes")
}

#[test]
fn rejects_protocol_and_invariant_failures() {
    assert!(matches!(
        parse_explorer_block(b"null".to_vec(), HEIGHT),
        Err(ExplorerError::NullResponse)
    ));
    assert!(matches!(
        parse_explorer_block(br#"{"type":"error","message":"missing"}"#.to_vec(), HEIGHT),
        Err(ExplorerError::Official(_))
    ));
    assert!(matches!(
        parse_explorer_block(b"{".to_vec(), HEIGHT),
        Err(ExplorerError::InvalidJson(_))
    ));
    assert!(matches!(
        parse_explorer_block(minimal(json!({"type":"future"})), HEIGHT),
        Err(ExplorerError::UnknownSchema(_))
    ));
    assert!(matches!(
        parse_explorer_block(minimal(json!({"type":"noop","future":true})), HEIGHT),
        Err(ExplorerError::UnknownSchema(_))
    ));

    let mut value: Value =
        serde_json::from_slice(&minimal(json!({"type":"noop"}))).expect("valid fixture");
    value["blockDetails"]["height"] = json!(HEIGHT + 1);
    assert!(matches!(
        parse_explorer_block(serde_json::to_vec(&value).expect("serializes"), HEIGHT),
        Err(ExplorerError::HeightMismatch { .. })
    ));
    value["blockDetails"]["height"] = json!(HEIGHT);
    value["blockDetails"]["numTxs"] = json!(2);
    assert!(matches!(
        parse_explorer_block(serde_json::to_vec(&value).expect("serializes"), HEIGHT),
        Err(ExplorerError::TransactionCountMismatch { .. })
    ));
    value["blockDetails"]["numTxs"] = json!(1);
    value["blockDetails"]["txs"][0]["block"] = json!(HEIGHT + 1);
    assert!(matches!(
        parse_explorer_block(serde_json::to_vec(&value).expect("serializes"), HEIGHT),
        Err(ExplorerError::TransactionHeightMismatch { .. })
    ));
}

#[tokio::test]
async fn reports_http_non_success() -> Result<(), Box<dyn Error>> {
    let (result, _) = fetch_from_mock("503 Service Unavailable", br#"{"message":"down"}"#).await?;
    assert!(
        matches!(result, Err(ExplorerError::HttpStatus { status, .. }) if status.as_u16() == 503)
    );
    Ok(())
}
