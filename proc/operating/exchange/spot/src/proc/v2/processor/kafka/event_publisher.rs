use std::sync::Arc;

use diff::ChangeLogEntry;
use futures::{FutureExt, TryFutureExt};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

use crate::proc::behavior::v2::spot_trade_behavior_v2::SpotTradeCmdOrQuery;

#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Kafka send error: {0}")]
    KafkaSend(String),

    #[error("Backend not available: {0}")]
    BackendUnavailable(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublisherBackend {
    Kafka,
}

#[derive(Debug, Clone)]
pub struct PublisherConfig {
    pub kafka_brokers: String,
    pub order_log_topic: String,
    pub balance_log_topic: String,
    pub trade_log_topic: String,
    pub enable_batch: bool,
    pub batch_size: usize,
    pub send_timeout_ms: u32,
}

impl PublisherConfig {
    const DEFAULT_TOPICS: (&'static str, &'static str, &'static str) =
        ("order-logs", "balance-logs", "trade-logs");

    pub fn persistent(kafka_brokers: impl Into<String>) -> Self {
        Self {
            kafka_brokers: kafka_brokers.into(),
            order_log_topic: Self::DEFAULT_TOPICS.0.to_string(),
            balance_log_topic: Self::DEFAULT_TOPICS.1.to_string(),
            trade_log_topic: Self::DEFAULT_TOPICS.2.to_string(),
            enable_batch: true,
            batch_size: 100,
            send_timeout_ms: 5000,
        }
    }

    pub fn low_latency(kafka_brokers: impl Into<String>) -> Self {
        Self {
            kafka_brokers: kafka_brokers.into(),
            order_log_topic: Self::DEFAULT_TOPICS.0.to_string(),
            balance_log_topic: Self::DEFAULT_TOPICS.1.to_string(),
            trade_log_topic: Self::DEFAULT_TOPICS.2.to_string(),
            enable_batch: true,
            batch_size: 100,
            send_timeout_ms: 100,
        }
    }
}

impl Default for PublisherConfig {
    fn default() -> Self {
        Self::persistent("localhost:9092")
    }
}

pub trait EventPublisher: Send + Sync {
    fn publish_command(&self, log: &SpotTradeCmdOrQuery) -> Result<(), PublishError>;

    fn publish_order_log(&self, log: &ChangeLogEntry) -> Result<(), PublishError>;
    fn publish_balance_log(&self, log: &ChangeLogEntry) -> Result<(), PublishError>;
    fn publish_trade_log(&self, log: &ChangeLogEntry) -> Result<(), PublishError>;
    fn publish_order_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), PublishError>;
    fn publish_balance_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), PublishError>;
    fn publish_trade_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), PublishError>;
}

pub struct KafkaEventPublisher {
    producer: FutureProducer,
    config: PublisherConfig,
}

impl KafkaEventPublisher {
    pub fn new(config: PublisherConfig) -> Result<Self, String> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.kafka_brokers)
            .set("message.timeout.ms", config.send_timeout_ms.to_string())
            .set("queue.buffering.max.messages", (config.batch_size * 10).to_string())
            .set("batch.num.messages", config.batch_size.to_string())
            .create()
            .map_err(|e| format!("Failed to create Kafka producer: {}", e))?;

        Ok(Self { producer, config })
    }

    fn serialize(&self, event: &impl Serialize) -> Result<Vec<u8>, PublishError> {
        Ok(serde_json::to_vec(event)?)
    }

    fn send(&self, topic: &str, event: &ChangeLogEntry) -> Result<(), PublishError> {
        let payload = self.serialize(event)?;
        let record = FutureRecord::to(topic).payload(&payload).key(&());
        let future = self
            .producer
            .send(record, std::time::Duration::from_millis(self.config.send_timeout_ms as u64));
        futures::executor::block_on(future)
            .map_err(|(e, _)| PublishError::KafkaSend(format!("{:?}", e)))?;
        Ok(())
    }

    fn publish(&self, topic: &str, logs: &[ChangeLogEntry]) -> Result<(), PublishError> {
        if logs.is_empty() {
            return Ok(());
        }
        for log in logs {
            self.send(topic, log)?;
        }
        Ok(())
    }
}

