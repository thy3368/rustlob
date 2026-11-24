use std::collections::HashSet;

use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
use common_entity::{
    CreatedEntityEvent, EntityError, PositionedEntity, ReplayableDomainEvent,
    ReplayableEntityChanges, Shard, TrackDiff, UpdatedEntityEvents,
};
use example_core_entity::entity::common::common_trade::TradeBase;
use example_core_entity::spot::{SpotTradeSettlementStatus, SpotTradeV3};
use example_core_entity::{OrderSide, OrderTif, SpotOrderStatus, SpotOrderV7, SpotOrderV7Command};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderInput {
    pub lsn: u64,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderPipeChanges {
    Resting,
    PartiallyFilled {
        taker: UpdatedEntityEvents<SpotOrderV7>,
        makers: Vec<UpdatedEntityEvents<SpotOrderV7>>,
        trades: Vec<CreatedEntityEvent<SpotTradeV3>>,
    },
    Filled {
        taker: UpdatedEntityEvents<SpotOrderV7>,
        makers: Vec<UpdatedEntityEvents<SpotOrderV7>>,
        trades: Vec<CreatedEntityEvent<SpotTradeV3>>,
    },
    CanceledAfterPartialFill {
        taker: UpdatedEntityEvents<SpotOrderV7>,
        makers: Vec<UpdatedEntityEvents<SpotOrderV7>>,
        trades: Vec<CreatedEntityEvent<SpotTradeV3>>,
    },
    Rejected {
        taker: UpdatedEntityEvents<SpotOrderV7>,
    },
}

impl ReplayableEntityChanges for MatchSpotOrderPipeChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let (taker, makers, trades) = match self {
            Self::Resting => return Vec::new(),
            Self::PartiallyFilled {
                taker,
                makers,
                trades,
            }
            | Self::Filled {
                taker,
                makers,
                trades,
                ..
            }
            | Self::CanceledAfterPartialFill {
                taker,
                makers,
                trades,
                ..
            } => (taker, makers.as_slice(), trades.as_slice()),
            Self::Rejected { taker, .. } => return taker.replayable_events(),
        };
        let mut events = Vec::with_capacity(1 + makers.len() + trades.len());
        events.extend(taker.replayable_events());
        for maker in makers {
            events.extend(maker.replayable_events());
        }
        for trade in trades {
            events.extend(trade.replayable_events());
        }
        events
    }
}

#[derive(Clone, Copy)]
struct PlannedFill {
    maker_index: usize,
    size: Decimal,
    quote_notional: Decimal,
    taker_fee: Decimal,
    maker_fee: Decimal,
    trade_id: u64,
}

/// 从只读订单快照生成一次现货撮合的实体变化与可重放事件。
pub fn match_spot_order(
    taker_before: &SpotOrderV7,
    makers: &[SpotOrderV7],
    input: MatchSpotOrderInput,
) -> Result<MatchSpotOrderPipeChanges, EntityError> {
    match_spot_order_from(taker_before, makers, input, 0)
}

