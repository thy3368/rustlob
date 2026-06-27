use super::{IssuedByParty, ReplayableChanges, UpdatedEntityPair};

/// 主 MI 的 authoritative truth。
///
/// 约束：
/// - create 场景必须显式暴露 `Created`
/// - update / state-advance 场景必须显式暴露 `UpdatedEntityPair`
/// - 不允许只把主 MI 真相藏在 replayable events 投影里
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMiAuthoritativeTruth<'a, T> {
    Created(&'a T),
    Updated(&'a UpdatedEntityPair<T>),
}

/// V5 changes 的最小框架契约：
/// 在 V4 的 replayable-events 投影能力之外，
/// 还必须显式给出主 MI 的 authoritative truth。
pub trait MainMiChanges: ReplayableChanges {
    type MainMi;

    /// 返回主 MI 的唯一 authoritative truth。
    ///
    /// 返回 `None` 代表 changes 结构不完整，`Executor5` 必须拒绝执行。
    fn main_mi_truth(&self) -> Option<MainMiAuthoritativeTruth<'_, Self::MainMi>>;
}

/// V5 收敛为 changes-first + main-MI-truth-first：
/// use case 只返回强类型业务 changes，
/// 但这些 changes 必须显式保留主 MI 的 authoritative truth。
pub trait CommandUseCase5: Send + Sync {
    type Command: IssuedByParty;
    type GivenState;
    type Error: std::error::Error;
    type Changes: MainMiChanges;

    /// 当前 use case 推进的主 MI 名称。
    fn main_mi_name(&self) -> &'static str;

    /// 当前主 MI 的身份字段名。
    fn main_mi_identity_field(&self) -> &'static str;

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
