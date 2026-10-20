use std::collections::HashMap;

use common_entity::{
    Entity, LogEntry, ReplayableChanges, StateMachineV2Diff, StateMachineV2,
};
use example_core_entity::{
    Balance, BalanceLedgerV2, ReservationStatus, SettlementTransferVoucher, SpotOrderSide,
    SpotOrderStatus, SpotOrderStatusReason, SpotOrderTif, SpotOrderType, SpotOrderV2, SpotTrade,
};
use example_core_use_case::{
    MatchSpotOrderV3Changes, MatchSpotOrderV3Cmd, MatchSpotOrderV3State, MatchSpotOrderV3UseCase,
    MatchSpotOrderV4AfterChanges, MatchSpotOrderV4Changes, MatchSpotOrderV4Cmd,
    MatchSpotOrderV4State, MatchSpotOrderV4UseCase, SettleMatchedSpotTradesV4Changes,
    SettleMatchedSpotTradesV4Cmd, SettleMatchedSpotTradesV4State, SettleMatchedSpotTradesV4UseCase,
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
    v3_state: MatchSpotOrderV3State,
    v3_command: MatchSpotOrderV3Cmd,
    v4_state: MatchSpotOrderV4State,
    v4_command: MatchSpotOrderV4Cmd,
    settlement_balances: Vec<Balance>,
    fee_account_id: String,
}

struct V4Pipeline {
    match_changes: MatchSpotOrderV4Changes,
    settle_changes: SettleMatchedSpotTradesV4Changes,
}

fn entry<P>(payload: P) -> LogEntry<P> {
    common_entity::LogEntrySequencer::from_metadata(1, EXECUTION_TIME_NS, 1, 1, payload)
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
    let maker_orders = maker.into_iter().collect::<Vec<_>>();
    let balances = vec![
        Balance::new("buyer".to_owned(), "BTC".to_owned(), 0, 0, 1),
        Balance::new("buyer".to_owned(), "USDT".to_owned(), 1_000, taker_principal + taker_fee, 1),
        Balance::new("seller".to_owned(), "BTC".to_owned(), 0, maker_base, 1),
        Balance::new("seller".to_owned(), "USDT".to_owned(), 0, maker_fee, 1),
        Balance::new("fee".to_owned(), "USDT".to_owned(), 0, 0, 1),
    ];

    Scenario {
        v3_command: MatchSpotOrderV3Cmd {
            party_id: "buyer".to_owned(),
            asset: ASSET,
            order_id: TAKER_BUY_ORDER_ID,
        },
        v4_command: MatchSpotOrderV4Cmd {
            party_id: "buyer".to_owned(),
            asset: ASSET,
            order_id: TAKER_BUY_ORDER_ID,
        },
        v3_state: MatchSpotOrderV3State {
            taker_order: taker.clone(),
            maker_orders: maker_orders.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            fee_account_id: "fee".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        },
        v4_state: MatchSpotOrderV4State {
            taker_order: taker,
            maker_orders,
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        },
        settlement_balances: balances,
        fee_account_id: "fee".to_owned(),
    }
}

fn run_v4(scenario: &Scenario) -> V4Pipeline {
    let match_after = MatchSpotOrderV4UseCase
        .compute_state_changed(&entry(scenario.v4_command.clone()), &scenario.v4_state)
        .expect("V4 match should compute after truth");
    let settlement_state = settlement_state_from_match_after(scenario, &match_after);
    let match_changes =
        MatchSpotOrderV4UseCase::do_compute_state_diff(scenario.v4_state.clone(), match_after)
            .expect("V4 match should compute changes");
    let settlement_after = SettleMatchedSpotTradesV4UseCase
        .compute_state_changed(&entry(SettleMatchedSpotTradesV4Cmd::default()), &settlement_state)
        .expect("V4 settlement should compute after truth");
    let settle_changes =
        SettleMatchedSpotTradesV4UseCase::do_compute_state_diff(settlement_state, settlement_after)
            .expect("V4 settlement should compute changes");

    V4Pipeline { match_changes, settle_changes }
}

