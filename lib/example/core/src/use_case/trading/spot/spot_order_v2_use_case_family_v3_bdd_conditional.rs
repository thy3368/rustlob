use common_entity::{
    MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2Unchecked, ReplayableChanges,
};

use super::*;
use crate::entity::account::balance_ledger_entry_v2::BalanceLedgerOperation;
use crate::entity::spot::spot_order_v2::test_principal_reservation;
use crate::{SpotOrderStatus, SpotOrderStatusReason};

fn conditional_cmd() -> PlaceTriggerPendingSpotOrderV2CmdV3 {
    PlaceTriggerPendingSpotOrderV2CmdV3 {
        party_id: "buyer".to_string(),
        asset: 10_001,
        is_buy: true,
        trigger_price: "95".to_string(),
        price: "100".to_string(),
        size: "2".to_string(),
        tif: "gtc".to_string(),
        trigger_role: "stop_loss".to_string(),
        cloid: Some("client-conditional-1".to_string()),
    }
}

fn conditional_order(order_id: &str, price: &str, size: &str) -> SpotOrderV2 {
    let mut cmd = conditional_cmd();
    cmd.price = price.to_string();
    cmd.size = size.to_string();
    build_place_trigger_pending_spot_order_v2_template_v3(
        &cmd,
        PlaceTriggerPendingSpotOrderV2TemplateContextV3 {
            order_id: order_id.to_string(),
            symbol: "BTCUSDT".to_string(),
        },
    )
    .unwrap()
}

fn trigger_cmd(order_id: &str) -> SpotOrderV2CommandV3 {
    SpotOrderV2CommandV3::Trigger(TriggerSpotOrderV2CmdV3 {
        party_id: "buyer".to_string(),
        asset: 10_001,
        order_id: order_id.to_string(),
    })
}

fn active_buy_order(order_id: &str) -> SpotOrderV2 {
    SpotOrderV2::place(PlaceSpotOrderV2Input {
        order_id: order_id.to_string(),
        asset: 10_001,
        account_id: "buyer".to_string(),
        symbol: "BTCUSDT".to_string(),
        side: SpotOrderSide::Buy,
        execution: SpotOrderExecution::Limit { price: 100 },
        time_in_force: SpotOrderTimeInForce::Gtc,
        qty: 2,
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        base_balance_entity_id: "buyer:BTC".to_string(),
        quote_balance_entity_id: "buyer:USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
        client_order_id: None,
    })
    .unwrap()
    .order
}

fn sell_order(order_id: &str, account_id: &str, price: u64, qty: u64) -> SpotOrderV2 {
    SpotOrderV2::new(
        order_id.to_string(),
        10_001,
        Some(price),
        account_id.to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price },
        SpotOrderTimeInForce::Gtc,
        qty,
        0,
        SpotOrderStatus::Open,
        None,
        test_principal_reservation(order_id, account_id, SpotOrderSide::Sell, qty, price),
        None,
        1,
    )
}

fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
    Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 1)
}

fn trigger_state(
    order: SpotOrderV2,
    maker_orders: Vec<SpotOrderV2>,
    settlement_balances: Vec<Balance>,
) -> SpotOrderV2GivenStateV3 {
    SpotOrderV2GivenStateV3::Trigger {
        order,
        maker_orders,
        settlement_balances,
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        fee_account_id: "fee".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    }
}

fn balance_after<'a>(
    changes: &'a TriggerSpotOrderV2ChangesV3,
    account_id: &str,
    asset_id: &str,
) -> &'a Balance {
    changes
        .updated_balances
        .iter()
        .find(|pair| pair.after.account_id == account_id && pair.after.asset_id == asset_id)
        .map(|pair| &pair.after)
        .unwrap()
}

#[test]
fn given_conditional_order_command_when_placed_then_trigger_pending_order_is_created_without_freeze_or_match()
 {
    let family = SpotOrderV2UseCaseFamilyV3;
    let cmd = conditional_cmd();
    let order_template = build_place_trigger_pending_spot_order_v2_template_v3(
        &cmd,
        PlaceTriggerPendingSpotOrderV2TemplateContextV3 {
            order_id: "conditional-1".to_string(),
            symbol: "BTCUSDT".to_string(),
        },
    )
    .unwrap();
    let state =
        SpotOrderV2GivenStateV3::PlaceTriggerPending { order_template: order_template.clone() };

    let SpotOrderV2CaseChangesV3::PlaceTriggerPending(changes) = family
        .compute_before_after_changes(&SpotOrderV2CommandV3::PlaceTriggerPending(cmd), state)
        .unwrap()
    else {
        panic!("expected place trigger pending changes");
    };

    assert_eq!(changes.created_order, order_template);
    assert!(changes.created_order.is_trigger_pending());
    assert!(changes.created_order.active_reservation().is_none());
    assert!(changes.created_order.active_fee_reservation().is_none());
    assert_eq!(changes.to_replayable_events().unwrap().len(), 1);
}

