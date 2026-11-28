use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use common_entity::{LogEntry, LogEntrySequencer, ReplayableEntityChanges};
use ed25519_dalek::{SigningKey, VerifyingKey};
use example_core_use_case::entity::account::coin::Coin;
use example_core_use_case::entity::account::symbol::Symbol;
use example_core_use_case::entity::common::identifiers::OrderId;
use example_core_use_case::{
    BalanceV2, CancelSpotOrderFsmV3Cmd, OrderGroupRelation, OrderSide, OrderTif, OrderType,
    PlaceSpotOrderPipelineV2Cmd,
};
use example_outbound_adapter::database::{
    State4BuildBlock, decode_build_block_state, encode_build_block_state,
};
use hotstuff_rs::app::{
    App, ProduceBlockRequest, ProduceBlockResponse, ValidateBlockRequest, ValidateBlockResponse,
};
use hotstuff_rs::block_tree::accessors::public::BlockTreeCamera;
use hotstuff_rs::block_tree::pluggables::{KVGet, KVStore, WriteBatch};
use hotstuff_rs::networking::messages::Message;
use hotstuff_rs::networking::network::Network;
use hotstuff_rs::replica::{Configuration, Replica, ReplicaSpec};
use hotstuff_rs::types::crypto_primitives::{CryptoHasher, Digest};
use hotstuff_rs::types::data_types::{
    BlockHeight, BufferSize, ChainID, CryptoHash, Data, Datum, EpochLength, Power,
};
use hotstuff_rs::types::update_sets::{AppStateUpdates, ValidatorSetUpdates};
use hotstuff_rs::types::validator_set::{ValidatorSet, ValidatorSetState};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::Notify;
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCommandExecutionV3, SpotBlockCommandV2, build_block_spot_v3,
};

const REPLICA_COUNT: usize = 3;
const CANONICAL_STATE_KEY: &[u8] = b"kiss:v3:canonical-state";
const RESULT_KEY_PREFIX: &[u8] = b"kiss:v3:result:";
const USER_ID: &str = "buyer";
const CLOID: &str = "kiss-v3-order-1";
const COMMAND_TIMEOUT: Duration = Duration::from_secs(15);
const LISTEN_ADDRESS: &str = "127.0.0.1:39004";

type DemoResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Clone, Default)]
struct MemStore(Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>);

impl KVGet for MemStore {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        lock_or_recover(&self.0).get(key).cloned()
    }
}

impl KVStore for MemStore {
    type WriteBatch = MemWriteBatch;
    type Snapshot<'a> = MemSnapshot<'a>;

    fn write(&mut self, batch: Self::WriteBatch) {
        let mut values = lock_or_recover(&self.0);
        for (key, value) in batch.insertions {
            values.insert(key, value);
        }
        for key in batch.deletions {
            values.remove(&key);
        }
    }

    fn clear(&mut self) {
        lock_or_recover(&self.0).clear();
    }

    fn snapshot(&self) -> Self::Snapshot<'_> {
        MemSnapshot(lock_or_recover(&self.0))
    }
}

struct MemWriteBatch {
    insertions: HashMap<Vec<u8>, Vec<u8>>,
    deletions: HashSet<Vec<u8>>,
}

impl WriteBatch for MemWriteBatch {
    fn new() -> Self {
        Self {
            insertions: HashMap::new(),
            deletions: HashSet::new(),
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.deletions.remove(key);
        self.insertions.insert(key.to_vec(), value.to_vec());
    }

    fn delete(&mut self, key: &[u8]) {
        self.insertions.remove(key);
        self.deletions.insert(key.to_vec());
    }
}

struct MemSnapshot<'a>(MutexGuard<'a, HashMap<Vec<u8>, Vec<u8>>>);

impl KVGet for MemSnapshot<'_> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.0.get(key).cloned()
    }
}

#[derive(Clone)]
struct MemoryNetwork {
    me: VerifyingKey,
    peers: HashMap<VerifyingKey, Sender<(VerifyingKey, Message)>>,
    inbox: Arc<Mutex<Receiver<(VerifyingKey, Message)>>>,
}

impl Network for MemoryNetwork {
    fn init_validator_set(&mut self, _validator_set: ValidatorSet) {}
    fn update_validator_set(&mut self, _updates: ValidatorSetUpdates) {}

    fn broadcast(&mut self, message: Message) {
        for peer in self.peers.values() {
            let _ = peer.send((self.me, message.clone()));
        }
    }

    fn send(&mut self, peer: VerifyingKey, message: Message) {
        if let Some(sender) = self.peers.get(&peer) {
            let _ = sender.send((self.me, message));
        }
    }

