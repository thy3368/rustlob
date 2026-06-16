use common_entity::{EntityError, EntityReplayableEvent};

/// V3 use case 的标准业务产出：
/// `output` 是当前用例内部可复用的强类型业务中间结果，
/// `events` 是唯一用于持久化 / 回放 / 发布的领域事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseOutput<O> {
    pub output: O,
    pub events: Vec<EntityReplayableEvent>,
}

/// V4 use case 的标准业务产出：
/// `changes` 是唯一业务真相，
/// `events` 是框架根据 `changes` 投影出的可持久化 / 可回放 / 可发布事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseChanges<C> {
    pub changes: C,
    pub events: Vec<EntityReplayableEvent>,
}

/// V4 change -> replayable events 投影阶段的统一错误。
pub type EventProjectError = EntityError;

/// 表达一次实体 update 必须同时保留 before / after。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedEntityPair<E> {
    pub before: E,
    pub after: E,
}

/// V4 changes 的最小框架契约：
/// 框架只要求业务 changes 能稳定投影成 replayable events。
pub trait ReplayableChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError>;
}

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

    /// 对command的检查
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

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

/// 更贴近跨 use case 业务复用的命令型抽象：
/// use case 一次推导出 typed output 与 replayable events。
///
/// 约束：
/// - 映射方向只定义为 `typed_output -> replayable_events`
/// - 该映射只在当前 use case 内部维护
/// - executor / persistence / replay / publish 只看 `events`
/// - 不提供通用 `events -> typed_output` 的框架恢复能力
pub trait CommandUseCase3: Send + Sync {
    /// 对应 cqrs 的 command。
    type Command: IssuedByParty;

    /// 对应 clean 架构中当前 use case 已加载的 GivenState。
    type GivenState;

    /// core.use_case 只表达业务错误。
    type Error: std::error::Error;

    /// 当前用例内部可复用的强类型业务中间结果。
    type Output;

    /// 对应四色建模的 role。
    fn role(&self) -> &'static str {
        "UnknownActor用来做权限控制和追溯"
    }

    /// 对 command 的快速检查。
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 对状态校验，可为空。
    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 一次业务推导，同时给出 typed output 与 replayable events。
    ///
    /// 要求：
    /// - `output` 必须是纯业务结果
    /// - `events` 必须由 `output` 推导生成
    /// - 不允许分别维护两套互相独立的 output / event 生成逻辑
    fn compute_output_and_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error>;
}

/// V4 收敛为 changes-first：
/// use case 只返回强类型业务 changes，
/// executor 统一把 changes 投影为 replayable events。
pub trait CommandUseCase4: Send + Sync {
    /// 对应 cqrs 的 command。
    type Command: IssuedByParty;

    /// 对应 clean 架构中当前 use case 已加载的 GivenState。
    type GivenState;

    /// core.use_case 只表达业务错误。
    type Error: std::error::Error;

    /// 当前 use case 的唯一业务真相。
    type Changes: ReplayableChanges;

    /// 对应四色建模的 role。
    fn role(&self) -> &'static str {
        "UnknownActor用来做权限控制和追溯"
    }

    /// 对 command 的快速检查。
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 对状态校验，可为空。
    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 一次业务推导，只返回强类型领域 changes。
    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error>;
}

/// 对外回复映射移出核心 Use Case，交给 Interface Adapters（接口适配器）。
pub trait UseCaseReplyMapper: Send + Sync {
    type Reply;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply;
}

/// V3 reply mapper 直接消费 typed output + replayable events 的组合结果。
pub trait UseCaseReplyMapper3: Send + Sync {
    type Output;
    type Reply;

    fn map(&self, result: UseCaseOutput<Self::Output>) -> Self::Reply;
}
