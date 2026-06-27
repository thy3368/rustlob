mod executor;
mod executor5;
mod executor6;
mod group_spec;
mod outbound;
pub(crate) mod trace;
mod use_case;
mod use_case5;
mod use_case6;

pub use executor::{
    CommandUseCaseExecutionError, CommandUseCaseExecutor2, CommandUseCaseExecutor3,
    CommandUseCaseExecutor4,
};
pub use executor5::CommandUseCaseExecutor5;
pub use executor6::CommandUseCaseExecutor6;
pub use group_spec::{
    GroupBoundarySpec, MiCausalChainSpec, MiCausalPointerSpec, MiInvariantSpec, MiPredicateSpec,
    MiSpec, MiStateMachineSpec, MiStateSpec, MiStateTransitionSpec, TruthCenterSpec,
    UseCaseGroupSpec, UseCaseInGroupSpec,
};
pub use outbound::{CommandUseCaseOutbound, CommandUseCaseOutboundPhase};
pub use use_case::{
    CommandEnvelope, CommandMeta, CommandUseCase2, CommandUseCase3, CommandUseCase4,
    EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair, UseCaseChanges,
    UseCaseOutput, UseCaseReplyMapper, UseCaseReplyMapper3,
};
pub use use_case5::{CommandUseCase5, MainMiAuthoritativeTruth, MainMiChanges};
pub use use_case6::{CommandUseCase6, CommandWithGivenState, MainMiStatefulChanges};

use crate::HandlerLatencyMetrics;

/// latency 观察端口，由执行编排侧注入。
pub trait ObserveHandlerLatency: Send + Sync {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics);
}

impl ObserveHandlerLatency for () {
    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}
