use std::collections::{HashMap, HashSet};
use std::fmt;

use common_entity::{
    CreatedEntityEvent, Entity, EntityError, EventPosition, ReplayableDomainEvent,
    ReplayableEntityChanges, UpdatedEntityEvents,
};
use example_core_entity::{
    BalanceV2, CreateSpotOrderV7Command, OrderGroupRelation, OrderSide, OrderType, SpotOrderV7,
};

use crate::use_case::trading::spot::v3::domain_service::freeze_spot_order::freeze_spot_order_from;
use crate::use_case::trading::spot::v3::domain_service::match_spot_order::{
    MatchSpotOrderPipeChanges, MatchSpotOrderInput, match_spot_order_from,
};
use crate::use_case::trading::spot::v3::domain_service::place_spot_order::place_spot_order_at;
use crate::use_case::trading::spot::v3::domain_service::settle_matched_spot_trades::{
    SettleTradesPipeChanges, SettleMatchedSpotTradesError, SettleMatchedSpotTradesInput,
    SpotOrderSettlementBalances, settle_matched_spot_trades_from,
};
use crate::use_case::trading::spot::v3::domain_service::unfreeze_matched_spot_orders::{
    SpotOrderHeldBalance, UnfreezeMatchedSpotOrdersError, unfreeze_matched_spot_orders_at,
};

/// 现货下单 pipeline 按实际执行路径划分的唯一业务变化真相。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderPipelineChanges {
    PendingTrigger {
        created_order: CreatedEntityEvent<SpotOrderV7>,
    },
    Resting {
        created_order: CreatedEntityEvent<SpotOrderV7>,
        frozen_order: UpdatedEntityEvents<SpotOrderV7>,
        frozen_balance: UpdatedEntityEvents<BalanceV2>,
        matched: MatchSpotOrderPipeChanges,
    },
    Rejected {
        created_order: CreatedEntityEvent<SpotOrderV7>,
        frozen_order: UpdatedEntityEvents<SpotOrderV7>,
        frozen_balance: UpdatedEntityEvents<BalanceV2>,
        matched: MatchSpotOrderPipeChanges,
        unfrozen_balances: Vec<UpdatedEntityEvents<BalanceV2>>,
    },
    PartiallyFilled {
        created_order: CreatedEntityEvent<SpotOrderV7>,
        frozen_order: UpdatedEntityEvents<SpotOrderV7>,
        frozen_balance: UpdatedEntityEvents<BalanceV2>,
        matched: MatchSpotOrderPipeChanges,
        settled: SettleTradesPipeChanges,
    },
    Filled {
        created_order: CreatedEntityEvent<SpotOrderV7>,
        frozen_order: UpdatedEntityEvents<SpotOrderV7>,
        frozen_balance: UpdatedEntityEvents<BalanceV2>,
        matched: MatchSpotOrderPipeChanges,
        settled: SettleTradesPipeChanges,
        unfrozen_balances: Vec<UpdatedEntityEvents<BalanceV2>>,
    },
    CanceledAfterPartialFill {
        created_order: CreatedEntityEvent<SpotOrderV7>,
        frozen_order: UpdatedEntityEvents<SpotOrderV7>,
        frozen_balance: UpdatedEntityEvents<BalanceV2>,
        matched: MatchSpotOrderPipeChanges,
        settled: SettleTradesPipeChanges,
        unfrozen_balances: Vec<UpdatedEntityEvents<BalanceV2>>,
    },
}

impl ReplayableEntityChanges for PlaceSpotOrderPipelineChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let (created, frozen, matched, settled, unfrozen) = match self {
            Self::PendingTrigger { created_order } => (created_order, None, None, None, None),
            Self::Resting {
                created_order,
                frozen_order,
                frozen_balance,
                matched,
            } => (
                created_order,
                Some((frozen_order, frozen_balance)),
                Some(matched),
                None,
                None,
            ),
            Self::Rejected {
                created_order,
                frozen_order,
                frozen_balance,
                matched,
                unfrozen_balances,
            } => (
                created_order,
                Some((frozen_order, frozen_balance)),
                Some(matched),
                None,
                Some(unfrozen_balances),
            ),
            Self::PartiallyFilled {
                created_order,
                frozen_order,
                frozen_balance,
                matched,
                settled,
            } => (
                created_order,
                Some((frozen_order, frozen_balance)),
                Some(matched),
                Some(settled),
                None,
            ),
            Self::Filled {
                created_order,
                frozen_order,
                frozen_balance,
                matched,
                settled,
                unfrozen_balances,
            }
            | Self::CanceledAfterPartialFill {
                created_order,
                frozen_order,
                frozen_balance,
                matched,
                settled,
                unfrozen_balances,
            } => (
                created_order,
                Some((frozen_order, frozen_balance)),
                Some(matched),
                Some(settled),
                Some(unfrozen_balances),
            ),
        };
        let mut events = Vec::new();
        events.extend(created.replayable_events());
        if let Some((order, balance)) = frozen {
            events.extend(order.replayable_events());
            events.extend(balance.replayable_events());
        }
        if let Some(changes) = matched {
            events.extend(changes.replayable_events());
        }
        if let Some(changes) = settled {
            events.extend(changes.replayable_events());
        }
        if let Some(changes) = unfrozen {
            for change in changes {
                events.extend(change.replayable_events());
            }
        }
        events
    }
}

