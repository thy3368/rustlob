use std::collections::{HashMap, HashSet};

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::spot::SpotTradeV3;
use example_core_entity::{BalanceV2, OrderGroupRelation, OrderType, SpotOrderStatus, SpotOrderV7};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::use_case::trading::spot::v3::domain_service::place_order_pipeline::{
    PlaceOrderPipeline, PlaceDomainServiceError, PlaceNormalSpotOrderV2Input,
    PlaceNormalSpotOrderV3AfterChanges, PlaceTriggerSpotOrderV2Input,
    PlaceTriggerSpotOrderV3AfterChanges,
};
pub use crate::use_case::trading::spot::v3::domain_service::place_order_pipeline::{
    PlaceSpotOrderV3OrderCmd, SpotOrderV3Balances,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderV3Cmd {
    Single(PlaceSpotOrderV3OrderCmd),
    NormalTpsl { parent: PlaceSpotOrderV3OrderCmd, children: Vec<PlaceSpotOrderV3OrderCmd> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV3State {
    pub maker_orders: Vec<SpotOrderV7>,
    pub balances_by_order_id: HashMap<u64, SpotOrderV3Balances>,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderV3AfterChanges {
    Normal(PlaceNormalSpotOrderV3AfterChanges),
    Trigger(PlaceTriggerSpotOrderV3AfterChanges),
    NormalTpsl {
        parent: PlaceNormalSpotOrderV3AfterChanges,
        children: Vec<PlaceTriggerSpotOrderV3AfterChanges>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderV3Result {
    Pending,
    Resting,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV3Changes {
    pub created_orders: Vec<SpotOrderV7>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV7>>,
    pub settled_trades: Vec<SpotTradeV3>,
    pub updated_balances: Vec<UpdatedEntityPair<BalanceV2>>,
    pub result: PlaceSpotOrderV3Result,
}

impl ReplayableChanges for PlaceSpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::new();
        for order in &self.created_orders {
            events.push(order.track_create_event()?);
        }
        for pair in &self.updated_maker_orders {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        for trade in &self.settled_trades {
            events.push(trade.track_create_event()?);
        }
        for pair in &self.updated_balances {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        Ok(events)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceSpotOrderV3Error {
    #[error("order price and size must be greater than zero")]
    InvalidAmount,
    #[error("order id must be non-zero and unique")]
    InvalidOrderId,
    #[error("normal TP/SL group is invalid")]
    InvalidGroup,
    #[error("maker state does not match the command")]
    InvalidMaker,
    #[error("command LSN must be newer than every loaded maker order")]
    StaleMaker,
    #[error("authoritative balances are missing, duplicated, or mismatched")]
    InvalidBalances,
    #[error("trade id range overflows")]
    TradeIdOverflow,
    #[error(transparent)]
    Domain(#[from] PlaceDomainServiceError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceSpotOrderV3UseCase;

impl UseCase for PlaceSpotOrderV3UseCase {
    type Command = LogEntry<PlaceSpotOrderV3Cmd>;
    type StateGiven = PlaceSpotOrderV3State;
    type Error = PlaceSpotOrderV3Error;
    type StateChanged = PlaceSpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        let orders = command_orders(cmd);
        let mut ids = HashSet::with_capacity(orders.len());
        for order in &orders {
            if order.order_id == 0 || !ids.insert(order.order_id) {
                return Err(Self::Error::InvalidOrderId);
            }
            if order.price <= Decimal::ZERO || order.size <= Decimal::ZERO {
                return Err(Self::Error::InvalidAmount);
            }
            if matches!(order.order_type, OrderType::Trigger { trigger_price: 0, .. }) {
                return Err(Self::Error::InvalidAmount);
            }
        }
        if let PlaceSpotOrderV3Cmd::NormalTpsl { parent, children } = cmd.payload() {
            if children.is_empty()
                || parent.order_type.is_trigger()
                || children.iter().any(|child| {
                    !child.order_type.is_trigger()
                        || child.party_id != parent.party_id
                        || child.symbol != parent.symbol
                        || child.side == parent.side
                })
            {
                return Err(Self::Error::InvalidGroup);
            }
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let parent = parent_command(cmd);
        if parent.order_type.is_trigger() {
            return Ok(());
        }
        let mut order_ids = HashSet::from([parent.order_id]);
        for maker in &state.maker_orders {
            let value = maker.order_base();
            if cmd.lsn() <= value.lsn {
                return Err(Self::Error::StaleMaker);
            }
            if !order_ids.insert(value.order_id)
                || value.symbol != parent.symbol
                || value.side == parent.side
                || !matches!(value.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
                || value.size <= Decimal::ZERO
            {
                return Err(Self::Error::InvalidMaker);
            }
        }
        if state.trade_id_start.checked_add(state.maker_orders.len() as u64).is_none() {
            return Err(Self::Error::TradeIdOverflow);
        }
        validate_normal_bindings(parent, state)?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let trigger = |order: &PlaceSpotOrderV3OrderCmd, relation| {
            PlaceOrderPipeline::place_trigger_order_v2(PlaceTriggerSpotOrderV2Input {
                order: order.clone(),
                relation,
                taker_fee_bps: state.taker_fee_bps,
                lsn: cmd.lsn(),
                timestamp: cmd.logical_ts(),
            })
        };
        match cmd.payload() {
            PlaceSpotOrderV3Cmd::Single(order) if order.order_type.is_trigger() => {
                Ok(PlaceSpotOrderV3AfterChanges::Trigger(trigger(
                    order,
                    OrderGroupRelation::Standalone,
                )?))
            }
            PlaceSpotOrderV3Cmd::Single(order) => Ok(PlaceSpotOrderV3AfterChanges::Normal(
                place_normal(order, OrderGroupRelation::Standalone, cmd, state)?,
            )),
            PlaceSpotOrderV3Cmd::NormalTpsl { parent, children } => {
                let parent_after =
                    place_normal(parent, OrderGroupRelation::NormalTpslParent, cmd, state)?;
                let children_after = if normal_is_rejected(&parent_after) {
                    Vec::new()
                } else {
                    children
                        .iter()
                        .map(|child| {
                            trigger(
                                child,
                                OrderGroupRelation::NormalTpslChild {
                                    parent_order_id: parent.order_id,
                                },
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?
                };
                Ok(PlaceSpotOrderV3AfterChanges::NormalTpsl {
                    parent: parent_after,
                    children: children_after,
                })
            }
        }
    }
}

impl StateDiff for PlaceSpotOrderV3UseCase {
    type StateDiff = PlaceSpotOrderV3Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        let flattened = flatten_after_changes(changed);
        let mut maker_orders_before = state
            .maker_orders
            .into_iter()
            .map(|maker| (maker.order_base().order_id, maker))
            .collect::<HashMap<_, _>>();
        let mut updated_maker_orders =
            Vec::with_capacity(flattened.updated_maker_orders_after.len());
        for after in flattened.updated_maker_orders_after {
            let before = maker_orders_before
                .remove(&after.order_base().order_id)
                .ok_or(PlaceSpotOrderV3Error::InvalidMaker)?;
            updated_maker_orders.push(UpdatedEntityPair { before, after });
        }
        let mut updated_balances = Vec::new();
        for after in flattened.updated_balances_after {
            let before = find_bound_balance(&state.balances_by_order_id, &after)?;
            updated_balances.push(UpdatedEntityPair { before, after });
        }
        Ok(PlaceSpotOrderV3Changes {
            created_orders: flattened.created_orders,
            updated_maker_orders,
            settled_trades: flattened.settled_trades,
            updated_balances,
            result: flattened.result,
        })
    }
}

fn place_normal(
    order: &PlaceSpotOrderV3OrderCmd,
    relation: OrderGroupRelation,
    cmd: &LogEntry<PlaceSpotOrderV3Cmd>,
    state: &PlaceSpotOrderV3State,
) -> Result<PlaceNormalSpotOrderV3AfterChanges, PlaceDomainServiceError> {
    PlaceOrderPipeline::place_normal_order_v2(PlaceNormalSpotOrderV2Input {
        order: order.clone(),
        relation,
        maker_orders: state.maker_orders.clone(),
        balances_by_order_id: state.balances_by_order_id.clone(),
        trade_id_start: state.trade_id_start,
        maker_fee_bps: state.maker_fee_bps,
        taker_fee_bps: state.taker_fee_bps,
        lsn: cmd.lsn(),
        timestamp: cmd.logical_ts(),
    })
}

fn command_orders(cmd: &LogEntry<PlaceSpotOrderV3Cmd>) -> Vec<&PlaceSpotOrderV3OrderCmd> {
    match cmd.payload() {
        PlaceSpotOrderV3Cmd::Single(order) => vec![order],
        PlaceSpotOrderV3Cmd::NormalTpsl { parent, children } => {
            let mut orders = Vec::with_capacity(children.len() + 1);
            orders.push(parent);
            orders.extend(children);
            orders
        }
    }
}

fn parent_command(cmd: &LogEntry<PlaceSpotOrderV3Cmd>) -> &PlaceSpotOrderV3OrderCmd {
    match cmd.payload() {
        PlaceSpotOrderV3Cmd::Single(order) => order,
        PlaceSpotOrderV3Cmd::NormalTpsl { parent, .. } => parent,
    }
}

fn validate_normal_bindings(
    parent: &PlaceSpotOrderV3OrderCmd,
    state: &PlaceSpotOrderV3State,
) -> Result<(), PlaceSpotOrderV3Error> {
    let mut orders = Vec::with_capacity(state.maker_orders.len() + 1);
    orders.push((parent.order_id, parent.party_id, parent.symbol));
    orders.extend(state.maker_orders.iter().map(|maker| {
        let base = maker.order_base();
        (base.order_id, base.owner.0, base.symbol)
    }));
    let mut snapshots = HashMap::<(u64, u64), &BalanceV2>::new();
    for (order_id, party_id, symbol) in orders {
        let binding = state
            .balances_by_order_id
            .get(&order_id)
            .ok_or(PlaceSpotOrderV3Error::InvalidBalances)?;
        if binding.base.party_id() != party_id
            || binding.quote.party_id() != party_id
            || binding.base.account_id() != binding.quote.account_id()
            || binding.base.coin_idx() != u64::from(symbol.base.index())
            || binding.quote.coin_idx() != u64::from(symbol.quote.index())
        {
            return Err(PlaceSpotOrderV3Error::InvalidBalances);
        }
        for balance in [&binding.base, &binding.quote] {
            let identity = (balance.account_id(), balance.coin_idx());
            if snapshots.insert(identity, balance).is_some_and(|old| old != balance) {
                return Err(PlaceSpotOrderV3Error::InvalidBalances);
            }
        }
    }
    Ok(())
}

fn find_bound_balance(
    bindings: &HashMap<u64, SpotOrderV3Balances>,
    balance: &BalanceV2,
) -> Result<BalanceV2, PlaceSpotOrderV3Error> {
    bindings
        .values()
        .flat_map(|binding| [&binding.base, &binding.quote])
        .find(|candidate| {
            candidate.account_id() == balance.account_id()
                && candidate.coin_idx() == balance.coin_idx()
        })
        .cloned()
        .ok_or(PlaceSpotOrderV3Error::InvalidBalances)
}

fn normal_is_rejected(value: &PlaceNormalSpotOrderV3AfterChanges) -> bool {
    matches!(value, PlaceNormalSpotOrderV3AfterChanges::Rejected { .. })
}

struct FlattenedAfter {
    created_orders: Vec<SpotOrderV7>,
    updated_maker_orders_after: Vec<SpotOrderV7>,
    settled_trades: Vec<SpotTradeV3>,
    updated_balances_after: Vec<BalanceV2>,
    result: PlaceSpotOrderV3Result,
}

fn flatten_after_changes(value: PlaceSpotOrderV3AfterChanges) -> FlattenedAfter {
    match value {
        PlaceSpotOrderV3AfterChanges::Normal(normal) => flatten_normal(normal, Vec::new()),
        PlaceSpotOrderV3AfterChanges::Trigger(trigger) => FlattenedAfter {
            created_orders: vec![trigger.created_trigger_order],
            updated_maker_orders_after: Vec::new(),
            settled_trades: Vec::new(),
            updated_balances_after: Vec::new(),
            result: PlaceSpotOrderV3Result::Pending,
        },
        PlaceSpotOrderV3AfterChanges::NormalTpsl { parent, children } => {
            flatten_normal(parent, children)
        }
    }
}

fn flatten_normal(
    normal: PlaceNormalSpotOrderV3AfterChanges,
    children: Vec<PlaceTriggerSpotOrderV3AfterChanges>,
) -> FlattenedAfter {
    let child_orders = children.into_iter().map(|child| child.created_trigger_order);
    let (parent, makers, trades, balances, result) = match normal {
        PlaceNormalSpotOrderV3AfterChanges::Resting {
            created_taker_order,
            updated_balances_after,
        } => (
            created_taker_order,
            Vec::new(),
            Vec::new(),
            updated_balances_after,
            PlaceSpotOrderV3Result::Resting,
        ),
        PlaceNormalSpotOrderV3AfterChanges::PartiallyFilled {
            created_taker_order,
            updated_maker_orders_after,
            created_settled_trades,
            updated_balances_after,
        } => (
            created_taker_order,
            updated_maker_orders_after,
            created_settled_trades,
            updated_balances_after,
            PlaceSpotOrderV3Result::PartiallyFilled,
        ),
        PlaceNormalSpotOrderV3AfterChanges::Filled {
            created_taker_order,
            updated_maker_orders_after,
            created_settled_trades,
            updated_balances_after,
        } => (
            created_taker_order,
            updated_maker_orders_after,
            created_settled_trades,
            updated_balances_after,
            PlaceSpotOrderV3Result::Filled,
        ),
        PlaceNormalSpotOrderV3AfterChanges::CanceledAfterPartialFill {
            created_taker_order,
            updated_maker_orders_after,
            created_settled_trades,
            updated_balances_after,
        } => (
            created_taker_order,
            updated_maker_orders_after,
            created_settled_trades,
            updated_balances_after,
            PlaceSpotOrderV3Result::Canceled,
        ),
        PlaceNormalSpotOrderV3AfterChanges::Rejected { created_taker_order } => (
            created_taker_order,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            PlaceSpotOrderV3Result::Rejected,
        ),
    };
    let mut created_orders = vec![parent];
    created_orders.extend(child_orders);
    FlattenedAfter {
        created_orders,
        updated_maker_orders_after: makers,
        settled_trades: trades,
        updated_balances_after: balances,
        result,
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, StateDiff, UseCase};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::{OrderBase, PriceType, UserId};
    use example_core_entity::{OrderSide, OrderTriggerRole};

    use super::*;

    const PARTY: u64 = 11;
    const ACCOUNT: u64 = 22;

    fn symbol() -> Symbol {
        Symbol::new(1, Coin::BTC, Coin::USDT)
    }

    fn order(order_id: u64, side: OrderSide, order_type: OrderType) -> PlaceSpotOrderV3OrderCmd {
        PlaceSpotOrderV3OrderCmd {
            party_id: PARTY,
            order_id,
            symbol: symbol(),
            side,
            price: Decimal::from(10),
            size: Decimal::ONE,
            order_type,
            cloid: None,
        }
    }

    fn trigger(order_id: u64) -> PlaceSpotOrderV3OrderCmd {
        order(
            order_id,
            OrderSide::Sell,
            OrderType::Trigger {
                is_market: true,
                trigger_price: 11,
                tpsl: OrderTriggerRole::TakeProfit,
            },
        )
    }

    fn binding() -> SpotOrderV3Balances {
        SpotOrderV3Balances {
            base: BalanceV2::new(PARTY, ACCOUNT, u64::from(Coin::BTC.index()), 100, 0, 1, 1),
            quote: BalanceV2::new(PARTY, ACCOUNT, u64::from(Coin::USDT.index()), 100, 0, 1, 1),
        }
    }

    fn maker(order_id: u64) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(PARTY + order_id),
            order_id,
            symbol: symbol(),
            side: OrderSide::Sell,
            limit_px: Decimal::from(10),
            size: Decimal::ONE,
            orig_size: Decimal::ONE,
            execution: PriceType::Limit,
            tif: example_core_entity::OrderTif::Gtc,
            status: SpotOrderStatus::Open,
            ..OrderBase::default()
        })
        .unwrap()
    }

    fn maker_binding(order_id: u64) -> SpotOrderV3Balances {
        SpotOrderV3Balances {
            base: BalanceV2::new(
                PARTY + order_id,
                ACCOUNT + order_id,
                u64::from(Coin::BTC.index()),
                100,
                0,
                1,
                1,
            ),
            quote: BalanceV2::new(
                PARTY + order_id,
                ACCOUNT + order_id,
                u64::from(Coin::USDT.index()),
                100,
                0,
                1,
                1,
            ),
        }
    }

    fn rejection_state(with_maker: bool) -> PlaceSpotOrderV3State {
        let mut balances_by_order_id = HashMap::from([(1, binding())]);
        let maker_orders = if with_maker {
            balances_by_order_id.insert(10, maker_binding(10));
            vec![maker(10)]
        } else {
            Vec::new()
        };
        PlaceSpotOrderV3State {
            maker_orders,
            balances_by_order_id,
            trade_id_start: 1,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        }
    }

    fn state(with_parent_balance: bool) -> PlaceSpotOrderV3State {
        PlaceSpotOrderV3State {
            maker_orders: Vec::new(),
            balances_by_order_id: if with_parent_balance {
                HashMap::from([(1, binding())])
            } else {
                HashMap::new()
            },
            trade_id_start: 1,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        }
    }

    fn command(payload: PlaceSpotOrderV3Cmd) -> LogEntry<PlaceSpotOrderV3Cmd> {
        LogEntrySequencer::from_metadata(1, 100, 1, 1, payload)
    }

    fn pending_order(order_id: u64, side: OrderSide, lsn: u64) -> SpotOrderV7 {
        SpotOrderV7::pending(
            OrderBase {
                owner: UserId(PARTY),
                order_id,
                symbol: symbol(),
                side,
                limit_px: Decimal::from(10),
                size: Decimal::ONE,
                orig_size: Decimal::ONE,
                execution: PriceType::Limit,
                tif: example_core_entity::OrderTif::Gtc,
                trigger: Default::default(),
                trigger_px: None,
                group_relation: OrderGroupRelation::Standalone,
                is_position_tpsl: false,
                reduce_only: false,
                status: SpotOrderStatus::Pending,
                buy_quote_fee_reserve_limit: Decimal::ZERO,
                buy_quote_fee_reserve_consumed: Decimal::ZERO,
                cloid: None,
                created_at: lsn,
                updated_at: lsn,
                lsn,
            },
            OrderType::Limit { tif: example_core_entity::OrderTif::Gtc },
            OrderGroupRelation::Standalone,
            0,
            lsn,
            lsn,
        )
        .unwrap()
    }

    #[test]
    fn single_trigger_is_pending_without_balances() {
        let cmd = command(PlaceSpotOrderV3Cmd::Single(trigger(2)));
        let changed = PlaceSpotOrderV3UseCase.compute_state_changed(&cmd, &state(false)).unwrap();
        let PlaceSpotOrderV3AfterChanges::Trigger(after) = changed else {
            panic!("expected trigger branch");
        };
        assert_eq!(after.created_trigger_order.order_base().status, SpotOrderStatus::Pending);
        assert_eq!(
            after.created_trigger_order.order_base().group_relation,
            OrderGroupRelation::Standalone
        );
    }

    #[test]
    fn normal_tpsl_creates_parent_then_pending_children_without_child_balances() {
        let cmd = command(PlaceSpotOrderV3Cmd::NormalTpsl {
            parent: order(
                1,
                OrderSide::Buy,
                OrderType::Limit { tif: example_core_entity::OrderTif::Gtc },
            ),
            children: vec![trigger(2)],
        });
        let changed = PlaceSpotOrderV3UseCase.compute_state_changed(&cmd, &state(true)).unwrap();
        let PlaceSpotOrderV3AfterChanges::NormalTpsl { parent, children } = changed else {
            panic!("expected normal TP/SL branch");
        };
        assert!(!normal_is_rejected(&parent));
        assert_eq!(children.len(), 1);
        assert_eq!(
            children[0].created_trigger_order.order_base().group_relation,
            OrderGroupRelation::NormalTpslChild { parent_order_id: 1 }
        );
    }

    #[test]
    fn diff_projects_parent_child_and_balance_in_fixed_order() {
        let cmd = command(PlaceSpotOrderV3Cmd::NormalTpsl {
            parent: order(
                1,
                OrderSide::Buy,
                OrderType::Limit { tif: example_core_entity::OrderTif::Gtc },
            ),
            children: vec![trigger(2)],
        });
        let changes = PlaceSpotOrderV3UseCase.compute_state_diff(&cmd, state(true)).unwrap();
        assert_eq!(changes.created_orders.len(), 2);
        assert_eq!(changes.created_orders[0].order_base().order_id, 1);
        assert_eq!(changes.created_orders[1].order_base().order_id, 2);
        assert!(changes.updated_maker_orders.is_empty());
        assert!(changes.settled_trades.is_empty());
        assert_eq!(changes.to_replayable_events().unwrap().len(), 3);
    }

    #[test]
    fn diff_pairs_changed_only_maker_by_order_id() {
        let first_before = pending_order(10, OrderSide::Sell, 1);
        let second_before = pending_order(20, OrderSide::Sell, 1);
        let second_after = pending_order(20, OrderSide::Sell, 2);
        let created_taker_order = pending_order(1, OrderSide::Buy, 2);
        let state = PlaceSpotOrderV3State {
            maker_orders: vec![first_before, second_before.clone()],
            balances_by_order_id: HashMap::new(),
            trade_id_start: 1,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        };
        let changed = PlaceSpotOrderV3AfterChanges::Normal(
            PlaceNormalSpotOrderV3AfterChanges::PartiallyFilled {
                created_taker_order,
                updated_maker_orders_after: vec![second_after.clone()],
                created_settled_trades: Vec::new(),
                updated_balances_after: Vec::new(),
            },
        );

        let changes =
            <PlaceSpotOrderV3UseCase as StateDiff>::do_compute_state_diff(state, changed).unwrap();

        assert_eq!(changes.updated_maker_orders.len(), 1);
        assert_eq!(changes.updated_maker_orders[0].before, second_before);
        assert_eq!(changes.updated_maker_orders[0].after, second_after);
    }

    #[test]
    fn alo_crossing_rejection_commits_only_created_taker_order() {
        let cmd = command(PlaceSpotOrderV3Cmd::Single(order(
            1,
            OrderSide::Buy,
            OrderType::Limit { tif: example_core_entity::OrderTif::Alo },
        )));

        let changed =
            PlaceSpotOrderV3UseCase.compute_state_changed(&cmd, &rejection_state(true)).unwrap();
        let PlaceSpotOrderV3AfterChanges::Normal(PlaceNormalSpotOrderV3AfterChanges::Rejected {
            created_taker_order,
        }) = changed
        else {
            panic!("expected ALO rejection")
        };
        assert_eq!(created_taker_order.order_base().status, SpotOrderStatus::Rejected);

        let changes =
            PlaceSpotOrderV3UseCase.compute_state_diff(&cmd, rejection_state(true)).unwrap();
        assert_eq!(changes.created_orders.len(), 1);
        assert!(changes.updated_maker_orders.is_empty());
        assert!(changes.settled_trades.is_empty());
        assert!(changes.updated_balances.is_empty());
        assert_eq!(changes.to_replayable_events().unwrap().len(), 1);
    }

    #[test]
    fn ioc_zero_fill_rejection_does_not_commit_balance_metadata() {
        let cmd = command(PlaceSpotOrderV3Cmd::Single(order(
            1,
            OrderSide::Buy,
            OrderType::Limit { tif: example_core_entity::OrderTif::Ioc },
        )));
        let before = rejection_state(false);
        let authoritative_balance = before.balances_by_order_id[&1].quote.clone();

        let changes = PlaceSpotOrderV3UseCase.compute_state_diff(&cmd, before).unwrap();

        assert_eq!(changes.result, PlaceSpotOrderV3Result::Rejected);
        assert!(changes.updated_maker_orders.is_empty());
        assert!(changes.settled_trades.is_empty());
        assert!(changes.updated_balances.is_empty());
        assert_eq!(changes.to_replayable_events().unwrap().len(), 1);
        assert_eq!(authoritative_balance, binding().quote);
    }

    #[test]
    fn rejected_normal_tpsl_parent_creates_no_children_or_balance_updates() {
        let cmd = command(PlaceSpotOrderV3Cmd::NormalTpsl {
            parent: order(
                1,
                OrderSide::Buy,
                OrderType::Limit { tif: example_core_entity::OrderTif::Alo },
            ),
            children: vec![trigger(2)],
        });

        let changed =
            PlaceSpotOrderV3UseCase.compute_state_changed(&cmd, &rejection_state(true)).unwrap();
        let PlaceSpotOrderV3AfterChanges::NormalTpsl { parent, children } = changed else {
            panic!("expected normal TP/SL rejection")
        };
        assert!(matches!(parent, PlaceNormalSpotOrderV3AfterChanges::Rejected { .. }));
        assert!(children.is_empty());

        let changes =
            PlaceSpotOrderV3UseCase.compute_state_diff(&cmd, rejection_state(true)).unwrap();
        assert_eq!(changes.created_orders.len(), 1);
        assert!(changes.updated_balances.is_empty());
        assert_eq!(changes.to_replayable_events().unwrap().len(), 1);
    }
}
