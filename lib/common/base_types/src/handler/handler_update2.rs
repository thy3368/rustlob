#[derive(Debug, Clone, Copy, Default)]
pub struct HandlerLatencyMetrics {
    pub total_ns: u128,
    pub pre_check_ns: u128,
    pub load_state_ns: u128,
    pub validate_in_lock_ns: u128,
    pub apply_changes_ns: u128,
    pub persist_domain_events_ns: u128,
    pub replay_domain_events_ns: u128,
    pub publish_domain_events_ns: u128,
    pub domain_event_count: usize,
}

pub trait DomainEventSet {
    fn domain_event_count(&self) -> usize;
}

pub trait ApplyCommandChanges2: Send + Sync {
    type Command;
    type StateSet;
    type StateChangedSet: DomainEventSet;
    type Error;

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Self::Command,
        state_set: Self::StateSet,
    ) -> Result<Self::StateChangedSet, Self::Error>;
}

// cpu操作，如果是soa则可以simd优化
pub trait CmdHandlerForUpdate2: ApplyCommandChanges2 + Send + Sync {
    fn cmd_handle(&self, cmd: Self::Command) -> Result<Self::StateChangedSet, Self::Error> {
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

        // 三更新：执行更新计算，同时产出刷新后的状态及 domain event：cpu操作
        let apply_changes_start = std::time::Instant::now();
        let changes = self.apply_command_and_collect_changes(&cmd, state_set)?;
        let apply_changes_ns = apply_changes_start.elapsed().as_nanos();

        // 先持久化 domain event：io操作
        let persist_start = std::time::Instant::now();
        self.persist_domain_events(&changes)?;
        let persist_domain_events_ns = persist_start.elapsed().as_nanos();

        // 再回放 domain event，实现 state 更新：io操作
        let replay_start = std::time::Instant::now();
        self.replay_domain_events_to_state(&changes)?;
        let replay_domain_events_ns = replay_start.elapsed().as_nanos();

        // 最后发布 domain event：io操作
        let publish_start = std::time::Instant::now();
        self.publish_domain_events(&changes)?;
        let publish_domain_events_ns = publish_start.elapsed().as_nanos();

        let metrics = HandlerLatencyMetrics {
            total_ns: total_start.elapsed().as_nanos(),
            pre_check_ns,
            load_state_ns,
            validate_in_lock_ns,
            apply_changes_ns,
            persist_domain_events_ns,
            replay_domain_events_ns,
            publish_domain_events_ns,
            domain_event_count: changes.domain_event_count(),
        };

        self.observe_latency(&metrics);

        Ok(changes)
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error>;

    fn load_state_set_for_update(&self, cmd: &Self::Command)
    -> Result<Self::StateSet, Self::Error>;

    fn validate_command_in_lock(
        &self,
        cmd: &Self::Command,
        state_set: &Self::StateSet,
    ) -> Result<(), Self::Error>;

    fn persist_domain_events(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error>;

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error>;

    fn publish_domain_events(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error>;

    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}
