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
    MatchSpotOrderChanges, MatchSpotOrderInput, match_spot_order_from,
};
use crate::use_case::trading::spot::v3::domain_service::place_spot_order::place_spot_order_at;
use crate::use_case::trading::spot::v3::domain_service::settle_matched_spot_trades::{
    SettleMatchedSpotTradesChanges, SettleMatchedSpotTradesError, SettleMatchedSpotTradesInput,
    SpotOrderSettlementBalances, settle_matched_spot_trades_from,
};
use crate::use_case::trading::spot::v3::domain_service::unfreeze_matched_spot_orders::{
    SpotOrderHeldBalance, UnfreezeMatchedSpotOrdersError, unfreeze_matched_spot_orders_at,
};

/// 现货下单 pipeline 的唯一业务变化真相。
///
/// 各字段按创建、冻结、撮合、结算、终态解冻排列；更新事实由
/// `UpdatedEntityEvents` 中的 before position、after entity 与事件链共同表达。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderPipelineChanges {
    pub created_order: CreatedEntityEvent<SpotOrderV7>,
    pub frozen_order: UpdatedEntityEvents<SpotOrderV7>,
    pub frozen_balance: UpdatedEntityEvents<BalanceV2>,
    pub matched: MatchSpotOrderChanges,
    pub settled: SettleMatchedSpotTradesChanges,
    pub unfrozen_balances: Vec<UpdatedEntityEvents<BalanceV2>>,
}

impl ReplayableEntityChanges for PlaceSpotOrderPipelineChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let mut events = Vec::new();
        events.extend(self.created_order.replayable_events());
        events.extend(self.frozen_order.replayable_events());
        events.extend(self.frozen_balance.replayable_events());
        events.extend(self.matched.replayable_events());
        events.extend(self.settled.replayable_events());
        for changes in &self.unfrozen_balances {
            events.extend(changes.replayable_events());
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

/// 纯计算地执行普通独立现货订单的创建、冻结、撮合、结算与终态解冻。
///
/// 调用方必须提供完整且权威的 maker 与订单余额快照。函数不加载或持久化状态，任一阶段
/// 失败时不返回部分 changes，也不会修改输入快照。
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
    validate_inputs(
        &command,
        maker_orders,
        balances_by_order_id,
        trade_id_start,
        lsn,
    )?;

    let created_order = place_spot_order_at(
        command.clone(),
        EventPosition::new(lsn, 0),
        logical_ts,
        topic,
        partition,
    )
    .map_err(PlaceSpotOrderPipelineError::Place)?;

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
    .map_err(PlaceSpotOrderPipelineError::Settle)?;
    for balance in &settled.updated_balances {
        replace_balance_snapshot(&mut working_balances, balance.after())?;
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

    Ok(PlaceSpotOrderPipelineChanges {
        created_order,
        frozen_order,
        frozen_balance,
        matched,
        settled,
        unfrozen_balances,
    })
}

fn validate_inputs(
    command: &CreateSpotOrderV7Command,
    makers: &[SpotOrderV7],
    balances: &HashMap<u64, SpotOrderSettlementBalances>,
    trade_id_start: u64,
    lsn: u64,
) -> Result<(), PlaceSpotOrderPipelineError> {
    if !matches!(command.order_type, OrderType::Limit { .. }) {
        return Err(invalid("trigger orders are not supported"));
    }
    if command.relation != OrderGroupRelation::Standalone {
        return Err(invalid("TPSL and grouped orders are not supported"));
    }
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
