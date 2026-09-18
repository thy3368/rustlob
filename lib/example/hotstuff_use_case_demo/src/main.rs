use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{StateSink, StateSource};
use ed25519_dalek::{SigningKey, VerifyingKey};
use example_core_use_case::{
    Balance, CancelSpotOrderV2Cmd, CancelSpotOrderV2Lookup, PlaceOnlySpotOrderV2Cmd,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, SpotBlockAppliedChanges,
    SpotBlockChanges, SpotBlockCmd, SpotBlockCommand, SpotBlockItemResult, SpotBlockState,
    SpotBlockUseCase, SpotOrderSide, SpotOrderStatus, SpotOrderTif, SpotOrderType, SpotOrderV2,
};
use hotstuff_rs::app::{
    App, ProduceBlockRequest, ProduceBlockResponse, ValidateBlockRequest, ValidateBlockResponse,
};
use hotstuff_rs::block_tree::accessors::public::BlockTreeSnapshot;
use hotstuff_rs::block_tree::pluggables::{KVGet, KVStore, WriteBatch};
use hotstuff_rs::events::{CommitBlockEvent, InsertBlockEvent};
use hotstuff_rs::networking::messages::Message;
use hotstuff_rs::networking::network::Network;
use hotstuff_rs::replica::{Configuration, Replica, ReplicaSpec};
use hotstuff_rs::types::crypto_primitives::{CryptoHasher, Digest};
use hotstuff_rs::types::data_types::{
    BufferSize, ChainID, CryptoHash, Data, Datum, EpochLength, Power,
};
use hotstuff_rs::types::update_sets::{AppStateUpdates, ValidatorSetUpdates};
use hotstuff_rs::types::validator_set::{ValidatorSet, ValidatorSetState};
use serde::Serialize;
use use_case_executor::trading::spot::spot_block_executor::execute_spot_block_with_outbound;

type DemoResult<T> = Result<T, Box<dyn std::error::Error>>;

const NODE_COUNT: usize = 3;
const PLACE_RESULT_KEY: &[u8] = b"place:buyer:demo-place-1";
const CANCEL_RESULT_KEY: &[u8] = b"cancel:buyer:77738308";

#[derive(Clone)]
struct MemDB(Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>);

impl MemDB {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}

impl KVStore for MemDB {
    type WriteBatch = MemWriteBatch;
    type Snapshot<'a> = MemDBSnapshot<'a>;

    fn write(&mut self, wb: Self::WriteBatch) {
        if let Ok(mut map) = self.0.lock() {
            for (key, value) in wb.insertions {
                map.insert(key, value);
            }
            for key in wb.deletions {
                map.remove(&key);
            }
        }
    }

    fn clear(&mut self) {
        if let Ok(mut map) = self.0.lock() {
            map.clear();
        }
    }

    fn snapshot<'a>(&'a self) -> MemDBSnapshot<'a> {
        MemDBSnapshot(lock_or_recover(&self.0))
    }
}

impl KVGet for MemDB {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.0.lock().ok().and_then(|map| map.get(key).cloned())
    }
}

struct MemWriteBatch {
    insertions: HashMap<Vec<u8>, Vec<u8>>,
    deletions: HashSet<Vec<u8>>,
}

impl WriteBatch for MemWriteBatch {
    fn new() -> Self {
        Self { insertions: HashMap::new(), deletions: HashSet::new() }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        let _ = self.deletions.remove(key);
        self.insertions.insert(key.to_vec(), value.to_vec());
    }

    fn delete(&mut self, key: &[u8]) {
        let _ = self.insertions.remove(key);
        self.deletions.insert(key.to_vec());
    }
}

struct MemDBSnapshot<'a>(MutexGuard<'a, HashMap<Vec<u8>, Vec<u8>>>);

impl KVGet for MemDBSnapshot<'_> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.0.get(key).cloned()
    }
}

#[derive(Clone)]
struct NetworkStub {
    my_verifying_key: VerifyingKey,
    all_peers: HashMap<VerifyingKey, Sender<(VerifyingKey, Message)>>,
    inbox: Arc<Mutex<Receiver<(VerifyingKey, Message)>>>,
}

impl Network for NetworkStub {
    fn init_validator_set(&mut self, _: ValidatorSet) {}

    fn update_validator_set(&mut self, _: ValidatorSetUpdates) {}

    fn send(&mut self, peer: VerifyingKey, message: Message) {
        if let Some(peer) = self.all_peers.get(&peer) {
            let _ = peer.send((self.my_verifying_key, message));
        }
    }

    fn broadcast(&mut self, message: Message) {
        for peer in self.all_peers.values() {
            let _ = peer.send((self.my_verifying_key, message.clone()));
        }
    }

