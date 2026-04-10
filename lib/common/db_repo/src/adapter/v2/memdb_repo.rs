use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::sync::RwLock;

use diff::diff_types::{ChangeType, DomainEvent};
use diff::Entity;
use immutable_derive::immutable;

use crate::core::db_repo2::{CmdRepo2, PageRequest, PageResult, QueryRepo2, RepoError};

struct StoredEntity {
    entity_id: String,
    sequence: u64,
    entity: Box<dyn Any + Send + Sync>,
}

impl StoredEntity {
    fn new<E: Entity>(entity_id: String, sequence: u64, entity: E) -> Self {
        Self { entity_id, sequence, entity: Box::new(entity) }
    }

    fn clone_as<E: Entity>(&self) -> Option<E> {
        self.entity.downcast_ref::<E>().cloned()
    }
}

#[derive(Default)]
struct TypeStore {
    by_id: HashMap<String, StoredEntity>,
    by_sequence: BTreeMap<u64, String>,
}

impl TypeStore {
    fn insert<E: Entity>(&mut self, entity_id: String, sequence: u64, entity: E) {
        if let Some(previous) = self.by_id.insert(
            entity_id.clone(),
            StoredEntity::new(entity_id.clone(), sequence, entity),
        ) {
            self.by_sequence.remove(&previous.sequence);
        }
        self.by_sequence.insert(sequence, entity_id);
    }

    fn remove(&mut self, entity_id: &str) -> Option<StoredEntity> {
        let removed = self.by_id.remove(entity_id)?;
        self.by_sequence.remove(&removed.sequence);
        Some(removed)
    }

    fn get<E: Entity>(&self, entity_id: &str) -> Option<E> {
        self.by_id.get(entity_id).and_then(StoredEntity::clone_as::<E>)
    }

    fn get_by_sequence<E: Entity>(&self, sequence: u64) -> Option<E> {
        let entity_id = self.by_sequence.get(&sequence)?;
        self.get::<E>(entity_id)
    }

    fn range<E: Entity>(&self, from_sequence: u64, to_sequence: u64) -> Vec<E> {
        self.by_sequence
            .range(from_sequence..=to_sequence)
            .filter_map(|(_, entity_id)| self.get::<E>(entity_id))
            .collect()
    }
}

///规则：内存版的repo,主要用来给bdd test
#[immutable]
pub struct MemdbRepo {
    stores: RwLock<HashMap<&'static str, TypeStore>>,
}

impl MemdbRepo {
    fn empty_page<E>(page_req: PageRequest) -> PageResult<E> {
        PageResult::new(Vec::new(), 0, page_req.page, page_req.page_size)
    }

    fn paginate<E>(items: Vec<E>, page_req: PageRequest) -> PageResult<E> {
        let total_elements = items.len() as u64;
        let start = page_req.offset() as usize;
        if start >= items.len() {
            return Self::empty_page(page_req);
        }
        let end = (start + page_req.limit() as usize).min(items.len());
        let content = items.into_iter().skip(start).take(end - start).collect();
        PageResult::new(content, total_elements, page_req.page, page_req.page_size)
    }

    fn ensure_entity_type<E: Entity>(event: &DomainEvent<E>) -> Result<(), RepoError> {
        let change_log = event.change_log();
        if change_log.entity_type() != E::entity_type() {
            return Err(RepoError::DeserializationFailed(format!(
                "Entity type mismatch: expected {}, got {}",
                E::entity_type(),
                change_log.entity_type()
            )));
        }
        Ok(())
    }

    fn store_read<E: Entity, T>(&self, f: impl FnOnce(&TypeStore) -> T) -> T {
        let stores = self.stores.read().expect("memdb repo read lock poisoned");
        if let Some(store) = stores.get(E::entity_type()) {
            f(store)
        } else {
            let empty = TypeStore::default();
            f(&empty)
        }
    }

    fn store_write<E: Entity, T>(&self, f: impl FnOnce(&mut TypeStore) -> T) -> T {
        let mut stores = self.stores.write().expect("memdb repo write lock poisoned");
        let store = stores.entry(E::entity_type()).or_default();
        f(store)
    }

    fn matches_condition<E: Entity>(entity: &E, condition: &E) -> bool {
        entity.entity_id().to_string() == condition.entity_id().to_string()
    }
}

