use example_core_entity::account::coin::Coin;
use example_core_entity::account::symbol::Symbol;
use example_core_entity::{
    OrderSide, PerpTradeKind, PerpTradeV3, PerpTradeV3State, TradeBase, UserId,
};
use rust_decimal::{Decimal, dec};

fn trade_base(taker: u64, maker: u64, side: OrderSide, price: Decimal, size: Decimal) -> TradeBase {
    TradeBase {
        trade_id: 42,
        symbol: Symbol::new(7, Coin::BTC, Coin::USDT),
        taker_user: UserId(taker),
        maker_user: UserId(maker),
        taker_order_id: 1001,
        maker_order_id: 2001,
        taker_side: side,
        price,
        size,
        taker_fee: dec!(0.5),
        maker_fee: dec!(0.2),
        created_at: 1_700_000_000,
        updated_at: 1_700_000_001,
        version: 3,
    }
}

fn normal_trade() -> PerpTradeV3 {
    PerpTradeV3 {
        trade_base: trade_base(1, 2, OrderSide::Buy, dec!(100), dec!(2)),
        state: PerpTradeV3State::Unsettled,
        kind: PerpTradeKind::Normal,
        reference_id: None,
        taker_start_position: dec!(0),
        maker_start_position: dec!(-2),
        taker_closed_pnl: Decimal::ZERO,
        maker_closed_pnl: Decimal::ZERO,
        liquidation: None,
    }
}

fn liquidation_trade() -> PerpTradeV3 {
    PerpTradeV3 {
        trade_base: trade_base(3, 4, OrderSide::Sell, dec!(95), dec!(1)),
        state: PerpTradeV3State::Unsettled,
        kind: PerpTradeKind::Liquidation,
        reference_id: Some("liq-001".to_string()),
        taker_start_position: dec!(-1),
        maker_start_position: dec!(1),
        taker_closed_pnl: dec!(-5),
        maker_closed_pnl: dec!(5),
        liquidation: None,
    }
}

fn insurance_fund_transfer() -> PerpTradeV3 {
    PerpTradeV3 {
        trade_base: trade_base(5, 6, OrderSide::Sell, Decimal::ONE, dec!(10)),
        state: PerpTradeV3State::Unsettled,
        kind: PerpTradeKind::InsuranceFundTransfer,
        reference_id: Some("allocation-001".to_string()),
        taker_start_position: Decimal::ZERO,
        maker_start_position: Decimal::ZERO,
        taker_closed_pnl: Decimal::ZERO,
        maker_closed_pnl: Decimal::ZERO,
        liquidation: None,
    }
}

fn adl_trade() -> PerpTradeV3 {
    PerpTradeV3 {
        trade_base: trade_base(7, 8, OrderSide::Buy, dec!(110), dec!(3)),
        state: PerpTradeV3State::Unsettled,
        kind: PerpTradeKind::Adl,
        reference_id: Some("adl-001".to_string()),
        taker_start_position: dec!(3),
        maker_start_position: dec!(-3),
        taker_closed_pnl: dec!(6),
        maker_closed_pnl: dec!(-6),
        liquidation: None,
    }
}

#[test]
fn normal_trade_preserves_market_execution_facts() {
    let trade = normal_trade();
    assert_eq!(trade.kind, PerpTradeKind::Normal);
    assert_eq!(trade.reference_id, None);
    assert_eq!(trade.trade_base.taker_user, UserId(1));
    assert_eq!(trade.trade_base.maker_user, UserId(2));
    assert_eq!(trade.trade_base.taker_side, OrderSide::Buy);
    assert_eq!(trade.trade_base.price, dec!(100));
    assert_eq!(trade.trade_base.size, dec!(2));
    assert_eq!(trade.trade_base.taker_fee, dec!(0.5));
    assert_eq!(trade.taker_closed_pnl, Decimal::ZERO);
    assert_eq!(trade.trade_base.version, 3);
    assert_eq!(trade.trade_base.created_at, 1_700_000_000);
    assert_eq!(trade.trade_base.updated_at, 1_700_000_001);
}

#[test]
fn liquidation_trade_is_traceable_without_new_trade_type() {
    let trade = liquidation_trade();
    assert_eq!(trade.kind, PerpTradeKind::Liquidation);
    assert_eq!(trade.reference_id.as_deref(), Some("liq-001"));
    assert_eq!(trade.trade_base.taker_user, UserId(3));
    assert_eq!(trade.trade_base.maker_user, UserId(4));
    assert_eq!(trade.trade_base.price, dec!(95));
    assert_eq!(trade.trade_base.size, dec!(1));
    assert_eq!(trade.taker_closed_pnl, dec!(-5));
    assert_eq!(trade.maker_closed_pnl, dec!(5));
    assert_eq!(trade.trade_base.taker_fee, dec!(0.5));
    assert_eq!(trade.trade_base.maker_fee, dec!(0.2));
}

#[test]
fn insurance_fund_transfer_is_a_zero_pnl_unit_price_fact() {
    let trade = insurance_fund_transfer();
    assert_eq!(trade.kind, PerpTradeKind::InsuranceFundTransfer);
    assert_eq!(trade.reference_id.as_deref(), Some("allocation-001"));
    assert_eq!(trade.trade_base.taker_user, UserId(5));
    assert_eq!(trade.trade_base.maker_user, UserId(6));
    assert_eq!(trade.trade_base.price, Decimal::ONE);
    assert_eq!(trade.trade_base.size, dec!(10));
    assert_eq!(trade.trade_base.taker_side, OrderSide::Sell);
    assert_eq!(trade.taker_closed_pnl, Decimal::ZERO);
    assert_eq!(trade.maker_closed_pnl, Decimal::ZERO);
    assert_eq!(trade.trade_base.taker_order_id, 1001);
    assert_eq!(trade.trade_base.maker_order_id, 2001);
}

#[test]
fn adl_trade_is_traceable_and_keeps_pnl_semantics() {
    let trade = adl_trade();
    assert_eq!(trade.kind, PerpTradeKind::Adl);
    assert_eq!(trade.reference_id.as_deref(), Some("adl-001"));
    assert_eq!(trade.trade_base.taker_user, UserId(7));
    assert_eq!(trade.trade_base.maker_user, UserId(8));
    assert_eq!(trade.trade_base.taker_side, OrderSide::Buy);
    assert_eq!(trade.trade_base.price, dec!(110));
    assert_eq!(trade.trade_base.size, dec!(3));
    assert_eq!(trade.taker_closed_pnl, dec!(6));
    assert_eq!(trade.maker_closed_pnl, dec!(-6));
    assert_eq!(trade.trade_base.version, 3);
}
