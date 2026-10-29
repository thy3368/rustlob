use rust_decimal::Decimal;
use rust_decimal::prelude::Signed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::common::UserId;
use crate::perp::fund::funding_payment::FundingPayment;
use crate::perp::trade::perp_trade_v3::{PerpTradeKind, PerpTradeV3};

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
    ///这个仓位用保证金赚的“收益率”，不是账户总收益率。
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
pub struct PerpPositionV3 {
    /// 仓位的唯一标识。
    position_id: u64,
    /// 持有该仓位的账户标识。
    user_id: UserId,
    /// 仓位对应的资产编号。
    asset: u32,
    /// 仓位对应的币种或合约名称。
    coin: String,
    /// 带方向的仓位数量；正数表示多仓，负数表示空仓，零表示无仓位。
    szi: Decimal,
    /// 当前仓位的开仓均价；平仓后为空仓时为零。
    entry_px: Decimal,
    /// 仓位使用的杠杆倍数。
    leverage: Decimal,
    /// 仓位采用的保证金模式。
    margin_mode: MarginModeV2,
    /// 逐仓模式下配置的保证金金额；全仓模式下为空。
    isolated_margin: Option<Decimal>,
    /// 该仓位累计产生的已实现盈亏。
    realized_pnl: Decimal,
    /// 该仓位的累计费用记录。
    cum_fee: Decimal,
    /// 该仓位的累计资金费记录。
    cum_funding: FundingAccumulatorV2,

    /// 订单创建时间，单位为 Unix 纳秒。
    created_at: u64,
    /// 订单最后更新时间，单位为 Unix 纳秒。
    updated_at: u64,
    /// 仓位状态版本号，每次状态变更后递增。
    version: u64,
}

impl PerpPositionV3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        position_key: u64,
        user_id: UserId,
        asset_id: u32,
        coin: String,
        signed_size: Decimal,
        entry_price: Decimal,
        leverage: Decimal,
        margin_mode: MarginModeV2,
        isolated_margin: Option<Decimal>,
        realized_pnl: Decimal,
        cum_funding: FundingAccumulatorV2,
        cum_fee: Decimal,
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
            user_id,
            asset: asset_id,
            coin,
            szi: signed_size,
            entry_px: entry_price,
            leverage,
            margin_mode,
            isolated_margin,
            realized_pnl,
            cum_fee,
            cum_funding,
            created_at: 0,
            updated_at: 0,
            version,
        })
    }
    pub fn empty_slot(position_id: u64, user_id: UserId, asset_id: u32, coin: String) -> Self {
        Self {
            position_id,
            user_id,
            asset: asset_id,
            coin,
            szi: Decimal::ZERO,
            entry_px: Decimal::ZERO,
            leverage: Decimal::ONE,
            margin_mode: MarginModeV2::Cross,
            isolated_margin: None,
            realized_pnl: Decimal::ZERO,
            cum_fee: Decimal::ZERO,
            cum_funding: Default::default(),
            created_at: 0,
            updated_at: 0,
            version: 0,
        }
    }
    pub fn position_key(&self) -> u64 {
        self.position_id
    }
    pub fn user_id(&self) -> UserId {
        self.user_id
    }
    pub fn asset_id(&self) -> u32 {
        self.asset
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
    pub fn cum_fee(&self) -> Decimal {
        self.cum_fee
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
        self.szi * (mark_price - self.entry_px)
    }

    /// 初始保证金 下单时有用，其它开仓时会减去这个
    pub fn im(&self) -> Decimal {
        self.szi.abs() * self.entry_px / self.leverage
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
    fn apply_funding_amount(&mut self, signed_funding: Decimal) -> Result<(), PerpPositionV2Error> {
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
        MarginModeV2::Isolated if isolated.is_none_or(|v| v < Decimal::ZERO) => {
            Err(PerpPositionV2Error::InvalidMarginConfiguration)
        }
        _ => Ok(()),
    }
}

impl PerpPositionV3 {
    pub fn apply_trade2(&mut self, trade: &PerpTradeV3) -> Result<Decimal, PerpPositionV2Error> {
        if trade.kind == PerpTradeKind::InsuranceFundTransfer {
            return Err(PerpPositionV2Error::InvalidState);
        }
        if self.asset_id() != trade.trade_base.asset {
            return Err(PerpPositionV2Error::InvalidState);
        }
        let (signed_quantity, closed_pnl, fee) = if self.user_id == trade.trade_base.taker_user {
            let signed = match trade.trade_base.taker_side {
                crate::OrderSide::Buy => Some(trade.trade_base.size),
                crate::OrderSide::Sell => Decimal::ZERO.checked_sub(trade.trade_base.size),
            }
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
            (signed, trade.taker_closed_pnl, trade.trade_base.taker_fee)
        } else if self.user_id == trade.trade_base.maker_user {
            let signed = match trade.trade_base.taker_side {
                crate::OrderSide::Buy => Decimal::ZERO.checked_sub(trade.trade_base.size),
                crate::OrderSide::Sell => Some(trade.trade_base.size),
            }
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
            (signed, trade.maker_closed_pnl, trade.trade_base.maker_fee)
        } else {
            return Err(PerpPositionV2Error::InvalidState);
        };
        let old = self.clone();
        let mut next = old.clone();
        next.apply_trade(signed_quantity, trade.trade_base.price)?;
        next.cum_fee =
            old.cum_fee.checked_add(fee).ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        next.realized_pnl = old
            .realized_pnl
            .checked_add(closed_pnl)
            .ok_or(PerpPositionV2Error::ArithmeticOverflow)?;
        *self = next;
        Ok(closed_pnl)
    }

    pub fn apply_funding(
        &mut self,
        trade: &FundingPayment,
    ) -> Result<Decimal, PerpPositionV2Error> {
        if self.user_id != trade.user || self.asset_id() != trade.coin.as_u32() {
            return Err(PerpPositionV2Error::InvalidState);
        }
        let signed = Decimal::from_i128_with_scale(trade.usdc.raw(), 6);
        self.apply_funding_amount(signed)?;
        Ok(signed)
    }
}
