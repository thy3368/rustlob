use super::shard::{LogAttr, Shard};
use crate::{EntityError, EntityReplayableEvent, TrackDiff};

/// 已由日志定序器确定的命令元数据。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandMetadata {
    pub lsn: u64,
    pub logical_ts: u64,
}

impl CommandMetadata {
    pub const fn new(lsn: u64, logical_ts: u64) -> Self {
        Self { lsn, logical_ts }
    }
}

/// 带确定性日志位置的业务命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequencedCommand<C> {
    pub metadata: CommandMetadata,
    pub payload: C,
}

impl<C> SequencedCommand<C> {
    pub const fn new(metadata: CommandMetadata, payload: C) -> Self {
        Self { metadata, payload }
    }

    pub fn into_parts(self) -> (CommandMetadata, C) {
        (self.metadata, self.payload)
    }
}

/// 迁移期输入适配；新调用点应显式构造 [`SequencedCommand`]。
pub trait IntoSequencedCommand<C> {
    fn into_sequenced(self) -> SequencedCommand<C>;
}

impl<C> IntoSequencedCommand<C> for SequencedCommand<C> {
    fn into_sequenced(self) -> SequencedCommand<C> {
        self
    }
}

impl<C: LogAttr> IntoSequencedCommand<C> for C {
    fn into_sequenced(self) -> SequencedCommand<C> {
        SequencedCommand::new(CommandMetadata::new(self.lsn(), self.logical_ts()), self)
    }
}

impl<C: Shard> Shard for SequencedCommand<C> {
    fn topic(&self) -> u8 {
        self.payload.topic()
    }
    fn partition(&self) -> u8 {
        self.payload.partition()
    }
    fn party_id(&self) -> u64 {
        self.payload.party_id()
    }
}

impl<C: Shard> LogAttr for SequencedCommand<C> {
    fn lsn(&self) -> u64 {
        self.metadata.lsn
    }
    fn logical_ts(&self) -> u64 {
        self.metadata.logical_ts
    }
}

/// use case 发布的、带命令因果位置的领域事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequencedDomainEvent<E> {
    pub metadata: CommandMetadata,
    pub event_index: u32,
    pub payload: E,
}

impl<E> SequencedDomainEvent<E> {
    pub const fn new(metadata: CommandMetadata, event_index: u32, payload: E) -> Self {
        Self {
            metadata,
            event_index,
            payload,
        }
    }
}

impl<E: Shard> Shard for SequencedDomainEvent<E> {
    fn topic(&self) -> u8 {
        self.payload.topic()
    }
    fn partition(&self) -> u8 {
        self.payload.partition()
    }
    fn party_id(&self) -> u64 {
        self.payload.party_id()
    }
}

impl<E: Shard> LogAttr for SequencedDomainEvent<E> {
    fn lsn(&self) -> u64 {
        self.metadata.lsn
    }
    fn logical_ts(&self) -> u64 {
        self.metadata.logical_ts
    }
}

/// 一次成功状态迁移提交后的完整结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition<S, O, E> {
    pub state: S,
    pub output: O,
    pub event: E,
}

/// 统一读取领域事件内嵌的实体 replay 事实。
pub trait ReplayableDomainEvent<C>: Sized {
    fn domain_event_type(command: &C) -> u8;
    fn from_command(command: C, replay_event: EntityReplayableEvent) -> Self;
    fn replayable_event(&self) -> &EntityReplayableEvent;
}

impl<S, O, E> Transition<S, O, E> {
    pub const fn new(state: S, output: O, event: E) -> Self {
        Self {
            state,
            output,
            event,
        }
    }
}

/// 以 command envelope 驱动、可确定性回放且原子提交的实体状态机。
pub trait Fsm: TrackDiff + Clone {
    type State;
    type Command: Clone;
    type DomainEvent: ReplayableDomainEvent<Self::Command>;
    type Output;
    type Error: From<EntityError>;

    #[doc(hidden)]
    fn transition_working(
        &mut self,
        metadata: CommandMetadata,
        command: Self::Command,
    ) -> Result<(Self::State, Self::Output), Self::Error>;

    fn transition<C>(
        &mut self,
        command: C,
    ) -> Result<Transition<Self::State, Self::Output, Self::DomainEvent>, Self::Error>
    where
        C: IntoSequencedCommand<Self::Command>,
    {
        let (metadata, payload) = command.into_sequenced().into_parts();
        let before = self.clone();
        let mut working = before.clone();
        let (state, output) = working.transition_working(metadata, payload.clone())?;
        let mut replay_event =
            working.track_update_event_from_command(&before, metadata.lsn, metadata.logical_ts)?;
        let domain_event_type = Self::DomainEvent::domain_event_type(&payload);
        if domain_event_type == 0 {
            return Err(
                EntityError::Custom("FSM domain event type must be non-zero".into()).into(),
            );
        }
        replay_event.domain_event_type = domain_event_type;
        let event = Self::DomainEvent::from_command(payload, replay_event);
        *self = working;
        Ok(Transition::new(state, output, event))
    }

    fn apply<C>(
        &mut self,
        command: C,
    ) -> Result<Transition<Self::State, Self::Output, Self::DomainEvent>, Self::Error>
    where
        C: IntoSequencedCommand<Self::Command>,
    {
        self.transition(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Command;

    impl Shard for Command {
        fn topic(&self) -> u8 {
            3
        }
        fn partition(&self) -> u8 {
            7
        }
        fn party_id(&self) -> u64 {
            11
        }
    }

    #[test]
    fn sequenced_command_delegates_routing_and_owns_log_metadata() {
        let command = SequencedCommand::new(CommandMetadata::new(13, 17), Command);
        assert_eq!(command.topic(), 3);
        assert_eq!(command.partition(), 7);
        assert_eq!(command.party_id(), 11);
        assert_eq!(command.lsn(), 13);
        assert_eq!(command.logical_ts(), 17);
    }

    #[test]
    fn sequenced_domain_event_preserves_causal_metadata_and_index() {
        let event = SequencedDomainEvent::new(CommandMetadata::new(13, 17), 2, Command);
        assert_eq!(event.lsn(), 13);
        assert_eq!(event.logical_ts(), 17);
        assert_eq!(event.event_index, 2);
        assert_eq!(event.partition(), 7);
    }
}
