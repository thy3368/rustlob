use common_entity::{
    AggregateRole, Entity, FieldDiff, FinancialClassification, FourColorArchetype,
};
use decimal::Decimal;

use super::hyperliquid_perp_order::{
    HyperliquidPerpOrder, HyperliquidPerpOrderBehaviorError, HyperliquidPerpOrderExecution,
    HyperliquidPerpOrderSide, HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
    PlaceHyperliquidPerpOrderInput,
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

// 下单会创建 Open 订单，并同时创建内部保证金 reservation。
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

// 订单 reservation 会派生冻结余额流水。
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

// 未撮合永续订单只冻结保证金，使 clearinghouse withdrawable 下降，不产生仓位。
#[test]
fn given_unmatched_perp_order_when_margin_is_reserved_then_clearinghouse_withdrawable_decreases() {
    let clearinghouse_before = clearinghouse_state_from_facts(1_000, vec![]);
    let mut balance = Balance::new("trader-1".to_string(), "USDC".to_string(), 1_000, 200, 1);
    let total_before = balance.total();

    let mut input = place_input();
    input.margin_balance_entity_id = balance.entity_id();
    let mut outcome = HyperliquidPerpOrder::place(input).unwrap();
    let reserved_amount = outcome.order.reservation.original_amount;

    outcome.freeze_ledger_entry.apply_to(&mut balance).unwrap();
    let clearinghouse_after =
        clearinghouse_state_from_facts(1_000, vec![outcome.order.reservation.clone()]);

    assert_eq!(clearinghouse_before.withdrawable(), dec(1_000));
    assert_eq!(clearinghouse_after.withdrawable(), dec(970));
    assert_eq!(
        clearinghouse_before.withdrawable() - dec(reserved_amount as i64),
        clearinghouse_after.withdrawable()
    );
    assert_eq!(balance.available, 970);
    assert_eq!(balance.frozen, 230);
    assert_eq!(balance.total(), total_before);
    assert_eq!(outcome.freeze_ledger_entry.before_available, Some(1_000));
    assert_eq!(outcome.freeze_ledger_entry.after_available, Some(970));
    assert_eq!(outcome.freeze_ledger_entry.before_frozen, Some(200));
    assert_eq!(outcome.freeze_ledger_entry.after_frozen, Some(230));
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
    zero_margin.margin_amount = 0;
    assert_eq!(
        HyperliquidPerpOrder::place(zero_margin),
        Err(HyperliquidPerpOrderBehaviorError::InvalidMarginAmount)
    );
}

// 创建事件字段必须包含 reservation 的关键事实。
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

// diff 能识别 reservation 归属、原因、市场、资产、金额和状态变化。
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

// 实体元数据把订单标记为 MI 聚合根业务凭证。
#[test]
fn metadata_marks_order_as_moment_interval_root_business_voucher() {
    assert_eq!(HyperliquidPerpOrder::four_color_archetype(), FourColorArchetype::MomentInterval);
    assert_eq!(HyperliquidPerpOrder::aggregate_role(), AggregateRole::AggregateRoot);
    assert_eq!(
        HyperliquidPerpOrder::financial_classification(),
        FinancialClassification::BusinessVoucher
    );
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

// 普通订单默认非强平，强平来源可被标记。
#[test]
fn liquidation_marker_defaults_and_can_be_set() {
    let normal = order();
    let liquidation = order().with_liquidation("liq-1".to_string());

    assert!(!normal.is_liquidation);
    assert_eq!(normal.liquidation_id, None);
    assert!(liquidation.is_liquidation);
    assert_eq!(liquidation.liquidation_id.as_deref(), Some("liq-1"));
}
