use crate::{EntityError, EntityReplayableEvent, LogAttr, Shard, TrackDiff};

/// 已由日志定序器确定的 v2 command 元数据。
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

/// v2 FSM 唯一接受的业务命令 envelope。
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

/// v2 FSM 一次成功迁移的完整结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition<S, O, E> {
    pub state: S,
    pub output: O,
    pub event: E,
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

/// v2 迁移的统一返回类型。
pub type TransitionResult<S, O, E, Err> = Result<Transition<S, O, E>, Err>;

/// v2 领域事件与实体 replay 事实之间的稳定映射。
pub trait ReplayableDomainEvent<C>: Sized {
    fn domain_event_type(command: &C) -> u8;
    fn from_command(command: C, replay_event: EntityReplayableEvent) -> Self;
    fn replayable_event(&self) -> &EntityReplayableEvent;
}

/// FSM v2：只接受已定序 command envelope 的单一路径。
///
/// 该契约独立定义完整执行路径。实现方必须提供 v2 自己的无提交业务迁移入口；
/// 新调用方显式构造 [`SequencedCommand`]。
pub trait Fsm: TrackDiff + Clone {
    type State;
    type Command: Clone;
    type DomainEvent: ReplayableDomainEvent<Self::Command>;
    type Output;
    type Error: From<EntityError>;

    /// 只在 `transition` 创建的 working clone 上执行业务迁移。
    ///
    /// 实现不得发布事件或产生外部副作用。
    #[doc(hidden)]
    fn transition_in_place(
        &mut self,
        metadata: CommandMetadata,
        command: Self::Command,
    ) -> Result<(Self::State, Self::Output), Self::Error>;

