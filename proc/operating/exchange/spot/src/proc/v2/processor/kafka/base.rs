use std::sync::Arc;
use std::time::Duration;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

#[derive(Debug, Clone)]
pub struct KafkaConsumerConfig {
    pub kafka_brokers: String,
    pub topic: String,
    pub group_id: String,
    pub session_timeout_ms: u32,
    pub auto_commit_interval_ms: u32,
}

#[derive(Debug, Clone)]
pub struct KafkaProcessorConfig {
    pub kafka_brokers: String,
    pub group_id: String,
    pub session_timeout_ms: u32,
    pub auto_commit_interval_ms: u32,
}

impl Default for KafkaProcessorConfig {
    fn default() -> Self {
        Self {
            kafka_brokers: "localhost:9092".to_string(),
            group_id: "kafka-processor-group".to_string(),
            session_timeout_ms: 6000,
            auto_commit_interval_ms: 5000,
        }
    }
}

impl KafkaProcessorConfig {
    pub fn new(kafka_brokers: impl Into<String>, group_id: impl Into<String>) -> Self {
        Self {
            kafka_brokers: kafka_brokers.into(),
            group_id: group_id.into(),
            session_timeout_ms: 6000,
            auto_commit_interval_ms: 5000,
        }
    }
}

impl KafkaConsumerConfig {
    pub fn new(
        kafka_brokers: impl Into<String>,
        topic: impl Into<String>,
        group_id: impl Into<String>,
    ) -> Self {
        Self {
            kafka_brokers: kafka_brokers.into(),
            topic: topic.into(),
            group_id: group_id.into(),
            session_timeout_ms: 6000,
            auto_commit_interval_ms: 5000,
        }
    }

    pub fn with_session_timeout(mut self, ms: u32) -> Self {
        self.session_timeout_ms = ms;
        self
    }

    pub fn with_auto_commit_interval(mut self, ms: u32) -> Self {
        self.auto_commit_interval_ms = ms;
        self
    }
}

pub fn create_kafka_consumer(config: &KafkaConsumerConfig) -> Result<StreamConsumer, String> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("group.id", &config.group_id)
        .set("enable.auto.commit", "true")
        .set("auto.commit.interval.ms", config.auto_commit_interval_ms.to_string())
        .set("session.timeout.ms", config.session_timeout_ms.to_string())
        .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .map_err(|e| format!("Failed to create Kafka consumer: {}", e))?;

    consumer
        .subscribe(&[&config.topic])
        .map_err(|e| format!("Failed to subscribe to topic: {}", e))?;

    Ok(consumer)
}

#[inline]
pub fn deserialize_change_log(bytes: &[u8]) -> Result<diff::ChangeLogEntry, SpotCmdErrorAny> {
    serde_json::from_slice(bytes).map_err(|e| {
        tracing::error!(error = ?e, bytes_len = bytes.len(), "Failed to deserialize change log");
        SpotCmdErrorAny::Common(CommonError::Internal {
            message: format!("Deserialization error: {}", e),
        })
    })
}

pub trait KafkaProcessor: Send + Sync {
    fn consumer(&self) -> &StreamConsumer;
    fn topic(&self) -> &str;
    fn group_id(&self) -> &str;
    fn kafka_brokers(&self) -> &str;

    fn handle_message(&self, payload: &[u8]) -> impl std::future::Future<Output = Result<(), SpotCmdErrorAny>>;

    async fn start(&self) {
        tracing::info!(
            kafka_brokers = %self.kafka_brokers(),
            topic = %self.topic(),
            group_id = %self.group_id(),
            "Starting Kafka processor"
        );

        loop {
            match self.consumer().recv().await {
                Ok(message) => {
                    if let Some(payload) = message.payload() {
                        if let Err(e) = self.handle_message(payload).await {
                            tracing::error!(
                                error = ?e,
                                offset = message.offset(),
                                partition = message.partition(),
                                "Failed to handle message"
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to receive message from Kafka");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    fn start_background(self: Arc<Self>) -> tokio::task::JoinHandle<()>
    where
        Self: 'static,
    {
        // Use spawn_blocking since rdkafka's StreamConsumer is not Send
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async move {
                self.start().await;
            });
        })
    }
}
