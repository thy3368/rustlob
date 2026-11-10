use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff, Fsm, Shard};
use lob_repo_v2::lob::LobOrder;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::entity::common::common_order::{OrderBase, SpotAmountConversionError};
use crate::entity::common::common_trade::TradeBase;
use crate::entity::spot::spot_order_primitives::{
    OrderGroupRelation, OrderSide, OrderTif, SpotOrderStatus,
};
use crate::spot::spot_trade_v3::{SpotTradeSettlementStatus, SpotTradeV3};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV7Event {
    Match {
        makers: Vec<SpotOrderV7>,
        input: MatchSpotOrderV7Input,
        timestamp: u64,
    },
    ReplaceOrFill {
        price: Decimal,
        size: Decimal,
        makers: Vec<SpotOrderV7>,
        input: MatchSpotOrderV7Input,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderV7Input {
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV7Outcome {
    Resting {
        taker_order_after: SpotOrderV7,
    },
    PartiallyFilled {
        taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
    },
    Filled {
        filled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
    },
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
    },
    Rejected {
        rejected_taker_order_after: SpotOrderV7,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV7 {
    order_base: OrderBase,
}

impl SpotOrderV7 {
    pub fn single(mut order: OrderBase) -> Self {
        order.status = SpotOrderStatus::Pending;
        order.group_relation = OrderGroupRelation::Standalone;
        Self { order_base: order }
    }

    pub fn normal_tpsl(parent: OrderBase, children: Vec<OrderBase>) -> (Self, Vec<Self>) {
        let mut parent = Self::single(parent);
        parent.order_base.group_relation = OrderGroupRelation::NormalTpslParent;
        let id = parent.order_base.order_id;
        let children = children
            .into_iter()
            .map(|mut child| {
                child.status = SpotOrderStatus::Pending;
                child.group_relation = OrderGroupRelation::NormalTpslChild { parent_order_id: id };
                Self { order_base: child }
            })
            .collect();
        (parent, children)
    }

    pub fn from_record(order: OrderBase) -> Result<Self, EntityError> {
        validate_record(&order)?;
        Ok(Self { order_base: order })
    }
    pub fn order_base(&self) -> &OrderBase {
        &self.order_base
    }
    pub fn status(&self) -> SpotOrderStatus {
        self.order_base.status
    }
    pub fn freeze_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        freeze_requirement(&self.order_base)
    }
    pub fn release_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        if !matches!(
            self.status(),
            SpotOrderStatus::Canceled | SpotOrderStatus::Filled | SpotOrderStatus::Rejected
        ) {
            return Err(SpotAmountConversionError::OrderNotCanceled);
        }
        let original = self.status() == SpotOrderStatus::Rejected;
        release_requirement(&self.order_base, original)
    }

    pub fn track_create_event(&self) -> Result<common_entity::EntityReplayableEvent, EntityError> {
        Entity::track_create_event(self)
    }

    ///todo 可以remove
    pub fn match_with_input(
        &mut self,
        makers: Vec<SpotOrderV7>,
        input: MatchSpotOrderV7Input,
    ) -> Result<Option<MatchSpotOrderV7Outcome>, EntityError> {
        self.transition(SpotOrderV7Event::Match { makers, timestamp: input.timestamp, input })
            .map(|(_, output)| output)
    }

    ///todo 可以remove
    pub fn modify(
        &mut self,
        price: Decimal,
        size: Decimal,
        timestamp: u64,
    ) -> Result<(), EntityError> {
        self.transition(SpotOrderV7Event::Modify { price, size, timestamp }).map(|_| ())
    }

    ///todo 可以remove
    pub fn replace_or_fill(
        &mut self,
        price: Decimal,
        size: Decimal,
        makers: Vec<SpotOrderV7>,
        input: MatchSpotOrderV7Input,
        timestamp: u64,
    ) -> Result<Option<MatchSpotOrderV7Outcome>, EntityError> {
        self.transition(SpotOrderV7Event::ReplaceOrFill { price, size, makers, input, timestamp })
            .map(|(_, output)| output)
    }
}

impl Fsm for SpotOrderV7 {
    type State = SpotOrderStatus;
    type Event = SpotOrderV7Event;
    type Output = Option<MatchSpotOrderV7Outcome>;
    type Error = EntityError;

    fn transition(
        &mut self,
        event: SpotOrderV7Event,
    ) -> Result<(Self::State, Self::Output), EntityError> {
        if matches!(
            self.order_base.status,
            SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
        ) {
            return Err(err("order is terminal"));
        }
        let (next, output) = match event {
            SpotOrderV7Event::Match { makers, input, timestamp } => {
                let mut adjusted_input = input;
                adjusted_input.timestamp = timestamp;
                let (next, outcome) = compute_match(&self.order_base, makers, adjusted_input)?;
                (next, Some(outcome))
            }
            SpotOrderV7Event::ReplaceOrFill { price, size, makers, input, timestamp } => {
                let replaced =
                    modified_order_without_version(&self.order_base, price, size, timestamp)?;
                validate_replace_makers(&replaced, &makers)?;
                let mut adjusted_input = input;
                adjusted_input.timestamp = timestamp;
                let (next, outcome) = compute_match(&replaced, makers, adjusted_input)?;
                (next, Some(outcome))
            }
            SpotOrderV7Event::Modify { price, size, timestamp } => {
                (modified_order(&self.order_base, price, size, timestamp)?, None)
            }
            SpotOrderV7Event::Cancel { timestamp } => {
                (canceled_order(&self.order_base, timestamp)?, None)
            }
        };
        self.order_base = next;
        Ok((self.order_base.status, output))
    }
}

fn modified_order(
    current: &OrderBase,
    price: Decimal,
    size: Decimal,
    timestamp: u64,
) -> Result<OrderBase, EntityError> {
    let filled =
        current.orig_size.checked_sub(current.size).ok_or_else(|| err("invalid filled size"))?;
    if price <= Decimal::ZERO || size <= Decimal::ZERO || size < filled {
        return Err(err("invalid modified order"));
    }
    let mut next = current.clone();
    next.limit_px = price;
    next.orig_size = size;
    next.size = size.checked_sub(filled).ok_or_else(|| err("size overflow"))?;
    next.updated_at = timestamp;
    next.version = current
        .version
        .checked_add(1)
        .ok_or(EntityError::VersionOverflow { version: current.version })?;
    Ok(next)
}

fn modified_order_without_version(
    current: &OrderBase,
    price: Decimal,
    size: Decimal,
    timestamp: u64,
) -> Result<OrderBase, EntityError> {
    let filled =
        current.orig_size.checked_sub(current.size).ok_or_else(|| err("invalid filled size"))?;
    if price <= Decimal::ZERO || size <= Decimal::ZERO || size < filled {
        return Err(err("invalid modified order"));
    }
    let mut next = current.clone();
    next.limit_px = price;
    next.orig_size = size;
    next.size = size.checked_sub(filled).ok_or_else(|| err("size overflow"))?;
    next.updated_at = timestamp;
    Ok(next)
}

fn validate_replace_makers(taker: &OrderBase, makers: &[SpotOrderV7]) -> Result<(), EntityError> {
    for maker in makers {
        let order = &maker.order_base;
        validate_record(order)?;
        if order.order_id == taker.order_id
            || order.side == taker.side
            || order.symbol != taker.symbol
            || !matches!(order.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
            || !crosses(taker, order)
        {
            return Err(err("maker does not match replacement"));
        }
    }
    Ok(())
}

fn canceled_order(current: &OrderBase, timestamp: u64) -> Result<OrderBase, EntityError> {
    let mut next = current.clone();
    next.status = SpotOrderStatus::Canceled;
    next.updated_at = timestamp;
    next.version = current
        .version
        .checked_add(1)
        .ok_or(EntityError::VersionOverflow { version: current.version })?;
    Ok(next)
}

impl FieldDiff for SpotOrderV7 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.order_base.order_id.to_string()),
            EntityFieldChange::new("status", "", format!("{:?}", self.status())),
            EntityFieldChange::new("version", "", self.order_base.version.to_string()),
        ]
    }
    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let a = &self.order_base;
        let b = &other.order_base;
        let mut v = Vec::new();
        for (n, x, y) in [
            ("limit_px", a.limit_px.to_string(), b.limit_px.to_string()),
            ("size", a.size.to_string(), b.size.to_string()),
            ("orig_size", a.orig_size.to_string(), b.orig_size.to_string()),
            ("status", format!("{:?}", a.status), format!("{:?}", b.status)),
            ("updated_at", a.updated_at.to_string(), b.updated_at.to_string()),
            ("version", a.version.to_string(), b.version.to_string()),
        ] {
            if x != y {
                v.push(EntityFieldChange::new(n, x, y));
            }
        }
        v
    }
}
impl Entity for SpotOrderV7 {
    fn entity_id(&self) -> u64 {
        self.order_base.order_id
    }
    fn is_derived() -> bool {
        false
    }
    fn entity_type() -> u8 {
        16
    }
    fn lsn(&self) -> u64 {
        self.order_base.version
    }
    fn four_color_archetype() -> common_entity::FourColorArchetype {
        common_entity::FourColorArchetype::MomentInterval
    }
    fn mutation_model() -> common_entity::EntityMutationModel {
        common_entity::EntityMutationModel::VersionedMutable
    }
    fn aggregate_role() -> common_entity::AggregateRole {
        common_entity::AggregateRole::AggregateRoot
    }
    fn replay_field_type(name: &str) -> u8 {
        if matches!(name, "updated_at" | "version") { 1 } else { 0 }
    }
}
impl Shard for SpotOrderV7 {
    fn partition(&self) -> u8 {
        self.order_base.symbol.base.index() as u8
    }
}

