use common_entity::{
    Entity, LogEntry, ReplayableChanges, StateMachineV2Diff, StateMachineV2,
};
use example_core_entity::{
    BalanceLedgerV2, ReservationStatus, SettlementTransferVoucher, SpotOrderSide,
    SpotOrderStatus, SpotOrderStatusReason, SpotOrderTif, SpotOrderType, SpotOrderV2, SpotTrade,
};
use example_core_use_case::{
    MatchSpotOrderV4AfterChanges, MatchSpotOrderV4Changes, MatchSpotOrderV4Cmd,
    MatchSpotOrderV4State, MatchSpotOrderV4UseCase,
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
    state: MatchSpotOrderV4State,
    command: MatchSpotOrderV4Cmd,
}

fn entry(payload: MatchSpotOrderV4Cmd) -> LogEntry<MatchSpotOrderV4Cmd> {
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

    Scenario {
        command: MatchSpotOrderV4Cmd {
            party_id: "buyer".to_owned(),
            asset: ASSET,
            order_id: TAKER_BUY_ORDER_ID,
        },
        state: MatchSpotOrderV4State {
            taker_order: taker,
            maker_orders: maker.into_iter().collect(),
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        },
    }
}

fn compute(
    scenario: Scenario,
    expected: ExpectedOutcome,
) -> (MatchSpotOrderV4State, MatchSpotOrderV4Changes) {
    let Scenario { state, command } = scenario;
    let after = MatchSpotOrderV4UseCase
        .compute_state_changed(&entry(command.clone()), &state)
        .expect("V4 match should compute after truth");
    assert_after_variant(&after, expected);
    let changes = MatchSpotOrderV4UseCase::do_compute_state_diff(state.clone(), after)
        .expect("V4 match should compute changes");
    assert_changes_variant(&changes, expected);
    assert_match_replay_contains_only_order_and_trade_events(&changes);
    (state, changes)
}

fn assert_after_variant(after: &MatchSpotOrderV4AfterChanges, expected: ExpectedOutcome) {
    let actual = match after {
        MatchSpotOrderV4AfterChanges::Resting => ExpectedOutcome::Resting,
        MatchSpotOrderV4AfterChanges::PartiallyFilled { .. } => ExpectedOutcome::PartiallyFilled,
        MatchSpotOrderV4AfterChanges::Filled { .. } => ExpectedOutcome::Filled,
        MatchSpotOrderV4AfterChanges::CanceledAfterPartialFill { .. } => {
            ExpectedOutcome::CanceledAfterPartialFill
        }
        MatchSpotOrderV4AfterChanges::Rejected { .. } => ExpectedOutcome::Rejected,
    };
    assert_eq!(actual, expected);
}

fn changes_parts(
    changes: &MatchSpotOrderV4Changes,
) -> (
    ExpectedOutcome,
    Option<&cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>>,
    &[cmd_handler::command_use_case_def2::UpdatedEntityPair<SpotOrderV2>],
    &[SpotTrade],
) {
    match changes {
        MatchSpotOrderV4Changes::Resting => (ExpectedOutcome::Resting, None, &[], &[]),
        MatchSpotOrderV4Changes::PartiallyFilled {
            updated_taker_order,
            updated_maker_orders,
            created_trades,
        } => (
            ExpectedOutcome::PartiallyFilled,
            Some(updated_taker_order),
            updated_maker_orders,
            created_trades,
        ),
        MatchSpotOrderV4Changes::Filled {
            filled_taker_order,
            updated_maker_orders,
            created_trades,
        } => (
            ExpectedOutcome::Filled,
            Some(filled_taker_order),
            updated_maker_orders,
            created_trades,
        ),
        MatchSpotOrderV4Changes::CanceledAfterPartialFill {
            canceled_taker_order,
            updated_maker_orders,
            created_trades,
        } => (
            ExpectedOutcome::CanceledAfterPartialFill,
            Some(canceled_taker_order),
            updated_maker_orders,
            created_trades,
        ),
        MatchSpotOrderV4Changes::Rejected { rejected_taker_order } => {
            (ExpectedOutcome::Rejected, Some(rejected_taker_order), &[], &[])
        }
    }
}

