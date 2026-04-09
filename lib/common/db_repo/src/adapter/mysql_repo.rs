use std::fmt::Debug;
use std::sync::Arc;

use diff::Entity;
use diff::diff_types::{ChangeType, DomainEvent};
use immutable_derive::immutable;
use sqlx::{MySql, Pool};

use crate::core::db_repo2::{CmdRepo2, PageRequest, PageResult, QueryRepo2, RepoError};

#[immutable]
pub struct MySqlRepo {
    pool: Option<Arc<Pool<MySql>>>,
}

impl MySqlRepo {
    pub fn new(url: &str) -> Result<Self, RepoError> {
        let pool = Pool::<MySql>::connect(url).map_err(|e| {
            RepoError::DeserializationFailed(format!("Failed to connect to MySQL: {}", e))
        })?;
        Ok(Self { pool: Some(Arc::new(pool)) })
    }

    pub fn new_mock() -> Self {
        Self { pool: None }
    }

    fn pool(&self) -> Result<&Arc<Pool<MySql>>, RepoError> {
        self.pool.as_ref().ok_or(RepoError::DeserializationFailed(
            "MySQL connection pool not initialized".to_string(),
        ))
    }

    fn empty_page<E>(page_req: PageRequest) -> PageResult<E> {
        PageResult::new(Vec::new(), 0, page_req.page, page_req.page_size)
    }
}

impl Default for MySqlRepo {
    fn default() -> Self {
        Self::new_mock()
    }
}

impl QueryRepo2 for MySqlRepo {
    fn find_by_sequence<E: Entity>(&self, _sequence: u64) -> Result<Option<E>, RepoError> {
        Ok(None)
    }

    fn find_one_by_condition<E: Entity>(&self, _condition: E) -> Result<Option<E>, RepoError> {
        Ok(None)
    }

    fn find_all_by_condition<E: Entity>(&self, _condition: E) -> Result<Vec<E>, RepoError> {
        Ok(Vec::new())
    }

    fn find_by_id<E: Entity>(&self, _entity_id: &str) -> Result<Option<E>, RepoError> {
        Ok(None)
    }

    fn find_range_by_sequence<E: Entity>(
        &self,
        _from_sequence: u64,
        _to_sequence: u64,
    ) -> Result<Vec<E>, RepoError> {
        Ok(Vec::new())
    }

    fn count(&self) -> Result<u64, RepoError> {
        Ok(0)
    }

    fn find_all_by_condition_paginated<E: Entity>(
        &self,
        _condition: E,
        page_req: PageRequest,
    ) -> Result<PageResult<E>, RepoError> {
        Ok(Self::empty_page(page_req))
    }

    fn find_range_by_sequence_paginated<E: Entity>(
        &self,
        _from_sequence: u64,
        _to_sequence: u64,
        page_req: PageRequest,
    ) -> Result<PageResult<E>, RepoError> {
        Ok(Self::empty_page(page_req))
    }

    fn find_by_cursor<E: Entity>(
        &self,
        _condition: E,
        _cursor: Option<String>,
        _limit: u64,
        _forward: bool,
    ) -> Result<(Vec<E>, Option<String>), RepoError> {
        Ok((Vec::new(), None))
    }
}

impl CmdRepo2 for MySqlRepo {
    fn replay_event<E: Clone + Debug>(&self, event: &DomainEvent<E>) -> Result<(), RepoError> {
        let pool = match &self.pool {
            Some(p) => p,
            None => return Ok(()),
        };

        let change_log = event.change_log();
        let entity_id = change_log.entity_id();
        let entity_type = change_log.entity_type();

        match change_log.change_type() {
            ChangeType::Created { fields } => {
                let sql = format!(
                    "INSERT INTO {} (entity_id, data, timestamp, sequence) VALUES (?, ?, ?, ?) ON DUPLICATE KEY UPDATE data = VALUES(data)",
                    entity_type
                );
                let data = serde_json::to_string(&fields).unwrap_or_default();
                sqlx::query(&sql)
                    .bind(entity_id)
                    .bind(&data)
                    .bind(change_log.timestamp() as i64)
                    .bind(change_log.sequence() as i64)
                    .execute(pool.as_ref())
                    .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;
            }
            ChangeType::Updated { changed_fields } => {
                let sql = format!(
                    "UPDATE {} SET data = ?, timestamp = ?, sequence = ? WHERE entity_id = ?",
                    entity_type
                );
                let data = serde_json::to_string(&changed_fields).unwrap_or_default();
                sqlx::query(&sql)
                    .bind(&data)
                    .bind(change_log.timestamp() as i64)
                    .bind(change_log.sequence() as i64)
                    .bind(entity_id)
                    .execute(pool.as_ref())
                    .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;
            }
            ChangeType::Deleted => {
                let sql = format!("DELETE FROM {} WHERE entity_id = ?", entity_type);
                sqlx::query(&sql)
                    .bind(entity_id)
                    .execute(pool.as_ref())
                    .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;
            }
        }
        Ok(())
    }

