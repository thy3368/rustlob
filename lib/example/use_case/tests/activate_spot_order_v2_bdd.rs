use common_entity::{
    ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use example_core_entity::{
    ActivatePendingSpotOrderV2Input, Balance, SpotOrderSide, SpotOrderStatus, SpotOrderTif,
    SpotOrderTriggerRole, SpotOrderType, SpotOrderV2,
};
use example_core_use_case::{
    ActivateSpotOrderV2Cmd, ActivateSpotOrderV2Error, ActivateSpotOrderV2State,
    ActivateSpotOrderV2UseCase,
};

fn context() -> ExecutionContext {
    ExecutionContext { execution_time_ns: 100 }
}

fn cmd(order_id: u64) -> ActivateSpotOrderV2Cmd {
    ActivateSpotOrderV2Cmd { party_id: "trader-1".to_string(), asset: 10_001, order_id }
}

fn pending_limit(order_id: u64, side: SpotOrderSide) -> SpotOrderV2 {
    SpotOrderV2::new_pending_limit(
        order_id,
        10_001,
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        side,
        2,
        100,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        None,
        1,
        1,
    )
}

fn pending_trigger(order_id: u64) -> SpotOrderV2 {
    SpotOrderV2::new_pending_trigger(
        order_id,
        10_001,
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        2,
        100,
        SpotOrderType::Trigger {
            is_market: false,
            trigger_price: 90,
            tpsl: SpotOrderTriggerRole::TakeProfit,
        },
        None,
        1,
        1,
    )
}

fn state(order: SpotOrderV2, maker_fee_bps: u64, taker_fee_bps: u64) -> ActivateSpotOrderV2State {
    ActivateSpotOrderV2State {
        pending_order: order,
        balances: vec![
            Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
            Balance::new("trader-1".to_string(), "BTC".to_string(), 10, 0, 1),
        ],
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps,
        taker_fee_bps,
    }
}

// Given：账户有足够的 USDT 余额，待激活买单需要冻结 quote principal 和 quote fee。
// When：调用 ActivateSpotOrderV2UseCase 激活买单。
// Then：订单变为 Open，USDT 可用余额与冻结余额正确变化，并生成 principal、fee ledger 及 replay events。
#[test]
fn buy_activation_freezes_quote_principal_and_quote_fee() {
    let order = pending_limit(1, SpotOrderSide::Buy);

    let changes = ActivateSpotOrderV2UseCase
        .compute_state_diff_with_context(&cmd(1), state(order, 5, 10), &context())
        .expect("buy activation should compute changes");

    assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Open);
    assert_eq!(changes.updated_order.after.reservation.asset_id, "USDT");
    assert_eq!(changes.updated_order.after.reservation.original_amount, 200);
    assert_eq!(changes.created_balance_ledger_entries.len(), 2);
    assert_eq!(changes.updated_balances[1].before.available, 1_000);
    assert_eq!(changes.updated_balances[1].after.available, 799);
    assert_eq!(changes.updated_balances[1].after.frozen, 201);

    let events = changes.to_replayable_events().expect("activation should project events");
    assert_eq!(events.len(), 5);
    assert!(events[0].is_updated());
    assert!(events[1].is_updated());
    assert!(events[2].is_updated());
    assert!(events[3].is_created());
    assert!(events[4].is_created());
}

// Given：账户有足够的 BTC 余额，待激活卖单需要冻结 base principal。
// When：调用 ActivateSpotOrderV2UseCase 激活卖单。
// Then：订单冻结 BTC principal，BTC 可用余额按冻结数量减少。
#[test]
fn sell_activation_freezes_base_principal() {
    let order = pending_limit(2, SpotOrderSide::Sell);

    let changes = ActivateSpotOrderV2UseCase
        .compute_state_diff_with_context(&cmd(2), state(order, 5, 10), &context())
        .expect("sell activation should compute changes");

    assert_eq!(changes.updated_order.after.reservation.asset_id, "BTC");
    assert_eq!(changes.updated_order.after.reservation.original_amount, 2);
    assert_eq!(changes.updated_balances[0].before.available, 10);
    assert_eq!(changes.updated_balances[0].after.available, 8);
}

// Given：待激活买单的 maker 和 taker 手续费率均为零。
// When：调用 ActivateSpotOrderV2UseCase 激活订单。
// Then：不创建 fee ledger，只创建 principal ledger，并投影出对应的 replay events。
#[test]
fn zero_fee_activation_skips_fee_ledger() {
    let order = pending_limit(3, SpotOrderSide::Buy);

    let changes = ActivateSpotOrderV2UseCase
        .compute_state_diff_with_context(&cmd(3), state(order, 0, 0), &context())
        .expect("zero fee activation should compute changes");

    assert_eq!(changes.updated_order.after.fee_reservation.original_amount, 0);
    assert_eq!(changes.created_balance_ledger_entries.len(), 1);
    assert_eq!(changes.to_replayable_events().expect("events should project").len(), 3);
}

// Given：账户余额充足，且订单是无需市场价格判断的待激活 trigger order。
// When：调用 ActivateSpotOrderV2UseCase 激活 trigger order。
// Then：订单直接变为 Open，并保留 Trigger 订单类型。
#[test]
fn pending_trigger_activation_does_not_need_market_price() {
    let order = pending_trigger(4);

    let changes = ActivateSpotOrderV2UseCase
        .compute_state_diff_with_context(&cmd(4), state(order, 5, 10), &context())
        .expect("trigger activation should compute without market price");

    assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Open);
    assert!(matches!(changes.updated_order.after.order_type, SpotOrderType::Trigger { .. }));
}

// Given：订单已经处于 Open 状态，不再是 Pending。
// When：对该订单执行激活状态校验。
// Then：校验返回 OrderNotPending 业务错误。
#[test]
fn open_order_is_rejected_by_activation_state_validation() {
    let mut order = pending_limit(5, SpotOrderSide::Buy);
    order
        .activate_pending(ActivatePendingSpotOrderV2Input {
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
            timestamp: 2,
        })
        .expect("fixture activation should work");

    let result = ActivateSpotOrderV2UseCase.validate_state_given(&cmd(5), &state(order, 5, 10));

    assert_eq!(result, Err(ActivateSpotOrderV2Error::OrderNotPending));
}
