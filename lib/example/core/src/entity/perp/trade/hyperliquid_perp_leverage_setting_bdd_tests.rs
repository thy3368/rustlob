use super::hyperliquid_perp_leverage_setting::{
    HyperliquidPerpLeverageSetting, HyperliquidPerpLeverageSettingError,
};
use super::hyperliquid_perp_position::HyperliquidPerpMarginMode;

fn cross_setting() -> HyperliquidPerpLeverageSetting {
    HyperliquidPerpLeverageSetting::new(
        "trader-1".to_string(),
        7,
        HyperliquidPerpMarginMode::Cross,
        5,
        3,
    )
}

#[test]
fn given_cross_leverage_setting_when_leverage_is_updated_then_only_leverage_and_version_change()
-> Result<(), HyperliquidPerpLeverageSettingError> {
    let before = cross_setting();

    let after = before.update_leverage(10)?;

    assert_eq!(after.setting_id, before.setting_id);
    assert_eq!(after.account_id, before.account_id);
    assert_eq!(after.asset, before.asset);
    assert_eq!(after.margin_mode, before.margin_mode);
    assert_eq!(after.leverage, 10);
    assert_eq!(after.version, 4);
    Ok(())
}

#[test]
fn given_leverage_setting_when_zero_leverage_is_requested_then_entity_rejects_it() {
    let before = cross_setting();

    let result = before.update_leverage(0);

    assert_eq!(result, Err(HyperliquidPerpLeverageSettingError::InvalidLeverage));
}

#[test]
fn given_max_version_leverage_setting_when_leverage_is_updated_then_entity_rejects_overflow() {
    let before = HyperliquidPerpLeverageSetting::new(
        "trader-1".to_string(),
        7,
        HyperliquidPerpMarginMode::Cross,
        5,
        u64::MAX,
    );

    let result = before.update_leverage(10);

    assert_eq!(result, Err(HyperliquidPerpLeverageSettingError::ArithmeticOverflow));
}

#[test]
fn given_leverage_settings_when_identity_queries_run_then_matching_and_non_matching_are_distinct() {
    let cross = cross_setting();
    let isolated = HyperliquidPerpLeverageSetting::new(
        "trader-1".to_string(),
        7,
        HyperliquidPerpMarginMode::Isolated,
        3,
        2,
    );

    assert!(cross.belongs_to_account("trader-1"));
    assert!(!cross.belongs_to_account("trader-2"));
    assert!(cross.trades_asset(7));
    assert!(!cross.trades_asset(8));
    assert!(cross.matches_margin_mode(HyperliquidPerpMarginMode::Cross));
    assert!(!cross.matches_margin_mode(HyperliquidPerpMarginMode::Isolated));
    assert!(isolated.matches_margin_mode(HyperliquidPerpMarginMode::Isolated));
    assert_ne!(cross.setting_id, isolated.setting_id);
}