/// 下单 pipeline 的业务拒绝；不包含加载、持久化或发布故障。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderPipelineError {
    InvalidInput(String),
    Place(EntityError),
    Freeze(EntityError),
    Match(EntityError),
    Settle(SettleMatchedSpotTradesError),
    Unfreeze(UnfreezeMatchedSpotOrdersError),
}

impl fmt::Display for PlaceSpotOrderPipelineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(formatter, "invalid pipeline input: {message}"),
            Self::Place(error) => write!(formatter, "place order failed: {error}"),
            Self::Freeze(error) => write!(formatter, "freeze order failed: {error}"),
            Self::Match(error) => write!(formatter, "match order failed: {error}"),
            Self::Settle(error) => write!(formatter, "settle trades failed: {}", error.0),
            Self::Unfreeze(error) => write!(formatter, "unfreeze orders failed: {}", error.0),
        }
    }
}

impl std::error::Error for PlaceSpotOrderPipelineError {}

/// 纯计算地执行独立现货订单实际需要的 pipeline 阶段。
///
/// 普通限价单要求调用方提供完整且权威的 maker 与订单余额快照；条件单只执行 place，
/// 不读取这些快照。函数不加载或持久化状态，任一阶段失败时不返回部分 changes，
/// 也不会修改输入快照。
#[allow(clippy::too_many_arguments)]
pub fn place_spot_order_pipeline(
    command: CreateSpotOrderV7Command,
    maker_orders: &[SpotOrderV7],
    balances_by_order_id: &HashMap<u64, SpotOrderSettlementBalances>,
    trade_id_start: u64,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
    lsn: u64,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<PlaceSpotOrderPipelineChanges, PlaceSpotOrderPipelineError> {
    validate_common_order(&command)?;

    let created_order = place_spot_order_at(
        command.clone(),
        EventPosition::new(lsn, 0),
        logical_ts,
        topic,
        partition,
    )
    .map_err(PlaceSpotOrderPipelineError::Place)?;

    if !matches!(command.order_type, OrderType::Limit { .. }) {
        return Ok(PlaceSpotOrderPipelineChanges::PendingTrigger { created_order });
    }

    validate_limit_execution(
        &command,
        maker_orders,
        balances_by_order_id,
        trade_id_start,
        lsn,
    )?;

    let taker_balances = balances_by_order_id.get(&command.order_id).ok_or_else(|| {
        invalid(format!(
            "missing balances for taker order {}",
            command.order_id
        ))
    })?;
    let held_before = held_balance(created_order.after(), taker_balances).clone();
    let (frozen_order, frozen_balance) = freeze_spot_order_from(
        created_order.after(),
        &held_before,
        lsn,
        1,
        logical_ts,
        topic,
        partition,
    )
    .map_err(PlaceSpotOrderPipelineError::Freeze)?;

    let mut working_balances = balances_by_order_id.clone();
    replace_balance_snapshot(&mut working_balances, frozen_balance.after())?;

    let match_start = 3;
    let matched = match_spot_order_from(
        frozen_order.after(),
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
    .map_err(PlaceSpotOrderPipelineError::Match)?;
    if matches!(matched, MatchSpotOrderPipeChanges::Resting) {
        return Ok(PlaceSpotOrderPipelineChanges::Resting {
            created_order,
            frozen_order,
            frozen_balance,
            matched,
        });
    }

    let settle_start = next_command_index(match_start, &matched)?;

    if matches!(matched, MatchSpotOrderPipeChanges::Rejected { .. }) {
        let unfreeze_previous = last_position(&matched)
            .unwrap_or(EventPosition::new(lsn, settle_start.saturating_sub(1)));
        let held_by_order_id =
            build_held_balances(frozen_order.after(), maker_orders, &working_balances)?;
        let unfrozen_balances = unfreeze_matched_spot_orders_at(
            frozen_order.after(),
            maker_orders,
            &matched,
            &held_by_order_id,
            unfreeze_previous,
            logical_ts,
            topic,
            partition,
        )
        .map_err(PlaceSpotOrderPipelineError::Unfreeze)?;
        return Ok(PlaceSpotOrderPipelineChanges::Rejected {
            created_order,
            frozen_order,
            frozen_balance,
            matched,
            unfrozen_balances,
        });
    }

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
    .map_err(PlaceSpotOrderPipelineError::Settle)?;
    for balance in &settled.updated_balances {
        replace_balance_snapshot(&mut working_balances, balance.after())?;
    }

    if matches!(matched, MatchSpotOrderPipeChanges::PartiallyFilled { .. }) {
        return Ok(PlaceSpotOrderPipelineChanges::PartiallyFilled {
            created_order,
            frozen_order,
            frozen_balance,
            matched,
            settled,
        });
    }

    let unfreeze_previous = last_position(&settled)
        .or_else(|| last_position(&matched))
        .unwrap_or(EventPosition::new(lsn, settle_start.saturating_sub(1)));
    let held_by_order_id =
        build_held_balances(frozen_order.after(), maker_orders, &working_balances)?;
    let unfrozen_balances = unfreeze_matched_spot_orders_at(
        frozen_order.after(),
        maker_orders,
        &matched,
        &held_by_order_id,
        unfreeze_previous,
        logical_ts,
        topic,
        partition,
    )
    .map_err(PlaceSpotOrderPipelineError::Unfreeze)?;

    let changes = match matched {
        matched @ MatchSpotOrderPipeChanges::Filled { .. } => PlaceSpotOrderPipelineChanges::Filled {
            created_order,
            frozen_order,
            frozen_balance,
            matched,
            settled,
            unfrozen_balances,
        },
        matched @ MatchSpotOrderPipeChanges::CanceledAfterPartialFill { .. } => {
            PlaceSpotOrderPipelineChanges::CanceledAfterPartialFill {
                created_order,
                frozen_order,
                frozen_balance,
                matched,
                settled,
                unfrozen_balances,
            }
        }
        _ => return Err(invalid("unexpected match result after terminal processing")),
    };
    Ok(changes)
}

fn validate_common_order(
    command: &CreateSpotOrderV7Command,
) -> Result<(), PlaceSpotOrderPipelineError> {
    if command.relation != OrderGroupRelation::Standalone {
        return Err(invalid("TPSL and grouped orders are not supported"));
    }
    Ok(())
}

fn validate_limit_execution(
    command: &CreateSpotOrderV7Command,
    makers: &[SpotOrderV7],
    balances: &HashMap<u64, SpotOrderSettlementBalances>,
    trade_id_start: u64,
    lsn: u64,
) -> Result<(), PlaceSpotOrderPipelineError> {
    let first_position = EventPosition::new(lsn, 0);
    let mut order_ids = HashSet::with_capacity(makers.len() + 1);
    order_ids.insert(command.order_id);
    let taker_balances = balances
        .get(&command.order_id)
        .ok_or_else(|| invalid(format!("missing balances for order {}", command.order_id)))?;
    validate_balance_binding(
        command.order_id,
        command.party_id,
        command.symbol,
        taker_balances,
    )?;
    for maker in makers {
        let id = maker.order_base().order_id;
        if !order_ids.insert(id) {
            return Err(invalid(format!("duplicate order {id}")));
        }
        if Entity::event_position(maker) >= first_position {
            return Err(invalid(format!("stale pipeline position for order {id}")));
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
            if balance.position() >= first_position {
                return Err(invalid(format!(
                    "stale pipeline position for balance {}",
                    Entity::entity_id(balance)
                )));
            }
            if let Some(existing) = snapshots.insert(Entity::entity_id(balance), balance) {
                if existing != balance {
                    return Err(invalid(format!(
                        "conflicting snapshots for balance {} referenced by order {order_id}",
                        Entity::entity_id(balance)
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
) -> Result<(), PlaceSpotOrderPipelineError> {
    if pair.base.party_id() != party_id
        || pair.quote.party_id() != party_id
        || pair.base.account_id() != party_id
        || pair.quote.account_id() != party_id
        || pair.base.coin_idx() != u64::from(symbol.base.index())
        || pair.quote.coin_idx() != u64::from(symbol.quote.index())
        || Entity::entity_id(&pair.base) == Entity::entity_id(&pair.quote)
    {
        return Err(invalid(format!(
            "balance owner, account, or coin mismatch for order {order_id}"
        )));
    }
    Ok(())
}

fn held_balance<'a>(
    order: &SpotOrderV7,
    balances: &'a SpotOrderSettlementBalances,
) -> &'a BalanceV2 {
    match order.order_base().side {
        OrderSide::Buy => &balances.quote,
        OrderSide::Sell => &balances.base,
    }
}

fn replace_balance_snapshot(
    balances: &mut HashMap<u64, SpotOrderSettlementBalances>,
    after: &BalanceV2,
) -> Result<(), PlaceSpotOrderPipelineError> {
    let target_id = Entity::entity_id(after);
    let mut replaced = false;
    for pair in balances.values_mut() {
        for balance in [&mut pair.base, &mut pair.quote] {
            if Entity::entity_id(balance) == target_id {
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
) -> Result<HashMap<u64, SpotOrderHeldBalance>, PlaceSpotOrderPipelineError> {
    let mut output = HashMap::new();
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
) -> Result<u32, PlaceSpotOrderPipelineError> {
    let count = u32::try_from(changes.replayable_events().len())
        .map_err(|_| invalid("command index overflow"))?;
    start
        .checked_add(count)
        .ok_or_else(|| invalid("command index overflow"))
}

fn last_position(changes: &impl ReplayableEntityChanges) -> Option<EventPosition> {
    changes
        .replayable_events()
        .into_iter()
        .map(|event| event.position)
        .max()
}

fn invalid(message: impl Into<String>) -> PlaceSpotOrderPipelineError {
    PlaceSpotOrderPipelineError::InvalidInput(message.into())
}
