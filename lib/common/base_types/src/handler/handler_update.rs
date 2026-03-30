pub struct ChangeSet<W, L> {
    pub writes: W,
    pub changelogs: Vec<L>,
}

pub trait CmdHandlerForUpdate<C, S, W, L, R, E>: Send + Sync {
    fn cmd_handle<F>(&self, cmd: C, result_mapper: F) -> Result<R, E>
    where
        F: FnOnce(&W, &[L]) -> R,
    {
        // 零预判：锁外快速失败，例如基本参数、时间窗、路由合法性检查
        self.pre_check_command(&cmd)?;

        // 一锁：读取并锁定本次更新需要参与计算的状态集合
        let state_set = self.load_state_set_for_update(&cmd)?;

        // 二判：锁内再次确认当前状态仍满足更新条件
        self.validate_command_in_lock(&cmd, &state_set)?;

        // 三更新：执行更新计算，同时产出写集及 changelog
        let changes = self.apply_command_and_collect_changes(&cmd, state_set)?;

        // 先持久化 changelog
        self.persist_changelogs(&changes.changelogs)?;

        // 再回放 changelog，实现 state 更新
        self.replay_changelogs_to_state(&changes.changelogs)?;

        // 最后发布 changelog / event
        self.publish_changelog(&changes.changelogs)?;

        Ok(result_mapper(&changes.writes, &changes.changelogs))
    }

    fn pre_check_command(&self, cmd: &C) -> Result<(), E>;

    fn load_state_set_for_update(&self, cmd: &C) -> Result<S, E>;

    fn validate_command_in_lock(&self, cmd: &C, state_set: &S) -> Result<(), E>;

    fn apply_command_and_collect_changes(
        &self,
        cmd: &C,
        state_set: S,
    ) -> Result<ChangeSet<W, L>, E>;

    fn persist_changelogs(&self, changelogs: &[L]) -> Result<(), E>;

    fn replay_changelogs_to_state(&self, changelogs: &[L]) -> Result<(), E>;

    fn publish_changelog(&self, changelogs: &[L]) -> Result<(), E>;
}
