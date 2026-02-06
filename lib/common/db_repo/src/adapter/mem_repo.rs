use diff::{ChangeLogEntry, Entity};
use immutable_derive::immutable;

use crate::{CmdRepo, PageRequest, PageResult, QueryRepo, RepoError};

/// 基于内存的仓储实现，支持所有实现了 Entity trait 的类型
#[immutable]

pub struct MemRepo<E: Entity> {
    _entity: std::marker::PhantomData<E>,
}

impl<E: Entity> CmdRepo for MemRepo<E> {
    type E = E;

    fn replay_event(&self, event: &ChangeLogEntry) -> Result<(), RepoError> {
        todo!()
    }

    fn replay_events(&self, events: &[ChangeLogEntry]) -> Result<(), RepoError> {
        todo!()
    }

    fn replay_from_sequence(
        &self,
        events: &[ChangeLogEntry],
        from_sequence: u64,
    ) -> Result<(), RepoError> {
        todo!()
    }
}

impl<E: Entity> QueryRepo for MemRepo<E> {
    type E = E;

    fn find_by_sequence(&self, sequence: u64) -> Result<Option<Self::E>, RepoError> {
        todo!()
    }

    fn find_one_by_condition(&self, condition: Self::E) -> Result<Option<Self::E>, RepoError> {
        todo!()
    }

    fn find_all_by_condition(&self, condition: Self::E) -> Result<Vec<Self::E>, RepoError> {
        todo!()
    }

    fn find_by_id(&self, entity_id: &str) -> Result<Option<Self::E>, RepoError> {
        todo!()
    }

    fn find_range_by_sequence(
        &self,
        from_sequence: u64,
        to_sequence: u64,
    ) -> Result<Vec<Self::E>, RepoError> {
        todo!()
    }

    fn count(&self) -> Result<u64, RepoError> {
        todo!()
    }

    fn exists(&self, entity_id: &str) -> Result<bool, RepoError> {
        todo!()
    }

    fn find_all_by_condition_paginated(
        &self,
        condition: Self::E,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError> {
        todo!()
    }

    fn find_range_by_sequence_paginated(
        &self,
        from_sequence: u64,
        to_sequence: u64,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError> {
        todo!()
    }

    fn find_by_cursor(
        &self,
        condition: Self::E,
        cursor: Option<String>,
        limit: u64,
        forward: bool,
    ) -> Result<(Vec<Self::E>, Option<String>), RepoError> {
        todo!()
    }
}
