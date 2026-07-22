use common_entity::Entity;
use decimal::Decimal;

use super::hyperliquid_perp_order::{
    HyperliquidPerpOrder, HyperliquidPerpOrderBehaviorError, HyperliquidPerpOrderExecution,
    HyperliquidPerpOrderSide, HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
    PlaceHyperliquidPerpOrderInput, PlaceHyperliquidPerpOrderIntent,
};
use crate::entity::{
    AccountId, Balance, BalanceLedgerReason, MarginReservation, PerpAssetRiskRule,
    PerpClearinghouseState, PerpClearinghouseStateCalcInput, PerpCollateralSnapshot,
    PerpMarketMark, PerpRiskPolicy, Reservation, ReservationKind, ReservationMarketKind,
    ReservationStatus,
};

fn dec(units: i64) -> Decimal {
    Decimal::from_raw(units * 100_000_000)
}

fn rate_bps(bps: i64) -> Decimal {
    Decimal::from_raw(bps * 10_000)
}

fn clearinghouse_state_from_facts(
    collateral: i64,
    reservations: Vec<MarginReservation>,
) -> PerpClearinghouseState {
    let calc_input = PerpClearinghouseStateCalcInput {
        account_id: AccountId::from("trader-1"),
        positions: vec![],
        collateral: PerpCollateralSnapshot {
            total_raw_usd: dec(collateral),
            pending_settlement_delta: dec(0),
        },
        market_marks: vec![PerpMarketMark { asset: 0, mark_price: dec(100) }],
        risk_rules: vec![PerpAssetRiskRule {
            asset: 0,
            initial_margin_rate: rate_bps(1_000),
            maintenance_margin_rate: rate_bps(500),
        }],
        open_order_margin_reservations: reservations,
        risk_policy: PerpRiskPolicy { reduce_only_withdrawable_threshold: dec(0) },
    };

    match PerpClearinghouseState::calculate_from_facts(calc_input) {
        Ok(state) => state,
        Err(error) => panic!("calculation must succeed: {error}"),
    }
}

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
        Some(reservation("order-1")),
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
        client_order_id: Some("client-1".to_string()),
        liquidation_id: None,
        intent: PlaceHyperliquidPerpOrderIntent::Open {
            margin_asset_id: "USDC".to_string(),
            margin_balance_entity_id: "balance-1".to_string(),
            margin_amount: 30,
        },
    }
}

// ===== 开仓：订单创建、保证金冻结、reservation 事实 =====

// 下单会创建 Open 订单，并同时创建内部保证金 reservation。
#[test]
fn place_creates_order_with_internal_reservation() {
    let outcome = HyperliquidPerpOrder::place(place_input()).unwrap();

    assert_eq!(outcome.order.order_id, "place-1");
    assert_eq!(outcome.order.exchange_oid, None);
    assert_eq!(outcome.order.filled_qty, 0);
    assert_eq!(outcome.order.status, HyperliquidPerpOrderStatus::Open);
    assert_eq!(outcome.order.version, 1);
    let reservation = outcome.order.reservation.as_ref().unwrap();
    assert_eq!(reservation.reservation_id, "reservation:place-1");
    assert_eq!(reservation.owner_account_id, "trader-1");
    assert_eq!(reservation.caused_by_order_id, "place-1");
    assert_eq!(reservation.market_kind, ReservationMarketKind::Perp);
    assert_eq!(reservation.asset_id, "USDC");
    assert_eq!(reservation.reservation_kind, ReservationKind::PerpOpenMargin);
    assert_eq!(reservation.original_amount, 30);
    assert_eq!(reservation.remaining_amount, 30);
    assert_eq!(reservation.status, ReservationStatus::Active);
}

// 订单 reservation 会派生冻结余额流水。
#[test]
fn place_derives_freeze_ledger_from_reservation() {
    let outcome = HyperliquidPerpOrder::place(place_input()).unwrap();
    let reservation = outcome.order.reservation.as_ref().unwrap();
    let freeze_ledger_entry = outcome.freeze_ledger_entry.as_ref().unwrap();

    assert_eq!(freeze_ledger_entry.entry_id, "balance-ledger:freeze:place-1");
    assert_eq!(freeze_ledger_entry.account_id, outcome.order.account_id);
    assert_eq!(freeze_ledger_entry.asset_id, reservation.asset_id);
    assert_eq!(freeze_ledger_entry.balance_entity_id, "balance-1");
    assert_eq!(freeze_ledger_entry.amount, reservation.original_amount);
    assert_eq!(
        freeze_ledger_entry.reason,
        BalanceLedgerReason::FreezeForOrder { order_id: "place-1".to_string() }
    );
}

