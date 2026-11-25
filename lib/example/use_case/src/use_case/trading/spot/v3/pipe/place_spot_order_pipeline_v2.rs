use std::collections::{HashMap, HashSet};
use std::fmt;

use common_entity::{Entity, EntityError, ReplayableDomainEvent, ReplayableEntityChanges};
use example_core_entity::{
    BalanceV2, PlaceSpotOrderPipelineV2Cmd, OrderGroupRelation, OrderSide, OrderType, SpotOrderV7,
};

use super::freeze_spot_order::{FreezeSpotOrderPipeChanges, freeze_spot_order_from};
use super::match_spot_order::{
    MatchSpotOrderInput, MatchSpotOrderPipeChanges, match_spot_order_from,
};
use super::place_spot_order::{PlaceSpotOrderPipeChanges, place_spot_order};
use super::settle_matched_spot_trades::{
    SettleMatchedSpotTradesError, SettleMatchedSpotTradesInput, SettleTradesPipeChanges,
    SpotOrderSettlementBalances, settle_matched_spot_trades_from,
};
use super::unfreeze_matched_spot_orders::{
    SpotOrderHeldBalance, UnfreezeMatchedSpotOrdersError, UnfreezeMatchedSpotOrdersPipeChanges,
    unfreeze_matched_spot_orders,
};

/// 独立现货下单 pipe_line 的分支化业务事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderPipelineV2Changes {
    Resting {
        created_order: PlaceSpotOrderPipeChanges,
        frozen: FreezeSpotOrderPipeChanges,
    },
    Rejected {
        created_order: PlaceSpotOrderPipeChanges,
        frozen: FreezeSpotOrderPipeChanges,
        matched: MatchSpotOrderPipeChanges,
        unfrozen: UnfreezeMatchedSpotOrdersPipeChanges,
    },
    PartiallyFilled {
        created_order: PlaceSpotOrderPipeChanges,
        frozen: FreezeSpotOrderPipeChanges,
        matched: MatchSpotOrderPipeChanges,
        settled: SettleTradesPipeChanges,
        unfrozen: UnfreezeMatchedSpotOrdersPipeChanges,
    },
    Filled {
        created_order: PlaceSpotOrderPipeChanges,
        frozen: FreezeSpotOrderPipeChanges,
        matched: MatchSpotOrderPipeChanges,
        settled: SettleTradesPipeChanges,
        unfrozen: UnfreezeMatchedSpotOrdersPipeChanges,
    },
    CanceledAfterPartialFill {
        created_order: PlaceSpotOrderPipeChanges,
        frozen: FreezeSpotOrderPipeChanges,
        matched: MatchSpotOrderPipeChanges,
        settled: SettleTradesPipeChanges,
        unfrozen: UnfreezeMatchedSpotOrdersPipeChanges,
    },
}

impl ReplayableEntityChanges for PlaceSpotOrderPipelineV2Changes {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let (created, frozen, matched, settled, unfrozen) = match self {
            Self::Resting {
                created_order,
                frozen,
            } => (created_order, frozen, None, None, None),
            Self::Rejected {
                created_order,
                frozen,
                matched,
                unfrozen,
            } => (created_order, frozen, Some(matched), None, Some(unfrozen)),
            Self::PartiallyFilled {
                created_order,
                frozen,
                matched,
                settled,
                unfrozen,
            } => (
                created_order,
                frozen,
                Some(matched),
                Some(settled),
                Some(unfrozen),
            ),
            Self::Filled {
                created_order,
                frozen,
                matched,
                settled,
                unfrozen,
            }
            | Self::CanceledAfterPartialFill {
                created_order,
                frozen,
                matched,
                settled,
                unfrozen,
            } => (
                created_order,
                frozen,
                Some(matched),
                Some(settled),
                Some(unfrozen),
            ),
        };
        let mut events = Vec::new();
        events.extend(created.replayable_events());
        events.extend(frozen.replayable_events());
        if let Some(changes) = matched {
            events.extend(changes.replayable_events());
        }
        if let Some(changes) = settled {
            events.extend(changes.replayable_events());
        }
        if let Some(changes) = unfrozen {
            events.extend(changes.replayable_events());
        }
        events
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderPipelineV2Error {
    InvalidInput(String),
    Place(EntityError),
    Freeze(EntityError),
    Match(EntityError),
    Settle(SettleMatchedSpotTradesError),
    Unfreeze(UnfreezeMatchedSpotOrdersError),
}

impl fmt::Display for PlaceSpotOrderPipelineV2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "invalid pipe_line input: {message}"),
            Self::Place(error) => write!(f, "place order failed: {error}"),
            Self::Freeze(error) => write!(f, "freeze order failed: {error}"),
            Self::Match(error) => write!(f, "match order failed: {error}"),
            Self::Settle(error) => write!(f, "settle trades failed: {}", error.0),
            Self::Unfreeze(error) => write!(f, "unfreeze orders failed: {}", error.0),
        }
    }
}

