use super::trace::{
    trace_field_or_placeholder, trace_phase, trace_query_use_case_completed,
    trace_query_use_case_failed, trace_query_use_case_started, use_case_query_summary,
};
use super::{
    QueryEnvelope, QueryMeta, QueryUseCase, QueryUseCaseOutbound, QueryUseCaseOutboundPhase,
};
use crate::HandlerLatencyMetrics;
use crate::use_case_def2::{IssuedByParty, ObserveHandlerLatency};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum QueryUseCaseExecutionError<BusinessError, OutboundError>
where
    BusinessError: std::error::Error + 'static,
    OutboundError: std::error::Error + 'static,
{
    #[error(transparent)]
    Business(#[from] BusinessError),
    #[error("outbound {phase} failed: {source}")]
    Outbound {
        phase: QueryUseCaseOutboundPhase,
        #[source]
        source: OutboundError,
    },
}

impl<BusinessError, OutboundError> QueryUseCaseExecutionError<BusinessError, OutboundError>
where
    BusinessError: std::error::Error + 'static,
    OutboundError: std::error::Error + 'static,
{
    pub fn outbound(
        phase: QueryUseCaseOutboundPhase,
        source: OutboundError,
    ) -> QueryUseCaseExecutionError<BusinessError, OutboundError> {
        Self::Outbound { phase, source }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryUseCaseExecutor;

impl QueryUseCaseExecutor {
    fn trace_span<U, O>(use_case: &U, meta: &QueryMeta, query: &U::Query) -> tracing::Span
    where
        U: QueryUseCase,
        O: ?Sized + Send + Sync + QueryUseCaseOutbound<Query = U::Query, ReadModel = U::ReadModel>,
        O::Error: 'static,
    {
        tracing::span!(
            tracing::Level::TRACE,
            "query_use_case_execute",
            use_case = std::any::type_name::<U>(),
            query_summary = ?use_case_query_summary::<U>(),
            role = use_case.role(),
            query_type = std::any::type_name::<U::Query>(),
            business_error_type = std::any::type_name::<U::Error>(),
            outbound_error_type = std::any::type_name::<O::Error>(),
            outbound = std::any::type_name::<O>(),
            trace_id = trace_field_or_placeholder(meta.trace_id.as_deref()),
            party_id = trace_field_or_placeholder(query.party_id()),
        )
    }

    /// 执行查询型 use case 的标准编排：
    /// 1. 先做 query 级别的快速预检查
    /// 2. 通过外部 load port 加载当前 read model
    /// 3. 基于 read model 做业务校验
    /// 4. 计算业务 view
    /// 5. 最后把整条链路的 latency 交给外部 observer
    pub fn execute<U, OB, O>(
        &self,
        use_case: &U,
        envelope: QueryEnvelope<U::Query>,
        outbound: &OB,
        latency_observer: &O,
    ) -> Result<U::View, QueryUseCaseExecutionError<U::Error, OB::Error>>
    where
        U: QueryUseCase,
        OB: ?Sized + Send + Sync + QueryUseCaseOutbound<Query = U::Query, ReadModel = U::ReadModel>,
        O: ?Sized + ObserveHandlerLatency,
        OB::Error: 'static,
    {
        use minstant::Instant;

        let QueryEnvelope { meta, query } = envelope;
        let query_summary = use_case_query_summary::<U>();
        let role = use_case.role().to_string();
        let party_id = query.party_id().map(str::to_string);
        let outbound_type = std::any::type_name::<OB>().to_string();
        let total_start = Instant::now();
        let execution_span = Self::trace_span::<U, OB>(use_case, &meta, &query);
        let _execution_guard = execution_span.enter();

        trace_query_use_case_started!();

        let execution = (|| -> Result<
            (U::View, HandlerLatencyMetrics),
            QueryUseCaseExecutionError<U::Error, OB::Error>,
        > {
            let ((), pre_check_ns) = trace_phase(
                "pre_check",
                "workflow.pre_check_query(&query)",
                || use_case.pre_check_query(&query),
            )
            .map_err(QueryUseCaseExecutionError::Business)?;
            let (read_model, load_read_model_ns) = trace_phase(
                "load_read_model",
                "outbound.load_read_model(&query)",
                || outbound.load_read_model(&query),
            )
            .map_err(|error| {
                QueryUseCaseExecutionError::outbound(
                    QueryUseCaseOutboundPhase::LoadReadModel,
                    error,
                )
            })?;
            let ((), validate_in_lock_ns) = trace_phase(
                "validate_against_read_model",
                "workflow.validate_against_read_model(&query, &read_model)",
                || use_case.validate_against_read_model(&query, &read_model),
            )
            .map_err(QueryUseCaseExecutionError::Business)?;
            let (view, compute_view_ns) = trace_phase(
                "compute_view",
                "workflow.compute_view(&query, read_model)",
                || use_case.compute_view(&query, read_model),
            )
            .map_err(QueryUseCaseExecutionError::Business)?;

            let metrics = HandlerLatencyMetrics {
                total_ns: total_start.elapsed().as_nanos(),
                pre_check_ns,
                load_state_ns: load_read_model_ns,
                validate_in_lock_ns,
                apply_changes_ns: compute_view_ns,
                persist_domain_events_ns: 0,
                replay_domain_events_ns: 0,
                publish_domain_events_ns: 0,
                domain_event_count: 0,
            };

            Ok((view, metrics))
        })();

        match execution {
            Ok((view, metrics)) => {
                trace_query_use_case_completed!(
                    query_summary,
                    role,
                    party_id,
                    outbound_type,
                    metrics
                );
                latency_observer.observe_latency(&metrics);
                Ok(view)
            }
            Err(error) => {
                trace_query_use_case_failed!(
                    query_summary,
                    role,
                    party_id,
                    outbound_type,
                    total_start.elapsed().as_nanos(),
                    error
                );
                Err(error)
            }
        }
    }
}