fn assert_changes_variant(changes: &MatchSpotOrderV4Changes, expected: ExpectedOutcome) {
    assert_eq!(changes_parts(changes).0, expected);
}

fn taker_after(changes: &MatchSpotOrderV4Changes) -> &SpotOrderV2 {
    changes.taker_order_after().expect("terminal or matched outcomes update the taker order")
}

fn assert_match_replay_contains_only_order_and_trade_events(changes: &MatchSpotOrderV4Changes) {
    let events = changes.to_replayable_events().expect("V4 match changes should replay");
    assert!(events.iter().all(|event| {
        event.entity_type == SpotOrderV2::entity_type()
            || event.entity_type == SpotTrade::entity_type()
    }));
    assert!(events.iter().all(|event| {
        event.entity_type != SettlementTransferVoucher::entity_type()
            && event.entity_type != BalanceLedgerV2::entity_type()
    }));
}

#[test]
fn given_gtc_without_liquidity_when_matched_then_order_rests_without_settlement_facts() {
    let (state, changes) = compute(scenario(2, SpotOrderTif::Gtc, None), ExpectedOutcome::Resting);

    assert_eq!(state.taker_order.status(), SpotOrderStatus::Open);
    assert_eq!(state.taker_order.filled_qty(), 0);
    assert_eq!(state.taker_order.reservation.status, ReservationStatus::Active);
    assert!(changes_parts(&changes).3.is_empty());
    assert!(changes.to_replayable_events().expect("resting should replay").is_empty());
}

#[test]
fn given_gtc_partial_liquidity_when_matched_then_only_orders_and_trade_change() {
    let (_, changes) =
        compute(scenario(2, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::PartiallyFilled);
    let order = taker_after(&changes);
    let (_, _, makers, trades) = changes_parts(&changes);

    assert_eq!(order.status(), SpotOrderStatus::PartiallyFilled);
    assert_eq!(order.filled_qty(), 1);
    assert_eq!(order.reservation.remaining_amount, order.reservation.original_amount);
    assert_eq!(makers.len(), 1);
    assert_eq!(trades.len(), 1);
}

#[test]
fn given_gtc_full_liquidity_when_matched_then_order_is_filled_without_balance_events() {
    let (_, changes) = compute(scenario(1, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::Filled);
    let order = taker_after(&changes);
    let events = changes.to_replayable_events().expect("filled should replay");

    assert_eq!(order.status(), SpotOrderStatus::Filled);
    assert_eq!(order.status_reason(), None);
    assert_eq!(order.filled_qty(), 1);
    assert!(events.iter().all(|event| event.entity_type != BalanceLedgerV2::entity_type()));
}

#[test]
fn given_ioc_without_liquidity_when_matched_then_order_is_rejected_but_not_released_yet() {
    let (_, changes) = compute(scenario(2, SpotOrderTif::Ioc, None), ExpectedOutcome::Rejected);
    let order = taker_after(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Rejected);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected));
    assert_eq!(order.reservation.status, ReservationStatus::Active);
    assert_eq!(order.fee_reservation.status, ReservationStatus::Active);
}

#[test]
fn given_ioc_partial_liquidity_when_matched_then_remainder_is_canceled_but_not_released_yet() {
    let (_, changes) =
        compute(scenario(2, SpotOrderTif::Ioc, Some(1)), ExpectedOutcome::CanceledAfterPartialFill);
    let order = taker_after(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Canceled);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::IocCancelRejected));
    assert_eq!(order.filled_qty(), 1);
    assert_eq!(order.reservation.status, ReservationStatus::Active);
}

#[test]
fn given_alo_crossing_maker_when_matched_then_order_is_bad_alo_rejected_but_not_released_yet() {
    let (_, changes) = compute(scenario(2, SpotOrderTif::Alo, Some(1)), ExpectedOutcome::Rejected);
    let order = taker_after(&changes);
    let (_, _, makers, trades) = changes_parts(&changes);

    assert_eq!(order.status(), SpotOrderStatus::Rejected);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::BadAloPxRejected));
    assert_eq!(order.filled_qty(), 0);
    assert_eq!(order.reservation.status, ReservationStatus::Active);
    assert!(makers.is_empty());
    assert!(trades.is_empty());
}
