use crate::{CommandWithGivenState, ReplayableChanges};

/// 业务命令携带发起主体的最小契约。
///
/// 这只表达“命令由哪个业务主体发出”，不承载权限判断本身。
pub trait IssuedByParty {
    fn party_id(&self) -> Option<&str> {
        None
    }
}

/// Command-style use case 的业务契约。
///
/// `CommandUseCase6` 把一个主 MI 状态机组收敛为三组总 enum：
/// - `Command`
/// - `GivenState`
/// - `Changes`
///
/// 它的职责是编排业务输入 `Command` 与已加载的 `GivenState`，
/// 调用聚合根对外公开的业务方法，完成一次业务目标推导，并产出唯一业务真相 `Changes`。
/// 后续可持久化、可回放、可发布的事实由 `ReplayableChanges::to_replayable_events()` 从
/// `Changes` 投影得到。
///
/// 这个 trait 不负责：
/// - DB 访问或其他状态加载
/// - replayable events 的发布或持久化执行
/// - HTTP / WebSocket reply shaping
/// - 权限、鉴权、审计等基础设施实现
///
/// 三者分支必须在 `compute_before_after_changes(cmd, &state)` 内显式匹配，
/// 分支错配时必须返回明确业务错误。
pub trait CommandUseCase6: Send + Sync {
    /// 当前业务输入。
    ///
    /// 它既表达一次业务动作，也必须能映射到对应 `GivenState`，
    /// 以便 outbound 先加载出本次推导所需的 authoritative state。
    type Command: IssuedByParty + CommandWithGivenState;

    /// 当前 use case 的业务错误。
    ///
    /// 这里只表达业务拒绝或业务推导失败，不承载基础设施层错误编排。
    type Error: std::error::Error;

    /// 当前 use case 的唯一业务真相。
    ///
    /// `Changes` 应保持强类型业务语义，后续 replayable facts 统一由
    /// `ReplayableChanges` 投影，而不是并列维护第二条业务真相路径。
    type Changes: ReplayableChanges;

    /// 对 `Command` 做不依赖状态的快速校验。
    ///
    /// 这里适合放命令自身即可判定的拒绝条件，例如字段缺失、格式非法、
    /// 明显违反命令基本约束等；不要在这里依赖已加载状态做业务判断。
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 基于已加载 authoritative state 做业务校验，可为空。
    ///
    /// 这里适合放“命令本身合法，但结合当前状态后不允许执行”的业务拒绝。
    /// 不要在这里做持久化、副作用发布，或把真正的业务推导拆到 adapter。
    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &<Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 一次确定性的业务推导，只返回带 before / after 语义的强类型领域 `Changes`。
    ///
    /// `GivenState` 表示 outbound 已加载出的 authoritative context。
    /// 默认只以只读借用形式传入 use case，不把它当作可直接消费的工作缓冲区。
    /// 如果实现内部确实需要可变工作态，应显式 `clone` 到局部变量后再重组。
    ///
    /// 这里是 use case 的业务核心：实现应显式匹配 `Command` 与 `GivenState` 分支，
    /// 调用聚合根公开业务方法，推导出本次业务目标对应的 before / after changes。
    /// 若一次业务目标需要多个聚合协作，应由 use case 在这一编排层分别驱动多个聚合根；
    /// 不要让任一聚合根或聚合成员直接访问、装载、调用或导航到其它聚合。
    /// 对同一组 `cmd + state`，结果应保持确定性。
    ///
    /// API 形状上，它表达的是：
    /// `command + loaded authoritative state -> changes`
    ///
    /// 它不直接负责落库、发事件或组装对外回复。
    fn compute_before_after_changes(
        &self,
        cmd: &Self::Command,
        state: &<Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<Self::Changes, Self::Error>;
}
