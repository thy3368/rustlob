use std::collections::{HashMap, HashSet};

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::account::symbol::Symbol;
use example_core_entity::common::{OrderBase, OrderTrigger, PriceType, UserId};
use example_core_entity::spot::SpotTradeV3;
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, OrderGroupRelation, OrderSide, OrderTif,
    OrderTriggerRole, OrderType, SpotOrderStatus, SpotOrderV7,
};
use rust_decimal::{Decimal, RoundingStrategy};
use thiserror::Error;

use crate::use_case::trading::spot::v3::domain_service::spot_order_domain_service::{
    ActivateSpotOrderV7Error, MatchSpotOrderV3Input, MatchSpotOrderV3Outcome,
    SpotOrderDomainService,
};
use crate::use_case::trading::spot::v3::{SpotTradeDomainService, SpotTradeDomainServiceError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV3OrderCmd {
    pub party_id: u64,
    pub order_id: u64,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub price: Decimal,
    pub size: Decimal,
    pub order_type: OrderType,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderV3Cmd {
    Single(PlaceSpotOrderV3OrderCmd),
    NormalTpsl { parent: PlaceSpotOrderV3OrderCmd, children: Vec<PlaceSpotOrderV3OrderCmd> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV3Balances {
    pub base: BalanceV2,
    pub quote: BalanceV2,
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
pub enum PlaceSpotOrderV3Result {
    Pending,
    Resting,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV3AfterChanges {
    pub created_orders: Vec<SpotOrderV7>,
    pub maker_orders_after: Vec<SpotOrderV7>,
    pub settled_trades: Vec<SpotTradeV3>,
    pub balances_after: Vec<BalanceV2>,
    pub result: PlaceSpotOrderV3Result,
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
        for value in &self.created_orders {
            events.push(value.track_create_event()?);
        }
        for pair in &self.updated_maker_orders {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        for value in &self.settled_trades {
            events.push(value.track_create_event()?);
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
    #[error("authoritative balances are missing, duplicated, or mismatched")]
    InvalidBalances,
    #[error("trade id range overflows")]
    TradeIdOverflow,
    #[error(transparent)]
    Entity(#[from] EntityError),
    #[error(transparent)]
    Activation(#[from] ActivateSpotOrderV7Error),
    #[error(transparent)]
    Settlement(#[from] SpotTradeDomainServiceError),
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
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
        let mut order_ids: HashSet<u64> =
            command_orders(cmd).into_iter().map(|o| o.order_id).collect();
        for maker in &state.maker_orders {
            let value = maker.order_base();
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
        BalanceWorkspace::new(cmd, state)?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let timestamp = cmd.logical_ts();
        let specs = command_orders(cmd);
        let mut balance_workspace = BalanceWorkspace::new(cmd, state)?;
        let mut created_orders = Vec::with_capacity(specs.len());
        for (index, spec) in specs.iter().enumerate() {
            let relation = match cmd.payload() {
                PlaceSpotOrderV3Cmd::Single(_) => OrderGroupRelation::Standalone,
                PlaceSpotOrderV3Cmd::NormalTpsl { .. } if index == 0 => {
                    OrderGroupRelation::NormalTpslParent
                }
                PlaceSpotOrderV3Cmd::NormalTpsl { parent, .. } => {
                    OrderGroupRelation::NormalTpslChild { parent_order_id: parent.order_id }
                }
            };
            created_orders.push(build_order(spec, relation, state.taker_fee_bps, timestamp)?);
        }
        if specs[0].order_type.is_trigger() {
            return Ok(PlaceSpotOrderV3AfterChanges {
                created_orders,
                maker_orders_after: state.maker_orders.clone(),
                settled_trades: vec![],
                balances_after: balance_workspace.working,
                result: PlaceSpotOrderV3Result::Pending,
            });
        }

        let taker_balance = balance_workspace.held_index(specs[0].order_id, specs[0].side)?;
        let frozen_before_command = balance_workspace.originals[taker_balance].frozen();
        let (active, frozen) = SpotOrderDomainService::freeze(
            created_orders[0].clone(),
            balance_workspace.working[taker_balance].clone(),
            timestamp,
        )?;
        created_orders[0] = active.clone();
        balance_workspace.working[taker_balance] = frozen;
        let outcome = SpotOrderDomainService::match_order(
            active,
            state.maker_orders.clone(),
            MatchSpotOrderV3Input {
                trade_id_start: state.trade_id_start,
                maker_fee_bps: state.maker_fee_bps,
                taker_fee_bps: state.taker_fee_bps,
                timestamp,
            },
        )?;
        let (taker_after, mut makers_after, trades, result) = unpack_outcome(outcome);
        if makers_after.is_empty() && trades.is_empty() {
            makers_after = state.maker_orders.clone();
        }
        created_orders[0] = taker_after;

        let mut settled_trades = Vec::with_capacity(trades.len());
        for trade in trades {
            let (buyer_order_id, seller_order_id) = match trade.comon_trade.taker_side {
                OrderSide::Buy => {
                    (trade.comon_trade.taker_order_id, trade.comon_trade.maker_order_id)
                }
                OrderSide::Sell => {
                    (trade.comon_trade.maker_order_id, trade.comon_trade.taker_order_id)
                }
            };
            let (ibb, ibq) = balance_workspace.indices(buyer_order_id)?;
            let (isb, isq) = balance_workspace.indices(seller_order_id)?;
            let settled = SpotTradeDomainService::settlement(
                trade,
                balance_workspace.working[ibb].clone(),
                balance_workspace.working[ibq].clone(),
                balance_workspace.working[isb].clone(),
                balance_workspace.working[isq].clone(),
                timestamp,
            )?;
            balance_workspace.working[ibb] = settled.buyer_base_balance_after;
            balance_workspace.working[ibq] = settled.buyer_quote_balance_after;
            balance_workspace.working[isb] = settled.seller_base_balance_after;
            balance_workspace.working[isq] = settled.seller_quote_balance_after;
            settled_trades.push(settled.trade_after);
        }
        normalize_hold(
            &created_orders[0],
            &mut balance_workspace.working[taker_balance],
            frozen_before_command,
            timestamp,
        )?;
        Ok(PlaceSpotOrderV3AfterChanges {
            created_orders,
            maker_orders_after: makers_after,
            settled_trades,
            balances_after: balance_workspace.working,
            result,
        })
    }
}

impl StateDiff for PlaceSpotOrderV3UseCase {
    type StateDiff = PlaceSpotOrderV3Changes;
    fn do_compute_state_diff(
        state: Self::StateGiven,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        let updated_maker_orders = state
            .maker_orders
            .into_iter()
            .zip(changed.maker_orders_after)
            .filter(|(a, b)| a != b)
            .map(|(before, after)| UpdatedEntityPair { before, after })
            .collect();
        let mut updated_balances = Vec::new();
        for after in changed.balances_after {
            let before = find_bound_balance(&state.balances_by_order_id, &after)?;
            if before != after {
                updated_balances.push(UpdatedEntityPair { before, after });
            }
        }
        Ok(PlaceSpotOrderV3Changes {
            created_orders: changed.created_orders,
            updated_maker_orders,
            settled_trades: changed.settled_trades,
            updated_balances,
            result: changed.result,
        })
    }
}

fn command_orders(cmd: &LogEntry<PlaceSpotOrderV3Cmd>) -> Vec<&PlaceSpotOrderV3OrderCmd> {
    match cmd.payload() {
        PlaceSpotOrderV3Cmd::Single(value) => vec![value],
        PlaceSpotOrderV3Cmd::NormalTpsl { parent, children } => {
            let mut values = vec![parent];
            values.extend(children);
            values
        }
    }
}
fn parent_command(cmd: &LogEntry<PlaceSpotOrderV3Cmd>) -> &PlaceSpotOrderV3OrderCmd {
    match cmd.payload() {
        PlaceSpotOrderV3Cmd::Single(value) => value,
        PlaceSpotOrderV3Cmd::NormalTpsl { parent, .. } => parent,
    }
}
type BalanceIdentity = (u64, u64);

struct BalanceWorkspace {
    originals: Vec<BalanceV2>,
    working: Vec<BalanceV2>,
    indices_by_order_id: HashMap<u64, (usize, usize)>,
}

impl BalanceWorkspace {
    fn new(
        cmd: &LogEntry<PlaceSpotOrderV3Cmd>,
        state: &PlaceSpotOrderV3State,
    ) -> Result<Self, PlaceSpotOrderV3Error> {
        validate_all_bound_snapshots(&state.balances_by_order_id)?;
        let command_specs = command_orders(cmd);
        let mut ordered_orders = Vec::with_capacity(command_specs.len() + state.maker_orders.len());
        ordered_orders.extend(
            command_specs.into_iter().map(|order| (order.order_id, order.party_id, order.symbol)),
        );
        ordered_orders.extend(state.maker_orders.iter().map(|order| {
            let base = order.order_base();
            (base.order_id, base.owner.0, base.symbol)
        }));

        let mut originals = Vec::with_capacity(ordered_orders.len() * 2);
        let mut index_by_identity = HashMap::<BalanceIdentity, usize>::new();
        let mut indices_by_order_id = HashMap::with_capacity(ordered_orders.len());
        for (order_id, party_id, symbol) in ordered_orders {
            let binding = state
                .balances_by_order_id
                .get(&order_id)
                .ok_or(PlaceSpotOrderV3Error::InvalidBalances)?;
            validate_binding(binding, party_id, symbol)?;
            let base_index =
                insert_bound_balance(&mut originals, &mut index_by_identity, &binding.base)?;
            let quote_index =
                insert_bound_balance(&mut originals, &mut index_by_identity, &binding.quote)?;
            indices_by_order_id.insert(order_id, (base_index, quote_index));
        }
        Ok(Self { working: originals.clone(), originals, indices_by_order_id })
    }

    fn indices(&self, order_id: u64) -> Result<(usize, usize), PlaceSpotOrderV3Error> {
        self.indices_by_order_id
            .get(&order_id)
            .copied()
            .ok_or(PlaceSpotOrderV3Error::InvalidBalances)
    }

    fn held_index(&self, order_id: u64, side: OrderSide) -> Result<usize, PlaceSpotOrderV3Error> {
        let (base, quote) = self.indices(order_id)?;
        Ok(match side {
            OrderSide::Buy => quote,
            OrderSide::Sell => base,
        })
    }
}

fn validate_all_bound_snapshots(
    bindings: &HashMap<u64, SpotOrderV3Balances>,
) -> Result<(), PlaceSpotOrderV3Error> {
    let mut snapshots = HashMap::<BalanceIdentity, &BalanceV2>::new();
    for balance in bindings.values().flat_map(|binding| [&binding.base, &binding.quote]) {
        let identity = (balance.account_id(), balance.coin_idx());
        if snapshots.insert(identity, balance).is_some_and(|previous| previous != balance) {
            return Err(PlaceSpotOrderV3Error::InvalidBalances);
        }
    }
    Ok(())
}

fn validate_binding(
    binding: &SpotOrderV3Balances,
    party_id: u64,
    symbol: Symbol,
) -> Result<(), PlaceSpotOrderV3Error> {
    if binding.base.coin_idx() != u64::from(symbol.base.index())
        || binding.quote.coin_idx() != u64::from(symbol.quote.index())
        || binding.base.party_id() != party_id
        || binding.quote.party_id() != party_id
        || binding.base.account_id() != binding.quote.account_id()
    {
        return Err(PlaceSpotOrderV3Error::InvalidBalances);
    }
    Ok(())
}

fn insert_bound_balance(
    originals: &mut Vec<BalanceV2>,
    index_by_identity: &mut HashMap<BalanceIdentity, usize>,
    balance: &BalanceV2,
) -> Result<usize, PlaceSpotOrderV3Error> {
    let identity = (balance.account_id(), balance.coin_idx());
    if let Some(index) = index_by_identity.get(&identity).copied() {
        if originals[index] != *balance {
            return Err(PlaceSpotOrderV3Error::InvalidBalances);
        }
        return Ok(index);
    }
    let index = originals.len();
    originals.push(balance.clone());
    index_by_identity.insert(identity, index);
    Ok(index)
}

fn find_bound_balance(
    bindings: &HashMap<u64, SpotOrderV3Balances>,
    balance: &BalanceV2,
) -> Result<BalanceV2, PlaceSpotOrderV3Error> {
    let identity = (balance.account_id(), balance.coin_idx());
    bindings
        .values()
        .flat_map(|binding| [&binding.base, &binding.quote])
        .find(|candidate| (candidate.account_id(), candidate.coin_idx()) == identity)
        .cloned()
        .ok_or(PlaceSpotOrderV3Error::InvalidBalances)
}

fn build_order(
    cmd: &PlaceSpotOrderV3OrderCmd,
    relation: OrderGroupRelation,
    fee_bps: u64,
    timestamp: u64,
) -> Result<SpotOrderV7, PlaceSpotOrderV3Error> {
    let (execution, tif, trigger, trigger_px) = match cmd.order_type {
        OrderType::Limit { tif } => (PriceType::Limit, tif, OrderTrigger::None, None),
        OrderType::Trigger { is_market, trigger_price, tpsl } => (
            if is_market { PriceType::Market } else { PriceType::Limit },
            if is_market { OrderTif::Ioc } else { OrderTif::Gtc },
            match tpsl {
                OrderTriggerRole::TakeProfit => OrderTrigger::TakeProfit,
                OrderTriggerRole::StopLoss => OrderTrigger::StopLoss,
            },
            Some(Decimal::from(trigger_price)),
        ),
    };
    let fee = if cmd.side == OrderSide::Buy {
        cmd.price
            .checked_mul(cmd.size)
            .and_then(|v| v.checked_mul(Decimal::from(fee_bps)))
            .and_then(|v| v.checked_div(Decimal::from(10_000u64)))
            .map(|v| {
                v.round_dp_with_strategy(
                    cmd.symbol.quote.scale(),
                    RoundingStrategy::ToPositiveInfinity,
                )
            })
            .ok_or(PlaceSpotOrderV3Error::InvalidAmount)?
    } else {
        Decimal::ZERO
    };
    Ok(SpotOrderV7::from_record(OrderBase {
        owner: UserId(cmd.party_id),
        order_id: cmd.order_id,
        symbol: cmd.symbol,
        side: cmd.side,
        limit_px: cmd.price,
        size: cmd.size,
        orig_size: cmd.size,
        execution,
        tif,
        trigger,
        trigger_px,
        group_relation: relation,
        is_position_tpsl: false,
        reduce_only: false,
        status: SpotOrderStatus::Pending,
        buy_quote_fee_reserve_limit: fee,
        buy_quote_fee_reserve_consumed: Decimal::ZERO,
        cloid: cmd.cloid.clone(),
        created_at: timestamp,
        updated_at: timestamp,
        lsn: 0,
    })?)
}

fn unpack_outcome(
    value: MatchSpotOrderV3Outcome,
) -> (SpotOrderV7, Vec<SpotOrderV7>, Vec<SpotTradeV3>, PlaceSpotOrderV3Result) {
    match value {
        MatchSpotOrderV3Outcome::Resting { taker_order_after } => {
            (taker_order_after, vec![], vec![], PlaceSpotOrderV3Result::Resting)
        }
        MatchSpotOrderV3Outcome::PartiallyFilled {
            taker_order_after,
            maker_orders_after,
            created_trades,
        } => (
            taker_order_after,
            maker_orders_after,
            created_trades,
            PlaceSpotOrderV3Result::PartiallyFilled,
        ),
        MatchSpotOrderV3Outcome::Filled {
            filled_taker_order_after,
            maker_orders_after,
            created_trades,
            ..
        } => (
            filled_taker_order_after,
            maker_orders_after,
            created_trades,
            PlaceSpotOrderV3Result::Filled,
        ),
        MatchSpotOrderV3Outcome::CanceledAfterPartialFill {
            canceled_taker_order_after,
            maker_orders_after,
            created_trades,
            ..
        } => (
            canceled_taker_order_after,
            maker_orders_after,
            created_trades,
            PlaceSpotOrderV3Result::Canceled,
        ),
        MatchSpotOrderV3Outcome::Rejected { rejected_taker_order_after, .. } => {
            (rejected_taker_order_after, vec![], vec![], PlaceSpotOrderV3Result::Rejected)
        }
    }
}

fn normalize_hold(
    order: &SpotOrderV7,
    balance: &mut BalanceV2,
    frozen_before_command: Decimal,
    timestamp: u64,
) -> Result<(), PlaceSpotOrderV3Error> {
    let base = order.order_base();
    let required =
        if matches!(base.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled) {
            match base.side {
                OrderSide::Buy => base
                    .limit_px
                    .checked_mul(base.size)
                    .and_then(|v| v.checked_add(base.buy_quote_fee_reserve_limit))
                    .and_then(|v| v.checked_sub(base.buy_quote_fee_reserve_consumed))
                    .ok_or(PlaceSpotOrderV3Error::InvalidAmount)?,
                OrderSide::Sell => base.size,
            }
        } else {
            Decimal::ZERO
        };
    let target =
        frozen_before_command.checked_add(required).ok_or(PlaceSpotOrderV3Error::InvalidAmount)?;
    let current = balance.frozen();
    if current > target {
        balance.apply(BalanceV2Event::UnfreezeAndPay {
            unfreeze_amount: current - target,
            payment_amount: Decimal::ZERO,
            timestamp,
        })?;
    } else if current < target {
        balance.apply(BalanceV2Event::Freeze { amount: target - current, timestamp })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use common_entity::LogEntrySequencer;
    use example_core_entity::account::coin::Coin;

    use super::*;

    const PARTY_ID: u64 = 11;
    const ACCOUNT_ID: u64 = 22;

    fn symbol() -> Symbol {
        Symbol::new(1, Coin::BTC, Coin::USDT)
    }

    fn order(order_id: u64, side: OrderSide, order_type: OrderType) -> PlaceSpotOrderV3OrderCmd {
        PlaceSpotOrderV3OrderCmd {
            party_id: PARTY_ID,
            order_id,
            symbol: symbol(),
            side,
            price: Decimal::from(10),
            size: Decimal::ONE,
            order_type,
            cloid: None,
        }
    }

    fn binding(party_id: u64, account_id: u64) -> SpotOrderV3Balances {
        SpotOrderV3Balances {
            base: BalanceV2::new(party_id, account_id, u64::from(Coin::BTC.index()), 100, 0, 1, 1),
            quote: BalanceV2::new(
                party_id,
                account_id,
                u64::from(Coin::USDT.index()),
                100,
                0,
                1,
                1,
            ),
        }
    }

    fn command(payload: PlaceSpotOrderV3Cmd) -> LogEntry<PlaceSpotOrderV3Cmd> {
        LogEntrySequencer::from_metadata(1, 100, 1, 1, payload)
    }

    fn state(
        bindings: impl IntoIterator<Item = (u64, SpotOrderV3Balances)>,
    ) -> PlaceSpotOrderV3State {
        PlaceSpotOrderV3State {
            maker_orders: vec![],
            balances_by_order_id: bindings.into_iter().collect(),
            trade_id_start: 1,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        }
    }

    #[test]
    fn accepts_complete_single_and_normal_tpsl_bindings() {
        let use_case = PlaceSpotOrderV3UseCase;
        let single = command(PlaceSpotOrderV3Cmd::Single(order(
            1,
            OrderSide::Buy,
            OrderType::Limit { tif: OrderTif::Gtc },
        )));
        assert_eq!(
            use_case.validate_state_given(&single, &state([(1, binding(PARTY_ID, 22))])),
            Ok(())
        );

        let grouped = command(PlaceSpotOrderV3Cmd::NormalTpsl {
            parent: order(1, OrderSide::Buy, OrderType::Limit { tif: OrderTif::Gtc }),
            children: vec![order(
                2,
                OrderSide::Sell,
                OrderType::Trigger {
                    is_market: true,
                    trigger_price: 11,
                    tpsl: OrderTriggerRole::TakeProfit,
                },
            )],
        });
        assert_eq!(
            use_case.validate_state_given(
                &grouped,
                &state([(1, binding(PARTY_ID, 22)), (2, binding(PARTY_ID, 22))]),
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_missing_child_binding() {
        let cmd = command(PlaceSpotOrderV3Cmd::NormalTpsl {
            parent: order(1, OrderSide::Buy, OrderType::Limit { tif: OrderTif::Gtc }),
            children: vec![order(
                2,
                OrderSide::Sell,
                OrderType::Trigger {
                    is_market: true,
                    trigger_price: 11,
                    tpsl: OrderTriggerRole::StopLoss,
                },
            )],
        });
        assert_eq!(
            PlaceSpotOrderV3UseCase
                .validate_state_given(&cmd, &state([(1, binding(PARTY_ID, 22))])),
            Err(PlaceSpotOrderV3Error::InvalidBalances)
        );
    }

    #[test]
    fn rejects_coin_party_and_account_mismatches() {
        let cmd = command(PlaceSpotOrderV3Cmd::Single(order(
            1,
            OrderSide::Buy,
            OrderType::Limit { tif: OrderTif::Gtc },
        )));
        let mut reversed = binding(PARTY_ID, ACCOUNT_ID);
        std::mem::swap(&mut reversed.base, &mut reversed.quote);
        let mut wrong_account = binding(PARTY_ID, ACCOUNT_ID);
        wrong_account.quote =
            BalanceV2::new(PARTY_ID, ACCOUNT_ID + 1, u64::from(Coin::USDT.index()), 100, 0, 1, 1);
        for invalid in [reversed, binding(PARTY_ID + 1, ACCOUNT_ID), wrong_account] {
            assert_eq!(
                PlaceSpotOrderV3UseCase.validate_state_given(&cmd, &state([(1, invalid)])),
                Err(PlaceSpotOrderV3Error::InvalidBalances)
            );
        }
    }

    #[test]
    fn deduplicates_consistent_snapshots_and_rejects_conflicts() {
        let cmd = command(PlaceSpotOrderV3Cmd::NormalTpsl {
            parent: order(1, OrderSide::Buy, OrderType::Limit { tif: OrderTif::Gtc }),
            children: vec![order(
                2,
                OrderSide::Sell,
                OrderType::Trigger {
                    is_market: false,
                    trigger_price: 11,
                    tpsl: OrderTriggerRole::TakeProfit,
                },
            )],
        });
        let workspace = BalanceWorkspace::new(
            &cmd,
            &state([(1, binding(PARTY_ID, 22)), (2, binding(PARTY_ID, 22))]),
        )
        .unwrap();
        assert_eq!(workspace.originals.len(), 2);

        let mut conflicting = binding(PARTY_ID, 22);
        conflicting.base = BalanceV2::new(PARTY_ID, 22, u64::from(Coin::BTC.index()), 99, 0, 1, 1);
        assert!(matches!(
            BalanceWorkspace::new(&cmd, &state([(1, binding(PARTY_ID, 22)), (2, conflicting)]),),
            Err(PlaceSpotOrderV3Error::InvalidBalances)
        ));
    }
}