#[test]
fn given_trigger_pending_order_when_triggered_then_order_becomes_active_and_freezes_principal() {
    let family = SpotOrderV2UseCaseFamilyV3;
    let order = conditional_order("conditional-2", "100", "2");
    let balances = vec![balance("buyer", "USDT", 1000, 0), balance("buyer", "BTC", 0, 0)];
    let state = trigger_state(order.clone(), vec![], balances);

    let SpotOrderV2CaseChangesV3::Trigger(changes) =
        family.compute_before_after_changes(&trigger_cmd("conditional-2"), state).unwrap()
    else {
        panic!("expected trigger changes");
    };

    assert!(changes.updated_order.before.is_trigger_pending());
    assert!(!changes.updated_order.after.is_trigger_pending());
    assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Open);
    assert_eq!(changes.updated_order.after.status_reason(), Some(SpotOrderStatusReason::Triggered));
    assert_eq!(changes.updated_order.after.reservation.original_amount, 200);

    let buyer_quote = balance_after(&changes, "buyer", "USDT");
    assert_eq!(buyer_quote.available, 800);
    assert_eq!(buyer_quote.frozen, 200);
    assert_eq!(changes.created_balance_ledger_entries.len(), 1);
    assert_eq!(changes.created_balance_ledger_entries[0].operation, BalanceLedgerOperation::Freeze);
    assert!(changes.created_trades.is_empty());
    assert!(changes.created_vouchers.is_empty());
    assert!(!changes.to_replayable_events().unwrap().is_empty());
}

#[test]
fn given_trigger_pending_order_crossing_book_when_triggered_then_it_freezes_matches_and_settles() {
    let family = SpotOrderV2UseCaseFamilyV3;
    let order = conditional_order("conditional-3", "100", "1");
    let maker = sell_order("maker-1", "seller", 100, 1);
    let balances = vec![
        balance("buyer", "USDT", 200, 1),
        balance("buyer", "BTC", 0, 0),
        balance("seller", "BTC", 0, 1),
        balance("seller", "USDT", 0, 1),
        balance("fee", "USDT", 0, 0),
    ];
    let state = trigger_state(order, vec![maker], balances);

    let SpotOrderV2CaseChangesV3::Trigger(changes) =
        family.compute_before_after_changes(&trigger_cmd("conditional-3"), state).unwrap()
    else {
        panic!("expected trigger changes");
    };

    assert_eq!(changes.created_trades.len(), 1);
    assert_eq!(changes.created_vouchers.len(), 1);
    assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Filled);
    assert_eq!(changes.updated_order.after.filled_qty(), 1);
    assert_eq!(changes.updated_order.after.reservation.remaining_amount, 0);
    assert_eq!(changes.updated_maker_orders[0].after.status(), SpotOrderStatus::Filled);

    assert_eq!(balance_after(&changes, "buyer", "BTC").available, 1);
    assert_eq!(balance_after(&changes, "buyer", "USDT").frozen, 0);
    assert_eq!(balance_after(&changes, "seller", "USDT").available, 100);
    assert!(
        changes
            .created_balance_ledger_entries
            .iter()
            .any(|entry| entry.operation == BalanceLedgerOperation::Freeze)
    );
    assert!(!changes.to_replayable_events().unwrap().is_empty());
}

#[test]
fn given_active_order_when_trigger_command_runs_then_it_rejects_non_pending_order() {
    let family = SpotOrderV2UseCaseFamilyV3;
    let order = active_buy_order("active-buy");
    let state = trigger_state(order, vec![], vec![balance("buyer", "USDT", 1000, 200)]);

    assert_eq!(
        family.validate_against_given_state(&trigger_cmd("active-buy"), &state),
        Err(SpotOrderV2UseCaseFamilyV3Error::OrderNotTriggerPending)
    );
}

#[test]
fn given_invalid_conditional_command_when_pre_checked_then_business_error_is_returned() {
    let family = SpotOrderV2UseCaseFamilyV3;
    let mut cmd = conditional_cmd();
    cmd.trigger_price = "0".to_string();

    assert_eq!(
        family.pre_check_command(&SpotOrderV2CommandV3::PlaceTriggerPending(cmd)),
        Err(SpotOrderV2UseCaseFamilyV3Error::InvalidTriggerPrice)
    );
}
