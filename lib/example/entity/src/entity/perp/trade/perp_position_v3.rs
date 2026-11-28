use common_entity::{
    CommandMetadata, Entity, EntityError, EntityFieldChange, Fsm, LogAttr, ReplayBackedDomainEvent,
    ReplayableDomainEvent, Shard, TrackDiff,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::account::coin::Coin;
use crate::entity::common::{FundingRate, PartyId};
use crate::perp::fund::user_funding::UserFunding;
use crate::perp::trade::perp_trade_v3::PerpTradeV3;

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
        Self {
            all_time: Decimal::ZERO,
            since_open: Decimal::ZERO,
            since_change: Decimal::ZERO,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PerpPositionV3Error {
    #[error("replay projection failed: {0}")]
    Replay(#[from] EntityError),
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
    #[error("funding user does not match position owner")]
    FundingUserMismatch,
    #[error("funding coin does not match position market")]
    FundingCoinMismatch,
    #[error("funding settlement time precedes the position update time")]
    InvalidFundingSettlementTime,
    #[error("incoming lsn {incoming_lsn} is older than current lsn {current_lsn}")]
    StaleLsn { incoming_lsn: u64, current_lsn: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerpPositionV3State {
    Active,
}

#[derive(Debug, Clone)]
pub enum PerpPositionV3Event {
    TradeFilled {
        trade: PerpTradeV3,
        logical_ts: u64,
        lsn: u64,
    },
    FundingSettled {
        payment: UserFunding,
        logical_ts: u64,
        lsn: u64,
    },
    FundingAmountSettled {
        party_id: PartyId,
        coin: Coin,
        signed_amount: Decimal,
        logical_ts: u64,
        lsn: u64,
    },
    LeverageUpdated {
        leverage: Decimal,
        margin_mode: MarginModeV3,
        logical_ts: u64,
        lsn: u64,
    },
    IsolatedMarginAdjusted {
        delta: Decimal,
        logical_ts: u64,
        lsn: u64,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpPositionV3DomainEvent {
    TradeApplied { replay_event: ReplayableDomainEvent },
    FundingApplied { replay_event: ReplayableDomainEvent },
    FundingAmountApplied { replay_event: ReplayableDomainEvent },
    LeverageUpdated { replay_event: ReplayableDomainEvent },
    IsolatedMarginAdjusted { replay_event: ReplayableDomainEvent },
}
impl ReplayBackedDomainEvent<PerpPositionV3Event> for PerpPositionV3DomainEvent {
    fn domain_event_type(c: &PerpPositionV3Event) -> u8 {
        match c {
            PerpPositionV3Event::TradeFilled { .. } => 1,
            PerpPositionV3Event::FundingSettled { .. } => 2,
            PerpPositionV3Event::FundingAmountSettled { .. } => 3,
            PerpPositionV3Event::LeverageUpdated { .. } => 4,
            PerpPositionV3Event::IsolatedMarginAdjusted { .. } => 5,
        }
    }
    fn from_command(c: PerpPositionV3Event, replay_event: ReplayableDomainEvent) -> Self {
        match c {
            PerpPositionV3Event::TradeFilled { .. } => Self::TradeApplied { replay_event },
            PerpPositionV3Event::FundingSettled { .. } => Self::FundingApplied { replay_event },
            PerpPositionV3Event::FundingAmountSettled { .. } => {
                Self::FundingAmountApplied { replay_event }
            }
            PerpPositionV3Event::LeverageUpdated { .. } => Self::LeverageUpdated { replay_event },
            PerpPositionV3Event::IsolatedMarginAdjusted { .. } => {
                Self::IsolatedMarginAdjusted { replay_event }
            }
        }
    }
    fn replayable_event(&self) -> &ReplayableDomainEvent {
        match self {
            Self::TradeApplied { replay_event }
            | Self::FundingApplied { replay_event }
            | Self::FundingAmountApplied { replay_event }
            | Self::LeverageUpdated { replay_event }
            | Self::IsolatedMarginAdjusted { replay_event } => replay_event,
        }
    }
}
impl Shard for PerpPositionV3Event {
    fn partition(&self) -> u8 {
        0
    }
}
impl LogAttr for PerpPositionV3Event {
    fn lsn(&self) -> u64 {
        match self {
            Self::TradeFilled { lsn, .. }
            | Self::FundingSettled { lsn, .. }
            | Self::FundingAmountSettled { lsn, .. }
            | Self::LeverageUpdated { lsn, .. }
            | Self::IsolatedMarginAdjusted { lsn, .. } => *lsn,
        }
    }
    fn logical_ts(&self) -> u64 {
        match self {
            Self::TradeFilled { logical_ts, .. }
            | Self::FundingSettled { logical_ts, .. }
            | Self::FundingAmountSettled { logical_ts, .. }
            | Self::LeverageUpdated { logical_ts, .. }
            | Self::IsolatedMarginAdjusted { logical_ts, .. } => *logical_ts,
        }
    }
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
    party_id: PartyId,
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
    lsn: u64,
}

impl PerpPositionV3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        position_key: u64,
        party_id: PartyId,
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
            party_id,
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
            lsn: version,
        })
    }
    pub fn empty(position_id: u64, party_id: PartyId, coin: Coin) -> Self {
        Self {
            position_id,
            party_id,
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
            lsn: 0,
        }
    }
    pub fn position_id(&self) -> u64 {
        self.position_id
    }
    pub fn party_id(&self) -> &PartyId {
        &self.party_id
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
        self.lsn
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
        if amount.is_zero() {
            Ok(None)
        } else {
            Ok(Some(amount))
        }
    }

    /// 现值
    pub fn notional(&self, mark_price: Decimal) -> Decimal {
        self.szi.abs() * mark_price
    }
}

impl TrackDiff for PerpPositionV3 {
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
        ("party_id", position.party_id.0.to_string()),
        ("coin", position.coin.to_string()),
        ("szi", position.szi.to_string()),
        ("entry_px", position.entry_px.to_string()),
        ("leverage", position.leverage.to_string()),
        ("margin_mode", format!("{:?}", position.margin_mode)),
        (
            "isolated_margin",
            position
                .isolated_margin
                .map(|value| value.to_string())
                .unwrap_or_default(),
        ),
        ("realized_pnl", position.realized_pnl.to_string()),
        ("cum_fee", position.cum_fee.to_string()),
        (
            "funding_all_time",
            position.cum_funding.all_time.to_string(),
        ),
        (
            "funding_since_open",
            position.cum_funding.since_open.to_string(),
        ),
        (
            "funding_since_change",
            position.cum_funding.since_change.to_string(),
        ),
        ("created_at", position.created_at.to_string()),
        ("updated_at", position.updated_at.to_string()),
    ]
}

impl Shard for PerpPositionV3 {
    fn partition(&self) -> u8 {
        (common_entity::stable_string_entity_id(self.party_id.as_ref()) % 256) as u8
    }

    fn party_id(&self) -> String {
        self.party_id.0.to_string()
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
        self.lsn
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

    fn owner_id(&self) -> String {
        todo!()
    }

    fn op_id(&self) -> String {
        todo!()
    }

    fn created_at(&self) -> u64 {
        todo!()
    }

    fn updated_at(&self) -> u64 {
        todo!()
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

impl PerpPositionV3 {}

impl Fsm for PerpPositionV3 {
    type State = PerpPositionV3State;
    type Command = PerpPositionV3Event;
    type DomainEvent = PerpPositionV3DomainEvent;
    type Output = PerpPositionV3Output;
    type Error = PerpPositionV3Error;

    fn transition_working(
        &mut self,
        metadata: CommandMetadata,
        mut event: Self::Command,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        match &mut event {
            PerpPositionV3Event::TradeFilled {
                logical_ts, lsn, ..
            }
            | PerpPositionV3Event::FundingSettled {
                logical_ts, lsn, ..
            }
            | PerpPositionV3Event::FundingAmountSettled {
                logical_ts, lsn, ..
            }
            | PerpPositionV3Event::LeverageUpdated {
                logical_ts, lsn, ..
            }
            | PerpPositionV3Event::IsolatedMarginAdjusted {
                logical_ts, lsn, ..
            } => {
                *logical_ts = metadata.logical_ts;
                *lsn = metadata.lsn;
            }
        }
        match event {
            PerpPositionV3Event::TradeFilled {
                trade, logical_ts, ..
            } => {
                todo!()
            }
            PerpPositionV3Event::FundingSettled {
                payment,
                logical_ts,
                ..
            } => {
                todo!()
            }
            PerpPositionV3Event::FundingAmountSettled {
                party_id,
                coin,
                signed_amount,
                logical_ts,
                lsn,
            } => {
                if lsn < self.lsn {
                    return Err(PerpPositionV3Error::StaleLsn {
                        incoming_lsn: lsn,
                        current_lsn: self.lsn,
                    });
                }
                if party_id != self.party_id {
                    return Err(PerpPositionV3Error::FundingUserMismatch);
                }
                if coin != self.coin {
                    return Err(PerpPositionV3Error::FundingCoinMismatch);
                }
                if logical_ts < self.updated_at {
                    return Err(PerpPositionV3Error::InvalidFundingSettlementTime);
                }

                let cum_funding = FundingAccumulatorV3 {
                    all_time: self
                        .cum_funding
                        .all_time
                        .checked_add(signed_amount)
                        .ok_or(PerpPositionV3Error::ArithmeticOverflow)?,
                    since_open: self
                        .cum_funding
                        .since_open
                        .checked_add(signed_amount)
                        .ok_or(PerpPositionV3Error::ArithmeticOverflow)?,
                    since_change: self
                        .cum_funding
                        .since_change
                        .checked_add(signed_amount)
                        .ok_or(PerpPositionV3Error::ArithmeticOverflow)?,
                };
                self.cum_funding = cum_funding;
                self.updated_at = logical_ts;
                self.lsn = lsn;
                Ok((
                    PerpPositionV3State::Active,
                    PerpPositionV3Output::FundingApplied { signed_amount },
                ))
            }
            PerpPositionV3Event::LeverageUpdated {
                leverage,
                margin_mode,
                logical_ts,
                ..
            } => {
                todo!()
            }
            PerpPositionV3Event::IsolatedMarginAdjusted {
                delta, logical_ts, ..
            } => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Fsm;
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
                taker_party: PartyId("7".to_owned()),
                maker_party: PartyId("9".to_owned()),
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
            PartyId("7".to_owned()),
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

        let output = position
            .transition(PerpPositionV3Event::TradeFilled {
                trade: trade(OrderSide::Sell, 110, 1, Decimal::new(25, 2)),
                logical_ts: 9,
                lsn: 5,
            })
            .unwrap()
            .output;

        assert_eq!(
            output,
            PerpPositionV3Output::TradeApplied {
                closed_pnl: Decimal::TEN
            }
        );
        assert_eq!(position.realized_pnl(), Decimal::TEN);
        assert_eq!(position.cum_fee(), Decimal::new(25, 2));
        assert_eq!(position.version(), 5);
    }

    #[test]
    fn funding_amount_settlement_updates_all_accumulators_and_metadata() {
        let mut position = PerpPositionV3::new(
            1,
            PartyId("7".to_owned()),
            Coin::BTC,
            Decimal::ONE,
            Decimal::from(100),
            Decimal::from(5),
            MarginModeV3::Cross,
            None,
            Decimal::ZERO,
            FundingAccumulatorV3 {
                all_time: Decimal::ONE,
                since_open: Decimal::from(2),
                since_change: Decimal::from(3),
            },
            Decimal::ZERO,
            4,
        )
        .unwrap();

        let transition = position
            .transition(PerpPositionV3Event::FundingAmountSettled {
                party_id: PartyId("7".to_owned()),
                coin: Coin::BTC,
                signed_amount: Decimal::from(-1),
                logical_ts: 9,
                lsn: 5,
            })
            .unwrap();
        let state = transition.state;
        let output = transition.output;

        assert_eq!(state, PerpPositionV3State::Active);
        assert_eq!(
            output,
            PerpPositionV3Output::FundingApplied {
                signed_amount: -Decimal::ONE
            }
        );
        assert_eq!(
            position.cum_funding(),
            FundingAccumulatorV3 {
                all_time: Decimal::ZERO,
                since_open: Decimal::ONE,
                since_change: Decimal::from(2),
            }
        );
        assert_eq!(position.version(), 5);
        assert_eq!(position.updated_at(), 9);
    }

    #[test]
    fn funding_amount_settlement_errors_leave_position_unchanged() {
        let base = PerpPositionV3::new(
            1,
            PartyId("7".to_owned()),
            Coin::BTC,
            Decimal::ONE,
            Decimal::from(100),
            Decimal::from(5),
            MarginModeV3::Cross,
            None,
            Decimal::ZERO,
            FundingAccumulatorV3 {
                all_time: Decimal::MAX,
                since_open: Decimal::ZERO,
                since_change: Decimal::ZERO,
            },
            Decimal::ZERO,
            4,
        )
        .unwrap();
        let cases = [
            (
                PerpPositionV3Event::FundingAmountSettled {
                    party_id: PartyId("8".to_owned()),
                    coin: Coin::BTC,
                    signed_amount: Decimal::ONE,
                    logical_ts: 9,
                    lsn: 5,
                },
                PerpPositionV3Error::FundingUserMismatch,
            ),
            (
                PerpPositionV3Event::FundingAmountSettled {
                    party_id: PartyId("7".to_owned()),
                    coin: Coin::ETH,
                    signed_amount: Decimal::ONE,
                    logical_ts: 9,
                    lsn: 5,
                },
                PerpPositionV3Error::FundingCoinMismatch,
            ),
            (
                PerpPositionV3Event::FundingAmountSettled {
                    party_id: PartyId("7".to_owned()),
                    coin: Coin::BTC,
                    signed_amount: Decimal::ONE,
                    logical_ts: 9,
                    lsn: 5,
                },
                PerpPositionV3Error::ArithmeticOverflow,
            ),
        ];

        for (event, expected) in cases {
            let mut position = base.clone();
            assert_eq!(position.transition(event).map(|_| ()), Err(expected));
            assert_eq!(position, base);
        }
    }
}
