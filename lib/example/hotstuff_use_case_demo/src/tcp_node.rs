use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ed25519_dalek::SigningKey;
use example_core_use_case::SpotBlockCommand;
use hotstuff_rs::block_tree::accessors::public::BlockTreeCamera;
use hotstuff_rs::events::{CommitBlockEvent, InsertBlockEvent};
use hotstuff_rs::replica::{Replica, ReplicaSpec};

use crate::demo_runtime::{
    DemoResult, MemDB, NODE_COUNT, SpotOrderApp, decode_requests, demo_replica_configuration,
    demo_validator_set_state, deterministic_signing_keys, start_http_server,
};
use crate::tcp_network::TcpNetwork;

pub struct TcpNodeConfig {
    pub node_index: usize,
    pub http_addr: SocketAddr,
    pub peer_addrs: Vec<SocketAddr>,
}

pub fn run_tcp_node(config: TcpNodeConfig) -> DemoResult<()> {
    if config.node_index >= NODE_COUNT {
        return Err(format!(
            "HOTSTUFF_DEMO_NODE_INDEX={} 超出范围，当前 demo 只支持 0..{}",
            config.node_index,
            NODE_COUNT - 1
        )
        .into());
    }
    if config.peer_addrs.len() != NODE_COUNT {
        return Err(format!(
            "peer_addrs 长度必须为 {NODE_COUNT}，当前为 {}",
            config.peer_addrs.len()
        )
        .into());
    }

    let signing_keys = deterministic_signing_keys();
    let verifying_keys = signing_keys.iter().map(SigningKey::verifying_key).collect::<Vec<_>>();
    let my_signing_key = signing_keys[config.node_index].clone();
    let my_verifying_key = verifying_keys[config.node_index];
    let my_tcp_addr = config.peer_addrs[config.node_index];
    let peer_map = verifying_keys
        .iter()
        .copied()
        .zip(config.peer_addrs.iter().copied())
        .collect::<HashMap<_, _>>();

    let kv_store = MemDB::new();
    Replica::initialize(
        kv_store.clone(),
        SpotOrderApp::initial_app_state(),
        demo_validator_set_state(&verifying_keys),
    );

    let request_queue = Arc::new(Mutex::new(Vec::<SpotBlockCommand>::new()));
    let network = TcpNetwork::bind(my_verifying_key, my_tcp_addr, peer_map)?;
    let configuration = demo_replica_configuration(my_signing_key);
    let node_index = config.node_index;
    let commit_kv_store = kv_store.clone();

    let replica = ReplicaSpec::builder()
        .app(SpotOrderApp::new(Arc::clone(&request_queue)))
        .network(network)
        .kv_store(kv_store)
        .configuration(configuration)
        .on_insert_block(move |event: &InsertBlockEvent| {
            let has_requests = decode_requests(&event.block.data)
                .map(|requests| !requests.is_empty())
                .unwrap_or(false);
            if has_requests {
                println!(
                    "[hotstuff_tcp_node:{node_index}] block inserted: height={}",
                    event.block.height
                );
            }
        })
        .on_commit_block(move |event: &CommitBlockEvent| {
            let camera = BlockTreeCamera::new(commit_kv_store.clone());
            let snapshot = camera.snapshot();
            let Some(block) = snapshot.block(&event.block).ok().flatten() else {
                return;
            };
            let has_requests =
                decode_requests(&block.data).map(|requests| !requests.is_empty()).unwrap_or(false);
            if has_requests {
                println!(
                    "[hotstuff_tcp_node:{node_index}] block committed: height={}",
                    block.height
                );
            }
        })
        .build()
        .start();

    let http_server = start_http_server(config.http_addr, request_queue)?;
    println!(
        "[hotstuff_tcp_node:{}] 已启动单节点 replica，tcp={}, http=http://{}",
        config.node_index, my_tcp_addr, config.http_addr
    );

    let _replica = replica;
    let _http_server = http_server;
    loop {
        std::thread::sleep(Duration::from_secs(60));
    }
}
