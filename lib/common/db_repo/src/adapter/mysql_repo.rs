use std::fmt::Debug;
use diff::diff_types::DomainEvent;
use crate::core::db_repo2::{CmdRepo2, RepoError};

pub struct MySqlRepo{}

impl CmdRepo2 for MySqlRepo{
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