use std::collections::{BTreeMap, HashMap, HashSet};

use common_entity::fsm_v2::Fsm as FsmV2;
use common_entity::{
    Entity, LogEntrySequencer, ReplayableDomainEvent, ReplayableEntityChanges, UpdatedEntityEvents,
};
use example_core_entity::account::symbol::Symbol;
use example_core_entity::spot::{SpotTradeSettlementStatus, SpotTradeV3, SpotTradeV3Command};
use example_core_entity::{BalanceV2, BalanceV2Command, OrderSide, PartyId, SpotOrderV7};
use rust_decimal::Decimal;

use super::match_spot_order::MatchSpotOrderPipeChanges;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettleMatchedSpotTradesInput {
    pub lsn: u64,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleTradesPipeChanges {
    pub settled_trades: Vec<UpdatedEntityEvents<SpotTradeV3>>,
    pub updated_balances: Vec<UpdatedEntityEvents<BalanceV2>>,
}

impl ReplayableEntityChanges for SettleTradesPipeChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        self.settled_trades
            .iter()
            .flat_map(ReplayableEntityChanges::replayable_events)
            .chain(
                self.updated_balances
                    .iter()
                    .flat_map(ReplayableEntityChanges::replayable_events),
            )
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleMatchedSpotTradesError(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BalanceOperation {
    DebitFrozen,
    CreditAvailable,
}

#[derive(Debug, Clone)]
struct AggregatedBalanceChange {
    balance: BalanceV2,
    operation: BalanceOperation,
    amount: Decimal,
}

/// 原子规划并结算一次撮合产生的全部成交。
///
/// 函数在执行任何 FSM 前完成全部输入、金额、余额和位置校验；返回错误时不会暴露部分结果。
pub fn settle_matched_spot_trades(
    matched: &MatchSpotOrderPipeChanges,
    quote_balances: &BTreeMap<(PartyId, Symbol), BalanceV2>,
    base_balances: &BTreeMap<(PartyId, Symbol), BalanceV2>,
    input: SettleMatchedSpotTradesInput,
) -> Result<SettleTradesPipeChanges, SettleMatchedSpotTradesError> {
    settle_matched_spot_trades_from(matched, quote_balances, base_balances, input, 0)
}

/// 从指定 `command_index` 起结算撮合结果。
pub fn settle_matched_spot_trades_from(
    matched: &MatchSpotOrderPipeChanges,
    quote_balances: &BTreeMap<(PartyId, Symbol), BalanceV2>,
    base_balances: &BTreeMap<(PartyId, Symbol), BalanceV2>,
    input: SettleMatchedSpotTradesInput,
    command_index_start: u32,
) -> Result<SettleTradesPipeChanges, SettleMatchedSpotTradesError> {
    let (orders, trades) = match matched {
        MatchSpotOrderPipeChanges::Resting | MatchSpotOrderPipeChanges::Rejected { .. } => {
            return Ok(empty_changes());
        }
        MatchSpotOrderPipeChanges::PartiallyFilled {
            taker,
            makers,
            trades,
        }
        | MatchSpotOrderPipeChanges::Filled {
            taker,
            makers,
            trades,
        }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill {
            taker,
            makers,
            trades,
        } => {
            let mut orders = HashMap::with_capacity(1 + makers.len());
            orders.insert(taker.after().order_base().order_id, taker.after());
            for maker in makers {
                let order_id = maker.after().order_base().order_id;
                if orders.insert(order_id, maker.after()).is_some() {
                    return Err(error(format!("duplicate matched order {order_id}")));
                }
            }
            (orders, trades.as_slice())
        }
    };
    if trades.is_empty() {
        return Ok(empty_changes());
    }

    let mut balance_changes = BTreeMap::<u64, AggregatedBalanceChange>::new();
    let mut trade_ids = HashSet::with_capacity(trades.len());
    for created in trades {
        let trade = created.after();
        if !trade_ids.insert(trade.comon_trade.trade_id) {
            return Err(error(format!(
                "duplicate trade {}",
                trade.comon_trade.trade_id
            )));
        }
        validate_trade(
            trade,
            created.event().lsn,
            created.event().command_index,
            &orders,
            input,
            command_index_start,
        )?;
        let facts = &trade.comon_trade;
        let taker = orders
            .get(&facts.taker_order_id)
            .ok_or_else(|| error(format!("missing taker order {}", facts.taker_order_id)))?;
        let maker = orders
            .get(&facts.maker_order_id)
            .ok_or_else(|| error(format!("missing maker order {}", facts.maker_order_id)))?;
        let (buyer, seller) = match facts.taker_side {
            OrderSide::Buy => (*taker, *maker),
            OrderSide::Sell => (*maker, *taker),
        };
        if buyer.order_base().owner == seller.order_base().owner {
            return Err(error(format!(
                "trade {} has the same participant",
                facts.trade_id
            )));
        }
        let (buyer_base, buyer_quote) = validated_balances(
            buyer,
            quote_balances,
            base_balances,
            input.lsn,
            command_index_start,
        )?;
        let (seller_base, seller_quote) = validated_balances(
            seller,
            quote_balances,
            base_balances,
            input.lsn,
            command_index_start,
        )?;
        let amounts = trade.settlement_amounts().map_err(|cause| {
            error(format!(
                "invalid settlement amounts for trade {}: {cause}",
                facts.trade_id
            ))
        })?;

        aggregate(
            &mut balance_changes,
            buyer_quote,
            BalanceOperation::DebitFrozen,
            amounts.buyer_quote_payment,
        )?;
        aggregate(
            &mut balance_changes,
            buyer_base,
            BalanceOperation::CreditAvailable,
            amounts.base_amount,
        )?;
        aggregate(
            &mut balance_changes,
            seller_base,
            BalanceOperation::DebitFrozen,
            amounts.base_amount,
        )?;
        if amounts.seller_quote_receipt > Decimal::ZERO {
            aggregate(
                &mut balance_changes,
                seller_quote,
                BalanceOperation::CreditAvailable,
                amounts.seller_quote_receipt,
            )?;
        }
    }

    for (balance_id, change) in &balance_changes {
        if change.amount <= Decimal::ZERO {
            return Err(error(format!("invalid amount for balance {balance_id}")));
        }
        if change.operation == BalanceOperation::DebitFrozen
            && change.balance.frozen() < change.amount
        {
            return Err(error(format!(
                "insufficient frozen amount for balance {balance_id}"
            )));
        }
        if change.operation == BalanceOperation::CreditAvailable
            && change
                .balance
                .available()
                .checked_add(change.amount)
                .is_none()
        {
            return Err(error(format!(
                "available amount overflow for balance {balance_id}"
            )));
        }
    }

    let settled_trades = transition_trades(trades, input, command_index_start)?;
    let updated_balances = transition_balances(
        balance_changes,
        settled_trades.len(),
        input,
        command_index_start,
    )?;
    Ok(SettleTradesPipeChanges {
        settled_trades,
        updated_balances,
    })
}

fn empty_changes() -> SettleTradesPipeChanges {
    SettleTradesPipeChanges {
        settled_trades: Vec::new(),
        updated_balances: Vec::new(),
    }
}

fn validate_trade<'a>(
    trade: &SpotTradeV3,
    create_lsn: u64,
    create_command_index: u32,
    orders: &HashMap<u64, &'a SpotOrderV7>,
    input: SettleMatchedSpotTradesInput,
    command_index_start: u32,
) -> Result<(), SettleMatchedSpotTradesError> {
    let facts = &trade.comon_trade;
    if trade.settlement_status != SpotTradeSettlementStatus::Unsettled {
        return Err(error(format!(
            "trade {} is already settled",
            facts.trade_id
        )));
    }
    if (trade.lsn, trade.command_index) != (create_lsn, create_command_index)
        || (input.lsn, command_index_start) <= (create_lsn, create_command_index)
    {
        return Err(error(format!(
            "settlement lsn is not later than trade {}",
            facts.trade_id
        )));
    }
    let taker = orders
        .get(&facts.taker_order_id)
        .ok_or_else(|| error(format!("missing taker order {}", facts.taker_order_id)))?;
    let maker = orders
        .get(&facts.maker_order_id)
        .ok_or_else(|| error(format!("missing maker order {}", facts.maker_order_id)))?;
    let taker_facts = taker.order_base();
    let maker_facts = maker.order_base();
    if taker_facts.owner != facts.taker_party
        || maker_facts.owner != facts.maker_party
        || taker_facts.symbol != facts.symbol
        || maker_facts.symbol != facts.symbol
        || taker_facts.side != facts.taker_side
        || maker_facts.side == facts.taker_side
    {
        return Err(error(format!("trade {} identity mismatch", facts.trade_id)));
    }
    Ok(())
}

fn validated_balances<'a>(
    order: &SpotOrderV7,
    quote_balances: &'a BTreeMap<(PartyId, Symbol), BalanceV2>,
    base_balances: &'a BTreeMap<(PartyId, Symbol), BalanceV2>,
    settlement_lsn: u64,
    settlement_command_index: u32,
) -> Result<(&'a BalanceV2, &'a BalanceV2), SettleMatchedSpotTradesError> {
    let facts = order.order_base();
    let key = (facts.owner.clone(), facts.symbol);
    let base = base_balances
        .get(&key)
        .ok_or_else(|| error(format!("missing base balance for order {}", facts.order_id)))?;
    let quote = quote_balances.get(&key).ok_or_else(|| {
        error(format!(
            "missing quote balance for order {}",
            facts.order_id
        ))
    })?;
    if base.party_id() != &facts.owner || quote.party_id() != &facts.owner {
        return Err(error(format!(
            "balance party mismatch for order {}",
            facts.order_id
        )));
    }
    if base.coin_idx() != u64::from(facts.symbol.base.index())
        || quote.coin_idx() != u64::from(facts.symbol.quote.index())
    {
        return Err(error(format!(
            "balance coin mismatch for order {}",
            facts.order_id
        )));
    }
    if base.entity_id() == quote.entity_id() {
        return Err(error(format!(
            "duplicate balance identity for order {}",
            facts.order_id
        )));
    }
    if (base.lsn(), base.command_index()) >= (settlement_lsn, settlement_command_index)
        || (quote.lsn(), quote.command_index()) >= (settlement_lsn, settlement_command_index)
    {
        return Err(error(format!(
            "stale settlement lsn for order {}",
            facts.order_id
        )));
    }
    Ok((base, quote))
}