    /// 在一个 working clone 上完成业务迁移、replay 投影和领域事件构造，全部成功后
    /// 才原子替换原实体。
    fn transition(
        &mut self,
        command: SequencedCommand<Self::Command>,
    ) -> TransitionResult<Self::State, Self::Output, Self::DomainEvent, Self::Error> {
        let (metadata, payload) = command.into_parts();
        let mut working = self.clone();
        let (state, output) = working.transition_in_place(metadata, payload.clone())?;
        let mut replay_event =
            working.track_update_event_from_command(self, metadata.lsn, metadata.logical_ts)?;
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{CommandMetadata, Fsm as FsmV2, ReplayableDomainEvent, SequencedCommand};
    use crate::{Entity, EntityFieldChange, EntityReplayableEvent, LogAttr, Shard, TrackDiff};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestError {
        Business,
        Projection,
    }

    impl From<crate::EntityError> for TestError {
        fn from(_: crate::EntityError) -> Self {
            Self::Projection
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Command {
        delta: u64,
        fail: bool,
    }

    impl Shard for Command {
        fn topic(&self) -> u8 {
            2
        }

        fn partition(&self) -> u8 {
            3
        }

        fn party_id(&self) -> u64 {
            5
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct DomainEvent(EntityReplayableEvent);

    impl ReplayableDomainEvent<Command> for DomainEvent {
        fn domain_event_type(_: &Command) -> u8 {
            7
        }

        fn from_command(_: Command, replay_event: EntityReplayableEvent) -> Self {
            Self(replay_event)
        }

        fn replayable_event(&self) -> &EntityReplayableEvent {
            &self.0
        }
    }

    #[derive(Debug)]
    struct Subject {
        value: u64,
        lsn: u64,
        updated_at: u64,
        clones: Arc<AtomicUsize>,
    }

    impl Clone for Subject {
        fn clone(&self) -> Self {
            self.clones.fetch_add(1, Ordering::Relaxed);
            Self {
                value: self.value,
                lsn: self.lsn,
                updated_at: self.updated_at,
                clones: Arc::clone(&self.clones),
            }
        }
    }

    impl Shard for Subject {
        fn partition(&self) -> u8 {
            3
        }
    }

    impl Entity for Subject {
        fn entity_id(&self) -> u64 {
            11
        }

        fn owner_id(&self) -> String {
            "owner".into()
        }

        fn op_id(&self) -> String {
            "operator".into()
        }

        fn lsn(&self) -> u64 {
            self.lsn
        }

        fn created_at(&self) -> u64 {
            1
        }

        fn updated_at(&self) -> u64 {
            self.updated_at
        }

        fn is_derived() -> bool {
            false
        }

        fn entity_type() -> u8 {
            99
        }
    }

    impl TrackDiff for Subject {
        fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
            if self.value == other.value {
                return Vec::with_capacity(0);
            }
            vec![EntityFieldChange::new(
                "value",
                self.value.to_string(),
                other.value.to_string(),
            )]
        }
    }

    impl FsmV2 for Subject {
        type State = u64;
        type Command = Command;
        type DomainEvent = DomainEvent;
        type Output = u64;
        type Error = TestError;

        fn transition_in_place(
            &mut self,
            metadata: CommandMetadata,
            command: Command,
        ) -> Result<(Self::State, Self::Output), Self::Error> {
            self.value = self.value.saturating_add(command.delta);
            self.lsn = metadata.lsn;
            self.updated_at = metadata.logical_ts;
            if command.fail {
                return Err(TestError::Business);
            }
            Ok((self.value, command.delta))
        }
    }

    fn subject() -> Subject {
        Subject {
            value: 10,
            lsn: 4,
            updated_at: 8,
            clones: Arc::new(AtomicUsize::new(0)),
        }
    }

    #[test]
    fn transition_uses_one_clone_and_envelope_metadata() {
        let mut subject = subject();
        let clones = Arc::clone(&subject.clones);
        let result = FsmV2::transition(
            &mut subject,
            SequencedCommand::new(
                CommandMetadata::new(9, 13),
                Command {
                    delta: 2,
                    fail: false,
                },
            ),
        );
        let transition = match result {
            Ok(transition) => transition,
            Err(error) => panic!("v2 transition failed: {error:?}"),
        };

        assert_eq!(clones.load(Ordering::Relaxed), 1);
        assert_eq!(
            (subject.value, subject.lsn, subject.updated_at),
            (12, 9, 13)
        );
        assert_eq!((transition.state, transition.output), (12, 2));
        assert_eq!(transition.event.0.lsn, 9);
        assert_eq!(transition.event.0.logical_ts, 13);
        assert_eq!(transition.event.0.domain_event_type, 7);
    }

    #[test]
    fn envelope_owns_metadata_and_delegates_routing() {
        let command = SequencedCommand::new(
            CommandMetadata::new(9, 13),
            Command {
                delta: 2,
                fail: false,
            },
        );

        assert_eq!((command.lsn(), command.logical_ts()), (9, 13));
        assert_eq!(
            (command.topic(), command.partition(), command.party_id()),
            (2, 3, 5)
        );
    }

    #[test]
    fn business_failure_does_not_commit_working_state() {
        let mut subject = subject();
        let result = FsmV2::transition(
            &mut subject,
            SequencedCommand::new(
                CommandMetadata::new(9, 13),
                Command {
                    delta: 2,
                    fail: true,
                },
            ),
        );

        assert_eq!(result, Err(TestError::Business));
        assert_eq!((subject.value, subject.lsn, subject.updated_at), (10, 4, 8));
    }

    #[test]
    fn projection_failure_does_not_commit_working_state() {
        let mut subject = subject();
        let result = FsmV2::transition(
            &mut subject,
            SequencedCommand::new(
                CommandMetadata::new(9, 13),
                Command {
                    delta: 0,
                    fail: false,
                },
            ),
        );

        assert_eq!(result, Err(TestError::Projection));
        assert_eq!((subject.value, subject.lsn, subject.updated_at), (10, 4, 8));
    }
}
