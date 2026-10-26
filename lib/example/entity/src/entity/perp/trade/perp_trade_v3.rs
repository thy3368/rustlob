use rust_decimal::Decimal;

use crate::entity::common::common_trade::TradeBase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerpTradeKind {
    Normal,
    Liquidation,
    InsuranceFundTransfer,
    Adl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpTradeV3 {
    pub trade_base: TradeBase,

    pub kind: PerpTradeKind,
    pub reference_id: Option<String>,

    pub taker_start_position: Decimal,

    pub maker_start_position: Decimal,

    /// 本次平仓实现盈亏（费前，开仓为 0）
    pub taker_closed_pnl: Decimal,
    /// 本次平仓实现盈亏（费前，开仓为 0）
    pub maker_closed_pnl: Decimal,
    // ── 上下文 ──
    // pub margin_mode: MarginMode,
    // pub liquidation: Option<LiqMarker>, // 强平标记
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiqMarker {
    pub method: LiqMethod, // Market / Backstop
    pub mark_px: Decimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiqMethod {
    Market,
    Backstop, // HLP 的 liquidator vault 接盘
}