fn aggregate(
    changes: &mut BTreeMap<u64, AggregatedBalanceChange>,
    balance: &BalanceV2,
    operation: BalanceOperation,
    amount: Decimal,
) -> Result<(), SettleMatchedSpotTradesError> {
    if amount <= Decimal::ZERO {
        return Err(error("settlement amount must be positive"));
    }
    let balance_id = balance.entity_id();
    match changes.get_mut(&balance_id) {
        Some(change) => {
            if change.balance != *balance || change.operation != operation {
                return Err(error(format!(
                    "conflicting snapshot or role for balance {balance_id}"
                )));
            }
            change.amount = change
                .amount
                .checked_add(amount)
                .ok_or_else(|| error(format!("amount overflow for balance {balance_id}")))?;
        }
        None => {
            changes.insert(
                balance_id,
                AggregatedBalanceChange {
                    balance: balance.clone(),
                    operation,
                    amount,
                },
            );
        }
    }
    Ok(())
}

fn transition_trades(
    trades: &[common_entity::CreatedEntityEvent<SpotTradeV3>],
    input: SettleMatchedSpotTradesInput,
    command_index_start: u32,
) -> Result<Vec<UpdatedEntityEvents<SpotTradeV3>>, SettleMatchedSpotTradesError> {
    let mut output = Vec::with_capacity(trades.len());
    for (index, created) in trades.iter().enumerate() {
        let increment = u32::try_from(index).map_err(|_| error("event position overflow"))?;
        let command_index = command_index_start
            .checked_add(increment)
            .ok_or_else(|| error("event position overflow"))?;
        let trade = created.after();
        let command = LogEntrySequencer::from_metadata_with_topic_at(
            input.topic,
            input.partition,
            input.lsn,
            command_index,
            input.logical_ts,
            0,
            0,
            trade.comon_trade.taker_party.0.to_string(),
            SpotTradeV3Command::Settle,
        );
        let (after, event) = FsmV2::transition(trade, command)
            .map_err(|cause| error(format!("trade transition failed: {cause}")))?;
        output.push(
            UpdatedEntityEvents::try_new(after, trade.lsn, trade.command_index, vec![event])
                .map_err(|cause| error(format!("invalid trade event: {cause:?}")))?,
        );
    }
    Ok(output)
}

