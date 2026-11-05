use common_entity::{Entity, EntityFieldChange, FieldDiff, Shard};
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

impl FieldDiff for PerpTradeV3 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let trade = &self.trade_base;
        vec![
            EntityFieldChange::new("symbol", "", trade.symbol.id().to_string()),
            EntityFieldChange::new("taker_user", "", trade.taker_user.0.to_string()),
            EntityFieldChange::new("maker_user", "", trade.maker_user.0.to_string()),
            EntityFieldChange::new("price", "", trade.price.to_string()),
            EntityFieldChange::new("size", "", trade.size.to_string()),
            EntityFieldChange::new("taker_fee", "", trade.taker_fee.to_string()),
            EntityFieldChange::new("maker_fee", "", trade.maker_fee.to_string()),
            EntityFieldChange::new("kind", "", format!("{:?}", self.kind)),
            EntityFieldChange::new("created_at", "", trade.created_at.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Shard for PerpTradeV3 {
    fn partition(&self) -> u8 {
        (self.trade_base.trade_id % 256) as u8
    }
}

impl Entity for PerpTradeV3 {
    fn entity_id(&self) -> u64 {
        self.trade_base.trade_id
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8 {
        10
    }

    fn lsn(&self) -> u64 {
        self.trade_base.version
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "price" | "size" | "taker_fee" | "maker_fee" => 1,
            _ => 0,
        }
    }
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
