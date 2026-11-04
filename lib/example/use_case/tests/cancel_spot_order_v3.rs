use common_entity::{
    LogEntry, LogEntrySequencer, ReplayableChanges, UseCase, StateDiff,
};
use example_core_entity::BalanceV2;
use example_core_entity::account::coin::Coin;
use example_core_entity::account::symbol::Symbol;
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::common::{CommonOrderExecution, CommonOrderTrigger, OrderBase, UserId};
use example_core_entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
use example_core_entity::spot::spot_order_v2::OrderGroupRelation;
use example_core_entity::spot::spot_order_v3::SpotOrderV3Any;
use example_core_use_case::{
    CancelSpotOrderV3Cmd, CancelSpotOrderV3Error, CancelSpotOrderV3State, CancelSpotOrderV3UseCase,
};
use rstest::rstest;
use rust_decimal::Decimal;

fn command(order_id: OrderId, logical_ts: u64) -> LogEntry<CancelSpotOrderV3Cmd> {
    LogEntrySequencer::from_metadata(7, logical_ts, 1, 2, CancelSpotOrderV3Cmd { order_id })
}

fn order(status: SpotOrderStatus, side: OrderSide, size: Decimal) -> OrderBase {
    OrderBase {
        owner: UserId(7),
        order_id: 11,
        symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
        side,
        limit_px: Decimal::from(100),
        size,
        orig_size: Decimal::from(2),
        execution: CommonOrderExecution::Limit,
        tif: OrderTif::Gtc,
        trigger: CommonOrderTrigger::None,
        trigger_px: None,
        group_relation: OrderGroupRelation::Standalone,
        is_position_tpsl: false,
        reduce_only: false,
        status,
        buy_quote_fee_reserve_limit: Decimal::new(2, 1),
        buy_quote_fee_reserve_consumed: Decimal::new(1, 1),
        cloid: Some("cancel-me".into()),
        created_at: 1,
        updated_at: 1,
        version: 3,
    }
}

fn state(order: OrderBase, coin: Coin, frozen: Decimal) -> CancelSpotOrderV3State {
    CancelSpotOrderV3State {
        order: SpotOrderV3Any::from_record(order).unwrap(),
        balance: BalanceV2::new(7, 7, u64::from(coin.index()), Decimal::from(10), frozen, 4, 100),
    }
}

#[rstest]
#[case(SpotOrderStatus::Pending)]
#[case(SpotOrderStatus::Open)]
#[case(SpotOrderStatus::PartiallyFilled)]
fn cancel_each_cancelable_order_status_successfully(#[case] status: SpotOrderStatus) {
    // Rule：三种可取消状态都必须投影为 Canceled，并递增订单和余额版本。
    let given =
        state(order(status, OrderSide::Buy, Decimal::from(2)), Coin::USDT, Decimal::from(201));
    let result = CancelSpotOrderV3UseCase
        .compute_state_changed(&command(OrderId::Cloid("cancel-me".into()), 99), &given)
        .unwrap();
    assert_eq!(result.canceled_order.order_base().status, SpotOrderStatus::Canceled);
    assert_eq!(result.canceled_order.order_base().updated_at, 99);
    assert_eq!(result.canceled_order.order_base().version, 4);
    assert_eq!(result.balance.version(), 5);
}

#[test]
fn open_buy_order_releases_quote_reservation() {
    // Rule：买单释放 quote 币冻结，金额为价格乘剩余数量加未消耗手续费储备。
    let given = state(
        order(SpotOrderStatus::Open, OrderSide::Buy, Decimal::from(2)),
        Coin::USDT,
        Decimal::from(201),
    );
    let before = given.balance.clone();
    let after = CancelSpotOrderV3UseCase
        .compute_state_changed(&command(OrderId::Cloid("cancel-me".into()), 99), &given)
        .unwrap();
    assert_eq!(after.balance.available(), before.available() + Decimal::new(2001, 1));
    assert_eq!(after.balance.frozen(), before.frozen() - Decimal::new(2001, 1));
}

#[test]
fn open_sell_order_releases_base_reservation() {
    // Rule：卖单释放 symbol 的 base 币，而不是 quote 币。
    let given = state(
        order(SpotOrderStatus::Open, OrderSide::Sell, Decimal::from(2)),
        Coin::BTC,
        Decimal::from(2),
    );
    let after = CancelSpotOrderV3UseCase
        .compute_state_changed(&command(OrderId::Cloid("cancel-me".into()), 99), &given)
        .unwrap();
    assert_eq!(after.balance.coin_idx(), u64::from(Coin::BTC.index()));
    assert_eq!(after.balance.available(), Decimal::from(12));
    assert_eq!(after.balance.frozen(), Decimal::ZERO);
}