impl std::error::Error for PlaceSpotOrderPipelineV2Error {}

/// 纯计算地执行创建、冻结、撮合、结算与终态解冻。
#[allow(clippy::too_many_arguments)]
pub fn place_spot_order_pipeline_v2(
    command: PlaceSpotOrderPipelineV2Cmd,
    maker_orders: &[SpotOrderV7],
    balances_by_order_id: &HashMap<u64, SpotOrderSettlementBalances>,
    trade_id_start: u64,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
    lsn: u64,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error> {
    validate_inputs(
        &command,
        maker_orders,
        balances_by_order_id,
        trade_id_start,
        lsn,
    )?;
    let created_order = place_spot_order(command.clone(), lsn, 0, logical_ts, topic, partition)
        .map_err(PlaceSpotOrderPipelineV2Error::Place)?;
    let pair = balances_by_order_id
        .get(&command.order_id)
        .ok_or_else(|| invalid(format!("missing balances for order {}", command.order_id)))?;
    let held = held_balance(created_order.after(), pair);
    let frozen = freeze_spot_order_from(
        created_order.after(),
        held,
        lsn,
        1,
        logical_ts,
        topic,
        partition,
    )
    .map_err(PlaceSpotOrderPipelineV2Error::Freeze)?;

    let mut working_balances = balances_by_order_id.clone();
    replace_balance_snapshot(&mut working_balances, frozen.balance.after())?;
    let match_start = 3;
    let matched = match_spot_order_from(
        frozen.order.after(),
        maker_orders,
        MatchSpotOrderInput {
            lsn,
            logical_ts,
            topic,
            partition,
            trade_id_start,
            maker_fee_bps,
            taker_fee_bps,
        },
        match_start,
    )
    .map_err(PlaceSpotOrderPipelineV2Error::Match)?;
    let settle_start = next_command_index(match_start, &matched)?;
    let settled = settle_matched_spot_trades_from(
        &matched,
        &working_balances,
        SettleMatchedSpotTradesInput {
            lsn,
            logical_ts,
            topic,
            partition,
        },
        settle_start,
    )
    .map_err(PlaceSpotOrderPipelineV2Error::Settle)?;
    for balance in &settled.updated_balances {
        replace_balance_snapshot(&mut working_balances, balance.after())?;
    }
    let unfreeze_previous = last_position(&settled)
        .or_else(|| last_position(&matched))
        .unwrap_or((lsn, settle_start.saturating_sub(1)));
    let held_by_order_id =
        build_held_balances(frozen.order.after(), maker_orders, &working_balances)?;
    let unfrozen = unfreeze_matched_spot_orders(
        frozen.order.after(),
        maker_orders,
        &matched,
        &held_by_order_id,
        unfreeze_previous.0,
        unfreeze_previous.1,
        logical_ts,
        topic,
        partition,
    )
    .map_err(PlaceSpotOrderPipelineV2Error::Unfreeze)?;

    Ok(match &matched {
        MatchSpotOrderPipeChanges::Resting => PlaceSpotOrderPipelineV2Changes::Resting {
            created_order,
            frozen,
        },
        MatchSpotOrderPipeChanges::Rejected { .. } => PlaceSpotOrderPipelineV2Changes::Rejected {
            created_order,
            frozen,
            matched,
            unfrozen,
        },
        MatchSpotOrderPipeChanges::PartiallyFilled { .. } => {
            PlaceSpotOrderPipelineV2Changes::PartiallyFilled {
                created_order,
                frozen,
                matched,
                settled,
                unfrozen,
            }
        }
        MatchSpotOrderPipeChanges::Filled { .. } => PlaceSpotOrderPipelineV2Changes::Filled {
            created_order,
            frozen,
            matched,
            settled,
            unfrozen,
        },
        MatchSpotOrderPipeChanges::CanceledAfterPartialFill { .. } => {
            PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill {
                created_order,
                frozen,
                matched,
                settled,
                unfrozen,
            }
        }
    })
}

fn validate_inputs(
    command: &PlaceSpotOrderPipelineV2Cmd,
    makers: &[SpotOrderV7],
    balances: &HashMap<u64, SpotOrderSettlementBalances>,
    trade_id_start: u64,
    lsn: u64,
) -> Result<(), PlaceSpotOrderPipelineV2Error> {
    validate_place_spot_order_pipeline_v2_command(command)?;
    validate_place_spot_order_pipeline_v2_state(command, makers, balances, trade_id_start, lsn)
}

pub(crate) fn validate_place_spot_order_pipeline_v2_command(
    command: &PlaceSpotOrderPipelineV2Cmd,
) -> Result<(), PlaceSpotOrderPipelineV2Error> {
    if !matches!(command.order_type, OrderType::Limit { .. }) {
        return Err(invalid("trigger orders are not supported"));
    }
    if command.relation != OrderGroupRelation::Standalone {
        return Err(invalid("TPSL and grouped orders are not supported"));
    }
    if command.order_id == 0 {
        return Err(invalid("order id must be greater than zero"));
    }
    if command.party_id == 0 {
        return Err(invalid("party id must be greater than zero"));
    }
    if command.price <= rust_decimal::Decimal::ZERO {
        return Err(invalid("price must be greater than zero"));
    }
    if command.size <= rust_decimal::Decimal::ZERO {
        return Err(invalid("size must be greater than zero"));
    }
    Ok(())
}

pub(crate) fn validate_place_spot_order_pipeline_v2_state(
    command: &PlaceSpotOrderPipelineV2Cmd,
    makers: &[SpotOrderV7],
    balances: &HashMap<u64, SpotOrderSettlementBalances>,
    trade_id_start: u64,
    lsn: u64,
) -> Result<(), PlaceSpotOrderPipelineV2Error> {
    let first_position = (lsn, 0);
    let mut order_ids = HashSet::with_capacity(makers.len() + 1);
    order_ids.insert(command.order_id);
    let taker_pair = balances
        .get(&command.order_id)
        .ok_or_else(|| invalid(format!("missing balances for order {}", command.order_id)))?;
    validate_balance_binding(
        command.order_id,
        command.party_id,
        command.symbol,
        taker_pair,
    )?;
    for maker in makers {
        let id = maker.order_base().order_id;
        if !order_ids.insert(id) {
            return Err(invalid(format!("duplicate order {id}")));
        }
        if (maker.lsn(), maker.command_index()) >= first_position {
            return Err(invalid(format!("stale pipe_line position for order {id}")));
        }
        let pair = balances
            .get(&id)
            .ok_or_else(|| invalid(format!("missing balances for order {id}")))?;
        validate_balance_binding(
            id,
            maker.order_base().owner.0,
            maker.order_base().symbol,
            pair,
        )?;
    }
    if !makers.is_empty() {
        let offset =
            u64::try_from(makers.len() - 1).map_err(|_| invalid("trade id range overflow"))?;
        trade_id_start
            .checked_add(offset)
            .ok_or_else(|| invalid("trade id range overflow"))?;
    }
    let mut snapshots = HashMap::<u64, &BalanceV2>::new();
    for (order_id, pair) in balances {
        for balance in [&pair.base, &pair.quote] {
            if (balance.lsn(), balance.command_index()) >= first_position {
                return Err(invalid(format!(
                    "stale pipe_line position for balance {}",
                    balance.entity_id()
                )));
            }
            if let Some(existing) = snapshots.insert(balance.entity_id(), balance) {
                if existing != balance {
                    return Err(invalid(format!(
                        "conflicting snapshots for balance {} referenced by order {order_id}",
                        balance.entity_id()
                    )));
                }
            }
        }
    }
    Ok(())
}

fn validate_balance_binding(
    order_id: u64,
    party_id: u64,
    symbol: example_core_entity::account::symbol::Symbol,
    pair: &SpotOrderSettlementBalances,
) -> Result<(), PlaceSpotOrderPipelineV2Error> {
    if pair.base.party_id() != party_id
        || pair.quote.party_id() != party_id
        || pair.base.account_id() != party_id
        || pair.quote.account_id() != party_id
        || pair.base.coin_idx() != u64::from(symbol.base.index())
        || pair.quote.coin_idx() != u64::from(symbol.quote.index())
        || pair.base.entity_id() == pair.quote.entity_id()
    {
        return Err(invalid(format!(
            "balance owner, account, or coin mismatch for order {order_id}"
        )));
    }
    Ok(())
}

fn held_balance<'a>(order: &SpotOrderV7, pair: &'a SpotOrderSettlementBalances) -> &'a BalanceV2 {
    match order.order_base().side {
        OrderSide::Buy => &pair.quote,
        OrderSide::Sell => &pair.base,
    }
}