impl EventPublisher for KafkaEventPublisher {
    fn publish_command(&self, cmd: &SpotTradeCmdOrQuery) -> Result<(), PublishError> {
        let payload = serde_json::to_vec(cmd).map_err(|e| PublishError::Serialization(e))?;
        let record = FutureRecord::to(&self.config.order_log_topic).payload(&payload).key(&());
        let future = self
            .producer
            .send(record, std::time::Duration::from_millis(self.config.send_timeout_ms as u64));
        futures::executor::block_on(future)
            .map_err(|(e, _)| PublishError::KafkaSend(format!("{:?}", e)))?;
        Ok(())
    }

    fn publish_order_log(&self, log: &ChangeLogEntry) -> Result<(), PublishError> {
        self.send(&self.config.order_log_topic, log)
    }

    fn publish_balance_log(&self, log: &ChangeLogEntry) -> Result<(), PublishError> {
        self.send(&self.config.balance_log_topic, log)
    }

    fn publish_trade_log(&self, log: &ChangeLogEntry) -> Result<(), PublishError> {
        self.send(&self.config.trade_log_topic, log)
    }

    fn publish_order_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), PublishError> {
        self.publish(&self.config.order_log_topic, logs)
    }

    fn publish_balance_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), PublishError> {
        self.publish(&self.config.balance_log_topic, logs)
    }

    fn publish_trade_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), PublishError> {
        self.publish(&self.config.trade_log_topic, logs)
    }
}

pub struct PublisherFactory;

impl PublisherFactory {
    pub fn create_kafka_publisher(
        config: PublisherConfig,
    ) -> Result<Arc<dyn EventPublisher>, String> {
        Ok(Arc::new(KafkaEventPublisher::new(config)?))
    }

    pub fn create(config: PublisherConfig) -> Result<Arc<dyn EventPublisher>, String> {
        Self::create_kafka_publisher(config)
    }
}

pub trait PublisherConfigExt {
    fn backend(self, _: PublisherBackend) -> Self;
    fn order_topic(self, topic: impl Into<String>) -> Self;
    fn balance_topic(self, topic: impl Into<String>) -> Self;
    fn trade_topic(self, topic: impl Into<String>) -> Self;
    fn enable_batch(self, enable: bool) -> Self;
    fn batch_size(self, size: usize) -> Self;
    fn send_timeout(self, ms: u32) -> Self;
}

impl PublisherConfigExt for PublisherConfig {
    fn backend(mut self, _: PublisherBackend) -> Self {
        self
    }

    fn order_topic(mut self, topic: impl Into<String>) -> Self {
        self.order_log_topic = topic.into();
        self
    }

    fn balance_topic(mut self, topic: impl Into<String>) -> Self {
        self.balance_log_topic = topic.into();
        self
    }

    fn trade_topic(mut self, topic: impl Into<String>) -> Self {
        self.trade_log_topic = topic.into();
        self
    }

    fn enable_batch(mut self, enable: bool) -> Self {
        self.enable_batch = enable;
        self
    }

    fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    fn send_timeout(mut self, ms: u32) -> Self {
        self.send_timeout_ms = ms;
        self
    }
}

impl PublisherConfig {
    pub fn into_publisher(self) -> Result<Arc<dyn EventPublisher>, String> {
        PublisherFactory::create(self)
    }
}

#[cfg(test)]
mod tests {
    use diff::ChangeType;

    use super::*;

    fn create_test_log() -> ChangeLogEntry {
        ChangeLogEntry::new(
            "test-123".to_string(),
            "Order".to_string(),
            ChangeType::Created { fields: Vec::new() },
            1000,
            1,
        )
    }

    #[test]
    fn test_config_default() {
        let config = PublisherConfig::default();
        assert_eq!(config.kafka_brokers, "localhost:9092");
        assert_eq!(config.order_log_topic, "order-logs");
    }

    #[test]
    fn test_config_builder() {
        let config = PublisherConfig::default().order_topic("custom-order").send_timeout(5000);
        assert_eq!(config.order_log_topic, "custom-order");
        assert_eq!(config.send_timeout_ms, 5000);
    }
}
