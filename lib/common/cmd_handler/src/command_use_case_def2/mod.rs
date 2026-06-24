mod executor;
mod group_spec;
mod outbound;
pub(crate) mod trace;
mod use_case;

pub use executor::{
    CommandUseCaseExecutionError, CommandUseCaseExecutor2, CommandUseCaseExecutor3,
    CommandUseCaseExecutor4,
};
pub use group_spec::{
    GroupBoundarySpec, MiCausalEdgeSpec, NonUseCaseItemSpec, TruthCenterSpec, UseCaseGroupSpec,
    UseCaseInGroupSpec,
};
pub use outbound::{CommandUseCaseOutbound, CommandUseCaseOutboundPhase};
pub use use_case::{
    CommandEnvelope, CommandMeta, CommandUseCase2, CommandUseCase3, CommandUseCase4,
    EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair, UseCaseChanges,
    UseCaseOutput, UseCaseReplyMapper, UseCaseReplyMapper3,
};

use crate::HandlerLatencyMetrics;

/// latency 观察端口，由执行编排侧注入。
pub trait ObserveHandlerLatency: Send + Sync {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics);
}

impl ObserveHandlerLatency for () {
    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}
