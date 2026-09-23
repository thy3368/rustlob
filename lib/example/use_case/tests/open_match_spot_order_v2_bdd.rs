use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff,
    StateMachineV2Unchecked,
};
use example_core_entity::{
    ActivatePendingSpotOrderV2Input, Balance, SpotOrderSide, SpotOrderStatus, SpotOrderTif,
    SpotOrderTriggerRole, SpotOrderType, SpotOrderV2, SpotOrderV2MatchError, SpotTrade,
    TriggerSpotOrderV2Input,
};
use example_core_use_case::{
    MatchSpotOrderV2Changes, MatchSpotOrderV2Cmd, MatchSpotOrderV2Error, MatchSpotOrderV2State,
    OpenMatchSpotOrderV2UseCase,
};

const EXECUTION_TIME_NS: u64 = 1_717_171_717_000_000_000;

fn command(order_id: &str, party_id: &str) -> MatchSpotOrderV2Cmd {
    MatchSpotOrderV2Cmd {
        party_id: party_id.to_owned(),
        asset: 10_001,
        order_id: order_id.to_owned(),
    }
}

fn pending_limit(order_id: &str, qty: u64, tif: SpotOrderTif) -> SpotOrderV2 {
    SpotOrderV2::new_pending_limit(
        order_id.to_owned(),
        10_001,
        None,
        "buyer".to_owned(),
        "BTCUSDT".to_owned(),
        SpotOrderSide::Buy,
        qty,
        100,
        SpotOrderType::Limit { tif },
        None,
        1,
        EXECUTION_TIME_NS,
    )
}

fn pending_trigger(order_id: &str) -> SpotOrderV2 {
    let mut order = SpotOrderV2::new_pending_trigger(
        order_id.to_owned(),
        10_001,
        None,
        "buyer".to_owned(),
        "BTCUSDT".to_owned(),
        SpotOrderSide::Buy,
        1,
        100,
        SpotOrderType::Trigger {
            is_market: false,
            trigger_price: 90,
            tpsl: SpotOrderTriggerRole::TakeProfit,
        },
        None,
        1,
        EXECUTION_TIME_NS,
    );
    order.reduce_only = true;
    order
}

fn active_maker(order_id: &str, qty: u64) -> SpotOrderV2 {
    SpotOrderV2::new_active(
        order_id.to_owned(),
        10_001,
        None,
        "seller".to_owned(),
        "BTCUSDT".to_owned(),
        SpotOrderSide::Sell,
        100,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        qty,
        "BTC",
        "USDT",
        5,
        10,
        None,
        EXECUTION_TIME_NS,
    )
    .expect("maker fixture should be valid")
}

fn settlement_balances() -> Vec<Balance> {
    vec![
        Balance::new("buyer".to_owned(), "USDT".to_owned(), 1_000, 0, 1),
        Balance::new("buyer".to_owned(), "BTC".to_owned(), 0, 0, 1),
        Balance::new("seller".to_owned(), "BTC".to_owned(), 0, 2, 1),
        Balance::new("seller".to_owned(), "USDT".to_owned(), 0, 2, 1),
        Balance::new("fee".to_owned(), "USDT".to_owned(), 0, 0, 1),
    ]
}

