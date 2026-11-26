use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use ed25519_dalek::{SigningKey, VerifyingKey};
use example_core_use_case::entity::account::coin::Coin;
use example_core_use_case::entity::account::symbol::Symbol;
use example_core_use_case::entity::common::identifiers::OrderId;
use example_core_use_case::{
    BalanceV2, CancelSpotOrderFsmV3Cmd, OrderGroupRelation, OrderSide, OrderTif, OrderType,
    PlaceSpotOrderPipelineV2Cmd, SpotOrderStatus,
};
use example_outbound_adapter::database::State4BuildBlock;
use hotstuff_rs::app::{
    App, ProduceBlockRequest, ProduceBlockResponse, ValidateBlockRequest, ValidateBlockResponse,
};
use hotstuff_rs::block_tree::pluggables::{KVGet, KVStore, WriteBatch};
use hotstuff_rs::networking::messages::Message;
use hotstuff_rs::networking::network::Network;
use hotstuff_rs::replica::{Configuration, Replica, ReplicaSpec};
use hotstuff_rs::types::crypto_primitives::{CryptoHasher, Digest};
use hotstuff_rs::types::data_types::{
    BufferSize, ChainID, CryptoHash, Data, Datum, EpochLength, Power,
};
use hotstuff_rs::types::update_sets::{AppStateUpdates, ValidatorSetUpdates};
use hotstuff_rs::types::validator_set::{ValidatorSet, ValidatorSetState};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;
use rust_decimal::Decimal;
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCmds, SpotBlockCommandExecutionV3, SpotBlockCommandV2, execute_spot_block_v3,
};

const COMMITTED_RESULT_KEY: &[u8] = b"kiss:spot-block:committed";
const COMMITTED_RESULT_VALUE: &[u8] = b"place+cancel:canceled";
const USER_ID: &str = "buyer";
const CLOID: &str = "kiss-order-1";

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
struct LoopbackNetwork {
    me: VerifyingKey,
    sender: Sender<(VerifyingKey, Message)>,
    receiver: Arc<Mutex<Receiver<(VerifyingKey, Message)>>>,
}

impl LoopbackNetwork {
    fn new(me: VerifyingKey) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            me,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

impl Network for LoopbackNetwork {
    fn init_validator_set(&mut self, _validator_set: ValidatorSet) {}

    fn update_validator_set(&mut self, _updates: ValidatorSetUpdates) {}

    fn broadcast(&mut self, message: Message) {
        let _ = self.sender.send((self.me, message));
    }

    fn send(&mut self, peer: VerifyingKey, message: Message) {
        if peer == self.me {
            let _ = self.sender.send((self.me, message));
        }
    }

    fn recv(&mut self) -> Option<(VerifyingKey, Message)> {
        lock_or_recover(&self.receiver).try_recv().ok()
    }
}

struct KissApp {
    command: Arc<Mutex<Option<SpotBlockCmds>>>,
}

impl KissApp {
    fn new(command: SpotBlockCmds) -> Self {
        Self {
            command: Arc::new(Mutex::new(Some(command))),
        }
    }

    fn execute(command: &SpotBlockCmds) -> Result<State4BuildBlock, String> {
        let mut state = initial_state();
        let execution =
            execute_spot_block_v3(&command.items, &mut state).map_err(|error| error.to_string())?;

        let succeeded = matches!(
            execution.command_results.as_slice(),
            [
                SpotBlockCommandExecutionV3::Place { result: Ok(_), .. },
                SpotBlockCommandExecutionV3::Cancel { result: Ok(_), .. }
            ]
        );
        let canceled = state
            .user_symbol_orders
            .get(&(USER_ID.to_owned(), symbol()))
            .and_then(|orders| orders.first())
            .is_some_and(|order| order.order_base().status == SpotOrderStatus::Canceled);
        if !succeeded || !canceled {
            return Err("place/cancel block did not finish with a canceled order".to_owned());
        }
        Ok(state)
    }

