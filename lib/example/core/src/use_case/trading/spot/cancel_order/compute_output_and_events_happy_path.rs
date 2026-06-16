use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

use super::*;
use crate::entity::{
    Balance, SpotOrder, SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTimeInForce,
};

fn cmd() -> CancelSpotOrderCmd {
    CancelSpotOrderCmd { party_id: "trader-1".to_string(), asset: 10_001, order_id: 42 }
}

fn buy_open_order() -> SpotOrder {
    SpotOrder::new(
        "42".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: 10 },
        SpotOrderTimeInForce::Gtc,
        2,
        0,
        20,
        None,
    )
}

fn sell_open_order() -> SpotOrder {
    SpotOrder::new(
        "42".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: 10 },
        SpotOrderTimeInForce::Gtc,
        2,
        2,
        0,
        None,
    )
}

fn base_balance(available: u64, frozen: u64) -> Balance {
    Balance {
        account_id: "trader-1".to_string(),
        asset_id: "BTC".to_string(),
        available,
        frozen,
        version: 3,
    }
}

fn quote_balance(available: u64, frozen: u64) -> Balance {
    Balance {
        account_id: "trader-1".to_string(),
        asset_id: "USDT".to_string(),
        available,
        frozen,
        version: 3,
    }
}

fn state(
    open_order: SpotOrder,
    base_balance: Balance,
    quote_balance: Balance,
) -> CancelSpotOrderState {
    CancelSpotOrderState {
        open_order: Some(open_order),
        account_id: "trader-1".to_string(),
        base_balance,
        quote_balance,
    }
}

fn compute_output(
    state: CancelSpotOrderState,
) -> Result<CancelSpotOrderChanges, CancelSpotOrderError> {
    CommandUseCase4::compute_changes(&CancelSpotOrderUseCase, &cmd(), state)
}

fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

fn event_field_u64(event: &EntityReplayableEvent, field_name: &str) -> Option<u64> {
    event_field(event, field_name)?.parse().ok()
}

#[test]
fn buy_order_cancel_output_releases_quote_and_emits_order_then_balance_events()
-> Result<(), CancelSpotOrderError> {
    let state = state(buy_open_order(), base_balance(5, 0), quote_balance(80, 20));

    let result = compute_output(state)?;
    let events =
        result.to_replayable_events().map_err(|_| CancelSpotOrderError::ArithmeticOverflow)?;

    assert_eq!(result.canceled_order.after.status, SpotOrderStatus::Canceled);
    assert_eq!(
        result.canceled_order.after.status_reason,
        Some(SpotOrderStatusReason::CanceledByUser)
    );
    assert_eq!(result.canceled_order.after.version, 2);
    assert_eq!(result.released_balances.len(), 1);
    assert_eq!(
        result.released_balances[0].after,
        Balance::new("trader-1".to_string(), "USDT".to_string(), 100, 0, 4)
    );
    assert_eq!(events.len(), 2);
    assert_eq!(result.released_balances.len(), events.len() - 1);
    assert!(events[0].is_updated());
    assert!(events[1].is_updated());
    assert_eq!(events[0].old_version, 1);
    assert_eq!(events[0].new_version, 2);
    assert_eq!(event_field(&events[0], "status"), Some(SpotOrderStatus::Canceled.as_str()));
    assert_eq!(
        event_field(&events[0], "status_reason"),
        Some(SpotOrderStatusReason::CanceledByUser.as_str())
    );
    assert_eq!(events[1].old_version, 3);
    assert_eq!(events[1].new_version, 4);
    assert_eq!(event_field(&events[1], "asset_id"), Some("USDT"));
    assert_eq!(event_field_u64(&events[1], "available"), Some(100));
    assert_eq!(event_field_u64(&events[1], "frozen"), Some(0));
    assert_eq!(
        event_field(&events[1], "asset_id"),
        Some(result.released_balances[0].after.asset_id.as_str())
    );
    assert_eq!(
        event_field_u64(&events[1], "available"),
        Some(result.released_balances[0].after.available)
    );
    assert_eq!(
        event_field_u64(&events[1], "frozen"),
        Some(result.released_balances[0].after.frozen)
    );

    Ok(())
}

#[test]
fn sell_order_cancel_output_releases_base_and_emits_order_then_balance_events()
-> Result<(), CancelSpotOrderError> {
    let state = state(sell_open_order(), base_balance(5, 2), quote_balance(80, 0));

    let result = compute_output(state)?;
    let events =
        result.to_replayable_events().map_err(|_| CancelSpotOrderError::ArithmeticOverflow)?;

    assert_eq!(result.canceled_order.after.status, SpotOrderStatus::Canceled);
    assert_eq!(
        result.canceled_order.after.status_reason,
        Some(SpotOrderStatusReason::CanceledByUser)
    );
    assert_eq!(result.canceled_order.after.version, 2);
    assert_eq!(result.released_balances.len(), 1);
    assert_eq!(
        result.released_balances[0].after,
        Balance::new("trader-1".to_string(), "BTC".to_string(), 7, 0, 4)
    );
    assert_eq!(events.len(), 2);
    assert_eq!(result.released_balances.len(), events.len() - 1);
    assert!(events[0].is_updated());
    assert!(events[1].is_updated());
    assert_eq!(events[0].old_version, 1);
    assert_eq!(events[0].new_version, 2);
    assert_eq!(event_field(&events[0], "status"), Some(SpotOrderStatus::Canceled.as_str()));
    assert_eq!(
        event_field(&events[0], "status_reason"),
        Some(SpotOrderStatusReason::CanceledByUser.as_str())
    );
    assert_eq!(events[1].old_version, 3);
    assert_eq!(events[1].new_version, 4);
    assert_eq!(event_field(&events[1], "asset_id"), Some("BTC"));
    assert_eq!(event_field_u64(&events[1], "available"), Some(7));
    assert_eq!(event_field_u64(&events[1], "frozen"), Some(0));
    assert_eq!(
        event_field(&events[1], "asset_id"),
        Some(result.released_balances[0].after.asset_id.as_str())
    );
    assert_eq!(
        event_field_u64(&events[1], "available"),
        Some(result.released_balances[0].after.available)
    );
    assert_eq!(
        event_field_u64(&events[1], "frozen"),
        Some(result.released_balances[0].after.frozen)
    );

    Ok(())
}
