use std::sync::Arc;

use db_repo::core::event_publish::{EventPublisher2, PublishError};
use diff::diff_types::DomainEvent;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

pub struct KafkaProducer {
    producer: Arc<FutureProducer>,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: impl Into<String>, topic: impl Into<String>) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers.into())
            .set("acks", "all")
            .create()
            .expect("Failed to create Kafka producer");

        Self { producer: Arc::new(producer), topic: topic.into() }
    }

    pub fn default_local(topic: impl Into<String>) -> Self {
        Self::new("localhost:9092", topic)
    }
}

impl EventPublisher2 for KafkaProducer {
    fn publish<E: Serialize>(&self, event: &DomainEvent<E>) -> Result<(), PublishError> {
        let payload = serde_json::to_vec(event).map_err(|e| PublishError(e.to_string()))?;
        let record = FutureRecord::to(&self.topic).payload(&payload);

        self.producer
            .send(record, std::time::Duration::from_secs(5))
            .map_err(|e| PublishError(e.to_string()))
            .map(|_| ())
    }

    fn publish_batch<E: Serialize>(&self, events: &[DomainEvent<E>]) -> Result<(), PublishError> {
        for event in events {
            self.publish(event)?;
        }
        Ok(())
    }
}
