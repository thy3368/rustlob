use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use ed25519_dalek::{SigningKey, VerifyingKey};
use example_core_use_case::{
    CancelSpotOrderV2Cmd, CancelSpotOrderV2Lookup, PlaceOnlySpotOrderV2Cmd,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, SpotBlockCommand,
};
use hotstuff_rs::events::{CommitBlockEvent, InsertBlockEvent};
use hotstuff_rs::replica::{Replica, ReplicaSpec};
use hotstuff_use_case_demo::demo_runtime::{
    CANCEL_RESULT_KEY, DEFAULT_HTTP_ADDR, DemoResult, MemDB, NODE_COUNT, PLACE_RESULT_KEY,
    SpotOrderApp, decode_requests, demo_replica_configuration, demo_validator_set_state,
    get_from_snapshot, mock_network, start_http_server,
};

fn main() -> DemoResult<()> {
    println!("[hotstuff_use_case_demo] 启动 3 节点 HotStuff spot order use case demo");

    let mut rng = rand_core::OsRng;
    let signing_keys: Vec<SigningKey> =
        (0..NODE_COUNT).map(|_| SigningKey::generate(&mut rng)).collect();
    let verifying_keys: Vec<VerifyingKey> =
        signing_keys.iter().map(SigningKey::verifying_key).collect();
    println!("[hotstuff_use_case_demo] 生成 {} 个验证者密钥", verifying_keys.len());

    let network_stubs = mock_network(verifying_keys.iter().copied());
    let init_vs_state = demo_validator_set_state(&verifying_keys);
    let initial_app_state = SpotOrderApp::initial_app_state();
    let request_queues: Vec<Arc<Mutex<Vec<SpotBlockCommand>>>> =
        (0..NODE_COUNT).map(|_| Arc::new(Mutex::new(Vec::new()))).collect();
    let inserted_payload_blocks = Arc::new(Mutex::new(0usize));
    let committed_blocks = Arc::new(Mutex::new(0usize));
    let mut replicas: Vec<Replica<MemDB>> = Vec::new();

    for index in 0..NODE_COUNT {
        let kv_store = MemDB::new();
        Replica::initialize(kv_store.clone(), initial_app_state.clone(), init_vs_state.clone());

        let inserted_payload_blocks_for_handler = Arc::clone(&inserted_payload_blocks);
        let committed_blocks_for_handler = Arc::clone(&committed_blocks);
        let replica = ReplicaSpec::builder()
            .app(SpotOrderApp::new(Arc::clone(&request_queues[index])))
            .network(network_stubs[index].clone())
            .kv_store(kv_store)
            .configuration(demo_replica_configuration(signing_keys[index].clone()))
            .on_insert_block(move |event: &InsertBlockEvent| {
                let has_requests = decode_requests(&event.block.data)
                    .map(|requests| !requests.is_empty())
                    .unwrap_or(false);
                if has_requests {
                    if let Ok(mut count) = inserted_payload_blocks_for_handler.lock() {
                        *count = count.saturating_add(1);
                    }
                    println!(
                        "[hotstuff_use_case_demo] block inserted: height={}",
                        event.block.height
                    );
                }
            })
            .on_commit_block(move |_event: &CommitBlockEvent| {
                if let Ok(mut count) = committed_blocks_for_handler.lock() {
                    *count = count.saturating_add(1);
                }
                println!("[hotstuff_use_case_demo] block committed");
            })
            .build()
            .start();

        replicas.push(replica);
        println!("[hotstuff_use_case_demo] 已启动副本 {index}");
    }

    let http_addr: SocketAddr = std::env::var("HOTSTUFF_DEMO_HTTP_ADDR")
        .unwrap_or_else(|_| DEFAULT_HTTP_ADDR.to_string())
        .parse()?;
    let _http_server = start_http_server(
        http_addr,
        Arc::clone(request_queues.first().ok_or("missing leader request queue")?),
    )?;

    let place_command = PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
        party_id: "buyer".to_string(),
        asset: 10_001,
        order_id: "taker-buy".to_string(),
        symbol: "BTCUSDT".to_string(),
        is_buy: true,
        price: "100".to_string(),
        size: "2".to_string(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".to_string() },
        reduce_only: false,
        cloid: Some("demo-place-1".to_string()),
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    });
    let cancel_command = CancelSpotOrderV2Cmd {
        party_id: "buyer".to_string(),
        asset: 10_001,
        lookup: CancelSpotOrderV2Lookup::Oid(77738308),
    };
    request_queues
        .first()
        .ok_or("missing leader request queue")?
        .lock()
        .map_err(|_| "leader request queue poisoned")?
        .extend([
            SpotBlockCommand::PlaceMatch(place_command),
            SpotBlockCommand::Cancel(cancel_command),
        ]);
    println!(
        "[hotstuff_use_case_demo] 已向 leader 队列提交 PlaceSpotOrderV2Cmd 和 CancelSpotOrderV2Cmd"
    );

    let start = Instant::now();
    let mut final_value = None;
    for attempt in 1..=80 {
        std::thread::sleep(Duration::from_millis(250));
        let place_results = replicas
            .iter()
            .map(|replica| {
                let snapshot = replica.block_tree_camera().snapshot();
                get_from_snapshot(&snapshot, PLACE_RESULT_KEY)
            })
            .collect::<Vec<_>>();
        let cancel_results = replicas
            .iter()
            .map(|replica| {
                let snapshot = replica.block_tree_camera().snapshot();
                get_from_snapshot(&snapshot, CANCEL_RESULT_KEY)
            })
            .collect::<Vec<_>>();
        let place_first = place_results.first().cloned().flatten();
        let cancel_first = cancel_results.first().cloned().flatten();
        if place_first.is_some()
            && cancel_first.is_some()
            && place_results.iter().all(|result| *result == place_first)
            && cancel_results.iter().all(|result| *result == cancel_first)
        {
            final_value = place_first;
            println!("[hotstuff_use_case_demo] 所有节点 committed app state 已一致");
            break;
        }

        if attempt % 8 == 0 {
            println!("[hotstuff_use_case_demo] 等待 committed app state，attempt={attempt}");
        }
    }

    let Some(value) = final_value else {
        return Err("30 秒内未查询到 committed place/cancel 执行摘要".into());
    };
    let place_summary: serde_json::Value = serde_json::from_slice(&value)?;
    let cancel_value = replicas
        .first()
        .and_then(|replica| {
            let snapshot = replica.block_tree_camera().snapshot();
            get_from_snapshot(&snapshot, CANCEL_RESULT_KEY)
        })
        .ok_or("committed cancel execution summary missing")?;
    let cancel_summary: serde_json::Value = serde_json::from_slice(&cancel_value)?;
    let inserted_count = *inserted_payload_blocks.lock().map_err(|_| "insert count poisoned")?;
    let committed_count = *committed_blocks.lock().map_err(|_| "commit count poisoned")?;
    if inserted_count == 0 {
        return Err("未观察到非空 payload block inserted event".into());
    }
    if committed_count == 0 {
        return Err("未观察到 block committed event".into());
    }

    println!(
        "[hotstuff_use_case_demo] committed app state 查询成功: place={}, cancel={}",
        serde_json::to_string(&place_summary)?,
        serde_json::to_string(&cancel_summary)?
    );
    println!("[hotstuff_use_case_demo] 成功完成，请求出块并提交，耗时 {:?}", start.elapsed());

    Ok(())
}
