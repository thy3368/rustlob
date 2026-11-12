use common_entity::{
    AggregateRole, Entity, EntityFieldChange, EntityMutationModel, FieldDiff, FourColorArchetype,
    Fsm, Shard,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::common::common_order::OrderBase;
use crate::entity::common::common_trade::TradeBase;
use crate::entity::{OrderSide, OrderTif, PerpTradeKind, PerpTradeV3, SpotOrderStatus};

const PERP_ORDER_V3_ENTITY_TYPE: u8 = 9;

/// 永续订单生命周期事件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpOrderV3Event {
    Match {
        makers: Vec<PerpOrderV3>,
        input: MatchPerpOrderV3Input,
        timestamp: u64,
    },
    ReplaceOrFill {
        price: Decimal,
        size: Decimal,
        makers: Vec<PerpOrderV3>,
        input: MatchPerpOrderV3Input,
        timestamp: u64,
    },
    Modify {
        price: Decimal,
        size: Decimal,
        timestamp: u64,
    },
    Cancel {
        timestamp: u64,
    },
}

/// 永续订单撮合所需的稳定业务输入。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchPerpOrderV3Input {
    pub match_id: u64,
    pub maker_fee_rate: Decimal,
    pub taker_fee_rate: Decimal,
    pub timestamp: u64,
}

/// 永续订单撮合后的订单与成交事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchPerpOrderV3Output {
    Resting {
        taker_order_after: PerpOrderV3,
    },
    PartiallyFilled {
        taker_order_after: PerpOrderV3,
        maker_orders_after: Vec<PerpOrderV3>,
        created_trades: Vec<PerpTradeV3>,
    },
    Filled {
        filled_taker_order_after: PerpOrderV3,
        maker_orders_after: Vec<PerpOrderV3>,
        created_trades: Vec<PerpTradeV3>,
    },
    CanceledAfterPartialFill {
        canceled_taker_order_after: PerpOrderV3,
        maker_orders_after: Vec<PerpOrderV3>,
        created_trades: Vec<PerpTradeV3>,
    },
    Rejected {
        rejected_taker_order_after: PerpOrderV3,
    },
}