impl Default for MemdbRepo {
    fn default() -> Self {
        Self { stores: RwLock::new(HashMap::new()) }
    }
}

impl CmdRepo2 for MemdbRepo {
    fn replay_event<E: Entity + Clone + Debug>(
        &self,
        event: &DomainEvent<E>,
    ) -> Result<(), RepoError> {
        Self::ensure_entity_type(event)?;

        let change_log = event.change_log();
        let entity_id = change_log.entity_id().clone();
        let sequence = *change_log.sequence();
        let entity = event.object().clone();

        self.store_write::<E, _>(|store| match change_log.change_type() {
            ChangeType::Created { .. } => {
                store.insert(entity_id, sequence, entity);
                Ok(())
            }
            ChangeType::Updated { .. } => {
                if !store.by_id.contains_key(&entity_id) {
                    return Err(RepoError::OrderNotFound);
                }
                store.insert(entity_id, sequence, entity);
                Ok(())
            }
            ChangeType::Deleted => {
                if store.remove(&entity_id).is_none() {
                    return Err(RepoError::OrderNotFound);
                }
                Ok(())
            }
        })
    }

    fn replay_events<E: Entity + Clone + Debug>(
        &self,
        events: &[DomainEvent<E>],
    ) -> Result<(), RepoError> {
        for event in events {
            self.replay_event(event)?;
        }
        Ok(())
    }

    fn replay_from_sequence<E: Entity + Clone + Debug>(
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

impl QueryRepo2 for MemdbRepo {
    fn find_by_sequence<E: Entity>(&self, sequence: u64) -> Result<Option<E>, RepoError> {
        Ok(self.store_read::<E, _>(|store| store.get_by_sequence(sequence)))
    }

    fn find_one_by_condition<E: Entity>(&self, condition: E) -> Result<Option<E>, RepoError> {
        let entity_id = condition.entity_id().to_string();
        if !entity_id.is_empty() {
            return self.find_by_id::<E>(&entity_id);
        }

        Ok(self.store_read::<E, _>(|store| {
            store
                .by_id
                .values()
                .filter_map(StoredEntity::clone_as::<E>)
                .find(|entity| Self::matches_condition(entity, &condition))
        }))
    }

    fn find_all_by_condition<E: Entity>(&self, condition: E) -> Result<Vec<E>, RepoError> {
        let entity_id = condition.entity_id().to_string();
        if !entity_id.is_empty() {
            return Ok(self.find_by_id::<E>(&entity_id)?.into_iter().collect());
        }

        Ok(self.store_read::<E, _>(|store| {
            store
                .by_sequence
                .values()
                .filter_map(|entity_id| store.get::<E>(entity_id))
                .filter(|entity| Self::matches_condition(entity, &condition))
                .collect()
        }))
    }

    fn find_by_id<E: Entity>(&self, entity_id: &str) -> Result<Option<E>, RepoError> {
        Ok(self.store_read::<E, _>(|store| store.get(entity_id)))
    }

    fn find_range_by_sequence<E: Entity>(
        &self,
        from_sequence: u64,
        to_sequence: u64,
    ) -> Result<Vec<E>, RepoError> {
        if from_sequence > to_sequence {
            return Ok(Vec::new());
        }
        Ok(self.store_read::<E, _>(|store| store.range(from_sequence, to_sequence)))
    }

    fn count(&self) -> Result<u64, RepoError> {
        let stores = self.stores.read().expect("memdb repo read lock poisoned");
        Ok(stores.values().map(|store| store.by_id.len() as u64).sum())
    }

    fn find_all_by_condition_paginated<E: Entity>(
        &self,
        condition: E,
        page_req: PageRequest,
    ) -> Result<PageResult<E>, RepoError> {
        let items = self.find_all_by_condition(condition)?;
        Ok(Self::paginate(items, page_req))
    }

    fn find_range_by_sequence_paginated<E: Entity>(
        &self,
        from_sequence: u64,
        to_sequence: u64,
        page_req: PageRequest,
    ) -> Result<PageResult<E>, RepoError> {
        let items = self.find_range_by_sequence(from_sequence, to_sequence)?;
        Ok(Self::paginate(items, page_req))
    }

    fn find_by_cursor<E: Entity>(
        &self,
        condition: E,
        cursor: Option<String>,
        limit: u64,
        forward: bool,
    ) -> Result<(Vec<E>, Option<String>), RepoError> {
        let cursor_sequence = cursor
            .map(|value| {
                value.parse::<u64>().map_err(|e| {
                    RepoError::DeserializationFailed(format!("Invalid cursor '{}': {}", value, e))
                })
            })
            .transpose()?;

        let entity_id = condition.entity_id().to_string();
        let items = self.store_read::<E, _>(|store| {
            let mut sequences: Vec<u64> = if forward {
                store
                    .by_sequence
                    .range(cursor_sequence.map_or(0.., |cursor| cursor.saturating_add(1)..))
                    .map(|(sequence, _)| *sequence)
                    .collect()
            } else {
                store
                    .by_sequence
                    .range(..cursor_sequence.unwrap_or(u64::MAX))
                    .map(|(sequence, _)| *sequence)
                    .collect()
            };

            if !forward {
                sequences.reverse();
            }

            let mut items: Vec<(u64, E)> = sequences
                .into_iter()
                .filter_map(|sequence| store.get_by_sequence::<E>(sequence).map(|entity| (sequence, entity)))
                .filter(|(_, entity)| {
                    entity_id.is_empty() || entity.entity_id().to_string() == entity_id
                })
                .take(limit as usize)
                .collect();

            if !forward {
                items.reverse();
            }
            items
        });

        let next_cursor = items.last().map(|(sequence, _)| sequence.to_string());
        Ok((items.into_iter().map(|(_, entity)| entity).collect(), next_cursor))
    }
}

#[cfg(test)]
mod tests {
    use diff::diff_types::{ChangeLog, FieldChange};
    use diff::{EntityError, FromCreatedEvent};

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
            match entry.change_type() {
                ChangeType::Created { fields } => {
                    for field in fields {
                        if field.field_name == "value" {
                            self.value = field.new_value.clone();
                        }
                    }
                }
                ChangeType::Updated { changed_fields } => {
                    for field in changed_fields {
                        if field.field_name == "value" {
                            self.value = field.new_value.clone();
                        }
                    }
                }
                ChangeType::Deleted => {}
            }
            Ok(())
        }
    }

