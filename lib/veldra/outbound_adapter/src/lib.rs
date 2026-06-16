use std::fs;
use std::path::Path;

use bincode::Options;
use cmd_handler::EntityReplayableEvent;
use example_core::{Balance, SpotOrder};
use libmdbx::{Cursor, Database, DatabaseOptions, NoWriteMap, TableFlags, WriteFlags};
use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;
use veldra_core::entity::{BlockExecutionBody, CommandEnvelope, NewBlock, ProductCommand};
use veldra_core::use_case::BlockEntityChange;

const SCHEMA_VERSION_V1: u16 = 1;
const TABLE_BLOCK_HEADERS: &str = "block_headers";
const TABLE_BLOCK_HASH_TO_HEIGHT: &str = "block_hash_to_height";
const TABLE_BLOCK_COMMANDS: &str = "block_commands";
const TABLE_BLOCK_EVENTS: &str = "block_events";
const TABLE_SPOT_ORDERS: &str = "spot_orders";
const TABLE_BALANCES: &str = "balances";

type MdbxDatabase = Database<NoWriteMap>;

#[derive(Debug, Error)]
pub enum VeldraMdbxStorageError {
    #[error("failed to open storage: {0}")]
    Open(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to read storage: {0}")]
    Read(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to write storage: {0}")]
    Write(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to encode record: {0}")]
    Encode(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to decode record: {0}")]
    Decode(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("block height {0} already exists")]
    DuplicateBlockHeight(u64),
    #[error("block hash '{0}' already exists")]
    DuplicateBlockHash(String),
    #[error(
        "execution body does not match header: header(height={header_height}, hash={header_hash}) body(height={body_height}, hash={body_hash})"
    )]
    HeaderBodyMismatch {
        header_height: u64,
        header_hash: String,
        body_height: u64,
        body_hash: String,
    },
    #[error("spot order snapshot identity changed during projection")]
    SpotOrderProjectionMismatch,
    #[error("balance snapshot identity changed during projection")]
    BalanceProjectionMismatch,
}

#[derive(Debug)]
pub struct VeldraMdbxBlockStore {
    db: MdbxDatabase,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlockHeaderRecord {
    pub schema_version: u16,
    pub block_height: u64,
    pub parent_block_hash: String,
    pub commands_root: String,
    pub events_root: String,
    pub post_state_root: String,
    pub block_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredCommandEnvelope {
    pub schema_version: u16,
    pub block_height: u64,
    pub command_index: u64,
    pub command_id: String,
    pub account_id: String,
    pub nonce: u64,
    pub timestamp_ns: u64,
    pub command_kind: String,
    pub encoded_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredReplayableEvent {
    pub schema_version: u16,
    pub block_height: u64,
    pub event_sequence: u64,
    pub entity_id: i64,
    pub entity_type: u8,
    pub change_type: u8,
    pub encoded_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredSpotOrderSnapshot {
    pub schema_version: u16,
    pub block_height: u64,
    pub order_id: String,
    pub account_id: String,
    pub symbol: String,
    pub encoded_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredBalanceSnapshot {
    pub schema_version: u16,
    pub block_height: u64,
    pub account_id: String,
    pub asset_id: String,
    pub encoded_payload: Vec<u8>,
}

impl VeldraMdbxBlockStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, VeldraMdbxStorageError> {
        fs::create_dir_all(path.as_ref())
            .map_err(|error| VeldraMdbxStorageError::Open(Box::new(error)))?;
        let db = MdbxDatabase::open_with_options(
            path,
            DatabaseOptions { max_tables: Some(16), ..Default::default() },
        )
        .map_err(|error| VeldraMdbxStorageError::Open(Box::new(error)))?;

        let store = Self { db };
        store.initialize_tables()?;
        Ok(store)
    }

    pub fn append_block(
        &self,
        header: &NewBlock,
        body: &BlockExecutionBody,
        changes: &[BlockEntityChange],
    ) -> Result<(), VeldraMdbxStorageError> {
        if header.block_height != body.block_height || header.block_hash != body.block_hash {
            return Err(VeldraMdbxStorageError::HeaderBodyMismatch {
                header_height: header.block_height,
                header_hash: header.block_hash.clone(),
                body_height: body.block_height,
                body_hash: body.block_hash.clone(),
            });
        }

        let txn = self
            .db
            .begin_rw_txn()
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        let block_headers = txn
            .open_table(Some(TABLE_BLOCK_HEADERS))
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        let block_hash_to_height = txn
            .open_table(Some(TABLE_BLOCK_HASH_TO_HEIGHT))
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        let block_commands = txn
            .open_table(Some(TABLE_BLOCK_COMMANDS))
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        let block_events = txn
            .open_table(Some(TABLE_BLOCK_EVENTS))
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        let spot_orders = txn
            .open_table(Some(TABLE_SPOT_ORDERS))
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        let balances = txn
            .open_table(Some(TABLE_BALANCES))
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;

        let height_key = encode_u64_be(header.block_height);
        if txn
            .get::<Vec<u8>>(&block_headers, &height_key)
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?
            .is_some()
        {
            return Err(VeldraMdbxStorageError::DuplicateBlockHeight(header.block_height));
        }

        if txn
            .get::<Vec<u8>>(&block_hash_to_height, header.block_hash.as_bytes())
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?
            .is_some()
        {
            return Err(VeldraMdbxStorageError::DuplicateBlockHash(header.block_hash.clone()));
        }

        let header_record = BlockHeaderRecord::from(header);
        txn.put(&block_headers, &height_key, &encode_record(&header_record)?, WriteFlags::empty())
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        txn.put(
            &block_hash_to_height,
            header.block_hash.as_bytes(),
            &height_key,
            WriteFlags::empty(),
        )
        .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;

        for (command_index, command) in body.commands.iter().enumerate() {
            let record = StoredCommandEnvelope::from_command(
                header.block_height,
                command_index as u64,
                command,
            )?;
            let key = encode_block_sequence_key(header.block_height, command_index as u64);
            txn.put(&block_commands, &key, &encode_record(&record)?, WriteFlags::empty())
                .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        }

        for event in &body.replayable_events {
            let record = StoredReplayableEvent::from_event(header.block_height, event)?;
            let key = encode_block_sequence_key(header.block_height, event.sequence);
            txn.put(&block_events, &key, &encode_record(&record)?, WriteFlags::empty())
                .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        }

        for change in changes {
            match change {
                BlockEntityChange::SpotOrderCreated(order) => {
                    let snapshot = StoredSpotOrderSnapshot::from_order(header.block_height, order)?;
                    txn.put(
                        &spot_orders,
                        order.order_id.as_bytes(),
                        &encode_record(&snapshot)?,
                        WriteFlags::empty(),
                    )
                    .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
                }
                BlockEntityChange::SpotOrderUpdated(order) => {
                    if order.before.order_id != order.after.order_id {
                        return Err(VeldraMdbxStorageError::SpotOrderProjectionMismatch);
                    }
                    let snapshot =
                        StoredSpotOrderSnapshot::from_order(header.block_height, &order.after)?;
                    txn.put(
                        &spot_orders,
                        order.after.order_id.as_bytes(),
                        &encode_record(&snapshot)?,
                        WriteFlags::empty(),
                    )
                    .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
                }
                BlockEntityChange::BalanceUpdated(balance) => {
                    if balance.before.account_id != balance.after.account_id
                        || balance.before.asset_id != balance.after.asset_id
                    {
                        return Err(VeldraMdbxStorageError::BalanceProjectionMismatch);
                    }
                    let snapshot =
                        StoredBalanceSnapshot::from_balance(header.block_height, &balance.after)?;
                    txn.put(
                        &balances,
                        &encode_account_asset_key(
                            &balance.after.account_id,
                            &balance.after.asset_id,
                        ),
                        &encode_record(&snapshot)?,
                        WriteFlags::empty(),
                    )
                    .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
                }
                BlockEntityChange::SpotTradeCreated(_)
                | BlockEntityChange::SpotSettlementCreated(_) => {}
            }
        }

        txn.commit().map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        Ok(())
    }

    pub fn get_block_header(
        &self,
        height: u64,
    ) -> Result<Option<NewBlock>, VeldraMdbxStorageError> {
        let txn = self
            .db
            .begin_ro_txn()
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let table = txn
            .open_table(Some(TABLE_BLOCK_HEADERS))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let result = txn
            .get::<Vec<u8>>(&table, &encode_u64_be(height))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?
            .map(|bytes| decode_record::<BlockHeaderRecord>(&bytes).map(NewBlock::from))
            .transpose()?;
        txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        Ok(result)
    }

    pub fn get_block_header_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<NewBlock>, VeldraMdbxStorageError> {
        let txn = self
            .db
            .begin_ro_txn()
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let hash_table = txn
            .open_table(Some(TABLE_BLOCK_HASH_TO_HEIGHT))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let Some(height_bytes) = txn
            .get::<Vec<u8>>(&hash_table, hash.as_bytes())
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?
        else {
            txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
            return Ok(None);
        };
        let height = decode_u64_be(&height_bytes)?;
        let headers = txn
            .open_table(Some(TABLE_BLOCK_HEADERS))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let result = txn
            .get::<Vec<u8>>(&headers, &encode_u64_be(height))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?
            .map(|bytes| decode_record::<BlockHeaderRecord>(&bytes).map(NewBlock::from))
            .transpose()?;
        txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        Ok(result)
    }

    pub fn scan_block_commands(
        &self,
        height: u64,
    ) -> Result<Vec<CommandEnvelope<ProductCommand>>, VeldraMdbxStorageError> {
        let txn = self
            .db
            .begin_ro_txn()
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let table = txn
            .open_table(Some(TABLE_BLOCK_COMMANDS))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let prefix = encode_u64_be(height);
        let mut cursor =
            txn.cursor(&table).map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let mut commands = Vec::new();
        for item in cursor.iter() {
            let (key, value) =
                item.map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
            if !key.starts_with(&prefix) {
                continue;
            }
            let record: StoredCommandEnvelope = decode_record(value)?;
            commands.push(record.decode_command()?);
        }
        txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        Ok(commands)
    }

    pub fn scan_block_events(
        &self,
        height: u64,
    ) -> Result<Vec<EntityReplayableEvent>, VeldraMdbxStorageError> {
        let txn = self
            .db
            .begin_ro_txn()
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let table = txn
            .open_table(Some(TABLE_BLOCK_EVENTS))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let prefix = encode_u64_be(height);
        let mut cursor =
            txn.cursor(&table).map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let mut events = Vec::new();
        for item in cursor.iter() {
            let (key, value) =
                item.map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
            if !key.starts_with(&prefix) {
                continue;
            }
            let record: StoredReplayableEvent = decode_record(value)?;
            events.push(record.decode_event()?);
        }
        txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        Ok(events)
    }

    pub fn load_current_spot_order(
        &self,
        order_id: &str,
    ) -> Result<Option<SpotOrder>, VeldraMdbxStorageError> {
        let txn = self
            .db
            .begin_ro_txn()
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let table = txn
            .open_table(Some(TABLE_SPOT_ORDERS))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let result = txn
            .get::<Vec<u8>>(&table, order_id.as_bytes())
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?
            .map(|bytes| {
                decode_record::<StoredSpotOrderSnapshot>(&bytes).and_then(|r| r.decode_order())
            })
            .transpose()?;
        txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        Ok(result)
    }

    pub fn load_current_balance(
        &self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<Option<Balance>, VeldraMdbxStorageError> {
        let txn = self
            .db
            .begin_ro_txn()
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let table = txn
            .open_table(Some(TABLE_BALANCES))
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        let key = encode_account_asset_key(account_id, asset_id);
        let result = txn
            .get::<Vec<u8>>(&table, &key)
            .map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?
            .map(|bytes| {
                decode_record::<StoredBalanceSnapshot>(&bytes).and_then(|r| r.decode_balance())
            })
            .transpose()?;
        txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        Ok(result)
    }

    fn initialize_tables(&self) -> Result<(), VeldraMdbxStorageError> {
        let txn = self
            .db
            .begin_rw_txn()
            .map_err(|error| VeldraMdbxStorageError::Open(Box::new(error)))?;
        for table_name in [
            TABLE_BLOCK_HEADERS,
            TABLE_BLOCK_HASH_TO_HEIGHT,
            TABLE_BLOCK_COMMANDS,
            TABLE_BLOCK_EVENTS,
            TABLE_SPOT_ORDERS,
            TABLE_BALANCES,
        ] {
            txn.create_table(Some(table_name), TableFlags::empty())
                .map_err(|error| VeldraMdbxStorageError::Open(Box::new(error)))?;
        }
        txn.commit().map_err(|error| VeldraMdbxStorageError::Open(Box::new(error)))?;
        Ok(())
    }
}

impl From<&NewBlock> for BlockHeaderRecord {
    fn from(value: &NewBlock) -> Self {
        Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height: value.block_height,
            parent_block_hash: value.parent_block_hash.clone(),
            commands_root: value.commands_root.clone(),
            events_root: value.events_root.clone(),
            post_state_root: value.post_state_root.clone(),
            block_hash: value.block_hash.clone(),
        }
    }
}

impl From<BlockHeaderRecord> for NewBlock {
    fn from(value: BlockHeaderRecord) -> Self {
        Self {
            block_height: value.block_height,
            parent_block_hash: value.parent_block_hash,
            commands_root: value.commands_root,
            events_root: value.events_root,
            post_state_root: value.post_state_root,
            block_hash: value.block_hash,
        }
    }
}

impl StoredCommandEnvelope {
    fn from_command(
        block_height: u64,
        command_index: u64,
        command: &CommandEnvelope<ProductCommand>,
    ) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            command_index,
            command_id: command.command_id.clone(),
            account_id: command.account_id.clone(),
            nonce: command.nonce,
            timestamp_ns: command.timestamp_ns,
            command_kind: command.command.kind().to_string(),
            encoded_payload: encode_record(command)?,
        })
    }

    fn decode_command(&self) -> Result<CommandEnvelope<ProductCommand>, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}

impl StoredReplayableEvent {
    fn from_event(
        block_height: u64,
        event: &EntityReplayableEvent,
    ) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            event_sequence: event.sequence,
            entity_id: event.entity_id,
            entity_type: event.entity_type,
            change_type: event.change_type,
            encoded_payload: encode_record(event)?,
        })
    }

