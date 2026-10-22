use rust_decimal::prelude::Signed;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginModeV2 {
    Cross,
    Isolated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FundingAccumulatorV2 {
    pub all_time: Decimal,
    pub since_open: Decimal,
    pub since_change: Decimal,
}
impl Default for FundingAccumulatorV2 {
    fn default() -> Self {
        Self { all_time: Decimal::ZERO, since_open: Decimal::ZERO, since_change: Decimal::ZERO }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpPositionMetricsV2 {
    pub position_value: Decimal,
    pub unrealized_pnl: Decimal,
    pub margin_used: Decimal,
    pub return_on_equity: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PerpPositionV2Error {
    #[error("quantity must be non-zero")]
    InvalidQuantity,
    #[error("price must be greater than zero")]
    InvalidPrice,
    #[error("leverage must be greater than zero")]
    InvalidLeverage,
    #[error("invalid margin configuration")]
    InvalidMarginConfiguration,
    #[error("position state is invalid")]
    InvalidState,
    #[error("arithmetic overflow")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpPositionV2 {
    position_id: String,
    account_id: String,
    asset_id: u32,
    coin: String,
    szi: Decimal,
    entry_px: Decimal,
    leverage: Decimal,
    margin_mode: MarginModeV2,
    isolated_margin: Option<Decimal>,
    realized_pnl: Decimal,
    cum_funding: FundingAccumulatorV2,
    version: u64,
}

impl PerpPositionV2 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        position_key: String,
        account_id: String,
        asset_id: u32,
        coin: String,
        signed_size: Decimal,
        entry_price: Decimal,
        leverage: Decimal,
        margin_mode: MarginModeV2,
        isolated_margin: Option<Decimal>,
        realized_pnl: Decimal,
        cum_funding: FundingAccumulatorV2,
        version: u64,
    ) -> Result<Self, PerpPositionV2Error> {
        if signed_size.is_zero() && !entry_price.is_zero() {
            return Err(PerpPositionV2Error::InvalidState);
        }
        if !signed_size.is_zero() && entry_price <= Decimal::ZERO {
            return Err(PerpPositionV2Error::InvalidPrice);
        }
        validate_margin(leverage, margin_mode, isolated_margin)?;
        Ok(Self {
            position_id: position_key,
            account_id,
            asset_id,
            coin,
            szi: signed_size,
            entry_px: entry_price,
            leverage,
            margin_mode,
            isolated_margin,
            realized_pnl,
            cum_funding,
            version,
        })
    }
    pub fn empty_slot(
        position_key: String,
        account_id: String,
        asset_id: u32,
        coin: String,
    ) -> Self {
        Self {
            position_id: position_key,
            account_id,
            asset_id,
            coin,
            szi: Decimal::ZERO,
            entry_px: Decimal::ZERO,
            leverage: Decimal::ONE,
            margin_mode: MarginModeV2::Cross,
            isolated_margin: None,
            realized_pnl: Decimal::ZERO,
            cum_funding: Default::default(),
            version: 0,
        }
    }
    pub fn position_key(&self) -> &str {
        &self.position_id
    }
    pub fn account_id(&self) -> &str {
        &self.account_id
    }
    pub fn asset_id(&self) -> u32 {
        self.asset_id
    }
    pub fn coin(&self) -> &str {
        &self.coin
    }
    pub fn signed_size(&self) -> Decimal {
        self.szi
    }
    pub fn entry_price(&self) -> Decimal {
        self.entry_px
    }
    pub fn leverage(&self) -> Decimal {
        self.leverage
    }
    pub fn margin_mode(&self) -> MarginModeV2 {
        self.margin_mode
    }
    pub fn isolated_margin(&self) -> Option<Decimal> {
        self.isolated_margin
    }
    pub fn realized_pnl(&self) -> Decimal {
        self.realized_pnl
    }
    pub fn cum_funding(&self) -> FundingAccumulatorV2 {
        self.cum_funding
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn is_flat(&self) -> bool {
        self.szi.is_zero()
    }
    pub fn is_long(&self) -> bool {
        self.szi > Decimal::ZERO
    }
    pub fn is_short(&self) -> bool {
        self.szi < Decimal::ZERO
    }
    pub fn quantity(&self) -> Decimal {
        self.szi.abs()
    }

    /// 现值
    pub fn notional(&self, mark_price: Decimal) -> Decimal {
        self.szi.abs() * mark_price
    }

    /// 浮盈
    pub fn unrealized_pnl(&self, mark_price: Decimal) -> Decimal {
        self.szi.abs() * (mark_price - self.entry_px)
    }

    pub fn side_label(&self) -> &'static str {
        if self.is_long() {
            "long"
        } else if self.is_short() {
            "short"
        } else {
            "flat"
        }
    }
    pub fn apply_trade(
        &mut self,
        signed_quantity: Decimal,
        trade_price: Decimal,
    ) -> Result<Decimal, PerpPositionV2Error> {
        if signed_quantity.is_zero() {
            return Err(PerpPositionV2Error::InvalidQuantity);
        }
        if trade_price <= Decimal::ZERO {
            return Err(PerpPositionV2Error::InvalidPrice);
        }
        let old = self.szi;
        let (new_size, realized, new_entry) = if old.is_zero() {
            (signed_quantity, Decimal::ZERO, trade_price)
        } else if old.signum() == signed_quantity.signum() {
            let total =
                old.checked_add(signed_quantity).ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
            let weighted = old
                .abs()
                .checked_mul(self.entry_px)
                .and_then(|v| {
                    signed_quantity.abs().checked_mul(trade_price).and_then(|w| v.checked_add(w))
                })
                .and_then(|v| v.checked_div(total.abs()))
                .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
            (total, Decimal::ZERO, weighted)
        } else {
            let closing = old.abs().min(signed_quantity.abs());
            let delta = trade_price
                .checked_sub(self.entry_px)
                .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
            let pnl = closing
                .checked_mul(delta)
                .and_then(|v| {
                    if old.is_sign_negative() { Decimal::ZERO.checked_sub(v) } else { Some(v) }
                })
                .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
            let total =
                old.checked_add(signed_quantity).ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
            (
                total,
                pnl,
                if total.is_zero() {
                    Decimal::ZERO
                } else if total.signum() == old.signum() {
                    self.entry_px
                } else {
                    trade_price
                },
            )
        };
        let cumulative = self
            .realized_pnl
            .checked_add(realized)
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        let version = self.version.checked_add(1).ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        self.szi = new_size;
        self.entry_px = new_entry;
        self.realized_pnl = cumulative;
        self.version = version;
        if old.is_zero() || old.signum() != new_size.signum() || new_size.is_zero() {
            self.cum_funding.since_open = Decimal::ZERO;
            self.cum_funding.since_change = Decimal::ZERO;
        }
        Ok(realized)
    }
    pub fn apply_funding(&mut self, signed_funding: Decimal) -> Result<(), PerpPositionV2Error> {
        let a = self
            .cum_funding
            .all_time
            .checked_add(signed_funding)
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        let o = self
            .cum_funding
            .since_open
            .checked_add(signed_funding)
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        let c = self
            .cum_funding
            .since_change
            .checked_add(signed_funding)
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        let v = self.version.checked_add(1).ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        self.cum_funding = FundingAccumulatorV2 { all_time: a, since_open: o, since_change: c };
        self.version = v;
        Ok(())
    }
    pub fn change_leverage(
        &mut self,
        leverage: Decimal,
        margin_mode: MarginModeV2,
        isolated_margin: Option<Decimal>,
    ) -> Result<(), PerpPositionV2Error> {
        validate_margin(leverage, margin_mode, isolated_margin)?;
        let v = self.version.checked_add(1).ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        self.leverage = leverage;
        self.margin_mode = margin_mode;
        self.isolated_margin = isolated_margin;
        self.version = v;
        Ok(())
    }
    pub fn metrics_at(
        &self,
        mark_price: Decimal,
        equity: Decimal,
    ) -> Result<PerpPositionMetricsV2, PerpPositionV2Error> {
        if mark_price <= Decimal::ZERO {
            return Err(PerpPositionV2Error::InvalidPrice);
        }
        let value = self
            .quantity()
            .checked_mul(mark_price)
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        let pnl = self
            .szi
            .checked_mul(
                mark_price
                    .checked_sub(self.entry_px)
                    .ok_or(PerpPositionV2Error::ArithmeticOverflow)?,
            )
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        let margin =
            value.checked_div(self.leverage).ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        let roe = if equity.is_zero() {
            None
        } else {
            Some(pnl.checked_div(equity).ok_or(PerpPositionV2Error::ArithmeticOverflow)?)
        };
        Ok(PerpPositionMetricsV2 {
            position_value: value,
            unrealized_pnl: pnl,
            margin_used: margin,
            return_on_equity: roe,
        })
    }
}
fn validate_margin(
    leverage: Decimal,
    mode: MarginModeV2,
    isolated: Option<Decimal>,
) -> Result<(), PerpPositionV2Error> {
    if leverage <= Decimal::ZERO {
        return Err(PerpPositionV2Error::InvalidLeverage);
    }
    match mode {
        MarginModeV2::Cross if isolated.is_some() => {
            Err(PerpPositionV2Error::InvalidMarginConfiguration)
        }
        MarginModeV2::Isolated if isolated.map_or(true, |v| v < Decimal::ZERO) => {
            Err(PerpPositionV2Error::InvalidMarginConfiguration)
        }
        _ => Ok(()),
    }
}
