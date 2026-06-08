use cmd_handler::EntityReplayableEvent;

use super::*;

pub(crate) fn sample_state() -> PlaceConditionalOrderState {
    PlaceConditionalOrderState {
        trading_enabled: true,
        next_order_sequence: 7,
        account_id: "trader-1".to_string(),
        market_rules: MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
    }
}

pub(crate) fn sample_cmd() -> PlaceConditionalOrderCmd {
    PlaceConditionalOrderCmd {
        party_id: "trader-1".to_string(),
        asset: 10_001,
        symbol: "BTCUSDT".to_string(),
        side: PlaceOrderSide::Buy,
        quantity: 2,
        trigger_price: 90,
        trigger_role: PlaceOrderTriggerRole::StopLoss,
        execution: PlaceOrderExecution::Market { aggressive_price: 95 },
        client_order_id: None,
    }
}

pub(crate) fn event_field<'a>(
    event: &'a EntityReplayableEvent,
    field_name: &str,
) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}
