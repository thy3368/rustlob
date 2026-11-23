use crate::{EntityError, EventPosition, LogAttr, ReplayableDomainEvent, Shard, TrackDiff};

/// 已由日志定序器确定的 v2 command 元数据。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandMetadata {
    pub lsn: u64,
    pub position: EventPosition,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
    pub party_id: String,
}

impl CommandMetadata {
    pub fn new(lsn: u64, logical_ts: u64, topic: u8, partition: u8, party_id: String) -> Self {
        Self {
            lsn,
            position: EventPosition::new(lsn, 0),
            logical_ts,
            topic,
            partition,
            party_id,
        }
    }

    pub fn new_at(
        position: EventPosition,
        logical_ts: u64,
        topic: u8,
        partition: u8,
        party_id: String,
    ) -> Self {
        Self {
            lsn: position.lsn,
            position,
            logical_ts,
            topic,
            partition,
            party_id,
        }
    }
}

impl Shard for CommandMetadata {
    fn topic(&self) -> u8 {
        self.topic
    }

    fn partition(&self) -> u8 {
        self.partition
    }

    fn party_id(&self) -> String {
        self.party_id.clone()
    }
}

impl LogAttr for CommandMetadata {
    fn lsn(&self) -> u64 {
        self.lsn
    }

    fn logical_ts(&self) -> u64 {
        self.logical_ts
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

impl<C> Shard for SequencedCommand<C> {
    fn topic(&self) -> u8 {
        self.metadata.topic()
    }

    fn partition(&self) -> u8 {
        self.metadata.partition()
    }

    fn party_id(&self) -> String {
        self.metadata.party_id()
    }
}

impl<C> LogAttr for SequencedCommand<C> {
    fn lsn(&self) -> u64 {
        self.metadata.lsn
    }

    fn logical_ts(&self) -> u64 {
        self.metadata.logical_ts
    }
}

/// FSM v2：只接受已定序 command envelope 的单一路径。
///
/// 该契约独立定义完整执行路径。实现方必须提供 v2 自己的无提交业务迁移入口；
/// 新调用方显式构造 [`SequencedCommand`]。
pub trait Fsm: TrackDiff + Sized {
    type Command;
    type Output;
    type Error: From<EntityError>;

    /// 基于当前实体只读推导迁移后的完整实体和业务输出。
    ///
    /// 实现不得修改当前实体、发布事件或产生外部副作用。
    #[doc(hidden)]
    fn derive_transition(
        &self,
        metadata: CommandMetadata,
        command: Self::Command,
    ) -> Result<(Self, Self::Output), Self::Error>;

    /// 返回 command 对应的稳定、非零业务事件码。
    fn event_type(command: &Self::Command) -> u8;

    /// 先纯计算 after entity，再完成 replay 投影；原实体始终保持不变。
    fn transition(
        &self,
        command: SequencedCommand<Self::Command>,
    ) -> Result<(Self, ReplayableDomainEvent, Self::Output), Self::Error> {
        let (metadata, payload) = command.into_parts();
        let event_type = Self::event_type(&payload);
        if event_type == 0 {
            return Err(EntityError::Custom("FSM event type must be non-zero".into()).into());
        }
        let logical_ts = metadata.logical_ts;
        let position = metadata.position;
        let (after, output) = self.derive_transition(metadata, payload)?;
        let mut replay_event =
            after.track_update_event_from_at(self, self.event_position(), position, logical_ts)?;
        replay_event.domain_event_type = event_type;
        Ok((after, replay_event, output))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
    use crate::{Entity, EntityFieldChange, LogAttr, Shard, TrackDiff};

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

    #[derive(Debug, PartialEq, Eq)]
    struct Command {
        delta: u64,
        fail: bool,
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
        type Command = Command;
        type Output = u64;
        type Error = TestError;

        fn event_type(command: &Command) -> u8 {
            if command.delta == u64::MAX { 0 } else { 7 }
        }

        fn derive_transition(
            &self,
            metadata: CommandMetadata,
            command: Command,
        ) -> Result<(Self, Self::Output), Self::Error> {
            let value = self.value.saturating_add(command.delta);
            if command.fail {
                return Err(TestError::Business);
            }
            Ok((
                Self {
                    value,
                    lsn: metadata.lsn,
                    updated_at: metadata.logical_ts,
                    clones: Arc::clone(&self.clones),
                },
                command.delta,
            ))
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
    fn transition_uses_zero_clones_and_envelope_metadata() {
        let subject = subject();
        let clones = Arc::clone(&subject.clones);
        let result = FsmV2::transition(
            &subject,
            SequencedCommand::new(
                CommandMetadata::new(9, 13, 2, 3, "5".to_owned()),
                Command {
                    delta: 2,
                    fail: false,
                },
            ),
        );
        let (after, event, output) = match result {
            Ok(transition) => transition,
            Err(error) => panic!("v2 transition failed: {error:?}"),
        };

        assert_eq!(clones.load(Ordering::Relaxed), 0);
        assert_eq!((subject.value, subject.lsn, subject.updated_at), (10, 4, 8));
        assert_eq!(
            (after.value, after.lsn, after.updated_at, output),
            (12, 9, 13, 2)
        );
        assert_eq!(event.lsn, 9);
        assert_eq!(event.logical_ts, 13);
        assert_eq!(event.domain_event_type, 7);
    }

    #[test]
    fn envelope_owns_metadata_and_delegates_routing() {
        let command = SequencedCommand::new(
            CommandMetadata::new(9, 13, 2, 3, "5".to_owned()),
            Command {
                delta: 2,
                fail: false,
            },
        );

        assert_eq!((command.lsn(), command.logical_ts()), (9, 13));
        assert_eq!(
            (command.topic(), command.partition(), command.party_id()),
            (2, 3, "5".to_owned())
        );
    }

    #[test]
    fn business_failure_does_not_commit_working_state() {
        let subject = subject();
        let result = FsmV2::transition(
            &subject,
            SequencedCommand::new(
                CommandMetadata::new(9, 13, 2, 3, "5".to_owned()),
                Command {
                    delta: 2,
                    fail: true,
                },
            ),
        );

        assert!(matches!(result, Err(TestError::Business)));
        assert_eq!((subject.value, subject.lsn, subject.updated_at), (10, 4, 8));
    }

    #[test]
    fn projection_failure_does_not_commit_working_state() {
        let subject = subject();
        let result = FsmV2::transition(
            &subject,
            SequencedCommand::new(
                CommandMetadata::new(9, 13, 2, 3, "5".to_owned()),
                Command {
                    delta: 0,
                    fail: false,
                },
            ),
        );

        assert!(matches!(result, Err(TestError::Projection)));
        assert_eq!((subject.value, subject.lsn, subject.updated_at), (10, 4, 8));
    }

    #[test]
    fn zero_event_type_is_rejected_before_transition() {
        let subject = subject();
        let clones = Arc::clone(&subject.clones);
        let result = FsmV2::transition(
            &subject,
            SequencedCommand::new(
                CommandMetadata::new(9, 13, 2, 3, "5".to_owned()),
                Command {
                    delta: u64::MAX,
                    fail: false,
                },
            ),
        );

        assert!(matches!(result, Err(TestError::Projection)));
        assert_eq!(clones.load(Ordering::Relaxed), 0);
        assert_eq!((subject.value, subject.lsn, subject.updated_at), (10, 4, 8));
    }
}
