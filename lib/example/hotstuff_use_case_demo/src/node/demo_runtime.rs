use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use common_entity::ReplayableEntityChanges;
use ed25519_dalek::{SigningKey, VerifyingKey};
use example_core_use_case::BalanceV2;
use example_outbound_adapter::database::{
    State4BuildBlock, decode_build_block_state, encode_build_block_state,
};
use hotstuff_rs::app::{
    App, ProduceBlockRequest, ProduceBlockResponse, ValidateBlockRequest, ValidateBlockResponse,
};
use hotstuff_rs::block_tree::accessors::public::BlockTreeSnapshot;
use hotstuff_rs::block_tree::pluggables::{KVGet, KVStore, WriteBatch};
use hotstuff_rs::replica::Configuration;
use hotstuff_rs::types::crypto_primitives::{CryptoHasher, Digest};
use hotstuff_rs::types::data_types::{
    BufferSize, ChainID, CryptoHash, Data, Datum, EpochLength, Power,
};
use hotstuff_rs::types::update_sets::AppStateUpdates;
use hotstuff_rs::types::validator_set::{ValidatorSet, ValidatorSetState};
use serde::Serialize;
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCmds, SpotBlockCommandExecutionV3, SpotBlockCommandV2, build_block_spot_v3,
};

pub type DemoResult<T> = Result<T, Box<dyn std::error::Error>>;
pub const NODE_COUNT: usize = 3;
pub const PLACE_RESULT_KEY: &[u8] = b"spot-block:place";
pub const CANCEL_RESULT_KEY: &[u8] = b"spot-block:cancel";
pub const CANONICAL_STATE_KEY: &[u8] = b"spot-block:state:v1";

#[derive(Clone)]
pub struct MemDB(Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>);

impl MemDB {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}

impl Default for MemDB {
    fn default() -> Self {
        Self::new()
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

    fn snapshot(&self) -> MemDBSnapshot<'_> {
        MemDBSnapshot(lock_or_recover(&self.0))
    }
}

impl KVGet for MemDB {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.0.lock().ok().and_then(|map| map.get(key).cloned())
    }
}

pub struct MemWriteBatch {
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
        let _ = self.deletions.remove(key);
        self.insertions.insert(key.to_vec(), value.to_vec());
    }
    fn delete(&mut self, key: &[u8]) {
        let _ = self.insertions.remove(key);
        self.deletions.insert(key.to_vec());
    }
}

pub struct MemDBSnapshot<'a>(MutexGuard<'a, HashMap<Vec<u8>, Vec<u8>>>);
impl KVGet for MemDBSnapshot<'_> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.0.get(key).cloned()
    }
}

#[derive(Debug, Serialize)]
struct SpotBlockExecutionSummary {
    status: &'static str,
    item_count: usize,
    applied_count: usize,
    rejected_count: usize,
    replayable_event_count: usize,
}

#[derive(Clone)]
pub struct SpotOrderApp {
    request_queue: Arc<Mutex<Vec<SpotBlockCommandV2>>>,
}

impl SpotOrderApp {
    pub fn new(request_queue: Arc<Mutex<Vec<SpotBlockCommandV2>>>) -> Self {
        Self { request_queue }
    }
    pub fn initial_app_state() -> AppStateUpdates {
        let mut updates = AppStateUpdates::new();
        let state = initial_build_block_state();
        updates.insert(
            CANONICAL_STATE_KEY.to_vec(),
            encode_build_block_state(&state).expect("demo initial state must be encodable"),
        );
        updates
    }

    fn execute(
        command: &SpotBlockCmds,
        parent_state: &[u8],
    ) -> Result<Option<AppStateUpdates>, ()> {
        if command.items.is_empty() {
            return Ok(None);
        }
        let mut state = decode_build_block_state(parent_state).map_err(|_| ())?;
        let result = build_block_spot_v3(&command.items, &mut state).map_err(|_| ())?;
        let applied_count = result
            .command_results
            .iter()
            .filter(|item| match item {
                SpotBlockCommandExecutionV3::Place { result, .. } => result.is_ok(),
                SpotBlockCommandExecutionV3::Cancel { result, .. } => result.is_ok(),
            })
            .count();
        let summary = SpotBlockExecutionSummary {
            status: "executed",
            item_count: result.command_results.len(),
            applied_count,
            rejected_count: result.command_results.len() - applied_count,
            replayable_event_count: result.replayable_events().len(),
        };
        let value = serde_json::to_vec(&summary).map_err(|_| ())?;
        let mut updates = AppStateUpdates::new();
        updates.insert(
            CANONICAL_STATE_KEY.to_vec(),
            encode_build_block_state(&state).map_err(|_| ())?,
        );
        if command
            .items
            .iter()
            .any(|item| matches!(item.payload(), SpotBlockCommandV2::Place(_)))
        {
            updates.insert(PLACE_RESULT_KEY.to_vec(), value.clone());
        }
        if command
            .items
            .iter()
            .any(|item| matches!(item.payload(), SpotBlockCommandV2::Cancel(_)))
        {
            updates.insert(CANCEL_RESULT_KEY.to_vec(), value);
        }
        Ok(Some(updates))
    }
}

