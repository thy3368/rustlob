use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff,
    StateMachineV2Unchecked,
};
use example_core_entity::{
    ActivatePendingSpotOrderV2Input, Balance, BalanceLedgerEntryV2, SpotOrderSide, SpotOrderStatus,
    SpotOrderTif, SpotOrderType, SpotOrderV2,
};
use example_core_use_case::{
    ActivateMatchSpotOrderV2Changes, ActivateMatchSpotOrderV2Cmd, ActivateMatchSpotOrderV2Error,
    ActivateMatchSpotOrderV2State, ActivateMatchSpotOrderV2UseCase, ActivateSpotOrderV2Error,
    MatchSpotOrderV3AfterChanges, MatchSpotOrderV3Changes, MatchSpotOrderV3Error,
};

const ASSET: u32 = 10_001;
const EXECUTION_TIME_NS: u64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedOutcome {
    Resting,
    PartiallyFilled,
    Filled,
    CanceledAfterPartialFill,
    Rejected,
}

fn context() -> ExecutionContext {
    ExecutionContext { execution_time_ns: EXECUTION_TIME_NS }
}

fn cmd(order_id: &str) -> ActivateMatchSpotOrderV2Cmd {
    ActivateMatchSpotOrderV2Cmd {
        party_id: "buyer".to_owned(),
        asset: ASSET,
        order_id: order_id.to_owned(),
    }
}

fn pending_order(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    qty: u64,
    price: u64,
    tif: SpotOrderTif,
) -> SpotOrderV2 {
    SpotOrderV2::new_pending_limit(
        order_id.to_owned(),
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
    )
}

fn activated_order(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    qty: u64,
    price: u64,
    tif: SpotOrderTif,
) -> SpotOrderV2 {
    let mut order = pending_order(order_id, account_id, side, qty, price, tif);
    order
        .activate_pending(ActivatePendingSpotOrderV2Input {
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
            timestamp: 2,
        })
        .expect("fixture order should activate");
    order
}

