use std::collections::{BTreeMap, HashMap};

use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
use common_entity::{Entity, EventPosition, UpdatedEntityEvents};
use example_core_entity::{BalanceV2, BalanceV2Command, OrderSide, SpotOrderStatus, SpotOrderV7};
use rust_decimal::Decimal;

use super::match_spot_order::MatchSpotOrderPipeChanges;

/// 订单冻结资金所在的余额角色。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderHeldBalance {
    Base(BalanceV2),
    Quote(BalanceV2),
}

/// 撮合后释放订单冻结资金失败。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnfreezeMatchedSpotOrdersError(pub String);

#[derive(Debug, Clone)]
struct AggregatedRelease {
    balance: BalanceV2,
    amount: Decimal,
}

#[derive(Debug, Clone, Copy)]
struct MatchEventMetadata {
    last_position: EventPosition,
    logical_ts: u64,
    topic: u8,
    partition: u8,
}

/// 从撮合订单终态事实聚合释放额，并为每个余额生成至多一个事件。
///
/// 所有订单、余额和金额在余额 FSM 执行前完成校验，错误时不会返回部分结果。
pub fn unfreeze_matched_spot_orders(
    taker_before: &SpotOrderV7,
    makers_before: &[SpotOrderV7],
    matched: &MatchSpotOrderPipeChanges,
    balances_by_order_id: &HashMap<u64, SpotOrderHeldBalance>,
) -> Result<Vec<UpdatedEntityEvents<BalanceV2>>, UnfreezeMatchedSpotOrdersError> {
    let changes = changed_orders(matched);
    if changes.is_empty() {
        return Ok(Vec::new());
    }
    let metadata = validate_match_event_metadata(&changes)?;
    unfreeze_matched_spot_orders_at(
        taker_before,
        makers_before,
        matched,
        balances_by_order_id,
        metadata.last_position,
        metadata.logical_ts,
        metadata.topic,
        metadata.partition,
    )
}

/// 使用显式定序元数据释放终态订单仍持有的冻结资金。
pub fn unfreeze_matched_spot_orders_at(
    taker_before: &SpotOrderV7,
    makers_before: &[SpotOrderV7],
    matched: &MatchSpotOrderPipeChanges,
    balances_by_order_id: &HashMap<u64, SpotOrderHeldBalance>,
    previous_position: EventPosition,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<Vec<UpdatedEntityEvents<BalanceV2>>, UnfreezeMatchedSpotOrdersError> {
    let changes = changed_orders(matched);
    if changes.is_empty() {
        return Ok(Vec::new());
    }
    let matched_metadata = validate_match_event_metadata(&changes)?;
    if previous_position < matched_metadata.last_position {
        return Err(error("unfreeze position precedes matched order events"));
    }
    let metadata = MatchEventMetadata {
        last_position: previous_position,
        logical_ts,
        topic,
        partition,
    };
    let before_by_id = index_before_orders(taker_before, makers_before)?;
    let mut releases = BTreeMap::<u64, AggregatedRelease>::new();

    for changed in changes {
        let order_id = changed.after().order_base().order_id;
        let before = before_by_id
            .get(&order_id)
            .ok_or_else(|| error(format!("missing before order {order_id}")))?;
        validate_changed_order(before, changed)?;
        let after = changed.after();
        if !matches!(
            after.status(),
            SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
        ) {
            continue;
        }
        let amount = after
            .terminal_unfreeze_delta(before)
            .map_err(|cause| {
                error(format!(
                    "invalid terminal unfreeze for order {order_id}: {cause}"
                ))
            })?
            .amount();
        if amount < Decimal::ZERO {
            return Err(error(format!("negative unfreeze for order {order_id}")));
        }
        if amount == Decimal::ZERO {
            continue;
        }
        let balance = validate_held_balance(
            before,
            balances_by_order_id
                .get(&order_id)
                .ok_or_else(|| error(format!("missing held balance for order {order_id}")))?,
        )?;
        let balance_id = balance.entity_id();
        match releases.get_mut(&balance_id) {
            Some(release) => {
                if release.balance != *balance {
                    return Err(error(format!(
                        "conflicting snapshots for balance {balance_id}"
                    )));
                }
                release.amount = release
                    .amount
                    .checked_add(amount)
                    .ok_or_else(|| error(format!("release overflow for balance {balance_id}")))?;
            }
            None => {
                releases.insert(
                    balance_id,
                    AggregatedRelease {
                        balance: balance.clone(),
                        amount,
                    },
                );
            }
        }
    }

    for (balance_id, release) in &releases {
        if release.amount <= Decimal::ZERO || release.balance.frozen() < release.amount {
            return Err(error(format!(
                "invalid or insufficient release for balance {balance_id}"
            )));
        }
    }
    transition_balances(releases, metadata)
}

fn changed_orders(matched: &MatchSpotOrderPipeChanges) -> Vec<&UpdatedEntityEvents<SpotOrderV7>> {
    match matched {
        MatchSpotOrderPipeChanges::Resting => Vec::new(),
        MatchSpotOrderPipeChanges::Rejected { taker } => vec![taker],
        MatchSpotOrderPipeChanges::PartiallyFilled { taker, makers, .. }
        | MatchSpotOrderPipeChanges::Filled { taker, makers, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { taker, makers, .. } => {
            let mut changes = Vec::with_capacity(1 + makers.len());
            changes.push(taker);
            changes.extend(makers);
            changes
        }
    }
}

fn index_before_orders<'a>(
    taker: &'a SpotOrderV7,
    makers: &'a [SpotOrderV7],
) -> Result<HashMap<u64, &'a SpotOrderV7>, UnfreezeMatchedSpotOrdersError> {
    let mut orders = HashMap::with_capacity(1 + makers.len());
    orders.insert(taker.order_base().order_id, taker);
    for maker in makers {
        let order_id = maker.order_base().order_id;
        if orders.insert(order_id, maker).is_some() {
            return Err(error(format!("duplicate before order {order_id}")));
        }
    }
    Ok(orders)
}