fn initial_build_block_state() -> State4BuildBlock {
    use base_types_v2::{Coin, Symbol};
    use lob_repo_v2::imp::lob_btree::LocalLobBtree;

    let symbol = Symbol::new(1, Coin::BTC, Coin::USDT);
    let buyer = common_entity::stable_string_entity_id("buyer");
    let party_id = example_core_use_case::PartyId::from("buyer");
    State4BuildBlock {
        next_trade_id: 1,
        maker_fee_bps: 5,
        taker_fee_bps: 10,
        spot_symbol_lob: [(symbol, LocalLobBtree::new(symbol))].into(),
        user_symbol_quote_balance: [(
            (party_id.clone(), symbol),
            BalanceV2::new(party_id.clone(), buyer, 2, 100_000, 0, 0, 0),
        )]
        .into(),
        user_symbol_base_balance: [(
            (party_id.clone(), symbol),
            BalanceV2::new(party_id, buyer, 0, 10, 0, 0, 0),
        )]
        .into(),
        user_symbol_trades: Default::default(),
        user_symbol_orders: Default::default(),
    }
}

impl App<MemDB> for SpotOrderApp {
    fn produce_block(&mut self, request: ProduceBlockRequest<MemDB>) -> ProduceBlockResponse {
        let block_sequence = request
            .parent_block()
            .and_then(|parent| request.block_tree().block_height(&parent).ok().flatten())
            .map_or(Some(1), |height| height.int().checked_add(2));
        let commands = self
            .request_queue
            .lock()
            .map(|queue| queue.clone())
            .unwrap_or_default();
        let command =
            block_sequence.and_then(|sequence| presequence_commands(sequence, &commands).ok());
        let parent_state = request.block_tree().app_state(CANONICAL_STATE_KEY);
        let execution = command.as_ref().and_then(|command| {
            parent_state
                .as_deref()
                .and_then(|state| Self::execute(command, state).ok())
        });
        let command = if commands.is_empty() || execution.is_some() {
            command.unwrap_or(SpotBlockCmds { items: Vec::new() })
        } else {
            SpotBlockCmds { items: Vec::new() }
        };
        if !command.items.is_empty() {
            if let Ok(mut queue) = self.request_queue.lock() {
                queue.drain(..commands.len());
            }
        }
        let data = encode_requests(&command).unwrap_or_else(|_| Data::new(Vec::new()));
        ProduceBlockResponse {
            data_hash: data_hash(&data),
            data,
            app_state_updates: execution.flatten(),
            validator_set_updates: None,
        }
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
        let Ok(command) = decode_requests(&block.data) else {
            return ValidateBlockResponse::Invalid;
        };
        if command.items.is_empty() {
            return ValidateBlockResponse::Valid {
                app_state_updates: None,
                validator_set_updates: None,
            };
        }
        let Some(parent_state) = request.block_tree().app_state(CANONICAL_STATE_KEY) else {
            return ValidateBlockResponse::Invalid;
        };
        let Ok(app_state_updates) = Self::execute(&command, &parent_state) else {
            return ValidateBlockResponse::Invalid;
        };
        ValidateBlockResponse::Valid {
            app_state_updates,
            validator_set_updates: None,
        }
    }
}

pub fn presequence_commands(
    block_sequence: u64,
    commands: &[SpotBlockCommandV2],
) -> Result<SpotBlockCmds, &'static str> {
    if commands.len() > u32::MAX as usize || block_sequence > u32::MAX as u64 {
        return Err("spot block sequence or item count overflows deterministic LSN layout");
    }
    let prefix = block_sequence
        .checked_shl(32)
        .ok_or("spot block sequence shift overflows")?;
    let mut items = Vec::with_capacity(commands.len());
    for (index, command) in commands.iter().cloned().enumerate() {
        let ordinal = u64::try_from(index)
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or("spot block item index overflows")?;
        let lsn = prefix
            .checked_add(ordinal)
            .ok_or("spot block LSN overflows")?;
        let user = "buyer".to_owned();
        items.push(common_entity::LogEntrySequencer::from_metadata_with_topic(
            0, 0, lsn, lsn, 0, ordinal, user, command,
        ));
    }
    Ok(SpotBlockCmds { items })
}

