/// 查询型 use case 的阶段延迟指标。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QueryUseCaseLatencyMetrics {
    pub total_ns: u128,
    pub pre_check_ns: u128,
    pub load_read_model_ns: u128,
    pub validate_against_read_model_ns: u128,
    pub compute_view_ns: u128,
}

/// 查询型 use case latency 观察端口，由执行编排侧注入。
pub trait ObserveQueryUseCaseLatency: Send + Sync {
    fn observe_latency(&self, metrics: &QueryUseCaseLatencyMetrics);
}

impl ObserveQueryUseCaseLatency for () {
    fn observe_latency(&self, _metrics: &QueryUseCaseLatencyMetrics) {}
}
