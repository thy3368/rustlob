use common_entity::{
    AggregateRole, Entity, EntityFieldChange, EntityMutationModel, FieldDiff, FourColorArchetype,
    Fsm, Shard,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::common::common_trade::TradeBase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerpTradeKind {
    Normal,
    Liquidation,
    InsuranceFundTransfer,
    Adl,
}

/// 永续成交的结算生命周期。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerpTradeV3State {
    Unsettled,
    Settled,
}

/// 永续成交生命周期事件。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerpTradeV3Event {
    Settle {
        taker_start_position: Decimal,
        maker_start_position: Decimal,
        taker_closed_pnl: Decimal,
        maker_closed_pnl: Decimal,
        timestamp: u64,
    },
}

/// 永续成交结算产生的稳定业务事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerpTradeV3Output {
    Settled { taker_closed_pnl: Decimal, maker_closed_pnl: Decimal },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PerpTradeV3Error {
    #[error("trade was already settled")]
    AlreadySettled,
    #[error("trade facts are invalid")]
    InvalidTrade,
    #[error("settlement timestamp precedes a trade timestamp")]
    TimestampRegression,
    #[error("entity version overflow at {version}")]
    VersionOverflow { version: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpTradeV3 {
    pub trade_base: TradeBase,

    pub state: PerpTradeV3State,

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
    pub liquidation: Option<LiqMarker>, // 强平标记
}

impl Fsm for PerpTradeV3 {
    type State = PerpTradeV3State;
    type Event = PerpTradeV3Event;
    type Output = PerpTradeV3Output;
    type Error = PerpTradeV3Error;

    fn transition(
        &mut self,
        event: Self::Event,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        if self.state == PerpTradeV3State::Settled {
            return Err(PerpTradeV3Error::AlreadySettled);
        }
        let base = &self.trade_base;
        if base.price <= Decimal::ZERO
            || base.size <= Decimal::ZERO
            || base.taker_fee < Decimal::ZERO
            || base.maker_fee < Decimal::ZERO
        {
            return Err(PerpTradeV3Error::InvalidTrade);
        }

        match event {
            PerpTradeV3Event::Settle {
                taker_start_position,
                maker_start_position,
                taker_closed_pnl,
                maker_closed_pnl,
                timestamp,
            } => {
                if timestamp < base.created_at || timestamp < base.updated_at {
                    return Err(PerpTradeV3Error::TimestampRegression);
                }
                let version = base
                    .version
                    .checked_add(1)
                    .ok_or(PerpTradeV3Error::VersionOverflow { version: base.version })?;

                self.taker_start_position = taker_start_position;
                self.maker_start_position = maker_start_position;
                self.taker_closed_pnl = taker_closed_pnl;
                self.maker_closed_pnl = maker_closed_pnl;
                self.trade_base.updated_at = timestamp;
                self.trade_base.version = version;
                self.state = PerpTradeV3State::Settled;
                Ok((self.state, PerpTradeV3Output::Settled { taker_closed_pnl, maker_closed_pnl }))
            }
        }
    }
}

impl FieldDiff for PerpTradeV3 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        trade_values(self)
            .into_iter()
            .map(|(name, value)| EntityFieldChange::new(name, "", value))
            .collect()
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        trade_values(self)
            .into_iter()
            .zip(trade_values(other))
            .filter(|((_, before), (_, after))| before != after)
            .map(|((name, before), (_, after))| EntityFieldChange::new(name, before, after))
            .collect()
    }
}

fn trade_values(trade: &PerpTradeV3) -> Vec<(&'static str, String)> {
    let base = &trade.trade_base;
    vec![
        ("symbol", base.symbol.id().to_string()),
        ("taker_user", base.taker_user.0.to_string()),
        ("maker_user", base.maker_user.0.to_string()),
        ("price", base.price.to_string()),
        ("size", base.size.to_string()),
        ("taker_fee", base.taker_fee.to_string()),
        ("maker_fee", base.maker_fee.to_string()),
        ("kind", format!("{:?}", trade.kind)),
        ("state", format!("{:?}", trade.state)),
        ("taker_start_position", trade.taker_start_position.to_string()),
        ("maker_start_position", trade.maker_start_position.to_string()),
        ("taker_closed_pnl", trade.taker_closed_pnl.to_string()),
        ("maker_closed_pnl", trade.maker_closed_pnl.to_string()),
        ("created_at", base.created_at.to_string()),
        ("updated_at", base.updated_at.to_string()),
        ("version", base.version.to_string()),
    ]
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

    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::VersionedMutable
    }

    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "price"
            | "size"
            | "taker_fee"
            | "maker_fee"
            | "taker_start_position"
            | "maker_start_position"
            | "taker_closed_pnl"
            | "maker_closed_pnl"
            | "version" => 1,
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