fn state(taker_order: SpotOrderV2, maker_orders: Vec<SpotOrderV2>) -> MatchSpotOrderV2State {
    MatchSpotOrderV2State {
        taker_order,
        maker_orders,
        settlement_balances: settlement_balances(),
        base_asset_id: "BTC".to_owned(),
        quote_asset_id: "USDT".to_owned(),
        fee_account_id: "fee".to_owned(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    }
}

fn compute(
    order: SpotOrderV2,
    maker_orders: Vec<SpotOrderV2>,
) -> Result<MatchSpotOrderV2Changes, MatchSpotOrderV2Error> {
    let use_case = OpenMatchSpotOrderV2UseCase;
    let cmd = command(order.order_id(), order.account_id());
    let given_state = state(order, maker_orders);
    use_case.check_command(&cmd)?;
    use_case.validate_state_given(&cmd, &given_state)?;
    use_case.compute_state_diff_with_context(
        &cmd,
        given_state,
        &ExecutionContext { execution_time_ns: EXECUTION_TIME_NS },
    )
}

fn order_events(events: &[EntityReplayableEvent]) -> Vec<&EntityReplayableEvent> {
    events.iter().filter(|event| event.entity_type == SpotOrderV2::entity_type()).collect()
}

#[test]
fn given_pending_limit_without_crossing_liquidity_when_matched_then_only_activation_is_emitted() {
    let changes = compute(pending_limit("pending-rest", 2, SpotOrderTif::Gtc), vec![])
        .expect("pending limit should enter matching");

    let activated = changes
        .activated_taker_order
        .as_ref()
        .expect("pending limit should have an activation pair");
    assert_eq!(activated.before.version, 1);
    assert_eq!(activated.after.version, 2);
    assert_eq!(activated.after.status, SpotOrderStatus::Open);
    assert!(changes.updated_taker_order.is_none());
    assert!(changes.created_trades.is_empty());

    let events = changes.to_replayable_events().expect("changes should replay");
    let order_events = order_events(&events);
    assert_eq!(order_events.len(), 1);
    assert_eq!(order_events[0].old_version, 1);
    assert_eq!(order_events[0].new_version, 2);
    assert!(order_events[0].is_updated());
}

#[test]
fn given_pending_limit_with_partial_fill_when_matched_then_activation_precedes_match_update() {
    let changes = compute(
        pending_limit("pending-partial", 2, SpotOrderTif::Gtc),
        vec![active_maker("maker-partial", 1)],
    )
    .expect("pending limit should partially match");

    let activation = changes.activated_taker_order.as_ref().expect("activation pair");
    let final_taker = changes.updated_taker_order.as_ref().expect("final taker pair");
    assert_eq!(activation.before.version, 1);
    assert_eq!(activation.after.version, 2);
    assert_eq!(final_taker.before.version, 2);
    assert_eq!(final_taker.after.version, 3);
    assert_eq!(final_taker.after.status, SpotOrderStatus::PartiallyFilled);
    assert_eq!(changes.created_trades.len(), 1);

    let events = changes.to_replayable_events().expect("changes should replay");
    assert_eq!(events[0].entity_type, SpotOrderV2::entity_type());
    assert_eq!((events[0].old_version, events[0].new_version), (1, 2));
    assert_eq!(events[1].entity_type, SpotOrderV2::entity_type());
    assert_eq!((events[1].old_version, events[1].new_version), (2, 3));
    assert_eq!(events[2].entity_type, SpotTrade::entity_type());
    assert_eq!(events[3].entity_type, SpotOrderV2::entity_type());
    assert_eq!((events[3].old_version, events[3].new_version), (1, 2));
}

#[test]
fn given_pending_ioc_with_partial_fill_when_matched_then_final_taker_is_canceled() {
    let changes = compute(
        pending_limit("pending-ioc-partial", 2, SpotOrderTif::Ioc),
        vec![active_maker("maker-ioc-partial", 1)],
    )
    .expect("pending IOC should partially match");

    let activation = changes.activated_taker_order.as_ref().expect("activation pair");
    let final_taker = changes.updated_taker_order.as_ref().expect("final taker pair");
    assert_eq!(activation.after.status, SpotOrderStatus::Open);
    assert_eq!(final_taker.before.version, 2);
    assert_eq!(final_taker.after.version, 3);
    assert_eq!(final_taker.after.status, SpotOrderStatus::Canceled);
    assert_eq!(changes.created_trades.len(), 1);

    let events = changes.to_replayable_events().expect("changes should replay");
    assert_eq!((events[0].old_version, events[0].new_version), (1, 2));
    assert_eq!((events[1].old_version, events[1].new_version), (2, 3));
    assert_eq!(events[2].entity_type, SpotTrade::entity_type());
}

#[test]
fn given_pending_limit_with_full_fill_when_matched_then_versions_replay_continuously() {
    let changes = compute(
        pending_limit("pending-filled", 2, SpotOrderTif::Gtc),
        vec![active_maker("maker-filled", 2)],
    )
    .expect("pending limit should fully match");

    let activation = changes.activated_taker_order.as_ref().expect("activation pair");
    let final_taker = changes.updated_taker_order.as_ref().expect("final taker pair");
    assert_eq!(activation.after.status, SpotOrderStatus::Open);
    assert_eq!(final_taker.before.version, 2);
    assert_eq!(final_taker.after.version, 3);
    assert_eq!(final_taker.after.status, SpotOrderStatus::Filled);

    let events = changes.to_replayable_events().expect("changes should replay");
    assert_eq!((events[0].old_version, events[0].new_version), (1, 2));
    assert_eq!((events[1].old_version, events[1].new_version), (2, 3));
    assert_eq!(events[2].entity_type, SpotTrade::entity_type());
    assert_eq!(events[3].entity_type, SpotOrderV2::entity_type());
}

#[test]
fn given_pending_trigger_when_sent_to_open_match_then_matching_is_rejected() {
    let order = pending_trigger("pending-trigger");
    let result = compute(order, vec![]);

    assert!(matches!(
        result,
        Err(MatchSpotOrderV2Error::OrderMatch(SpotOrderV2MatchError::OrderNotMatchable))
    ));
}

#[test]
fn given_triggered_order_when_sent_to_open_match_then_it_can_match_normally() {
    let mut order = pending_trigger("triggered-order");
    order
        .trigger(TriggerSpotOrderV2Input {
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
            timestamp: EXECUTION_TIME_NS,
        })
        .expect("trigger should activate the order");
    assert_eq!(order.status, SpotOrderStatus::Open);
    assert_eq!(order.version, 2);

    let changes = compute(order, vec![active_maker("maker-triggered", 1)])
        .expect("triggered order should enter matching");
    let final_taker = changes.updated_taker_order.as_ref().expect("final taker pair");
    assert_eq!(final_taker.before.version, 2);
    assert_eq!(final_taker.after.version, 3);
    assert_eq!(final_taker.after.status, SpotOrderStatus::Filled);
    assert!(changes.activated_taker_order.is_none());
}

#[test]
fn activation_input_cannot_bypass_trigger_order_semantics() {
    let mut order = pending_trigger("trigger-cannot-bypass");
    let result = order.activate_pending_limit(ActivatePendingSpotOrderV2Input {
        base_asset_id: "BTC".to_owned(),
        quote_asset_id: "USDT".to_owned(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
        timestamp: EXECUTION_TIME_NS,
    });

    assert!(result.is_err());
    assert_eq!(order.status, SpotOrderStatus::Pending);
    assert_eq!(order.version, 1);
}