pub fn encode_requests(command: &SpotBlockCmds) -> Result<Data, serde_json::Error> {
    Ok(Data::new(vec![Datum::new(serde_json::to_vec(command)?)]))
}
pub fn decode_requests(data: &Data) -> Result<SpotBlockCmds, serde_json::Error> {
    let Some(datum) = data.vec().first() else {
        return Ok(SpotBlockCmds { items: Vec::new() });
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
pub fn get_from_snapshot<S: KVGet>(snapshot: &BlockTreeSnapshot<S>, key: &[u8]) -> Option<Vec<u8>> {
    snapshot.committed_app_state(key)
}
pub fn lock_or_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
pub fn demo_replica_configuration(signing_key: SigningKey) -> Configuration {
    Configuration::builder()
        .me(signing_key)
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
        .build()
}
pub fn demo_validator_set_state(verifying_keys: &[VerifyingKey]) -> ValidatorSetState {
    let mut set = ValidatorSet::new();
    for key in verifying_keys {
        set.put(key, Power::new(1));
    }
    ValidatorSetState::new(set.clone(), set, None, true)
}
pub fn deterministic_signing_keys() -> Vec<SigningKey> {
    (1..=NODE_COUNT)
        .map(|seed| SigningKey::from_bytes(&[seed as u8; 32]))
        .collect()
}

#[cfg(test)]
mod tests {
    use base_types_v2::{Coin, Symbol};
    use example_core_use_case::entity::common::identifiers::OrderId;
    use example_core_use_case::{
        CancelSpotOrderFsmV3Cmd, OrderGroupRelation, OrderSide, OrderTif, OrderType,
        PlaceSpotOrderPipelineV2Cmd, SpotOrderStatus,
    };
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn deterministic_lsn_and_json_round_trip() {
        let command = presequence_commands(
            7,
            &[SpotBlockCommandV2::Cancel(CancelSpotOrderFsmV3Cmd {
                order_id: OrderId::Oid(1),
            })],
        )
        .expect("sequence must fit");
        assert_eq!(command.items[0].lsn(), (7_u64 << 32) | 1);
        let decoded = decode_requests(&encode_requests(&command).expect("encode must work"))
            .expect("decode must work");
        assert_eq!(decoded, command);
    }

    #[test]
    fn place_then_cancel_uses_the_previous_canonical_state() {
        let initial = SpotOrderApp::initial_app_state();
        let initial_bytes = initial
            .get_insert(&CANONICAL_STATE_KEY.to_vec())
            .expect("initial canonical state");
        let symbol = Symbol::new(1, Coin::BTC, Coin::USDT);
        let place = SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
            party_id: example_core_use_case::PartyId::from("buyer"),
            order_id: 1,
            symbol,
            side: OrderSide::Buy,
            price: Decimal::from(100),
            size: Decimal::ONE,
            order_type: OrderType::Limit { tif: OrderTif::Gtc },
            relation: OrderGroupRelation::Standalone,
            cloid: Some("chain-1".to_owned()),
            fee_bps: 10,
        });
        let place_block = presequence_commands(1, &[place]).unwrap();
        let place_updates = SpotOrderApp::execute(&place_block, initial_bytes)
            .unwrap()
            .expect("place updates");
        let placed_bytes = place_updates
            .get_insert(&CANONICAL_STATE_KEY.to_vec())
            .expect("placed canonical state");
        let placed = decode_build_block_state(placed_bytes).unwrap();
        assert_eq!(
            placed.user_symbol_orders[&(example_core_use_case::PartyId::from("buyer"), symbol)]
                .len(),
            1
        );

        let cancel = SpotBlockCommandV2::Cancel(CancelSpotOrderFsmV3Cmd {
            order_id: OrderId::Cloid("chain-1".to_owned()),
        });
        let cancel_block = presequence_commands(2, &[cancel]).unwrap();
        let cancel_updates = SpotOrderApp::execute(&cancel_block, placed_bytes)
            .unwrap()
            .expect("cancel updates");
        let cancelled = decode_build_block_state(
            cancel_updates
                .get_insert(&CANONICAL_STATE_KEY.to_vec())
                .expect("cancelled canonical state"),
        )
        .unwrap();
        assert_eq!(
            cancelled.user_symbol_orders[&(example_core_use_case::PartyId::from("buyer"), symbol)]
                [0]
            .order_base()
            .status,
            SpotOrderStatus::Canceled
        );
        assert!(
            place_updates
                .get_insert(&PLACE_RESULT_KEY.to_vec())
                .is_some()
        );
        assert!(
            cancel_updates
                .get_insert(&CANCEL_RESULT_KEY.to_vec())
                .is_some()
        );
    }
}
