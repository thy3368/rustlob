#![allow(clippy::disallowed_methods)]

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread};

const HEIGHT: u64 = 932_387_680;

fn temporary_directory(test: &str) -> Result<PathBuf, Box<dyn Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "hyperliquid-node-{test}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&path)?;
    Ok(path)
}

fn response(action: &str) -> Vec<u8> {
    format!(
        " {{\"type\":\"blockDetails\",\"blockDetails\":{{\"height\":{HEIGHT},\"blockTime\":1,\"hash\":\"0xh\",\"proposer\":\"0xp\",\"numTxs\":1,\"txs\":[{{\"time\":1,\"user\":\"0xu\",\"action\":{action},\"block\":{HEIGHT},\"hash\":\"0xt\",\"error\":null}}]}}}}\n"
    )
    .into_bytes()
}

fn serve_once(
    body: Vec<u8>,
) -> Result<(String, thread::JoinHandle<std::io::Result<Vec<u8>>>), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept()?;
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let count = stream.read(&mut buffer)?;
            request.extend_from_slice(&buffer[..count]);
            if count == 0 || request.windows(4).any(|part| part == b"\r\n\r\n") {
                break;
            }
        }
        let headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(headers.as_bytes())?;
        stream.write_all(&body)?;
        Ok(request)
    });
    Ok((format!("http://{address}/explorer"), handle))
}

fn run(
    directory: &Path,
    endpoint: &str,
    output: Option<&Path>,
) -> Result<std::process::Output, Box<dyn Error>> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_fetch_explorer_block"));
    command
        .current_dir(directory)
        .env("HYPERLIQUID_EXPLORER_URL", endpoint)
        .arg(HEIGHT.to_string());
    if let Some(output) = output {
        command.arg(output);
    }
    Ok(command.output()?)
}

#[test]
fn default_name_preserves_official_bytes() -> Result<(), Box<dyn Error>> {
    let directory = temporary_directory("default")?;
    let body = response(r#"{"type":"noop"}"#);
    let (endpoint, server) = serve_once(body.clone())?;
    let output = run(&directory, &endpoint, None)?;
    server.join().map_err(|_| "server panicked")??;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read(directory.join(format!("explorer_block_{HEIGHT}.json")))?,
        body
    );
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn explicit_path_is_not_overwritten() -> Result<(), Box<dyn Error>> {
    let directory = temporary_directory("overwrite")?;
    let target = directory.join("chosen.json");
    fs::write(&target, b"existing")?;
    let (endpoint, server) = serve_once(response(r#"{"type":"noop"}"#))?;
    let output = run(&directory, &endpoint, Some(&target))?;
    server.join().map_err(|_| "server panicked")??;
    assert!(!output.status.success());
    assert_eq!(fs::read(&target)?, b"existing");
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn invalid_schema_saves_unparsed_evidence_without_touching_target() -> Result<(), Box<dyn Error>> {
    let directory = temporary_directory("unparsed")?;
    let target = directory.join("chosen.json");
    let body = response(r#"{"type":"futureAction"}"#);
    let (endpoint, server) = serve_once(body.clone())?;
    let output = run(&directory, &endpoint, Some(&target))?;
    server.join().map_err(|_| "server panicked")??;
    assert!(!output.status.success());
    assert!(!target.exists());
    assert_eq!(fs::read(directory.join("chosen.json.unparsed.json"))?, body);
    fs::remove_dir_all(directory)?;
    Ok(())
}
