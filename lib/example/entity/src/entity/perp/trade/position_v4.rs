use common_entity::{
    AggregateRole, CommandMetadata, Entity, EntityError, EntityFieldChange, EntityMutationModel,
    FourColorArchetype, Fsm, LogAttr, ReplayBackedDomainEvent, ReplayableDomainEvent, Shard,
    TrackDiff,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::Signed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::account::coin::Coin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginMode {
    Cross,
    Isolated,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leverage {
    pub leverage_type: MarginMode,
    pub value: u8,
    pub raw_usd: Option<Decimal>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FundingAccum {
    pub all_time: Decimal,
    pub since_open: Decimal,
    pub since_change: Decimal,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionState {
    Empty,
    Open,
    Flat,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillSource {
    Regular,
    Liquidation { liquidation_id: u64 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionFill {
    pub signed_quantity: Decimal,
    pub price: Decimal,
    pub source: FillSource,
}
#[derive(Debug, Clone)]
pub enum PositionEvent {
    TradeFilled {
        fill: PositionFill,
        logical_ts: u64,
        lsn: u64,
    },
    FundingSettled {
        signed_amount: Decimal,
        logical_ts: u64,
        lsn: u64,
    },
    LeverageUpdated {
        leverage: Leverage,
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
pub enum PositionV4DomainEvent {
    TradeFilled { replay_event: ReplayableDomainEvent },
    FundingSettled { replay_event: ReplayableDomainEvent },
    LeverageUpdated { replay_event: ReplayableDomainEvent },
    IsolatedMarginAdjusted { replay_event: ReplayableDomainEvent },
}
impl ReplayBackedDomainEvent<PositionEvent> for PositionV4DomainEvent {
    fn domain_event_type(c: &PositionEvent) -> u8 {
        match c {
            PositionEvent::TradeFilled { .. } => 1,
            PositionEvent::FundingSettled { .. } => 2,
            PositionEvent::LeverageUpdated { .. } => 3,
            PositionEvent::IsolatedMarginAdjusted { .. } => 4,
        }
    }
    fn from_command(c: PositionEvent, replay_event: ReplayableDomainEvent) -> Self {
        match c {
            PositionEvent::TradeFilled { .. } => Self::TradeFilled { replay_event },
            PositionEvent::FundingSettled { .. } => Self::FundingSettled { replay_event },
            PositionEvent::LeverageUpdated { .. } => Self::LeverageUpdated { replay_event },
            PositionEvent::IsolatedMarginAdjusted { .. } => {
                Self::IsolatedMarginAdjusted { replay_event }
            }
        }
    }
    fn replayable_event(&self) -> &ReplayableDomainEvent {
        match self {
            Self::TradeFilled { replay_event }
            | Self::FundingSettled { replay_event }
            | Self::LeverageUpdated { replay_event }
            | Self::IsolatedMarginAdjusted { replay_event } => replay_event,
        }
    }
}
impl Shard for PositionEvent {
    fn partition(&self) -> u8 {
        0
    }
}
impl LogAttr for PositionEvent {
    fn lsn(&self) -> u64 {
        match self {
            Self::TradeFilled { lsn, .. }
            | Self::FundingSettled { lsn, .. }
            | Self::LeverageUpdated { lsn, .. }
            | Self::IsolatedMarginAdjusted { lsn, .. } => *lsn,
        }
    }
    fn logical_ts(&self) -> u64 {
        match self {
            Self::TradeFilled { logical_ts, .. }
            | Self::FundingSettled { logical_ts, .. }
            | Self::LeverageUpdated { logical_ts, .. }
            | Self::IsolatedMarginAdjusted { logical_ts, .. } => *logical_ts,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeChange {
    Opened,
    Increased,
    Reduced,
    Closed,
    Flipped,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionOutput {
    TradeApplied {
        change: TradeChange,
        realized_pnl: Decimal,
        size_before: Decimal,
        size_after: Decimal,
        source: FillSource,
    },
    FundingApplied {
        signed_amount: Decimal,
    },
    LeverageApplied,
    IsolatedMarginApplied {
        margin_after: Decimal,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PositionError {
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
    #[error("maintenance margin rate must be in (0, 1]")]
    InvalidMaintenanceMarginRate,
    #[error("event is not valid for this state")]
    InvalidState,
    #[error("isolated margin cannot be negative")]
    InvalidIsolatedMargin,
    #[error("arithmetic overflow")]
    ArithmeticOverflow,
    #[error("stale lsn {incoming_lsn}; current lsn is {current_lsn}")]
    StaleLsn { incoming_lsn: u64, current_lsn: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 永续仓位；`version` 是最近应用的全局 command LSN，可跳号或被同一 command 复用。
pub struct PositionV4 {
    position_id: u64,
    account_id: u64,
    coin: Coin,
    state: PositionState,
    szi: Decimal,
    entry_px: Decimal,
    leverage: Leverage,
    cum_funding: FundingAccum,
    version: u64,
    created_at: u64,
    updated_at: u64,
}
impl PositionV4 {
    pub fn empty(position_id: u64, account_id: u64, coin: Coin) -> Self {
        Self {
            position_id,
            account_id,
            coin,
            state: PositionState::Empty,
            szi: Decimal::ZERO,
            entry_px: Decimal::ZERO,
            leverage: Leverage {
                leverage_type: MarginMode::Cross,
                value: 1,
                raw_usd: None,
            },
            cum_funding: FundingAccum::default(),
            version: 0,
            created_at: 0,
            updated_at: 0,
        }
    }
    pub fn new(
        position_id: u64,
        account_id: u64,
        coin: Coin,
        leverage: Leverage,
        created_at: u64,
    ) -> Result<Self, PositionError> {
        validate_leverage(leverage)?;
        let mut p = Self::empty(position_id, account_id, coin);
        p.leverage = leverage;
        p.created_at = created_at;
        p.updated_at = created_at;
        Ok(p)
    }
    pub fn position_id(&self) -> u64 {
        self.position_id
    }
    pub fn account_id(&self) -> u64 {
        self.account_id
    }
    pub fn coin(&self) -> Coin {
        self.coin
    }
    pub fn state(&self) -> PositionState {
        self.state
    }
    pub fn signed_size(&self) -> Decimal {
        self.szi
    }
    pub fn entry_price(&self) -> Decimal {
        self.entry_px
    }
    pub fn leverage(&self) -> Leverage {
        self.leverage
    }
    pub fn cum_funding(&self) -> FundingAccum {
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
}
fn validate_leverage(l: Leverage) -> Result<(), PositionError> {
    if l.value == 0 {
        return Err(PositionError::InvalidLeverage);
    }
    match l.leverage_type {
        MarginMode::Cross if l.raw_usd.is_some() => Err(PositionError::InvalidMarginConfiguration),
        MarginMode::Isolated if l.raw_usd.is_none_or(|v| v < Decimal::ZERO) => {
            Err(PositionError::InvalidMarginConfiguration)
        }
        _ => Ok(()),
    }
}

impl Fsm for PositionV4 {
    type State = PositionState;
    type Command = PositionEvent;
    type DomainEvent = PositionV4DomainEvent;
    type Output = PositionOutput;
    type Error = PositionError;
    /// 使用事件的 LSN 与逻辑时间推进仓位，不从当前实体版本推导下一版本。
    fn transition_working(
        &mut self,
        metadata: CommandMetadata,
        mut event: Self::Command,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        match &mut event {
            PositionEvent::TradeFilled {
                logical_ts, lsn, ..
            }
            | PositionEvent::FundingSettled {
                logical_ts, lsn, ..
            }
            | PositionEvent::LeverageUpdated {
                logical_ts, lsn, ..
            }
            | PositionEvent::IsolatedMarginAdjusted {
                logical_ts, lsn, ..
            } => {
                *logical_ts = metadata.logical_ts;
                *lsn = metadata.lsn;
            }
        }
        let lsn = event.lsn();
        if lsn < self.version {
            return Err(PositionError::StaleLsn {
                incoming_lsn: lsn,
                current_lsn: self.version,
            });
        }
        let mut n = self.clone();
        let out = match event {
            PositionEvent::TradeFilled {
                fill, logical_ts, ..
            } => {
                if fill.signed_quantity.is_zero() {
                    return Err(PositionError::InvalidQuantity);
                }
                if fill.price <= Decimal::ZERO {
                    return Err(PositionError::InvalidPrice);
                }
                let before = n.szi;
                let same = !before.is_zero() && before.signum() == fill.signed_quantity.signum();
                let total = before
                    .checked_add(fill.signed_quantity)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                let closing = if same || before.is_zero() {
                    Decimal::ZERO
                } else {
                    before.abs().min(fill.signed_quantity.abs())
                };
                let d = fill
                    .price
                    .checked_sub(n.entry_px)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                let mut pnl = closing
                    .checked_mul(d)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                if before.is_sign_negative() {
                    pnl = -pnl;
                }
                let entry = if total.is_zero() {
                    Decimal::ZERO
                } else if before.is_zero() || (!same && total.signum() != before.signum()) {
                    fill.price
                } else if same {
                    before
                        .abs()
                        .checked_mul(n.entry_px)
                        .and_then(|v| {
                            fill.signed_quantity
                                .abs()
                                .checked_mul(fill.price)
                                .and_then(|w| v.checked_add(w))
                        })
                        .and_then(|v| v.checked_div(total.abs()))
                        .ok_or(PositionError::ArithmeticOverflow)?
                } else {
                    n.entry_px
                };
                n.szi = total;
                n.entry_px = entry;
                n.state = if total.is_zero() {
                    PositionState::Flat
                } else {
                    PositionState::Open
                };
                n.version = lsn;
                n.updated_at = logical_ts;
                n.cum_funding = if total.is_zero() {
                    FundingAccum {
                        since_open: Decimal::ZERO,
                        since_change: Decimal::ZERO,
                        ..n.cum_funding
                    }
                } else if before.is_zero() || (!same && total.signum() != before.signum()) {
                    FundingAccum {
                        since_open: Decimal::ZERO,
                        since_change: Decimal::ZERO,
                        ..n.cum_funding
                    }
                } else {
                    FundingAccum {
                        since_change: Decimal::ZERO,
                        ..n.cum_funding
                    }
                };
                let change = if before.is_zero() {
                    TradeChange::Opened
                } else if total.is_zero() {
                    TradeChange::Closed
                } else if same {
                    TradeChange::Increased
                } else if total.signum() == before.signum() {
                    TradeChange::Reduced
                } else {
                    TradeChange::Flipped
                };
                PositionOutput::TradeApplied {
                    change,
                    realized_pnl: pnl,
                    size_before: before,
                    size_after: total,
                    source: fill.source,
                }
            }
            PositionEvent::FundingSettled {
                signed_amount,
                logical_ts,
                ..
            } => {
                if n.state != PositionState::Open {
                    return Err(PositionError::InvalidState);
                }
                n.cum_funding.all_time = n
                    .cum_funding
                    .all_time
                    .checked_add(signed_amount)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                n.cum_funding.since_open = n
                    .cum_funding
                    .since_open
                    .checked_add(signed_amount)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                n.cum_funding.since_change = n
                    .cum_funding
                    .since_change
                    .checked_add(signed_amount)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                n.version = lsn;
                n.updated_at = logical_ts;
                PositionOutput::FundingApplied { signed_amount }
            }
            PositionEvent::LeverageUpdated {
                leverage,
                logical_ts,
                ..
            } => {
                validate_leverage(leverage)?;
                n.leverage = leverage;
                n.cum_funding.since_change = Decimal::ZERO;
                n.version = lsn;
                n.updated_at = logical_ts;
                PositionOutput::LeverageApplied
            }
            PositionEvent::IsolatedMarginAdjusted {
                delta, logical_ts, ..
            } => {
                if n.state != PositionState::Open
                    || n.leverage.leverage_type != MarginMode::Isolated
                {
                    return Err(PositionError::InvalidMarginConfiguration);
                }
                let cur = n
                    .leverage
                    .raw_usd
                    .ok_or(PositionError::InvalidMarginConfiguration)?;
                let after = cur
                    .checked_add(delta)
                    .ok_or(PositionError::ArithmeticOverflow)?;
                if after < Decimal::ZERO {
                    return Err(PositionError::InvalidIsolatedMargin);
                }
                n.leverage.raw_usd = Some(after);
                n.cum_funding.since_change = Decimal::ZERO;
                n.version = lsn;
                n.updated_at = logical_ts;
                PositionOutput::IsolatedMarginApplied {
                    margin_after: after,
                }
            }
        };
        *self = n;
        Ok((self.state, out))
    }
}

/// 单个活动全仓仓位在给定 mark price 下的风险事实。
///
/// 该结果只包含持仓风险，不包含挂单保证金占用。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrossMarginPositionRisk {
    /// 仓位绝对名义价值：`abs(size) * mark_price`。
    pub notional: Decimal,
    /// 未实现盈亏：`(mark_price - entry_price) * signed_size`。
    pub unrealized_pnl: Decimal,
    /// 初始保证金占用：`notional / leverage`，不包含挂单保证金。
    pub initial_margin_used: Decimal,
    /// 维持保证金占用：`notional * maintenance_margin_rate`，不包含挂单保证金。
    pub maintenance_margin_used: Decimal,
}
impl PositionV4 {
    /// 无副作用地计算单个活动全仓仓位的风险。
    ///
    /// 要求仓位为非零 `Open` 全仓仓位、`mark_price > 0`、
    /// `0 < maintenance_margin_rate <= 1` 且杠杆非零；任一计算无法由
    /// [`Decimal`] 表示时返回 [`PositionError::ArithmeticOverflow`]。
    pub fn cross_margin_risk(
        &self,
        mark_price: Decimal,
        maintenance_margin_rate: Decimal,
    ) -> Result<CrossMarginPositionRisk, PositionError> {
        if self.state != PositionState::Open || self.szi.is_zero() {
            return Err(PositionError::InvalidState);
        }
        if self.leverage.leverage_type != MarginMode::Cross {
            return Err(PositionError::InvalidMarginConfiguration);
        }
        if mark_price <= Decimal::ZERO {
            return Err(PositionError::InvalidPrice);
        }
        if maintenance_margin_rate <= Decimal::ZERO || maintenance_margin_rate > Decimal::ONE {
            return Err(PositionError::InvalidMaintenanceMarginRate);
        }
        if self.leverage.value == 0 {
            return Err(PositionError::InvalidLeverage);
        }

        let notional = self
            .szi
            .abs()
            .checked_mul(mark_price)
            .ok_or(PositionError::ArithmeticOverflow)?;
        let unrealized_pnl = mark_price
            .checked_sub(self.entry_px)
            .and_then(|v| v.checked_mul(self.szi))
            .ok_or(PositionError::ArithmeticOverflow)?;
        let initial_margin_used = notional
            .checked_div(Decimal::from(self.leverage.value))
            .ok_or(PositionError::ArithmeticOverflow)?;
        let maintenance_margin_used = notional
            .checked_mul(maintenance_margin_rate)
            .ok_or(PositionError::ArithmeticOverflow)?;

        Ok(CrossMarginPositionRisk {
            notional,
            unrealized_pnl,
            initial_margin_used,
            maintenance_margin_used,
        })
    }
}
impl TrackDiff for PositionV4 {
    fn diff(&self, o: &Self) -> Vec<EntityFieldChange> {
        [
            (
                "state",
                format!("{:?}", self.state),
                format!("{:?}", o.state),
            ),
            ("szi", self.szi.to_string(), o.szi.to_string()),
            (
                "entry_px",
                self.entry_px.to_string(),
                o.entry_px.to_string(),
            ),
            (
                "leverage",
                format!("{:?}", self.leverage),
                format!("{:?}", o.leverage),
            ),
            (
                "cum_funding",
                format!("{:?}", self.cum_funding),
                format!("{:?}", o.cum_funding),
            ),
            (
                "updated_at",
                self.updated_at.to_string(),
                o.updated_at.to_string(),
            ),
        ]
        .into_iter()
        .filter(|(_, a, b)| a != b)
        .map(|(n, a, b)| EntityFieldChange::new(n, a, b))
        .collect()
    }
}
impl Shard for PositionV4 {
    fn partition(&self) -> u8 {
        (self.account_id % 256) as u8
    }
}
impl Entity for PositionV4 {
    fn entity_id(&self) -> u64 {
        self.position_id
    }
    fn is_derived() -> bool {
        false
    }
    fn entity_type() -> u8 {
        0xB4
    }
    fn lsn(&self) -> u64 {
        self.version
    }
    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::PartyPlaceThing
    }
    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::VersionedMutable
    }
    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }
    fn replay_field_type(_: &str) -> u8 {
        1
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

#[cfg(test)]
mod tests {
    use common_entity::Fsm;
    use rust_decimal::Decimal;

    use super::{
        FillSource, Leverage, MarginMode, PositionError, PositionEvent, PositionFill,
        PositionState, PositionV4,
    };
    use crate::account::coin::Coin;

    fn open_position(size: Decimal, entry_price: Decimal, leverage: u8) -> PositionV4 {
        let mut position = PositionV4::new(
            1,
            2,
            Coin::BTC,
            Leverage {
                leverage_type: MarginMode::Cross,
                value: leverage,
                raw_usd: None,
            },
            1,
        )
        .unwrap();
        position
            .apply(PositionEvent::TradeFilled {
                fill: PositionFill {
                    signed_quantity: size,
                    price: entry_price,
                    source: FillSource::Regular,
                },
                logical_ts: 2,
                lsn: 1,
            })
            .unwrap();
        position
    }

    #[test]
    fn constructors_preserve_coin() {
        let empty = PositionV4::empty(1, 2, Coin::BTC);
        assert_eq!(empty.coin(), Coin::BTC);

        let position = PositionV4::new(
            3,
            4,
            Coin::ETH,
            Leverage {
                leverage_type: MarginMode::Cross,
                value: 5,
                raw_usd: None,
            },
            10,
        )
        .expect("valid leverage should construct a position");
        assert_eq!(position.coin(), Coin::ETH);
    }

    #[test]
    fn command_lsn_can_jump_repeat_and_reach_max_while_stale_is_atomic() {
        let mut position = open_position(Decimal::ONE, Decimal::from(100), 2);
        position
            .apply(PositionEvent::FundingSettled {
                signed_amount: Decimal::ONE,
                logical_ts: 20,
                lsn: 20,
            })
            .unwrap();
        position
            .apply(PositionEvent::FundingSettled {
                signed_amount: Decimal::ONE,
                logical_ts: 21,
                lsn: 20,
            })
            .unwrap();
        assert_eq!(position.version(), 20);

        let before = position.clone();
        assert_eq!(
            position
                .apply(PositionEvent::FundingSettled {
                    signed_amount: Decimal::ONE,
                    logical_ts: 22,
                    lsn: 19,
                })
                .map(|_| ()),
            Err(PositionError::StaleLsn {
                incoming_lsn: 19,
                current_lsn: 20
            })
        );
        assert_eq!(position, before);

        position.version = u64::MAX;
        position
            .apply(PositionEvent::FundingSettled {
                signed_amount: Decimal::ONE,
                logical_ts: 23,
                lsn: u64::MAX,
            })
            .unwrap();
        assert_eq!(position.version(), u64::MAX);
    }

    #[test]
    fn calculates_long_and_short_cross_margin_risk_without_mutation() {
        let long = open_position(Decimal::from(2), Decimal::from(100), 10);
        let before = long.clone();
        let risk = long
            .cross_margin_risk(Decimal::from(110), Decimal::new(5, 2))
            .unwrap();
        assert_eq!(risk.notional, Decimal::from(220));
        assert_eq!(risk.unrealized_pnl, Decimal::from(20));
        assert_eq!(risk.initial_margin_used, Decimal::from(22));
        assert_eq!(risk.maintenance_margin_used, Decimal::from(11));
        assert_eq!(long, before);

        let short = open_position(Decimal::from(-3), Decimal::from(50), 5);
        let risk = short
            .cross_margin_risk(Decimal::from(60), Decimal::new(1, 1))
            .unwrap();
        assert_eq!(risk.notional, Decimal::from(180));
        assert_eq!(risk.unrealized_pnl, Decimal::from(-30));
        assert_eq!(risk.initial_margin_used, Decimal::from(36));
        assert_eq!(risk.maintenance_margin_used, Decimal::from(18));
    }

    #[test]
    fn rejects_inactive_isolated_and_invalid_risk_inputs() {
        let empty = PositionV4::empty(1, 2, Coin::BTC);
        assert_eq!(
            empty.cross_margin_risk(Decimal::ONE, Decimal::ONE),
            Err(PositionError::InvalidState)
        );

        let mut zero_size = open_position(Decimal::ONE, Decimal::ONE, 1);
        zero_size.szi = Decimal::ZERO;
        assert_eq!(
            zero_size.cross_margin_risk(Decimal::ONE, Decimal::ONE),
            Err(PositionError::InvalidState)
        );

        let mut isolated = open_position(Decimal::ONE, Decimal::ONE, 1);
        isolated.leverage = Leverage {
            leverage_type: MarginMode::Isolated,
            value: 1,
            raw_usd: Some(Decimal::ZERO),
        };
        assert_eq!(
            isolated.cross_margin_risk(Decimal::ONE, Decimal::ONE),
            Err(PositionError::InvalidMarginConfiguration)
        );

        let position = open_position(Decimal::ONE, Decimal::ONE, 1);
        for mark_price in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
            assert_eq!(
                position.cross_margin_risk(mark_price, Decimal::ONE),
                Err(PositionError::InvalidPrice)
            );
        }
        for rate in [Decimal::ZERO, Decimal::NEGATIVE_ONE, Decimal::from(2)] {
            assert_eq!(
                position.cross_margin_risk(Decimal::ONE, rate),
                Err(PositionError::InvalidMaintenanceMarginRate)
            );
        }

        let mut zero_leverage = position;
        zero_leverage.leverage.value = 0;
        assert_eq!(
            zero_leverage.cross_margin_risk(Decimal::ONE, Decimal::ONE),
            Err(PositionError::InvalidLeverage)
        );
    }

    #[test]
    fn rejects_cross_margin_risk_arithmetic_overflow() {
        let mut notional_overflow = open_position(Decimal::ONE, Decimal::ONE, 1);
        notional_overflow.szi = Decimal::MAX;
        assert_eq!(
            notional_overflow.cross_margin_risk(Decimal::from(2), Decimal::ONE),
            Err(PositionError::ArithmeticOverflow)
        );

        let mut pnl_overflow = open_position(Decimal::ONE, Decimal::ONE, 1);
        pnl_overflow.entry_px = Decimal::NEGATIVE_ONE;
        assert_eq!(
            pnl_overflow.cross_margin_risk(Decimal::MAX, Decimal::ONE),
            Err(PositionError::ArithmeticOverflow)
        );

        let mut flat = open_position(Decimal::ONE, Decimal::ONE, 1);
        flat.state = PositionState::Flat;
        assert_eq!(
            flat.cross_margin_risk(Decimal::ONE, Decimal::ONE),
            Err(PositionError::InvalidState)
        );
    }
}
