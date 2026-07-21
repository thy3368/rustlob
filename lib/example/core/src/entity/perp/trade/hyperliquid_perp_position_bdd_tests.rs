use super::hyperliquid_perp_position::{
    HyperliquidPerpMarginMode, HyperliquidPerpPosition, HyperliquidPerpPositionError,
    HyperliquidPerpPositionSide, HyperliquidPerpPositionStatus, required_position_margin,
};

fn empty_position() -> HyperliquidPerpPosition {
    HyperliquidPerpPosition::empty_slot(
        "trader-1:7".to_string(),
        "trader-1".to_string(),
        7,
        "BTC-PERP".to_string(),
        10,
    )
}

fn open_position(
    side: HyperliquidPerpPositionSide,
    qty: u64,
    entry_price: u64,
    leverage: u64,
) -> HyperliquidPerpPosition {
    HyperliquidPerpPosition::new(
        "trader-1:7".to_string(),
        "trader-1".to_string(),
        7,
        "BTC-PERP".to_string(),
        side,
        qty,
        entry_price,
        leverage,
        HyperliquidPerpMarginMode::Cross,
        required_position_margin(qty, entry_price, leverage).unwrap(),
        Some(45_000),
        123,
        7,
        2,
    )
}

fn closed_position(leverage: u64) -> HyperliquidPerpPosition {
    HyperliquidPerpPosition::new(
        "trader-1:7".to_string(),
        "trader-1".to_string(),
        7,
        "BTC-PERP".to_string(),
        HyperliquidPerpPositionSide::Flat,
        0,
        0,
        leverage,
        HyperliquidPerpMarginMode::Cross,
        0,
        None,
        0,
        11,
        3,
    )
}

#[test]
fn given_empty_slot_when_open_long_trade_settles_then_long_position_opens()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = empty_position();

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Long, 3, 100)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Long);
    assert_eq!(position.qty, 3);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin, 30);
    assert_eq!(position.status, HyperliquidPerpPositionStatus::Open);
    assert_eq!(position.version, 1);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 30);
    Ok(())
}

#[test]
fn given_empty_slot_when_open_short_trade_settles_then_short_position_opens()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = empty_position();

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Short, 4, 100)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Short);
    assert_eq!(position.qty, 4);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin, 40);
    assert_eq!(position.status, HyperliquidPerpPositionStatus::Open);
    assert_eq!(position.version, 1);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 40);
    Ok(())
}

#[test]
fn given_open_long_when_add_long_trade_settles_then_long_position_increases_with_weighted_entry_price()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Long, 2, 130)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Long);
    assert_eq!(position.qty, 5);
    assert_eq!(position.entry_price, 112);
    assert_eq!(position.required_margin, 56);
    assert_eq!(position.version, 3);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 26);
    Ok(())
}

#[test]
fn given_open_short_when_add_short_trade_settles_then_short_position_increases_with_weighted_entry_price()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Short, 3, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Short, 2, 130)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Short);
    assert_eq!(position.qty, 5);
    assert_eq!(position.entry_price, 112);
    assert_eq!(position.required_margin, 56);
    assert_eq!(position.version, 3);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 26);
    Ok(())
}

#[test]
fn given_open_long_when_close_long_trade_partially_settles_then_long_position_reduces_and_realizes_pnl()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 5, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Short, 2, 130)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Long);
    assert_eq!(position.qty, 3);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin, 30);
    assert_eq!(position.realized_pnl, 67);
    assert_eq!(outcome.realized_pnl_delta, 60);
    assert_eq!(outcome.required_margin_delta, -20);
    Ok(())
}

#[test]
fn given_open_short_when_close_short_trade_partially_settles_then_short_position_reduces_and_realizes_pnl()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Short, 5, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Long, 2, 70)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Short);
    assert_eq!(position.qty, 3);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin, 30);
    assert_eq!(position.realized_pnl, 67);
    assert_eq!(outcome.realized_pnl_delta, 60);
    assert_eq!(outcome.required_margin_delta, -20);
    Ok(())
}

#[test]
fn given_open_long_when_close_long_trade_fully_settles_then_position_closes_and_margin_is_released()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Short, 3, 80)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Flat);
    assert_eq!(position.qty, 0);
    assert_eq!(position.entry_price, 0);
    assert_eq!(position.required_margin, 0);
    assert_eq!(position.status, HyperliquidPerpPositionStatus::Closed);
    assert_eq!(position.realized_pnl, -53);
    assert_eq!(outcome.realized_pnl_delta, -60);
    assert_eq!(outcome.required_margin_delta, -30);
    Ok(())
}

#[test]
fn given_open_short_when_close_short_trade_fully_settles_then_position_closes_and_margin_is_released()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Short, 3, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Long, 3, 120)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Flat);
    assert_eq!(position.qty, 0);
    assert_eq!(position.entry_price, 0);
    assert_eq!(position.required_margin, 0);
    assert_eq!(position.status, HyperliquidPerpPositionStatus::Closed);
    assert_eq!(position.realized_pnl, -53);
    assert_eq!(outcome.realized_pnl_delta, -60);
    assert_eq!(outcome.required_margin_delta, -30);
    Ok(())
}

