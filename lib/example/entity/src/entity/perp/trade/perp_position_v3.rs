use common_entity::{Entity, EntityFieldChange, FieldDiff, Fsm, Shard};
use rust_decimal::Decimal;
use rust_decimal::prelude::Signed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::account::coin::Coin;
use crate::entity::common::{FundingRate, UserId};
use crate::perp::fund::funding_payment::FundingPayment;
use crate::perp::trade::perp_trade_v3::{PerpTradeKind, PerpTradeV3};

const PERP_POSITION_V3_ENTITY_TYPE: u8 = 11;

/// 仓位使用的保证金方式。
///
/// 大白话说，核心区别就是：仓位亏损时从哪里扣钱，以及会不会影响其他仓位。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginModeV3 {
    /// 全仓：所有全仓仓位共用账户的 Cross 保证金池。
    ///
    /// 一个全仓仓位发生亏损时，可能会动用共享资金，因此也可能影响同一账户的其他全仓仓位。
    Cross,
    /// 逐仓：这个仓位只使用单独分配给自己的保证金。
    ///
    /// 当仓位亏损到这笔保证金耗尽时，通常不会直接影响其他仓位。
    Isolated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FundingAccumulatorV3 {
    pub all_time: Decimal,
    pub since_open: Decimal,
    pub since_change: Decimal,
}
impl Default for FundingAccumulatorV3 {
    fn default() -> Self {
        Self { all_time: Decimal::ZERO, since_open: Decimal::ZERO, since_change: Decimal::ZERO }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PerpPositionV3Error {
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
    #[error("isolated margin is invalid")]
    InvalidIsolatedMargin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerpPositionV3State {
    Active,
}

#[derive(Debug, Clone)]
pub enum PerpPositionV3Event {
    TradeFilled { trade: PerpTradeV3, logical_ts: u64 },
    FundingSettled { payment: FundingPayment, logical_ts: u64 },
    FundingAmountSettled { user_id: UserId, coin: Coin, signed_amount: Decimal, logical_ts: u64 },
    LeverageUpdated { leverage: Decimal, margin_mode: MarginModeV3, logical_ts: u64 },
    IsolatedMarginAdjusted { delta: Decimal, logical_ts: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerpPositionV3Output {
    TradeApplied { closed_pnl: Decimal },
    FundingApplied { signed_amount: Decimal },
    LeverageApplied,
    IsolatedMarginApplied { margin_after: Decimal },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpPositionV3 {
    /// 仓位的唯一标识。
    position_id: u64,
    /// 持有该仓位的账户标识。
    user_id: UserId,
    // /// 仓位对应的资产编号。
    // asset: u32,
    /// 仓位对应的币种或合约名称。
    coin: Coin,
    /// 带方向的仓位数量；正数表示多仓，负数表示空仓，零表示无仓位。
    szi: Decimal,
    /// 当前仓位的开仓均价；平仓后为空仓时为零。
    entry_px: Decimal,
    /// 仓位使用的杠杆倍数。
    leverage: Decimal,
    /// 仓位采用的保证金模式。
    margin_mode: MarginModeV3,
    /// 逐仓模式下配置的保证金金额；全仓模式下为空。
    isolated_margin: Option<Decimal>,
    /// 该仓位累计产生的已实现盈亏。
    realized_pnl: Decimal,
    /// 该仓位的累计费用记录。
    cum_fee: Decimal,
    /// 该仓位的累计资金费记录。
    cum_funding: FundingAccumulatorV3,

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
        coin: Coin,
        signed_size: Decimal,
        entry_price: Decimal,
        leverage: Decimal,
        margin_mode: MarginModeV3,
        isolated_margin: Option<Decimal>,
        realized_pnl: Decimal,
        cum_funding: FundingAccumulatorV3,
        cum_fee: Decimal,
        version: u64,
    ) -> Result<Self, PerpPositionV3Error> {
        if signed_size.is_zero() && !entry_price.is_zero() {
            return Err(PerpPositionV3Error::InvalidState);
        }
        if !signed_size.is_zero() && entry_price <= Decimal::ZERO {
            return Err(PerpPositionV3Error::InvalidPrice);
        }
        validate_margin(leverage, margin_mode, isolated_margin)?;
        Ok(Self {
            position_id: position_key,
            user_id,
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
    pub fn empty(position_id: u64, user_id: UserId, coin: Coin) -> Self {
        Self {
            position_id,
            user_id,
            coin,
            szi: Decimal::ZERO,
            entry_px: Decimal::ZERO,
            leverage: Decimal::ONE,
            margin_mode: MarginModeV3::Cross,
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
    pub fn coin(&self) -> Coin {
        self.coin
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
    pub fn margin_mode(&self) -> MarginModeV3 {
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
    pub fn cum_funding(&self) -> FundingAccumulatorV3 {
        self.cum_funding
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    pub fn updated_at(&self) -> u64 {
        self.updated_at
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
    /// 计算该仓位在指定 oracle 价格和规范化资金费率下的 USDC 变动。
    pub fn funding_usdc_delta(
        &self,
        oracle_price: Decimal,
        funding_rate: FundingRate,
    ) -> Result<Option<Decimal>, PerpPositionV3Error> {
        if oracle_price <= Decimal::ZERO {
            return Err(PerpPositionV3Error::InvalidPrice);
        }
        if self.is_flat() || funding_rate.value().is_zero() {
            return Ok(None);
        }
        let amount = self
            .szi
            .checked_mul(oracle_price)
            .and_then(|value| value.checked_mul(funding_rate.value()))
            .and_then(|value| Decimal::ZERO.checked_sub(value))
            .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        if amount.is_zero() { Ok(None) } else { Ok(Some(amount)) }
    }

    /// 现值
    pub fn notional(&self, mark_price: Decimal) -> Decimal {
        self.szi.abs() * mark_price
    }

    fn apply_trade(
        &mut self,
        signed_quantity: Decimal,
        trade_price: Decimal,
    ) -> Result<Decimal, PerpPositionV3Error> {
        if signed_quantity.is_zero() {
            return Err(PerpPositionV3Error::InvalidQuantity);
        }
        if trade_price <= Decimal::ZERO {
            return Err(PerpPositionV3Error::InvalidPrice);
        }
        let old = self.szi;
        let (new_size, realized, new_entry) = if old.is_zero() {
            (signed_quantity, Decimal::ZERO, trade_price)
        } else if old.signum() == signed_quantity.signum() {
            let total =
                old.checked_add(signed_quantity).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
            let weighted = old
                .abs()
                .checked_mul(self.entry_px)
                .and_then(|v| {
                    signed_quantity.abs().checked_mul(trade_price).and_then(|w| v.checked_add(w))
                })
                .and_then(|v| v.checked_div(total.abs()))
                .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
            (total, Decimal::ZERO, weighted)
        } else {
            let closing = old.abs().min(signed_quantity.abs());
            let delta = trade_price
                .checked_sub(self.entry_px)
                .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
            let pnl = closing
                .checked_mul(delta)
                .and_then(|v| {
                    if old.is_sign_negative() { Decimal::ZERO.checked_sub(v) } else { Some(v) }
                })
                .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
            let total =
                old.checked_add(signed_quantity).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
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
            .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        let version = self.version.checked_add(1).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
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
    fn apply_funding_amount(&mut self, signed_funding: Decimal) -> Result<(), PerpPositionV3Error> {
        let a = self
            .cum_funding
            .all_time
            .checked_add(signed_funding)
            .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        let o = self
            .cum_funding
            .since_open
            .checked_add(signed_funding)
            .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        let c = self
            .cum_funding
            .since_change
            .checked_add(signed_funding)
            .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        let v = self.version.checked_add(1).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        self.cum_funding = FundingAccumulatorV3 { all_time: a, since_open: o, since_change: c };
        self.version = v;
        Ok(())
    }
    fn change_leverage(
        &mut self,
        leverage: Decimal,
        margin_mode: MarginModeV3,
        isolated_margin: Option<Decimal>,
    ) -> Result<(), PerpPositionV3Error> {
        validate_margin(leverage, margin_mode, isolated_margin)?;
        let v = self.version.checked_add(1).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        self.leverage = leverage;
        self.margin_mode = margin_mode;
        self.isolated_margin = isolated_margin;
        self.version = v;
        Ok(())
    }
}

impl FieldDiff for PerpPositionV3 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        position_fields(self, "")
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let before = position_field_values(self);
        let after = position_field_values(other);
        before
            .into_iter()
            .zip(after)
            .filter(|((_, before), (_, after))| before != after)
            .map(|((name, before), (_, after))| EntityFieldChange::new(name, before, after))
            .collect()
    }
}

fn position_fields(position: &PerpPositionV3, before: &str) -> Vec<EntityFieldChange> {
    position_field_values(position)
        .into_iter()
        .map(|(name, after)| EntityFieldChange::new(name, before, after))
        .collect()
}

fn position_field_values(position: &PerpPositionV3) -> Vec<(&'static str, String)> {
    vec![
        ("user_id", position.user_id.0.to_string()),
        ("coin", position.coin.to_string()),
        ("szi", position.szi.to_string()),
        ("entry_px", position.entry_px.to_string()),
        ("leverage", position.leverage.to_string()),
        ("margin_mode", format!("{:?}", position.margin_mode)),
        (
            "isolated_margin",
            position.isolated_margin.map(|value| value.to_string()).unwrap_or_default(),
        ),
        ("realized_pnl", position.realized_pnl.to_string()),
        ("cum_fee", position.cum_fee.to_string()),
        ("funding_all_time", position.cum_funding.all_time.to_string()),
        ("funding_since_open", position.cum_funding.since_open.to_string()),
        ("funding_since_change", position.cum_funding.since_change.to_string()),
        ("created_at", position.created_at.to_string()),
        ("updated_at", position.updated_at.to_string()),
    ]
}

impl Shard for PerpPositionV3 {
    fn partition(&self) -> u8 {
        (self.user_id.0 % 256) as u8
    }

    fn party_id(&self) -> u64 {
        self.user_id.0
    }
}

impl Entity for PerpPositionV3 {
    fn entity_id(&self) -> u64 {
        self.position_id
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8 {
        PERP_POSITION_V3_ENTITY_TYPE
    }

    fn lsn(&self) -> u64 {
        self.version
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "szi"
            | "entry_px"
            | "leverage"
            | "isolated_margin"
            | "realized_pnl"
            | "cum_fee"
            | "funding_all_time"
            | "funding_since_open"
            | "funding_since_change" => 1,
            _ => 0,
        }
    }
}
fn validate_margin(
    leverage: Decimal,
    mode: MarginModeV3,
    isolated: Option<Decimal>,
) -> Result<(), PerpPositionV3Error> {
    if leverage <= Decimal::ZERO {
        return Err(PerpPositionV3Error::InvalidLeverage);
    }
    match mode {
        MarginModeV3::Cross if isolated.is_some() => {
            Err(PerpPositionV3Error::InvalidMarginConfiguration)
        }
        MarginModeV3::Isolated if isolated.is_none_or(|v| v < Decimal::ZERO) => {
            Err(PerpPositionV3Error::InvalidMarginConfiguration)
        }
        _ => Ok(()),
    }
}

impl PerpPositionV3 {
    fn apply_trade_filled(&mut self, trade: &PerpTradeV3) -> Result<Decimal, PerpPositionV3Error> {
        if trade.kind == PerpTradeKind::InsuranceFundTransfer {
            return Err(PerpPositionV3Error::InvalidState);
        }
        if self.coin != trade.trade_base.symbol.base {
            return Err(PerpPositionV3Error::InvalidState);
        }
        let (signed_quantity, fee) = if self.user_id == trade.trade_base.taker_user {
            let signed = match trade.trade_base.taker_side {
                crate::OrderSide::Buy => Some(trade.trade_base.size),
                crate::OrderSide::Sell => Decimal::ZERO.checked_sub(trade.trade_base.size),
            }
            .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
            (signed, trade.trade_base.taker_fee)
        } else if self.user_id == trade.trade_base.maker_user {
            let signed = match trade.trade_base.taker_side {
                crate::OrderSide::Buy => Decimal::ZERO.checked_sub(trade.trade_base.size),
                crate::OrderSide::Sell => Some(trade.trade_base.size),
            }
            .ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
            (signed, trade.trade_base.maker_fee)
        } else {
            return Err(PerpPositionV3Error::InvalidState);
        };
        let cum_fee =
            self.cum_fee.checked_add(fee).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
        let closed_pnl = self.apply_trade(signed_quantity, trade.trade_base.price)?;
        self.cum_fee = cum_fee;
        Ok(closed_pnl)
    }

    fn apply_funding_settled(
        &mut self,
        trade: &FundingPayment,
    ) -> Result<Decimal, PerpPositionV3Error> {
        if self.user_id != trade.user || self.coin != trade.coin {
            return Err(PerpPositionV3Error::InvalidState);
        }
        let signed = Decimal::from_i128_with_scale(trade.usdc.raw(), 6);
        self.apply_funding_amount(signed)?;
        Ok(signed)
    }
}

impl Fsm for PerpPositionV3 {
    type State = PerpPositionV3State;
    type BizEvent = PerpPositionV3Event;
    type Output = PerpPositionV3Output;
    type Error = PerpPositionV3Error;

    fn transition(
        &mut self,
        event: Self::BizEvent,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        match event {
            PerpPositionV3Event::TradeFilled { trade, logical_ts } => {
                let mut next = self.clone();
                let closed_pnl = next.apply_trade_filled(&trade)?;
                next.updated_at = logical_ts;
                *self = next;
                Ok((PerpPositionV3State::Active, PerpPositionV3Output::TradeApplied { closed_pnl }))
            }
            PerpPositionV3Event::FundingSettled { payment, logical_ts } => {
                let mut next = self.clone();
                let signed_amount = next.apply_funding_settled(&payment)?;
                next.updated_at = logical_ts;
                *self = next;
                Ok((
                    PerpPositionV3State::Active,
                    PerpPositionV3Output::FundingApplied { signed_amount },
                ))
            }
            PerpPositionV3Event::FundingAmountSettled {
                user_id,
                coin,
                signed_amount,
                logical_ts,
            } => {
                if self.user_id != user_id || self.coin != coin {
                    return Err(PerpPositionV3Error::InvalidState);
                }
                let mut next = self.clone();
                next.apply_funding_amount(signed_amount)?;
                next.updated_at = logical_ts;
                *self = next;
                Ok((
                    PerpPositionV3State::Active,
                    PerpPositionV3Output::FundingApplied { signed_amount },
                ))
            }
            PerpPositionV3Event::LeverageUpdated { leverage, margin_mode, logical_ts } => {
                let mut next = self.clone();
                next.change_leverage(leverage, margin_mode, next.isolated_margin)?;
                next.updated_at = logical_ts;
                *self = next;
                Ok((PerpPositionV3State::Active, PerpPositionV3Output::LeverageApplied))
            }
            PerpPositionV3Event::IsolatedMarginAdjusted { delta, logical_ts } => {
                if self.margin_mode != MarginModeV3::Isolated {
                    return Err(PerpPositionV3Error::InvalidMarginConfiguration);
                }
                let current =
                    self.isolated_margin.ok_or(PerpPositionV3Error::InvalidMarginConfiguration)?;
                let margin_after =
                    current.checked_add(delta).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
                if margin_after < Decimal::ZERO {
                    return Err(PerpPositionV3Error::InvalidIsolatedMargin);
                }
                let version =
                    self.version.checked_add(1).ok_or(PerpPositionV3Error::ArithmeticOverflow)?;
                self.isolated_margin = Some(margin_after);
                self.version = version;
                self.updated_at = logical_ts;
                Ok((
                    PerpPositionV3State::Active,
                    PerpPositionV3Output::IsolatedMarginApplied { margin_after },
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, Fsm};
    use rust_decimal::Decimal;

    use super::*;
    use crate::account::symbol::Symbol;
    use crate::entity::common::common_trade::TradeBase;
    use crate::{OrderSide, PerpTradeKind};

    fn trade(side: OrderSide, price: i64, size: i64, fee: Decimal) -> PerpTradeV3 {
        PerpTradeV3 {
            trade_base: TradeBase {
                trade_id: 8,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDC),
                taker_user: UserId(7),
                maker_user: UserId(9),
                taker_order_id: 10,
                maker_order_id: 11,
                taker_side: side,
                price: Decimal::from(price),
                size: Decimal::from(size),
                taker_fee: fee,
                maker_fee: Decimal::ZERO,
                created_at: 1,
                updated_at: 1,
                version: 0,
            },
            state: crate::entity::PerpTradeV3State::Unsettled,
            kind: PerpTradeKind::Normal,
            reference_id: None,
            taker_start_position: Decimal::ZERO,
            maker_start_position: Decimal::ZERO,
            taker_closed_pnl: Decimal::ZERO,
            maker_closed_pnl: Decimal::ZERO,
            liquidation: None,
        }
    }

    #[test]
    fn trade_output_uses_position_truth_for_closed_pnl_and_tracks_fee() {
        let mut position = PerpPositionV3::new(
            1,
            UserId(7),
            Coin::BTC,
            Decimal::from(2),
            Decimal::from(100),
            Decimal::from(5),
            MarginModeV3::Cross,
            None,
            Decimal::ZERO,
            FundingAccumulatorV3::default(),
            Decimal::ZERO,
            4,
        )
        .unwrap();

        let (_, output) = position
            .transition(PerpPositionV3Event::TradeFilled {
                trade: trade(OrderSide::Sell, 110, 1, Decimal::new(25, 2)),
                logical_ts: 9,
            })
            .unwrap();

        assert_eq!(output, PerpPositionV3Output::TradeApplied { closed_pnl: Decimal::TEN });
        assert_eq!(position.realized_pnl(), Decimal::TEN);
        assert_eq!(position.cum_fee(), Decimal::new(25, 2));
        assert_eq!(position.version(), 5);
    }

    #[test]
    fn failed_transition_is_atomic_and_success_projects_single_step_update() {
        let mut position = PerpPositionV3::empty(1, UserId(7), Coin::BTC);
        let before_failure = position.clone();
        assert_eq!(
            position.apply(PerpPositionV3Event::TradeFilled {
                trade: trade(OrderSide::Buy, 0, 1, Decimal::ZERO),
                logical_ts: 9,
            }),
            Err(PerpPositionV3Error::InvalidPrice)
        );
        assert_eq!(position, before_failure);

        let before = position.clone();
        position
            .apply(PerpPositionV3Event::LeverageUpdated {
                leverage: Decimal::from(3),
                margin_mode: MarginModeV3::Cross,
                logical_ts: 10,
            })
            .unwrap();
        let event = position.track_update_event_from(&before).unwrap();
        assert_eq!(event.entity_id, 1);
        assert_eq!(event.entity_type, 11);
        assert_eq!(position.version(), before.version() + 1);
    }
}
