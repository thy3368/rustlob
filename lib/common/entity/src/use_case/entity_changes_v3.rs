use crate::{EventPosition, ReplayableDomainEvent};

/// 为逐实体变更提供稳定 identity 与最终复合版本位置。
pub trait PositionedEntity {
    fn entity_id(&self) -> u64;
    fn entity_type(&self) -> u8;
    fn event_position(&self) -> EventPosition;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdatedEntityEventsError {
    EmptyEvents,
    EntityIdentityMismatch {
        event_index: usize,
    },
    BrokenVersionChain {
        event_index: usize,
        expected: EventPosition,
        actual: EventPosition,
    },
    AfterPositionMismatch {
        expected: EventPosition,
        actual: EventPosition,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatedEntityEventError {
    EntityIdentityMismatch,
    NonZeroPreviousPosition { actual: EventPosition },
    AfterPositionMismatch {
        expected: EventPosition,
        actual: EventPosition,
    },
}

/// 新建实体的 after truth 与唯一 create replay event。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedEntityEvent<E> {
    after: E,
    event: ReplayableDomainEvent,
}

impl<E: PositionedEntity> CreatedEntityEvent<E> {
    pub fn try_new(
        after: E,
        event: ReplayableDomainEvent,
    ) -> Result<Self, CreatedEntityEventError> {
        if event.entity_id != after.entity_id() || event.entity_type != after.entity_type() {
            return Err(CreatedEntityEventError::EntityIdentityMismatch);
        }
        if event.previous_position != EventPosition::ZERO {
            return Err(CreatedEntityEventError::NonZeroPreviousPosition {
                actual: event.previous_position,
            });
        }
        if after.event_position() != event.position {
            return Err(CreatedEntityEventError::AfterPositionMismatch {
                expected: event.position,
                actual: after.event_position(),
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

/// 至少包含一条 replay event 的集合。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyReplayEvents(Vec<ReplayableDomainEvent>);

impl NonEmptyReplayEvents {
    pub fn try_new(events: Vec<ReplayableDomainEvent>) -> Result<Self, UpdatedEntityEventsError> {
        if events.is_empty() {
            return Err(UpdatedEntityEventsError::EmptyEvents);
        }
        Ok(Self(events))
    }

    pub fn as_slice(&self) -> &[ReplayableDomainEvent] {
        &self.0
    }
}

/// 一个实体在本次 use case 中的完整更新历史及最终 after truth。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedEntityEvents<E> {
    after: E,
    events: NonEmptyReplayEvents,
}

impl<E: PositionedEntity> UpdatedEntityEvents<E> {
    pub fn try_new(
        after: E,
        loaded_position: EventPosition,
        events: Vec<ReplayableDomainEvent>,
    ) -> Result<Self, UpdatedEntityEventsError> {
        let events = NonEmptyReplayEvents::try_new(events)?;
        let mut expected_previous = loaded_position;
        for (event_index, event) in events.as_slice().iter().enumerate() {
            if event.entity_id != after.entity_id() || event.entity_type != after.entity_type() {
                return Err(UpdatedEntityEventsError::EntityIdentityMismatch { event_index });
            }
            if event.previous_position != expected_previous {
                return Err(UpdatedEntityEventsError::BrokenVersionChain {
                    event_index,
                    expected: expected_previous,
                    actual: event.previous_position,
                });
            }
            expected_previous = event.position;
        }
        if after.event_position() != expected_previous {
            return Err(UpdatedEntityEventsError::AfterPositionMismatch {
                expected: expected_previous,
                actual: after.event_position(),
            });
        }
        Ok(Self { after, events })
    }

    pub fn after(&self) -> &E {
        &self.after
    }

    pub fn events(&self) -> &[ReplayableDomainEvent] {
        self.events.as_slice()
    }
}

/// `StateChanged` 按确定业务顺序暴露本次 use case 的全部事件。
pub trait ReplayableEntityChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent>;
}

impl<E> ReplayableEntityChanges for UpdatedEntityEvents<E> {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        self.events.as_slice().iter().collect()
    }
}

impl<E> ReplayableEntityChanges for CreatedEntityEvent<E> {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        vec![&self.event]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Entity {
        id: u64,
        kind: u8,
        position: EventPosition,
    }

    impl PositionedEntity for Entity {
        fn entity_id(&self) -> u64 {
            self.id
        }
        fn entity_type(&self) -> u8 {
            self.kind
        }
        fn event_position(&self) -> EventPosition {
            self.position
        }
    }

    fn event(
        id: u64,
        kind: u8,
        previous: EventPosition,
        position: EventPosition,
    ) -> ReplayableDomainEvent {
        ReplayableDomainEvent::new_at(0, 0, 20, previous, position, id, kind, 1)
    }

    #[test]
    fn accepts_two_updates_of_one_entity_at_the_same_lsn() {
        let p0 = EventPosition::new(9, 3);
        let p1 = EventPosition::new(10, 0);
        let p2 = EventPosition::new(10, 1);
        let changed = UpdatedEntityEvents::try_new(
            Entity {
                id: 7,
                kind: 2,
                position: p2,
            },
            p0,
            vec![event(7, 2, p0, p1), event(7, 2, p1, p2)],
        )
        .unwrap();
        assert_eq!(
            changed
                .events()
                .iter()
                .map(|event| event.position)
                .collect::<Vec<_>>(),
            vec![p1, p2]
        );
    }

    #[test]
    fn rejects_empty_identity_chain_and_after_position_errors() {
        let p0 = EventPosition::new(9, 0);
        let p1 = EventPosition::new(10, 0);
        let after = || Entity {
            id: 7,
            kind: 2,
            position: p1,
        };
        assert_eq!(
            UpdatedEntityEvents::try_new(after(), p0, vec![]),
            Err(UpdatedEntityEventsError::EmptyEvents)
        );
        assert_eq!(
            UpdatedEntityEvents::try_new(after(), p0, vec![event(8, 2, p0, p1)]),
            Err(UpdatedEntityEventsError::EntityIdentityMismatch { event_index: 0 })
        );
        assert!(matches!(
            UpdatedEntityEvents::try_new(after(), p0, vec![event(7, 2, EventPosition::ZERO, p1)]),
            Err(UpdatedEntityEventsError::BrokenVersionChain { event_index: 0, .. })
        ));
        assert!(matches!(
            UpdatedEntityEvents::try_new(
                Entity {
                    id: 7,
                    kind: 2,
                    position: p0
                },
                p0,
                vec![event(7, 2, p0, p1)]
            ),
            Err(UpdatedEntityEventsError::AfterPositionMismatch { .. })
        ));
    }
}
