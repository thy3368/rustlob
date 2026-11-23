use crate::{EntityError, LogAttr, ReplayableDomainEvent, Shard, TrackDiff};

/// 已由日志定序器确定的 v2 command 元数据。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandMetadata {
    pub lsn: u64,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
    pub party_id: String,
}

impl CommandMetadata {
    pub fn new(lsn: u64, logical_ts: u64, topic: u8, partition: u8, party_id: String) -> Self {
        Self {
            lsn,
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
    type State;
    type Command;
    type Output;
    type Error: From<EntityError>;

    /// 基于当前实体只读推导迁移后的完整实体、状态和业务输出。
    ///
    /// 实现不得修改当前实体、发布事件或产生外部副作用。
    #[doc(hidden)]
    fn derive_transition(
        &mut self,
        metadata: CommandMetadata,
        command: Self::Command,
    ) -> Result<Self::Output, Self::Error>;

    /// 返回 command 对应的稳定、非零业务事件码。
    fn event_type(command: &Self::Command) -> u8;

    /// 先纯计算 after entity，再完成 replay 投影；全部成功后才原子替换原实体。
    fn transition(
        &mut self,
        command: SequencedCommand<Self::Command>,
    ) -> Result<(ReplayableDomainEvent, Self::Output), Self::Error> {
        ///主要问题，可能不原子， error报错，但self已经改了
        todo!()
    }
}