impl MatchPerpOrderV3Output {
    pub fn into_parts(self) -> (PerpOrderV3, Vec<PerpOrderV3>, Vec<PerpTradeV3>) {
        match self {
            Self::Resting { taker_order_after } => (taker_order_after, Vec::new(), Vec::new()),
            Self::Rejected { rejected_taker_order_after } => {
                (rejected_taker_order_after, Vec::new(), Vec::new())
            }
            Self::PartiallyFilled { taker_order_after, maker_orders_after, created_trades } => {
                (taker_order_after, maker_orders_after, created_trades)
            }
            Self::Filled { filled_taker_order_after, maker_orders_after, created_trades } => {
                (filled_taker_order_after, maker_orders_after, created_trades)
            }
            Self::CanceledAfterPartialFill {
                canceled_taker_order_after,
                maker_orders_after,
                created_trades,
            } => (canceled_taker_order_after, maker_orders_after, created_trades),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PerpOrderV3Error {
    #[error("order is terminal")]
    Terminal,
    #[error("order facts are invalid")]
    InvalidOrder,
    #[error("modified order is invalid")]
    InvalidModification,
    #[error("fee rate or match id is invalid")]
    InvalidFeeRate,
    #[error("event timestamp precedes an order timestamp")]
    TimestampRegression,
    #[error("arithmetic overflow while changing order")]
    ArithmeticOverflow,
    #[error("entity version overflow at {version}")]
    VersionOverflow { version: u64 },
}

/// 永续订单实体，共享订单价格、数量与触发价的语义单位。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpOrderV3 {
    pub comon_order: OrderBase,
    /// 是否仅允许减少既有敞口。实体保存意图，不在订单内模拟仓位。
    pub reduce_only: bool,
    /// 是否是强平流程发出的风险订单。
    pub is_liquidation: bool,
}

impl Fsm for PerpOrderV3 {
    type State = SpotOrderStatus;
    type BizEvent = PerpOrderV3Event;
    type Output = Option<MatchPerpOrderV3Output>;
    type Error = PerpOrderV3Error;

    fn transition(
        &mut self,
        event: Self::BizEvent,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        if is_terminal(self.comon_order.status) {
            return Err(PerpOrderV3Error::Terminal);
        }
        let (next, output) = match event {
            PerpOrderV3Event::Match { makers, mut input, timestamp } => {
                input.timestamp = timestamp;
                let (next, output) = compute_match(self, makers, input)?;
                (next, Some(output))
            }
            PerpOrderV3Event::ReplaceOrFill { price, size, makers, mut input, timestamp } => {
                let replaced = modified(self, price, size, timestamp, false)?;
                input.timestamp = timestamp;
                let (next, output) = compute_match(&replaced, makers, input)?;
                (next, Some(output))
            }
            PerpOrderV3Event::Modify { price, size, timestamp } => {
                (modified(self, price, size, timestamp, true)?, None)
            }
            PerpOrderV3Event::Cancel { timestamp } => {
                validate_timestamp(&self.comon_order, timestamp)?;
                let mut next = self.clone();
                next.comon_order.status = SpotOrderStatus::Canceled;
                advance(&mut next.comon_order, timestamp)?;
                (next, None)
            }
        };
        *self = next;
        Ok((self.comon_order.status, output))
    }
}

fn compute_match(
    current: &PerpOrderV3,
    mut makers: Vec<PerpOrderV3>,
    input: MatchPerpOrderV3Input,
) -> Result<(PerpOrderV3, MatchPerpOrderV3Output), PerpOrderV3Error> {
    validate_order(current)?;
    validate_timestamp(&current.comon_order, input.timestamp)?;
    if input.match_id == 0
        || input.maker_fee_rate < Decimal::ZERO
        || input.taker_fee_rate < Decimal::ZERO
    {
        return Err(PerpOrderV3Error::InvalidFeeRate);
    }
    for maker in &makers {
        validate_maker(maker)?;
        validate_timestamp(&maker.comon_order, input.timestamp)?;
    }

    let mut taker = current.clone();
    if taker.comon_order.effective_tif() == OrderTif::Alo
        && makers.iter().any(|maker| compatible(&taker, maker) && crosses(&taker, maker))
    {
        taker.comon_order.status = SpotOrderStatus::Rejected;
        advance(&mut taker.comon_order, input.timestamp)?;
        return Ok((
            taker.clone(),
            MatchPerpOrderV3Output::Rejected { rejected_taker_order_after: taker },
        ));
    }

    let mut trades = Vec::new();
    for maker in &mut makers {
        if taker.comon_order.size == Decimal::ZERO {
            break;
        }
        if !compatible(&taker, maker) || !crosses(&taker, maker) {
            continue;
        }
        let fill = taker.comon_order.size.min(maker.comon_order.size);
        let notional = maker
            .comon_order
            .limit_px
            .checked_mul(fill)
            .ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
        let taker_fee = notional
            .checked_mul(input.taker_fee_rate)
            .ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
        let maker_fee = notional
            .checked_mul(input.maker_fee_rate)
            .ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
        taker.comon_order.size =
            taker.comon_order.size.checked_sub(fill).ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
        maker.comon_order.size =
            maker.comon_order.size.checked_sub(fill).ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
        maker.comon_order.status = if maker.comon_order.size == Decimal::ZERO {
            SpotOrderStatus::Filled
        } else {
            SpotOrderStatus::PartiallyFilled
        };
        advance(&mut maker.comon_order, input.timestamp)?;
        let ordinal = u64::try_from(trades.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
        let trade_id = input
            .match_id
            .checked_mul(1_000_000)
            .and_then(|base| base.checked_add(ordinal))
            .ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
        trades.push(PerpTradeV3 {
            trade_base: TradeBase {
                trade_id,
                symbol: taker.comon_order.symbol,
                taker_user: taker.comon_order.owner,
                maker_user: maker.comon_order.owner,
                taker_order_id: taker.comon_order.order_id,
                maker_order_id: maker.comon_order.order_id,
                taker_side: taker.comon_order.side,
                price: maker.comon_order.limit_px,
                size: fill,
                taker_fee,
                maker_fee,
                created_at: input.timestamp,
                updated_at: input.timestamp,
                version: 0,
            },
            state: crate::entity::PerpTradeV3State::Unsettled,
            kind: if taker.is_liquidation {
                PerpTradeKind::Liquidation
            } else {
                PerpTradeKind::Normal
            },
            reference_id: Some(input.match_id.to_string()),
            taker_start_position: Decimal::ZERO,
            maker_start_position: Decimal::ZERO,
            taker_closed_pnl: Decimal::ZERO,
            maker_closed_pnl: Decimal::ZERO,
            liquidation: None,
        });
    }

    taker.comon_order.status = if taker.comon_order.size == Decimal::ZERO {
        SpotOrderStatus::Filled
    } else if trades.is_empty() {
        if taker.comon_order.effective_tif() == OrderTif::Ioc {
            SpotOrderStatus::Rejected
        } else if taker.comon_order.status == SpotOrderStatus::Pending {
            SpotOrderStatus::Open
        } else {
            taker.comon_order.status
        }
    } else if taker.comon_order.effective_tif() == OrderTif::Ioc {
        SpotOrderStatus::Canceled
    } else {
        SpotOrderStatus::PartiallyFilled
    };
    advance(&mut taker.comon_order, input.timestamp)?;

    let outcome = match taker.comon_order.status {
        SpotOrderStatus::Filled => MatchPerpOrderV3Output::Filled {
            filled_taker_order_after: taker.clone(),
            maker_orders_after: makers,
            created_trades: trades,
        },
        SpotOrderStatus::Rejected => {
            MatchPerpOrderV3Output::Rejected { rejected_taker_order_after: taker.clone() }
        }
        SpotOrderStatus::Canceled => MatchPerpOrderV3Output::CanceledAfterPartialFill {
            canceled_taker_order_after: taker.clone(),
            maker_orders_after: makers,
            created_trades: trades,
        },
        SpotOrderStatus::PartiallyFilled => MatchPerpOrderV3Output::PartiallyFilled {
            taker_order_after: taker.clone(),
            maker_orders_after: makers,
            created_trades: trades,
        },
        SpotOrderStatus::Pending | SpotOrderStatus::Open => {
            MatchPerpOrderV3Output::Resting { taker_order_after: taker.clone() }
        }
    };
    Ok((taker, outcome))
}

fn modified(
    current: &PerpOrderV3,
    price: Decimal,
    total_size: Decimal,
    timestamp: u64,
    bump_version: bool,
) -> Result<PerpOrderV3, PerpOrderV3Error> {
    validate_order(current)?;
    validate_timestamp(&current.comon_order, timestamp)?;
    let filled = current
        .comon_order
        .orig_size
        .checked_sub(current.comon_order.size)
        .ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
    if price <= Decimal::ZERO || total_size <= filled {
        return Err(PerpOrderV3Error::InvalidModification);
    }
    let mut next = current.clone();
    next.comon_order.limit_px = price;
    next.comon_order.orig_size = total_size;
    next.comon_order.size =
        total_size.checked_sub(filled).ok_or(PerpOrderV3Error::ArithmeticOverflow)?;
    next.comon_order.updated_at = timestamp;
    if bump_version {
        next.comon_order.version = checked_version(current.comon_order.version)?;
    }
    Ok(next)
}

fn validate_order(order: &PerpOrderV3) -> Result<(), PerpOrderV3Error> {
    let base = &order.comon_order;
    if !base.has_consistent_execution_facts()
        || base.orig_size <= Decimal::ZERO
        || base.size < Decimal::ZERO
        || base.size > base.orig_size
        || base.limit_px <= Decimal::ZERO
        || is_terminal(base.status)
    {
        return Err(PerpOrderV3Error::InvalidOrder);
    }
    Ok(())
}

fn validate_maker(order: &PerpOrderV3) -> Result<(), PerpOrderV3Error> {
    let base = &order.comon_order;
    if !base.has_consistent_execution_facts()
        || base.orig_size <= Decimal::ZERO
        || base.size <= Decimal::ZERO
        || base.size > base.orig_size
        || base.limit_px <= Decimal::ZERO
    {
        return Err(PerpOrderV3Error::InvalidOrder);
    }
    Ok(())
}

fn validate_timestamp(order: &OrderBase, timestamp: u64) -> Result<(), PerpOrderV3Error> {
    if timestamp < order.created_at || timestamp < order.updated_at {
        Err(PerpOrderV3Error::TimestampRegression)
    } else {
        Ok(())
    }
}

fn compatible(taker: &PerpOrderV3, maker: &PerpOrderV3) -> bool {
    maker.comon_order.order_id != taker.comon_order.order_id
        && maker.comon_order.side != taker.comon_order.side
        && maker.comon_order.symbol == taker.comon_order.symbol
        && matches!(
            maker.comon_order.status,
            SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        )
}

fn crosses(taker: &PerpOrderV3, maker: &PerpOrderV3) -> bool {
    match taker.comon_order.side {
        OrderSide::Buy => taker.comon_order.limit_px >= maker.comon_order.limit_px,
        OrderSide::Sell => taker.comon_order.limit_px <= maker.comon_order.limit_px,
    }
}

fn advance(order: &mut OrderBase, timestamp: u64) -> Result<(), PerpOrderV3Error> {
    order.version = checked_version(order.version)?;
    order.updated_at = timestamp;
    Ok(())
}

fn checked_version(version: u64) -> Result<u64, PerpOrderV3Error> {
    version.checked_add(1).ok_or(PerpOrderV3Error::VersionOverflow { version })
}

fn is_terminal(status: SpotOrderStatus) -> bool {
    matches!(
        status,
        SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
    )
}

impl FieldDiff for PerpOrderV3 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        fields(self, "")
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        values(self)
            .into_iter()
            .zip(values(other))
            .filter(|((_, before), (_, after))| before != after)
            .map(|((name, before), (_, after))| EntityFieldChange::new(name, before, after))
            .collect()
    }
}

fn fields(order: &PerpOrderV3, before: &str) -> Vec<EntityFieldChange> {
    values(order)
        .into_iter()
        .map(|(name, after)| EntityFieldChange::new(name, before, after))
        .collect()
}

fn values(order: &PerpOrderV3) -> Vec<(&'static str, String)> {
    let base = &order.comon_order;
    vec![
        ("owner", base.owner.0.to_string()),
        ("symbol", base.symbol.id().to_string()),
        ("side", format!("{:?}", base.side)),
        ("limit_px", base.limit_px.to_string()),
        ("size", base.size.to_string()),
        ("orig_size", base.orig_size.to_string()),
        ("status", format!("{:?}", base.status)),
        ("reduce_only", order.reduce_only.to_string()),
        ("is_liquidation", order.is_liquidation.to_string()),
        ("created_at", base.created_at.to_string()),
        ("updated_at", base.updated_at.to_string()),
        ("version", base.version.to_string()),
    ]
}

impl Shard for PerpOrderV3 {
    fn partition(&self) -> u8 {
        (self.comon_order.owner.0 % 256) as u8
    }

    fn party_id(&self) -> u64 {
        self.comon_order.owner.0
    }
}

impl Entity for PerpOrderV3 {
    fn entity_id(&self) -> u64 {
        self.comon_order.order_id
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8 {
        PERP_ORDER_V3_ENTITY_TYPE
    }

    fn lsn(&self) -> u64 {
        self.comon_order.version
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
            "limit_px" | "size" | "orig_size" | "version" => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
#[path = "perp_order_v3_tests.rs"]
mod tests;