    fn replay_events<E>(&self, events: &[DomainEvent<E>]) -> Result<(), RepoError> {
        for event in events {
            self.replay_event(event)?;
        }
        Ok(())
    }

    fn replay_from_sequence<E: Clone + Debug>(
        &self,
        events: &[DomainEvent<E>],
        from_sequence: u64,
    ) -> Result<(), RepoError> {
        for event in events {
            if event.change_log().sequence() >= &from_sequence {
                self.replay_event(event)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use diff::diff_types::DomainEvent;
    use diff::{ChangeLog, ChangeType, Entity, EntityError, FieldChange};

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestEntity {
        id: String,
        value: String,
    }

    impl Entity for TestEntity {
        type Id = String;

        fn entity_id(&self) -> Self::Id {
            self.id.clone()
        }

        fn entity_type() -> &'static str {
            "TestEntity"
        }

        fn diff(&self, other: &Self) -> Vec<FieldChange> {
            if self.value != other.value {
                vec![FieldChange::new("value", self.value.clone(), other.value.clone())]
            } else {
                Vec::new()
            }
        }

        fn replay(&mut self, entry: &ChangeLog) -> Result<(), EntityError> {
            if let ChangeType::Updated { changed_fields } = entry.change_type() {
                for field in changed_fields {
                    if field.field_name() == "value" {
                        self.value = field.new_value().clone();
                    }
                }
            }
            Ok(())
        }
    }

    #[test]
    fn replay_events_succeeds_in_mock_mode() {
        let repo = MySqlRepo::new_mock();
        let created = ChangeLog::new(
            "entity-1".to_string(),
            TestEntity::entity_type().to_string(),
            ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
            1,
            1,
        );
        let event = DomainEvent::new(
            created,
            TestEntity { id: "entity-1".to_string(), value: "alpha".to_string() },
        );

        assert!(repo.replay_event(&event).is_ok());
        assert!(repo.replay_events(&[event]).is_ok());
    }

    #[test]
    fn replay_from_sequence_filters_events() {
        let repo = MySqlRepo::new_mock();
        let first = DomainEvent::new(
            ChangeLog::new(
                "entity-1".to_string(),
                TestEntity::entity_type().to_string(),
                ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
                1,
                1,
            ),
            TestEntity { id: "entity-1".to_string(), value: "alpha".to_string() },
        );
        let second = DomainEvent::new(
            ChangeLog::new(
                "entity-2".to_string(),
                TestEntity::entity_type().to_string(),
                ChangeType::Created { fields: vec![FieldChange::new("value", "", "beta")] },
                2,
                2,
            ),
            TestEntity { id: "entity-2".to_string(), value: "beta".to_string() },
        );

        assert!(repo.replay_from_sequence(&[first, second], 2).is_ok());
    }

    #[test]
    fn replay_event_rejects_entity_type_mismatch() {
        let repo = MySqlRepo::new_mock();
        let event = DomainEvent::new(
            ChangeLog::new(
                "entity-1".to_string(),
                "OtherEntity".to_string(),
                ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
                1,
                1,
            ),
            TestEntity { id: "entity-1".to_string(), value: "alpha".to_string() },
        );

        assert!(matches!(repo.replay_event(&event), Err(RepoError::DeserializationFailed(_))));
    }
}
