//! Common persistence utilities for message processors.
//!
//! This module provides shared functionality for persisting change logs to RocksDB
//! and replaying them to MySQL. Both Kafka and NATS processors can use these utilities.

use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::ChangeLog;
use rocksdb::{DB, Options};
use serde::de::DeserializeOwned;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

/// Key-value store for persisting change logs
pub struct ChangeLogStore {
    db: Arc<DB>,
}

impl ChangeLogStore {
    /// Open or create a RocksDB database at the given path
    pub fn new(db_path: &str) -> Result<Self, String> {
        let mut db_options = Options::default();
        db_options.create_if_missing(true);

        let db = DB::open(&db_options, db_path)
            .map_err(|e| format!("Failed to open RocksDB at {}: {}", db_path, e))?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Persist a change log entry to RocksDB
    pub fn persist(
        &self,
        entity_type: &str,
        entity_id: &str,
        log: &ChangeLog,
    ) -> Result<(), String> {
        let key = format!("{}:{}", entity_type, entity_id);
        let value =
            serde_json::to_vec(log).map_err(|e| format!("Failed to serialize log: {}", e))?;

        self.db
            .put(key.as_bytes(), value.as_slice())
            .map_err(|e| format!("Failed to persist to RocksDB: {}", e))
    }

    /// Retrieve a change log entry by entity type and ID
    pub fn get<E: DeserializeOwned>(&self, entity_type: &str, entity_id: &str) -> Option<E> {
        let key = format!("{}:{}", entity_type, entity_id);
        self.db
            .get(key.as_bytes())
            .ok()
            .flatten()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
    }

    /// List all entity IDs for a given entity type
    pub fn list_keys(&self, entity_type: &str) -> Vec<String> {
        let prefix = format!("{}:", entity_type);
        self.db
            .prefix_iterator(prefix.as_bytes())
            .flatten()
            .filter_map(|(k, _)| {
                String::from_utf8(k.to_vec())
                    .ok()
                    .and_then(|s| s.strip_prefix(&prefix).map(|id| id.to_string()))
            })
            .collect()
    }

    /// Get the latest sequence number stored
    pub fn get_latest_sequence(&self) -> Option<u64> {
        self.db
            .get("meta:latest_sequence")
            .ok()
            .flatten()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|s| s.parse().ok())
    }

    /// Set the latest sequence number
    pub fn set_latest_sequence(&self, sequence: u64) -> Result<(), String> {
        self.db
            .put("meta:latest_sequence", sequence.to_string().as_bytes())
            .map_err(|e| format!("Failed to update sequence: {}", e))
    }

    /// Update sequence number only if the new sequence is higher
    pub fn update_sequence_if_higher(&self, new_sequence: u64) -> Result<(), String> {
        let current_seq = self.get_latest_sequence().unwrap_or(0);
        if new_sequence > current_seq {
            self.set_latest_sequence(new_sequence)?;
        }
        Ok(())
    }
}

/// MySQL repository for replaying change logs
pub struct ChangeLogReplay {
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,
    trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
    balance_repo: Arc<MySqlDbRepo<Balance>>,
}

impl ChangeLogReplay {
    /// Create a new ChangeLogReplay with the given repositories
    pub fn new(
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        balance_repo: Arc<MySqlDbRepo<Balance>>,
    ) -> Self {
        Self { order_repo, trade_repo, balance_repo }
    }

    /// Replay a change log entry to MySQL based on entity type
    pub fn replay(&self, log: &ChangeLog) -> Result<(), String> {
        let entity_type = log.entity_type();

        match entity_type.as_str() {
            "SpotOrder" | "Order" => {
                self.order_repo
                    .replay_event(log)
                    .map_err(|e| format!("Failed to replay order to MySQL: {}", e))?;
            }
            "SpotTrade" | "Trade" => {
                self.trade_repo
                    .replay_event(log)
                    .map_err(|e| format!("Failed to replay trade to MySQL: {}", e))?;
            }
            "Balance" => {
                self.balance_repo
                    .replay_event(log)
                    .map_err(|e| format!("Failed to replay balance to MySQL: {}", e))?;
            }
            _ => {
                tracing::warn!(entity_type = %entity_type, "Unknown entity type, skipping MySQL replay");
            }
        }

        Ok(())
    }
}

/// Common error type for persistence operations
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("RocksDB error: {0}")]
    RocksDb(String),

    #[error("MySQL replay error: {0}")]
    MySqlReplay(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<PersistenceError> for SpotCmdErrorAny {
    fn from(e: PersistenceError) -> Self {
        SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
    }
}

/// Trait for processors that handle change log messages
pub trait ChangeLogProcessor: Send + Sync + 'static {
    /// Process a single change log entry
    fn process_change_log(
        &self,
        log: &ChangeLog,
    ) -> impl std::future::Future<Output = Result<(), SpotCmdErrorAny>> + Send;
}

/// Helper function to deserialize change log from bytes
pub fn deserialize_change_log(bytes: &[u8]) -> Result<ChangeLog, SpotCmdErrorAny> {
    serde_json::from_slice(bytes).map_err(|e| {
        tracing::error!(error = ?e, bytes_len = bytes.len(), "Failed to deserialize change log");
        SpotCmdErrorAny::Common(CommonError::Internal {
            message: format!("Deserialization error: {}", e),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_change_log_valid() {
        let json = r#"{
            "timestamp": 1234567890,
            "sequence": 1,
            "old_version": 0,
            "new_version": 1,
            "entity_id": 100,
            "entity_type": "SpotOrder",
            "change_type": 0,
            "field_changes": []
        }"#;

        let result: Result<ChangeLog, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_change_log_invalid() {
        let json = "not valid json";
        let result = deserialize_change_log(json.as_bytes());
        assert!(result.is_err());
    }
}
