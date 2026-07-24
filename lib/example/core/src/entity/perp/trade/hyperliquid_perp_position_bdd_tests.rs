use super::hyperliquid_perp_position::{
    HyperliquidPerpMarginMode, HyperliquidPerpPosition, HyperliquidPerpPositionError,
    HyperliquidPerpPositionStatus,
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

fn open_from_empty_slot(
    signed_size: i64,
    entry_price: u64,
) -> Result<
    (
        HyperliquidPerpPosition,
        super::hyperliquid_perp_position::HyperliquidPerpPositionTradeOutcome,
    ),
    HyperliquidPerpPositionError,
> {
    let slot = empty_position();
    HyperliquidPerpPosition::open_position(
        slot.position_key.clone(),
        slot.account_id.clone(),
        slot.perp_asset_id,
        slot.coin.clone(),
        signed_size,
        entry_price,
        slot.leverage_value,
        slot.margin_mode,
    )
}

fn open_position(signed_size: i64, entry_price: u64, leverage: u64) -> HyperliquidPerpPosition {
    HyperliquidPerpPosition::new(
        "trader-1:7".to_string(),
        "trader-1".to_string(),
        7,
        "BTC-PERP".to_string(),
        signed_size,
        entry_price,
        leverage,
        HyperliquidPerpMarginMode::Cross,
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
        0,
        0,
        leverage,
        HyperliquidPerpMarginMode::Cross,
        11,
        3,
    )
}

#[test]
fn given_empty_slot_when_open_long_trade_settles_then_long_position_opens()
-> Result<(), HyperliquidPerpPositionError> {
    let slot = empty_position();

    let (position, outcome) = open_from_empty_slot(3 as i64, 100)?;

    assert!(position.is_long());
    assert_eq!(position.qty(), 3);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin().unwrap_or(0), 30);
    assert_eq!(position.lifecycle_status(), HyperliquidPerpPositionStatus::Open);
    assert_eq!(position.version, 1);
    assert_eq!(slot.lifecycle_status(), HyperliquidPerpPositionStatus::EmptySlot);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 30);
    Ok(())
}

#[test]
fn given_empty_slot_when_open_short_trade_settles_then_short_position_opens()
-> Result<(), HyperliquidPerpPositionError> {
    let (position, outcome) = open_from_empty_slot(-(4 as i64), 100)?;

    assert!(position.is_short());
    assert_eq!(position.qty(), 4);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin().unwrap_or(0), 40);
    assert_eq!(position.lifecycle_status(), HyperliquidPerpPositionStatus::Open);
    assert_eq!(position.version, 1);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 40);
    Ok(())
}

#[test]
fn given_open_long_when_add_long_trade_settles_then_long_position_increases_with_weighted_entry_price()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(3 as i64, 100, 10);

    let outcome = position.increase_position(2 as i64, 130)?;

    assert!(position.is_long());
    assert_eq!(position.qty(), 5);
    assert_eq!(position.entry_price, 112);
    assert_eq!(position.required_margin().unwrap_or(0), 56);
    assert_eq!(position.version, 3);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 26);
    Ok(())
}

#[test]
fn given_open_short_when_add_short_trade_settles_then_short_position_increases_with_weighted_entry_price()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(-(3 as i64), 100, 10);

    let outcome = position.increase_position(-(2 as i64), 130)?;

    assert!(position.is_short());
    assert_eq!(position.qty(), 5);
    assert_eq!(position.entry_price, 112);
    assert_eq!(position.required_margin().unwrap_or(0), 56);
    assert_eq!(position.version, 3);
    assert_eq!(outcome.realized_pnl_delta, 0);
    assert_eq!(outcome.required_margin_delta, 26);
    Ok(())
}

#[test]
fn given_open_long_when_close_long_trade_partially_settles_then_long_position_reduces_and_realizes_pnl()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(5 as i64, 100, 10);

    let outcome = position.reduce_position(-(2 as i64), 130)?;

    assert!(position.is_long());
    assert_eq!(position.qty(), 3);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin().unwrap_or(0), 30);
    assert_eq!(position.cumulative_realized_pnl, 67);
    assert_eq!(outcome.realized_pnl_delta, 60);
    assert_eq!(outcome.required_margin_delta, -20);
    Ok(())
}

#[test]
fn given_open_short_when_close_short_trade_partially_settles_then_short_position_reduces_and_realizes_pnl()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(-(5 as i64), 100, 10);

    let outcome = position.reduce_position(2 as i64, 70)?;

    assert!(position.is_short());
    assert_eq!(position.qty(), 3);
    assert_eq!(position.entry_price, 100);
    assert_eq!(position.required_margin().unwrap_or(0), 30);
    assert_eq!(position.cumulative_realized_pnl, 67);
    assert_eq!(outcome.realized_pnl_delta, 60);
    assert_eq!(outcome.required_margin_delta, -20);
    Ok(())
}

#[test]
fn given_open_long_when_close_long_trade_fully_settles_then_position_closes_and_margin_is_released()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(3 as i64, 100, 10);

    let outcome = position.close_position(-(3 as i64), 80)?;

    assert!(position.is_flat());
    assert_eq!(position.qty(), 0);
    assert_eq!(position.entry_price, 0);
    assert_eq!(position.required_margin().unwrap_or(0), 0);
    assert_eq!(position.lifecycle_status(), HyperliquidPerpPositionStatus::Closed);
    assert_eq!(position.cumulative_realized_pnl, -53);
    assert_eq!(outcome.realized_pnl_delta, -60);
    assert_eq!(outcome.required_margin_delta, -30);
    Ok(())
}

