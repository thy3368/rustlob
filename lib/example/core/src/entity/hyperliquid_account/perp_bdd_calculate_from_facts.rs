use decimal::Decimal;

use super::AccountId;
use super::perp::{
    PerpAssetRiskRule, PerpClearinghouseState, PerpClearinghouseStateCalcError,
    PerpClearinghouseStateCalcInput, PerpCollateralSnapshot, PerpMarketMark, PerpRiskPolicy,
    RiskState,
};
use crate::entity::{
    HyperliquidPerpMarginMode, HyperliquidPerpPosition, HyperliquidPerpPositionSide,
    MarginReservation, Reservation, ReservationKind, ReservationMarketKind,
};

fn dec(units: i64) -> Decimal {
    Decimal::from_raw(units * 100_000_000)
}

fn rate_bps(bps: i64) -> Decimal {
    Decimal::from_raw(bps * 10_000)
}

fn input(
    positions: Vec<HyperliquidPerpPosition>,
    collateral: i64,
) -> PerpClearinghouseStateCalcInput {
    PerpClearinghouseStateCalcInput {
        account_id: AccountId::from("sub-1"),
        positions,
        collateral: PerpCollateralSnapshot {
            total_raw_usd: dec(collateral),
            pending_settlement_delta: dec(0),
        },
        market_marks: vec![mark(0, 100), mark(1, 50)],
        risk_rules: vec![risk_rule(0, 1_000, 500), risk_rule(1, 2_000, 1_000)],
        open_order_margin_reservations: vec![],
        risk_policy: PerpRiskPolicy { reduce_only_withdrawable_threshold: dec(100) },
    }
}

fn mark(asset: u32, price: i64) -> PerpMarketMark {
    PerpMarketMark { asset, mark_price: dec(price) }
}

fn risk_rule(asset: u32, initial_bps: i64, maintenance_bps: i64) -> PerpAssetRiskRule {
    PerpAssetRiskRule {
        asset,
        initial_margin_rate: rate_bps(initial_bps),
        maintenance_margin_rate: rate_bps(maintenance_bps),
    }
}

fn position(
    asset: u32,
    symbol: &str,
    qty: u64,
    side: HyperliquidPerpPositionSide,
    margin_mode: HyperliquidPerpMarginMode,
    unrealized_pnl: i64,
) -> HyperliquidPerpPosition {
    HyperliquidPerpPosition::new(
        format!("sub-1-{symbol}"),
        "sub-1".to_owned(),
        asset,
        symbol.to_owned(),
        side,
        qty,
        100,
        5,
        margin_mode,
        20,
        None,
        unrealized_pnl,
        0,
        1,
    )
}

fn active_perp_reservation(amount: u64) -> MarginReservation {
    match Reservation::new(
        "reservation:open-1".to_owned(),
        "sub-1".to_owned(),
        "order-1".to_owned(),
        ReservationMarketKind::Perp,
        ReservationKind::PerpOpenMargin,
        "USDC".to_owned(),
        amount,
    ) {
        Ok(reservation) => reservation,
        Err(error) => panic!("fixture reservation must be valid: {error}"),
    }
}

#[test]
fn given_cross_positions_when_calculated_then_margin_summaries_include_cross_risk() {
    let mut calc_input = input(
        vec![
            position(
                0,
                "BTC-PERP",
                2,
                HyperliquidPerpPositionSide::Long,
                HyperliquidPerpMarginMode::Cross,
                30,
            ),
            position(
                1,
                "ETH-PERP",
                4,
                HyperliquidPerpPositionSide::Short,
                HyperliquidPerpMarginMode::Isolated,
                -10,
            ),
        ],
        1_000,
    );
    calc_input.risk_policy.reduce_only_withdrawable_threshold = dec(0);

    let state = match PerpClearinghouseState::calculate_from_facts(calc_input) {
        Ok(state) => state,
        Err(error) => panic!("calculation must succeed: {error}"),
    };

    assert_eq!(state.margin_summary().total_position_notional(), dec(400));
    assert_eq!(state.margin_summary().total_margin_used(), dec(60));
    assert_eq!(state.margin_summary().account_value(), dec(1_200));
    assert_eq!(state.cross_margin_summary().total_position_notional(), dec(200));
    assert_eq!(state.cross_margin_summary().total_margin_used(), dec(20));
    assert_eq!(state.cross_margin_summary().account_value(), dec(1_000));
    assert_eq!(state.cross_maintenance_margin_used(), Some(dec(10)));
    assert_eq!(state.withdrawable(), dec(1_140));
    assert_eq!(state.risk_state(), RiskState::Normal);

    assert_eq!(state.position_risks().len(), 2);
    let btc_risk = state.position_risk_of("BTC-PERP").expect("BTC risk snapshot must exist");
    assert_eq!(btc_risk.position.position_key, "sub-1-BTC-PERP");
    assert_eq!(btc_risk.mark_price, dec(100));
    assert_eq!(btc_risk.position_value, dec(200));
    assert_eq!(btc_risk.unrealized_pnl, dec(0));
    assert_eq!(btc_risk.margin_used, dec(20));

    let eth_risk = state.position_risk_of("ETH-PERP").expect("ETH risk snapshot must exist");
    assert_eq!(eth_risk.position.side(), HyperliquidPerpPositionSide::Short);
    assert_eq!(eth_risk.mark_price, dec(50));
    assert_eq!(eth_risk.position_value, dec(200));
    assert_eq!(eth_risk.unrealized_pnl, dec(200));
    assert_eq!(eth_risk.margin_used, dec(40));
}

