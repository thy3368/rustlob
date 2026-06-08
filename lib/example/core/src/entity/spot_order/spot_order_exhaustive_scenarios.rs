use common_entity::Entity;

use super::*;

const ORDER_ID: &str = "order-42";
const TRIGGER_ORDER_ID: &str = "trigger-42";
const ASSET: u32 = 10_001;
const EXCHANGE_OID: u64 = 42;
const ACCOUNT_ID: &str = "trader-1";
const SYMBOL: &str = "BTCUSDT";
const QTY: u64 = 2;
const LIMIT_PRICE: u64 = 100;
const MARKET_AGGRESSIVE_PRICE: u64 = 110;
const TRIGGER_PRICE: u64 = 90;
const TRIGGER_MARKET_AGGRESSIVE_PRICE: u64 = 95;
const CLIENT_ORDER_ID: &str = "0123456789abcdef0123456789abcdef";

fn buy_limit_order(status: SpotOrderStatus, filled_qty: u64) -> SpotOrder {
    active_order(
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        status,
        filled_qty,
        0,
        QTY * LIMIT_PRICE,
    )
}

fn sell_limit_order(status: SpotOrderStatus, filled_qty: u64) -> SpotOrder {
    active_order(
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        status,
        filled_qty,
        QTY,
        0,
    )
}