#[test]
fn given_open_short_when_close_short_trade_fully_settles_then_position_closes_and_margin_is_released()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(-(3 as i64), 100, 10);

    let outcome = position.close_position(3 as i64, 120)?;

    assert!(position.is_flat());
    assert_eq!(position.qty(), 0);
    assert_eq!(position.entry_price, 0);
    assert_eq!(position.required_margin().unwrap_or(0), 0);
    assert_eq!(position.lifecycle_status(), HyperliquidPerpPositionStatus::Closed);
    assert_eq!(position.cumulative_realized_pnl, -53);
    assert_eq!(outcome.realized_pnl_delta, -60);
    assert_eq!(outcome.required_margin_delta, -30);
    Ok(())
}

#[test]
fn given_position_when_mark_moves_then_unrealized_pnl_is_signed() {
    let long = open_position(2 as i64, 100, 10);
    let short = open_position(-(2 as i64), 100, 10);
    let flat = closed_position(10);
    let max_sized_long = open_position(i64::MAX, 0, 10);

    assert_eq!(long.notional_at(130), Some(260));
    assert_eq!(long.unrealized_pnl_at(130), Some(60));
    assert_eq!(long.unrealized_pnl_at(70), Some(-60));
    assert_eq!(short.unrealized_pnl_at(70), Some(60));
    assert_eq!(short.unrealized_pnl_at(130), Some(-60));
    assert_eq!(flat.unrealized_pnl_at(130), Some(0));
    assert_eq!(max_sized_long.unrealized_pnl_at(2), None);
}

#[test]
fn given_invalid_trade_facts_when_trade_settles_then_entity_rejects_without_state_change() {
    let position = open_position(2 as i64, 100, 10);

    let mut zero_qty = position.clone();
    assert_eq!(
        zero_qty.increase_position(0 as i64, 100),
        Err(HyperliquidPerpPositionError::InvalidTradeQty)
    );
    assert_eq!(zero_qty, position);

    let mut zero_price = position.clone();
    assert_eq!(
        zero_price.increase_position(1 as i64, 0),
        Err(HyperliquidPerpPositionError::InvalidTradePrice)
    );
    assert_eq!(zero_price, position);

    let mut zero_signed_size = position.clone();
    assert_eq!(
        zero_signed_size.reduce_position(0, 100),
        Err(HyperliquidPerpPositionError::InvalidTradeQty)
    );
    assert_eq!(zero_signed_size, position);
}

#[test]
fn given_wrong_trade_direction_or_state_when_position_behavior_runs_then_entity_rejects_without_state_change()
 {
    let position = open_position(2 as i64, 100, 10);

    let mut increase_with_reverse = position.clone();
    assert_eq!(
        increase_with_reverse.increase_position(-(1 as i64), 100),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(increase_with_reverse, position);

    let mut reduce_with_same_side = position.clone();
    assert_eq!(
        reduce_with_same_side.reduce_position(1 as i64, 100),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(reduce_with_same_side, position);

    let mut reduce_with_full_qty = position.clone();
    assert_eq!(
        reduce_with_full_qty.reduce_position(-(2 as i64), 100),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(reduce_with_full_qty, position);

    let mut close_with_partial_qty = position.clone();
    assert_eq!(
        close_with_partial_qty.close_position(-(1 as i64), 100),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(close_with_partial_qty, position);

    let mut empty = empty_position();
    assert_eq!(
        empty.increase_position(1 as i64, 100),
        Err(HyperliquidPerpPositionError::InconsistentState)
    );
    assert_eq!(empty, empty_position());
}

#[test]
fn given_inconsistent_position_when_trade_settles_then_entity_rejects_without_state_change() {
    let mut position = open_position(2 as i64, 100, 10);
    position.signed_size = 0;
    let before = position.clone();

    let result = position.increase_position(1 as i64, 100);

    assert_eq!(result, Err(HyperliquidPerpPositionError::InconsistentState));
    assert_eq!(position, before);
}

#[test]
fn given_open_position_when_matching_leverage_facts_apply_then_margin_is_recomputed_and_version_advances()
-> Result<(), HyperliquidPerpPositionError> {
    let mut position = open_position(3 as i64, 100, 5);

    let outcome =
        position.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 10)?;

    assert_eq!(position.leverage_value, 10);
    assert_eq!(position.required_margin().unwrap_or(0), 30);
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
    assert_eq!(empty.lifecycle_status(), HyperliquidPerpPositionStatus::Closed);
    assert_eq!(empty.leverage_value, 20);
    assert_eq!(empty.required_margin().unwrap_or(0), 0);
    assert_eq!(empty.version, 1);
    assert_eq!(empty_outcome.required_margin_delta, 0);

    let mut closed = closed_position(5);
    let closed_outcome =
        closed.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 20)?;
    assert_eq!(closed.lifecycle_status(), HyperliquidPerpPositionStatus::Closed);
    assert_eq!(closed.leverage_value, 20);
    assert_eq!(closed.required_margin().unwrap_or(0), 0);
    assert_eq!(closed.version, 4);
    assert_eq!(closed_outcome.required_margin_delta, 0);
    Ok(())
}

#[test]
fn given_mismatched_leverage_facts_when_apply_then_entity_rejects_without_state_change() {
    let position = open_position(3 as i64, 100, 5);

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
    let mut position = open_position(3 as i64, 100, 5);
    let before = position.clone();

    let result =
        position.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 0);

    assert_eq!(result, Err(HyperliquidPerpPositionError::InvalidLeverage));
    assert_eq!(position, before);
}
