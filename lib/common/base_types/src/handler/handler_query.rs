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

pub trait QueryHandler: Send + Sync {
    type Query;
    type Reply;
    type StateSet;
    type Error;

    fn pre_check_command(&self, cmd: &Self::Query) -> Result<(), Self::Error>;

    fn load_state_set(&self, cmd: &Self::Query) -> Result<Self::StateSet, Self::Error>;

    fn state_set_to_reply(&self, state_changed_set: Self::StateSet) -> Self::Reply;

    fn query(&self, cmd: Self::Query) -> Result<Self::Reply, Self::Error> {
        // 1 检查命令
        // 2 查库
        // 3 转换返回值

        todo!()
    }
}
