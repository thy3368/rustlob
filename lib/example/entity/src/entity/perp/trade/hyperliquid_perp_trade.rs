use common_entity::{Entity, EntityFieldChange, FieldDiff, Fsm};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::account::coin::Coin;
use crate::account::symbol::Symbol;
use crate::common::UserId;
use crate::perp::trade::perp_position_v3::{
    PerpPositionV3, PerpPositionV3Error, PerpPositionV3Event, PerpPositionV3Output,
};
use crate::perp::trade::perp_trade_v3::{PerpTradeKind, PerpTradeV3};
use crate::{OrderSide, TradeBase};

const HYPERLIQUID_PERP_TRADE_ENTITY_TYPE: u8 = 10;
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HyperliquidPerpTradeError {
    #[error("invalid account id")]
    InvalidAccountId,
    #[error("invalid symbol")]
    InvalidSymbol,
    #[error("invalid trade")]
    InvalidTrade,
    #[error(transparent)]
    Position(#[from] PerpPositionV3Error),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpTrade {
    pub trade_id: String,
    pub match_id: String,
    pub asset: u32,
    pub symbol: String,
    pub taker_order_id: u64,
    pub maker_order_id: u64,
    pub taker_account_id: String,
    pub maker_account_id: String,
    pub taker_side: OrderSide,
    pub price: u64,
    pub qty: u64,
    pub created_at: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpTradePositionSettlement {
    pub taker_position_after: PerpPositionV3,
    pub maker_position_after: PerpPositionV3,
    pub taker_realized_pnl: Decimal,
    pub maker_realized_pnl: Decimal,
}
impl HyperliquidPerpTrade {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trade_id: String,
        match_id: String,
        asset: u32,
        symbol: String,
        taker_order_id: u64,
        maker_order_id: u64,
        taker_account_id: String,
        maker_account_id: String,
        taker_side: OrderSide,
        price: u64,
        qty: u64,
        executed_at_ms: u64,
    ) -> Self {
        Self {
            trade_id,
            match_id,
            asset,
            symbol,
            taker_order_id,
            maker_order_id,
            taker_account_id,
            maker_account_id,
            taker_side,
            price,
            qty,
            created_at: executed_at_ms,
        }
    }
    pub fn notional_quote(&self) -> Option<u64> {
        self.price.checked_mul(self.qty)
    }
    pub fn settle_into_positions(
        &self,
        taker: &PerpPositionV3,
        maker: &PerpPositionV3,
    ) -> Result<HyperliquidPerpTradePositionSettlement, HyperliquidPerpTradeError> {
        let tu = parse_user(&self.taker_account_id)?;
        let mu = parse_user(&self.maker_account_id)?;
        let base = parse_coin(&self.symbol)?;
        if taker.user_id() != tu
            || maker.user_id() != mu
            || taker.coin() != base
            || maker.coin() != base
        {
            return Err(PerpPositionV3Error::InvalidState.into());
        }
        let trade = self.trade(tu, mu, base, taker.signed_size(), maker.signed_size())?;
        let mut ta = taker.clone();
        let (_, to) = ta.transition(PerpPositionV3Event::TradeFilled {
            trade: trade.clone(),
            logical_ts: self.created_at,
        })?;
        let mut ma = maker.clone();
        let (_, mo) =
            ma.transition(PerpPositionV3Event::TradeFilled { trade, logical_ts: self.created_at })?;
        let pnl = |o| match o {
            PerpPositionV3Output::TradeApplied { closed_pnl } => closed_pnl,
            _ => Decimal::ZERO,
        };
        Ok(HyperliquidPerpTradePositionSettlement {
            taker_position_after: ta,
            maker_position_after: ma,
            taker_realized_pnl: pnl(to),
            maker_realized_pnl: pnl(mo),
        })
    }
    fn trade(
        &self,
        tu: UserId,
        mu: UserId,
        base: Coin,
        ts: Decimal,
        ms: Decimal,
    ) -> Result<PerpTradeV3, HyperliquidPerpTradeError> {
        Ok(PerpTradeV3 {
            trade_base: TradeBase {
                trade_id: self
                    .trade_id
                    .parse()
                    .map_err(|_| HyperliquidPerpTradeError::InvalidTrade)?,
                symbol: Symbol::new(self.asset, base, Coin::USDC),
                taker_user: tu,
                maker_user: mu,
                taker_order_id: self.taker_order_id,
                maker_order_id: self.maker_order_id,
                taker_side: self.taker_side,
                price: Decimal::from(self.price),
                size: Decimal::from(self.qty),
                taker_fee: Decimal::ZERO,
                maker_fee: Decimal::ZERO,
                created_at: self.created_at,
                updated_at: self.created_at,
                version: 0,
            },
            state: crate::entity::PerpTradeV3State::Unsettled,
            kind: PerpTradeKind::Normal,
            reference_id: Some(self.match_id.clone()),
            taker_start_position: ts,
            maker_start_position: ms,
            taker_closed_pnl: Decimal::ZERO,
            maker_closed_pnl: Decimal::ZERO,
            liquidation: None,
        })
    }
}
fn parse_user(s: &str) -> Result<UserId, HyperliquidPerpTradeError> {
    s.parse().map(UserId).map_err(|_| HyperliquidPerpTradeError::InvalidAccountId)
}
fn parse_coin(s: &str) -> Result<Coin, HyperliquidPerpTradeError> {
    match s.split('/').next().unwrap_or(s).split('-').next().unwrap_or(s) {
        "BTC" => Ok(Coin::BTC),
        "ETH" => Ok(Coin::ETH),
        "SOL" => Ok(Coin::SOL),
        _ => Err(HyperliquidPerpTradeError::InvalidSymbol),
    }
}
impl FieldDiff for HyperliquidPerpTrade {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
    fn diff(&self, _: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}
impl common_entity::Shard for HyperliquidPerpTrade {
    fn partition(&self) -> u8 {
        0
    }
}
impl Entity for HyperliquidPerpTrade {
    fn entity_id(&self) -> u64 {
        common_entity::stable_string_entity_id(&self.trade_id)
    }
    fn is_derived() -> bool {
        false
    }
    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_TRADE_ENTITY_TYPE
    }
    fn lsn(&self) -> u64 {
        1
    }
    fn replay_field_type(_: &str) -> u8 {
        0
    }
}