    fn recv(&mut self) -> Option<(VerifyingKey, Message)> {
        let Ok(inbox) = self.inbox.lock() else {
            return None;
        };
        inbox.try_recv().ok()
    }
}

fn mock_network(peers: impl Iterator<Item = VerifyingKey>) -> Vec<NetworkStub> {
    let mut all_peers = HashMap::new();
    let peer_and_inboxes: Vec<_> = peers
        .map(|peer| {
            let (sender, receiver) = mpsc::channel();
            all_peers.insert(peer, sender);
            (peer, receiver)
        })
        .collect();

    peer_and_inboxes
        .into_iter()
        .map(|(my_verifying_key, inbox)| NetworkStub {
            my_verifying_key,
            all_peers: all_peers.clone(),
            inbox: Arc::new(Mutex::new(inbox)),
        })
        .collect()
}

#[derive(Debug, Serialize)]
struct SpotBlockExecutionSummary {
    status: &'static str,
    command_count: usize,
    applied_count: usize,
    rejected_count: usize,
    replayable_event_count: usize,
    place_command_count: usize,
    cancel_command_count: usize,
    place_party_id: Option<String>,
    place_asset: Option<u32>,
    place_order_id: Option<String>,
    place_cloid: Option<String>,
    cancel_party_id: Option<String>,
    cancel_asset: Option<u32>,
    cancel_lookup: Option<CancelSpotOrderV2Lookup>,
}

#[derive(Clone)]
struct SpotOrderApp {
    request_queue: Arc<Mutex<Vec<SpotBlockCommand>>>,
}

impl SpotOrderApp {
    fn new(request_queue: Arc<Mutex<Vec<SpotBlockCommand>>>) -> Self {
        Self { request_queue }
    }

    fn initial_app_state() -> AppStateUpdates {
        AppStateUpdates::new()
    }

    fn execute(requests: &[SpotBlockCommand]) -> Option<AppStateUpdates> {
        if requests.is_empty() {
            return None;
        }

        let spot_block_command = SpotBlockCmd { commands: requests.to_vec() };
        let outbound = DemoSpotBlockOutbound;
        let Ok(result) = execute_spot_block_with_outbound(&spot_block_command, &outbound) else {
            return None;
        };

        let summary = SpotBlockExecutionSummary::from_result(requests, &result);
        let Ok(value) = serde_json::to_vec(&summary) else {
            return None;
        };

        let mut updates = AppStateUpdates::new();
        for request in requests {
            match request {
                SpotBlockCommand::PlaceMatch(command) => {
                    updates.insert(place_result_key(command), value.clone());
                }
                SpotBlockCommand::Cancel(command) => {
                    updates.insert(cancel_result_key(command), value.clone());
                }
                SpotBlockCommand::Modify(_) | SpotBlockCommand::Match(_) => {}
            }
        }

        Some(updates)
    }
}

impl SpotBlockExecutionSummary {
    fn from_result(
        requests: &[SpotBlockCommand],
        result: &cmd_handler::command_use_case_def2::ExecutionResult<SpotBlockChanges>,
    ) -> Self {
        let applied_count = result
            .changes
            .item_results
            .iter()
            .filter(|item| matches!(item, SpotBlockItemResult::Applied { .. }))
            .count();
        let rejected_count = result
            .changes
            .item_results
            .iter()
            .filter(|item| matches!(item, SpotBlockItemResult::Rejected { .. }))
            .count();
        let place_command_count = result
            .changes
            .item_results
            .iter()
            .filter(|item| {
                matches!(
                    item,
                    SpotBlockItemResult::Applied {
                        changes: SpotBlockAppliedChanges::PlaceMatch(_),
                        ..
                    } | SpotBlockItemResult::Rejected {
                        command: SpotBlockCommand::PlaceMatch(_),
                        ..
                    }
                )
            })
            .count();
        let cancel_command_count = result
            .changes
            .item_results
            .iter()
            .filter(|item| {
                matches!(
                    item,
                    SpotBlockItemResult::Applied {
                        changes: SpotBlockAppliedChanges::Cancel(_),
                        ..
                    } | SpotBlockItemResult::Rejected { command: SpotBlockCommand::Cancel(_), .. }
                )
            })
            .count();

        let first_place = requests.iter().find_map(|request| match request {
            SpotBlockCommand::PlaceMatch(command) => place_order_cmd(command),
            SpotBlockCommand::Cancel(_)
            | SpotBlockCommand::Modify(_)
            | SpotBlockCommand::Match(_) => None,
        });
        let first_cancel = requests.iter().find_map(|request| match request {
            SpotBlockCommand::Cancel(command) => Some(command),
            SpotBlockCommand::PlaceMatch(_)
            | SpotBlockCommand::Modify(_)
            | SpotBlockCommand::Match(_) => None,
        });

        Self {
            status: "executed",
            command_count: result.changes.item_results.len(),
            applied_count,
            rejected_count,
            replayable_event_count: result.events.len(),
            place_command_count,
            cancel_command_count,
            place_party_id: first_place.map(|command| command.party_id.clone()),
            place_asset: first_place.map(|command| command.asset),
            place_order_id: first_place.map(|command| command.order_id.clone()),
            place_cloid: first_place.and_then(|command| command.cloid.clone()),
            cancel_party_id: first_cancel.map(|command| command.party_id.clone()),
            cancel_asset: first_cancel.map(|command| command.asset),
            cancel_lookup: first_cancel.map(|command| command.lookup.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DemoSpotBlockOutboundError;

impl std::fmt::Display for DemoSpotBlockOutboundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "demo spot block outbound error")
    }
}

impl std::error::Error for DemoSpotBlockOutboundError {}

#[derive(Debug, Default)]
struct DemoSpotBlockOutbound;

impl StateSource<SpotBlockUseCase> for DemoSpotBlockOutbound {
    type Error = DemoSpotBlockOutboundError;

