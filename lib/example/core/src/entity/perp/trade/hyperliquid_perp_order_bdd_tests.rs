use common_entity::{
    AggregateRole, Entity, FieldDiff, FinancialClassification, FourColorArchetype,
};

use super::hyperliquid_perp_order::{
    HyperliquidPerpOrder, HyperliquidPerpOrderBehaviorError, HyperliquidPerpOrderExecution,
    HyperliquidPerpOrderSide, HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
    PlaceHyperliquidPerpOrderInput,
};
use crate::entity::{
    Balance, BalanceLedgerReason, Reservation, ReservationKind, ReservationMarketKind,
    ReservationStatus,
};

fn reservation(order_id: &str) -> Reservation {
    Reservation::new(
        format!("reservation:{order_id}"),
        "trader-1".to_string(),
        order_id.to_string(),
        ReservationMarketKind::Perp,
        ReservationKind::PerpOpenMargin,
        "USDC".to_string(),
        25,
    )
    .unwrap()
}

fn order() -> HyperliquidPerpOrder {
    HyperliquidPerpOrder::new(
        "order-1".to_string(),
        Some(42),
        0,
        "trader-1".to_string(),
        "BTC-PERP".to_string(),
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Limit { price: 100 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        false,
        Some("client-1".to_string()),
        reservation("order-1"),
    )
}

fn place_input() -> PlaceHyperliquidPerpOrderInput {
    PlaceHyperliquidPerpOrderInput {
        order_id: "place-1".to_string(),
        asset: 0,
        account_id: "trader-1".to_string(),
        symbol: "BTC-PERP".to_string(),
        side: HyperliquidPerpOrderSide::Buy,
        execution: HyperliquidPerpOrderExecution::Limit { price: 100 },
        time_in_force: HyperliquidPerpOrderTimeInForce::Gtc,
        qty: 3,
        reduce_only: false,
        client_order_id: Some("client-1".to_string()),
        margin_asset_id: "USDC".to_string(),
        margin_balance_entity_id: "balance-1".to_string(),
        margin_amount: 30,
        reservation_kind: ReservationKind::PerpOpenMargin,
    }
}

#[test]
fn place_creates_order_with_internal_reservation() {
    let outcome = HyperliquidPerpOrder::place(place_input()).unwrap();

    assert_eq!(outcome.order.order_id, "place-1");
    assert_eq!(outcome.order.exchange_oid, None);
    assert_eq!(outcome.order.filled_qty, 0);
    assert_eq!(outcome.order.status, HyperliquidPerpOrderStatus::Open);
    assert_eq!(outcome.order.version, 1);
    assert_eq!(outcome.order.reservation.reservation_id, "reservation:place-1");
    assert_eq!(outcome.order.reservation.owner_account_id, "trader-1");
    assert_eq!(outcome.order.reservation.caused_by_order_id, "place-1");
    assert_eq!(outcome.order.reservation.market_kind, ReservationMarketKind::Perp);
    assert_eq!(outcome.order.reservation.asset_id, "USDC");
    assert_eq!(outcome.order.reservation.reservation_kind, ReservationKind::PerpOpenMargin);
    assert_eq!(outcome.order.reservation.original_amount, 30);
    assert_eq!(outcome.order.reservation.remaining_amount, 30);
    assert_eq!(outcome.order.reservation.status, ReservationStatus::Active);
}

#[test]
fn place_derives_freeze_ledger_from_reservation() {
    let outcome = HyperliquidPerpOrder::place(place_input()).unwrap();

    assert_eq!(outcome.freeze_ledger_entry.entry_id, "balance-ledger:freeze:place-1");
    assert_eq!(outcome.freeze_ledger_entry.account_id, outcome.order.account_id);
    assert_eq!(outcome.freeze_ledger_entry.asset_id, outcome.order.reservation.asset_id);
    assert_eq!(outcome.freeze_ledger_entry.balance_entity_id, "balance-1");
    assert_eq!(outcome.freeze_ledger_entry.amount, outcome.order.reservation.original_amount);
    assert_eq!(
        outcome.freeze_ledger_entry.reason,
        BalanceLedgerReason::FreezeForOrder { order_id: "place-1".to_string() }
    );
}

#[test]
fn given_place_order_when_freeze_applies_then_available_and_withdrawable_decrease() {
    let mut balance = Balance::new("trader-1".to_string(), "USDC".to_string(), 1_000, 200, 1);
    let total_before = balance.total();
    let withdrawable_before = balance.available;

    let mut input = place_input();
    input.margin_balance_entity_id = balance.entity_id();
    let mut outcome = HyperliquidPerpOrder::place(input).unwrap();
    let reserved_amount = outcome.order.reservation.original_amount;

    outcome.freeze_ledger_entry.apply_to(&mut balance).unwrap();
    let withdrawable_after = withdrawable_before - reserved_amount;

    assert_eq!(balance.available, 970);
    assert_eq!(balance.frozen, 230);
    assert_eq!(balance.total(), total_before);
    assert_eq!(outcome.freeze_ledger_entry.before_available, Some(1_000));
    assert_eq!(outcome.freeze_ledger_entry.after_available, Some(970));
    assert_eq!(outcome.freeze_ledger_entry.before_frozen, Some(200));
    assert_eq!(outcome.freeze_ledger_entry.after_frozen, Some(230));
    assert_eq!(withdrawable_after, 970);
}

#[test]
fn place_accepts_market_and_limit_positive_prices() {
    let mut market = place_input();
    market.execution = HyperliquidPerpOrderExecution::Market { aggressive_price: 200 };
    assert!(HyperliquidPerpOrder::place(market).is_ok());

    let mut limit = place_input();
    limit.execution = HyperliquidPerpOrderExecution::Limit { price: 200 };
    assert!(HyperliquidPerpOrder::place(limit).is_ok());
}

#[test]
fn place_rejects_zero_quantity_price_and_margin() {
    let mut zero_qty = place_input();
    zero_qty.qty = 0;
    assert_eq!(
        HyperliquidPerpOrder::place(zero_qty),
        Err(HyperliquidPerpOrderBehaviorError::InvalidQuantity)
    );

    let mut zero_limit_price = place_input();
    zero_limit_price.execution = HyperliquidPerpOrderExecution::Limit { price: 0 };
    assert_eq!(
        HyperliquidPerpOrder::place(zero_limit_price),
        Err(HyperliquidPerpOrderBehaviorError::InvalidPrice)
    );

    let mut zero_market_price = place_input();
    zero_market_price.execution = HyperliquidPerpOrderExecution::Market { aggressive_price: 0 };
    assert_eq!(
        HyperliquidPerpOrder::place(zero_market_price),
        Err(HyperliquidPerpOrderBehaviorError::InvalidPrice)
    );

    let mut zero_margin = place_input();
    zero_margin.margin_amount = 0;
    assert_eq!(
        HyperliquidPerpOrder::place(zero_margin),
        Err(HyperliquidPerpOrderBehaviorError::InvalidMarginAmount)
    );
}

#[test]
fn created_field_changes_include_reservation_fields() {
    let order = order();
    let changes = order.created_field_changes();

    assert!(changes.iter().any(|change| change.field_name == "reservation_id"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_owner_account_id"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_caused_by_order_id"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_market_kind"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_kind"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_asset_id"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_original_amount"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_remaining_amount"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_status"));
}

#[test]
fn diff_detects_reservation_changes() {
    let before = order();
    let mut after = order();
    after.reservation.owner_account_id = "trader-2".to_string();
    after.reservation.caused_by_order_id = "order-2".to_string();
    after.reservation.market_kind = ReservationMarketKind::Spot;
    after.reservation.asset_id = "USDC2".to_string();
    after.reservation.reservation_kind = ReservationKind::PerpFlipNetNewMargin;
    after.reservation.original_amount = 50;
    after.reservation.remaining_amount = 40;
    after.reservation.status = ReservationStatus::ClosedMixed;

    let changes = before.diff(&after);

    assert!(changes.iter().any(|change| change.field_name == "reservation_owner_account_id"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_caused_by_order_id"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_market_kind"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_asset_id"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_kind"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_original_amount"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_remaining_amount"));
    assert!(changes.iter().any(|change| change.field_name == "reservation_status"));
}

#[test]
fn metadata_marks_order_as_moment_interval_root_business_voucher() {
    assert_eq!(HyperliquidPerpOrder::four_color_archetype(), FourColorArchetype::MomentInterval);
    assert_eq!(HyperliquidPerpOrder::aggregate_role(), AggregateRole::AggregateRoot);
    assert_eq!(
        HyperliquidPerpOrder::financial_classification(),
        FinancialClassification::BusinessVoucher
    );
}

#[test]
fn order_exposes_matching_facts() {
    let order = order();

    assert!(order.belongs_to_account("trader-1"));
    assert!(order.trades_asset(0));
    assert!(order.trades_symbol("BTC-PERP"));
    assert_eq!(order.remaining_qty(), Some(3));
    assert_eq!(order.order_price(), 100);
    assert_eq!(order.limit_price(), Some(100));
    assert!(order.is_matchable());
    assert!(order.has_consistent_execution_state());
}

#[test]
fn execution_state_detects_inconsistent_fills() {
    let order = order().with_execution_state(HyperliquidPerpOrderStatus::PartiallyFilled, 3);

    assert!(!order.has_consistent_execution_state());
    assert!(!order.is_matchable());
}

#[test]
fn liquidation_marker_defaults_and_can_be_set() {
    let normal = order();
    let liquidation = order().with_liquidation("liq-1".to_string());

    assert!(!normal.is_liquidation);
    assert_eq!(normal.liquidation_id, None);
    assert!(liquidation.is_liquidation);
    assert_eq!(liquidation.liquidation_id.as_deref(), Some("liq-1"));
}
