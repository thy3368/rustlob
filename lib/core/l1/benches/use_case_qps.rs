use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCase2, CommandUseCaseExecutor2,
    CommandUseCaseOutbound, ObserveHandlerLatency,
};
use cmd_handler::{EntityReplayableEvent, HandlerLatencyMetrics};
use criterion::{
    BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use l1_core::{
    ChainState, IngressDecision, PendingRequest, ReceiveAndAdmitTransactionsCmd,
    ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsStateSnapshot,
    ReceiveAndAdmitTransactionsUseCase, SignedTransactionRequest, StateRoot, VmCapability, VmKind,
};

fn make_request(index: usize) -> SignedTransactionRequest {
    SignedTransactionRequest {
        trace_id: Some(format!("trace-{index}")),
        request_id: format!("req-{index}"),
        account: format!("acct-{}", index % 32),
        nonce: index.to_string(),
        expires_at: format!("2026-06-{:02}T{:02}:00:00Z", (index % 28) + 1, index % 24),
        action_type: if index % 3 == 0 { "order" } else { "cancel" }.to_string(),
        payload_hash: format!("payload-{index:08x}"),
        signature_hash: format!("sig-{index:08x}"),
    }
}

fn benchmark_command(request_count: usize) -> ReceiveAndAdmitTransactionsCmd {
    ReceiveAndAdmitTransactionsCmd {
        requests: (0..request_count).map(make_request).collect(),
    }
}

fn request_is_admitted(request: &SignedTransactionRequest) -> bool {
    request
        .nonce
        .chars()
        .last()
        .is_some_and(|digit| matches!(digit, '0' | '2' | '4' | '6' | '8'))
}

fn ingress_decision_from_request(request: &SignedTransactionRequest) -> IngressDecision {
    if request_is_admitted(request) { IngressDecision::Admit } else { IngressDecision::Reject }
}

fn pending_request_from_request(request: &SignedTransactionRequest) -> PendingRequest {
    PendingRequest {
        trace_id: request.trace_id.clone(),
        request_id: request.request_id.clone(),
        performer: request.account.clone(),
        vm_kind: if request.signature_hash.len() % 2 == 0 {
            VmKind::RustVm
        } else {
            VmKind::Evm
        },
        capability: VmCapability::new(format!("ingress.{}", request.action_type)),
        action_type: request.action_type.clone(),
        payload_hash: request.payload_hash.clone(),
        payload: Some(format!("nonce:{}|sig:{}", request.nonce, request.signature_hash)),
    }
}

fn benchmark_state(
    cmd: &ReceiveAndAdmitTransactionsCmd,
) -> ReceiveAndAdmitTransactionsStateSnapshot {
    let ingress_decisions = cmd
        .requests
        .iter()
        .map(ingress_decision_from_request)
        .collect::<Vec<_>>();
    let admitted_requests = cmd
        .requests
        .iter()
        .filter(|request| request_is_admitted(request))
        .map(pending_request_from_request)
        .collect::<Vec<_>>();

    ReceiveAndAdmitTransactionsStateSnapshot {
        chain_state: ChainState {
            height: cmd.requests.len() as u64,
            state_root: StateRoot(format!("root-{}", cmd.requests.len())),
        },
        admitted_requests,
        ingress_decisions,
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct BenchmarkOutbound;

impl
    CommandUseCaseOutbound<
        ReceiveAndAdmitTransactionsCmd,
        ReceiveAndAdmitTransactionsStateSnapshot,
        ReceiveAndAdmitTransactionsError,
    > for BenchmarkOutbound
{
    fn load_state(
        &self,
        cmd: &ReceiveAndAdmitTransactionsCmd,
    ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError> {
        Ok(benchmark_state(cmd))
    }

    fn persist(
        &self,
        _events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        Ok(())
    }

    fn replay(
        &self,
        _events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        Ok(())
    }

    fn publish(
        &self,
        _events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
struct RecordingLatencyObserver {
    samples: AtomicUsize,
    total_latency_ns: AtomicU64,
    total_domain_event_count: AtomicUsize,
}

impl RecordingLatencyObserver {
    fn snapshot(&self) -> (usize, u64, usize) {
        (
            self.samples.load(Ordering::Relaxed),
            self.total_latency_ns.load(Ordering::Relaxed),
            self.total_domain_event_count.load(Ordering::Relaxed),
        )
    }
}

impl ObserveHandlerLatency for RecordingLatencyObserver {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics) {
        self.samples.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ns.fetch_add(metrics.total_ns as u64, Ordering::Relaxed);
        self.total_domain_event_count.fetch_add(metrics.domain_event_count, Ordering::Relaxed);
    }
}

fn benchmark_receive_and_admit_transactions(c: &mut Criterion) {
    let request_counts = [1usize, 8, 64, 256];
    let use_case = ReceiveAndAdmitTransactionsUseCase;
    let executor = CommandUseCaseExecutor2;
    let outbound = BenchmarkOutbound;
    let observer = RecordingLatencyObserver::default();

    let mut group = c.benchmark_group("receive_and_admit_transactions");
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(100);

    for request_count in request_counts {
        let command = benchmark_command(request_count);
        let state = benchmark_state(&command);

        group.throughput(Throughput::Elements(request_count as u64));
        group.bench_function(BenchmarkId::new("compute_replayable_events", request_count), |b| {
            b.iter(|| {
                let events = use_case
                    .compute_replayable_events(black_box(&command), black_box(state.clone()))
                    .expect("benchmark state should be valid");
                black_box(events);
            });
        });

        group.bench_function(BenchmarkId::new("executor.execute", request_count), |b| {
            b.iter(|| {
                let events = executor
                    .execute(
                        &use_case,
                        black_box(CommandEnvelope {
                            meta: CommandMeta::default(),
                            command: command.clone(),
                        }),
                        black_box(&outbound),
                        black_box(&observer),
                    )
                    .expect("benchmark command should be valid");
                black_box(events);
            });
        });
    }

    black_box(observer.snapshot());
    group.finish();
}

criterion_group!(benches, benchmark_receive_and_admit_transactions);
criterion_main!(benches);