    impl FromCreatedEvent for TestEntity {
        fn from_created_event(entry: &ChangeLog) -> Result<Self, EntityError> {
            let mut id = entry.entity_id().clone();
            let mut value = String::new();
            if let ChangeType::Created { fields } = entry.change_type() {
                for field in fields {
                    if field.field_name == "id" {
                        id = field.new_value.clone();
                    } else if field.field_name == "value" {
                        value = field.new_value.clone();
                    }
                }
            }
            Ok(Self { id, value })
        }
    }

    fn test_event(id: &str, value: &str, sequence: u64, change_type: ChangeType) -> DomainEvent<TestEntity> {
        DomainEvent::new(
            ChangeLog::new(
                id.to_string(),
                TestEntity::entity_type().to_string(),
                change_type,
                sequence,
                sequence,
            ),
            TestEntity { id: id.to_string(), value: value.to_string() },
        )
    }

    #[test]
    fn replay_create_and_basic_queries_work() {
        let repo = MemdbRepo::default();
        let created = test_event(
            "entity-1",
            "alpha",
            1,
            ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
        );

        repo.replay_event(&created).unwrap();

        assert_eq!(repo.count().unwrap(), 1);
        assert_eq!(repo.find_by_id::<TestEntity>("entity-1").unwrap(), Some(created.object().clone()));
        assert_eq!(repo.find_by_sequence::<TestEntity>(1).unwrap(), Some(created.object().clone()));
    }

    #[test]
    fn replay_update_replaces_latest_state() {
        let repo = MemdbRepo::default();
        repo.replay_event(&test_event(
            "entity-1",
            "alpha",
            1,
            ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
        ))
        .unwrap();

        repo.replay_event(&test_event(
            "entity-1",
            "beta",
            2,
            ChangeType::Updated { changed_fields: vec![FieldChange::new("value", "alpha", "beta")] },
        ))
        .unwrap();

        assert_eq!(repo.find_by_id::<TestEntity>("entity-1").unwrap().unwrap().value, "beta");
        assert!(repo.find_by_sequence::<TestEntity>(1).unwrap().is_none());
        assert_eq!(repo.find_by_sequence::<TestEntity>(2).unwrap().unwrap().value, "beta");
    }

