use crate::{Entity, EntityError, EntityReplayableEvent};

/// 命令自己声明执行该命令时，外部必须额外提供的输入类型。
///
/// 这是 `Command -> GivenState` 的配对契约，尤其用于 enum / command family 场景。
/// 它保留命令族需要的外部 authoritative state 绑定，避免把多个动作压成一个松散公共超集。
/// `GivenState` 不是状态机内部字段的别名，而是一次业务推导所需的外部输入运输形状。
pub trait CommandWithGivenState {
    type GivenState;
}

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