    fn updates(command: &SpotBlockCmds) -> Result<AppStateUpdates, String> {
        Self::execute(command)?;
        let mut updates = AppStateUpdates::new();
        updates.insert(
            COMMITTED_RESULT_KEY.to_vec(),
            COMMITTED_RESULT_VALUE.to_vec(),
        );
        Ok(updates)
    }
}

impl App<MemStore> for KissApp {
    /// HotStuff 在本节点成为当前 view 的 Leader/Proposer 时调用的出块入口。
    ///
    /// 该方法从待处理队列中一次性取出一组 `Place + Cancel` 命令，并将编码后的
    /// `data` 作为参与共识、广播给验证者的区块载荷。`app_state_updates` 描述该区块
    /// 提交后才写入的状态变更，提案阶段不会立即成为 committed state。
    ///
    /// 队列为空时产生空块；业务执行失败时不生成成功状态更新，编码失败时则退化为
    /// 空载荷。`data_hash` 用于校验区块载荷完整性，验证者会在 `validate_block` 中对
    /// 同一组命令进行确定性重放，并重新计算待提交的状态更新。
    fn produce_block(&mut self, _request: ProduceBlockRequest<MemStore>) -> ProduceBlockResponse {
        let command = lock_or_recover(&self.command).take();
        let (data, app_state_updates) = command.map_or_else(
            || (Data::new(Vec::new()), None),
            |command| {
                let updates = Self::updates(&command).ok();
                let data = encode_command(&command).unwrap_or_else(|_| Data::new(Vec::new()));
                (data, updates)
            },
        );
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
        if block.data.vec().is_empty() {
            return ValidateBlockResponse::Valid {
                app_state_updates: None,
                validator_set_updates: None,
            };
        }
        let Ok(command) = decode_command(&block.data) else {
            return ValidateBlockResponse::Invalid;
        };
        let Ok(updates) = Self::updates(&command) else {
            return ValidateBlockResponse::Invalid;
        };
        ValidateBlockResponse::Valid {
            app_state_updates: Some(updates),
            validator_set_updates: None,
        }
    }
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

fn fixed_command() -> SpotBlockCmds {
    let place = SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
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
    });
    let cancel = SpotBlockCommandV2::Cancel(CancelSpotOrderFsmV3Cmd {
        order_id: OrderId::Cloid(CLOID.to_owned()),
    });
    SpotBlockCmds {
        items: vec![
            common_entity::LogEntrySequencer::from_metadata_with_topic(
                0,
                0,
                1,
                1_000,
                0,
                1,
                USER_ID.to_owned(),
                place,
            ),
            common_entity::LogEntrySequencer::from_metadata_with_topic(
                0,
                0,
                2,
                1_001,
                0,
                2,
                USER_ID.to_owned(),
                cancel,
            ),
        ],
    }
}

fn encode_command(command: &SpotBlockCmds) -> Result<Data, serde_json::Error> {
    Ok(Data::new(vec![Datum::new(serde_json::to_vec(command)?)]))
}

fn decode_command(data: &Data) -> Result<SpotBlockCmds, serde_json::Error> {
    let bytes = data
        .vec()
        .first()
        .map(|datum| datum.bytes().as_slice())
        .unwrap_or_default();
    serde_json::from_slice(bytes)
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
        .chain_id(ChainID::new(17))
        .block_sync_request_limit(10)
        .block_sync_server_advertise_time(Duration::from_secs(10))
        .block_sync_response_timeout(Duration::from_secs(3))
        .block_sync_blacklist_expiry_time(Duration::from_secs(10))
        .block_sync_trigger_min_view_difference(2)
        .block_sync_trigger_timeout(Duration::from_secs(60))
        .progress_msg_buffer_capacity(BufferSize::new(256))
        .epoch_length(EpochLength::new(50))
        .max_view_time(Duration::from_millis(1_500))
        .log_events(false)
        .build()
}

fn validator_set_state(key: VerifyingKey) -> ValidatorSetState {
    let mut set = ValidatorSet::new();
    set.put(&key, Power::new(1));
    ValidatorSetState::new(set.clone(), set, None, true)
}

fn lock_or_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn main() -> DemoResult<()> {
    println!("[KISS] initialize one-validator replica");
    let signing_key = SigningKey::from_bytes(&[7; 32]);
    let verifying_key = signing_key.verifying_key();
    let store = MemStore::default();
    Replica::initialize(
        store.clone(),
        AppStateUpdates::new(),
        validator_set_state(verifying_key),
    );
    let replica = ReplicaSpec::builder()
        .app(KissApp::new(fixed_command()))
        .network(LoopbackNetwork::new(verifying_key))
        .kv_store(store)
        .configuration(configuration(signing_key))
        .on_view_timeout(|event| {
            println!("[Pacemaker] view {} timeout", event.view.int());
        })
        .on_timeout_vote(|_| println!("[Pacemaker] broadcast timeout vote"))
        .on_receive_timeout_vote(|_| println!("[Pacemaker] receive timeout vote"))
        .on_collect_tc(|_| println!("[Pacemaker] form timeout certificate"))
        .on_advance_view(|_| println!("[Pacemaker] broadcast advance-view"))
        .on_receive_advance_view(|_| println!("[Pacemaker] receive advance-view"))
        .on_start_view(|event| println!("[HotStuff] enter view {}", event.view.int()))
        .on_propose(|event| {
            println!(
                "[HotStuff] propose block height={}",
                event.proposal.block.height.int()
            );
        })
        .on_phase_vote(|event| {
            println!("[HotStuff] vote phase={:?}", event.vote.phase);
        })
        .on_collect_pc(|event| {
            println!(
                "[HotStuff] quorum certificate phase={:?}",
                event.phase_certificate.phase
            );
        })
        .on_commit_block(|_| println!("[HotStuff] commit block"))
        .build()
        .start();
    println!("[KISS] replica started; wait for consensus");

    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        let committed = {
            let snapshot = replica.block_tree_camera().snapshot();
            snapshot.committed_app_state(COMMITTED_RESULT_KEY)
        };
        if committed.as_deref() == Some(COMMITTED_RESULT_VALUE) {
            println!("committed: place + cancel -> Canceled");
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err("timed out waiting for committed place/cancel result".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_and_cancel_succeed_in_one_block() {
        let command = fixed_command();
        let state = KissApp::execute(&command).expect("place and cancel should succeed");
        let orders = &state.user_symbol_orders[&(USER_ID.to_owned(), symbol())];

        assert_eq!(command.items.len(), 2);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_base().status, SpotOrderStatus::Canceled);
    }
}
