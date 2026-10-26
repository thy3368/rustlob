use crate::{Entity, EntityError, EntityReplayableEvent};

/// 实体业务变更的最小回放契约。
///
/// `Changes` 是一次业务 case 产生的唯一业务真相；后续 replay / persist / publish
/// 所需事件统一由它稳定投影。
pub trait ReplayableChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError>;
}

/// 单个实体实例的一次最终 before/after 更新真相。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedEntityPair<E> {
    pub before: E,
    pub after: E,
}

impl<E> UpdatedEntityPair<E> {
    #[inline]
    pub fn new(before: E, after: E) -> Self {
        Self { before, after }
    }
}

impl<E> UpdatedEntityPair<E>
where
    E: Entity,
{
    #[inline]
    pub fn to_replayable_event(&self) -> Result<EntityReplayableEvent, EntityError> {
        self.after.track_update_event_from(&self.before)
    }
}
