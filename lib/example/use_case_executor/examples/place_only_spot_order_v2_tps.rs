#![allow(
    clippy::arithmetic_side_effects,
    clippy::print_stdout,
    reason = "this release example is an explicit command-line measurement tool."
)]

use std::hint::black_box;
use std::time::{Duration, Instant};

use common_entity::LogEntry;

fn entry(payload: PlaceOnlySpotOrderV2Cmd, lsn: u64) -> LogEntry<PlaceOnlySpotOrderV2Cmd> {
    LogEntry { lsn, logical_ts: lsn, client_id: 1, request_id: lsn, payload }
}

use example_core_use_case::{
    PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
};
use example_outbound_adapter::NoopSpotOrderV2PlaceOnlyOutbound;
use use_case_executor::trading::spot::place_only_spot_order_v2_executor::{
    compute_place_only_spot_order_v2_business, execute_place_only_spot_order_v2_with_outbound,
};

const DEFAULT_WARMUP_SECONDS: u64 = 1;
const DEFAULT_MEASUREMENT_SECONDS: u64 = 10;

#[derive(Debug, Clone, Copy)]
struct RunStats {
    successful: u64,
    elapsed: Duration,
}

impl RunStats {
    fn tps(self) -> f64 {
        self.successful as f64 / self.elapsed.as_secs_f64()
    }

    fn average_ns(self) -> f64 {
        self.elapsed.as_nanos() as f64 / self.successful as f64
    }
}

fn env_seconds(name: &str, default: u64) -> Duration {
    let seconds = std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|seconds| *seconds > 0)
        .unwrap_or(default);
    Duration::from_secs(seconds)
}

fn limit_command(order_id: u64) -> PlaceOnlySpotOrderV2Cmd {
    PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
        party_id: "tps-trader".to_string(),
        asset: 10_001,
        order_id,
        symbol: "BTCUSDT".to_string(),
        is_buy: true,
        price: "100".to_string(),
        size: "2".to_string(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "gtc".to_string() },
        reduce_only: false,
        cloid: Some("tps-cloid".to_string()),
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

fn run_business(warmup: Duration, measurement: Duration) -> RunStats {
    let mut command = limit_command(1);
    let mut order_id = 1;
    let warmup_start = Instant::now();
    while warmup_start.elapsed() < warmup {
        set_order_id(&mut command, order_id);
        order_id = order_id.saturating_add(1);
        let _ = black_box(compute_place_only_spot_order_v2_business(black_box(&entry(
            command.clone(),
            order_id,
        ))));
    }

    let start = Instant::now();
    let mut successful: u64 = 0;
    while start.elapsed() < measurement {
        set_order_id(&mut command, order_id);
        order_id = order_id.saturating_add(1);
        let result =
            compute_place_only_spot_order_v2_business(black_box(&entry(command.clone(), order_id)));
        if result.is_ok() {
            successful = successful.saturating_add(1);
        }
        let _ = black_box(result);
    }

    RunStats { successful, elapsed: start.elapsed() }
}

fn run_executor(warmup: Duration, measurement: Duration) -> RunStats {
    let outbound = NoopSpotOrderV2PlaceOnlyOutbound;
    let mut command = limit_command(1);
    let mut order_id = 1;
    let warmup_start = Instant::now();
    while warmup_start.elapsed() < warmup {
        set_order_id(&mut command, order_id);
        order_id = order_id.saturating_add(1);
        let _ = black_box(execute_place_only_spot_order_v2_with_outbound(
            black_box(&entry(command.clone(), order_id)),
            &outbound,
        ));
    }

    let start = Instant::now();
    let mut successful: u64 = 0;
    while start.elapsed() < measurement {
        set_order_id(&mut command, order_id);
        order_id = order_id.saturating_add(1);
        let result = execute_place_only_spot_order_v2_with_outbound(
            black_box(&entry(command.clone(), order_id)),
            &outbound,
        );
        if result.is_ok() {
            successful = successful.saturating_add(1);
        }
        let _ = black_box(result);
    }

    RunStats { successful, elapsed: start.elapsed() }
}

fn main() {
    let warmup = env_seconds("PLACE_ONLY_SPOT_ORDER_V2_TPS_WARMUP_SECONDS", DEFAULT_WARMUP_SECONDS);
    let measurement =
        env_seconds("PLACE_ONLY_SPOT_ORDER_V2_TPS_SECONDS", DEFAULT_MEASUREMENT_SECONDS);
    let business = run_business(warmup, measurement);
    let executor = run_executor(warmup, measurement);

    println!("place-only spot order v2 single-limit TPS");
    println!("warmup_seconds={}", warmup.as_secs());
    println!("measurement_seconds={}", measurement.as_secs());
    println!(
        "business_hot_path: successful={}, elapsed_ms={}, tps={:.2}, average_ns={:.2}",
        business.successful,
        business.elapsed.as_millis(),
        business.tps(),
        business.average_ns()
    );
    println!(
        "full_executor_noop_outbound: successful={}, elapsed_ms={}, tps={:.2}, average_ns={:.2}",
        executor.successful,
        executor.elapsed.as_millis(),
        executor.tps(),
        executor.average_ns()
    );
}