    fn recv(&mut self) -> Option<(VerifyingKey, Message)> {
        match lock_or_recover(&self.inbox).try_recv() {
            Ok(message) => Some(message),
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => None,
        }
    }
}

fn memory_network(keys: &[VerifyingKey]) -> Vec<MemoryNetwork> {
    let mut senders = HashMap::new();
    let mut inboxes = Vec::with_capacity(keys.len());
    for key in keys {
        let (sender, receiver) = mpsc::channel();
        senders.insert(*key, sender);
        inboxes.push((*key, receiver));
    }
    inboxes
        .into_iter()
        .map(|(me, inbox)| MemoryNetwork {
            me,
            peers: senders.clone(),
            inbox: Arc::new(Mutex::new(inbox)),
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CommandKind {
    Place,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ExecutionStatus {
    Applied,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Lifecycle {
    Pending,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StableExecutionResult {
    request_id: u64,
    command: CommandKind,
    execution_status: ExecutionStatus,
    error: Option<String>,
    replayable_event_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CompletedResult {
    request_id: u64,
    lifecycle: Lifecycle,
    command: CommandKind,
    execution_status: ExecutionStatus,
    error: Option<String>,
    replayable_event_count: usize,
    committed_block_hash: String,
    height: u64,
    confirming_replicas: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct PendingResult {
    request_id: u64,
    lifecycle: Lifecycle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
enum CommandResponse {
    Pending(PendingResult),
    Completed(CompletedResult),
}

struct RegistryEntry {
    result: Option<CompletedResult>,
    notify: Arc<Notify>,
}

#[derive(Default)]
struct ResultRegistry {
    next_request_id: AtomicU64,
    entries: Mutex<HashMap<u64, RegistryEntry>>,
}

impl ResultRegistry {
    fn allocate(&self) -> Result<u64, &'static str> {
        let request_id = self
            .next_request_id
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                value.checked_add(1)
            })
            .map_err(|_| "request id exhausted")?
            .checked_add(1)
            .ok_or("request id exhausted")?;
        lock_or_recover(&self.entries).insert(
            request_id,
            RegistryEntry {
                result: None,
                notify: Arc::new(Notify::new()),
            },
        );
        Ok(request_id)
    }

    fn get(&self, request_id: u64) -> Option<CommandResponse> {
        lock_or_recover(&self.entries)
            .get(&request_id)
            .map(|entry| {
                entry.result.clone().map_or_else(
                    || {
                        CommandResponse::Pending(PendingResult {
                            request_id,
                            lifecycle: Lifecycle::Pending,
                        })
                    },
                    CommandResponse::Completed,
                )
            })
    }

    fn complete(&self, result: CompletedResult) -> bool {
        let mut entries = lock_or_recover(&self.entries);
        let Some(entry) = entries.get_mut(&result.request_id) else {
            return false;
        };
        if entry.result.is_some() {
            return false;
        }
        entry.result = Some(result);
        entry.notify.notify_one();
        true
    }

    async fn wait(&self, request_id: u64, timeout: Duration) -> Option<CommandResponse> {
        let notify = {
            let entries = lock_or_recover(&self.entries);
            let entry = entries.get(&request_id)?;
            if let Some(result) = &entry.result {
                return Some(CommandResponse::Completed(result.clone()));
            }
            entry.notify.clone()
        };
        if tokio::time::timeout(timeout, notify.notified())
            .await
            .is_err()
        {
            return self.get(request_id);
        }
        self.get(request_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BrokeredCommand {
    request_id: u64,
    entry: LogEntry<SpotBlockCommandV2>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct ProposedBatch {
    commands: Vec<BrokeredCommand>,
}

#[derive(Default)]
struct PendingBroker {
    pending: BTreeMap<u64, BrokeredCommand>,
}

impl PendingBroker {
    fn enqueue(&mut self, request_id: u64, command: SpotBlockCommandV2) -> BrokeredCommand {
        let entry = LogEntrySequencer::from_metadata_with_topic(
            0,
            0,
            request_id,
            request_id,
            0,
            request_id,
            USER_ID.to_owned(),
            command,
        );
        let brokered = BrokeredCommand { request_id, entry };
        self.pending.insert(request_id, brokered.clone());
        brokered
    }

    fn proposed_batch(&self) -> ProposedBatch {
        ProposedBatch {
            commands: self.pending.values().cloned().collect(),
        }
    }

    fn committed(&mut self, request_id: u64) {
        self.pending.remove(&request_id);
    }
}

struct CommandSubmission {
    request_id: u64,
    command: SpotBlockCommandV2,
}

#[derive(Clone)]
struct SpotConsensusApp {
    broker: Arc<Mutex<PendingBroker>>,
    submissions: Arc<Mutex<Receiver<CommandSubmission>>>,
}

impl SpotConsensusApp {
    fn initial_app_state() -> DemoResult<AppStateUpdates> {
        let mut updates = AppStateUpdates::new();
        updates.insert(
            CANONICAL_STATE_KEY.to_vec(),
            encode_build_block_state(&initial_state())?,
        );
        Ok(updates)
    }

    fn execute(
        batch: &ProposedBatch,
        state_get: impl Fn(&[u8]) -> Option<Vec<u8>>,
    ) -> Result<Option<AppStateUpdates>, String> {
        if batch.commands.is_empty() {
            return Ok(None);
        }
        let state_bytes = state_get(CANONICAL_STATE_KEY)
            .ok_or_else(|| "canonical state is missing".to_owned())?;
        let mut state =
            decode_build_block_state(&state_bytes).map_err(|error| error.to_string())?;
        let mut updates = AppStateUpdates::new();
        let mut changed = false;
        for brokered in &batch.commands {
            let key = result_key(brokered.request_id);
            if state_get(&key).is_some() {
                continue;
            }
            let execution = build_block_spot_v3(std::slice::from_ref(&brokered.entry), &mut state)
                .map_err(|error| error.to_string())?;
            let result = project_result(brokered.request_id, &execution.command_results[0]);
            updates.insert(
                key,
                serde_json::to_vec(&result).map_err(|error| error.to_string())?,
            );
            changed = true;
        }
        if !changed {
            return Ok(None);
        }
        updates.insert(
            CANONICAL_STATE_KEY.to_vec(),
            encode_build_block_state(&state).map_err(|error| error.to_string())?,
        );
        Ok(Some(updates))
    }
}

impl App<MemStore> for SpotConsensusApp {
    fn produce_block(&mut self, request: ProduceBlockRequest<MemStore>) -> ProduceBlockResponse {
        while let Ok(submission) = lock_or_recover(&self.submissions).try_recv() {
            lock_or_recover(&self.broker).enqueue(submission.request_id, submission.command);
        }
        let batch = lock_or_recover(&self.broker).proposed_batch();
        if batch.commands.is_empty() {
            thread::sleep(Duration::from_millis(5));
        }
        let execution = Self::execute(&batch, |key| request.block_tree().app_state(key));
        let encoded = encode_batch(&batch);
        let (data, app_state_updates) = match (encoded, execution) {
            (Ok(data), Ok(updates)) => (data, updates),
            _ => (Data::new(Vec::new()), None),
        };
        ProduceBlockResponse {
            data_hash: data_hash(&data),
            data,
            app_state_updates,
            validator_set_updates: None,
        }
    }

    fn validate_block(&mut self, request: ValidateBlockRequest<MemStore>) -> ValidateBlockResponse {
        self.validate_block_for_sync(request)
    }

    fn validate_block_for_sync(
        &mut self,
        request: ValidateBlockRequest<MemStore>,
    ) -> ValidateBlockResponse {
        let block = request.proposed_block();
        if block.data_hash != data_hash(&block.data) {
            return ValidateBlockResponse::Invalid;
        }
        let Ok(batch) = decode_batch(&block.data) else {
            return ValidateBlockResponse::Invalid;
        };
        match Self::execute(&batch, |key| request.block_tree().app_state(key)) {
            Ok(app_state_updates) => ValidateBlockResponse::Valid {
                app_state_updates,
                validator_set_updates: None,
            },
            Err(_) => ValidateBlockResponse::Invalid,
        }
    }
}

fn project_result(
    request_id: u64,
    execution: &SpotBlockCommandExecutionV3,
) -> StableExecutionResult {
    match execution {
        SpotBlockCommandExecutionV3::Place { result, .. } => StableExecutionResult {
            request_id,
            command: CommandKind::Place,
            execution_status: if result.is_ok() {
                ExecutionStatus::Applied
            } else {
                ExecutionStatus::Rejected
            },
            error: result.as_ref().err().map(|error| format!("{error:?}")),
            replayable_event_count: result
                .as_ref()
                .map_or(0, |value| value.state_changed.replayable_events().len()),
        },
        SpotBlockCommandExecutionV3::Cancel { result, .. } => StableExecutionResult {
            request_id,
            command: CommandKind::Cancel,
            execution_status: if result.is_ok() {
                ExecutionStatus::Applied
            } else {
                ExecutionStatus::Rejected
            },
            error: result.as_ref().err().map(|error| format!("{error:?}")),
            replayable_event_count: result
                .as_ref()
                .map_or(0, |value| value.state_changed.replayable_events().len()),
        },
    }
}

#[derive(Clone)]
struct CommitNotice {
    replica_index: usize,
    block_hash: CryptoHash,
    height: BlockHeight,
}

struct Confirmation {
    block_hash: CryptoHash,
    height: BlockHeight,
    result: StableExecutionResult,
    replicas: HashSet<usize>,
}

impl Confirmation {
    fn matches(
        &self,
        block_hash: &CryptoHash,
        height: BlockHeight,
        result: &StableExecutionResult,
    ) -> bool {
        self.block_hash == *block_hash && self.height == height && self.result == *result
    }

    fn is_fully_confirmed(&self) -> bool {
        self.replicas.len() == REPLICA_COUNT
    }
}

fn coordinate(
    commit_rx: Receiver<CommitNotice>,
    shutdown_rx: Receiver<()>,
    cameras: Vec<BlockTreeCamera<MemStore>>,
    broker: Arc<Mutex<PendingBroker>>,
    registry: Arc<ResultRegistry>,
    stopped: Arc<AtomicBool>,
) {
    let mut confirmations = HashMap::<u64, Vec<Confirmation>>::new();
    loop {
        match shutdown_rx.try_recv() {
            Ok(()) => {
                stopped.store(true, Ordering::Release);
                break;
            }
            Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }
        match commit_rx.recv_timeout(Duration::from_millis(10)) {
            Ok(notice) => observe_commit(notice, &cameras, &broker, &registry, &mut confirmations),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn observe_commit(
    notice: CommitNotice,
    cameras: &[BlockTreeCamera<MemStore>],
    broker: &Arc<Mutex<PendingBroker>>,
    registry: &Arc<ResultRegistry>,
    confirmations: &mut HashMap<u64, Vec<Confirmation>>,
) {
    let snapshot = cameras[notice.replica_index].snapshot();
    let pending_ids: Vec<_> = lock_or_recover(broker).pending.keys().copied().collect();
    for request_id in pending_ids {
        let Some(bytes) = snapshot.committed_app_state(&result_key(request_id)) else {
            continue;
        };
        let Ok(result) = serde_json::from_slice::<StableExecutionResult>(&bytes) else {
            continue;
        };
        let candidates = confirmations.entry(request_id).or_default();
        let candidate_index = candidates
            .iter()
            .position(|candidate| candidate.matches(&notice.block_hash, notice.height, &result));
        let candidate_index = candidate_index.unwrap_or_else(|| {
            candidates.push(Confirmation {
                block_hash: notice.block_hash.clone(),
                height: notice.height,
                result: result.clone(),
                replicas: HashSet::new(),
            });
            candidates.len() - 1
        });
        let confirmation = &mut candidates[candidate_index];
        confirmation.replicas.insert(notice.replica_index);
        if confirmation.is_fully_confirmed() {
            let completed = CompletedResult {
                request_id,
                lifecycle: Lifecycle::Completed,
                command: result.command,
                execution_status: result.execution_status,
                error: result.error.clone(),
                replayable_event_count: result.replayable_event_count,
                committed_block_hash: format!("{:?}", notice.block_hash),
                height: notice.height.int(),
                confirming_replicas: REPLICA_COUNT,
            };
            registry.complete(completed);
            lock_or_recover(broker).committed(request_id);
            confirmations.remove(&request_id);
        }
    }
}

struct DemoCluster {
    sender: Sender<CommandSubmission>,
    registry: Arc<ResultRegistry>,
    shutdown: Sender<()>,
    coordinator: Option<JoinHandle<()>>,
    replicas: Vec<Replica<MemStore>>,
    coordinator_stopped: Arc<AtomicBool>,
    #[cfg(test)]
    cameras: Vec<BlockTreeCamera<MemStore>>,
}

impl DemoCluster {
    fn start() -> DemoResult<Self> {
        let signing_keys: Vec<_> = (1..=REPLICA_COUNT)
            .map(|seed| SigningKey::from_bytes(&[seed as u8; 32]))
            .collect();
        let verifying_keys: Vec<_> = signing_keys.iter().map(SigningKey::verifying_key).collect();
        let networks = memory_network(&verifying_keys);
        let validator_set = validator_set_state(&verifying_keys);
        let initial_state = SpotConsensusApp::initial_app_state()?;
        let broker = Arc::new(Mutex::new(PendingBroker::default()));
        let registry = Arc::new(ResultRegistry::default());
        let (submission_tx, submission_rx) = mpsc::channel::<CommandSubmission>();
        let submissions = Arc::new(Mutex::new(submission_rx));
        let stores: Vec<_> = (0..REPLICA_COUNT).map(|_| MemStore::default()).collect();
        let cameras: Vec<_> = stores.iter().cloned().map(BlockTreeCamera::new).collect();
        for store in &stores {
            Replica::initialize(store.clone(), initial_state.clone(), validator_set.clone());
        }
        let (commit_tx, commit_rx) = mpsc::channel();
        let mut replicas = Vec::with_capacity(REPLICA_COUNT);
        for (replica_index, ((signing_key, network), store)) in signing_keys
            .into_iter()
            .zip(networks)
            .zip(stores)
            .enumerate()
        {
            let commit_tx = commit_tx.clone();
            let commit_camera = cameras[replica_index].clone();
            replicas.push(
                ReplicaSpec::builder()
                    .app(SpotConsensusApp {
                        broker: broker.clone(),
                        submissions: submissions.clone(),
                    })
                    .network(network)
                    .kv_store(store)
                    .configuration(configuration(signing_key))
                    .on_commit_block(move |event| {
                        let snapshot = commit_camera.snapshot();
                        if let Ok(Some(height)) = snapshot.block_height(&event.block) {
                            let _ = commit_tx.send(CommitNotice {
                                replica_index,
                                block_hash: event.block.clone(),
                                height,
                            });
                        }
                    })
                    .build()
                    .start(),
            );
        }
        drop(commit_tx);
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        let coordinator_registry = registry.clone();
        let coordinator_cameras = cameras.clone();
        let coordinator_stopped = Arc::new(AtomicBool::new(false));
        let stopped = coordinator_stopped.clone();
        let coordinator = thread::spawn(move || {
            coordinate(
                commit_rx,
                shutdown_rx,
                coordinator_cameras,
                broker,
                coordinator_registry,
                stopped,
            );
        });
        Ok(Self {
            sender: submission_tx,
            registry,
            shutdown: shutdown_tx,
            coordinator: Some(coordinator),
            replicas,
            coordinator_stopped,
            #[cfg(test)]
            cameras,
        })
    }

    fn http_state(&self) -> HttpState {
        HttpState {
            sender: self.sender.clone(),
            registry: self.registry.clone(),
        }
    }

    #[cfg(test)]
    fn committed_states(&self) -> DemoResult<Vec<State4BuildBlock>> {
        self.cameras
            .iter()
            .map(|camera| {
                let bytes = camera
                    .snapshot()
                    .committed_app_state(CANONICAL_STATE_KEY)
                    .ok_or("canonical state is missing")?;
                decode_build_block_state(&bytes).map_err(Into::into)
            })
            .collect()
    }
}

impl Drop for DemoCluster {
    fn drop(&mut self) {
        let _ = self.shutdown.send(());
        if let Some(coordinator) = self.coordinator.take() {
            let _ = coordinator.join();
        }
        debug_assert!(self.coordinator_stopped.load(Ordering::Acquire));
        self.replicas.clear();
    }
}

#[derive(Clone)]
struct HttpState {
    sender: Sender<CommandSubmission>,
    registry: Arc<ResultRegistry>,
}

impl HttpState {
    fn submit(&self, command: SpotBlockCommandV2) -> Result<u64, StatusCode> {
        let request_id = self
            .registry
            .allocate()
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        if self
            .sender
            .send(CommandSubmission {
                request_id,
                command,
            })
            .is_err()
        {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        Ok(request_id)
    }
}

async fn submit_sync(
    State(state): State<HttpState>,
    Json(command): Json<SpotBlockCommandV2>,
) -> Result<(StatusCode, Json<CommandResponse>), StatusCode> {
    let request_id = state.submit(command)?;
    let response = state
        .registry
        .wait(request_id, COMMAND_TIMEOUT)
        .await
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let status = match response {
        CommandResponse::Pending(_) => StatusCode::GATEWAY_TIMEOUT,
        CommandResponse::Completed(_) => StatusCode::OK,
    };
    Ok((status, Json(response)))
}

async fn submit_async(
    State(state): State<HttpState>,
    Json(command): Json<SpotBlockCommandV2>,
) -> Result<(StatusCode, Json<CommandResponse>), StatusCode> {
    let request_id = state.submit(command)?;
    let response = state
        .registry
        .get(request_id)
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    Ok((StatusCode::ACCEPTED, Json(response)))
}

async fn get_result(
    State(state): State<HttpState>,
    Path(request_id): Path<u64>,
) -> Result<Json<CommandResponse>, StatusCode> {
    state
        .registry
        .get(request_id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

fn router(state: HttpState) -> Router {
    Router::new()
        .route("/spot-block/commands", post(submit_sync))
        .route("/spot-block/commands/async", post(submit_async))
        .route("/spot-block/commands/:request_id", get(get_result))
        .with_state(state)
}

fn result_key(request_id: u64) -> Vec<u8> {
    let mut key = RESULT_KEY_PREFIX.to_vec();
    key.extend_from_slice(&request_id.to_be_bytes());
    key
}

fn encode_batch(batch: &ProposedBatch) -> Result<Data, serde_json::Error> {
    if batch.commands.is_empty() {
        return Ok(Data::new(Vec::new()));
    }
    Ok(Data::new(vec![Datum::new(serde_json::to_vec(batch)?)]))
}

fn decode_batch(data: &Data) -> Result<ProposedBatch, serde_json::Error> {
    let Some(datum) = data.vec().first() else {
        return Ok(ProposedBatch::default());
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

fn configuration(signing_key: SigningKey) -> Configuration {
    Configuration::builder()
        .me(signing_key)
        .chain_id(ChainID::new(28))
        .block_sync_request_limit(10)
        .block_sync_server_advertise_time(Duration::from_secs(10))
        .block_sync_response_timeout(Duration::from_secs(3))
        .block_sync_blacklist_expiry_time(Duration::from_secs(10))
        .block_sync_trigger_min_view_difference(2)
        .block_sync_trigger_timeout(Duration::from_secs(60))
        .progress_msg_buffer_capacity(BufferSize::new(1024))
        .epoch_length(EpochLength::new(50))
        .max_view_time(Duration::from_millis(1_500))
        .log_events(false)
        .build()
}

fn validator_set_state(keys: &[VerifyingKey]) -> ValidatorSetState {
    let mut set = ValidatorSet::new();
    for key in keys {
        set.put(key, Power::new(1));
    }
    ValidatorSetState::new(set.clone(), set, None, true)
}

fn symbol() -> Symbol {
    Symbol::new(1, Coin::BTC, Coin::USDT)
}

fn initial_state() -> State4BuildBlock {
    let symbol = symbol();
    let buyer = common_entity::stable_string_entity_id(USER_ID);
    State4BuildBlock {
        next_trade_id: 1,
        maker_fee_bps: 5,
        taker_fee_bps: 10,
        spot_symbol_lob: [(symbol, LocalLobBtree::new(symbol))].into(),
        user_symbol_quote_balance: [(
            (example_core_use_case::PartyId::from(USER_ID), symbol),
            BalanceV2::new(
                example_core_use_case::PartyId::from(USER_ID),
                buyer,
                2,
                100_000,
                0,
                0,
                0,
            ),
        )]
        .into(),
        user_symbol_base_balance: [(
            (example_core_use_case::PartyId::from(USER_ID), symbol),
            BalanceV2::new(
                example_core_use_case::PartyId::from(USER_ID),
                buyer,
                0,
                10,
                0,
                0,
                0,
            ),
        )]
        .into(),
        user_symbol_trades: Default::default(),
        user_symbol_orders: Default::default(),
    }
}

fn place_command() -> SpotBlockCommandV2 {
    SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
        party_id: example_core_use_case::PartyId::from(USER_ID),
        order_id: 1,
        symbol: symbol(),
        side: OrderSide::Buy,
        price: Decimal::from(100),
        size: Decimal::from(2),
        order_type: OrderType::Limit { tif: OrderTif::Gtc },
        relation: OrderGroupRelation::Standalone,
        cloid: Some(CLOID.to_owned()),
        fee_bps: 10,
    })
}

fn cancel_command(cloid: &str) -> SpotBlockCommandV2 {
    SpotBlockCommandV2::Cancel(CancelSpotOrderFsmV3Cmd {
        order_id: OrderId::Cloid(cloid.to_owned()),
    })
}

fn lock_or_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn curl(command: &SpotBlockCommandV2, path: &str) -> DemoResult<String> {
    Ok(format!(
        "curl -sS -X POST http://{LISTEN_ADDRESS}{path} -H 'content-type: application/json' -d '{}'",
        serde_json::to_string(command)?
    ))
}

#[tokio::main]
async fn main() -> DemoResult<()> {
    let cluster = DemoCluster::start()?;
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await?;
    println!("HotStuff spot block KISS v3 listening on http://{LISTEN_ADDRESS}");
    println!(
        "place:  {}",
        curl(&place_command(), "/spot-block/commands")?
    );
    println!(
        "cancel: {}",
        curl(&cancel_command(CLOID), "/spot-block/commands")?
    );
    println!(
        "async:  {}",
        curl(&place_command(), "/spot-block/commands/async")?
    );
    axum::serve(listener, router(cluster.http_state())).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use example_core_use_case::SpotOrderStatus;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;

    async fn submit_and_wait(state: &HttpState, command: SpotBlockCommandV2) -> CompletedResult {
        let request_id = state.submit(command).unwrap();
        match state
            .registry
            .wait(request_id, COMMAND_TIMEOUT)
            .await
            .unwrap()
        {
            CommandResponse::Completed(result) => result,
            CommandResponse::Pending(_) => panic!("command did not complete"),
        }
    }

    #[test]
    fn memory_store_applies_batches_snapshots_deletes_and_clear() {
        let mut store = MemStore::default();
        let mut insert = MemWriteBatch::new();
        insert.set(b"key", b"value");
        store.write(insert);
        assert_eq!(store.get(b"key"), Some(b"value".to_vec()));
        assert_eq!(store.snapshot().get(b"key"), Some(b"value".to_vec()));

        let mut delete = MemWriteBatch::new();
        delete.delete(b"key");
        store.write(delete);
        assert_eq!(store.get(b"key"), None);

        let mut insert = MemWriteBatch::new();
        insert.set(b"key", b"value");
        store.write(insert);
        store.clear();
        assert_eq!(store.get(b"key"), None);
    }

    #[test]
    fn confirmation_requires_hash_height_and_result_to_all_match() {
        let result = StableExecutionResult {
            request_id: 1,
            command: CommandKind::Place,
            execution_status: ExecutionStatus::Applied,
            error: None,
            replayable_event_count: 1,
        };
        let hash = CryptoHash::new([1; 32]);
        let confirmation = Confirmation {
            block_hash: hash.clone(),
            height: BlockHeight::new(7),
            result: result.clone(),
            replicas: HashSet::new(),
        };
        assert!(confirmation.matches(&hash, BlockHeight::new(7), &result));
        assert!(!confirmation.matches(&CryptoHash::new([2; 32]), BlockHeight::new(7), &result));
        assert!(!confirmation.matches(&hash, BlockHeight::new(8), &result));
        assert!(!confirmation.matches(
            &hash,
            BlockHeight::new(7),
            &StableExecutionResult {
                execution_status: ExecutionStatus::Rejected,
                ..result
            }
        ));
    }

    #[test]
    fn confirmation_completes_at_exactly_three_distinct_replicas() {
        let mut confirmation = Confirmation {
            block_hash: CryptoHash::new([1; 32]),
            height: BlockHeight::new(7),
            result: StableExecutionResult {
                request_id: 1,
                command: CommandKind::Place,
                execution_status: ExecutionStatus::Applied,
                error: None,
                replayable_event_count: 1,
            },
            replicas: HashSet::new(),
        };
        assert!(!confirmation.is_fully_confirmed());
        confirmation.replicas.extend([0, 1]);
        assert!(!confirmation.is_fully_confirmed());
        confirmation.replicas.insert(2);
        assert!(confirmation.is_fully_confirmed());
        confirmation.replicas.insert(3);
        assert!(!confirmation.is_fully_confirmed());
    }

    #[test]
    fn broker_keeps_request_metadata_monotonic_until_commit() {
        let registry = ResultRegistry::default();
        let first_id = registry.allocate().unwrap();
        let second_id = registry.allocate().unwrap();
        let mut broker = PendingBroker::default();
        let first = broker.enqueue(first_id, place_command());
        let second = broker.enqueue(second_id, cancel_command(CLOID));
        assert_eq!(
            (
                first.request_id,
                first.entry.lsn(),
                first.entry.logical_ts()
            ),
            (1, 1, 1)
        );
        assert_eq!(
            (
                second.request_id,
                second.entry.lsn(),
                second.entry.logical_ts()
            ),
            (2, 2, 2)
        );
        assert_eq!(broker.proposed_batch().commands.len(), 2);
        broker.committed(first_id);
        assert_eq!(broker.proposed_batch().commands, vec![second]);
    }

    #[tokio::test]
    async fn place_and_cancel_complete_with_three_matching_replicas() {
        let cluster = DemoCluster::start().unwrap();
        let state = cluster.http_state();
        let place = submit_and_wait(&state, place_command()).await;
        let cancel = submit_and_wait(&state, cancel_command(CLOID)).await;
        assert_eq!(place.execution_status, ExecutionStatus::Applied);
        assert_eq!(cancel.execution_status, ExecutionStatus::Applied);
        assert_eq!(place.confirming_replicas, REPLICA_COUNT);
        assert_eq!(cancel.confirming_replicas, REPLICA_COUNT);
        let states = cluster.committed_states().unwrap();
        let encoded: Vec<_> = states
            .iter()
            .map(|state| encode_build_block_state(state).unwrap())
            .collect();
        assert!(encoded.windows(2).all(|pair| pair[0] == pair[1]));
        assert_eq!(
            states[0].user_symbol_orders
                [&(example_core_use_case::PartyId::from(USER_ID), symbol())][0]
                .order_base()
                .status,
            SpotOrderStatus::Canceled
        );
    }

    #[tokio::test]
    async fn rejected_command_is_a_completed_business_result() {
        let cluster = DemoCluster::start().unwrap();
        let result = submit_and_wait(&cluster.http_state(), cancel_command("missing")).await;
        assert_eq!(result.lifecycle, Lifecycle::Completed);
        assert_eq!(result.execution_status, ExecutionStatus::Rejected);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn async_query_and_sync_waiter_share_the_completed_result() {
        let cluster = DemoCluster::start().unwrap();
        let state = cluster.http_state();
        let request_id = state.submit(place_command()).unwrap();
        assert!(matches!(
            state.registry.get(request_id),
            Some(CommandResponse::Pending(_))
        ));
        let waited = state
            .registry
            .wait(request_id, COMMAND_TIMEOUT)
            .await
            .unwrap();
        assert_eq!(state.registry.get(request_id), Some(waited));
        assert_eq!(state.registry.get(u64::MAX), None);
    }

    #[tokio::test]
    async fn timeout_keeps_request_id_available_for_later_query() {
        let registry = ResultRegistry::default();
        let request_id = registry.allocate().unwrap();
        let response = registry.wait(request_id, Duration::ZERO).await.unwrap();
        assert_eq!(
            response,
            CommandResponse::Pending(PendingResult {
                request_id,
                lifecycle: Lifecycle::Pending,
            })
        );
        assert_eq!(registry.get(request_id), Some(response));
    }

    #[tokio::test]
    async fn http_handlers_report_pending_not_found_and_unavailable() {
        let (sender, receiver) = mpsc::channel();
        let state = HttpState {
            sender,
            registry: Arc::new(ResultRegistry::default()),
        };
        let (status, Json(response)) = submit_async(State(state.clone()), Json(place_command()))
            .await
            .unwrap();
        assert_eq!(status, StatusCode::ACCEPTED);
        assert!(matches!(response, CommandResponse::Pending(_)));
        assert_eq!(
            get_result(State(state.clone()), Path(u64::MAX))
                .await
                .unwrap_err(),
            StatusCode::NOT_FOUND
        );

        drop(receiver);
        assert_eq!(
            submit_async(State(state), Json(place_command()))
                .await
                .unwrap_err(),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[tokio::test]
    async fn router_serves_async_command_over_http() {
        let (sender, _receiver) = mpsc::channel();
        let state = HttpState {
            sender,
            registry: Arc::new(ResultRegistry::default()),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, router(state)).await.unwrap();
        });
        let body = serde_json::to_string(&place_command()).unwrap();
        let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
        let request = format!(
            "POST /spot-block/commands/async HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(request.as_bytes()).await.unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).await.unwrap();
        assert!(response.starts_with("HTTP/1.1 202 Accepted"));
        assert!(response.contains("\"lifecycle\":\"pending\""));
        server.abort();
    }

    #[test]
    fn dropping_cluster_synchronously_stops_the_coordinator() {
        let cluster = DemoCluster::start().unwrap();
        let stopped = cluster.coordinator_stopped.clone();
        drop(cluster);
        assert!(stopped.load(Ordering::Acquire));
    }

    #[test]
    fn completion_is_idempotent() {
        let registry = ResultRegistry::default();
        let request_id = registry.allocate().unwrap();
        let result = CompletedResult {
            request_id,
            lifecycle: Lifecycle::Completed,
            command: CommandKind::Place,
            execution_status: ExecutionStatus::Applied,
            error: None,
            replayable_event_count: 1,
            committed_block_hash: "hash".to_owned(),
            height: 7,
            confirming_replicas: REPLICA_COUNT,
        };
        assert!(registry.complete(result.clone()));
        assert!(!registry.complete(CompletedResult {
            height: 8,
            ..result.clone()
        }));
        assert_eq!(
            registry.get(request_id),
            Some(CommandResponse::Completed(result))
        );
    }
}
