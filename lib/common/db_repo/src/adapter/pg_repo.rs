use std::fmt::Debug;
use diff::diff_types::DomainEvent;
use diff::Entity;
use crate::core::db_repo2::{CmdRepo2, PageRequest, PageResult, QueryRepo2, RepoError};

pub struct PgRepo {}

impl QueryRepo2 for PgRepo {
    fn find_by_sequence<E: Entity>(&self, sequence: u64) -> Result<Option<E>, RepoError> {
        todo!()
    }

    fn find_one_by_condition<E: Entity>(&self, condition: E) -> Result<Option<E>, RepoError> {
        todo!()
    }

    fn find_all_by_condition<E: Entity>(&self, condition: E) -> Result<Vec<E>, RepoError> {
        todo!()
    }

    fn find_by_id<E: Entity>(&self, entity_id: &str) -> Result<Option<E>, RepoError> {
        todo!()
    }

    fn find_range_by_sequence<E: Entity>(&self, from_sequence: u64, to_sequence: u64) -> Result<Vec<E>, RepoError> {
        todo!()
    }

    fn count(&self) -> Result<u64, RepoError> {
        todo!()
    }

    fn find_all_by_condition_paginated<E: Entity>(&self, condition: E, page_req: PageRequest) -> Result<PageResult<E>, RepoError> {
        todo!()
    }

    fn find_range_by_sequence_paginated<E: Entity>(&self, from_sequence: u64, to_sequence: u64, page_req: PageRequest) -> Result<PageResult<E>, RepoError> {
        todo!()
    }

    fn find_by_cursor<E: Entity>(&self, condition: E, cursor: Option<String>, limit: u64, forward: bool) -> Result<(Vec<E>, Option<String>), RepoError> {
        todo!()
    }
}

impl CmdRepo2 for PgRepo {
    fn replay_event<E>(&self, event: &DomainEvent<E>) -> Result<(), RepoError> {
        todo!()
    }

    fn replay_events<E>(&self, events: &[DomainEvent<E>]) -> Result<(), RepoError> {
        todo!()
    }

    fn replay_from_sequence<E: Clone + Debug>(&self, events: &[DomainEvent<E>], from_sequence: u64) -> Result<(), RepoError> {
        todo!()
    }
}