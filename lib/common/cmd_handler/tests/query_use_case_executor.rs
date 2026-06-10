use std::fmt;
use std::sync::Mutex;

use cmd_handler::command_use_case_def2::IssuedByParty;
use cmd_handler::query_use_case_def::{
    ObserveQueryUseCaseLatency, QueryEnvelope, QueryMeta, QueryUseCase, QueryUseCaseExecutionError,
    QueryUseCaseExecutor, QueryUseCaseLatencyMetrics, QueryUseCaseOutbound,
    QueryUseCaseOutboundPhase,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubQuery {
    party_id: String,
    symbol: String,
}

impl IssuedByParty for StubQuery {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubReadModel {
    symbol_exists: bool,
    visible_to_party: bool,
    order_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubView {
    order_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubBusinessError {
    MissingPartyId,
    SymbolNotFound,
    NotVisible,
}

impl fmt::Display for StubBusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPartyId => f.write_str("missing party id"),
            Self::SymbolNotFound => f.write_str("symbol not found"),
            Self::NotVisible => f.write_str("query not visible to party"),
        }
    }
}

impl std::error::Error for StubBusinessError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubOutboundError {
    ReadReplicaUnavailable,
}

impl fmt::Display for StubOutboundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadReplicaUnavailable => f.write_str("read replica unavailable"),
        }
    }
}

impl std::error::Error for StubOutboundError {}

#[derive(Debug, Clone, Copy, Default)]
struct StubQueryUseCase;

impl QueryUseCase for StubQueryUseCase {
    type Query = StubQuery;
    type ReadModel = StubReadModel;
    type View = StubView;
    type Error = StubBusinessError;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_query(&self, query: &Self::Query) -> Result<(), Self::Error> {
        if query.party_id.trim().is_empty() {
            return Err(StubBusinessError::MissingPartyId);
        }

        Ok(())
    }

    fn validate_against_read_model(
        &self,
        _query: &Self::Query,
        read_model: &Self::ReadModel,
    ) -> Result<(), Self::Error> {
        if !read_model.symbol_exists {
            return Err(StubBusinessError::SymbolNotFound);
        }
        if !read_model.visible_to_party {
            return Err(StubBusinessError::NotVisible);
        }

        Ok(())
    }

    fn compute_view(
        &self,
        _query: &Self::Query,
        read_model: Self::ReadModel,
    ) -> Result<Self::View, Self::Error> {
        Ok(StubView { order_count: read_model.order_count })
    }
}

#[derive(Debug, Clone)]
struct StubQueryOutbound {
    result: Result<StubReadModel, StubOutboundError>,
}

impl QueryUseCaseOutbound for StubQueryOutbound {
    type Query = StubQuery;
    type ReadModel = StubReadModel;
    type Error = StubOutboundError;

    fn load_read_model(&self, _query: &Self::Query) -> Result<Self::ReadModel, Self::Error> {
        self.result.clone()
    }
}

#[derive(Debug, Default)]
struct StubObserver {
    observed: Mutex<Vec<QueryUseCaseLatencyMetrics>>,
}

impl StubObserver {
    fn observed_count(&self) -> usize {
        self.observed.lock().unwrap().len()
    }
}

impl ObserveQueryUseCaseLatency for StubObserver {
    fn observe_latency(&self, metrics: &QueryUseCaseLatencyMetrics) {
        self.observed.lock().unwrap().push(*metrics);
    }
}

fn sample_envelope(party_id: &str) -> QueryEnvelope<StubQuery> {
    QueryEnvelope {
        meta: QueryMeta { trace_id: Some("trace-1".to_string()) },
        query: StubQuery { party_id: party_id.to_string(), symbol: "BTCUSDT".to_string() },
    }
}

#[test]
fn execute_returns_view_on_happy_path() {
    let executor = QueryUseCaseExecutor;
    let use_case = StubQueryUseCase;
    let outbound = StubQueryOutbound {
        result: Ok(StubReadModel { symbol_exists: true, visible_to_party: true, order_count: 3 }),
    };
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer);

    assert_eq!(result, Ok(StubView { order_count: 3 }));
    assert_eq!(observer.observed_count(), 1);
}

#[test]
fn execute_rejects_invalid_query_before_loading_read_model() {
    let executor = QueryUseCaseExecutor;
    let use_case = StubQueryUseCase;
    let outbound = StubQueryOutbound {
        result: Ok(StubReadModel { symbol_exists: true, visible_to_party: true, order_count: 3 }),
    };
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope(""), &outbound, &observer);

    assert_eq!(
        result,
        Err(QueryUseCaseExecutionError::Business(StubBusinessError::MissingPartyId))
    );
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn execute_wraps_outbound_load_failure() {
    let executor = QueryUseCaseExecutor;
    let use_case = StubQueryUseCase;
    let outbound = StubQueryOutbound { result: Err(StubOutboundError::ReadReplicaUnavailable) };
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer);

    assert_eq!(
        result,
        Err(QueryUseCaseExecutionError::Outbound {
            phase: QueryUseCaseOutboundPhase::LoadReadModel,
            source: StubOutboundError::ReadReplicaUnavailable,
        })
    );
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn execute_rejects_invalid_read_model() {
    let executor = QueryUseCaseExecutor;
    let use_case = StubQueryUseCase;
    let outbound = StubQueryOutbound {
        result: Ok(StubReadModel { symbol_exists: true, visible_to_party: false, order_count: 3 }),
    };
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer);

    assert_eq!(result, Err(QueryUseCaseExecutionError::Business(StubBusinessError::NotVisible)));
    assert_eq!(observer.observed_count(), 0);
}
