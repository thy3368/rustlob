use cmd_handler::EntityReplayableEvent;

use super::*;

pub(crate) fn sample_state() -> PlaceImmediateOrderState {
    PlaceImmediateOrderState {
        trading_enabled: true,
        next_order_sequence: 7,
        account_id: "trader-1".to_string(),
        base_balance: Balance {
            account_id: "trader-1".to_string(),
            asset_id: "BTC".to_string(),
            available: 1_000,
            frozen: 0,
            entry_notional: None,
            identifier: None,
            version: 3,
        },
        quote_balance: Balance {
            account_id: "trader-1".to_string(),
            asset_id: "USDT".to_string(),
            available: 1_000,
            frozen: 0,
            entry_notional: None,
            identifier: None,
            version: 3,
        },
        market_rules: MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
    }
}

pub(crate) fn sample_cmd() -> PlaceImmediateOrderCmd {
    PlaceImmediateOrderCmd {
        party_id: "trader-1".to_string(),
        asset: 10_001,
        symbol: "BTCUSDT".to_string(),
        is_buy: true,
        size: 2,
        reduce_only: false,
        execution: PlaceImmediateOrderExecution::Limit {
            price: 100,
            time_in_force: PlaceOrderTimeInForce::Gtc,
        },
        cloid: None,
    }
}

pub(crate) fn cmd_with_price_and_size(price: u64, size: u64) -> PlaceImmediateOrderCmd {
    PlaceImmediateOrderCmd {
        size,
        execution: PlaceImmediateOrderExecution::Limit {
            price,
            time_in_force: PlaceOrderTimeInForce::Gtc,
        },
        ..sample_cmd()
    }
}

pub(crate) fn market_cmd_with_price_and_size(price: u64, size: u64) -> PlaceImmediateOrderCmd {
    PlaceImmediateOrderCmd {
        size,
        execution: PlaceImmediateOrderExecution::Market { aggressive_price: price },
        ..sample_cmd()
    }
}

pub(crate) fn state_with_balances(
    available_quote: u64,
    frozen_quote: u64,
    version: u64,
) -> PlaceImmediateOrderState {
    let mut state = sample_state();
    state.quote_balance.available = available_quote;
    state.quote_balance.frozen = frozen_quote;
    state.quote_balance.version = version;
    state
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