fn buy_market_order() -> SpotOrder {
    active_order(
        SpotOrderSide::Buy,
        SpotOrderExecution::Market { aggressive_price: MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotOrderStatus::Open,
        0,
        0,
        QTY * MARKET_AGGRESSIVE_PRICE,
    )
}

fn sell_market_order() -> SpotOrder {
    active_order(
        SpotOrderSide::Sell,
        SpotOrderExecution::Market { aggressive_price: MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotOrderStatus::Open,
        0,
        QTY,
        0,
    )
}

fn active_order(
    side: SpotOrderSide,
    execution: SpotOrderExecution,
    time_in_force: SpotOrderTimeInForce,
    status: SpotOrderStatus,
    filled_qty: u64,
    reserved_base: u64,
    reserved_quote: u64,
) -> SpotOrder {
    SpotOrder::new(
        ORDER_ID.to_string(),
        ASSET,
        Some(EXCHANGE_OID),
        ACCOUNT_ID.to_string(),
        SYMBOL.to_string(),
        side,
        execution,
        time_in_force,
        QTY,
        reserved_base,
        reserved_quote,
        Some(CLIENT_ORDER_ID.to_string()),
    )
    .with_execution_state(status, filled_qty)
}

fn conditional_order(
    side: SpotOrderSide,
    trigger_role: SpotOrderTriggerRole,
    execution: SpotOrderExecution,
    time_in_force: SpotOrderTimeInForce,
    status: SpotConditionalOrderStatus,
) -> SpotConditionalOrder {
    SpotConditionalOrder::new(
        TRIGGER_ORDER_ID.to_string(),
        ASSET,
        Some(EXCHANGE_OID),
        ACCOUNT_ID.to_string(),
        SYMBOL.to_string(),
        side,
        TRIGGER_PRICE,
        trigger_role,
        execution,
        time_in_force,
        QTY,
        Some(CLIENT_ORDER_ID.to_string()),
    )
    .with_status(status)
}

fn buy_stop_loss_market_conditional() -> SpotConditionalOrder {
    conditional_order(
        SpotOrderSide::Buy,
        SpotOrderTriggerRole::StopLoss,
        SpotOrderExecution::Market { aggressive_price: TRIGGER_MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotConditionalOrderStatus::Open,
    )
}

fn buy_take_profit_limit_conditional() -> SpotConditionalOrder {
    conditional_order(
        SpotOrderSide::Buy,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Open,
    )
}

fn sell_stop_loss_market_conditional() -> SpotConditionalOrder {
    conditional_order(
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::StopLoss,
        SpotOrderExecution::Market { aggressive_price: TRIGGER_MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotConditionalOrderStatus::Open,
    )
}

fn sell_take_profit_limit_conditional() -> SpotConditionalOrder {
    conditional_order(
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Open,
    )
}

fn created_value<E>(entity: &E, field_name: &str) -> Option<String>
where
    E: Entity,
{
    entity
        .created_field_changes()
        .into_iter()
        .find(|change| change.field_name == field_name)
        .map(|change| change.new_value)
}

fn assert_active_order_business_facts(
    scenario_name: &str,
    order: &SpotOrder,
    expected_side: SpotOrderSide,
    expected_execution: SpotOrderExecution,
    expected_time_in_force: SpotOrderTimeInForce,
    expected_status: SpotOrderStatus,
    expected_filled_qty: u64,
    expected_reserved_base: u64,
    expected_reserved_quote: u64,
) {
    println!(
        "active scenario={scenario_name}, order_id={}, asset={}, exchange_oid={:?}, account_id={}, symbol={}, side={:?}, execution={:?}, tif={:?}, qty={}, filled_qty={}, status={:?}, reserved_base={}, reserved_quote={}, can_cancel={}, client_order_id={:?}",
        order.order_id,
        order.asset,
        order.exchange_oid,
        order.account_id,
        order.symbol,
        order.side,
        order.execution,
        order.time_in_force,
        order.qty,
        order.filled_qty,
        order.status,
        order.reserved_base,
        order.reserved_quote,
        order.can_be_cancelled(),
        order.client_order_id,
    );

    assert_eq!(order.order_id, ORDER_ID);
    assert_eq!(order.asset, ASSET);
    assert_eq!(order.exchange_oid, Some(EXCHANGE_OID));
    assert_eq!(order.account_id, ACCOUNT_ID);
    assert_eq!(order.symbol, SYMBOL);
    assert_eq!(order.side, expected_side);
    assert_eq!(order.execution, expected_execution);
    assert_eq!(order.time_in_force, expected_time_in_force);
    assert_eq!(order.qty, QTY);
    assert_eq!(order.filled_qty, expected_filled_qty);
    assert_eq!(order.status, expected_status);
    assert_eq!(order.reserved_base, expected_reserved_base);
    assert_eq!(order.reserved_quote, expected_reserved_quote);
    assert_eq!(order.can_be_cancelled(), expected_status.is_cancelable());
    assert!(order.has_consistent_execution_state());
    assert!(order.has_consistent_reserved_base());
    assert!(order.has_consistent_reserved_quote());
    assert_eq!(order.base_to_release_on_cancel(), expected_reserved_base);
    assert_eq!(order.quote_to_release_on_cancel(), expected_reserved_quote);
    assert_eq!(created_value(order, "asset"), Some(ASSET.to_string()));
    assert_eq!(created_value(order, "symbol"), Some(SYMBOL.to_string()));
    assert_eq!(created_value(order, "client_order_id"), Some(CLIENT_ORDER_ID.to_string()));
}

fn assert_conditional_order_business_facts(
    scenario_name: &str,
    order: &SpotConditionalOrder,
    expected_side: SpotOrderSide,
    expected_trigger_role: SpotOrderTriggerRole,
    expected_execution: SpotOrderExecution,
    expected_time_in_force: SpotOrderTimeInForce,
    expected_status: SpotConditionalOrderStatus,
) {
    println!(
        "conditional scenario={scenario_name}, trigger_order_id={}, asset={}, exchange_oid={:?}, account_id={}, symbol={}, side={:?}, trigger_price={}, trigger_role={:?}, execution={:?}, tif={:?}, qty={}, status={:?}, can_cancel={}, client_order_id={:?}",
        order.trigger_order_id,
        order.asset,
        order.exchange_oid,
        order.account_id,
        order.symbol,
        order.side,
        order.trigger_price,
        order.trigger_role,
        order.execution,
        order.time_in_force,
        order.qty,
        order.status,
        order.can_be_cancelled(),
        order.client_order_id,
    );

    assert_eq!(order.trigger_order_id, TRIGGER_ORDER_ID);
    assert_eq!(order.asset, ASSET);
    assert_eq!(order.exchange_oid, Some(EXCHANGE_OID));
    assert_eq!(order.account_id, ACCOUNT_ID);
    assert_eq!(order.symbol, SYMBOL);
    assert_eq!(order.side, expected_side);
    assert_eq!(order.trigger_price, TRIGGER_PRICE);
    assert_eq!(order.trigger_role, expected_trigger_role);
    assert_eq!(order.execution, expected_execution);
    assert_eq!(order.time_in_force, expected_time_in_force);
    assert_eq!(order.qty, QTY);
    assert_eq!(order.status, expected_status);
    assert_eq!(order.can_be_cancelled(), expected_status.is_cancelable());
    assert_eq!(created_value(order, "asset"), Some(ASSET.to_string()));
    assert_eq!(created_value(order, "trigger_price"), Some(TRIGGER_PRICE.to_string()));
    assert_eq!(
        created_value(order, "trigger_role"),
        Some(expected_trigger_role.as_str().to_string())
    );
}

fn assert_triggered_active_order(
    scenario_name: &str,
    conditional: &SpotConditionalOrder,
    expected_reserved_base: u64,
    expected_reserved_quote: u64,
) {
    let active = conditional.triggered_order(
        format!("active-{scenario_name}"),
        expected_reserved_base,
        expected_reserved_quote,
    );

    println!(
        "trigger scenario={scenario_name}, trigger_order_id={}, active_order_id={}, side={:?}, execution={:?}, tif={:?}, qty={}, reserved_base={}, reserved_quote={}",
        conditional.trigger_order_id,
        active.order_id,
        active.side,
        active.execution,
        active.time_in_force,
        active.qty,
        active.reserved_base,
        active.reserved_quote,
    );

    assert_eq!(active.asset, conditional.asset);
    assert_eq!(active.exchange_oid, conditional.exchange_oid);
    assert_eq!(active.account_id, conditional.account_id);
    assert_eq!(active.symbol, conditional.symbol);
    assert_eq!(active.side, conditional.side);
    assert_eq!(active.execution, conditional.execution);
    assert_eq!(active.time_in_force, conditional.time_in_force);
    assert_eq!(active.qty, conditional.qty);
    assert_eq!(active.reserved_base, expected_reserved_base);
    assert_eq!(active.reserved_quote, expected_reserved_quote);
}

#[test]
fn buy_limit_open_order() {
    assert_active_order_business_facts(
        "buy_limit_open",
        &buy_limit_order(SpotOrderStatus::Open, 0),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::Open,
        0,
        0,
        QTY * LIMIT_PRICE,
    );
}

#[test]
fn buy_limit_partially_filled_order() {
    assert_active_order_business_facts(
        "buy_limit_partially_filled",
        &buy_limit_order(SpotOrderStatus::PartiallyFilled, 1),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::PartiallyFilled,
        1,
        0,
        QTY * LIMIT_PRICE,
    );
}

#[test]
fn buy_limit_filled_order() {
    assert_active_order_business_facts(
        "buy_limit_filled",
        &buy_limit_order(SpotOrderStatus::Filled, QTY),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::Filled,
        QTY,
        0,
        QTY * LIMIT_PRICE,
    );
}

#[test]
fn buy_limit_canceled_order() {
    assert_active_order_business_facts(
        "buy_limit_canceled",
        &buy_limit_order(SpotOrderStatus::Canceled, 1),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::Canceled,
        1,
        0,
        QTY * LIMIT_PRICE,
    );
}

#[test]
fn sell_limit_open_order() {
    assert_active_order_business_facts(
        "sell_limit_open",
        &sell_limit_order(SpotOrderStatus::Open, 0),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::Open,
        0,
        QTY,
        0,
    );
}

#[test]
fn sell_limit_partially_filled_order() {
    assert_active_order_business_facts(
        "sell_limit_partially_filled",
        &sell_limit_order(SpotOrderStatus::PartiallyFilled, 1),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::PartiallyFilled,
        1,
        QTY,
        0,
    );
}

#[test]
fn sell_limit_filled_order() {
    assert_active_order_business_facts(
        "sell_limit_filled",
        &sell_limit_order(SpotOrderStatus::Filled, QTY),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::Filled,
        QTY,
        QTY,
        0,
    );
}

#[test]
fn sell_limit_canceled_order() {
    assert_active_order_business_facts(
        "sell_limit_canceled",
        &sell_limit_order(SpotOrderStatus::Canceled, 1),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotOrderStatus::Canceled,
        1,
        QTY,
        0,
    );
}

#[test]
fn buy_market_open_order() {
    assert_active_order_business_facts(
        "buy_market_open",
        &buy_market_order(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Market { aggressive_price: MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotOrderStatus::Open,
        0,
        0,
        QTY * MARKET_AGGRESSIVE_PRICE,
    );
}

#[test]
fn sell_market_open_order() {
    assert_active_order_business_facts(
        "sell_market_open",
        &sell_market_order(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Market { aggressive_price: MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotOrderStatus::Open,
        0,
        QTY,
        0,
    );
}

#[test]
fn buy_stop_loss_market_conditional_order() {
    assert_conditional_order_business_facts(
        "buy_stop_loss_market",
        &buy_stop_loss_market_conditional(),
        SpotOrderSide::Buy,
        SpotOrderTriggerRole::StopLoss,
        SpotOrderExecution::Market { aggressive_price: TRIGGER_MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotConditionalOrderStatus::Open,
    );
}

#[test]
fn buy_take_profit_limit_conditional_order() {
    assert_conditional_order_business_facts(
        "buy_take_profit_limit",
        &buy_take_profit_limit_conditional(),
        SpotOrderSide::Buy,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Open,
    );
}

#[test]
fn sell_stop_loss_market_conditional_order() {
    assert_conditional_order_business_facts(
        "sell_stop_loss_market",
        &sell_stop_loss_market_conditional(),
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::StopLoss,
        SpotOrderExecution::Market { aggressive_price: TRIGGER_MARKET_AGGRESSIVE_PRICE },
        SpotOrderTimeInForce::Ioc,
        SpotConditionalOrderStatus::Open,
    );
}

#[test]
fn sell_take_profit_limit_conditional_order() {
    assert_conditional_order_business_facts(
        "sell_take_profit_limit",
        &sell_take_profit_limit_conditional(),
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Open,
    );
}

#[test]
fn triggered_conditional_order_is_not_cancelable() {
    assert_conditional_order_business_facts(
        "triggered",
        &conditional_order(
            SpotOrderSide::Sell,
            SpotOrderTriggerRole::TakeProfit,
            SpotOrderExecution::Limit { price: LIMIT_PRICE },
            SpotOrderTimeInForce::Gtc,
            SpotConditionalOrderStatus::Triggered,
        ),
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Triggered,
    );
}

#[test]
fn canceled_conditional_order_is_not_cancelable() {
    assert_conditional_order_business_facts(
        "canceled",
        &conditional_order(
            SpotOrderSide::Sell,
            SpotOrderTriggerRole::TakeProfit,
            SpotOrderExecution::Limit { price: LIMIT_PRICE },
            SpotOrderTimeInForce::Gtc,
            SpotConditionalOrderStatus::Canceled,
        ),
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Canceled,
    );
}

#[test]
fn rejected_conditional_order_is_not_cancelable() {
    assert_conditional_order_business_facts(
        "rejected",
        &conditional_order(
            SpotOrderSide::Sell,
            SpotOrderTriggerRole::TakeProfit,
            SpotOrderExecution::Limit { price: LIMIT_PRICE },
            SpotOrderTimeInForce::Gtc,
            SpotConditionalOrderStatus::Rejected,
        ),
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Rejected,
    );
}

#[test]
fn expired_conditional_order_is_not_cancelable() {
    assert_conditional_order_business_facts(
        "expired",
        &conditional_order(
            SpotOrderSide::Sell,
            SpotOrderTriggerRole::TakeProfit,
            SpotOrderExecution::Limit { price: LIMIT_PRICE },
            SpotOrderTimeInForce::Gtc,
            SpotConditionalOrderStatus::Expired,
        ),
        SpotOrderSide::Sell,
        SpotOrderTriggerRole::TakeProfit,
        SpotOrderExecution::Limit { price: LIMIT_PRICE },
        SpotOrderTimeInForce::Gtc,
        SpotConditionalOrderStatus::Expired,
    );
}

#[test]
fn buy_stop_loss_market_conditional_triggers_active_order() {
    assert_triggered_active_order(
        "buy_stop_loss_market",
        &buy_stop_loss_market_conditional(),
        0,
        190,
    );
}

#[test]
fn buy_take_profit_limit_conditional_triggers_active_order() {
    assert_triggered_active_order(
        "buy_take_profit_limit",
        &buy_take_profit_limit_conditional(),
        0,
        QTY * LIMIT_PRICE,
    );
}

#[test]
fn sell_stop_loss_market_conditional_triggers_active_order() {
    assert_triggered_active_order(
        "sell_stop_loss_market",
        &sell_stop_loss_market_conditional(),
        QTY,
        0,
    );
}

#[test]
fn sell_take_profit_limit_conditional_triggers_active_order() {
    assert_triggered_active_order(
        "sell_take_profit_limit",
        &sell_take_profit_limit_conditional(),
        QTY,
        0,
    );
}
