use std::net::SocketAddr;

use hotstuff_use_case_demo::demo_runtime::{DemoResult, NODE_COUNT};
use hotstuff_use_case_demo::tcp_node::{TcpNodeConfig, run_tcp_node};

const DEFAULT_PEER_ADDRS: [&str; NODE_COUNT] =
    ["127.0.0.1:39101", "127.0.0.1:39102", "127.0.0.1:39103"];
const DEFAULT_HTTP_ADDRS: [&str; NODE_COUNT] =
    ["127.0.0.1:39001", "127.0.0.1:39002", "127.0.0.1:39003"];

fn main() -> DemoResult<()> {
    let node_index = std::env::var("HOTSTUFF_DEMO_NODE_INDEX")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<usize>()?;
    if node_index >= NODE_COUNT {
        return Err(format!(
            "HOTSTUFF_DEMO_NODE_INDEX={node_index} 超出范围，当前 demo 只支持 0..{}",
            NODE_COUNT - 1
        )
        .into());
    }

    let peer_addrs = DEFAULT_PEER_ADDRS
        .iter()
        .map(|addr| addr.parse::<SocketAddr>())
        .collect::<Result<Vec<_>, _>>()?;
    let http_addr = std::env::var("HOTSTUFF_DEMO_HTTP_ADDR")
        .unwrap_or_else(|_| DEFAULT_HTTP_ADDRS[node_index].to_string())
        .parse::<SocketAddr>()?;

    run_tcp_node(TcpNodeConfig { node_index, http_addr, peer_addrs })
}
