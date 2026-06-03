use diff::EntityReplayableEvent;

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

/// 对外回复映射移出核心 Use Case，交给 Interface Adapters（接口适配器）。
pub trait UseCaseReplyMapper: Send + Sync {
    type Reply;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply;
}
