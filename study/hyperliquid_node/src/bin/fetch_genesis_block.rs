use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, fmt};

use hyperliquid_node::replica_rpc::get_block;

#[derive(Debug)]
struct UsageError;

impl fmt::Display for UsageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("usage: fetch_genesis_block <RPC_URL> [OUTPUT_JSON]")
    }
}

impl Error for UsageError {}

fn main() -> Result<(), Box<dyn Error>> {
    tokio::runtime::Builder::new_multi_thread().enable_all().build()?.block_on(run())
}

#[allow(clippy::disallowed_methods)] // 此离线 CLI 的职责就是把 RPC artifact 写入调用方指定路径。
async fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let rpc_url = args.next().ok_or(UsageError)?;
    let rpc_url = rpc_url.to_str().ok_or(UsageError)?;
    let output =
        args.next().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("hyperliquid_block_0.json"));
    if args.next().is_some() {
        return Err(UsageError.into());
    }

    let fetched = get_block(rpc_url, 0).await?;
    let block_json = serde_json::to_vec_pretty(fetched.raw_block())?;
    let mut stdout = io::stdout().lock();
    stdout.write_all(&block_json)?;
    stdout.write_all(b"\n")?;

    let json = serde_json::to_vec_pretty(fetched.raw_response())?;
    File::create(output)?.write_all(&json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_error_explains_required_arguments() {
        assert_eq!(UsageError.to_string(), "usage: fetch_genesis_block <RPC_URL> [OUTPUT_JSON]");
    }
}
