use crate::{Entity, ReplayableDomainEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdatedEntityEventsError {
    EmptyEvents,
    EntityIdentityMismatch {
        event_index: usize,
    },
    BrokenVersionChain {
        event_index: usize,
        expected_lsn: u64,
        expected_command_index: u32,
        actual_lsn: u64,
        actual_command_index: u32,
    },
    AfterPositionMismatch {
        expected_lsn: u64,
        expected_command_index: u32,
        actual_lsn: u64,
        actual_command_index: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatedEntityEventError {
    EntityIdentityMismatch,
    NonZeroPreviousPosition {
        actual_lsn: u64,
        actual_command_index: u32,
    },
    AfterPositionMismatch {
        expected_lsn: u64,
        expected_command_index: u32,
        actual_lsn: u64,
        actual_command_index: u32,
    },
}

/// 新建实体的 after truth 与唯一 create replay event。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedEntityEvent<E> {
    after: E,
    /// todo events可以去掉了
    event: ReplayableDomainEvent,
}

impl<E: Entity> CreatedEntityEvent<E> {
    pub fn try_new(
        after: E,
        event: ReplayableDomainEvent,
    ) -> Result<Self, CreatedEntityEventError> {
        if event.entity_id != after.entity_id() || event.entity_type != E::entity_type() {
            return Err(CreatedEntityEventError::EntityIdentityMismatch);
        }
        if (event.previous_lsn, event.previous_command_index) != (0, 0) {
            return Err(CreatedEntityEventError::NonZeroPreviousPosition {
                actual_lsn: event.previous_lsn,
                actual_command_index: event.previous_command_index,
            });
        }
        if (after.lsn(), after.command_index()) != (event.lsn, event.command_index) {
            return Err(CreatedEntityEventError::AfterPositionMismatch {
                expected_lsn: event.lsn,
                expected_command_index: event.command_index,
                actual_lsn: after.lsn(),
                actual_command_index: after.command_index(),
            });
        }
        Ok(Self { after, event })
    }

    pub fn after(&self) -> &E {
        &self.after
    }

    pub fn event(&self) -> &ReplayableDomainEvent {
        &self.event
    }
}

/// 一个实体在本次 use case 中的最终 after truth。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedEntityEvents<E> {
    after: E,
}

impl<E: Entity> UpdatedEntityEvents<E> {
    pub fn try_new(
        after: E,
        loaded_lsn: u64,
        loaded_command_index: u32,
    ) -> Result<Self, UpdatedEntityEventsError> {
        let replay_events = after.replay_events();
        let start = replay_events
            .iter()
            .position(|event| {
                (event.previous_lsn, event.previous_command_index)
                    == (loaded_lsn, loaded_command_index)
            })
            .ok_or(UpdatedEntityEventsError::EmptyEvents)?;
        let events = &replay_events[start..];
        let mut expected_previous = (loaded_lsn, loaded_command_index);
        for (event_index, event) in events.iter().enumerate() {
            if event.entity_id != after.entity_id() || event.entity_type != E::entity_type() {
                return Err(UpdatedEntityEventsError::EntityIdentityMismatch { event_index });
            }
            if (event.previous_lsn, event.previous_command_index) != expected_previous {
                return Err(UpdatedEntityEventsError::BrokenVersionChain {
                    event_index,
                    expected_lsn: expected_previous.0,
                    expected_command_index: expected_previous.1,
                    actual_lsn: event.previous_lsn,
                    actual_command_index: event.previous_command_index,
                });
            }
            expected_previous = (event.lsn, event.command_index);
        }
        if (after.lsn(), after.command_index()) != expected_previous {
            return Err(UpdatedEntityEventsError::AfterPositionMismatch {
                expected_lsn: expected_previous.0,
                expected_command_index: expected_previous.1,
                actual_lsn: after.lsn(),
                actual_command_index: after.command_index(),
            });
        }
        Ok(Self { after })
    }

    pub fn after(&self) -> &E {
        &self.after
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Entity {
        id: u64,
        lsn: u64,
        command_index: u32,
        replay_events: Vec<crate::ReplayableDomainEvent>,
    }

    impl crate::Shard for Entity {
        fn partition(&self) -> u8 {
            0
        }
    }

    impl crate::Entity for Entity {
        fn append_event(&mut self, event: crate::ReplayableDomainEvent) {
            self.replay_events.push(event);
        }

        fn replay_events(&self) -> &[crate::ReplayableDomainEvent] {
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
            self.lsn
        }

        fn command_index(&self) -> u32 {
            self.command_index
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

    fn event(
        id: u64,
        kind: u8,
        previous: (u64, u32),
        position: (u64, u32),
    ) -> ReplayableDomainEvent {
        ReplayableDomainEvent::new_at(
            0, 0, 20, previous.0, previous.1, position.0, position.1, id, kind, 1,
        )
    }

    #[test]
    fn accepts_two_updates_of_one_entity_at_the_same_lsn() {
        let changed = UpdatedEntityEvents::try_new(
            Entity {
                id: 7,
                lsn: 10,
                command_index: 1,
                replay_events: vec![
                    event(7, 2, (0, 0), (9, 3)),
                    event(7, 2, (9, 3), (10, 0)),
                    event(7, 2, (10, 0), (10, 1)),
                ],
            },
            9,
            3,
        )
        .unwrap();
        assert_eq!(
            crate::Entity::replay_events(changed.after())
                .iter()
                .map(|event| (event.lsn, event.command_index))
                .collect::<Vec<_>>(),
            vec![(9, 3), (10, 0), (10, 1)]
        );
    }

    #[test]
    fn rejects_empty_history() {
        assert!(matches!(
            UpdatedEntityEvents::try_new(
                Entity {
                    id: 7,
                    lsn: 9,
                    command_index: 0,
                    replay_events: Vec::new(),
                },
                9,
                0,
            ),
            Err(UpdatedEntityEventsError::EmptyEvents)
        ));
    }

    #[test]
    fn rejects_identity_mismatch() {
        assert!(matches!(
            UpdatedEntityEvents::try_new(
                Entity {
                    id: 7,
                    lsn: 10,
                    command_index: 1,
                    replay_events: vec![event(8, 2, (9, 0), (10, 1))],
                },
                9,
                0,
            ),
            Err(UpdatedEntityEventsError::EntityIdentityMismatch { event_index: 0 })
        ));
    }

    #[test]
    fn rejects_broken_chain() {
        assert!(matches!(
            UpdatedEntityEvents::try_new(
                Entity {
                    id: 7,
                    lsn: 10,
                    command_index: 1,
                    replay_events: vec![event(7, 2, (9, 0), (10, 0)), event(7, 2, (8, 0), (10, 1)),],
                },
                9,
                0,
            ),
            Err(UpdatedEntityEventsError::BrokenVersionChain { event_index: 1, .. })
        ));
    }

    #[test]
    fn rejects_after_position_mismatch() {
        assert!(matches!(
            UpdatedEntityEvents::try_new(
                Entity {
                    id: 7,
                    lsn: 10,
                    command_index: 1,
                    replay_events: vec![event(7, 2, (9, 0), (10, 0))],
                },
                9,
                0,
            ),
            Err(UpdatedEntityEventsError::AfterPositionMismatch { .. })
        ));
    }

    #[test]
    fn create_requires_zero_predecessor() {
        let after = Entity {
            id: 7,
            lsn: 10,
            command_index: 1,
            replay_events: Vec::new(),
        };
        assert!(CreatedEntityEvent::try_new(after.clone(), event(7, 2, (0, 0), (10, 1))).is_ok());
        assert!(matches!(
            CreatedEntityEvent::try_new(after, event(7, 2, (9, 0), (10, 1))),
            Err(CreatedEntityEventError::NonZeroPreviousPosition { .. })
        ));
    }
}

/// `StateChanged` 按确定业务顺序暴露本次 use case 的全部事件。
pub trait ReplayableEntityChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent>;
}

impl<E> ReplayableEntityChanges for CreatedEntityEvent<E> {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        vec![&self.event]
    }
}
