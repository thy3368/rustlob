use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::SpotTrade;
use db_repo::MySqlDbRepo;
use diff::ChangeLog;
use rdkafka::consumer::StreamConsumer;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::trade_cmd_handlers::settlement_handler::{SettlementHandler, SettlementResult};
use crate::proc::v2::processor::kafka::base::{
    create_kafka_consumer, deserialize_change_log, KafkaConsumerConfig, KafkaProcessor, KafkaProcessorConfig,
};

pub struct KafkaSettlementProcessor {
    consumer: Arc<StreamConsumer>,
    handler: Arc<dyn SettlementHandler>,
    config: KafkaProcessorConfig,
    topic: String,
}

impl KafkaSettlementProcessor {
    pub fn new(
        handler: Arc<dyn SettlementHandler>,
        config: KafkaProcessorConfig,
        topic: String,
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
            handler,
            config,
            topic,
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
        let trade_log: ChangeLog = deserialize_change_log(payload)?;

        tracing::debug!(
            entity_id = %trade_log.entity_id(),
            change_type = ?trade_log.change_type(),
            "Received trade log from Kafka"
        );

        let trade = self.handler.reconstruct_trade(&trade_log)?;
        let settlement_result = self.handler.settle_trade(&trade)?;

        if settlement_result.success && !settlement_result.balance_logs.is_empty() {
            self.handler.publish_balance_logs(&settlement_result.balance_logs);
        }

        Ok(())
    }
}

pub struct KafkaSettlementProcessorFactory;

impl KafkaSettlementProcessorFactory {
    pub fn create_and_start(
        handler: Arc<dyn SettlementHandler>,
        config: KafkaProcessorConfig,
        topic: String,
    ) -> Result<tokio::task::JoinHandle<()>, String> {
        let processor = KafkaSettlementProcessor::new(handler, config, topic)?;
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