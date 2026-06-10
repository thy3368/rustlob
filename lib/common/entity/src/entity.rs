use std::fmt::Debug;

use crate::entity_field_change::{current_timestamp, next_sequence};
use crate::{EntityError, EntityFieldChange, EntityReplayableEvent, ReplayFieldChange};

/// Enhanced entity contract for generating compact replayable entity events.
pub trait Entity: Clone + Debug + Send + Sync + 'static {
    type Id: Debug + Clone + PartialEq + ToString;

    fn entity_id(&self) -> Self::Id;

    fn entity_type() -> u8
    where
        Self: Sized;

    fn entity_version(&self) -> u64;

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange>;

    #[inline]
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    #[inline]
    fn replay_field_type(_field_name: &str) -> u8 {
        0
    }

    #[inline]
    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        let entity_id = self.entity_id().to_string();
        entity_id.parse::<i64>().map_err(|error| EntityError::EntityIdParseError {
            entity_id,
            reason: error.to_string(),
        })
    }

    #[inline]
    fn track_create_event(&self) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
    {
        let mut event = EntityReplayableEvent::new_created(
            current_timestamp()?,
            next_sequence(),
            self.replay_entity_id()?,
            Self::entity_type(),
        );

        for change in self.created_field_changes() {
            let field_type = Self::replay_field_type(change.field_name.as_ref());
            event
                .add_field_change(ReplayFieldChange::from_entity_field_change(&change, field_type));
        }

        Ok(event)
    }

    #[inline]
    fn track_update_event<F>(&mut self, updater: F) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
        F: FnOnce(&mut Self),
    {
        let old_state = self.clone();
        updater(self);
        self.track_update_event_from(&old_state)
    }

    #[inline]
    fn track_update_event_from(
        &self,
        old_state: &Self,
    ) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
    {
        ensure_same_entity(old_state, self)?;

        let old_version = old_state.entity_version();
        let expected_next = increment_version(old_version)?;
        let new_version = self.entity_version();
        if new_version != expected_next {
            return Err(EntityError::EntityVersionMismatch {
                expected_next,
                actual_next: new_version,
            });
        }

        let changes = old_state.diff(self);
        if changes.is_empty() {
            return Err(EntityError::NoChangesDetected);
        }

        let mut event = EntityReplayableEvent::new_updated(
            current_timestamp()?,
            next_sequence(),
            old_version,
            new_version,
            self.replay_entity_id()?,
            Self::entity_type(),
        );

        for change in changes {
            let field_type = Self::replay_field_type(change.field_name.as_ref());
            event
                .add_field_change(ReplayFieldChange::from_entity_field_change(&change, field_type));
        }

        Ok(event)
    }

    #[inline]
    fn track_delete_event(&self) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
    {
        let old_version = self.entity_version();
        Ok(EntityReplayableEvent::new_deleted(
            current_timestamp()?,
            next_sequence(),
            old_version,
            increment_version(old_version)?,
            self.replay_entity_id()?,
            Self::entity_type(),
        ))
    }
}

#[inline]
fn increment_version(version: u64) -> Result<u64, EntityError> {
    version.checked_add(1).ok_or(EntityError::VersionOverflow { version })
}

