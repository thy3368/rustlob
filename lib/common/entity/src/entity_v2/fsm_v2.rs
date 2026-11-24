use crate::{EntityError, LogAttr, ReplayableDomainEvent, Shard, TrackDiff};

/// 已由日志定序器确定的 v2 command 元数据。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandMetadata {
    pub lsn: u64,
    pub command_index: u32,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
    pub party_id: String,
}

impl CommandMetadata {
    pub fn new(lsn: u64, logical_ts: u64, topic: u8, partition: u8, party_id: String) -> Self {
        Self {
            lsn,
            command_index: 0,
            logical_ts,
            topic,
            partition,
            party_id,
        }
    }

    pub fn new_at(
        lsn: u64,
        command_index: u32,
        logical_ts: u64,
        topic: u8,
        partition: u8,
        party_id: String,
    ) -> Self {
        Self {
            lsn,
            command_index,
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
    type Error: From<EntityError>;

    /// 基于当前实体只读推导迁移后的完整实体。
    ///
    /// 实现不得修改当前实体、发布事件或产生外部副作用。
    #[doc(hidden)]
    fn derive_transition(
        &self,
        command: SequencedCommand<Self::Command>,
    ) -> Result<Self, Self::Error>;

    /// 返回 command 对应的稳定、非零业务事件码。
    fn event_type(command: &Self::Command) -> u8;

    /// 先纯计算 after entity，再完成 replay 投影；原实体始终保持不变。
    ///
    /// replay event 的 `topic` 和 `partition` 以已定序 command metadata 为权威来源，
    /// 不使用实体自身的 shard 覆盖日志路由。
    fn transition(
        &self,
        command: SequencedCommand<Self::Command>,
    ) -> Result<(Self, ReplayableDomainEvent), Self::Error> {
        let event_type = Self::event_type(&command.payload);
        if event_type == 0 {
            return Err(EntityError::Custom("FSM event type must be non-zero".into()).into());
        }
        let logical_ts = command.metadata.logical_ts;
        let lsn = command.metadata.lsn;
        let command_index = command.metadata.command_index;
        let topic = command.metadata.topic;
        let partition = command.metadata.partition;
        let after = self.derive_transition(command)?;
        let mut replay_event = after.track_update_event_from_at(
            self,
            self.lsn(),
            self.command_index(),
            lsn,
            command_index,
            logical_ts,
        )?;
        replay_event.t = topic;
        replay_event.p = partition;
        replay_event.domain_event_type = event_type;
        Ok((after, replay_event))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entity, EntityFieldChange};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEntity {
        value: u64,
        lsn: u64,
        command_index: u32,
        logical_ts: u64,
    }

    impl Shard for TestEntity {
        fn topic(&self) -> u8 {
            1
        }

        fn partition(&self) -> u8 {
            2
        }

        fn party_id(&self) -> String {
            "entity-party".to_owned()
        }
    }

    impl Entity for TestEntity {
        fn entity_id(&self) -> u64 {
            11
        }

        fn owner_id(&self) -> String {
            "owner".to_owned()
        }

        fn op_id(&self) -> String {
            "operator".to_owned()
        }

        fn lsn(&self) -> u64 {
            self.lsn
        }

        fn command_index(&self) -> u32 {
            self.command_index
        }

        fn created_at(&self) -> u64 {
            3
        }

        fn updated_at(&self) -> u64 {
            self.logical_ts
        }

        fn is_derived() -> bool {
            false
        }

        fn entity_type() -> u8 {
            12
        }
    }

    impl TrackDiff for TestEntity {
        fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
            (self.value != other.value)
                .then(|| {
                    EntityFieldChange::new("value", self.value.to_string(), other.value.to_string())
                })
                .into_iter()
                .collect()
        }
    }

    impl Fsm for TestEntity {
        type Command = u64;
        type Error = EntityError;

        fn derive_transition(
            &self,
            command: SequencedCommand<Self::Command>,
        ) -> Result<Self, Self::Error> {
            let mut after = self.clone();
            after.value = command.payload;
            after.lsn = command.metadata.lsn;
            after.command_index = command.metadata.command_index;
            after.logical_ts = command.metadata.logical_ts;
            Ok(after)
        }

        fn event_type(_command: &Self::Command) -> u8 {
            9
        }
    }

    #[test]
    fn transition_uses_command_routing_and_preserves_replay_metadata() {
        let before = TestEntity {
            value: 5,
            lsn: 20,
            command_index: 4,
            logical_ts: 30,
        };
        let metadata = CommandMetadata::new_at(21, 7, 31, 8, 6, "command-party".to_owned());

        let (after, event) = before
            .transition(SequencedCommand::new(metadata, 6))
            .expect("FSM transition should succeed");

        assert_eq!(after.value, 6);
        assert_eq!((event.t, event.p), (8, 6));
        assert_eq!((event.lsn, event.command_index), (21, 7));
        assert_eq!((event.previous_lsn, event.previous_command_index), (20, 4));
        assert_eq!(event.logical_ts, 31);
        assert_eq!(event.domain_event_type, 9);
    }
}