    #[test]
    fn replay_delete_cleans_indexes() {
        let repo = MemdbRepo::default();
        repo.replay_event(&test_event(
            "entity-1",
            "alpha",
            1,
            ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
        ))
        .unwrap();

        repo.replay_event(&test_event("entity-1", "alpha", 2, ChangeType::Deleted)).unwrap();

        assert_eq!(repo.count().unwrap(), 0);
        assert!(repo.find_by_id::<TestEntity>("entity-1").unwrap().is_none());
        assert!(repo.find_by_sequence::<TestEntity>(1).unwrap().is_none());
    }

    #[test]
    fn replay_from_sequence_filters_events() {
        let repo = MemdbRepo::default();
        let events = vec![
            test_event(
                "entity-1",
                "alpha",
                1,
                ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
            ),
            test_event(
                "entity-2",
                "beta",
                2,
                ChangeType::Created { fields: vec![FieldChange::new("value", "", "beta")] },
            ),
        ];

        repo.replay_from_sequence(&events, 2).unwrap();

        assert!(repo.find_by_id::<TestEntity>("entity-1").unwrap().is_none());
        assert_eq!(repo.find_by_id::<TestEntity>("entity-2").unwrap().unwrap().value, "beta");
    }

    #[test]
    fn range_and_pagination_queries_are_stable() {
        let repo = MemdbRepo::default();
        for sequence in 1..=3 {
            repo.replay_event(&test_event(
                &format!("entity-{}", sequence),
                &format!("v{}", sequence),
                sequence,
                ChangeType::Created { fields: vec![FieldChange::new("value", "", format!("v{}", sequence))] },
            ))
            .unwrap();
        }

        let range = repo.find_range_by_sequence::<TestEntity>(1, 3).unwrap();
        assert_eq!(range.iter().map(|e| e.value.as_str()).collect::<Vec<_>>(), vec!["v1", "v2", "v3"]);

        let page = repo
            .find_range_by_sequence_paginated::<TestEntity>(1, 3, PageRequest::new(1, 2))
            .unwrap();
        assert_eq!(page.total_elements, 3);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].value, "v3");
    }

    #[test]
    fn cursor_query_supports_forward_and_backward_navigation() {
        let repo = MemdbRepo::default();
        for sequence in 1..=4 {
            repo.replay_event(&test_event(
                &format!("entity-{}", sequence),
                &format!("v{}", sequence),
                sequence,
                ChangeType::Created { fields: vec![FieldChange::new("value", "", format!("v{}", sequence))] },
            ))
            .unwrap();
        }

        let condition = TestEntity { id: String::new(), value: String::new() };
        let (forward_items, next_cursor) = repo.find_by_cursor(condition.clone(), None, 2, true).unwrap();
        assert_eq!(forward_items.iter().map(|e| e.value.as_str()).collect::<Vec<_>>(), vec!["v1", "v2"]);
        assert_eq!(next_cursor, Some("2".to_string()));

        let (backward_items, prev_cursor) = repo.find_by_cursor(condition, Some("4".to_string()), 2, false).unwrap();
        assert_eq!(backward_items.iter().map(|e| e.value.as_str()).collect::<Vec<_>>(), vec!["v2", "v3"]);
        assert_eq!(prev_cursor, Some("3".to_string()));
    }

    #[test]
    fn condition_queries_use_entity_id_matching() {
        let repo = MemdbRepo::default();
        repo.replay_event(&test_event(
            "entity-1",
            "alpha",
            1,
            ChangeType::Created { fields: vec![FieldChange::new("value", "", "alpha")] },
        ))
        .unwrap();

        let condition = TestEntity { id: "entity-1".to_string(), value: "ignored".to_string() };
        let one = repo.find_one_by_condition(condition.clone()).unwrap();
        let all = repo.find_all_by_condition(condition.clone()).unwrap();
        let paged = repo
            .find_all_by_condition_paginated(condition, PageRequest::new(0, 10))
            .unwrap();

        assert_eq!(one.unwrap().value, "alpha");
        assert_eq!(all.len(), 1);
        assert_eq!(paged.total_elements, 1);
        assert_eq!(paged.content[0].value, "alpha");
    }

    #[test]
    fn replay_event_rejects_entity_type_mismatch() {
        let repo = MemdbRepo::default();
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