fn settlement_state_from_match_after(
    scenario: &Scenario,
    after: &MatchSpotOrderV4AfterChanges,
) -> SettleMatchedSpotTradesV4State {
    let (taker_order, maker_orders, created_trades) = match after {
        MatchSpotOrderV4AfterChanges::Resting => (
            scenario.v4_state.taker_order.clone(),
            scenario.v4_state.maker_orders.clone(),
            Vec::new(),
        ),
        MatchSpotOrderV4AfterChanges::PartiallyFilled {
            taker_order_after,
            maker_orders_after,
            created_trades,
        } => (taker_order_after.clone(), maker_orders_after.clone(), created_trades.clone()),
        MatchSpotOrderV4AfterChanges::Filled {
            filled_taker_order_after,
            maker_orders_after,
            created_trades,
        } => (filled_taker_order_after.clone(), maker_orders_after.clone(), created_trades.clone()),
        MatchSpotOrderV4AfterChanges::CanceledAfterPartialFill {
            canceled_taker_order_after,
            maker_orders_after,
            created_trades,
        } => {
            (canceled_taker_order_after.clone(), maker_orders_after.clone(), created_trades.clone())
        }
        MatchSpotOrderV4AfterChanges::Rejected { rejected_taker_order_after } => {
            (rejected_taker_order_after.clone(), scenario.v4_state.maker_orders.clone(), Vec::new())
        }
    };

    SettleMatchedSpotTradesV4State {
        taker_order,
        maker_orders,
        created_trades,
        settlement_balances: scenario.settlement_balances.clone(),
        base_asset_id: "BTC".to_owned(),
        quote_asset_id: "USDT".to_owned(),
        fee_account_id: scenario.fee_account_id.clone(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    }
}

fn settle_parts(
    changes: &SettleMatchedSpotTradesV4Changes,
) -> (
    Option<&cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>>,
    &[cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>],
    &[cmd_handler::command_use_case_def2::UpdatedEntityPair<Balance>],
    &[SettlementTransferVoucher],
    &[BalanceLedgerV2],
) {
    match changes {
        SettleMatchedSpotTradesV4Changes::Unchanged => (None, &[], &[], &[], &[]),
        SettleMatchedSpotTradesV4Changes::Settled {
            updated_taker_order,
            updated_maker_orders,
            updated_balances,
            created_vouchers,
            created_balance_ledger_entries,
        } => (
            Some(updated_taker_order),
            updated_maker_orders,
            updated_balances,
            created_vouchers,
            created_balance_ledger_entries,
        ),
    }
}

fn v3_parts(
    changes: &MatchSpotOrderV3Changes,
) -> (
    ExpectedOutcome,
    Option<&cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>>,
    &[cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>],
    &[cmd_handler::command_use_case_def2::UpdatedEntityPair<Balance>],
    &[SpotTrade],
    &[SettlementTransferVoucher],
    &[BalanceLedgerV2],
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
            filled_taker_order,
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        } => (
            ExpectedOutcome::Filled,
            Some(filled_taker_order),
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        ),
        MatchSpotOrderV3Changes::CanceledAfterPartialFill {
            canceled_taker_order,
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        } => (
            ExpectedOutcome::CanceledAfterPartialFill,
            Some(canceled_taker_order),
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        ),
        MatchSpotOrderV3Changes::Rejected {
            rejected_taker_order,
            updated_balances,
            created_balance_ledger_entries,
        } => (
            ExpectedOutcome::Rejected,
            Some(rejected_taker_order),
            &[],
            updated_balances,
            &[],
            &[],
            created_balance_ledger_entries,
        ),
    }
}

fn run_v3(scenario: &Scenario) -> MatchSpotOrderV3Changes {
    let after = MatchSpotOrderV3UseCase
        .compute_state_changed(&entry(scenario.v3_command.clone()), &scenario.v3_state)
        .expect("V3 should compute after truth");
    MatchSpotOrderV3UseCase::do_compute_state_diff(scenario.v3_state.clone(), after)
        .expect("V3 should compute changes")
}