impl LobOrder for SpotOrderV7 {
    type Side = OrderSide;
    fn entity_type() -> u8 {
        <Self as Entity>::entity_type()
    }
    fn entity_id(&self) -> u64 {
        self.order_base.order_id
    }
    fn asset(&self) -> u32 {
        self.order_base.symbol.id()
    }
    fn side(&self) -> Self::Side {
        self.order_base.side
    }
    fn is_buy_side(side: Self::Side) -> bool {
        side == OrderSide::Buy
    }
    fn price(&self) -> u64 {
        self.order_base.limit_px.to_u64().unwrap_or(0)
    }
    fn qty(&self) -> u64 {
        self.order_base.orig_size.to_u64().unwrap_or(0)
    }
    fn filled_qty(&self) -> u64 {
        (self.order_base.orig_size - self.order_base.size).to_u64().unwrap_or(0)
    }
    fn created_at(&self) -> u64 {
        self.order_base.created_at
    }
    fn entity_ver(&self) -> u64 {
        self.order_base.version
    }
    fn is_active(&self) -> bool {
        matches!(
            self.status(),
            SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) && self.order_base.size > Decimal::ZERO
    }
    fn from_replay_event(
        _: &common_entity::EntityReplayableEvent,
        _: u64,
    ) -> Result<Self, lob_repo_v2::LobError> {
        Err(lob_repo_v2::LobError::Event("SpotOrderV7 replay construction is not wired".into()))
    }
    fn apply_replay_event(
        self,
        _: &common_entity::EntityReplayableEvent,
    ) -> Result<Self, lob_repo_v2::LobError> {
        Err(lob_repo_v2::LobError::Event("SpotOrderV7 replay update is not wired".into()))
    }
}