fn replace_balance_snapshot(
    balances: &mut HashMap<u64, SpotOrderSettlementBalances>,
    after: &BalanceV2,
) -> Result<(), PlaceSpotOrderPipelineV2Error> {
    let target_id = after.entity_id();
    let mut replaced = false;
    for pair in balances.values_mut() {
        for balance in [&mut pair.base, &mut pair.quote] {
            if balance.entity_id() == target_id {
                *balance = after.clone();
                replaced = true;
            }
        }
    }
    if !replaced {
        return Err(invalid(format!("unknown updated balance {target_id}")));
    }
    Ok(())
}

fn build_held_balances(
    taker: &SpotOrderV7,
    makers: &[SpotOrderV7],
    balances: &HashMap<u64, SpotOrderSettlementBalances>,
) -> Result<HashMap<u64, SpotOrderHeldBalance>, PlaceSpotOrderPipelineV2Error> {
    let mut output = HashMap::with_capacity(makers.len() + 1);
    for order in std::iter::once(taker).chain(makers) {
        let id = order.order_base().order_id;
        let pair = balances
            .get(&id)
            .ok_or_else(|| invalid(format!("missing balances for order {id}")))?;
        let held = match order.order_base().side {
            OrderSide::Buy => SpotOrderHeldBalance::Quote(pair.quote.clone()),
            OrderSide::Sell => SpotOrderHeldBalance::Base(pair.base.clone()),
        };
        output.insert(id, held);
    }
    Ok(output)
}

fn next_command_index(
    start: u32,
    changes: &impl ReplayableEntityChanges,
) -> Result<u32, PlaceSpotOrderPipelineV2Error> {
    let count = u32::try_from(changes.replayable_events().len())
        .map_err(|_| invalid("command index overflow"))?;
    start
        .checked_add(count)
        .ok_or_else(|| invalid("command index overflow"))
}

fn last_position(changes: &impl ReplayableEntityChanges) -> Option<(u64, u32)> {
    changes
        .replayable_events()
        .into_iter()
        .map(|event| (event.lsn, event.command_index))
        .max()
}

fn invalid(message: impl Into<String>) -> PlaceSpotOrderPipelineV2Error {
    PlaceSpotOrderPipelineV2Error::InvalidInput(message.into())
}
