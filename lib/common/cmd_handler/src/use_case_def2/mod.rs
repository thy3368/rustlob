mod executor;
mod trace;

use diff::EntityReplayableEvent;
pub use executor::CommandUseCaseExecutor2;
use thiserror::Error;

use crate::HandlerLatencyMetrics;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandMeta {
    /// Tracing correlation id for spans/logs across retries and service hops.
    /// This is only for observability and troubleshooting, not for business idempotency.
    pub trace_id: Option<String>,
    /// Stable business command identity.
    /// Use this as the primary idempotency and deduplication key for the same business command.
    /// Retries of the same command should keep the same command_id.
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandEnvelope<C> {
    pub meta: CommandMeta,
    pub command: C,
}

/// Business actor instance carried by the command.
/// Semantically: `party_id` plays the `role()` of the use case and issues the command.
pub trait IssuedByParty {
    fn party_id(&self) -> Option<&str> {
        None
    }
}

/// 更贴近 Use Cases（用例）的命令型抽象：
/// 只定义业务输入、业务校验与可重放事件产出。
pub trait CommandUseCase2: Send + Sync {
    /// 对应cqrs的 command
    type Command: IssuedByParty;

    /// 对应clean 架构的 entity , 从数据库/内存/文件等
    type GivenState;

    /// core.use_case 只表达业务错误。
    type Error: std::error::Error;

    /// 对应四色建模的 role。
    /// 语义上：`party_id()` 标识哪个业务主体，以这个 role 下达当前 command。
    fn role(&self) -> &'static str {
        "UnknownActor用来做权限控制和追溯"
    }

    /// Optional hook for adapters that want a stable, use-case-specific error string.
    fn format_error(&self, _error: &Self::Error) -> Option<String> {
        None
    }

    /// 对command的检查
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error>;

    /// 对状态较验，可为空
    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 计算可重放事件，核心方法，测试要覆盖 cmd/state矩阵
    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error>;
}

/// 对外回复映射移出核心 Use Case，交给 Interface Adapters（接口适配器）。
pub trait UseCaseReplyMapper: Send + Sync {
    type Reply;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply;
}

/// Use case 视角下统一的 outbound port。
///
/// `load_state / persist / replay / publish` 都属于 adapter.outbound，
/// executor 只依赖这一个抽象。
pub trait CommandUseCaseOutbound: Send + Sync {
    type Command;
    type State;
    type Error: std::error::Error;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error>;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandUseCaseOutboundPhase {
    LoadState,
    Persist,
    Replay,
    Publish,
}

impl std::fmt::Display for CommandUseCaseOutboundPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadState => f.write_str("load_state"),
            Self::Persist => f.write_str("persist"),
            Self::Replay => f.write_str("replay"),
            Self::Publish => f.write_str("publish"),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommandUseCaseExecutionError<BusinessError, OutboundError>
where
    BusinessError: std::error::Error + 'static,
    OutboundError: std::error::Error + 'static,
{
    #[error(transparent)]
    Business(#[from] BusinessError),
    #[error("outbound {phase} failed: {source}")]
    Outbound {
        phase: CommandUseCaseOutboundPhase,
        #[source]
        source: OutboundError,
    },
}

impl<BusinessError, OutboundError> CommandUseCaseExecutionError<BusinessError, OutboundError>
where
    BusinessError: std::error::Error + 'static,
    OutboundError: std::error::Error + 'static,
{
    pub fn outbound(
        phase: CommandUseCaseOutboundPhase,
        source: OutboundError,
    ) -> CommandUseCaseExecutionError<BusinessError, OutboundError> {
        Self::Outbound { phase, source }
    }
}

impl<T> CommandUseCaseOutbound for &T
where
    T: ?Sized + CommandUseCaseOutbound,
{
    type Command = T::Command;
    type State = T::State;
    type Error = T::Error;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        (*self).load_state(cmd)
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).persist(events)
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).replay(events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).publish(events)
    }
}

/// latency 观察端口，由执行编排侧注入。
pub trait ObserveHandlerLatency: Send + Sync {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics);
}

impl ObserveHandlerLatency for () {
    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}
