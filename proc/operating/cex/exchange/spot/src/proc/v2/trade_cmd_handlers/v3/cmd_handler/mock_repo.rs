use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;

#[derive(Clone, Copy, Debug, Default)]
pub struct MockEventPublisher;

impl EventPublisher2 for MockEventPublisher {
    fn publish<E>(
        &self,
        _event: &DomainEvent<E>,
    ) -> Result<(), db_repo::core::event_publish::PublishError> {
        Ok(())
    }

    fn publish_batch<E>(
        &self,
        _events: &[DomainEvent<E>],
    ) -> Result<(), db_repo::core::event_publish::PublishError> {
        Ok(())
    }
}
