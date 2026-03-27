use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotOrder;
use diff::ChangeLog;
use rdkafka::consumer::StreamConsumer;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;
use crate::proc::v2::processor::kafka::base::{create_kafka_consumer, deserialize_change_log, KafkaConsumerConfig, KafkaProcessor, KafkaProcessorConfig};
use crate::proc::v2::trade_handlers::matching_handler::{MatchResult, MatchingHandler};

pub struct KafkaMatchingProcessor {
    consumer: Arc<StreamConsumer>,
    matching_engine: Arc<MatchingHandler>,
    event_publisher: Arc<dyn EventPublisher>,
    config: KafkaProcessorConfig,
    topic: String,
}

impl KafkaMatchingProcessor {
    pub fn new(
        matching_engine: Arc<MatchingHandler>,
        event_publisher: Arc<dyn EventPublisher>,
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
            matching_engine,
            event_publisher,
            config,
            topic,
        })
    }
}

impl KafkaProcessor for KafkaMatchingProcessor {
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
        let order_log: ChangeLog = deserialize_change_log(payload)?;

        tracing::debug!(
            entity_id = %order_log.entity_id(),
            change_type = ?order_log.change_type(),
            "Received order change log from Kafka"
        );

        let order = self.reconstruct_order(&order_log)?;
        let match_result = self.matching_engine.match_order(order)?;

        self.publish_match_result(match_result).await
    }
}

impl KafkaMatchingProcessor {
    fn reconstruct_order(&self, order_log: &ChangeLog) -> Result<SpotOrder, SpotCmdErrorAny> {
        tracing::warn!(
            entity_id = %order_log.entity_id(),
            "Order reconstruction not implemented yet"
        );

        Err(SpotCmdErrorAny::Common(
            crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
                message: "Order reconstruction not implemented".to_string(),
            },
        ))
    }

    async fn publish_match_result(&self, match_result: MatchResult) -> Result<(), SpotCmdErrorAny> {
        if !match_result.order_logs.is_empty() {
            if let Err(e) = self.event_publisher.publish_order_logs(&match_result.order_logs) {
                tracing::error!(
                    count = match_result.order_logs.len(),
                    error = ?e,
                    "Failed to publish order logs to Kafka"
                );
            }
        }

        if !match_result.trade_logs.is_empty() {
            if let Err(e) = self.event_publisher.publish_trade_logs(&match_result.trade_logs) {
                tracing::error!(
                    count = match_result.trade_logs.len(),
                    error = ?e,
                    "Failed to publish trade logs to Kafka"
                );
            }
        }

        Ok(())
    }
}

pub struct KafkaMatchingProcessorFactory;

impl KafkaMatchingProcessorFactory {
    pub fn create_and_start(
        matching_engine: Arc<MatchingHandler>,
        event_publisher: Arc<dyn EventPublisher>,
        config: KafkaProcessorConfig,
        topic: String,
    ) -> Result<tokio::task::JoinHandle<()>, String> {
        let processor =
            KafkaMatchingProcessor::new(matching_engine, event_publisher, config, topic)?;

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
