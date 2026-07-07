use crate::{Entity, EntityError, EntityReplayableEvent};

/// 命令主体识别契约。
///
/// 默认没有可识别主体；需要审计、鉴权或链路追踪的命令可覆盖 `party_id`。
pub trait IssuedByParty {
    fn party_id(&self) -> Option<&str> {
        None
    }
}

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
