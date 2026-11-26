use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;

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
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCommandExecutionV3, SpotBlockCommandV2, execute_spot_block_v3,
};

const REPLICA_COUNT: usize = 3;
const CANONICAL_STATE_KEY: &[u8] = b"kiss:v2:canonical-state";
const RESULT_KEY_PREFIX: &[u8] = b"kiss:v2:result:";
const USER_ID: &str = "buyer";
const CLOID: &str = "kiss-v2-order-1";
const COMMAND_TIMEOUT: Duration = Duration::from_secs(15);

type DemoResult<T> = Result<T, Box<dyn Error>>;

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
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => None,
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

struct CommandSubmission {
    command: SpotBlockCommandV2,
    reply: Sender<CommandExecutionReceipt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CommandKind {
    Place,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ExecutionStatus {
    Applied,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandExecutionReceipt {
    request_id: u64,
    command: CommandKind,
    status: ExecutionStatus,
    error: Option<String>,
    replayable_event_count: usize,
    committed_block_hash: CryptoHash,
    height: BlockHeight,
    confirming_replicas: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StableExecutionResult {
    request_id: u64,
    command: CommandKind,
    status: ExecutionStatus,
    error: Option<String>,
    replayable_event_count: usize,
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
    next_request_id: u64,
    pending: BTreeMap<u64, BrokeredCommand>,
}

impl PendingBroker {
    fn enqueue(&mut self, command: SpotBlockCommandV2) -> Result<BrokeredCommand, &'static str> {
        let request_id = self
            .next_request_id
            .checked_add(1)
            .ok_or("request id exhausted")?;
        self.next_request_id = request_id;
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
        Ok(brokered)
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

#[derive(Clone)]
struct SpotConsensusApp {
    broker: Arc<Mutex<PendingBroker>>,
    submissions: Arc<Mutex<Receiver<CommandSubmission>>>,
    replies: Arc<Mutex<HashMap<u64, Sender<CommandExecutionReceipt>>>>,
}

impl SpotConsensusApp {
    fn new(
        broker: Arc<Mutex<PendingBroker>>,
        submissions: Arc<Mutex<Receiver<CommandSubmission>>>,
        replies: Arc<Mutex<HashMap<u64, Sender<CommandExecutionReceipt>>>>,
    ) -> Self {
        Self {
            broker,
            submissions,
            replies,
        }
    }

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
            let execution =
                execute_spot_block_v3(std::slice::from_ref(&brokered.entry), &mut state)
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
            if let Ok(brokered) = lock_or_recover(&self.broker).enqueue(submission.command) {
                lock_or_recover(&self.replies).insert(brokered.request_id, submission.reply);
            }
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
            status: if result.is_ok() {
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
            status: if result.is_ok() {
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

fn coordinate(
    commit_rx: Receiver<CommitNotice>,
    shutdown_rx: Receiver<()>,
    cameras: Vec<BlockTreeCamera<MemStore>>,
    broker: Arc<Mutex<PendingBroker>>,
    replies: Arc<Mutex<HashMap<u64, Sender<CommandExecutionReceipt>>>>,
) {
    let mut confirmations = HashMap::<u64, Confirmation>::new();
    loop {
        if shutdown_rx.try_recv().is_ok() {
            break;
        }
        match commit_rx.recv_timeout(Duration::from_millis(10)) {
            Ok(notice) => observe_commit(notice, &cameras, &broker, &replies, &mut confirmations),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn observe_commit(
    notice: CommitNotice,
    cameras: &[BlockTreeCamera<MemStore>],
    broker: &Arc<Mutex<PendingBroker>>,
    replies: &Arc<Mutex<HashMap<u64, Sender<CommandExecutionReceipt>>>>,
    confirmations: &mut HashMap<u64, Confirmation>,
) {
    let snapshot = cameras[notice.replica_index].snapshot();
    let pending_ids: Vec<_> = lock_or_recover(broker).pending.keys().copied().collect();
    for request_id in pending_ids {
        let key = result_key(request_id);
        let Some(bytes) = snapshot.committed_app_state(&key) else {
            continue;
        };
        let Ok(result) = serde_json::from_slice::<StableExecutionResult>(&bytes) else {
            continue;
        };
        let confirmation = confirmations
            .entry(request_id)
            .or_insert_with(|| Confirmation {
                block_hash: notice.block_hash.clone(),
                height: notice.height,
                result: result.clone(),
                replicas: HashSet::new(),
            });
        if confirmation.block_hash != notice.block_hash
            || confirmation.height != notice.height
            || confirmation.result != result
        {
            continue;
        }
        confirmation.replicas.insert(notice.replica_index);
        if confirmation.replicas.len() == REPLICA_COUNT {
            lock_or_recover(broker).committed(request_id);
            if let Some(reply) = lock_or_recover(replies).remove(&request_id) {
                let _ = reply.send(CommandExecutionReceipt {
                    request_id: result.request_id,
                    command: result.command,
                    status: result.status,
                    error: result.error,
                    replayable_event_count: result.replayable_event_count,
                    committed_block_hash: notice.block_hash.clone(),
                    height: notice.height,
                    confirming_replicas: REPLICA_COUNT,
                });
            }
            confirmations.remove(&request_id);
        }
    }
}

struct DemoCluster {
    sender: Sender<CommandSubmission>,
    shutdown: Sender<()>,
    coordinator: Option<JoinHandle<()>>,
    replicas: Vec<Replica<MemStore>>,
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
        let (submission_tx, submission_rx) = mpsc::channel::<CommandSubmission>();
        let submissions = Arc::new(Mutex::new(submission_rx));
        let replies = Arc::new(Mutex::new(HashMap::new()));
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
                    .app(SpotConsensusApp::new(
                        broker.clone(),
                        submissions.clone(),
                        replies.clone(),
                    ))
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
        let coordinator_cameras = cameras.clone();
        let coordinator_broker = broker.clone();
        let coordinator = thread::spawn(move || {
            coordinate(
                commit_rx,
                shutdown_rx,
                coordinator_cameras,
                coordinator_broker,
                replies,
            );
        });
        Ok(Self {
            sender: submission_tx,
            shutdown: shutdown_tx,
            coordinator: Some(coordinator),
            replicas,
            #[cfg(test)]
            cameras,
        })
    }

    fn submit(&self, command: SpotBlockCommandV2) -> DemoResult<CommandExecutionReceipt> {
        let (reply, receiver) = mpsc::channel();
        self.sender.send(CommandSubmission { command, reply })?;
        receiver
            .recv_timeout(COMMAND_TIMEOUT)
            .map_err(|_| "timed out waiting for three matching commit confirmations".into())
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
        self.replicas.clear();
    }
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
        .chain_id(ChainID::new(27))
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
            (USER_ID.to_owned(), symbol),
            BalanceV2::new(buyer, buyer, 2, 100_000, 0, 0, 0),
        )]
        .into(),
        user_symbol_base_balance: [(
            (USER_ID.to_owned(), symbol),
            BalanceV2::new(buyer, buyer, 0, 10, 0, 0, 0),
        )]
        .into(),
        user_symbol_trades: Default::default(),
        user_symbol_orders: Default::default(),
    }
}

fn place_command() -> SpotBlockCommandV2 {
    SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
        party_id: common_entity::stable_string_entity_id(USER_ID),
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

fn main() -> DemoResult<()> {
    let cluster = DemoCluster::start()?;
    let place = cluster.submit(place_command())?;
    println!("committed place: {place:?}");
    let cancel = cluster.submit(cancel_command(CLOID))?;
    println!("committed cancel: {cancel:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use example_core_use_case::SpotOrderStatus;

    use super::*;

    #[test]
    fn broker_assigns_monotonic_metadata_and_keeps_pending_until_commit() {
        let mut broker = PendingBroker::default();
        let first = broker.enqueue(place_command()).unwrap();
        let second = broker.enqueue(cancel_command(CLOID)).unwrap();

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
        assert_eq!(broker.proposed_batch().commands.len(), 2);
        broker.committed(first.request_id);
        assert_eq!(broker.proposed_batch().commands, vec![second]);
    }

    #[test]
    fn result_projection_distinguishes_applied_and_rejected() {
        let mut state = initial_state();
        let mut broker = PendingBroker::default();
        let place = broker.enqueue(place_command()).unwrap();
        let rejected = broker.enqueue(cancel_command("missing")).unwrap();
        let execution =
            execute_spot_block_v3(&[place.entry.clone(), rejected.entry.clone()], &mut state)
                .unwrap();

        let applied = project_result(1, &execution.command_results[0]);
        let rejected = project_result(2, &execution.command_results[1]);
        assert_eq!(applied.status, ExecutionStatus::Applied);
        assert!(applied.replayable_event_count > 0);
        assert_eq!(applied.error, None);
        assert_eq!(rejected.status, ExecutionStatus::Rejected);
        assert_eq!(rejected.replayable_event_count, 0);
        assert!(rejected.error.is_some());
    }

    #[test]
    fn three_replicas_commit_place_then_cancel() {
        let cluster = DemoCluster::start().unwrap();
        let place = cluster.submit(place_command()).unwrap();
        let cancel = cluster.submit(cancel_command(CLOID)).unwrap();

        assert_eq!(place.status, ExecutionStatus::Applied);
        assert_eq!(cancel.status, ExecutionStatus::Applied);
        assert_eq!(place.confirming_replicas, REPLICA_COUNT);
        assert_eq!(cancel.confirming_replicas, REPLICA_COUNT);
        let states = cluster.committed_states().unwrap();
        let encoded: Vec<_> = states
            .iter()
            .map(|state| encode_build_block_state(state).unwrap())
            .collect();
        assert!(encoded.windows(2).all(|pair| pair[0] == pair[1]));
        assert_eq!(
            states[0].user_symbol_orders[&(USER_ID.to_owned(), symbol())][0]
                .order_base()
                .status,
            SpotOrderStatus::Canceled
        );
    }

    #[test]
    fn missing_order_cancel_commits_as_rejected_on_all_replicas() {
        let cluster = DemoCluster::start().unwrap();
        let receipt = cluster.submit(cancel_command("missing")).unwrap();

        assert_eq!(receipt.command, CommandKind::Cancel);
        assert_eq!(receipt.status, ExecutionStatus::Rejected);
        assert_eq!(receipt.confirming_replicas, REPLICA_COUNT);
        assert!(receipt.error.is_some());
        let states = cluster.committed_states().unwrap();
        let encoded: Vec<_> = states
            .iter()
            .map(|state| encode_build_block_state(state).unwrap())
            .collect();
        assert!(encoded.windows(2).all(|pair| pair[0] == pair[1]));
    }
}
