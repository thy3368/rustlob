use crate::EntityReplayableEvent;

/// 统一处理可回放实体事件的静态 handler 抽象。
///
/// 非目标实体或非目标变更类型返回 `Ok(None)`；目标事件解析失败或业务执行失败返回
/// `Err(...)`。
pub trait EntityReplayableEventHandler {
    type Output;
    type Error;

    fn handle(
        &mut self,
        event: &EntityReplayableEvent,
    ) -> Result<Option<Self::Output>, Self::Error>;
}
