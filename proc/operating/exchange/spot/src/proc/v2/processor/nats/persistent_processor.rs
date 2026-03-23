use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use db_repo::MySqlDbRepo;
use diff::ChangeLog;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

use crate::proc::v2::processor::nats::base::{NatsProcessor, NatsProcessorConfig};
use crate::proc::v2::trade_handlers::change_log_store::{deserialize_change_log, ChangeLogReplay, ChangeLogStore};

pub struct NatsPersistentProcessor {
    client: Arc<Client>,
    store: Arc<ChangeLogStore>,
    replay: Arc<ChangeLogReplay>,
    config: NatsProcessorConfig,
    subject: String,
}

impl NatsPersistentProcessor {
    pub fn new(
        config: NatsProcessorConfig,
        subject: String,
        db_path: &str,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        balance_repo: Arc<MySqlDbRepo<Balance>>,
    ) -> Result<Self, String> {
        let store = ChangeLogStore::new(db_path)?;
        let replay = ChangeLogReplay::new(order_repo, trade_repo, balance_repo);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;

        let client = rt.block_on(async_nats::connect(&config.nats_url))
            .map_err(|e| format!("Failed to connect to NATS at {}: {}", config.nats_url, e))?;

        Ok(Self {
            client: Arc::new(client),
            store: Arc::new(store),
            replay: Arc::new(replay),
            config,
            subject,
        })
    }

    fn handle_change_log(&self, log: &ChangeLog) -> Result<(), SpotCmdErrorAny> {
        let entity_type = log.entity_type();
        let entity_id = log.entity_id();
        let sequence = log.sequence();

        tracing::debug!(
            entity_id = %entity_id,
            entity_type = %entity_type,
            sequence = sequence,
            change_type = ?log.change_type(),
            "Processing change log"
        );

        self.store
            .persist(&entity_type, &entity_id, log)
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to persist to RocksDB");
                SpotCmdErrorAny::Common(CommonError::Internal { message: e })
            })?;

        self.replay
            .replay(log)
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to replay to MySQL");
                SpotCmdErrorAny::Common(CommonError::Internal { message: e })
            })?;

        self.store
            .update_sequence_if_higher(*sequence)
            .map_err(|e| SpotCmdErrorAny::Common(CommonError::Internal { message: e }))?;

        Ok(())
    }

    pub fn get<E: serde::de::DeserializeOwned>(&self, entity_type: &str, entity_id: &str) -> Option<E> {
        self.store.get(entity_type, entity_id)
    }

    pub fn list_keys(&self, entity_type: &str) -> Vec<String> {
        self.store.list_keys(entity_type)
    }

    pub fn get_latest_sequence(&self) -> Option<u64> {
        self.store.get_latest_sequence()
    }
}

impl NatsProcessor for NatsPersistentProcessor {
    fn client(&self) -> &Client {
        &self.client
    }

    fn subject(&self) -> &str {
        &self.subject
    }

    fn nats_url(&self) -> &str {
        &self.config.nats_url
    }

    async fn handle_message(&self, payload: &[u8]) -> Result<(), SpotCmdErrorAny> {
        let log: ChangeLog = deserialize_change_log(payload)?;
        self.handle_change_log(&log)
    }
}

pub struct NatsPersistentProcessorFactory;

impl NatsPersistentProcessorFactory {
    pub fn create_and_start(
        config: NatsProcessorConfig,
        subject: String,
        db_path: &str,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        balance_repo: Arc<MySqlDbRepo<Balance>>,
    ) -> Result<tokio::task::JoinHandle<()>, String> {
        let processor = NatsPersistentProcessor::new(
            config,
            subject,
            db_path,
            order_repo,
            trade_repo,
            balance_repo,
        )?;
        Ok(Arc::new(processor).start_background())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = NatsProcessorConfig::default();
        assert_eq!(config.nats_url, "nats://localhost:4222");
    }
}