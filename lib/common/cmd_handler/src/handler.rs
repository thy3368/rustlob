use db_repo::{CmdRepo2, EventPublisher2};

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
    // fn events(&self) -> &[ChangeLog];
}

pub trait CmdHandlerInternal: Send + Sync {
    type Command;
    type Reply;
    type GivenStateSet;
    type ThenStateSet: DomainEventSet;
    type Error;

    type Repo: CmdRepo2;
    type Publisher: EventPublisher2;

    fn then(
        &self,
        cmd: &Self::Command,
        state_set: Self::GivenStateSet,
    ) -> Result<Self::ThenStateSet, Self::Error>;

    fn state_changed_set_to_reply(&self, state_changed_set: Self::ThenStateSet) -> Self::Reply;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error>;

    fn give(
        &self,
        cmd: &Self::Command,
        repo: &Self::Repo,
    ) -> Result<Self::GivenStateSet, Self::Error>;

    fn validate_command_in_lock(
        &self,
        cmd: &Self::Command,
        state_set: &Self::GivenStateSet,
    ) -> Result<(), Self::Error>;

    fn persist_domain_events(
        &self,
        domain_events: &Self::ThenStateSet,
        repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::ThenStateSet,
        repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        //todo 在这里回放
        // repo.replay_event::<SpotOrder>(&domain_events.events)
        //     .map_err(|e| SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() }))

        todo!()
    }

    fn publish_domain_events(
        &self,
        domain_events: &Self::ThenStateSet,
        publisher: Self::Publisher,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}

pub trait CmdHandlerForUpdate3: CmdHandlerInternal + Send + Sync {
    fn cmd_handle_state(
        &self,
        cmd: Self::Command,
        repo: Self::Repo,
        publisher: Self::Publisher,
    ) -> Result<Self::ThenStateSet, Self::Error> {
        use minstant::Instant;

        let total_start = Instant::now();

        let pre_check_start = Instant::now();
        self.pre_check_command(&cmd)?;
        let pre_check_ns = pre_check_start.elapsed().as_nanos();

        let load_state_start = Instant::now();
        let state_set = self.give(&cmd, &repo)?;
        let load_state_ns = load_state_start.elapsed().as_nanos();

        let validate_start = Instant::now();
        self.validate_command_in_lock(&cmd, &state_set)?;
        let validate_in_lock_ns = validate_start.elapsed().as_nanos();

        let apply_changes_start = Instant::now();
        let changes = self.then(&cmd, state_set)?;
        let apply_changes_ns = apply_changes_start.elapsed().as_nanos();

        let persist_start = Instant::now();
        self.persist_domain_events(&changes, &repo)?;
        let persist_domain_events_ns = persist_start.elapsed().as_nanos();

        let replay_start = Instant::now();
        self.replay_domain_events_to_state(&changes, &repo)?;
        let replay_domain_events_ns = replay_start.elapsed().as_nanos();

        let publish_start = Instant::now();
        self.publish_domain_events(&changes, publisher)?;
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

    fn cmd_handle(
        &self,
        cmd: Self::Command,
        repo: Self::Repo,
        publisher: Self::Publisher,
    ) -> Result<Self::Reply, Self::Error> {
        let changes = self.cmd_handle_state(cmd, repo, publisher)?;
        Ok(self.state_changed_set_to_reply(changes))
    }
}
