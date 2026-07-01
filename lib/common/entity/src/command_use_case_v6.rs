use crate::{CommandWithGivenState, ReplayableChanges};

/// 业务命令携带发起主体的最小契约。
///
/// 这只表达“命令由哪个业务主体发出”，不承载权限判断本身。
pub trait IssuedByParty {
    fn party_id(&self) -> Option<&str> {
        None
    }
}

/// CommandUseCase6 把一个主 MI 状态机组收敛为三组总 enum：
/// - `Command`
/// - `GivenState`
/// - `Changes`
///
/// 三者分支必须在 `compute_changes(cmd, &state)` 内显式匹配，
/// 分支错配时必须返回明确业务错误。
pub trait CommandUseCase6: Send + Sync {
    type Command: IssuedByParty + CommandWithGivenState;
    type Error: std::error::Error;
    type Changes: ReplayableChanges;

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
    ///
    /// `GivenState` 表示 outbound 已加载出的 authoritative context。
    /// 默认只以只读借用形式传入 use case，不把它当作可直接消费的工作缓冲区。
    /// 如果实现内部确实需要可变工作态，应显式 `clone` 到局部变量后再重组。
    fn compute_before_after_changes(
        &self,
        cmd: &Self::Command,
        state: &<Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<Self::Changes, Self::Error>;
}
