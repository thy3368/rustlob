use diff::diff_types::DomainEvent;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct PublishError(pub String);

impl From<serde_json::Error> for PublishError {
    fn from(e: serde_json::Error) -> Self {
        PublishError(e.to_string())
    }
}

impl From<rdkafka::error::KafkaError> for PublishError {
    fn from(e: rdkafka::error::KafkaError) -> Self {
        PublishError(e.to_string())
    }
}

impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PublishError: {}", self.0)
    }
}

impl std::error::Error for PublishError {}

pub enum ReceiveError {}

pub trait EventPublisher2: Send + Sync {
    fn publish<E: Serialize>(&self, event: &DomainEvent<E>) -> Result<(), PublishError>;

    fn publish_batch<E: Serialize>(&self, events: &[DomainEvent<E>]) -> Result<(), PublishError>;
}

pub trait EventReceiver2: Send + Sync {
    fn receive<E: Serialize>(&self) -> Result<DomainEvent<E>, ReceiveError>;
}