fn transition_balances(
    changes: BTreeMap<u64, AggregatedBalanceChange>,
    trade_count: usize,
    input: SettleMatchedSpotTradesInput,
    command_index_start: u32,
) -> Result<Vec<UpdatedEntityEvents<BalanceV2>>, SettleMatchedSpotTradesError> {
    let mut output = Vec::with_capacity(changes.len());
    for (offset, change) in changes.into_values().enumerate() {
        let index = trade_count
            .checked_add(offset)
            .ok_or_else(|| error("event position overflow"))?;
        let increment = u32::try_from(index).map_err(|_| error("event position overflow"))?;
        let command_index = command_index_start
            .checked_add(increment)
            .ok_or_else(|| error("event position overflow"))?;
        let command = match change.operation {
            BalanceOperation::DebitFrozen => BalanceV2Command::DebitFrozen {
                amount: change.amount,
            },
            BalanceOperation::CreditAvailable => BalanceV2Command::CreditAvailable {
                amount: change.amount,
            },
        };
        let command = LogEntrySequencer::from_metadata_with_topic_at(
            input.topic,
            input.partition,
            input.lsn,
            command_index,
            input.logical_ts,
            0,
            0,
            change.balance.party_id().to_string(),
            command,
        );
        let (after, event) = FsmV2::transition(&change.balance, command)
            .map_err(|cause| error(format!("balance transition failed: {cause}")))?;
        output.push(
            UpdatedEntityEvents::try_new(
                after,
                change.balance.lsn(),
                change.balance.command_index(),
                vec![event],
            )
            .map_err(|cause| error(format!("invalid balance event: {cause:?}")))?,
        );
    }
    Ok(output)
}

fn error(message: impl Into<String>) -> SettleMatchedSpotTradesError {
    SettleMatchedSpotTradesError(message.into())
}
