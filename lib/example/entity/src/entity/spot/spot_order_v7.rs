use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff, Fsm, Shard};
use rust_decimal::Decimal;

use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::entity::common::common_order::{OrderBase, SpotAmountConversionError};
use crate::entity::common::common_trade::TradeBase;
use crate::entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
use crate::entity::spot::spot_order_v2::OrderGroupRelation;
use crate::spot::spot_trade_v3::{SpotTradeSettlementStatus, SpotTradeV3};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV7Event {
    Match { makers_after: Vec<SpotOrderV7>, input: MatchSpotOrderV7Input, timestamp: u64 },
    Modify { price: Decimal, size: Decimal, timestamp: u64 },
    Cancel { timestamp: u64 },
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

    pub fn match_with_input(
        &mut self,
        makers_after: Vec<SpotOrderV7>,
        input: MatchSpotOrderV7Input,
    ) -> Result<Option<MatchSpotOrderV7Outcome>, EntityError> {
        self.transition(SpotOrderV7Event::Match { makers_after, timestamp: input.timestamp, input })
            .map(|(_, output)| output)
    }

    pub fn modify(
        &mut self,
        price: Decimal,
        size: Decimal,
        timestamp: u64,
    ) -> Result<(), EntityError> {
        self.transition(SpotOrderV7Event::Modify { price, size, timestamp }).map(|_| ())
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
            SpotOrderV7Event::Match { makers_after, input, timestamp } => {
                let mut adjusted_input = input;
                adjusted_input.timestamp = timestamp;
                let (next, outcome) =
                    compute_match(&self.order_base, makers_after, adjusted_input)?;
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
mod tests {
    use common_entity::Fsm;
    use rust_decimal::Decimal;

    use super::{MatchSpotOrderV7Input, MatchSpotOrderV7Outcome, SpotOrderV7, SpotOrderV7Event};
    use crate::entity::common::common_order::OrderBase;
    use crate::{OrderSide, SpotOrderStatus};

    fn order(
        id: u64,
        side: OrderSide,
        size: i64,
        status: SpotOrderStatus,
        version: u64,
    ) -> SpotOrderV7 {
        let record = OrderBase {
            order_id: id,
            side,
            limit_px: Decimal::from(100),
            size: Decimal::from(size),
            orig_size: Decimal::from(size),
            status,
            version,
            ..OrderBase::default()
        };
        SpotOrderV7::from_record(record).unwrap()
    }

    fn input() -> MatchSpotOrderV7Input {
        MatchSpotOrderV7Input {
            trade_id_start: 1000,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
            timestamp: 42,
        }
    }

    #[test]
    fn match_returns_output_and_advances_each_changed_order_once() {
        let mut taker = order(1, OrderSide::Buy, 2, SpotOrderStatus::Pending, 7);
        let maker_a = order(2, OrderSide::Sell, 1, SpotOrderStatus::Open, 3);
        let maker_b = order(3, OrderSide::Sell, 1, SpotOrderStatus::Open, 5);

        let (state, result) = taker
            .transition(SpotOrderV7Event::Match {
                makers_after: vec![maker_a, maker_b],
                input: input(),
                timestamp: 42,
            })
            .unwrap();
        let MatchSpotOrderV7Outcome::Filled {
            filled_taker_order_after,
            maker_orders_after,
            created_trades,
        } = result.unwrap()
        else {
            panic!("expected filled outcome")
        };

        assert_eq!(state, SpotOrderStatus::Filled);
        assert_eq!(filled_taker_order_after.order_base().version, 8);
        assert_eq!(created_trades.len(), 2);
        assert_eq!(created_trades[0].comon_trade.trade_id, 1000);
        assert_eq!(created_trades[1].comon_trade.trade_id, 1001);
        assert_eq!(taker.order_base().version, 8);
        assert_eq!(maker_orders_after[0].order_base().version, 4);
        assert_eq!(maker_orders_after[1].order_base().version, 6);
        assert_eq!(maker_orders_after[0].status(), SpotOrderStatus::Filled);
        assert_eq!(maker_orders_after[1].status(), SpotOrderStatus::Filled);
    }

    #[test]
    fn failed_match_and_modify_preserve_the_complete_order() {
        let mut taker = order(1, OrderSide::Buy, 1, SpotOrderStatus::Pending, 7);
        let before = taker.clone();
        let maker = order(2, OrderSide::Sell, 1, SpotOrderStatus::Open, 3);
        let mut costly_input = input();
        costly_input.taker_fee_bps = 1;

        assert!(taker.match_with_input(vec![maker], costly_input).is_err());
        assert_eq!(taker, before);

        assert!(taker.modify(Decimal::ZERO, Decimal::ONE, 50).is_err());
        assert_eq!(taker, before);
    }

    #[test]
    fn every_terminal_state_rejects_every_event_without_mutation() {
        for status in
            [SpotOrderStatus::Filled, SpotOrderStatus::Canceled, SpotOrderStatus::Rejected]
        {
            let events = [
                SpotOrderV7Event::Match { makers_after: Vec::new(), input: input(), timestamp: 42 },
                SpotOrderV7Event::Modify { price: Decimal::ONE, size: Decimal::ONE, timestamp: 42 },
                SpotOrderV7Event::Cancel { timestamp: 42 },
            ];
            for event in events {
                let mut order = order(1, OrderSide::Buy, 1, status, 7);
                let before = order.clone();
                assert!(order.transition(event).is_err());
                assert_eq!(order, before);
            }
        }
    }

    #[test]
    fn apply_ignores_non_match_output_and_commits_cancel() {
        let mut order = order(1, OrderSide::Buy, 1, SpotOrderStatus::Open, 7);

        order.apply(SpotOrderV7Event::Cancel { timestamp: 42 }).unwrap();

        assert_eq!(order.status(), SpotOrderStatus::Canceled);
        assert_eq!(order.order_base().version, 8);
        assert_eq!(order.order_base().updated_at, 42);
    }

    #[test]
    fn active_state_event_matrix_allows_match_modify_and_cancel() {
        for initial in
            [SpotOrderStatus::Pending, SpotOrderStatus::Open, SpotOrderStatus::PartiallyFilled]
        {
            let mut matching = order(1, OrderSide::Buy, 1, initial, 0);
            let (state, output) = matching
                .transition(SpotOrderV7Event::Match {
                    makers_after: Vec::new(),
                    input: input(),
                    timestamp: 42,
                })
                .unwrap();
            let expected_match_state =
                if initial == SpotOrderStatus::Pending { SpotOrderStatus::Open } else { initial };
            assert_eq!(state, expected_match_state);
            assert!(matches!(output.unwrap(), MatchSpotOrderV7Outcome::Resting { .. }));

            let mut modifying = order(1, OrderSide::Buy, 1, initial, 0);
            assert_eq!(
                modifying
                    .transition(SpotOrderV7Event::Modify {
                        price: Decimal::from(101),
                        size: Decimal::from(2),
                        timestamp: 42,
                    })
                    .unwrap()
                    .0,
                initial
            );

            let mut canceling = order(1, OrderSide::Buy, 1, initial, 0);
            assert_eq!(
                canceling.transition(SpotOrderV7Event::Cancel { timestamp: 42 }).unwrap().0,
                SpotOrderStatus::Canceled
            );
        }
    }

    #[test]
    fn version_overflow_preserves_order_for_each_active_event() {
        let events = [
            SpotOrderV7Event::Match { makers_after: Vec::new(), input: input(), timestamp: 42 },
            SpotOrderV7Event::Modify {
                price: Decimal::from(101),
                size: Decimal::from(2),
                timestamp: 42,
            },
            SpotOrderV7Event::Cancel { timestamp: 42 },
        ];
        for event in events {
            let mut order = order(1, OrderSide::Buy, 1, SpotOrderStatus::Pending, u64::MAX);
            let before = order.clone();

            assert!(order.transition(event).is_err());
            assert_eq!(order, before);
        }
    }
}