/// 从指定 `command_index` 起生成撮合订单和成交事件。
pub fn match_spot_order_from(
    taker_before: &SpotOrderV7,
    makers: &[SpotOrderV7],
    input: MatchSpotOrderInput,
    command_index_start: u32,
) -> Result<MatchSpotOrderPipeChanges, EntityError> {
    validate_orders(taker_before, makers)?;
    let taker_facts = taker_before.order_base();
    let crosses = |maker: &SpotOrderV7| match taker_facts.side {
        OrderSide::Buy => taker_facts.limit_px >= maker.order_base().limit_px,
        OrderSide::Sell => taker_facts.limit_px <= maker.order_base().limit_px,
    };

    if taker_facts.effective_tif() == OrderTif::Alo && makers.iter().any(crosses) {
        let taker = transition_order(
            taker_before,
            SpotOrderV7Command::RejectedAsTaker,
            input.lsn,
            command_index_start,
            input,
        )?;
        return Ok(MatchSpotOrderPipeChanges::Rejected { taker });
    }

    let mut remaining = taker_facts.size;
    let mut taker_fee_total = Decimal::ZERO;
    let mut quote_notional_total = Decimal::ZERO;
    let mut fills = Vec::new();
    for (maker_index, maker) in makers.iter().enumerate() {
        if remaining == Decimal::ZERO {
            break;
        }
        if !crosses(maker) {
            continue;
        }
        let size = remaining.min(maker.order_base().size);
        let quote_notional = maker
            .order_base()
            .limit_px
            .checked_mul(size)
            .ok_or_else(|| match_error("notional overflow"))?;
        let taker_fee = match_fee(
            quote_notional,
            input.taker_fee_bps,
            taker_facts.symbol.quote.scale(),
        )?;
        let maker_fee = match_fee(
            quote_notional,
            input.maker_fee_bps,
            taker_facts.symbol.quote.scale(),
        )?;
        taker_fee_total = taker_fee_total
            .checked_add(taker_fee)
            .ok_or_else(|| match_error("fee overflow"))?;
        quote_notional_total = quote_notional_total
            .checked_add(quote_notional)
            .ok_or_else(|| match_error("notional total overflow"))?;
        let offset = u64::try_from(fills.len()).map_err(|_| match_error("trade id overflow"))?;
        let trade_id = input
            .trade_id_start
            .checked_add(offset)
            .ok_or_else(|| match_error("trade id overflow"))?;
        remaining = remaining
            .checked_sub(size)
            .ok_or_else(|| match_error("size underflow"))?;
        fills.push(PlannedFill {
            maker_index,
            size,
            quote_notional,
            taker_fee,
            maker_fee,
            trade_id,
        });
    }

    let filled_size = taker_facts
        .size
        .checked_sub(remaining)
        .ok_or_else(|| match_error("size underflow"))?;
    if filled_size == Decimal::ZERO && taker_facts.effective_tif() != OrderTif::Ioc {
        return Ok(MatchSpotOrderPipeChanges::Resting);
    }

    let taker = transition_order(
        taker_before,
        SpotOrderV7Command::MatchedAsTaker {
            filled_size,
            quote_notional: quote_notional_total,
            quote_fee: taker_fee_total,
        },
        input.lsn,
        command_index_start,
        input,
    )?;

    let mut maker_changes = Vec::with_capacity(fills.len());
    for (fill_index, fill) in fills.iter().enumerate() {
        let maker = makers
            .get(fill.maker_index)
            .ok_or_else(|| match_error("planned maker index is missing"))?;
        let increment =
            u32::try_from(fill_index + 1).map_err(|_| match_error("command index overflow"))?;
        let command_index = command_index_start
            .checked_add(increment)
            .ok_or_else(|| match_error("command index overflow"))?;
        maker_changes.push(transition_order(
            maker,
            SpotOrderV7Command::MatchedAsMaker {
                filled_size: fill.size,
                quote_notional: fill.quote_notional,
                quote_fee: fill.maker_fee,
            },
            input.lsn,
            command_index,
            input,
        )?);
    }

    let trade_index_start = 1usize
        .checked_add(maker_changes.len())
        .ok_or_else(|| match_error("command index overflow"))?;
    let mut trades = Vec::with_capacity(fills.len());
    for (fill_index, fill) in fills.iter().enumerate() {
        let maker = makers
            .get(fill.maker_index)
            .ok_or_else(|| match_error("planned maker index is missing"))?;
        let increment = u32::try_from(
            trade_index_start
                .checked_add(fill_index)
                .ok_or_else(|| match_error("command index overflow"))?,
        )
        .map_err(|_| match_error("command index overflow"))?;
        let command_index = command_index_start
            .checked_add(increment)
            .ok_or_else(|| match_error("command index overflow"))?;
        trades.push(create_trade(
            &taker,
            maker,
            *fill,
            input.lsn,
            command_index,
            input,
        )?);
    }

    match taker.after().status() {
        SpotOrderStatus::Filled => Ok(MatchSpotOrderPipeChanges::Filled {
            taker,
            makers: maker_changes,
            trades,
        }),
        SpotOrderStatus::Canceled => Ok(MatchSpotOrderPipeChanges::CanceledAfterPartialFill {
            taker,
            makers: maker_changes,
            trades,
        }),
        SpotOrderStatus::Rejected => Ok(MatchSpotOrderPipeChanges::Rejected { taker }),
        SpotOrderStatus::PartiallyFilled => Ok(MatchSpotOrderPipeChanges::PartiallyFilled {
            taker,
            makers: maker_changes,
            trades,
        }),
        _ => Err(match_error("unexpected taker state after matching")),
    }
}