#[rstest]
#[case(SpotOrderStatus::Filled)]
#[case(SpotOrderStatus::Canceled)]
#[case(SpotOrderStatus::Rejected)]
fn terminal_order_cannot_be_canceled(#[case] status: SpotOrderStatus) {
    // Rule：终止态订单不得再次撤单，且错误必须是具体业务错误。
    let given =
        state(order(status, OrderSide::Buy, Decimal::from(2)), Coin::USDT, Decimal::from(201));
    let error = CancelSpotOrderV3UseCase
        .compute_state_changed(&command(OrderId::Cloid("cancel-me".into()), 99), &given)
        .unwrap_err();
    assert_eq!(error, CancelSpotOrderV3Error::OrderNotCancelable);
    assert_eq!(given.order.order_base().status, status);
}

#[test]
fn cancellation_projects_order_event_before_balance_event() {
    // Rule：持久化先记录订单终态，再记录余额释放，保证业务因果顺序可回放。
    let given = state(
        order(SpotOrderStatus::Open, OrderSide::Buy, Decimal::from(2)),
        Coin::USDT,
        Decimal::from(201),
    );
    let changes = CancelSpotOrderV3UseCase
        .compute_state_diff(&command(OrderId::Cloid("cancel-me".into()), 99), given)
        .unwrap();
    assert_eq!(changes.updated_order.before.order_base().status, SpotOrderStatus::Open);
    assert_eq!(changes.updated_order.after.order_base().status, SpotOrderStatus::Canceled);
    let events = changes.to_replayable_events().unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].entity_type, 16);
    assert_eq!(events[1].entity_type, 12);
    assert_eq!(events[0].logical_ts, 99);
    assert_eq!(events[1].logical_ts, 99);
}

#[test]
fn oid_lookup_cancels_order() {
    let given = state(
        order(SpotOrderStatus::Open, OrderSide::Buy, Decimal::from(2)),
        Coin::USDT,
        Decimal::from(201),
    );
    let result = CancelSpotOrderV3UseCase
        .compute_state_changed(&command(OrderId::Oid(11), 99), &given)
        .unwrap();
    assert_eq!(result.canceled_order.order_base().status, SpotOrderStatus::Canceled);
}

#[test]
fn oid_and_cloid_mismatches_are_rejected() {
    let given = state(
        order(SpotOrderStatus::Open, OrderSide::Buy, Decimal::from(2)),
        Coin::USDT,
        Decimal::from(201),
    );
    assert_eq!(
        CancelSpotOrderV3UseCase
            .compute_state_changed(&command(OrderId::Oid(12), 99), &given)
            .unwrap_err(),
        CancelSpotOrderV3Error::OidMismatch
    );
    assert_eq!(
        CancelSpotOrderV3UseCase
            .compute_state_changed(&command(OrderId::Cloid("other".into()), 99), &given)
            .unwrap_err(),
        CancelSpotOrderV3Error::CloidMismatch
    );
}

#[test]
fn cloid_lookup_requires_loaded_cloid() {
    let mut record = order(SpotOrderStatus::Open, OrderSide::Buy, Decimal::from(2));
    record.cloid = None;
    let given = state(record, Coin::USDT, Decimal::from(201));
    assert_eq!(
        CancelSpotOrderV3UseCase
            .compute_state_changed(&command(OrderId::Cloid("cancel-me".into()), 99), &given)
            .unwrap_err(),
        CancelSpotOrderV3Error::MissingCloid
    );
}

#[test]
fn invalid_oid_and_empty_cloid_are_rejected() {
    let given = state(
        order(SpotOrderStatus::Open, OrderSide::Buy, Decimal::from(2)),
        Coin::USDT,
        Decimal::from(201),
    );
    assert_eq!(
        CancelSpotOrderV3UseCase
            .compute_state_changed(&command(OrderId::Oid(0), 99), &given)
            .unwrap_err(),
        CancelSpotOrderV3Error::InvalidOid
    );
    assert_eq!(
        CancelSpotOrderV3UseCase
            .compute_state_changed(&command(OrderId::Cloid(String::new()), 99), &given)
            .unwrap_err(),
        CancelSpotOrderV3Error::EmptyCloid
    );
}
