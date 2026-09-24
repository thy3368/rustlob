use std::collections::HashMap;

use common_entity::{
    Entity, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use example_core_entity::{
    Balance, ReservationStatus, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTif, SpotOrderType, SpotOrderV2,
};
use example_core_use_case::{
    MatchSpotOrderV3AfterChanges, MatchSpotOrderV3Changes, MatchSpotOrderV3Cmd,
    MatchSpotOrderV3State, MatchSpotOrderV3UseCase,
};

const ASSET: u32 = 10_001;
const EXECUTION_TIME_NS: u64 = 1_000_000_000;
const TAKER_BUY_ORDER_ID: u64 = 1;
const MAKER_SELL_ORDER_ID: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedOutcome {
    Resting,
    PartiallyFilled,
    Filled,
    CanceledAfterPartialFill,
    Rejected,
}

struct Scenario {
    state: MatchSpotOrderV3State,
    command: MatchSpotOrderV3Cmd,
}

fn context() -> ExecutionContext {
    ExecutionContext { execution_time_ns: EXECUTION_TIME_NS }
}

fn activated_order(
    order_id: u64,
    account_id: &str,
    side: SpotOrderSide,
    qty: u64,
    price: u64,
    tif: SpotOrderTif,
) -> SpotOrderV2 {
    let mut order = SpotOrderV2::new_pending_limit(
        order_id,
        ASSET,
        account_id.to_owned(),
        "BTCUSDT".to_owned(),
        side,
        qty,
        price,
        SpotOrderType::Limit { tif },
        None,
        1,
        1,
    );
    order
        .activate_pending(example_core_entity::ActivatePendingSpotOrderV2Input {
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
            timestamp: 2,
        })
        .expect("fixture order should activate");
    order
}

fn scenario(taker_qty: u64, taker_tif: SpotOrderTif, maker_qty: Option<u64>) -> Scenario {
    let taker =
        activated_order(TAKER_BUY_ORDER_ID, "buyer", SpotOrderSide::Buy, taker_qty, 100, taker_tif);
    let maker = maker_qty.map(|qty| {
        activated_order(
            MAKER_SELL_ORDER_ID,
            "seller",
            SpotOrderSide::Sell,
            qty,
            100,
            SpotOrderTif::Gtc,
        )
    });

    let taker_principal = taker.reservation.original_amount;
    let taker_fee = taker.fee_reservation.original_amount;
    let (maker_base, maker_fee) = maker
        .as_ref()
        .map(|order| (order.reservation.original_amount, order.fee_reservation.original_amount))
        .unwrap_or((0, 0));

    Scenario {
        command: MatchSpotOrderV3Cmd {
            party_id: "buyer".to_owned(),
            asset: ASSET,
            order_id: TAKER_BUY_ORDER_ID,
        },
        state: MatchSpotOrderV3State {
            taker_order: taker,
            maker_orders: maker.into_iter().collect(),
            settlement_balances: vec![
                Balance::new("buyer".to_owned(), "BTC".to_owned(), 0, 0, 1),
                Balance::new(
                    "buyer".to_owned(),
                    "USDT".to_owned(),
                    1_000,
                    taker_principal + taker_fee,
                    1,
                ),
                Balance::new("seller".to_owned(), "BTC".to_owned(), 0, maker_base, 1),
                Balance::new("seller".to_owned(), "USDT".to_owned(), 0, maker_fee, 1),
                Balance::new("fee".to_owned(), "USDT".to_owned(), 0, 0, 1),
            ],
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            fee_account_id: "fee".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        },
    }
}

fn compute(
    scenario: Scenario,
    expected: ExpectedOutcome,
) -> (MatchSpotOrderV3State, MatchSpotOrderV3Changes) {
    let Scenario { state, command } = scenario;
    let after = MatchSpotOrderV3UseCase
        .compute_state_changed_with_context(&command, &state, &context())
        .expect("V3 should compute after truth");
    assert_after_variant(&after, expected);
    let changes = MatchSpotOrderV3UseCase::do_compute_state_diff(state.clone(), after)
        .expect("V3 should compute changes");
    assert_changes_variant(&changes, expected);
    assert_replay_chain(&state, &changes);
    assert_balance_conservation(&state, &changes);
    (state, changes)
}

fn assert_after_variant(after: &MatchSpotOrderV3AfterChanges, expected: ExpectedOutcome) {
    let actual = match after {
        MatchSpotOrderV3AfterChanges::Resting { .. } => ExpectedOutcome::Resting,
        MatchSpotOrderV3AfterChanges::PartiallyFilled { .. } => ExpectedOutcome::PartiallyFilled,
        MatchSpotOrderV3AfterChanges::Filled { .. } => ExpectedOutcome::Filled,
        MatchSpotOrderV3AfterChanges::CanceledAfterPartialFill { .. } => {
            ExpectedOutcome::CanceledAfterPartialFill
        }
        MatchSpotOrderV3AfterChanges::Rejected { .. } => ExpectedOutcome::Rejected,
    };
    assert_eq!(actual, expected);
}

fn changes_parts(
    changes: &MatchSpotOrderV3Changes,
) -> (
    ExpectedOutcome,
    Option<&cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>>,
    &[cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>],
    &[cmd_handler::command_use_case_def2::UpdatedEntityPair<Balance>],
    &[example_core_entity::SpotTrade],
    &[example_core_entity::SettlementTransferVoucher],
    &[example_core_entity::BalanceLedgerEntryV2],
) {
    match changes {
        MatchSpotOrderV3Changes::Resting => {
            (ExpectedOutcome::Resting, None, &[], &[], &[], &[], &[])
        }
        MatchSpotOrderV3Changes::PartiallyFilled {
            updated_taker_order,
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        } => (
            ExpectedOutcome::PartiallyFilled,
            Some(updated_taker_order),
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        ),
        MatchSpotOrderV3Changes::Filled {
            filled_taker_order: updated_taker_order,
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        } => (
            ExpectedOutcome::Filled,
            Some(updated_taker_order),
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        ),
        MatchSpotOrderV3Changes::CanceledAfterPartialFill {
            canceled_taker_order: updated_taker_order,
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        } => (
            ExpectedOutcome::CanceledAfterPartialFill,
            Some(updated_taker_order),
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        ),
        MatchSpotOrderV3Changes::Rejected {
            rejected_taker_order: updated_taker_order,
            updated_balances,
            created_balance_ledger_entries,
        } => (
            ExpectedOutcome::Rejected,
            Some(updated_taker_order),
            &[],
            updated_balances,
            &[],
            &[],
            created_balance_ledger_entries,
        ),
    }
}

fn assert_changes_variant(changes: &MatchSpotOrderV3Changes, expected: ExpectedOutcome) {
    assert_eq!(changes_parts(changes).0, expected);
}

fn taker_after(changes: &MatchSpotOrderV3Changes) -> &SpotOrderV2 {
    changes.taker_order_after().expect("all terminal or resting outcomes update the taker order")
}

fn assert_replay_chain(state: &MatchSpotOrderV3State, changes: &MatchSpotOrderV3Changes) {
    let (outcome, taker, makers, balances, trades, vouchers, ledger_entries) =
        changes_parts(changes);
    let events = changes.to_replayable_events().expect("V3 changes should replay");

    if outcome == ExpectedOutcome::Resting {
        assert!(events.is_empty());
        return;
    }
    assert!(!events.is_empty());
    assert!(events.iter().all(|event| event.is_updated() || event.is_created()));
    for event in &events {
        if event.is_updated() {
            assert_eq!(event.new_version, event.old_version + 1);
        } else {
            assert_eq!(event.old_version, 0);
            assert_eq!(event.new_version, 1);
        }
    }

    if let Some(taker) = taker {
        assert_eq!(events[0].old_version, taker.before.version);
        assert_eq!(events[0].new_version, taker.after.version);
    }
    if !trades.is_empty() {
        assert!(events.iter().any(|event| {
            event.is_created() && event.entity_type == example_core_entity::SpotTrade::entity_type()
        }));
        assert_eq!(trades.len(), vouchers.len());
    }
    assert_eq!(
        events
            .iter()
            .filter(|event| event.is_created()
                && event.entity_type == example_core_entity::BalanceLedgerEntryV2::entity_type())
            .count(),
        ledger_entries.len()
    );

    for maker in makers {
        assert_eq!(maker.after.version, maker.before.version + 1);
    }
    for balance in balances {
        let ledger_count = ledger_entries
            .iter()
            .filter(|entry| entry.balance_entity_id == balance.after.entity_id().to_string())
            .count() as u64;
        assert_eq!(balance.after.version, balance.before.version + ledger_count);
    }

    if taker.is_none() {
        assert!(matches!(
            state.taker_order.status(),
            SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ));
    }
}

fn assert_balance_conservation(state: &MatchSpotOrderV3State, changes: &MatchSpotOrderV3Changes) {
    let (_, _, _, updated_balances, _, _, _) = changes_parts(changes);
    let mut before = HashMap::<String, u64>::new();
    let mut after = HashMap::<String, u64>::new();
    for balance in &state.settlement_balances {
        *before.entry(balance.asset_id.clone()).or_default() += balance.total().unwrap();
        *after.entry(balance.asset_id.clone()).or_default() += balance.total().unwrap();
    }
    for pair in updated_balances {
        let asset = pair.after.asset_id.clone();
        *after.get_mut(&asset).expect("asset should exist") -= pair.before.total().unwrap();
        *after.get_mut(&asset).expect("asset should exist") += pair.after.total().unwrap();
    }
    assert_eq!(before.get("BTC"), after.get("BTC"));
    assert_eq!(before.get("USDT"), after.get("USDT"));
}

#[test]
fn given_gtc_without_liquidity_when_matched_then_order_rests() {
    let (state, changes) = compute(scenario(2, SpotOrderTif::Gtc, None), ExpectedOutcome::Resting);
    let order = &state.taker_order;

    assert_eq!(order.status(), SpotOrderStatus::Open);
    assert_eq!(order.filled_qty(), 0);
    assert_eq!(order.reservation.status, ReservationStatus::Active);
    assert!(changes_parts(&changes).4.is_empty());
    assert!(changes_parts(&changes).5.is_empty());
    assert!(changes_parts(&changes).6.is_empty());
}

#[test]
fn given_gtc_partial_liquidity_when_matched_then_order_remains_partially_filled() {
    let (_, changes) =
        compute(scenario(2, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::PartiallyFilled);
    let order = taker_after(&changes);
    let (_, _, makers, balances, trades, vouchers, ledger_entries) = changes_parts(&changes);

    assert_eq!(order.status(), SpotOrderStatus::PartiallyFilled);
    assert_eq!(order.filled_qty(), 1);
    assert_eq!(order.reservation.remaining_amount, 100);
    assert_eq!(order.fee_reservation.remaining_amount, 0);
    assert_eq!(makers.len(), 1);
    assert_eq!(balances.len(), 5);
    assert_eq!(trades.len(), 1);
    assert_eq!(vouchers.len(), 1);
    assert_eq!(ledger_entries.len(), 8);
}

#[test]
fn given_gtc_full_liquidity_when_matched_then_order_is_filled() {
    let (_, changes) = compute(scenario(1, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::Filled);
    let order = taker_after(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Filled);
    assert_eq!(order.status_reason(), None);
    assert_eq!(order.filled_qty(), 1);
    assert_eq!(order.reservation.status, ReservationStatus::ExhaustedByConsume);
    assert_eq!(changes_parts(&changes).4.len(), 1);
    assert_eq!(changes_parts(&changes).6.len(), 8);
}

#[test]
fn given_ioc_without_liquidity_when_matched_then_order_is_rejected_and_released() {
    let (_, changes) = compute(scenario(2, SpotOrderTif::Ioc, None), ExpectedOutcome::Rejected);
    let order = taker_after(&changes);
    let (_, _, _, balances, trades, vouchers, ledger_entries) = changes_parts(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Rejected);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected));
    assert_eq!(order.reservation.status, ReservationStatus::ClosedByRelease);
    assert_eq!(order.reservation.remaining_amount, 0);
    assert_eq!(order.fee_reservation.status, ReservationStatus::ClosedByRelease);
    assert!(trades.is_empty());
    assert!(vouchers.is_empty());
    assert_eq!(balances.len(), 1);
    assert_eq!(ledger_entries.len(), 2);
}

#[test]
fn given_ioc_full_liquidity_when_matched_then_order_is_filled() {
    let (_, changes) = compute(scenario(1, SpotOrderTif::Ioc, Some(1)), ExpectedOutcome::Filled);
    let order = taker_after(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Filled);
    assert_eq!(order.status_reason(), None);
    assert_eq!(order.filled_qty(), 1);
    assert_eq!(changes_parts(&changes).4.len(), 1);
    assert_eq!(changes_parts(&changes).5.len(), 1);
    assert_eq!(changes_parts(&changes).6.len(), 8);
}

#[test]
fn given_ioc_partial_liquidity_when_matched_then_remainder_is_canceled_and_released() {
    let (_, changes) =
        compute(scenario(2, SpotOrderTif::Ioc, Some(1)), ExpectedOutcome::CanceledAfterPartialFill);
    let order = taker_after(&changes);
    let (_, _, _, balances, trades, vouchers, ledger_entries) = changes_parts(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Canceled);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::IocCancelRejected));
    assert_eq!(order.filled_qty(), 1);
    assert_eq!(order.reservation.status, ReservationStatus::ClosedMixed);
    assert_eq!(order.reservation.remaining_amount, 0);
    assert_eq!(order.fee_reservation.status, ReservationStatus::ExhaustedByConsume);
    assert_eq!(balances.len(), 5);
    assert_eq!(trades.len(), 1);
    assert_eq!(vouchers.len(), 1);
    assert_eq!(ledger_entries.len(), 9);
}

#[test]
fn given_alo_without_crossing_liquidity_when_matched_then_order_rests() {
    let (state, changes) = compute(scenario(2, SpotOrderTif::Alo, None), ExpectedOutcome::Resting);
    let order = &state.taker_order;

    assert_eq!(order.status(), SpotOrderStatus::Open);
    assert_eq!(order.filled_qty(), 0);
    assert_eq!(order.reservation.status, ReservationStatus::Active);
    assert!(changes_parts(&changes).4.is_empty());
}

#[test]
fn given_alo_crossing_maker_when_matched_then_order_is_bad_alo_rejected() {
    let (_, changes) = compute(scenario(2, SpotOrderTif::Alo, Some(1)), ExpectedOutcome::Rejected);
    let order = taker_after(&changes);
    let (_, _, makers, balances, trades, vouchers, ledger_entries) = changes_parts(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Rejected);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::BadAloPxRejected));
    assert_eq!(order.filled_qty(), 0);
    assert_eq!(order.reservation.status, ReservationStatus::ClosedByRelease);
    assert_eq!(makers.len(), 0);
    assert_eq!(balances.len(), 1);
    assert!(trades.is_empty());
    assert!(vouchers.is_empty());
    assert_eq!(ledger_entries.len(), 2);
}
