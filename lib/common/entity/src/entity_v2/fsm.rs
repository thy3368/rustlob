use super::shard::{LogAttr, Shard};
use crate::EntityReplayableEvent;

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

/// 将迁移期旧命令或显式 envelope 统一转换为已定序命令。
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
        let metadata = CommandMetadata::new(self.lsn(), self.logical_ts());
        SequencedCommand::new(metadata, self)
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
pub trait ReplayableDomainEvent {
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

/// 实体状态机：命令表达意图，领域事件表达成功后成立的事实。
pub trait Fsm {
    type State;
    type Command: LogAttr + Clone;
    type DomainEvent: From<Self::Command>;
    type Output;
    type Error;

    #[doc(hidden)]
    fn transition_legacy(
        &mut self,
        command: Self::Command,
    ) -> Result<(Self::State, Self::Output), Self::Error>;

    fn transition<C>(
        &mut self,
        command: C,
    ) -> Result<Transition<Self::State, Self::Output, Self::DomainEvent>, Self::Error>
    where
        C: IntoSequencedCommand<Self::Command>,
    {
        let command = command.into_sequenced();
        let event = Self::DomainEvent::from(command.payload.clone());
        let (state, output) = self.transition_legacy(command.payload)?;
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