fn err(s: &str) -> EntityError {
    EntityError::Custom(s.to_owned())
}
fn validate_record(o: &OrderBase) -> Result<(), EntityError> {
    if !o.has_consistent_execution_facts()
        || o.orig_size <= Decimal::ZERO
        || o.size < Decimal::ZERO
        || o.size > o.orig_size
        || o.limit_px <= Decimal::ZERO
    {
        return Err(err("invalid order facts"));
    }
    Ok(())
}
fn freeze_requirement(o: &OrderBase) -> Result<CoinAmount, SpotAmountConversionError> {
    if o.orig_size <= Decimal::ZERO || o.buy_quote_fee_reserve_limit < Decimal::ZERO {
        return Err(SpotAmountConversionError::InvalidAmount);
    }
    let amount = match o.side {
        OrderSide::Buy => o
            .limit_px
            .checked_mul(o.orig_size)
            .and_then(|x| x.checked_add(o.buy_quote_fee_reserve_limit))
            .ok_or(SpotAmountConversionError::ArithmeticOverflow)?,
        OrderSide::Sell => o.orig_size,
    };
    CoinAmount::new(if o.side == OrderSide::Buy { o.symbol.quote } else { o.symbol.base }, amount)
        .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
}
fn release_requirement(
    o: &OrderBase,
    original: bool,
) -> Result<CoinAmount, SpotAmountConversionError> {
    let size = if original { o.orig_size } else { o.size };
    let amount = match o.side {
        OrderSide::Buy => o
            .limit_px
            .checked_mul(size)
            .and_then(|x| x.checked_add(o.buy_quote_fee_reserve_limit))
            .and_then(|x| x.checked_sub(o.buy_quote_fee_reserve_consumed))
            .ok_or(SpotAmountConversionError::ArithmeticOverflow)?,
        OrderSide::Sell => size,
    };
    CoinAmount::new(if o.side == OrderSide::Buy { o.symbol.quote } else { o.symbol.base }, amount)
        .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
}
fn fee(n: Decimal, b: u64, scale: u32) -> Result<Decimal, EntityError> {
    Ok(n.checked_mul(Decimal::from(b))
        .and_then(|x| x.checked_div(Decimal::from(10_000u64)))
        .ok_or_else(|| err("fee overflow"))?
        .round_dp_with_strategy(scale, rust_decimal::RoundingStrategy::ToPositiveInfinity))
}
fn compute_match(
    t: &OrderBase,
    mut makers: Vec<SpotOrderV7>,
    i: MatchSpotOrderV7Input,
) -> Result<(OrderBase, MatchSpotOrderV7Outcome), EntityError> {
    validate_record(t)?;
    if !matches!(
        t.status,
        SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
    ) {
        return Err(err("order is not matchable"));
    }
    let mut taker = t.clone();
    let mut trades = Vec::new();
    if taker.effective_tif() == OrderTif::Alo
        && makers.iter().any(|m| m.order_base.order_id != t.order_id && crosses(&t, &m.order_base))
    {
        taker.status = SpotOrderStatus::Rejected;
        taker.version =
            t.version.checked_add(1).ok_or(EntityError::VersionOverflow { version: t.version })?;
        taker.updated_at = i.timestamp;
        return Ok((
            taker.clone(),
            MatchSpotOrderV7Outcome::Rejected {
                rejected_taker_order_after: SpotOrderV7 { order_base: taker },
            },
        ));
    }
    for maker in &mut makers {
        if taker.size == Decimal::ZERO {
            break;
        }
        validate_record(&maker.order_base)?;
        if maker.order_base.order_id == t.order_id
            || maker.order_base.side == t.side
            || maker.order_base.symbol != t.symbol
            || maker.order_base.status == SpotOrderStatus::Pending
            || !crosses(&taker, &maker.order_base)
        {
            continue;
        }
        let q = taker.size.min(maker.order_base.size);
        let n = maker.order_base.limit_px.checked_mul(q).ok_or_else(|| err("notional overflow"))?;
        let tf = fee(n, i.taker_fee_bps, t.symbol.quote.scale())?;
        let mf = fee(n, i.maker_fee_bps, t.symbol.quote.scale())?;
        taker.size = taker.size.checked_sub(q).ok_or_else(|| err("size underflow"))?;
        maker.order_base.size =
            maker.order_base.size.checked_sub(q).ok_or_else(|| err("size underflow"))?;
        if t.side == OrderSide::Buy {
            taker.buy_quote_fee_reserve_consumed = taker
                .buy_quote_fee_reserve_consumed
                .checked_add(tf)
                .ok_or_else(|| err("fee overflow"))?;
            if taker.buy_quote_fee_reserve_consumed > taker.buy_quote_fee_reserve_limit {
                return Err(err("fee exceeds reserve"));
            }
        }
        maker.order_base.version = maker
            .order_base
            .version
            .checked_add(1)
            .ok_or(EntityError::VersionOverflow { version: maker.order_base.version })?;
        taker.updated_at = i.timestamp;
        maker.order_base.updated_at = i.timestamp;
        maker.order_base.status = if maker.order_base.size == Decimal::ZERO {
            SpotOrderStatus::Filled
        } else {
            SpotOrderStatus::PartiallyFilled
        };
        trades.push(SpotTradeV3 {
            comon_trade: TradeBase {
                trade_id: i
                    .trade_id_start
                    .checked_add((trades.len()) as u64)
                    .ok_or_else(|| err("trade id overflow"))?,
                symbol: t.symbol,
                taker_user: t.owner,
                maker_user: maker.order_base.owner,
                taker_order_id: t.order_id,
                maker_order_id: maker.order_base.order_id,
                taker_side: t.side,
                price: maker.order_base.limit_px,
                size: q,
                taker_fee: tf,
                maker_fee: mf,
                created_at: i.timestamp,
                updated_at: i.timestamp,
                version: 0,
            },
            settlement_status: SpotTradeSettlementStatus::Unsettled,
        });
    }
    if taker.size == Decimal::ZERO {
        taker.status = SpotOrderStatus::Filled;
    } else if trades.is_empty() {
        if taker.effective_tif() == OrderTif::Ioc {
            taker.status = SpotOrderStatus::Rejected;
        } else {
            if taker.status == SpotOrderStatus::Pending {
                taker.status = SpotOrderStatus::Open
            }
        }
    } else if taker.effective_tif() == OrderTif::Ioc {
        taker.status = SpotOrderStatus::Canceled;
    } else {
        taker.status = SpotOrderStatus::PartiallyFilled;
    }
    taker.version =
        t.version.checked_add(1).ok_or(EntityError::VersionOverflow { version: t.version })?;
    taker.updated_at = i.timestamp;
    let after = SpotOrderV7 { order_base: taker };
    let next = after.order_base.clone();
    let outcome = if after.status() == SpotOrderStatus::Filled {
        MatchSpotOrderV7Outcome::Filled {
            filled_taker_order_after: after,
            maker_orders_after: makers,
            created_trades: trades,
        }
    } else if trades.is_empty() {
        if after.status() == SpotOrderStatus::Rejected {
            MatchSpotOrderV7Outcome::Rejected { rejected_taker_order_after: after }
        } else {
            MatchSpotOrderV7Outcome::Resting { taker_order_after: after }
        }
    } else if after.status() == SpotOrderStatus::Canceled {
        MatchSpotOrderV7Outcome::CanceledAfterPartialFill {
            canceled_taker_order_after: after,
            maker_orders_after: makers,
            created_trades: trades,
        }
    } else {
        MatchSpotOrderV7Outcome::PartiallyFilled {
            taker_order_after: after,
            maker_orders_after: makers,
            created_trades: trades,
        }
    };
    Ok((next, outcome))
}
fn crosses(t: &OrderBase, m: &OrderBase) -> bool {
    match t.side {
        OrderSide::Buy => t.limit_px >= m.limit_px,
        OrderSide::Sell => t.limit_px <= m.limit_px,
    }
}

#[cfg(test)]
#[path = "spot_order_v7_tests.rs"]
mod spot_order_v7_tests;
