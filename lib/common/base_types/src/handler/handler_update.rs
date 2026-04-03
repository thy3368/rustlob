pub struct ChangeSet<W, L> {
    pub writes: W,
    pub changelogs: Vec<L>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HandlerLatencyMetrics {
    pub total_ns: u128,
    pub pre_check_ns: u128,
    pub load_state_ns: u128,
    pub validate_in_lock_ns: u128,
    pub apply_changes_ns: u128,
    pub persist_changelogs_ns: u128,
    pub replay_changelogs_ns: u128,
    pub publish_changelog_ns: u128,
    pub changelog_count: usize,
}

// cpu操作，如果是soa则可以simd优化
pub trait CmdHandlerForUpdate<C, S, W, L, E>: Send + Sync {
    fn cmd_handle<R, F>(&self, cmd: C, result_mapper: F) -> Result<R, E>
    where
        F: FnOnce(&W, &[L]) -> R,
    {
        let total_start = std::time::Instant::now();

        // 零预判：锁外快速失败，例如基本参数、时间窗、路由合法性检查:cpu操作
        let pre_check_start = std::time::Instant::now();
        self.pre_check_command(&cmd)?;
        let pre_check_ns = pre_check_start.elapsed().as_nanos();

        // 一锁：读取并锁定本次更新需要参与计算的状态集合：io操作
        let load_state_start = std::time::Instant::now();
        let state_set = self.load_state_set_for_update(&cmd)?;
        let load_state_ns = load_state_start.elapsed().as_nanos();

        // 二判：锁内再次确认当前状态仍满足更新条件：cpu操作
        let validate_start = std::time::Instant::now();
        self.validate_command_in_lock(&cmd, &state_set)?;
        let validate_in_lock_ns = validate_start.elapsed().as_nanos();

        // 三更新：执行更新计算，同时产出写集及 changelog：cpu操作
        let apply_changes_start = std::time::Instant::now();
        let changes = self.apply_command_and_collect_changes(&cmd, state_set)?;
        let apply_changes_ns = apply_changes_start.elapsed().as_nanos();

        // 先持久化 changelog：io操作
        let persist_start = std::time::Instant::now();
        self.persist_changelogs(&changes.changelogs)?;
        let persist_changelogs_ns = persist_start.elapsed().as_nanos();

        // 再回放 changelog，实现 state 更新：io操作
        let replay_start = std::time::Instant::now();
        self.replay_changelogs_to_state(&changes.changelogs)?;
        let replay_changelogs_ns = replay_start.elapsed().as_nanos();

        // 最后发布 changelog / event：io操作
        let publish_start = std::time::Instant::now();
        self.publish_changelog(&changes.changelogs)?;
        let publish_changelog_ns = publish_start.elapsed().as_nanos();

        let metrics = HandlerLatencyMetrics {
            total_ns: total_start.elapsed().as_nanos(),
            pre_check_ns,
            load_state_ns,
            validate_in_lock_ns,
            apply_changes_ns,
            persist_changelogs_ns,
            replay_changelogs_ns,
            publish_changelog_ns,
            changelog_count: changes.changelogs.len(),
        };

        self.observe_latency(&metrics);

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

    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}
