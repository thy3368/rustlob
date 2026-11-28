use crate::{Entity, ReplayableDomainEvent};

/// 新建实体的 after truth。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedEntityEvent<E> {
    after: E,
}

impl<E> CreatedEntityEvent<E> {
    pub fn new(after: E) -> Self {
        Self { after }
    }
    pub fn after(&self) -> &E {
        &self.after
    }
}

/// 一个实体在本次 use case 中的最终 after truth。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedEntityEvents<E> {
    after: E,
}

impl<E> UpdatedEntityEvents<E> {
    pub fn new(after: E) -> Self {
        Self { after }
    }
    pub fn after(&self) -> &E {
        &self.after
    }
}

/// `StateChanged` 按确定业务顺序暴露本次 use case 的全部事件。
pub trait ReplayableEntityChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent>;
}

impl<E: Entity> ReplayableEntityChanges for CreatedEntityEvent<E> {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        self.after.replay_events().iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEntity {
        id: u64,
        replay_events: Vec<ReplayableDomainEvent>,
    }

    impl crate::Shard for TestEntity {
        fn partition(&self) -> u8 {
            0
        }
    }

    impl Entity for TestEntity {
        fn append_event(&mut self, event: ReplayableDomainEvent) {
            self.replay_events.push(event);
        }
        fn replay_events(&self) -> &[ReplayableDomainEvent] {
            &self.replay_events
        }
        fn entity_id(&self) -> u64 {
            self.id
        }
        fn owner_id(&self) -> String {
            String::new()
        }
        fn op_id(&self) -> String {
            String::new()
        }
        fn lsn(&self) -> u64 {
            10
        }
        fn command_index(&self) -> u32 {
            1
        }
        fn created_at(&self) -> u64 {
            0
        }
        fn updated_at(&self) -> u64 {
            0
        }
        fn is_derived() -> bool {
            false
        }
        fn entity_type() -> u8 {
            2
        }
    }

    fn entity() -> TestEntity {
        TestEntity {
            id: 7,
            replay_events: Vec::new(),
        }
    }

    #[test]
    fn created_entity_exposes_events_appended_to_after_truth() {
        let mut after = entity();
        after.append_event(ReplayableDomainEvent::new_at(
            0, 0, 20, 0, 0, 10, 1, 7, 2, 1,
        ));
        let changes = CreatedEntityEvent::new(after.clone());

        assert_eq!(changes.after(), &after);
        assert_eq!(
            changes.replayable_events(),
            after.replay_events().iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn updated_entity_preserves_after_truth() {
        let after = entity();
        let changes = UpdatedEntityEvents::new(after.clone());
        assert_eq!(changes.after(), &after);
    }
}
