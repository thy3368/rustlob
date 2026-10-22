#![allow(clippy::arithmetic_side_effects, clippy::print_stdout)]

use std::time::{Duration, Instant};

use common_entity::stable_string_entity_id;
use example_core_use_case::{
    ActivatePendingSpotOrderV2Input, MarketRules, PlaceOnlySpotOrderV2Cmd,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, Balance, SpotOrderSide,
    SpotOrderTif, SpotOrderType, SpotOrderV2,
};
use example_outbound_adapter::InMemoryPlaceOrderOutbound;
use use_case_executor::trading::spot::place_match_spot_order_v2_executor::execute_place_match_spot_order_v2_with_outbound;

const N: u64 = 1_000;
const ROUNDS: usize = 5;

fn sell_order(id: u64) -> Result<SpotOrderV2, Box<dyn std::error::Error>> {
    let mut order = SpotOrderV2::new_pending_limit(
        id,
        10_001,
        "seller".into(),
        "BTCUSDT".into(),
        SpotOrderSide::Sell,
        1,
        100,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        None,
        0,
        1,
    );
    order.activate_pending(ActivatePendingSpotOrderV2Input {
        base_asset_id: "BTC".into(),
        quote_asset_id: "USDT".into(),
        maker_fee_bps: 1,
        taker_fee_bps: 1,
        timestamp: 1,
    })?;
    Ok(order)
}

fn outbound() -> Result<InMemoryPlaceOrderOutbound, Box<dyn std::error::Error>> {
    let out = InMemoryPlaceOrderOutbound::seeded(
        Balance::new(
            stable_string_entity_id("buyer"),
            stable_string_entity_id("buyer"),
            stable_string_entity_id("BTC"),
            0,
            0,
            1,
        ),
        Balance::new(
            stable_string_entity_id("buyer"),
            stable_string_entity_id("buyer"),
            stable_string_entity_id("USDT"),
            200_000,
            0,
            1,
        ),
        MarketRules { symbol: "BTCUSDT".into(), min_qty: 1 },
    )?;
    out.seed_balance(Balance::new(
        stable_string_entity_id("seller"),
        stable_string_entity_id("seller"),
        stable_string_entity_id("BTC"),
        0,
        N,
        1,
    ))?;
    out.seed_balance(Balance::new(
        stable_string_entity_id("seller"),
        stable_string_entity_id("seller"),
        stable_string_entity_id("USDT"),
        N * 100,
        0,
        1,
    ))?;
    for id in 9_000..9_000 + N {
        out.seed_order(sell_order(id)?)?;
    }
    Ok(out)
}

fn command(order_id: u64) -> PlaceOnlySpotOrderV2Cmd {
    PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
        party_id: "buyer".into(),
        asset: 10_001,
        order_id,
        symbol: "BTCUSDT".into(),
        is_buy: true,
        price: "100".into(),
        size: "1".into(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".into() },
        reduce_only: false,
        cloid: None,
        base_asset_id: "BTC".into(),
        quote_asset_id: "USDT".into(),
        maker_fee_bps: 1,
        taker_fee_bps: 1,
    })
}

fn run_round() -> Result<(usize, Duration), Box<dyn std::error::Error>> {
    let out = outbound()?;
    let start = Instant::now();
    for id in 1..=N {
        let result = execute_place_match_spot_order_v2_with_outbound(
            &common_entity::LogEntrySequencer::from_metadata(id, id, 1, id, command(id)),
            &out,
        )
        .map_err(|e| format!("order {id} failed: {e:?}"))?;
        let trades = match &result.changes {
            example_core_use_case::PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
                match_changes,
                ..
            }
            | example_core_use_case::PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
                match_changes,
                ..
            } => match_changes.created_trades().len(),
            _ => 0,
        };
        if trades != 1 {
            return Err(format!("order {id} produced no single trade").into());
        }
    }
    let snapshot = out.snapshot()?;
    if snapshot.trades.len() != N as usize {
        return Err(format!("expected {N} trades, got {}", snapshot.trades.len()).into());
    }
    Ok((snapshot.trades.len(), start.elapsed()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = run_round()?; // warmup, excluded from measurements
    let mut tps = Vec::with_capacity(ROUNDS);
    for round in 1..=ROUNDS {
        let (successful, elapsed) = run_round()?;
        let rate = successful as f64 / elapsed.as_secs_f64();
        tps.push(rate);
        println!(
            "round={round} successful={successful} elapsed_ms={} tps={rate:.2}",
            elapsed.as_millis()
        );
    }
    tps.sort_by(f64::total_cmp);
    println!("median_tps={:.2} rounds={ROUNDS} orders_per_round={N}", tps[ROUNDS / 2]);
    println!("scope=single-threaded in-memory 1000-order-per-round link throughput");
    Ok(())
}
