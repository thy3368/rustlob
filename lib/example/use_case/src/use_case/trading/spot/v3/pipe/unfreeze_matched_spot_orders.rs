use std::collections::{BTreeMap, HashMap};

use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
use common_entity::{Entity, ReplayableDomainEvent, ReplayableEntityChanges, UpdatedEntityEvents};
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

/// 撮合终态订单解冻阶段产生的余额变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnfreezeMatchedSpotOrdersPipeChanges {
    pub updated_balances: Vec<UpdatedEntityEvents<BalanceV2>>,
}

impl ReplayableEntityChanges for UnfreezeMatchedSpotOrdersPipeChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        self.updated_balances
            .iter()
            .flat_map(ReplayableEntityChanges::replayable_events)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct AggregatedRelease {
    balance: BalanceV2,
    amount: Decimal,
}

#[derive(Debug, Clone, Copy)]
struct MatchEventMetadata {
    lsn: u64,
    last_command_index: u32,
    logical_ts: u64,
    topic: u8,
    partition: u8,
}

/// 使用显式定序元数据释放终态订单仍持有的冻结资金。
pub fn unfreeze_matched_spot_orders(
    taker_before: &SpotOrderV7,
    makers_before: &[SpotOrderV7],
    matched: &MatchSpotOrderPipeChanges,
    balances_by_order_id: &HashMap<u64, SpotOrderHeldBalance>,
    previous_lsn: u64,
    previous_command_index: u32,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<UnfreezeMatchedSpotOrdersPipeChanges, UnfreezeMatchedSpotOrdersError> {
    let changes = changed_orders(matched);
    if changes.is_empty() {
        return Ok(UnfreezeMatchedSpotOrdersPipeChanges {
            updated_balances: Vec::new(),
        });
    }
    let matched_metadata = validate_match_event_metadata(&changes)?;
    if (previous_lsn, previous_command_index)
        < (matched_metadata.lsn, matched_metadata.last_command_index)
    {
        return Err(error("unfreeze position precedes matched order events"));
    }
    let metadata = MatchEventMetadata {
        lsn: previous_lsn,
        last_command_index: previous_command_index,
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
            SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
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
    Ok(UnfreezeMatchedSpotOrdersPipeChanges {
        updated_balances: transition_balances(releases, metadata)?,
    })
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
        || changed.events().first().is_none_or(|event| {
            (event.previous_lsn, event.previous_command_index)
                != (before.lsn(), before.command_index())
        })
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
        lsn: first.lsn,
        last_command_index: first.command_index,
        logical_ts: first.logical_ts,
        topic: first.t,
        partition: first.p,
    };
    for event in changes.iter().flat_map(|change| change.events()) {
        if event.lsn != first.lsn
            || event.logical_ts != first.logical_ts
            || event.t != first.t
            || event.p != first.p
        {
            return Err(error("matched order event metadata mismatch"));
        }
        metadata.last_command_index = metadata.last_command_index.max(event.command_index);
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
            .last_command_index
            .checked_add(increment)
            .ok_or_else(|| error("balance event position overflow"))?;
        let command = SequencedCommand::new(
            CommandMetadata::new_at(
                metadata.lsn,
                command_index,
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
        let (after, event) = FsmV2::transition(&release.balance, command)
            .map_err(|cause| error(format!("balance transition failed: {cause}")))?;
        output.push(
            UpdatedEntityEvents::try_new(
                after,
                release.balance.lsn(),
                release.balance.command_index(),
                vec![event],
            )
            .map_err(|cause| error(format!("invalid balance event: {cause:?}")))?,
        );
    }
    Ok(output)
}

fn error(message: impl Into<String>) -> UnfreezeMatchedSpotOrdersError {
    UnfreezeMatchedSpotOrdersError(message.into())
}

#[cfg(test)]
mod tests {
    use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
    use common_entity::{Entity, ReplayableEntityChanges, UpdatedEntityEvents};
    use example_core_entity::{BalanceV2, BalanceV2Command};
    use rust_decimal::Decimal;

    use super::UnfreezeMatchedSpotOrdersPipeChanges;

    fn updated_balance(
        party_id: u64,
        coin_idx: u64,
        command_index: u32,
    ) -> UpdatedEntityEvents<BalanceV2> {
        let before = BalanceV2::new(party_id, party_id, coin_idx, 10, 2, 1, 1);
        let command = SequencedCommand::new(
            CommandMetadata::new_at(2, command_index, 3, 4, 5, party_id.to_string()),
            BalanceV2Command::UnfreezeAndPay {
                unfreeze_amount: Decimal::ONE,
                payment_amount: Decimal::ZERO,
            },
        );
        let (after, event) = FsmV2::transition(&before, command).expect("余额解冻应成功");
        UpdatedEntityEvents::try_new(after, before.lsn(), before.command_index(), vec![event])
            .expect("余额事件链应有效")
    }

    #[test]
    fn no_unfreeze_changes_project_no_events() {
        // Rule: 无需释放冻结资金时，阶段结果和事件投影都必须为空。
        // Given: 解冻阶段没有产生余额变化。
        let changes = UnfreezeMatchedSpotOrdersPipeChanges {
            updated_balances: Vec::new(),
        };

        // When: 投影阶段事件。
        let events = changes.replayable_events();

        // Then: 业务变化和投影事件均为空。
        assert!(changes.updated_balances.is_empty());
        assert!(events.is_empty());
    }

    #[test]
    fn multiple_unfreeze_changes_preserve_balance_and_event_order() {
        // Rule: 解冻事件必须按 updated_balances 的业务顺序稳定投影。
        // Given: 两个余额依次完成解冻。
        let first = updated_balance(10, 1, 7);
        let second = updated_balance(20, 2, 8);
        let expected_entity_ids = [first.after().entity_id(), second.after().entity_id()];
        let changes = UnfreezeMatchedSpotOrdersPipeChanges {
            updated_balances: vec![first, second],
        };

        // When: 投影阶段事件。
        let events = changes.replayable_events();

        // Then: 先保留完整余额变化，再以相同顺序展平底层事件。
        assert_eq!(changes.updated_balances.len(), 2);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].entity_id, expected_entity_ids[0]);
        assert_eq!(events[0].command_index, 7);
        assert_eq!(events[1].entity_id, expected_entity_ids[1]);
        assert_eq!(events[1].command_index, 8);
    }
}
