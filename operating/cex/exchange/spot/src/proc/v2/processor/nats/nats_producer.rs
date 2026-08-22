use db_repo::core::event_publish::{EventPublisher2, PublishError};
use diff::diff_types::DomainEvent;

// nats 事件发布器
pub struct NatsProducer {}

impl EventPublisher2 for NatsProducer {
    fn publish<E>(&self, _event: &DomainEvent<E>) -> Result<(), PublishError> {
        todo!()
    }

    fn publish_batch<E>(&self, _events: &[DomainEvent<E>]) -> Result<(), PublishError> {
        todo!()
    }
}