    fn load_given_state(&self, _request: &SpotBlockCmd) -> Result<SpotBlockState, Self::Error> {
        let cancel_order = demo_buy_order()?;
        let buyer_frozen = cancel_order
            .reservation
            .remaining_amount
            .checked_add(cancel_order.fee_reservation.remaining_amount)
            .ok_or(DemoSpotBlockOutboundError)?;

        Ok(SpotBlockState {
            orders: vec![demo_sell_order("maker-1", "seller", 100, 1)?, cancel_order],
            balances: vec![
                Balance::new("buyer".to_string(), "USDT".to_string(), 100_000, buyer_frozen, 1),
                Balance::new("buyer".to_string(), "BTC".to_string(), 0, 0, 1),
                Balance::new("seller".to_string(), "BTC".to_string(), 0, 1, 1),
                Balance::new("seller".to_string(), "USDT".to_string(), 0, 1, 1),
                Balance::new("fee".to_string(), "USDT".to_string(), 0, 0, 1),
            ],
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        })
    }
}

impl StateSink<SpotBlockUseCase> for DemoSpotBlockOutbound {
    type Error = DemoSpotBlockOutboundError;

    fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn demo_sell_order(
    order_id: &str,
    account_id: &str,
    price: u64,
    qty: u64,
) -> Result<SpotOrderV2, DemoSpotBlockOutboundError> {
    let reservation = SpotOrderV2::principal_reservation(
        order_id,
        account_id,
        SpotOrderSide::Sell,
        qty,
        price,
        "BTC",
        "USDT",
    )
    .map_err(|_| DemoSpotBlockOutboundError)?;

    Ok(SpotOrderV2::new(
        order_id.to_string(),
        10_001,
        Some(price),
        account_id.to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        price,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        qty,
        0,
        SpotOrderStatus::Open,
        None,
        reservation,
        None,
        1,
    ))
}

fn demo_buy_order() -> Result<SpotOrderV2, DemoSpotBlockOutboundError> {
    SpotOrderV2::new_active(
        "cancel-buy".to_string(),
        10_001,
        Some(77738308),
        "buyer".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        100,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        2,
        "BTC",
        "USDT",
        5,
        10,
        Some("demo-cancel-1".to_string()),
    )
    .map_err(|_| DemoSpotBlockOutboundError)
}

impl App<MemDB> for SpotOrderApp {
    fn produce_block(&mut self, _request: ProduceBlockRequest<MemDB>) -> ProduceBlockResponse {
        let requests = self
            .request_queue
            .lock()
            .map(|mut queue| queue.drain(..).collect::<Vec<_>>())
            .unwrap_or_default();
        let data = encode_requests(&requests).unwrap_or_else(|_| Data::new(Vec::new()));
        let data_hash = data_hash(&data);
        let app_state_updates = Self::execute(&requests);

        ProduceBlockResponse { data_hash, data, app_state_updates, validator_set_updates: None }
    }

    fn validate_block(&mut self, request: ValidateBlockRequest<MemDB>) -> ValidateBlockResponse {
        self.validate_block_for_sync(request)
    }

