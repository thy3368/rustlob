use db_repo::core::event_publish::{EventPublisher2, PublishError};
use diff::diff_types::{ChangeLog, ChangeType, DomainEvent};
use futures::executor::block_on;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: impl Into<String>, topic: impl Into<String>) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers.into())
            .set("acks", "all")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create Kafka producer");

        Self { producer, topic: topic.into() }
    }

    pub fn default_local(topic: impl Into<String>) -> Self {
        Self::new("localhost:9092", topic)
    }
}

impl EventPublisher2 for KafkaProducer {
    fn publish<E: Serialize>(&self, event: &DomainEvent<E>) -> Result<(), PublishError> {
        let payload = serde_json::to_vec(event)?;
        let record: FutureRecord<'_, (), [u8]> =
            FutureRecord::to(&self.topic).payload(&payload).key(&());

        block_on(self.producer.send(record, std::time::Duration::from_secs(5)))
            .map_err(|e| PublishError(format!("Kafka error: {:?}", e)))?;
        Ok(())
    }

    fn publish_batch<E: Serialize>(&self, events: &[DomainEvent<E>]) -> Result<(), PublishError> {
        for event in events {
            self.publish(event)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestEvent {
        id: String,
        name: String,
    }

    fn create_test_event() -> DomainEvent<TestEvent> {
        let change_log = ChangeLog::new(
            "test-123".to_string(),
            "TestEvent".to_string(),
            ChangeType::Created { fields: vec![] },
            1000,
            1,
        );
        let state = TestEvent { id: "123".to_string(), name: "test".to_string() };
        DomainEvent::new(change_log, state)
    }

    #[test]
    fn test_create_producer_with_valid_brokers() {
        let producer = KafkaProducer::new("localhost:9092", "test-topic");
        assert_eq!(producer.topic, "test-topic");
    }

    #[test]
    fn test_create_producer_default_local() {
        let producer = KafkaProducer::default_local("test-topic");
        assert_eq!(producer.topic, "test-topic");
    }

    #[test]
    fn test_event_publisher_trait_implementation() {
        let producer = KafkaProducer::new("localhost:9092", "test-topic");
        let _ = producer as &dyn EventPublisher2;
    }
}
