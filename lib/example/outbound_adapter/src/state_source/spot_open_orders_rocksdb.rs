use std::path::Path;

use common_entity::{LogEntry, StateSource, stable_string_entity_id};
use example_core_entity::common::{OrderBase, identifiers::OrderId};
use example_core_entity::{BalanceV2, OrderSide, SpotOrderV7};
use example_core_use_case::use_case::trading::spot::v3::pipeline::cancel_spot_order::{
    CancelSpotOrderV3Cmd, CancelSpotOrderV3State, CancelSpotOrderV3UseCase,
};
use rocksdb::{DB, Options, WriteBatch};
use thiserror::Error;
use example_core_use_case::BalanceV2;
use example_core_use_case::common::OrderBase;

const KEY_VERSION: u8 = 1;
const OID_INDEX: u8 = 1;
const CLOID_INDEX: u8 = 2;
const ORDER: u8 = 3;
const BALANCE: u8 = 4;

#[derive(Debug, Error)]
pub enum CancelSpotOrderV3RocksDbError {
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] rocksdb::Error),
    #[error("state codec error: {0}")]
    Codec(#[from] bincode::Error),
    #[error("order not found for user {user_id}")]
    OrderNotFound { user_id: String },
    #[error("balance {coin_idx} not found for user {user_id}")]
    BalanceNotFound { user_id: String, coin_idx: u64 },
    #[error("cloid index {cloid} for user {user_id} points to missing order {oid}")]
    DanglingCloidIndex { user_id: String, cloid: String, oid: u64 },
    #[error("order owner {actual} does not match user {user_id} ({expected})")]
    OrderOwnerMismatch { user_id: String, expected: u64, actual: u64 },
    #[error("balance party {actual} does not match user {user_id} ({expected})")]
    BalancePartyMismatch { user_id: String, expected: u64, actual: u64 },
    #[error("cloid {cloid} for user {user_id} already belongs to order {existing_oid}")]
    CloidConflict { user_id: String, cloid: String, existing_oid: u64 },
    #[error("stored order is invalid: {0}")]
    InvalidOrder(#[from] common_entity::EntityError),
}

pub struct CancelSpotOrderV3UseCaseRocksDB {
    db: DB,
}

impl CancelSpotOrderV3UseCaseRocksDB {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CancelSpotOrderV3RocksDbError> {
        let mut options = Options::default();
        options.create_if_missing(true);
        Ok(Self { db: DB::open(&options, path)? })
    }

    pub fn upsert_order(
        &self,
        user_id: &str,
        order: &SpotOrderV7,
    ) -> Result<(), CancelSpotOrderV3RocksDbError> {
        let expected = stable_string_entity_id(user_id);
        let record = order.order_base();
        if record.owner.0 != expected {
            return Err(CancelSpotOrderV3RocksDbError::OrderOwnerMismatch {
                user_id: user_id.to_owned(),
                expected,
                actual: record.owner.0,
            });
        }
        let oid = record.order_id;
        let order_key = order_key(user_id, oid);
        let old_record = self
            .db
            .get(&order_key)?
            .map(|bytes| bincode::deserialize::<OrderBase>(&bytes))
            .transpose()?;
        if let Some(cloid) = record.cloid.as_deref() {
            if let Some(bytes) = self.db.get(cloid_key(user_id, cloid))? {
                let existing_oid: u64 = bincode::deserialize(&bytes)?;
                if existing_oid != oid {
                    return Err(CancelSpotOrderV3RocksDbError::CloidConflict {
                        user_id: user_id.to_owned(),
                        cloid: cloid.to_owned(),
                        existing_oid,
                    });
                }
            }
        }

        let mut batch = WriteBatch::default();
        batch.put(&order_key, bincode::serialize(record)?);
        batch.put(oid_key(user_id, oid), bincode::serialize(&oid)?);
        if let Some(old_cloid) = old_record.as_ref().and_then(|old| old.cloid.as_deref()) {
            if record.cloid.as_deref() != Some(old_cloid) {
                batch.delete(cloid_key(user_id, old_cloid));
            }
        }
        if let Some(cloid) = record.cloid.as_deref() {
            batch.put(cloid_key(user_id, cloid), bincode::serialize(&oid)?);
        }
        self.db.write(batch)?;
        Ok(())
    }

    pub fn upsert_balance(
        &self,
        user_id: &str,
        balance: &BalanceV2,
    ) -> Result<(), CancelSpotOrderV3RocksDbError> {
        let expected = stable_string_entity_id(user_id);
        if balance.party_id() != expected {
            return Err(CancelSpotOrderV3RocksDbError::BalancePartyMismatch {
                user_id: user_id.to_owned(),
                expected,
                actual: balance.party_id(),
            });
        }
        self.db.put(balance_key(user_id, balance.coin_idx()), bincode::serialize(balance)?)?;
        Ok(())
    }
}

impl StateSource<CancelSpotOrderV3UseCase> for CancelSpotOrderV3UseCaseRocksDB {
    type Error = CancelSpotOrderV3RocksDbError;

    fn load_given_state(
        &self,
        cmd: &LogEntry<CancelSpotOrderV3Cmd>,
    ) -> Result<CancelSpotOrderV3State, Self::Error> {
        let user_id = cmd.user_id();
        let snapshot = self.db.snapshot();
        let (oid, from_cloid) = match &cmd.order_id {
            OrderId::Oid(oid) => {
                let bytes = snapshot.get(oid_key(user_id, *oid))?.ok_or_else(|| {
                    CancelSpotOrderV3RocksDbError::OrderNotFound { user_id: user_id.to_owned() }
                })?;
                (bincode::deserialize(&bytes)?, None)
            }
            OrderId::Cloid(cloid) => {
                let bytes = snapshot.get(cloid_key(user_id, cloid))?.ok_or_else(|| {
                    CancelSpotOrderV3RocksDbError::OrderNotFound { user_id: user_id.to_owned() }
                })?;
                (bincode::deserialize(&bytes)?, Some(cloid.as_str()))
            }
        };
        let order_bytes = snapshot.get(order_key(user_id, oid))?.ok_or_else(|| {
            if let Some(cloid) = from_cloid {
                CancelSpotOrderV3RocksDbError::DanglingCloidIndex {
                    user_id: user_id.to_owned(),
                    cloid: cloid.to_owned(),
                    oid,
                }
            } else {
                CancelSpotOrderV3RocksDbError::OrderNotFound { user_id: user_id.to_owned() }
            }
        })?;
        let record: OrderBase = bincode::deserialize(&order_bytes)?;
        let expected = stable_string_entity_id(user_id);
        if record.owner.0 != expected {
            return Err(CancelSpotOrderV3RocksDbError::OrderOwnerMismatch {
                user_id: user_id.to_owned(),
                expected,
                actual: record.owner.0,
            });
        }
        let coin_idx = u64::from(match record.side {
            OrderSide::Buy => record.symbol.quote.index(),
            OrderSide::Sell => record.symbol.base.index(),
        });
        let order = SpotOrderV7::from_record(record)?;
        let balance_bytes = snapshot.get(balance_key(user_id, coin_idx))?.ok_or_else(|| {
            CancelSpotOrderV3RocksDbError::BalanceNotFound {
                user_id: user_id.to_owned(),
                coin_idx,
            }
        })?;
        let balance: BalanceV2 = bincode::deserialize(&balance_bytes)?;
        if balance.party_id() != expected {
            return Err(CancelSpotOrderV3RocksDbError::BalancePartyMismatch {
                user_id: user_id.to_owned(),
                expected,
                actual: balance.party_id(),
            });
        }
        Ok(CancelSpotOrderV3State { order, balance })
    }
}

fn prefix(namespace: u8, user_id: &str) -> Vec<u8> {
    let bytes = user_id.as_bytes();
    let mut key = Vec::with_capacity(6 + bytes.len());
    key.push(KEY_VERSION);
    key.push(namespace);
    key.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    key.extend_from_slice(bytes);
    key
}

fn oid_key(user_id: &str, oid: u64) -> Vec<u8> {
    let mut key = prefix(OID_INDEX, user_id);
    key.extend_from_slice(&oid.to_be_bytes());
    key
}

fn cloid_key(user_id: &str, cloid: &str) -> Vec<u8> {
    let mut key = prefix(CLOID_INDEX, user_id);
    key.extend_from_slice(&(cloid.len() as u32).to_be_bytes());
    key.extend_from_slice(cloid.as_bytes());
    key
}

fn order_key(user_id: &str, oid: u64) -> Vec<u8> {
    let mut key = prefix(ORDER, user_id);
    key.extend_from_slice(&oid.to_be_bytes());
    key
}

fn balance_key(user_id: &str, coin_idx: u64) -> Vec<u8> {
    let mut key = prefix(BALANCE, user_id);
    key.extend_from_slice(&coin_idx.to_be_bytes());
    key
}
