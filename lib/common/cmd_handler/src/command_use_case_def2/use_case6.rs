use super::{IssuedByParty, MainMiChanges};

/// 命令自己声明执行该命令所需的 GivenState 类型。
pub trait CommandWithGivenState {
    type GivenState;
}

/// V6 changes 在 V5 authoritative truth 之外，
/// 还必须显式暴露主 MI 当前状态与稳定命令分支名。
pub trait MainMiStatefulChanges: MainMiChanges {
    /// 当前 changes 属于哪个稳定命令分支，例如 `place` / `cancel` / `fill`。
    fn command_kind(&self) -> &'static str;

    /// 当前主 MI authoritative truth 上显式可见的状态值。
    ///
    /// 返回 `None` 代表 changes 缺少主状态机状态，`Executor6` 必须拒绝执行。
    fn main_mi_current_state(&self) -> Option<&str>;
}

/// V6 把一个主 MI 状态机组收敛为三组总 enum：
/// - `Command`
/// - `GivenState`
/// - `Changes`
///
/// 三者分支必须在 `compute_changes(cmd, state)` 内显式匹配，
/// 分支错配时必须返回明确业务错误。
pub trait CommandUseCase6: Send + Sync {
    type Command: IssuedByParty + CommandWithGivenState;
    type Error: std::error::Error;
    type Changes: MainMiStatefulChanges;

    /// 当前 use case 推进的主 MI 名称。
    fn main_mi_name(&self) -> &'static str;

    /// 当前主 MI 的身份字段名。
    fn main_mi_identity_field(&self) -> &'static str;

    /// 当前主 MI 的状态字段名，例如 `status`。
    fn main_mi_state_field(&self) -> &'static str;

    /// 对 command 的快速检查。
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 对状态校验，可为空。
    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &<Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 一次业务推导，只返回强类型领域 changes。
    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: <Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<Self::Changes, Self::Error>;
}