    fn validate_block_for_sync(
        &mut self,
        request: ValidateBlockRequest<MemDB>,
    ) -> ValidateBlockResponse {
        let block = request.proposed_block();
        if block.data_hash != data_hash(&block.data) {
            return ValidateBlockResponse::Invalid;
        }

        let Ok(requests) = decode_requests(&block.data) else {
            return ValidateBlockResponse::Invalid;
        };

        ValidateBlockResponse::Valid {
            app_state_updates: Self::execute(&requests),
            validator_set_updates: None,
        }
    }
}

fn encode_requests(requests: &[SpotBlockCommand]) -> Result<Data, serde_json::Error> {
    let payload = serde_json::to_vec(requests)?;
    Ok(Data::new(vec![Datum::new(payload)]))
}

fn decode_requests(data: &Data) -> Result<Vec<SpotBlockCommand>, serde_json::Error> {
    let Some(datum) = data.vec().first() else {
        return Ok(Vec::new());
    };
    serde_json::from_slice(datum.bytes())
}

fn data_hash(data: &Data) -> CryptoHash {
    let mut hasher = CryptoHasher::new();
    for datum in data.iter() {
        hasher.update(datum.bytes());
    }
    CryptoHash::new(hasher.finalize().into())
}

fn cancel_result_key(command: &CancelSpotOrderV2Cmd) -> Vec<u8> {
    match &command.lookup {
        CancelSpotOrderV2Lookup::Oid(oid) => {
            format!("cancel:{}:{oid}", command.party_id).into_bytes()
        }
        CancelSpotOrderV2Lookup::Cloid(cloid) => {
            format!("cancel:{}:{cloid}", command.party_id).into_bytes()
        }
        CancelSpotOrderV2Lookup::Missing => {
            format!("cancel:{}:missing", command.party_id).into_bytes()
        }
    }
}

fn place_result_key(command: &PlaceOnlySpotOrderV2Cmd) -> Vec<u8> {
    let Some(order) = place_order_cmd(command) else {
        return b"place:missing:missing".to_vec();
    };
    format!(
        "place:{}:{}",
        order.party_id,
        order.cloid.as_deref().unwrap_or(order.order_id.as_str())
    )
    .into_bytes()
}

fn place_order_cmd(command: &PlaceOnlySpotOrderV2Cmd) -> Option<&PlaceOnlySpotOrderV2OrderCmd> {
    match command {
        PlaceOnlySpotOrderV2Cmd::Single(order) => Some(order),
        PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, .. } => Some(parent),
    }
}

fn get_from_snapshot<S: KVGet>(snapshot: &BlockTreeSnapshot<S>, key: &[u8]) -> Option<Vec<u8>> {
    snapshot.committed_app_state(key)
}

fn lock_or_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn main() -> DemoResult<()> {
    println!("[hotstuff_use_case_demo] 启动 3 节点 HotStuff spot order use case demo");

    let mut rng = rand_core::OsRng;
    let signing_keys: Vec<SigningKey> =
        (0..NODE_COUNT).map(|_| SigningKey::generate(&mut rng)).collect();
    let verifying_keys: Vec<VerifyingKey> =
        signing_keys.iter().map(SigningKey::verifying_key).collect();
    println!("[hotstuff_use_case_demo] 生成 {} 个验证者密钥", verifying_keys.len());

    let network_stubs = mock_network(verifying_keys.iter().copied());

    let mut initial_validator_set = ValidatorSet::new();
    for verifying_key in &verifying_keys {
        initial_validator_set.put(verifying_key, Power::new(1));
    }
    let init_vs_state =
        ValidatorSetState::new(initial_validator_set.clone(), initial_validator_set, None, true);

    let initial_app_state = SpotOrderApp::initial_app_state();
    let request_queues: Vec<Arc<Mutex<Vec<SpotBlockCommand>>>> =
        (0..NODE_COUNT).map(|_| Arc::new(Mutex::new(Vec::new()))).collect();
    let inserted_payload_blocks = Arc::new(Mutex::new(0usize));
    let committed_blocks = Arc::new(Mutex::new(0usize));
    let mut replicas: Vec<Replica<MemDB>> = Vec::new();

    for index in 0..NODE_COUNT {
        let kv_store = MemDB::new();
        Replica::initialize(kv_store.clone(), initial_app_state.clone(), init_vs_state.clone());

        let configuration = Configuration::builder()
            .me(signing_keys[index].clone())
            .chain_id(ChainID::new(7))
            .block_sync_request_limit(10)
            .block_sync_server_advertise_time(Duration::from_secs(10))
            .block_sync_response_timeout(Duration::from_secs(3))
            .block_sync_blacklist_expiry_time(Duration::from_secs(10))
            .block_sync_trigger_min_view_difference(2)
            .block_sync_trigger_timeout(Duration::from_secs(60))
            .progress_msg_buffer_capacity(BufferSize::new(1024))
            .epoch_length(EpochLength::new(50))
            .max_view_time(Duration::from_millis(1500))
            .log_events(false)
            .build();

        let inserted_payload_blocks_for_handler = Arc::clone(&inserted_payload_blocks);
        let committed_blocks_for_handler = Arc::clone(&committed_blocks);
        let replica = ReplicaSpec::builder()
            .app(SpotOrderApp::new(Arc::clone(&request_queues[index])))
            .network(network_stubs[index].clone())
            .kv_store(kv_store)
            .configuration(configuration)
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
