use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::{AccountId, AssetId};
use db_repo::MySqlDbRepo;
use diff::ChangeLogEntry;
use rdkafka::consumer::StreamConsumer;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;
use crate::proc::v2::processor::kafka::base::{create_kafka_consumer, deserialize_change_log, KafkaConsumerConfig, KafkaProcessor, KafkaProcessorConfig};

#[derive(Debug, Clone)]
pub struct SettlementResult {
    pub balance_logs: Vec<ChangeLogEntry>,
    pub success: bool,
}

impl SettlementResult {
    pub fn success(balance_logs: Vec<ChangeLogEntry>) -> Self {
        Self { balance_logs, success: true }
    }

    pub fn failure() -> Self {
        Self { balance_logs: Vec::new(), success: false }
    }
}

pub struct KafkaSettlementProcessor {
    consumer: Arc<StreamConsumer>,
    balance_repo: Arc<MySqlDbRepo<Balance>>,
    event_publisher: Arc<dyn EventPublisher>,
    config: KafkaProcessorConfig,
    topic: String,
    buffer_size: usize,
}

impl KafkaSettlementProcessor {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        event_publisher: Arc<dyn EventPublisher>,
        config: KafkaProcessorConfig,
        topic: String,
        buffer_size: usize,
    ) -> Result<Self, String> {
        let kafka_config = KafkaConsumerConfig::new(
            &config.kafka_brokers,
            &topic,
            &config.group_id,
        )
        .with_session_timeout(config.session_timeout_ms)
        .with_auto_commit_interval(config.auto_commit_interval_ms);

        let consumer = create_kafka_consumer(&kafka_config)?;

        Ok(Self {
            consumer: Arc::new(consumer),
            balance_repo,
            event_publisher,
            config,
            topic,
            buffer_size,
        })
    }
}

impl KafkaProcessor for KafkaSettlementProcessor {
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
        let trade_log: ChangeLogEntry = deserialize_change_log(payload)?;

        tracing::debug!(
            entity_id = %trade_log.entity_id(),
            change_type = ?trade_log.change_type(),
            "Received trade log from Kafka"
        );

        let trade = self.reconstruct_trade(&trade_log)?;
        let settlement_result = self.settle_trade(&trade)?;

        if settlement_result.success && !settlement_result.balance_logs.is_empty() {
            if let Err(e) = self.event_publisher.publish_balance_logs(&settlement_result.balance_logs) {
                tracing::error!(error = ?e, "Failed to publish balance logs to Kafka");
            }
        }

        Ok(())
    }
}

impl KafkaSettlementProcessor {
    fn reconstruct_trade(&self, trade_log: &ChangeLogEntry) -> Result<SpotTrade, SpotCmdErrorAny> {
        tracing::warn!(entity_id = %trade_log.entity_id(), "Trade reconstruction not implemented yet");

        Err(SpotCmdErrorAny::Common(
            crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
                message: "Trade reconstruction not implemented".to_string(),
            },
        ))
    }

    fn settle_trade(&self, trade: &SpotTrade) -> Result<SettlementResult, SpotCmdErrorAny> {
        tracing::debug!(
            trade_id = %trade.trade_id,
            taker_order_id = %trade.taker_order_id,
            maker_order_id = %trade.maker_order_id,
            "Starting trade settlement"
        );

        let mut balance_logs = Vec::new();
        let taker_logs = self.settle_taker_side(trade)?;
        balance_logs.extend(taker_logs);
        let maker_logs = self.settle_maker_side(trade)?;
        balance_logs.extend(maker_logs);

        tracing::info!(trade_id = %trade.trade_id, balance_log_count = balance_logs.len(), "Trade settlement completed");

        Ok(SettlementResult::success(balance_logs))
    }

    fn settle_taker_side(&self, trade: &SpotTrade) -> Result<Vec<ChangeLogEntry>, SpotCmdErrorAny> {
        tracing::warn!("Taker settlement not implemented yet");
        Ok(Vec::new())
    }

    fn settle_maker_side(&self, trade: &SpotTrade) -> Result<Vec<ChangeLogEntry>, SpotCmdErrorAny> {
        tracing::warn!("Maker settlement not implemented yet");
        Ok(Vec::new())
    }

    #[inline]
    fn build_balance_id(&self, account_id: AccountId, asset_id: AssetId) -> String {
        format!("{}:{}", account_id.0, u32::from(asset_id))
    }
}

pub struct KafkaSettlementProcessorFactory;

impl KafkaSettlementProcessorFactory {
    pub fn create_and_start(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        event_publisher: Arc<dyn EventPublisher>,
        config: KafkaProcessorConfig,
        topic: String,
        buffer_size: usize,
    ) -> Result<tokio::task::JoinHandle<()>, String> {
        let processor = KafkaSettlementProcessor::new(balance_repo, event_publisher, config, topic, buffer_size)?;

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

    #[test]
    fn test_settlement_result() {
        let result = SettlementResult::success(vec![]);
        assert!(result.success);
        assert_eq!(result.balance_logs.len(), 0);

        let result = SettlementResult::failure();
        assert!(!result.success);
    }
}
