use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use db_repo::MySqlDbRepo;
use diff::ChangeLogEntry;
use rdkafka::consumer::StreamConsumer;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::v2::processor::common::{
    deserialize_change_log, ChangeLogReplay, ChangeLogStore,
};
use crate::proc::v2::processor::kafka::base::{
    create_kafka_consumer, KafkaConsumerConfig, KafkaProcessor, KafkaProcessorConfig,
};

pub struct PersistentProcessor {
    consumer: Arc<StreamConsumer>,
    store: Arc<ChangeLogStore>,
    replay: Arc<ChangeLogReplay>,
    config: KafkaProcessorConfig,
    topic: String,
}

impl PersistentProcessor {
    pub fn new(
        config: KafkaProcessorConfig,
        topic: String,
        db_path: &str,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        balance_repo: Arc<MySqlDbRepo<Balance>>,
    ) -> Result<Self, String> {
        let kafka_config = KafkaConsumerConfig::new(
            &config.kafka_brokers,
            &topic,
            &config.group_id,
        )
        .with_session_timeout(config.session_timeout_ms)
        .with_auto_commit_interval(config.auto_commit_interval_ms);

        let consumer = create_kafka_consumer(&kafka_config)?;
        let store = ChangeLogStore::new(db_path)?;
        let replay = ChangeLogReplay::new(order_repo, trade_repo, balance_repo);

        Ok(Self {
            consumer: Arc::new(consumer),
            store: Arc::new(store),
            replay: Arc::new(replay),
            config,
            topic,
        })
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

impl KafkaProcessor for PersistentProcessor {
    fn consumer(&self) -> &StreamConsumer {
        &self.consumer
    }

    fn topic(&self) -> &str {
        &self.topic
    }

    fn group_id(&self) -> &str {
        &self.config.group_id
    }

    fn kafka_brokers(&self) -> &str {
        &self.config.kafka_brokers
    }

    async fn handle_message(&self, payload: &[u8]) -> Result<(), SpotCmdErrorAny> {
        let log: ChangeLogEntry = deserialize_change_log(payload)?;

        tracing::debug!(
            entity_id = %log.entity_id(),
            entity_type = %log.entity_type(),
            sequence = log.sequence(),
            change_type = ?log.change_type(),
            "Processing change log"
        );

        let entity_type = log.entity_type();
        let entity_id = log.entity_id();

        self.store
            .persist(&entity_type, &entity_id, &log)
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to persist to RocksDB");
                SpotCmdErrorAny::Common(CommonError::Internal { message: e })
            })?;

        self.replay
            .replay(&log)
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to replay to MySQL");
                SpotCmdErrorAny::Common(CommonError::Internal { message: e })
            })?;

        self.store
            .update_sequence_if_higher(*log.sequence())
            .map_err(|e| SpotCmdErrorAny::Common(CommonError::Internal { message: e }))?;

        Ok(())
    }
}

pub struct PersistentProcessorFactory;

impl PersistentProcessorFactory {
    pub fn create_and_start(
        config: KafkaProcessorConfig,
        topic: String,
        db_path: &str,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        balance_repo: Arc<MySqlDbRepo<Balance>>,
    ) -> Result<tokio::task::JoinHandle<()>, String> {
        let processor = PersistentProcessor::new(
            config, topic, db_path, order_repo, trade_repo, balance_repo,
        )?;
        Ok(Arc::new(processor).start_background())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = KafkaProcessorConfig::default();
        assert_eq!(config.kafka_brokers, "localhost:9092");
        assert_eq!(config.group_id, "kafka-processor-group");
    }
}