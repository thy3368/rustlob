use common_entity::{ReplayableChanges, StateMachineOwnedV2Diff};
use example_core_entity::spot::spot_order_v2::{SpotOrderLifecycle, SpotTerminalOrderState};
use example_core_use_case::*;
use rstest::{fixture, rstest};

fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
    Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 1)
}

fn buy_limit_order(order_id: &str, price: u64, qty: u64, cloid: Option<&str>) -> SpotOrderV2 {
    SpotOrderV2::place(PlaceSpotOrderV2Input {
        order_id: order_id.to_string(),
        asset: 10_001,
        account_id: "buyer".to_string(),
        symbol: "BTCUSDT".to_string(),
        side: SpotOrderSide::Buy,
        execution: SpotOrderExecution::Limit { price },
        time_in_force: SpotOrderTif::Gtc,
        qty,
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        base_balance_entity_id: "buyer:BTC".to_string(),
        quote_balance_entity_id: "buyer:USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
        client_order_id: cloid.map(str::to_string),
    })
    .expect("valid open buy limit order fixture")
    .order
}

fn terminal_order(status: SpotOrderStatus) -> SpotOrderV2 {
    let mut order = buy_limit_order("terminal-order", 100, 2, None);
    order.status = status;
    order.lifecycle = match status {
        SpotOrderStatus::Filled => SpotOrderLifecycle::Filled(SpotTerminalOrderState {
            status,
            status_reason: Some(SpotOrderStatusReason::Filled),
            filled_qty: 2,
        }),
        SpotOrderStatus::Canceled => SpotOrderLifecycle::Canceled(SpotTerminalOrderState {
            status,
            status_reason: Some(SpotOrderStatusReason::CanceledByUser),
            filled_qty: 0,
        }),
        SpotOrderStatus::Rejected => SpotOrderLifecycle::Rejected(SpotTerminalOrderState {
            status,
            status_reason: Some(SpotOrderStatusReason::RejectedAtPlacement),
            filled_qty: 0,
        }),
        _ => panic!("terminal order fixture requires terminal status"),
    };
    order
}

#[fixture]
fn given_open_buy_limit_order() -> SpotOrderV2GivenStateV3 {
    SpotOrderV2GivenStateV3::Modify {
        order: buy_limit_order("spot-order-1", 100, 2, None),
        balances: vec![balance("buyer", "USDT", 800, 200), balance("buyer", "BTC", 0, 0)],
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    }
}

fn modify_cmd(lookup: ModifySpotOrderV2LookupV3, price: u64, size: u64) -> SpotOrderV2CommandV3 {
    SpotOrderV2CommandV3::Modify(ModifySpotOrderV2CmdV3 {
        party_id: "buyer".to_string(),
        asset: 10_001,
        lookup,
        is_buy: true,
        price: price.to_string(),
        size: size.to_string(),
        tif: "gtc".to_string(),
        cloid: None,
        trigger_price: None,
        trigger_role: None,
    })
}

fn balance_after<'a>(
    changes: &'a ModifySpotOrderV2ChangesV3,
    account_id: &str,
    asset_id: &str,
) -> &'a Balance {
    changes
        .updated_balances
        .iter()
        .find(|pair| pair.after.account_id == account_id && pair.after.asset_id == asset_id)
        .map(|pair| &pair.after)
        .expect("modified balance should be present")
}

// competitor: hyperliquid
// feature: modify
// state: RED
// target: use_case
#[rstest]
#[case(120, 3, 360)]
#[case(80, 1, 80)]
fn given_open_limit_order_when_modify_price_and_size_then_order_keeps_identity_and_reprices_reservation(
    given_open_buy_limit_order: SpotOrderV2GivenStateV3,
    #[case] new_price: u64,
    #[case] new_size: u64,
    #[case] expected_quote_hold: u64,
) {
    let family = SpotOrderV2UseCaseFamilyV3;

    let SpotOrderV2CaseChangesV3::Modify(changes) = family
        .compute_state_diff(
            &modify_cmd(ModifySpotOrderV2LookupV3::Oid(88_001), new_price, new_size),
            given_open_buy_limit_order,
        )
        .expect("open limit order should be modifiable")
    else {
        panic!("expected modify changes");
    };

    assert_eq!(changes.updated_order.before.order_id, changes.updated_order.after.order_id);
    assert_eq!(changes.updated_order.before.account_id, changes.updated_order.after.account_id);
    assert_eq!(changes.updated_order.before.asset, changes.updated_order.after.asset);
    assert_eq!(changes.updated_order.after.order_price(), new_price);
    assert_eq!(changes.updated_order.after.qty(), new_size);
    assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Open);
    assert_eq!(
        changes
            .updated_order
            .after
            .active_reservation()
            .expect("active modified order keeps principal reservation")
            .remaining_amount,
        expected_quote_hold
    );

    let buyer_quote = balance_after(&changes, "buyer", "USDT");
    assert_eq!(buyer_quote.frozen, expected_quote_hold);
    assert!(
        changes
            .created_balance_ledger_entries
            .iter()
            .any(|entry| entry.operation == BalanceLedgerOperation::Freeze
                || entry.operation == BalanceLedgerOperation::Unfreeze)
    );
    assert!(!changes.to_replayable_events().expect("modify events should project").is_empty());
}