#[test]
fn given_active_open_order_reservation_when_calculated_then_margin_used_increases() {
    let mut calc_input = input(
        vec![position(
            0,
            "BTC-PERP",
            2,
            HyperliquidPerpPositionSide::Long,
            HyperliquidPerpMarginMode::Cross,
            0,
        )],
        1_000,
    );
    calc_input.open_order_margin_reservations = vec![active_perp_reservation(300)];

    let state = match PerpClearinghouseState::calculate_from_facts(calc_input) {
        Ok(state) => state,
        Err(error) => panic!("calculation must succeed: {error}"),
    };

    assert_eq!(state.margin_summary().total_margin_used(), dec(320));
    assert_eq!(state.withdrawable(), dec(680));
}

#[test]
fn given_account_value_at_maintenance_when_calculated_then_state_is_liquidation() {
    let calc_input = input(
        vec![position(
            0,
            "BTC-PERP",
            2,
            HyperliquidPerpPositionSide::Long,
            HyperliquidPerpMarginMode::Cross,
            0,
        )],
        10,
    );

    let state = match PerpClearinghouseState::calculate_from_facts(calc_input) {
        Ok(state) => state,
        Err(error) => panic!("calculation must succeed: {error}"),
    };

    assert_eq!(state.cross_maintenance_margin_used(), Some(dec(10)));
    assert_eq!(state.risk_state(), RiskState::Liquidation);
}

#[test]
fn given_withdrawable_at_threshold_when_calculated_then_state_is_reduce_only() {
    let mut calc_input = input(
        vec![position(
            0,
            "BTC-PERP",
            2,
            HyperliquidPerpPositionSide::Long,
            HyperliquidPerpMarginMode::Cross,
            0,
        )],
        120,
    );
    calc_input.risk_policy.reduce_only_withdrawable_threshold = dec(100);

    let state = match PerpClearinghouseState::calculate_from_facts(calc_input) {
        Ok(state) => state,
        Err(error) => panic!("calculation must succeed: {error}"),
    };

    assert_eq!(state.withdrawable(), dec(100));
    assert_eq!(state.risk_state(), RiskState::ReduceOnly);
}

#[test]
fn given_missing_mark_or_risk_rule_when_calculated_then_business_error_is_returned() {
    let base_input = input(
        vec![position(
            0,
            "BTC-PERP",
            2,
            HyperliquidPerpPositionSide::Long,
            HyperliquidPerpMarginMode::Cross,
            0,
        )],
        1_000,
    );

    let mut missing_mark = base_input.clone();
    missing_mark.market_marks.clear();
    assert_eq!(
        PerpClearinghouseState::calculate_from_facts(missing_mark),
        Err(PerpClearinghouseStateCalcError::MissingMarketMark { asset: 0 })
    );

    let mut missing_rule = base_input;
    missing_rule.risk_rules.clear();
    assert_eq!(
        PerpClearinghouseState::calculate_from_facts(missing_rule),
        Err(PerpClearinghouseStateCalcError::MissingRiskRule { asset: 0 })
    );
}

#[test]
fn given_negative_input_when_calculated_then_business_error_is_returned() {
    let mut calc_input = input(vec![], 750);
    calc_input.market_marks = vec![PerpMarketMark { asset: 0, mark_price: dec(-1) }];

    assert_eq!(
        PerpClearinghouseState::calculate_from_facts(calc_input),
        Err(PerpClearinghouseStateCalcError::NegativeInput { field: "market_mark.mark_price" })
    );
}

#[test]
fn given_no_positions_and_collateral_when_calculated_then_withdrawable_is_collateral() {
    let state = match PerpClearinghouseState::calculate_from_facts(input(vec![], 750)) {
        Ok(state) => state,
        Err(error) => panic!("calculation must succeed: {error}"),
    };

    assert_eq!(state.margin_summary().total_position_notional(), dec(0));
    assert_eq!(state.margin_summary().total_margin_used(), dec(0));
    assert_eq!(state.margin_summary().account_value(), dec(750));
    assert_eq!(state.withdrawable(), dec(750));
    assert_eq!(state.risk_state(), RiskState::Normal);
}