// 未撮合永续订单只冻结保证金，使 clearinghouse withdrawable 下降，不产生仓位。
#[test]
fn given_unmatched_perp_order_when_margin_is_reserved_then_clearinghouse_withdrawable_decreases() {
    let clearinghouse_before = clearinghouse_state_from_facts(1_000, vec![]);
    let mut balance = Balance::new("trader-1".to_string(), "USDC".to_string(), 1_000, 200, 1);
    let total_before = balance.total();

    let mut input = place_input();
    input.intent = PlaceHyperliquidPerpOrderIntent::Open {
        margin_asset_id: "USDC".to_string(),
        margin_balance_entity_id: balance.entity_id(),
        margin_amount: 30,
    };
    let mut outcome = HyperliquidPerpOrder::place(input).unwrap();
    let reservation = outcome.order.reservation.clone().unwrap();
    let reserved_amount = reservation.original_amount;

    outcome.freeze_ledger_entry.as_mut().unwrap().apply_to(&mut balance).unwrap();
    let clearinghouse_after = clearinghouse_state_from_facts(1_000, vec![reservation]);

    assert_eq!(clearinghouse_before.withdrawable(), dec(1_000));
    assert_eq!(clearinghouse_after.withdrawable(), dec(970));
    assert_eq!(
        clearinghouse_before.withdrawable() - dec(reserved_amount as i64),
        clearinghouse_after.withdrawable()
    );
    assert_eq!(balance.available, 970);
    assert_eq!(balance.frozen, 230);
    assert_eq!(balance.total(), total_before);
    let freeze_ledger_entry = outcome.freeze_ledger_entry.as_ref().unwrap();
    assert_eq!(freeze_ledger_entry.before_available, Some(1_000));
    assert_eq!(freeze_ledger_entry.after_available, Some(970));
    assert_eq!(freeze_ledger_entry.before_frozen, Some(200));
    assert_eq!(freeze_ledger_entry.after_frozen, Some(230));
    assert_eq!(outcome.order.status, HyperliquidPerpOrderStatus::Open);
    assert_eq!(outcome.order.filled_qty, 0);
    assert!(!clearinghouse_after.has_open_positions());
    assert_eq!(clearinghouse_before.margin_summary().total_position_notional(), dec(0));
    assert_eq!(clearinghouse_after.margin_summary().total_position_notional(), dec(0));
}

// 市价意图和限价意图只要价格为正都可以创建订单。
#[test]
fn place_accepts_market_and_limit_positive_prices() {
    let mut market = place_input();
    market.execution = HyperliquidPerpOrderExecution::Market { aggressive_price: 200 };
    assert!(HyperliquidPerpOrder::place(market).is_ok());

    let mut limit = place_input();
    limit.execution = HyperliquidPerpOrderExecution::Limit { price: 200 };
    assert!(HyperliquidPerpOrder::place(limit).is_ok());
}

// 数量、价格、保证金为 0 时会按业务错误拒绝下单。
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
    zero_margin.intent = PlaceHyperliquidPerpOrderIntent::Open {
        margin_asset_id: "USDC".to_string(),
        margin_balance_entity_id: "balance-1".to_string(),
        margin_amount: 0,
    };
    assert_eq!(
        HyperliquidPerpOrder::place(zero_margin),
        Err(HyperliquidPerpOrderBehaviorError::InvalidMarginAmount)
    );
}

// ===== 平仓：Close 意图与剩余量一致性 =====

// Close 意图会创建 reduce-only 订单，且不会创建保证金 reservation 或冻结流水。
#[test]
fn place_close_creates_reduce_only_order_without_reservation_or_freeze_ledger() {
    let mut input = place_input();
    input.intent = PlaceHyperliquidPerpOrderIntent::Close;

    let outcome = HyperliquidPerpOrder::place(input).unwrap();

    assert_eq!(outcome.order.order_id, "place-1");
    assert_eq!(outcome.order.status, HyperliquidPerpOrderStatus::Open);
    assert!(outcome.order.reduce_only);
    assert_eq!(outcome.order.reservation, None);
    assert_eq!(outcome.freeze_ledger_entry, None);
}

// reduce-only 部分成交订单的剩余量与生命周期状态必须一致。
#[test]
fn reduce_only_partially_filled_order_keeps_remaining_qty_and_execution_state_consistent() {
    let mut input = place_input();
    input.intent = PlaceHyperliquidPerpOrderIntent::Close;
    let order = HyperliquidPerpOrder::place(input)
        .unwrap()
        .order
        .with_execution_state(HyperliquidPerpOrderStatus::PartiallyFilled, 1);

    assert!(order.reduce_only);
    assert_eq!(order.remaining_qty(), Some(2));
    assert!(order.has_consistent_execution_state());
    assert!(order.is_matchable());
}

// 订单会暴露撮合所需的归属、合约、剩余量、价格和状态事实。
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

// 不一致成交状态会导致订单不可撮合。
#[test]
fn execution_state_detects_inconsistent_fills() {
    let order = order().with_execution_state(HyperliquidPerpOrderStatus::PartiallyFilled, 3);

    assert!(!order.has_consistent_execution_state());
    assert!(!order.is_matchable());
}

// ===== 强平：风险订单来源标记 =====

// 普通订单默认非强平，强平来源可被标记。
#[test]
fn liquidation_marker_defaults_and_can_be_set() {
    let normal = HyperliquidPerpOrder::place(place_input()).unwrap().order;
    let mut liquidation_input = place_input();
    liquidation_input.liquidation_id = Some("liq-1".to_string());
    let liquidation = HyperliquidPerpOrder::place(liquidation_input).unwrap().order;

    assert!(!normal.is_liquidation);
    assert_eq!(normal.liquidation_id, None);
    assert!(liquidation.is_liquidation);
    assert_eq!(liquidation.liquidation_id.as_deref(), Some("liq-1"));
}