#[inline]
fn ensure_same_entity<T>(old_state: &T, new_state: &T) -> Result<(), EntityError>
where
    T: Entity,
{
    let expected = old_state.entity_id().to_string();
    let actual = new_state.entity_id().to_string();
    if expected == actual {
        Ok(())
    } else {
        Err(EntityError::EntityIdMismatch { expected, actual })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_field_change::ReplayFieldChange;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEntity {
        id: i64,
        value: String,
        version: u64,
    }

    impl Entity for TestEntity {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            7
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
            let mut changes = Vec::new();
            if self.value != other.value {
                changes.push(EntityFieldChange::new("value", &self.value, &other.value));
            }
            changes
        }

        fn replay_field_type(field_name: &str) -> u8 {
            match field_name {
                "value" => 1,
                _ => 0,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct StringIdEntity {
        id: String,
        version: u64,
    }

    impl Entity for StringIdEntity {
        type Id = String;

        fn entity_id(&self) -> Self::Id {
            self.id.clone()
        }

        fn entity_type() -> u8 {
            9
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
            Vec::new()
        }
    }

    #[test]
    fn create_event_uses_initial_version_transition() {
        let entity = TestEntity { id: 42, value: "new".to_string(), version: 1 };

        let event = entity.track_create_event().unwrap();

        assert!(event.is_created());
        assert_eq!(event.old_version, 0);
        assert_eq!(event.new_version, 1);
        assert_eq!(event.entity_id, 42);
        assert_eq!(event.entity_type, 7);
    }

    #[test]
    fn update_event_contains_diff_field_changes() {
        let old = TestEntity { id: 1, value: "old".to_string(), version: 3 };
        let new = TestEntity { id: 1, value: "new".to_string(), version: 4 };

        let event = new.track_update_event_from(&old).unwrap();

        assert!(event.is_updated());
        assert_eq!(event.old_version, 3);
        assert_eq!(event.new_version, 4);
        assert_eq!(event.field_change_count(), 1);

        let change = &event.field_changes[0];
        assert_eq!(change.field_name_as_str().unwrap(), "value");
        assert_eq!(change.old_value_bytes(), b"old");
        assert_eq!(change.new_value_bytes(), b"new");
        assert_eq!(change.field_type, 1);
    }

    #[test]
    fn update_event_rejects_no_changes() {
        let old = TestEntity { id: 1, value: "same".to_string(), version: 3 };
        let new = TestEntity { id: 1, value: "same".to_string(), version: 4 };

        let result = new.track_update_event_from(&old);

        assert_eq!(result.unwrap_err(), EntityError::NoChangesDetected);
    }

    #[test]
    fn update_event_rejects_non_incremented_version() {
        let old = TestEntity { id: 1, value: "old".to_string(), version: 3 };
        let new = TestEntity { id: 1, value: "new".to_string(), version: 5 };

        let result = new.track_update_event_from(&old);

        assert_eq!(
            result.unwrap_err(),
            EntityError::EntityVersionMismatch { expected_next: 4, actual_next: 5 }
        );
    }

    #[test]
    fn update_event_rejects_entity_id_mismatch() {
        let old = TestEntity { id: 1, value: "old".to_string(), version: 3 };
        let new = TestEntity { id: 2, value: "new".to_string(), version: 4 };

        let result = new.track_update_event_from(&old);

        assert_eq!(
            result.unwrap_err(),
            EntityError::EntityIdMismatch { expected: "1".to_string(), actual: "2".to_string() }
        );
    }

    #[test]
    fn update_event_from_closure_tracks_after_mutation() {
        let mut entity = TestEntity { id: 1, value: "old".to_string(), version: 3 };

        let event = entity
            .track_update_event(|entity| {
                entity.value = "new".to_string();
                entity.version += 1;
            })
            .unwrap();

        assert_eq!(entity.value, "new");
        assert_eq!(event.old_version, 3);
        assert_eq!(event.new_version, 4);
        assert_eq!(event.field_change_count(), 1);
    }

    #[test]
    fn delete_event_uses_current_to_next_version() {
        let entity = TestEntity { id: 42, value: "old".to_string(), version: 8 };

        let event = entity.track_delete_event().unwrap();

        assert!(event.is_deleted());
        assert_eq!(event.old_version, 8);
        assert_eq!(event.new_version, 9);
        assert_eq!(event.entity_id, 42);
    }

    #[test]
    fn non_numeric_entity_id_returns_parse_error() {
        let entity = StringIdEntity { id: "account:BTC".to_string(), version: 1 };

        let result = entity.track_create_event();

        assert!(matches!(result, Err(EntityError::EntityIdParseError { .. })));
    }

    #[test]
    fn replay_field_change_truncates_fixed_width_data() {
        let long_field_name = "abcdefghijklmnopqrstuvwxyz0123456789";
        let long_old = vec![b'o'; 80];
        let long_new = vec![b'n'; 96];

        let change = ReplayFieldChange::new(
            ReplayFieldChange::field_name_from_str(long_field_name),
            &long_old,
            &long_new,
            2,
        );

        assert_eq!(change.field_name_as_str().unwrap().len(), 32);
        assert_eq!(change.old_value_bytes().len(), 64);
        assert_eq!(change.new_value_bytes().len(), 64);
        assert_eq!(change.field_type, 2);
    }
}