fn scenario(
    taker_qty: u64,
    taker_tif: SpotOrderTif,
    maker_qty: Option<u64>,
) -> ActivateMatchSpotOrderV2State {
    let pending_taker =
        pending_order("taker-buy", "buyer", SpotOrderSide::Buy, taker_qty, 100, taker_tif);
    let maker = maker_qty.map(|qty| {
        activated_order("maker-sell", "seller", SpotOrderSide::Sell, qty, 100, SpotOrderTif::Gtc)
    });
    let (maker_base, maker_fee) = maker
        .as_ref()
        .map(|order| (order.reservation.original_amount, order.fee_reservation.original_amount))
        .unwrap_or((0, 0));

    ActivateMatchSpotOrderV2State {
        pending_order: pending_taker,
        maker_orders: maker.into_iter().collect(),
        settlement_balances: vec![
            Balance::new("buyer".to_owned(), "BTC".to_owned(), 0, 0, 1),
            Balance::new("buyer".to_owned(), "USDT".to_owned(), 1_000, 0, 1),
            Balance::new("seller".to_owned(), "BTC".to_owned(), 0, maker_base, 1),
            Balance::new("seller".to_owned(), "USDT".to_owned(), 0, maker_fee, 1),
            Balance::new("fee".to_owned(), "USDT".to_owned(), 0, 0, 1),
        ],
        base_asset_id: "BTC".to_owned(),
        quote_asset_id: "USDT".to_owned(),
        fee_account_id: "fee".to_owned(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    }
}

fn compute(
    state: ActivateMatchSpotOrderV2State,
    expected: ExpectedOutcome,
) -> (ActivateMatchSpotOrderV2State, ActivateMatchSpotOrderV2Changes) {
    let after = ActivateMatchSpotOrderV2UseCase
        .compute_state_changed_with_context(&cmd("taker-buy"), &state, &context())
        .expect("activate-match should compute after truth");
    assert_after_variant(&after.match_after, expected);
    let changes = ActivateMatchSpotOrderV2UseCase::do_compute_state_diff(state.clone(), after)
        .expect("activate-match should compute replayable changes");
    assert_changes_variant(&changes.match_changes, expected);
    (state, changes)
}

fn assert_after_variant(after: &MatchSpotOrderV3AfterChanges, expected: ExpectedOutcome) {
    let actual = match after {
        MatchSpotOrderV3AfterChanges::Resting => ExpectedOutcome::Resting,
        MatchSpotOrderV3AfterChanges::PartiallyFilled { .. } => ExpectedOutcome::PartiallyFilled,
        MatchSpotOrderV3AfterChanges::Filled { .. } => ExpectedOutcome::Filled,
        MatchSpotOrderV3AfterChanges::CanceledAfterPartialFill { .. } => {
            ExpectedOutcome::CanceledAfterPartialFill
        }
        MatchSpotOrderV3AfterChanges::Rejected { .. } => ExpectedOutcome::Rejected,
    };
    assert_eq!(actual, expected);
}

fn assert_changes_variant(changes: &MatchSpotOrderV3Changes, expected: ExpectedOutcome) {
    let actual = match changes {
        MatchSpotOrderV3Changes::Resting => ExpectedOutcome::Resting,
        MatchSpotOrderV3Changes::PartiallyFilled { .. } => ExpectedOutcome::PartiallyFilled,
        MatchSpotOrderV3Changes::Filled { .. } => ExpectedOutcome::Filled,
        MatchSpotOrderV3Changes::CanceledAfterPartialFill { .. } => {
            ExpectedOutcome::CanceledAfterPartialFill
        }
        MatchSpotOrderV3Changes::Rejected { .. } => ExpectedOutcome::Rejected,
    };
    assert_eq!(actual, expected);
}

fn match_taker_before(changes: &MatchSpotOrderV3Changes) -> Option<&SpotOrderV2> {
    match changes {
        MatchSpotOrderV3Changes::Resting => None,
        MatchSpotOrderV3Changes::PartiallyFilled { updated_taker_order, .. } => {
            Some(&updated_taker_order.before)
        }
        MatchSpotOrderV3Changes::Filled { filled_taker_order, .. } => {
            Some(&filled_taker_order.before)
        }
        MatchSpotOrderV3Changes::CanceledAfterPartialFill { canceled_taker_order, .. } => {
            Some(&canceled_taker_order.before)
        }
        MatchSpotOrderV3Changes::Rejected { rejected_taker_order, .. } => {
            Some(&rejected_taker_order.before)
        }
    }
}

fn event_signatures(events: &[EntityReplayableEvent]) -> Vec<(u8, u8, u64, u64)> {
    events
        .iter()
        .map(|event| (event.entity_type, event.change_type, event.old_version, event.new_version))
        .collect()
}

#[test]
fn command_rejects_empty_party_id_and_order_id() {
    let mut invalid = cmd("taker-buy");
    invalid.party_id.clear();
    assert!(matches!(
        ActivateMatchSpotOrderV2UseCase.check_command(&invalid),
        Err(ActivateMatchSpotOrderV2Error::Activation(ActivateSpotOrderV2Error::InvalidPartyId))
    ));

    let mut invalid = cmd("");
    invalid.order_id.clear();
    assert!(matches!(
        ActivateMatchSpotOrderV2UseCase.check_command(&invalid),
        Err(ActivateMatchSpotOrderV2Error::Activation(ActivateSpotOrderV2Error::InvalidOrderId))
    ));
}

#[test]
fn state_validation_reuses_activation_rules_and_rejects_empty_fee_account() {
    let mut state = scenario(1, SpotOrderTif::Gtc, None);
    state.fee_account_id.clear();

    assert_eq!(
        ActivateMatchSpotOrderV2UseCase.validate_state_given(&cmd("taker-buy"), &state),
        Err(ActivateMatchSpotOrderV2Error::Match(MatchSpotOrderV3Error::InvalidFeeAccountId))
    );

    state.fee_account_id = "fee".to_owned();
    state.pending_order.order_id = "other".to_owned();
    assert!(matches!(
        ActivateMatchSpotOrderV2UseCase.validate_state_given(&cmd("taker-buy"), &state),
        Err(ActivateMatchSpotOrderV2Error::Activation(ActivateSpotOrderV2Error::OrderIdMismatch))
    ));
}

#[test]
fn insufficient_available_balance_is_reported_by_activation_stage() {
    let mut state = scenario(1, SpotOrderTif::Gtc, None);
    let buyer_quote = state
        .settlement_balances
        .iter_mut()
        .find(|balance| balance.account_id == "buyer" && balance.asset_id == "USDT")
        .expect("fixture should include buyer quote balance");
    buyer_quote.available = 1;

    assert!(matches!(
        ActivateMatchSpotOrderV2UseCase.compute_state_changed_with_context(
            &cmd("taker-buy"),
            &state,
            &context()
        ),
        Err(ActivateMatchSpotOrderV2Error::Activation(
            ActivateSpotOrderV2Error::InsufficientAvailableBalance
        ))
    ));
}

#[test]
fn gtc_without_liquidity_rests_after_activation_and_keeps_activation_events() {
    let (state, changes) = compute(scenario(2, SpotOrderTif::Gtc, None), ExpectedOutcome::Resting);

    assert_eq!(changes.activation_changes.updated_order.before, state.pending_order);
    assert_eq!(changes.activation_changes.updated_order.after.status(), SpotOrderStatus::Open);
    assert!(matches!(changes.match_changes, MatchSpotOrderV3Changes::Resting));

    let activation_events = changes
        .activation_changes
        .to_replayable_events()
        .expect("activation events should project");
    let events = changes.to_replayable_events().expect("combined events should project");
    assert_eq!(event_signatures(&events), event_signatures(&activation_events));
    assert!(events.iter().any(|event| {
        event.is_created() && event.entity_type == BalanceLedgerEntryV2::entity_type()
    }));
}

#[test]
fn gtc_partial_liquidity_uses_activation_after_as_match_before() {
    let (_, changes) =
        compute(scenario(2, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::PartiallyFilled);

    assert_eq!(
        match_taker_before(&changes.match_changes),
        Some(&changes.activation_changes.updated_order.after)
    );
}

#[test]
fn gtc_full_liquidity_fills_and_projects_activation_before_match_events() {
    let (_, changes) = compute(scenario(1, SpotOrderTif::Gtc, Some(1)), ExpectedOutcome::Filled);

    let activation_events = changes
        .activation_changes
        .to_replayable_events()
        .expect("activation events should project");
    let match_events =
        changes.match_changes.to_replayable_events().expect("match events should project");
    let events = changes.to_replayable_events().expect("combined events should project");

    assert_eq!(
        event_signatures(&events[..activation_events.len()]),
        event_signatures(&activation_events)
    );
    assert_eq!(
        event_signatures(&events[activation_events.len()..]),
        event_signatures(&match_events)
    );
    assert_eq!(
        match_taker_before(&changes.match_changes),
        Some(&changes.activation_changes.updated_order.after)
    );
}

#[test]
fn ioc_partial_liquidity_cancels_remainder() {
    let (_, changes) =
        compute(scenario(2, SpotOrderTif::Ioc, Some(1)), ExpectedOutcome::CanceledAfterPartialFill);

    assert!(matches!(
        changes.match_changes,
        MatchSpotOrderV3Changes::CanceledAfterPartialFill { .. }
    ));
}

#[test]
fn alo_crossing_maker_is_rejected_by_v3_match_stage() {
    let (_, changes) = compute(scenario(2, SpotOrderTif::Alo, Some(1)), ExpectedOutcome::Rejected);

    assert!(matches!(changes.match_changes, MatchSpotOrderV3Changes::Rejected { .. }));
}
