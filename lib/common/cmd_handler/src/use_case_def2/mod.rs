mod executor;
mod outbound;
mod trace;
mod use_case;

pub use executor::{CommandUseCaseExecutionError, CommandUseCaseExecutor2};
pub use outbound::{CommandUseCaseOutbound, CommandUseCaseOutboundPhase};
pub use use_case::{
    CommandEnvelope, CommandMeta, CommandUseCase2, IssuedByParty, UseCaseReplyMapper,
};

use crate::HandlerLatencyMetrics;

/// latency 观察端口，由执行编排侧注入。
pub trait ObserveHandlerLatency: Send + Sync {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics);
}

impl ObserveHandlerLatency for () {
    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}