#[cfg(test)]
mod tests {
    use common_entity::{Entity, Fsm};

    use super::*;
    use crate::OrderSide;
    use crate::account::coin::Coin;
    use crate::account::symbol::Symbol;
    use crate::entity::common::UserId;

    fn trade() -> PerpTradeV3 {
        PerpTradeV3 {
            trade_base: TradeBase {
                trade_id: 7,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDC),
                taker_user: UserId(10),
                maker_user: UserId(11),
                taker_order_id: 20,
                maker_order_id: 21,
                taker_side: OrderSide::Buy,
                price: Decimal::from(100),
                size: Decimal::from(2),
                taker_fee: Decimal::ONE,
                maker_fee: Decimal::ZERO,
                created_at: 5,
                updated_at: 6,
                version: 3,
            },
            state: PerpTradeV3State::Unsettled,
            kind: PerpTradeKind::Normal,
            reference_id: None,
            taker_start_position: Decimal::ZERO,
            maker_start_position: Decimal::ZERO,
            taker_closed_pnl: Decimal::ZERO,
            maker_closed_pnl: Decimal::ZERO,
            liquidation: None,
        }
    }

    fn settle_event(timestamp: u64) -> PerpTradeV3Event {
        PerpTradeV3Event::Settle {
            taker_start_position: Decimal::from(2),
            maker_start_position: Decimal::from(-3),
            taker_closed_pnl: Decimal::from(4),
            maker_closed_pnl: Decimal::from(-5),
            timestamp,
        }
    }

    #[test]
    fn unsettled_trade_settles_all_facts_once() {
        let mut trade = trade();
        let before = trade.clone();
        let result = trade.transition(settle_event(8)).unwrap();

        assert_eq!(result.0, PerpTradeV3State::Settled);
        assert_eq!(
            result.1,
            PerpTradeV3Output::Settled {
                taker_closed_pnl: Decimal::from(4),
                maker_closed_pnl: Decimal::from(-5),
            }
        );
        assert_eq!(trade.taker_start_position, Decimal::from(2));
        assert_eq!(trade.maker_start_position, Decimal::from(-3));
        assert_eq!(trade.trade_base.updated_at, 8);
        assert_eq!(trade.trade_base.version, 4);
        assert!(trade.diff(&before).len() >= 7);
        assert!(trade.track_update_event_from(&before).is_ok());
    }

    #[test]
    fn settled_trade_rejects_another_settlement_without_mutation() {
        let mut trade = trade();
        trade.transition(settle_event(8)).unwrap();
        let settled = trade.clone();

        assert_eq!(trade.transition(settle_event(9)), Err(PerpTradeV3Error::AlreadySettled));
        assert_eq!(trade, settled);
    }

    #[test]
    fn invalid_facts_timestamp_and_version_overflow_are_atomic() {
        let mut invalid = trade();
        invalid.trade_base.price = Decimal::ZERO;
        let before = invalid.clone();
        assert_eq!(invalid.transition(settle_event(8)), Err(PerpTradeV3Error::InvalidTrade));
        assert_eq!(invalid, before);

        let mut regressed = trade();
        let before = regressed.clone();
        assert_eq!(
            regressed.transition(settle_event(4)),
            Err(PerpTradeV3Error::TimestampRegression)
        );
        assert_eq!(regressed, before);

        let mut overflow = trade();
        overflow.trade_base.version = u64::MAX;
        let before = overflow.clone();
        assert_eq!(
            overflow.transition(settle_event(8)),
            Err(PerpTradeV3Error::VersionOverflow { version: u64::MAX })
        );
        assert_eq!(overflow, before);
    }

    #[test]
    fn trade_kind_does_not_change_settlement_lifecycle() {
        for kind in [PerpTradeKind::Normal, PerpTradeKind::Liquidation, PerpTradeKind::Adl] {
            let mut trade = trade();
            trade.kind = kind;
            assert!(trade.transition(settle_event(8)).is_ok());
            assert_eq!(trade.state, PerpTradeV3State::Settled);
        }
    }
}