fn assert_balance_conservation(
    before_balances: &[Balance],
    updated_balances: &[cmd_handler::command_use_case_def2::UpdatedEntityPair<Balance>],
) {
    let mut before = HashMap::<String, u64>::new();
    let mut after = HashMap::<String, u64>::new();
    for balance in before_balances {
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
fn given_match_resting_when_settled_then_no_settlement_events_are_created() {
    let pipeline = run_v4(&scenario(2, SpotOrderTif::Gtc, None));

    assert!(matches!(pipeline.match_changes, MatchSpotOrderV4Changes::Resting));
    assert!(matches!(pipeline.settle_changes, SettleMatchedSpotTradesV4Changes::Unchanged));
    assert!(
        pipeline
            .settle_changes
            .to_replayable_events()
            .expect("unchanged settlement should replay")
            .is_empty()
    );
}

#[test]
fn given_matched_trade_when_settled_then_vouchers_ledger_and_balances_are_created() {
    let scenario = scenario(1, SpotOrderTif::Gtc, Some(1));
    let pipeline = run_v4(&scenario);
    let (taker, makers, balances, vouchers, ledger_entries) =
        settle_parts(&pipeline.settle_changes);
    let taker = taker.expect("filled trade should update taker settlement state");

    assert_eq!(taker.after.status(), SpotOrderStatus::Filled);
    assert_eq!(taker.after.reservation.status, ReservationStatus::ExhaustedByConsume);
    assert_eq!(makers.len(), 1);
    assert_eq!(balances.len(), 5);
    assert_eq!(vouchers.len(), 1);
    assert_eq!(ledger_entries.len(), 8);
    assert_balance_conservation(&scenario.settlement_balances, balances);
}

#[test]
fn given_ioc_remainder_when_settled_then_remaining_freeze_is_released() {
    let scenario = scenario(2, SpotOrderTif::Ioc, Some(1));
    let pipeline = run_v4(&scenario);
    let (taker, _, balances, vouchers, ledger_entries) = settle_parts(&pipeline.settle_changes);
    let taker = taker.expect("ioc partial should update taker settlement state");

    assert_eq!(taker.after.status(), SpotOrderStatus::Canceled);
    assert_eq!(taker.after.status_reason(), Some(SpotOrderStatusReason::IocCancelRejected));
    assert_eq!(taker.after.reservation.status, ReservationStatus::ClosedMixed);
    assert_eq!(taker.after.reservation.remaining_amount, 0);
    assert_eq!(vouchers.len(), 1);
    assert_eq!(ledger_entries.len(), 9);
    assert_balance_conservation(&scenario.settlement_balances, balances);
}

#[test]
fn given_rejected_order_when_settled_then_remaining_freeze_is_released() {
    let scenario = scenario(2, SpotOrderTif::Ioc, None);
    let pipeline = run_v4(&scenario);
    let (taker, makers, balances, vouchers, ledger_entries) =
        settle_parts(&pipeline.settle_changes);
    let taker = taker.expect("rejected order should update taker settlement state");

    assert_eq!(taker.after.status(), SpotOrderStatus::Rejected);
    assert_eq!(taker.after.reservation.status, ReservationStatus::ClosedByRelease);
    assert_eq!(taker.after.fee_reservation.status, ReservationStatus::ClosedByRelease);
    assert!(makers.is_empty());
    assert_eq!(balances.len(), 1);
    assert!(vouchers.is_empty());
    assert_eq!(ledger_entries.len(), 2);
}

#[test]
fn given_settlement_changes_when_replayed_then_order_vouchers_balance_and_ledger_are_ordered() {
    let pipeline = run_v4(&scenario(1, SpotOrderTif::Gtc, Some(1)));
    let events =
        pipeline.settle_changes.to_replayable_events().expect("settlement changes should replay");

    assert_eq!(events[0].entity_type, SpotOrderV2::entity_type());
    assert!(
        events
            .iter()
            .position(|event| event.entity_type == SettlementTransferVoucher::entity_type())
            .expect("voucher event should exist")
            < events
                .iter()
                .position(|event| event.entity_type == BalanceLedgerV2::entity_type())
                .expect("ledger event should exist")
    );
    assert!(events.iter().all(|event| event.is_updated() || event.is_created()));
}

#[test]
fn v4_match_then_settle_keeps_v3_key_business_results_for_representative_outcomes() {
    let scenarios = [
        (scenario(2, SpotOrderTif::Gtc, None), ExpectedOutcome::Resting),
        (scenario(2, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::PartiallyFilled),
        (scenario(1, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::Filled),
        (scenario(2, SpotOrderTif::Ioc, Some(1)), ExpectedOutcome::CanceledAfterPartialFill),
        (scenario(2, SpotOrderTif::Ioc, None), ExpectedOutcome::Rejected),
    ];

    for (scenario, expected) in scenarios {
        let v3 = run_v3(&scenario);
        let v4 = run_v4(&scenario);
        let (v3_outcome, v3_taker, v3_makers, v3_balances, v3_trades, v3_vouchers, v3_ledger) =
            v3_parts(&v3);
        let (v4_taker_settle, v4_makers_settle, v4_balances, v4_vouchers, v4_ledger) =
            settle_parts(&v4.settle_changes);
        let v4_trade_count = v4.match_changes.created_trades().len();

        assert_eq!(v3_outcome, expected);
        assert_eq!(v4_trade_count, v3_trades.len());
        assert_eq!(v4_vouchers.len(), v3_vouchers.len());
        assert_eq!(v4_ledger.len(), v3_ledger.len());
        assert_eq!(v4_balances.len(), v3_balances.len());

        if expected == ExpectedOutcome::Resting {
            assert!(v4_taker_settle.is_none());
            continue;
        }

        let v3_taker_after = &v3_taker.expect("V3 should update taker").after;
        let v4_taker_after = &v4_taker_settle.expect("V4 settlement should update taker").after;
        assert_eq!(v4_taker_after.status(), v3_taker_after.status());
        assert_eq!(v4_taker_after.status_reason(), v3_taker_after.status_reason());
        assert_eq!(v4_taker_after.filled_qty(), v3_taker_after.filled_qty());
        assert_eq!(v4_taker_after.reservation.status, v3_taker_after.reservation.status);
        assert_eq!(v4_makers_settle.len(), v3_makers.len());
    }
}
