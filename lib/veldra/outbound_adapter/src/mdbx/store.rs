use std::fs;
use std::path::Path;

use cmd_handler::EntityReplayableEvent;
use example_core::{Balance, SpotOrderV2};
use libmdbx::{Database, DatabaseOptions, NoWriteMap, TableFlags, WriteFlags};
use veldra_core::entity::{BlockExecutionBody, CommandEnvelope, NewBlock, ProductCommand};
use veldra_core::use_case::BlockEntityChange;

use super::codec::{decode_record, encode_record};
use super::error::VeldraMdbxStorageError;
use super::key::{
    decode_u64_be, encode_account_asset_key, encode_block_sequence_key, encode_u64_be,
};
use super::record::{
    BlockHeaderRecord, StoredBalanceSnapshot, StoredCommandEnvelope, StoredReplayableEvent,
    StoredSpotOrderSnapshot,
};
use super::table::{
    TABLE_BALANCES, TABLE_BLOCK_COMMANDS, TABLE_BLOCK_EVENTS, TABLE_BLOCK_HASH_TO_HEIGHT,
    TABLE_BLOCK_HEADERS, TABLE_NAMES, TABLE_SPOT_ORDERS,
};

type MdbxDatabase = Database<NoWriteMap>;

#[derive(Debug)]
pub struct VeldraMdbxBlockStore {
    db: MdbxDatabase,
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
        // 先保证 header 与 execution body 描述的是同一个 block，避免写入后出现主记录与明细不一致。
        if header.block_height != body.block_height || header.block_hash != body.block_hash {
            return Err(VeldraMdbxStorageError::HeaderBodyMismatch {
                header_height: header.block_height,
                header_hash: header.block_hash.clone(),
                body_height: body.block_height,
                body_hash: body.block_hash.clone(),
            });
        }

        // 在单个写事务里完成整笔落库，后续任一步失败都应整体回滚。
        let txn = self
            .db
            .begin_rw_txn()
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        // 打开本次 append 会写到的全部表：header/hash 索引、commands/events、以及当前快照投影。
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
        // 先检查高度主键是否已存在，防止覆盖历史 block。
        if txn
            .get::<Vec<u8>>(&block_headers, &height_key)
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?
            .is_some()
        {
            return Err(VeldraMdbxStorageError::DuplicateBlockHeight(header.block_height));
        }

        // 再检查 hash 反向索引，保证不同高度也不能复用同一个 block hash。
        if txn
            .get::<Vec<u8>>(&block_hash_to_height, header.block_hash.as_bytes())
            .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?
            .is_some()
        {
            return Err(VeldraMdbxStorageError::DuplicateBlockHash(header.block_hash.clone()));
        }

        // 先写 block header 本体，再写 hash -> height 的查找索引，支撑按高度和按 hash 两种读取路径。
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

        // 将 block 内命令按 (height, command_index) 顺序编码后落盘，保留回放顺序。
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

        // 将 replayable events 按 (height, event_sequence) 落盘，供事件重放和核对使用。
        for event in &body.replayable_events {
            let record = StoredReplayableEvent::from_event(header.block_height, event)?;
            let key = encode_block_sequence_key(header.block_height, event.sequence);
            txn.put(&block_events, &key, &encode_record(&record)?, WriteFlags::empty())
                .map_err(|error| VeldraMdbxStorageError::Write(Box::new(error)))?;
        }

        // 把本 block 产生的实体变化投影到“当前快照”表里，形成最新订单/余额视图。
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
                    // 更新订单前先校验 identity 不变，避免把不同订单错误投影到同一快照键。
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
                    // 更新余额前先校验账户+资产维度不变，避免把快照写到错误的账户资产键。
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
                BlockEntityChange::SpotTradeCreated(_) => {}
            }
        }

        // 所有明细和投影都成功后再一次性提交事务，保证 block 级原子性。
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
        for item in cursor.iter::<Vec<u8>, Vec<u8>>() {
            let (key, value) =
                item.map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
            if !key.starts_with(&prefix) {
                continue;
            }
            let record: StoredCommandEnvelope = decode_record(&value)?;
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
        for item in cursor.iter::<Vec<u8>, Vec<u8>>() {
            let (key, value) =
                item.map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
            if !key.starts_with(&prefix) {
                continue;
            }
            let record: StoredReplayableEvent = decode_record(&value)?;
            events.push(record.decode_event()?);
        }
        txn.commit().map_err(|error| VeldraMdbxStorageError::Read(Box::new(error)))?;
        Ok(events)
    }

    pub fn load_current_spot_order(
        &self,
        order_id: &str,
    ) -> Result<Option<SpotOrderV2>, VeldraMdbxStorageError> {
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
        for table_name in TABLE_NAMES {
            txn.create_table(Some(table_name), TableFlags::empty())
                .map_err(|error| VeldraMdbxStorageError::Open(Box::new(error)))?;
        }
        txn.commit().map_err(|error| VeldraMdbxStorageError::Open(Box::new(error)))?;
        Ok(())
    }
}
