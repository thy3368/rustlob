use common_entity::{
    AggregateRole, Entity, FieldDiff, FinancialClassification, FourColorArchetype,
};

use crate::entity::option::cex::cex_option::*;

fn put_instrument() -> CexOptionInstrument {
    CexOptionInstrument::new(
        "BTC-20260828-100000-PUT".to_string(),
        "BTC".to_string(),
        "USDT".to_string(),
        "USDT".to_string(),
        1_787_865_600_000,
        100_000_000_000,
        CexOptionType::Put,
        CexOptionInstrumentStatus::Trading,
    )
}

fn call_instrument() -> CexOptionInstrument {
    CexOptionInstrument::new(
        "BTC-20260828-100000-CALL".to_string(),
        "BTC".to_string(),
        "USDT".to_string(),
        "USDT".to_string(),
        1_787_865_600_000,
        100_000_000_000,
        CexOptionType::Call,
        CexOptionInstrumentStatus::Trading,
    )
}

fn place_input(
    instrument: CexOptionInstrument,
    side: CexOptionOrderSide,
    intent: PlaceCexOptionOrderIntent,
) -> PlaceCexOptionOrderInput {
    PlaceCexOptionOrderInput {
        order_id: "option-order-1".to_string(),
        account_id: "account-1".to_string(),
        instrument_id: instrument.instrument_id.clone(),
        instrument,
        order_side: side,
        execution: CexOptionOrderExecution::Limit { price: 2_500_000 },
        time_in_force: CexOptionOrderTimeInForce::Gtc,
        quantity: 10,
        intent,
        premium_amount: Some(25_000_000),
        short_margin_amount: Some(100_000_000),
        fee_amount: None,
        client_order_id: Some("client-option-1".to_string()),
    }
}

#[test]
fn cex_option_instrument_keeps_strike_price_while_order_references_instrument_id() {
    let instrument = put_instrument();
    let outcome = CexOptionOrder::place(place_input(
        instrument.clone(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::OpenLong,
    ))
    .unwrap();

    assert_eq!(instrument.strike_price, 100_000_000_000);
    assert_eq!(outcome.order.instrument_id, "BTC-20260828-100000-PUT");
    assert!(!outcome
        .order
        .created_field_changes()
        .iter()
        .any(|change| change.field_name == "strike_price" || change.field_name == "option_type"));
}

#[test]
fn buy_put_open_long_derives_premium_hold_requirement() {
    let outcome = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::OpenLong,
    ))
    .unwrap();

    assert_eq!(outcome.order.order_side, CexOptionOrderSide::Buy);
    assert!(!outcome.order.reduce_only);
    assert_eq!(
        outcome.premium_hold,
        Some(CexOptionPremiumHoldRequirement {
            account_id: "account-1".to_string(),
            order_id: "option-order-1".to_string(),
            instrument_id: "BTC-20260828-100000-PUT".to_string(),
            asset_id: "USDT".to_string(),
            premium_price: 2_500_000,
            quantity: 10,
            premium_amount: 25_000_000,
        })
    );
    assert_eq!(outcome.short_margin_hold, None);
}

#[test]
fn sell_put_open_short_derives_short_margin_hold_requirement() {
    let outcome = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Sell,
        PlaceCexOptionOrderIntent::OpenShort,
    ))
    .unwrap();

    assert_eq!(outcome.order.order_side, CexOptionOrderSide::Sell);
    assert!(!outcome.order.reduce_only);
    assert_eq!(
        outcome.short_margin_hold,
        Some(CexOptionShortMarginHoldRequirement {
            account_id: "account-1".to_string(),
            order_id: "option-order-1".to_string(),
            instrument_id: "BTC-20260828-100000-PUT".to_string(),
            asset_id: "USDT".to_string(),
            margin_amount: 100_000_000,
        })
    );
    assert_eq!(outcome.premium_hold, None);
}

#[test]
fn buy_call_and_sell_call_orders_can_be_created() {
    let buy_call = CexOptionOrder::place(place_input(
        call_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::IncreaseLong,
    ))
    .unwrap();
    let sell_call = CexOptionOrder::place(place_input(
        call_instrument(),
        CexOptionOrderSide::Sell,
        PlaceCexOptionOrderIntent::IncreaseShort,
    ))
    .unwrap();

    assert_eq!(buy_call.order.instrument_id, "BTC-20260828-100000-CALL");
    assert_eq!(sell_call.order.instrument_id, "BTC-20260828-100000-CALL");
    assert!(buy_call.premium_hold.is_some());
    assert!(sell_call.short_margin_hold.is_some());
}

#[test]
fn place_rejects_zero_quantity() {
    let mut input =
        place_input(put_instrument(), CexOptionOrderSide::Buy, PlaceCexOptionOrderIntent::OpenLong);
    input.quantity = 0;

    assert_eq!(CexOptionOrder::place(input), Err(CexOptionOrderBehaviorError::InvalidQuantity));
}

#[test]
fn place_rejects_zero_premium_price() {
    let mut input =
        place_input(put_instrument(), CexOptionOrderSide::Buy, PlaceCexOptionOrderIntent::OpenLong);
    input.execution = CexOptionOrderExecution::Limit { price: 0 };

    assert_eq!(CexOptionOrder::place(input), Err(CexOptionOrderBehaviorError::InvalidPrice));
}

#[test]
fn remaining_quantity_and_matchable_follow_status_and_filled_quantity() {
    let order = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::OpenLong,
    ))
    .unwrap()
    .order;

    assert_eq!(order.remaining_quantity(), Some(10));
    assert!(order.is_open());
    assert!(order.is_cancelable());
    assert!(order.is_matchable());
    assert!(order.has_consistent_execution_state());

    let partial = order.clone().with_execution_state(CexOptionOrderStatus::PartiallyFilled, 4);
    assert_eq!(partial.remaining_quantity(), Some(6));
    assert!(partial.is_matchable());
    assert!(partial.has_consistent_execution_state());

    let filled = order.clone().with_execution_state(CexOptionOrderStatus::Filled, 10);
    assert_eq!(filled.remaining_quantity(), Some(0));
    assert!(filled.is_terminal());
    assert!(!filled.is_matchable());
    assert!(filled.has_consistent_execution_state());

    let inconsistent = order.with_execution_state(CexOptionOrderStatus::Open, 1);
    assert!(!inconsistent.has_consistent_execution_state());
}

#[test]
fn order_business_queries_return_owned_facts() {
    let outcome = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::CloseShort,
    ))
    .unwrap();

    assert!(outcome.order.reduce_only);
    assert!(outcome.order.belongs_to_account("account-1"));
    assert!(!outcome.order.belongs_to_account("account-2"));
    assert!(outcome.order.trades_instrument("BTC-20260828-100000-PUT"));
    assert!(!outcome.order.trades_instrument("BTC-20260828-100000-CALL"));
    assert_eq!(outcome.order.order_price(), 2_500_000);
    assert_eq!(outcome.order.limit_price(), Some(2_500_000));
    assert_eq!(outcome.premium_hold, None);
    assert_eq!(outcome.short_margin_hold, None);
}

#[test]
fn entity_metadata_matches_option_write_model_classification() {
    assert_eq!(CexOptionInstrument::four_color_archetype(), FourColorArchetype::Description);
    assert_eq!(CexOptionOrder::four_color_archetype(), FourColorArchetype::MomentInterval);
    assert_eq!(CexOptionOrder::aggregate_role(), AggregateRole::AggregateRoot);
    assert_eq!(
        CexOptionOrder::financial_classification(),
        FinancialClassification::BusinessVoucher
    );
}