fn validate_orders(taker: &SpotOrderV7, makers: &[SpotOrderV7]) -> Result<(), EntityError> {
    taker.validate_matchable_as_taker()?;
    let facts = taker.order_base();
    let mut ids = HashSet::with_capacity(makers.len() + 1);
    ids.insert(facts.order_id);
    for maker in makers {
        maker.validate_matchable_as_maker()?;
        let maker_facts = maker.order_base();
        if !ids.insert(maker_facts.order_id) {
            return Err(match_error("order ids must be unique"));
        }
        if maker_facts.symbol != facts.symbol || maker_facts.side == facts.side {
            return Err(match_error("maker market or side mismatch"));
        }
    }
    Ok(())
}

fn transition_order(
    before: &SpotOrderV7,
    command: SpotOrderV7Command,
    lsn: u64,
    command_index: u32,
    input: MatchSpotOrderInput,
) -> Result<UpdatedEntityEvents<SpotOrderV7>, EntityError> {
    let metadata = CommandMetadata::new_at(
        lsn,
        command_index,
        input.logical_ts,
        input.topic,
        input.partition,
        before.party_id(),
    );
    let (after, mut event) = FsmV2::transition(before, SequencedCommand::new(metadata, command))?;
    event.t = input.topic;
    event.p = input.partition;
    UpdatedEntityEvents::try_new(after, before.lsn(), before.command_index(), vec![event])
        .map_err(|error| match_error(&format!("invalid order event chain: {error:?}")))
}

fn create_trade(
    taker: &UpdatedEntityEvents<SpotOrderV7>,
    maker: &SpotOrderV7,
    fill: PlannedFill,
    lsn: u64,
    command_index: u32,
    input: MatchSpotOrderInput,
) -> Result<CreatedEntityEvent<SpotTradeV3>, EntityError> {
    let taker_facts = taker.after().order_base();
    let trade = SpotTradeV3 {
        comon_trade: TradeBase {
            trade_id: fill.trade_id,
            symbol: taker_facts.symbol,
            taker_user: taker_facts.owner,
            maker_user: maker.order_base().owner,
            taker_order_id: taker_facts.order_id,
            maker_order_id: maker.order_base().order_id,
            taker_side: taker_facts.side,
            price: maker.order_base().limit_px,
            size: fill.size,
            taker_fee: fill.taker_fee,
            maker_fee: fill.maker_fee,
            created_at: input.logical_ts,
            updated_at: input.logical_ts,
            version: input.lsn,
        },
        settlement_status: SpotTradeSettlementStatus::Unsettled,
        lsn,
        command_index,
    };
    let mut event = trade.track_create_event_at(lsn, command_index, input.logical_ts)?;
    event.t = input.topic;
    event.p = input.partition;
    event.domain_event_type = 1;
    CreatedEntityEvent::try_new(trade, event)
        .map_err(|error| match_error(&format!("invalid trade event: {error:?}")))
}

fn match_fee(notional: Decimal, bps: u64, scale: u32) -> Result<Decimal, EntityError> {
    notional
        .checked_mul(Decimal::from(bps))
        .and_then(|value| value.checked_div(Decimal::from(10_000u64)))
        .map(|value| {
            value.round_dp_with_strategy(scale, rust_decimal::RoundingStrategy::ToPositiveInfinity)
        })
        .ok_or_else(|| match_error("fee overflow"))
}

fn match_error(message: &str) -> EntityError {
    EntityError::Custom(message.to_owned())
}
