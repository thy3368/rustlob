use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};

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
