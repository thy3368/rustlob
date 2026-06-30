use crate::{CommandWithGivenState, ReplayableChanges, UpdatedEntityPair};

/// 业务命令携带发起主体的最小契约。
///
/// 这只表达“命令由哪个业务主体发出”，不承载权限判断本身。
pub trait IssuedByParty {
    fn party_id(&self) -> Option<&str> {
        None
    }
}

/// 主 MI authoritative truth 的单入口表达。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMiTruth<'a, MainMi> {
    Created(&'a MainMi),
    Updated(&'a UpdatedEntityPair<MainMi>),
}

/// stateful changes 在主 MI authoritative truth 之外，
/// 还必须显式暴露主 MI 当前状态与稳定命令分支名。
pub trait MainMiStatefulChanges: ReplayableChanges {
    type MainMi;

    /// 主 MI authoritative truth 的单入口。
    fn main_mi_truth<'a>(&'a self) -> Option<MainMiTruth<'a, Self::MainMi>> {
        None
    }

    /// 当前 changes 属于哪个稳定命令分支，例如 `place` / `cancel` / `fill`。
    fn command_kind(&self) -> &'static str;

    /// 当前主 MI authoritative truth 上显式可见的状态值。
    ///
    /// 返回 `None` 代表 changes 缺少主状态机状态，`Executor6` 必须拒绝执行。
    fn main_mi_current_state(&self) -> Option<&str>;
}

/// CommandUseCase6 把一个主 MI 状态机组收敛为三组总 enum：
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