fn validate_changed_order(
    before: &SpotOrderV7,
    changed: &UpdatedEntityEvents<SpotOrderV7>,
) -> Result<(), UnfreezeMatchedSpotOrdersError> {
    let after = changed.after();
    if before.order_base().order_id != after.order_base().order_id
        || before.order_base().owner != after.order_base().owner
        || before.order_base().symbol != after.order_base().symbol
        || before.order_base().side != after.order_base().side
        || changed
            .events()
            .first()
            .is_none_or(|event| event.previous_position != before.event_position())
    {
        return Err(error(format!(
            "order identity or predecessor mismatch for order {}",
            after.order_base().order_id
        )));
    }
    Ok(())
}

fn validate_match_event_metadata(
    changes: &[&UpdatedEntityEvents<SpotOrderV7>],
) -> Result<MatchEventMetadata, UnfreezeMatchedSpotOrdersError> {
    let first = changes
        .first()
        .and_then(|change| change.events().first())
        .ok_or_else(|| error("matched order changes contain no event"))?;
    let mut metadata = MatchEventMetadata {
        last_position: first.position,
        logical_ts: first.logical_ts,
        topic: first.t,
        partition: first.p,
    };
    for event in changes.iter().flat_map(|change| change.events()) {
        if event.position.lsn != first.position.lsn
            || event.logical_ts != first.logical_ts
            || event.t != first.t
            || event.p != first.p
        {
            return Err(error("matched order event metadata mismatch"));
        }
        metadata.last_position = metadata.last_position.max(event.position);
    }
    Ok(metadata)
}

fn validate_held_balance<'a>(
    order: &SpotOrderV7,
    held: &'a SpotOrderHeldBalance,
) -> Result<&'a BalanceV2, UnfreezeMatchedSpotOrdersError> {
    let facts = order.order_base();
    let (balance, expected_coin) = match (facts.side, held) {
        (OrderSide::Buy, SpotOrderHeldBalance::Quote(balance)) => {
            (balance, u64::from(facts.symbol.quote.index()))
        }
        (OrderSide::Sell, SpotOrderHeldBalance::Base(balance)) => {
            (balance, u64::from(facts.symbol.base.index()))
        }
        _ => {
            return Err(error(format!(
                "held balance role mismatch for order {}",
                facts.order_id
            )));
        }
    };
    if balance.party_id() != facts.owner.0
        || balance.account_id() != facts.owner.0
        || balance.coin_idx() != expected_coin
    {
        return Err(error(format!(
            "held balance owner or coin mismatch for order {}",
            facts.order_id
        )));
    }
    Ok(balance)
}

fn transition_balances(
    releases: BTreeMap<u64, AggregatedRelease>,
    metadata: MatchEventMetadata,
) -> Result<Vec<UpdatedEntityEvents<BalanceV2>>, UnfreezeMatchedSpotOrdersError> {
    let mut output = Vec::with_capacity(releases.len());
    for (offset, release) in releases.into_values().enumerate() {
        let increment =
            u32::try_from(offset + 1).map_err(|_| error("balance event position overflow"))?;
        let command_index = metadata
            .last_position
            .command_index
            .checked_add(increment)
            .ok_or_else(|| error("balance event position overflow"))?;
        let position = EventPosition::new(metadata.last_position.lsn, command_index);
        let command = SequencedCommand::new(
            CommandMetadata::new_at(
                position,
                metadata.logical_ts,
                metadata.topic,
                metadata.partition,
                release.balance.party_id().to_string(),
            ),
            BalanceV2Command::UnfreezeAndPay {
                unfreeze_amount: release.amount,
                payment_amount: Decimal::ZERO,
            },
        );
        let (after, mut event) = FsmV2::transition(&release.balance, command)
            .map_err(|cause| error(format!("balance transition failed: {cause}")))?;
        event.t = metadata.topic;
        event.p = metadata.partition;
        output.push(
            UpdatedEntityEvents::try_new(after, release.balance.event_position(), vec![event])
                .map_err(|cause| error(format!("invalid balance event: {cause:?}")))?,
        );
    }
    Ok(output)
}

fn error(message: impl Into<String>) -> UnfreezeMatchedSpotOrdersError {
    UnfreezeMatchedSpotOrdersError(message.into())
}
