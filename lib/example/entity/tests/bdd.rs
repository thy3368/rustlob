use example_core_entity::{
    Reservation, ReservationKind, ReservationMarketKind, SpotOrderSide, SpotOrderV2BehaviorError,
};

fn test_principal_reservation(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    qty: u64,
    order_price: u64,
) -> Result<Reservation, SpotOrderV2BehaviorError> {
    let (reservation_kind, asset_id, original_amount) = match side {
        SpotOrderSide::Buy => (
            ReservationKind::SpotBuyQuote,
            "USDT",
            qty.checked_mul(order_price).ok_or(SpotOrderV2BehaviorError::ArithmeticOverflow)?,
        ),
        SpotOrderSide::Sell => (ReservationKind::SpotSellBase, "BTC", qty),
    };
    Reservation::new(
        format!("reservation:{order_id}:principal"),
        account_id.to_string(),
        order_id.to_string(),
        ReservationMarketKind::Spot,
        reservation_kind,
        asset_id.to_string(),
        original_amount,
    )
    .map_err(SpotOrderV2BehaviorError::Reservation)
}

#[path = "bdd/account/balance_bdd_tests.rs"]
mod account_balance_bdd_tests;
#[path = "bdd/account/balance_ledger_entry_v2_bdd_tests.rs"]
mod account_balance_ledger_entry_v2_bdd_tests;
#[path = "bdd/account/settlement_transfer_voucher_bdd_tests.rs"]
mod account_settlement_transfer_voucher_bdd_tests;

#[path = "bdd/spot/spot_trade/spot_trade_bdd_notional_quote.rs"]
mod spot_trade_bdd_notional_quote;
#[path = "bdd/spot/spot_trade/spot_trade_bdd_settlement_transfer_voucher.rs"]
mod spot_trade_bdd_settlement_transfer_voucher;

#[path = "bdd/perp/fund/hyperliquid_perp_funding_settlement_bdd_tests.rs"]
mod perp_funding_settlement_bdd_tests;
#[path = "bdd/perp/trade/hyperliquid_perp_leverage_setting_bdd_tests.rs"]
mod perp_leverage_setting_bdd_tests;
#[path = "bdd/perp/trade/hyperliquid_perp_order_business_intent_bdd_tests.rs"]
mod perp_order_business_intent_bdd_tests;
#[path = "bdd/perp/trade/hyperliquid_perp_position_bdd_tests.rs"]
mod perp_position_bdd_tests;

#[path = "bdd/option/cex_option_bdd_tests.rs"]
mod option_cex_option_bdd_tests;

#[path = "bdd/reservation/reservation_bdd_happy_path.rs"]
mod reservation_bdd_happy_path;

#[path = "bdd/hyperliquid_account/perp_bdd_calculate_from_facts.rs"]
mod hyperliquid_account_perp_bdd_calculate_from_facts;