// competitor: hyperliquid
// feature: modify
// state: RED
// target: use_case
#[test]
fn given_open_order_when_modify_lookup_is_cloid_then_same_order_is_modified() {
    let family = SpotOrderV2UseCaseFamilyV3;
    let state = SpotOrderV2GivenStateV3::Modify {
        order: buy_limit_order(
            "spot-order-with-cloid",
            100,
            2,
            Some("0x1234567890abcdef1234567890abcdef"),
        ),
        balances: vec![balance("buyer", "USDT", 800, 200)],
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    };

    let SpotOrderV2CaseChangesV3::Modify(changes) = family
        .compute_state_diff(
            &modify_cmd(
                ModifySpotOrderV2LookupV3::Cloid("0x1234567890abcdef1234567890abcdef".into()),
                90,
                2,
            ),
            state,
        )
        .expect("cloid lookup should modify the same business order")
    else {
        panic!("expected modify changes");
    };

    assert_eq!(changes.updated_order.before.order_id, changes.updated_order.after.order_id);
    assert_eq!(
        changes.updated_order.before.client_order_id,
        changes.updated_order.after.client_order_id
    );
    assert_eq!(changes.updated_order.after.order_price(), 90);
}

// competitor: hyperliquid
// feature: modify
// state: RED
// target: use_case
#[rstest]
#[case(SpotOrderStatus::Filled)]
#[case(SpotOrderStatus::Canceled)]
#[case(SpotOrderStatus::Rejected)]
fn given_terminal_order_when_modify_then_business_error_is_returned(
    #[case] status: SpotOrderStatus,
) {
    let family = SpotOrderV2UseCaseFamilyV3;
    let state = SpotOrderV2GivenStateV3::Modify {
        order: terminal_order(status),
        balances: vec![balance("buyer", "USDT", 800, 200)],
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    };

    assert_eq!(
        family.compute_state_diff(
            &modify_cmd(ModifySpotOrderV2LookupV3::Oid(88_002), 120, 3),
            state,
        ),
        Err(SpotOrderV2UseCaseFamilyV3Error::OrderNotModifiable)
    );
}

// competitor: hyperliquid
// feature: modify
// state: RED
// target: use_case
#[test]
fn given_trigger_pending_order_when_modify_trigger_price_then_pending_trigger_terms_change_without_freeze()
 {
    let family = SpotOrderV2UseCaseFamilyV3;
    let state = SpotOrderV2GivenStateV3::Modify {
        order: SpotOrderV2::new_trigger_pending(
            "trigger-pending-1".to_string(),
            10_001,
            Some(88_003),
            "buyer".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            2,
            95,
            SpotOrderTriggerRole::StopLoss,
            SpotOrderExecution::Limit { price: 100 },
            SpotOrderTif::Gtc,
            Some("0xabcdefabcdefabcdefabcdefabcdefab".to_string()),
            1,
        ),
        balances: vec![balance("buyer", "USDT", 1000, 0)],
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    };

    let SpotOrderV2CaseChangesV3::Modify(changes) = family
        .compute_state_diff(
            &SpotOrderV2CommandV3::Modify(ModifySpotOrderV2CmdV3 {
                party_id: "buyer".to_string(),
                asset: 10_001,
                lookup: ModifySpotOrderV2LookupV3::Oid(88_003),
                is_buy: true,
                price: "110".to_string(),
                size: "3".to_string(),
                tif: "gtc".to_string(),
                cloid: Some("0xabcdefabcdefabcdefabcdefabcdefab".to_string()),
                trigger_price: Some("105".to_string()),
                trigger_role: Some("stop_loss".to_string()),
            }),
            state,
        )
        .expect("trigger pending order should allow trigger term modification")
    else {
        panic!("expected modify changes");
    };

    assert!(changes.updated_order.before.is_trigger_pending());
    assert_eq!(changes.updated_order.before.status(), SpotOrderStatus::Pending);
    assert!(changes.updated_order.after.is_trigger_pending());
    assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Pending);
    assert_eq!(changes.updated_order.after.order_price(), 110);
    assert_eq!(changes.updated_order.after.qty(), 3);
    assert!(changes.updated_order.after.active_reservation().is_none());
    assert!(changes.updated_order.after.active_fee_reservation().is_none());
    assert!(changes.updated_balances.is_empty());
    assert!(changes.created_balance_ledger_entries.is_empty());
    assert!(!changes.to_replayable_events().expect("modify events should project").is_empty());
}
