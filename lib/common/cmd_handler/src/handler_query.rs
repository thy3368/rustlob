use std::time::Instant;

/// 查询结果包装
pub struct QueryResult<R> {
    pub reply: R,
    pub metrics: QueryLatencyMetrics,
}

/// 查询延迟指标
#[derive(Debug, Clone, Copy, Default)]
pub struct QueryLatencyMetrics {
    pub total_ns: u128,
    pub pre_check_ns: u128,
    pub load_state_ns: u128,
    pub query_execute_ns: u128,
    pub state_to_reply_ns: u128,
    pub result_count: usize,
}

pub trait DomainEventSet {
    fn domain_event_count(&self) -> usize;
}


/// 查询处理器 trait
pub trait QueryHandler: Send + Sync {
    type Query;
    type Reply;
    type StateSet;
    type Error;

    /// 预检查命令（锁外快速失败）
    fn pre_check_command(&self, cmd: &Self::Query) -> Result<(), Self::Error>;

    /// 加载状态集合（IO操作）
    fn load_state_set(&self, cmd: &Self::Query) -> Result<Self::StateSet, Self::Error>;

    /// 状态集合转回复（CPU操作）
    fn state_set_to_reply(&self, state_set: Self::StateSet) -> Self::Reply;

    /// 执行查询
    fn query(&self, cmd: Self::Query) -> Result<Self::Reply, Self::Error> {
        let total_start = Instant::now();

        // 零预判：锁外快速失败，例如基本参数、权限、路由合法性检查
        let pre_check_start = Instant::now();
        self.pre_check_command(&cmd)?;
        let pre_check_ns = pre_check_start.elapsed().as_nanos();

        // 一查：读取查询需要的状态集合（IO操作）
        let load_state_start = Instant::now();
        let state_set = self.load_state_set(&cmd)?;
        let load_state_ns = load_state_start.elapsed().as_nanos();

        // 二转换：执行状态到回复的转换（CPU操作）
        let to_reply_start = Instant::now();
        let reply = self.state_set_to_reply(state_set);
        let state_to_reply_ns = to_reply_start.elapsed().as_nanos();

        let metrics = QueryLatencyMetrics {
            total_ns: total_start.elapsed().as_nanos(),
            pre_check_ns,
            load_state_ns,
            query_execute_ns: 0, // 查询执行与加载合并
            state_to_reply_ns,
            result_count: 0, // 可由实现类自行设置
        };

        self.observe_query_latency(&metrics);

        Ok(reply)
    }

    /// 观察查询延迟（可选实现）
    fn observe_query_latency(&self, _metrics: &QueryLatencyMetrics) {}
}