    fn decode_event(&self) -> Result<EntityReplayableEvent, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}

impl StoredSpotOrderSnapshot {
    fn from_order(block_height: u64, order: &SpotOrder) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            order_id: order.order_id.clone(),
            account_id: order.account_id.clone(),
            symbol: order.symbol.clone(),
            encoded_payload: encode_record(order)?,
        })
    }

    fn decode_order(&self) -> Result<SpotOrder, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}

impl StoredBalanceSnapshot {
    fn from_balance(block_height: u64, balance: &Balance) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            account_id: balance.account_id.clone(),
            asset_id: balance.asset_id.clone(),
            encoded_payload: encode_record(balance)?,
        })
    }

    fn decode_balance(&self) -> Result<Balance, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}

fn encode_record<T: Serialize>(value: &T) -> Result<Vec<u8>, VeldraMdbxStorageError> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialize(value)
        .map_err(|error| VeldraMdbxStorageError::Encode(Box::new(error)))
}

fn decode_record<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, VeldraMdbxStorageError> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .map_err(|error| VeldraMdbxStorageError::Decode(Box::new(error)))
}

pub fn encode_u64_be(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

pub fn encode_block_sequence_key(block_height: u64, sequence: u64) -> [u8; 16] {
    let mut key = [0u8; 16];
    key[..8].copy_from_slice(&block_height.to_be_bytes());
    key[8..].copy_from_slice(&sequence.to_be_bytes());
    key
}

pub fn encode_account_asset_key(account_id: &str, asset_id: &str) -> Vec<u8> {
    let mut key = Vec::with_capacity(account_id.len() + asset_id.len() + 1);
    key.extend_from_slice(account_id.as_bytes());
    key.push(0);
    key.extend_from_slice(asset_id.as_bytes());
    key
}

fn decode_u64_be(bytes: &[u8]) -> Result<u64, VeldraMdbxStorageError> {
    let array: [u8; 8] =
        bytes.try_into().map_err(|error| VeldraMdbxStorageError::Decode(Box::new(error)))?;
    Ok(u64::from_be_bytes(array))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    use cmd_handler::command_use_case_def2::CommandUseCase4;
    use example_core::{
        Balance, DepositQuoteCmd, ExecuteImmediateSpotOrderPipelineCmd, MarketRules,
        PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceOrderTimeInForce,
    };
    use veldra_core::entity::{
        AccountAssetKey, ExchangeState, NewBlock, ProductCommand, SpotAssetPair, SpotCommand,
        TreasuryCommand,
    };
    use veldra_core::use_case::{
        BuildBlockFromCommandsChanges, BuildBlockFromCommandsCommand, BuildBlockFromCommandsState,
        BuildBlockFromCommandsUseCase,
    };

    use super::*;

    struct TestDir {
        path: std::path::PathBuf,
    }

    impl TestDir {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock should advance")
                .as_nanos();
            let path = std::env::temp_dir().join(format!("veldra-mdbx-test-{unique}"));
            fs::create_dir_all(&path).expect("temp dir should be created");
            Self { path }
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn sample_command() -> BuildBlockFromCommandsCommand {
        BuildBlockFromCommandsCommand { block_height: 2 }
    }

    fn sample_state() -> BuildBlockFromCommandsState {
        let mut market_rules_by_symbol = BTreeMap::new();
        market_rules_by_symbol.insert(
            "BTCUSDT".to_string(),
            MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
        );

        let mut asset_pairs_by_symbol = BTreeMap::new();
        asset_pairs_by_symbol.insert("BTCUSDT".to_string(), SpotAssetPair::new("BTC", "USDT"));

        let mut trading_enabled_by_symbol = BTreeMap::new();
        trading_enabled_by_symbol.insert("BTCUSDT".to_string(), true);

        let mut spot_balances = BTreeMap::new();
        spot_balances.insert(
            AccountAssetKey::new("trader-1", "USDT"),
            Balance::new("trader-1".to_string(), "USDT".to_string(), 10_000, 0, 3),
        );
        spot_balances.insert(
            AccountAssetKey::new("trader-1", "BTC"),
            Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 2),
        );

        let mut treasury_balances = BTreeMap::new();
        treasury_balances.insert(
            AccountAssetKey::new("trader-1", "USDT"),
            Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
        );

        let mut next_order_sequence_by_account = BTreeMap::new();
        next_order_sequence_by_account.insert("trader-1".to_string(), 7);

        BuildBlockFromCommandsState {
            parent_height: 1,
            parent_block_hash: "parent-1".to_string(),
            exchange_state: ExchangeState {
                spot: veldra_core::entity::SpotState {
                    market_rules_by_symbol,
                    asset_pairs_by_symbol,
                    trading_enabled_by_symbol,
                    balances: spot_balances,
                    orders: BTreeMap::new(),
                    settled_trade_ids: Default::default(),
                    next_order_sequence_by_account,
                },
                treasury: veldra_core::entity::TreasuryState {
                    balances: treasury_balances,
                    withdraw_locks: Default::default(),
                },
                ..ExchangeState::default()
            },
            commands: vec![
                CommandEnvelope {
                    command_id: "cmd-1".to_string(),
                    account_id: "trader-1".to_string(),
                    nonce: 1,
                    timestamp_ns: 1_000,
                    command: ProductCommand::Spot(SpotCommand::ExecuteImmediateOrderPipeline(
                        ExecuteImmediateSpotOrderPipelineCmd {
                            place: PlaceImmediateOrderCmd {
                                party_id: "trader-1".to_string(),
                                asset: 10_001,
                                symbol: "BTCUSDT".to_string(),
                                is_buy: true,
                                size: 2,
                                reduce_only: false,
                                execution: PlaceImmediateOrderExecution::Limit {
                                    price: 100,
                                    time_in_force: PlaceOrderTimeInForce::Gtc,
                                },
                                cloid: Some("cl-1".to_string()),
                            },
                            match_id: "match-1".to_string(),
                            settlement_batch_id: "settle-1".to_string(),
                        },
                    )),
                },
                CommandEnvelope {
                    command_id: "cmd-2".to_string(),
                    account_id: "trader-1".to_string(),
                    nonce: 2,
                    timestamp_ns: 1_001,
                    command: ProductCommand::Treasury(TreasuryCommand::DepositQuote(
                        DepositQuoteCmd { party_id: "trader-1".to_string(), amount: 500 },
                    )),
                },
            ],
        }
    }

    fn built_block() -> BuildBlockFromCommandsChanges {
        BuildBlockFromCommandsUseCase
            .compute_changes(&sample_command(), sample_state())
            .expect("block should build")
    }

    fn header_body_changes(
        changes: &BuildBlockFromCommandsChanges,
    ) -> (NewBlock, BlockExecutionBody, Vec<BlockEntityChange>) {
        (
            changes.new_block.clone().expect("new block is required"),
            changes.execution_body.clone().expect("execution body is required"),
            changes.ordered_changes.clone(),
        )
    }

    #[test]
    fn append_and_read_back_block_data_and_snapshots() {
        let temp_dir = TestDir::new();
        let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
        let built = built_block();
        let (header, body, changes) = header_body_changes(&built);

        store.append_block(&header, &body, &changes).expect("append should succeed");

        let stored_header = store
            .get_block_header(header.block_height)
            .expect("header read should succeed")
            .expect("header should exist");
        let header_by_hash = store
            .get_block_header_by_hash(&header.block_hash)
            .expect("hash lookup should succeed")
            .expect("header should exist by hash");
        let stored_commands =
            store.scan_block_commands(header.block_height).expect("command scan should succeed");
        let stored_events =
            store.scan_block_events(header.block_height).expect("event scan should succeed");
        let stored_order = store
            .load_current_spot_order("trader-1-BTCUSDT-7")
            .expect("order load should succeed")
            .expect("order should exist");
        let stored_balance = store
            .load_current_balance("trader-1", "USDT")
            .expect("balance load should succeed")
            .expect("balance should exist");

        assert_eq!(stored_header, header);
        assert_eq!(header_by_hash, header);
        assert_eq!(
            stored_commands.iter().map(|command| command.command_id.as_str()).collect::<Vec<_>>(),
            vec!["cmd-1", "cmd-2"]
        );
        assert_eq!(stored_events, body.replayable_events);
        assert_eq!(stored_order.order_id, "trader-1-BTCUSDT-7");
        assert_eq!((stored_order.reserved_quote, stored_order.status.as_str()), (200, "open"));
        assert_eq!(
            (stored_balance.available, stored_balance.frozen, stored_balance.version),
            (1_500, 0, 2)
        );
    }

    #[test]
    fn duplicate_block_height_is_rejected() {
        let temp_dir = TestDir::new();
        let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
        let built = built_block();
        let (header, body, changes) = header_body_changes(&built);

        store.append_block(&header, &body, &changes).expect("first append should succeed");
        let error =
            store.append_block(&header, &body, &changes).expect_err("second append should fail");

        assert!(matches!(error, VeldraMdbxStorageError::DuplicateBlockHeight(2)));
    }

    #[test]
    fn duplicate_block_hash_is_rejected() {
        let temp_dir = TestDir::new();
        let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
        let built = built_block();
        let (header, body, changes) = header_body_changes(&built);
        store.append_block(&header, &body, &changes).expect("first append should succeed");

        let second_header = NewBlock {
            block_height: header.block_height + 1,
            parent_block_hash: header.parent_block_hash.clone(),
            commands_root: header.commands_root.clone(),
            events_root: header.events_root.clone(),
            post_state_root: header.post_state_root.clone(),
            block_hash: header.block_hash.clone(),
        };
        let second_body = BlockExecutionBody {
            block_hash: second_header.block_hash.clone(),
            block_height: second_header.block_height,
            commands: body.commands.clone(),
            replayable_events: body.replayable_events.clone(),
        };

        let error = store
            .append_block(&second_header, &second_body, &changes)
            .expect_err("duplicate hash should fail");

        assert!(
            matches!(error, VeldraMdbxStorageError::DuplicateBlockHash(hash) if hash == header.block_hash)
        );
    }

    #[test]
    fn projection_failure_leaves_no_partial_block() {
        let temp_dir = TestDir::new();
        let store = VeldraMdbxBlockStore::open(&temp_dir.path).expect("store should open");
        let built = built_block();
        let (header, body, mut changes) = header_body_changes(&built);

        changes.push(BlockEntityChange::BalanceUpdated(
            cmd_handler::command_use_case_def2::UpdatedEntityPair {
                before: Balance::new("trader-1".to_string(), "USDT".to_string(), 10, 0, 1),
                after: Balance::new("trader-1".to_string(), "BTC".to_string(), 10, 0, 2),
            },
        ));

        let error = store
            .append_block(&header, &body, &changes)
            .expect_err("projection mismatch should abort append");
        assert!(matches!(error, VeldraMdbxStorageError::BalanceProjectionMismatch));
        assert!(
            store
                .get_block_header(header.block_height)
                .expect("header read should succeed")
                .is_none()
        );
        assert!(
            store
                .load_current_spot_order("trader-1-BTCUSDT-7")
                .expect("order read should succeed")
                .is_none()
        );
        assert!(
            store
                .load_current_balance("trader-1", "USDT")
                .expect("balance read should succeed")
                .is_none()
        );
    }
}
