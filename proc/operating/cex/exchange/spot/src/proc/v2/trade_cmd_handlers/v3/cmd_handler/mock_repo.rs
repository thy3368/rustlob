use db_repo::core::db_repo2::{CmdRepo2, PageRequest, PageResult, QueryRepo2, RepoError};
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use diff::Entity;

#[derive(Clone)]
pub struct MockMySqlRepo;

impl QueryRepo2 for MockMySqlRepo {
    fn find_by_sequence<E: Entity>(&self, _sequence: u64) -> Result<Option<E>, RepoError> {
        todo!()
    }

    fn find_one_by_condition<E: Entity>(&self, _condition: E) -> Result<Option<E>, RepoError> {
        todo!()
    }

    fn find_all_by_condition<E: Entity>(&self, _condition: E) -> Result<Vec<E>, RepoError> {
        todo!()
    }

    fn find_by_id<E: Entity>(&self, _entity_id: &str) -> Result<Option<E>, RepoError> {
        todo!()
    }

    fn find_range_by_sequence<E: Entity>(
        &self,
        _from_sequence: u64,
        _to_sequence: u64,
    ) -> Result<Vec<E>, RepoError> {
        todo!()
    }

    fn count(&self) -> Result<u64, RepoError> {
        todo!()
    }

    fn find_all_by_condition_paginated<E: Entity>(
        &self,
        _condition: E,
        _page_req: PageRequest,
    ) -> Result<PageResult<E>, RepoError> {
        todo!()
    }

    fn find_range_by_sequence_paginated<E: Entity>(
        &self,
        _from_sequence: u64,
        _to_sequence: u64,
        _page_req: PageRequest,
    ) -> Result<PageResult<E>, RepoError> {
        todo!()
    }

    fn find_by_cursor<E: Entity>(
        &self,
        _condition: E,
        _cursor: Option<String>,
        _limit: u64,
        _forward: bool,
    ) -> Result<(Vec<E>, Option<String>), RepoError> {
        todo!()
    }
}

impl CmdRepo2 for MockMySqlRepo {
    fn replay_event<E>(&self, _event: &DomainEvent<E>) -> Result<(), RepoError> {
        Ok(())
    }

    fn replay_events<E>(&self, _events: &[DomainEvent<E>]) -> Result<(), RepoError> {
        Ok(())
    }

    fn replay_from_sequence<E: Clone + std::fmt::Debug>(
        &self,
        _events: &[DomainEvent<E>],
        _from_sequence: u64,
    ) -> Result<(), RepoError> {
        Ok(())
    }
}


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