#[test]
fn given_open_long_when_short_trade_exceeds_position_then_position_flips_to_short()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 2, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Short, 5, 90)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Short);
    assert_eq!(position.qty, 3);
    assert_eq!(position.entry_price, 90);
    assert_eq!(position.required_margin, 27);
    assert_eq!(position.realized_pnl, -13);
    assert_eq!(outcome.realized_pnl_delta, -20);
    assert_eq!(outcome.required_margin_delta, 7);
    Ok(())
}

#[test]
fn given_open_short_when_long_trade_exceeds_position_then_position_flips_to_long()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Short, 2, 100, 10);

    let outcome = position.settle_trade(HyperliquidPerpPositionSide::Long, 5, 110)?;

    assert_eq!(position.side, HyperliquidPerpPositionSide::Long);
    assert_eq!(position.qty, 3);
    assert_eq!(position.entry_price, 110);
    assert_eq!(position.required_margin, 33);
    assert_eq!(position.realized_pnl, -13);
    assert_eq!(outcome.realized_pnl_delta, -20);
    assert_eq!(outcome.required_margin_delta, 13);
    Ok(())
}

#[test]
fn given_position_with_risk_snapshot_when_trade_settles_then_risk_snapshot_fields_are_cleared()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 2, 100, 10);
    assert_eq!(position.liquidation_price, Some(45_000));
    assert_eq!(position.unrealized_pnl, 123);

    position.settle_trade(HyperliquidPerpPositionSide::Long, 1, 100)?;

    assert_eq!(position.liquidation_price, None);
    assert_eq!(position.unrealized_pnl, 0);
    Ok(())
}

#[test]
fn given_invalid_trade_facts_when_trade_settles_then_entity_rejects_without_state_change() {
    let position = open_position(HyperliquidPerpPositionSide::Long, 2, 100, 10);

    let mut zero_qty = position.clone();
    assert_eq!(
        zero_qty.settle_trade(HyperliquidPerpPositionSide::Long, 0, 100),
        Err(HyperliquidPerpPositionError::InvalidTradeQty)
    );
    assert_eq!(zero_qty, position);

    let mut zero_price = position.clone();
    assert_eq!(
        zero_price.settle_trade(HyperliquidPerpPositionSide::Long, 1, 0),
        Err(HyperliquidPerpPositionError::InvalidTradePrice)
    );
    assert_eq!(zero_price, position);

    let mut flat_side = position.clone();
    assert_eq!(
        flat_side.settle_trade(HyperliquidPerpPositionSide::Flat, 1, 100),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(flat_side, position);
}

#[test]
fn given_inconsistent_position_when_trade_settles_then_entity_rejects_without_state_change() {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 2, 100, 10);
    position.required_margin = 19;
    let before = position.clone();

    let result = position.settle_trade(HyperliquidPerpPositionSide::Long, 1, 100);

    assert_eq!(result, Err(HyperliquidPerpPositionError::InconsistentState));
    assert_eq!(position, before);
}

#[test]
fn given_open_position_when_matching_leverage_facts_apply_then_margin_is_recomputed_and_version_advances()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 5);

    let outcome =
        position.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 10)?;

    assert_eq!(position.leverage, 10);
    assert_eq!(position.required_margin, 30);
    assert_eq!(position.version, 3);
    assert_eq!(outcome.required_margin_delta, -30);
    Ok(())
}

#[test]
fn given_empty_or_closed_position_when_matching_leverage_facts_apply_then_margin_stays_zero()
-> Result<(), HyperliquidPerpPositionError> {
    let mut empty = empty_position();
    let empty_outcome =
        empty.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 20)?;
    assert_eq!(empty.status, HyperliquidPerpPositionStatus::EmptySlot);
    assert_eq!(empty.leverage, 20);
    assert_eq!(empty.required_margin, 0);
    assert_eq!(empty.version, 1);
    assert_eq!(empty_outcome.required_margin_delta, 0);

    let mut closed = closed_position(5);
    let closed_outcome =
        closed.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 20)?;
    assert_eq!(closed.status, HyperliquidPerpPositionStatus::Closed);
    assert_eq!(closed.leverage, 20);
    assert_eq!(closed.required_margin, 0);
    assert_eq!(closed.version, 4);
    assert_eq!(closed_outcome.required_margin_delta, 0);
    Ok(())
}

#[test]
fn given_mismatched_leverage_facts_when_apply_then_entity_rejects_without_state_change() {
    let position = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 5);

    let mut wrong_account = position.clone();
    assert_eq!(
        wrong_account.apply_leverage_setting("other", 7, HyperliquidPerpMarginMode::Cross, 10),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(wrong_account, position);

    let mut wrong_asset = position.clone();
    assert_eq!(
        wrong_asset.apply_leverage_setting("trader-1", 8, HyperliquidPerpMarginMode::Cross, 10),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(wrong_asset, position);

    let mut wrong_mode = position.clone();
    assert_eq!(
        wrong_mode.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Isolated, 10),
        Err(HyperliquidPerpPositionError::MarginModeMismatch)
    );
    assert_eq!(wrong_mode, position);
}

#[test]
fn given_zero_leverage_when_apply_then_entity_rejects_without_state_change() {
    let mut position = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 5);
    let before = position.clone();

    let result =
        position.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 0);

    assert_eq!(result, Err(HyperliquidPerpPositionError::InvalidLeverage));
    assert_eq!(position, before);
}
