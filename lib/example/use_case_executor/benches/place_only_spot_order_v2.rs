#![allow(
    clippy::arithmetic_side_effects,
    clippy::expect_used,
    clippy::unwrap_used,
    reason = "benchmark setup and measurement code intentionally keeps the hot loop minimal."
)]

use common_entity::{EntityReplayableEvent, LogEntry, LogEntrySequencer};
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use example_core_use_case::{
    PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
};
use example_outbound_adapter::NoopSpotOrderV2PlaceOnlyOutbound;
use use_case_executor::trading::spot::place_only_spot_order_v2_executor::{
    compute_place_only_spot_order_v2_business, execute_place_only_spot_order_v2_with_outbound,
};

fn limit_command(order_id: u64) -> PlaceOnlySpotOrderV2Cmd {
    PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
        party_id: "bench-trader".to_string(),
        asset: 10_001,
        order_id,
        symbol: "BTCUSDT".to_string(),
        is_buy: true,
        price: "100".to_string(),
        size: "2".to_string(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "gtc".to_string() },
        reduce_only: false,
        cloid: Some("bench-cloid".to_string()),
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 1,
        taker_fee_bps: 5,
    })
}

fn set_order_id(command: &mut PlaceOnlySpotOrderV2Cmd, order_id: u64) {
    let PlaceOnlySpotOrderV2Cmd::Single(order) = command else {
        return;
    };
    order.order_id = order_id;
}

fn log_entry(command: PlaceOnlySpotOrderV2Cmd, sequence: u64) -> LogEntry<PlaceOnlySpotOrderV2Cmd> {
    LogEntrySequencer::from_metadata(sequence, sequence, 1, sequence, command)
}

fn black_box_events(result: Result<(impl Sized, Vec<EntityReplayableEvent>), impl Sized>) {
    let _ = black_box(result);
}

fn benchmark_place_only_spot_order_v2(c: &mut Criterion) {
    let mut group = c.benchmark_group("place_only_spot_order_v2");
    group.throughput(Throughput::Elements(1));

    let mut business_command = limit_command(1);
    let mut business_order_id = 1;
    group.bench_function("business_hot_path", |b| {
        b.iter(|| {
            set_order_id(&mut business_command, business_order_id);
            business_order_id = business_order_id.saturating_add(1);
            let entry = log_entry(business_command.clone(), business_order_id);
            black_box_events(compute_place_only_spot_order_v2_business(black_box(&entry)));
        });
    });

    let outbound = NoopSpotOrderV2PlaceOnlyOutbound;
    let mut executor_command = limit_command(1);
    let mut executor_order_id = 1;
    group.bench_function("full_executor_noop_outbound", |b| {
        b.iter(|| {
            set_order_id(&mut executor_command, executor_order_id);
            executor_order_id = executor_order_id.saturating_add(1);
            let entry = log_entry(executor_command.clone(), executor_order_id);
            let _ = black_box(execute_place_only_spot_order_v2_with_outbound(
                black_box(&entry),
                &outbound,
            ));
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_place_only_spot_order_v2);
criterion_main!(benches);
