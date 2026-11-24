use std::collections::{BTreeMap, HashMap, HashSet};

use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
use common_entity::{
    Entity, EventPosition, ReplayableDomainEvent, ReplayableEntityChanges, UpdatedEntityEvents,
};
use example_core_entity::spot::{SpotTradeSettlementStatus, SpotTradeV3, SpotTradeV3Command};
use example_core_entity::{BalanceV2, BalanceV2Command, OrderSide, SpotOrderV7};
use rust_decimal::Decimal;

use super::match_spot_order::MatchSpotOrderChanges;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettleMatchedSpotTradesInput {
    pub lsn: u64,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderSettlementBalances {
    pub base: BalanceV2,
    pub quote: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleMatchedSpotTradesChanges {
    pub settled_trades: Vec<UpdatedEntityEvents<SpotTradeV3>>,
    pub updated_balances: Vec<UpdatedEntityEvents<BalanceV2>>,
}

impl ReplayableEntityChanges for SettleMatchedSpotTradesChanges {
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
    matched: &MatchSpotOrderChanges,
    balances_by_order_id: &HashMap<u64, SpotOrderSettlementBalances>,
    input: SettleMatchedSpotTradesInput,
) -> Result<SettleMatchedSpotTradesChanges, SettleMatchedSpotTradesError> {
    let (orders, trades) = match matched {
        MatchSpotOrderChanges::Resting | MatchSpotOrderChanges::Rejected { .. } => {
            return Ok(empty_changes());
        }
        MatchSpotOrderChanges::PartiallyFilled {
            taker,
            makers,
            trades,
        }
        | MatchSpotOrderChanges::Filled {
            taker,
            makers,
            trades,
        }
        | MatchSpotOrderChanges::CanceledAfterPartialFill {
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
        validate_trade(trade, created.event().position, &orders, input)?;
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
        let buyer_balances = validated_balances(buyer, balances_by_order_id, input.lsn)?;
        let seller_balances = validated_balances(seller, balances_by_order_id, input.lsn)?;
        let amounts = trade.settlement_amounts().map_err(|cause| {
            error(format!(
                "invalid settlement amounts for trade {}: {cause}",
                facts.trade_id
            ))
        })?;

        aggregate(
            &mut balance_changes,
            &buyer_balances.quote,
            BalanceOperation::DebitFrozen,
            amounts.buyer_quote_payment,
        )?;
        aggregate(
            &mut balance_changes,
            &buyer_balances.base,
            BalanceOperation::CreditAvailable,
            amounts.base_amount,
        )?;
        aggregate(
            &mut balance_changes,
            &seller_balances.base,
            BalanceOperation::DebitFrozen,
            amounts.base_amount,
        )?;
        if amounts.seller_quote_receipt > Decimal::ZERO {
            aggregate(
                &mut balance_changes,
                &seller_balances.quote,
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

    let settled_trades = transition_trades(trades, input)?;
    let updated_balances = transition_balances(balance_changes, settled_trades.len(), input)?;
    Ok(SettleMatchedSpotTradesChanges {
        settled_trades,
        updated_balances,
    })
}

fn empty_changes() -> SettleMatchedSpotTradesChanges {
    SettleMatchedSpotTradesChanges {
        settled_trades: Vec::new(),
        updated_balances: Vec::new(),
    }
}

fn validate_trade<'a>(
    trade: &SpotTradeV3,
    create_position: EventPosition,
    orders: &HashMap<u64, &'a SpotOrderV7>,
    input: SettleMatchedSpotTradesInput,
) -> Result<(), SettleMatchedSpotTradesError> {
    let facts = &trade.comon_trade;
    if trade.settlement_status != SpotTradeSettlementStatus::Unsettled {
        return Err(error(format!(
            "trade {} is already settled",
            facts.trade_id
        )));
    }
    if trade.position != create_position || input.lsn <= create_position.lsn {
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
    if taker_facts.owner != facts.taker_user
        || maker_facts.owner != facts.maker_user
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
    balances: &'a HashMap<u64, SpotOrderSettlementBalances>,
    settlement_lsn: u64,
) -> Result<&'a SpotOrderSettlementBalances, SettleMatchedSpotTradesError> {
    let facts = order.order_base();
    let pair = balances
        .get(&facts.order_id)
        .ok_or_else(|| error(format!("missing balances for order {}", facts.order_id)))?;
    let owner = facts.owner.0;
    if pair.base.party_id() != owner || pair.quote.party_id() != owner {
        return Err(error(format!(
            "balance party mismatch for order {}",
            facts.order_id
        )));
    }
    if pair.base.account_id() != pair.quote.account_id() || pair.base.account_id() != owner {
        return Err(error(format!(
            "balance account mismatch for order {}",
            facts.order_id
        )));
    }
    if pair.base.coin_idx() != u64::from(facts.symbol.base.index())
        || pair.quote.coin_idx() != u64::from(facts.symbol.quote.index())
    {
        return Err(error(format!(
            "balance coin mismatch for order {}",
            facts.order_id
        )));
    }
    if pair.base.entity_id() == pair.quote.entity_id() {
        return Err(error(format!(
            "duplicate balance identity for order {}",
            facts.order_id
        )));
    }
    if pair.base.position().lsn >= settlement_lsn || pair.quote.position().lsn >= settlement_lsn {
        return Err(error(format!(
            "stale settlement lsn for order {}",
            facts.order_id
        )));
    }
    Ok(pair)
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
) -> Result<Vec<UpdatedEntityEvents<SpotTradeV3>>, SettleMatchedSpotTradesError> {
    let mut output = Vec::with_capacity(trades.len());
    for (index, created) in trades.iter().enumerate() {
        let command_index = u32::try_from(index).map_err(|_| error("event position overflow"))?;
        let trade = created.after();
        let metadata = CommandMetadata::new_at(
            EventPosition::new(input.lsn, command_index),
            input.logical_ts,
            input.topic,
            input.partition,
            trade.comon_trade.taker_user.0.to_string(),
        );
        let (after, mut event) = FsmV2::transition(
            trade,
            SequencedCommand::new(metadata, SpotTradeV3Command::Settle),
        )
        .map_err(|cause| error(format!("trade transition failed: {cause}")))?;
        event.t = input.topic;
        event.p = input.partition;
        output.push(
            UpdatedEntityEvents::try_new(after, trade.event_position(), vec![event])
                .map_err(|cause| error(format!("invalid trade event: {cause:?}")))?,
        );
    }
    Ok(output)
}

fn transition_balances(
    changes: BTreeMap<u64, AggregatedBalanceChange>,
    trade_count: usize,
    input: SettleMatchedSpotTradesInput,
) -> Result<Vec<UpdatedEntityEvents<BalanceV2>>, SettleMatchedSpotTradesError> {
    let mut output = Vec::with_capacity(changes.len());
    for (offset, change) in changes.into_values().enumerate() {
        let index = trade_count
            .checked_add(offset)
            .ok_or_else(|| error("event position overflow"))?;
        let command_index = u32::try_from(index).map_err(|_| error("event position overflow"))?;
        let command = match change.operation {
            BalanceOperation::DebitFrozen => BalanceV2Command::DebitFrozen {
                amount: change.amount,
            },
            BalanceOperation::CreditAvailable => BalanceV2Command::CreditAvailable {
                amount: change.amount,
            },
        };
        let metadata = CommandMetadata::new_at(
            EventPosition::new(input.lsn, command_index),
            input.logical_ts,
            input.topic,
            input.partition,
            change.balance.party_id().to_string(),
        );
        let (after, mut event) =
            FsmV2::transition(&change.balance, SequencedCommand::new(metadata, command))
                .map_err(|cause| error(format!("balance transition failed: {cause}")))?;
        event.t = input.topic;
        event.p = input.partition;
        output.push(
            UpdatedEntityEvents::try_new(after, change.balance.event_position(), vec![event])
                .map_err(|cause| error(format!("invalid balance event: {cause:?}")))?,
        );
    }
    Ok(output)
}

fn error(message: impl Into<String>) -> SettleMatchedSpotTradesError {
    SettleMatchedSpotTradesError(message.into())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::PriceType;
    use example_core_entity::entity::common::UserId;
    use example_core_entity::entity::common::common_order::OrderBase;
    use example_core_entity::{OrderGroupRelation, OrderTif, SpotOrderStatus};

    use super::*;
    use crate::use_case::trading::spot::v3::domain_service::match_spot_order::{
        MatchSpotOrderInput, match_spot_order,
    };

    const MATCH_LSN: u64 = 50;
    const SETTLE_LSN: u64 = 60;

    fn order(id: u64, owner: u64, side: OrderSide, size: i64) -> SpotOrderV7 {
        let mut record = OrderBase {
            owner: UserId(owner),
            order_id: id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(100),
            size: Decimal::from(size),
            orig_size: Decimal::from(size),
            execution: PriceType::Limit,
            tif: OrderTif::Gtc,
            group_relation: OrderGroupRelation::Standalone,
            status: SpotOrderStatus::Open,
            lsn: 10,
            ..OrderBase::default()
        };
        match side {
            OrderSide::Buy => {
                record.buy_quote_principal_frozen = Decimal::from(100 * size);
                record.buy_quote_fee_reserve_frozen = Decimal::from(10);
            }
            OrderSide::Sell => record.sell_base_principal_frozen = Decimal::from(size),
        }
        SpotOrderV7::from_record(record).unwrap()
    }

    fn match_input(taker_fee_bps: u64, maker_fee_bps: u64) -> MatchSpotOrderInput {
        MatchSpotOrderInput {
            lsn: MATCH_LSN,
            logical_ts: 900,
            topic: 7,
            partition: 3,
            trade_id_start: 100,
            maker_fee_bps,
            taker_fee_bps,
        }
    }

    fn settle_input() -> SettleMatchedSpotTradesInput {
        SettleMatchedSpotTradesInput {
            lsn: SETTLE_LSN,
            logical_ts: 1_000,
            topic: 8,
            partition: 4,
        }
    }

    fn balances(owner: u64, base_frozen: i64, quote_frozen: i64) -> SpotOrderSettlementBalances {
        SpotOrderSettlementBalances {
            base: BalanceV2::new(
                owner,
                owner,
                u64::from(Coin::BTC.index()),
                Decimal::ZERO,
                Decimal::from(base_frozen),
                EventPosition::new(20, 0),
                1,
            ),
            quote: BalanceV2::new(
                owner,
                owner,
                u64::from(Coin::USDT.index()),
                Decimal::ZERO,
                Decimal::from(quote_frozen),
                EventPosition::new(20, 1),
                1,
            ),
        }
    }

    fn matched(
        taker: &SpotOrderV7,
        makers: &[SpotOrderV7],
        taker_fee: u64,
        maker_fee: u64,
    ) -> MatchSpotOrderChanges {
        match_spot_order(taker, makers, match_input(taker_fee, maker_fee)).unwrap()
    }

    fn after_balance(
        changes: &SettleMatchedSpotTradesChanges,
        owner: u64,
        coin: Coin,
    ) -> &BalanceV2 {
        changes
            .updated_balances
            .iter()
            .map(UpdatedEntityEvents::after)
            .find(|balance| {
                balance.party_id() == owner && balance.coin_idx() == u64::from(coin.index())
            })
            .unwrap()
    }

    #[test]
    fn settles_taker_buy_with_trade_first_metadata_and_four_balance_legs() {
        let taker = order(1, 10, OrderSide::Buy, 2);
        let maker = order(2, 20, OrderSide::Sell, 2);
        let matched = matched(&taker, &[maker], 100, 50);
        let snapshots = HashMap::from([(1, balances(10, 0, 300)), (2, balances(20, 2, 0))]);

        let changes = settle_matched_spot_trades(&matched, &snapshots, settle_input()).unwrap();

        assert_eq!(changes.settled_trades.len(), 1);
        assert_eq!(changes.updated_balances.len(), 4);
        assert_eq!(
            changes.settled_trades[0].after().settlement_status,
            SpotTradeSettlementStatus::Settled
        );
        assert_eq!(
            after_balance(&changes, 10, Coin::USDT).frozen(),
            Decimal::from(98)
        );
        assert_eq!(
            after_balance(&changes, 10, Coin::BTC).available(),
            Decimal::from(2)
        );
        assert_eq!(
            after_balance(&changes, 20, Coin::BTC).frozen(),
            Decimal::ZERO
        );
        assert_eq!(
            after_balance(&changes, 20, Coin::USDT).available(),
            Decimal::from(199)
        );

        let events = changes.replayable_events();
        assert_eq!(events.len(), 5);
        assert_eq!(
            events[0].entity_type,
            <SpotTradeV3 as Entity>::entity_type()
        );
        assert!(
            events[1..]
                .iter()
                .all(|event| event.entity_type == <BalanceV2 as Entity>::entity_type())
        );
        assert!(events.iter().enumerate().all(|(index, event)| {
            event.position == EventPosition::new(SETTLE_LSN, index as u32)
                && event.logical_ts == 1_000
                && event.t == 8
                && event.p == 4
        }));
        assert_eq!(
            events[0].previous_position,
            EventPosition::new(MATCH_LSN, 2)
        );
    }

    #[test]
    fn settles_taker_sell_with_taker_fee_charged_to_seller() {
        let taker = order(1, 10, OrderSide::Sell, 2);
        let maker = order(2, 20, OrderSide::Buy, 2);
        let matched = matched(&taker, &[maker], 100, 50);
        let snapshots = HashMap::from([(1, balances(10, 2, 0)), (2, balances(20, 0, 300))]);

        let changes = settle_matched_spot_trades(&matched, &snapshots, settle_input()).unwrap();

        assert_eq!(
            after_balance(&changes, 20, Coin::USDT).frozen(),
            Decimal::from(99)
        );
        assert_eq!(
            after_balance(&changes, 10, Coin::USDT).available(),
            Decimal::from(198)
        );
    }

    #[test]
    fn aggregates_shared_balances_and_sorts_balance_events_by_entity_id() {
        let taker = order(1, 10, OrderSide::Buy, 3);
        let makers = vec![
            order(2, 20, OrderSide::Sell, 1),
            order(3, 30, OrderSide::Sell, 2),
        ];
        let matched = matched(&taker, &makers, 0, 0);
        let snapshots = HashMap::from([
            (1, balances(10, 0, 300)),
            (2, balances(20, 1, 0)),
            (3, balances(30, 2, 0)),
        ]);

        let changes = settle_matched_spot_trades(&matched, &snapshots, settle_input()).unwrap();

        assert_eq!(changes.settled_trades.len(), 2);
        assert_eq!(changes.updated_balances.len(), 6);
        assert_eq!(
            after_balance(&changes, 10, Coin::BTC).available(),
            Decimal::from(3)
        );
        assert_eq!(
            after_balance(&changes, 10, Coin::USDT).frozen(),
            Decimal::ZERO
        );
        let balance_ids = changes
            .updated_balances
            .iter()
            .map(|change| change.after().entity_id())
            .collect::<Vec<_>>();
        assert!(balance_ids.windows(2).all(|ids| ids[0] < ids[1]));
        let keys = changes
            .replayable_events()
            .into_iter()
            .map(|event| (event.position.lsn, event.entity_type, event.entity_id))
            .collect::<HashSet<_>>();
        assert_eq!(keys.len(), 8);
    }

    #[test]
    fn skips_zero_seller_receipt_and_empty_match_variants() {
        let taker = order(1, 10, OrderSide::Buy, 1);
        let maker = order(2, 20, OrderSide::Sell, 1);
        let matched = matched(&taker, &[maker], 0, 10_000);
        let snapshots = HashMap::from([(1, balances(10, 0, 100)), (2, balances(20, 1, 0))]);
        let changes = settle_matched_spot_trades(&matched, &snapshots, settle_input()).unwrap();
        assert_eq!(changes.updated_balances.len(), 3);
        assert!(changes.updated_balances.iter().all(|change| {
            !(change.after().party_id() == 20
                && change.after().coin_idx() == u64::from(Coin::USDT.index()))
        }));

        assert_eq!(
            settle_matched_spot_trades(
                &MatchSpotOrderChanges::Resting,
                &HashMap::new(),
                settle_input()
            )
            .unwrap(),
            empty_changes()
        );
    }

    #[test]
    fn rejects_invalid_inputs_without_mutating_snapshots() {
        let taker = order(1, 10, OrderSide::Buy, 1);
        let maker = order(2, 20, OrderSide::Sell, 1);
        let matched = matched(&taker, &[maker], 0, 0);
        let snapshots = HashMap::from([(1, balances(10, 0, 100)), (2, balances(20, 0, 0))]);
        let original = snapshots.clone();

        assert!(settle_matched_spot_trades(&matched, &snapshots, settle_input()).is_err());
        assert_eq!(snapshots, original);
        assert!(
            settle_matched_spot_trades(
                &matched,
                &HashMap::new(),
                SettleMatchedSpotTradesInput {
                    lsn: MATCH_LSN,
                    ..settle_input()
                },
            )
            .is_err()
        );
    }
}
