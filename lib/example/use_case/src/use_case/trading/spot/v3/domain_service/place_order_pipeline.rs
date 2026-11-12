use std::collections::HashMap;

use common_entity::EntityError;
use example_core_entity::account::symbol::Symbol;
use example_core_entity::common::{OrderBase, PriceType, UserId};
use example_core_entity::spot::SpotTradeV3;
use example_core_entity::{
    BalanceV2, BalanceV2Error, OrderGroupRelation, OrderSide, OrderTif, SpotOrderStatus,
    SpotOrderV7,
};
use rust_decimal::Decimal;
use thiserror::Error;

use super::atom_spot_order_domain_service::{
    ActivateSpotOrderV7Error, MatchSpotOrderV3Input, MatchSpotOrderV3Outcome,
    ReconcileSpotOrderHoldError, SpotOrderDomainService,
};
use super::atom_spot_trade_domain_service::{SpotTradeDomainService, SpotTradeDomainServiceError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV3OrderCmd {
    pub party_id: u64,
    pub order_id: u64,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub price: Decimal,
    pub size: Decimal,
    pub order_type: example_core_entity::OrderType,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV3Balances {
    pub base: BalanceV2,
    pub quote: BalanceV2,
}

#[derive(Debug, Clone)]
pub struct PlaceNormalSpotOrderV2Input {
    pub order: PlaceSpotOrderV3OrderCmd,
    pub relation: OrderGroupRelation,
    pub maker_orders: Vec<SpotOrderV7>,
    pub balances_by_order_id: HashMap<u64, SpotOrderV3Balances>,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
    pub lsn: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct PlaceTriggerSpotOrderV2Input {
    pub order: PlaceSpotOrderV3OrderCmd,
    pub relation: OrderGroupRelation,
    pub taker_fee_bps: u64,
    pub lsn: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceNormalSpotOrderV3AfterChanges {
    Resting {
        created_taker_order: SpotOrderV7,
        updated_balances_after: Vec<BalanceV2>,
    },
    PartiallyFilled {
        created_taker_order: SpotOrderV7,
        updated_maker_orders_after: Vec<SpotOrderV7>,
        created_settled_trades: Vec<SpotTradeV3>,
        updated_balances_after: Vec<BalanceV2>,
    },
    Filled {
        created_taker_order: SpotOrderV7,
        updated_maker_orders_after: Vec<SpotOrderV7>,
        created_settled_trades: Vec<SpotTradeV3>,
        updated_balances_after: Vec<BalanceV2>,
    },
    CanceledAfterPartialFill {
        created_taker_order: SpotOrderV7,
        updated_maker_orders_after: Vec<SpotOrderV7>,
        created_settled_trades: Vec<SpotTradeV3>,
        updated_balances_after: Vec<BalanceV2>,
    },
    Rejected {
        created_taker_order: SpotOrderV7,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceTriggerSpotOrderV3AfterChanges {
    pub created_trigger_order: SpotOrderV7,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceDomainServiceError {
    #[error("normal order must not be a trigger order")]
    ExpectedNormalOrder,
    #[error("trigger order is required")]
    ExpectedTriggerOrder,
    #[error("authoritative balances are missing, duplicated, or mismatched")]
    InvalidBalances,
    #[error(transparent)]
    Entity(#[from] EntityError),
    #[error(transparent)]
    Activation(#[from] ActivateSpotOrderV7Error),
    #[error(transparent)]
    Settlement(#[from] SpotTradeDomainServiceError),
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Hold(#[from] ReconcileSpotOrderHoldError),
}

pub struct PlaceOrderPipeline;

impl PlaceOrderPipeline {
    pub fn place_normal_order_v2(
        input: PlaceNormalSpotOrderV2Input,
    ) -> Result<PlaceNormalSpotOrderV3AfterChanges, PlaceDomainServiceError> {
        if input.order.order_type.is_trigger() {
            return Err(PlaceDomainServiceError::ExpectedNormalOrder);
        }
        let mut balances =
            BalanceWorkspace::new(&input.order, &input.maker_orders, &input.balances_by_order_id)?;
        let order = build_order(
            &input.order,
            input.relation,
            input.taker_fee_bps,
            input.lsn,
            input.timestamp,
        )?;
        let held_index = balances.held_index(input.order.order_id, input.order.side)?;
        let frozen_before_command = balances.originals[held_index].frozen();
        let (active, frozen) = SpotOrderDomainService::freeze(
            order,
            balances.working[held_index].clone(),
            input.lsn,
            input.timestamp,
        )?;
        balances.working[held_index] = frozen;
        let outcome = SpotOrderDomainService::match_order(
            active,
            input.maker_orders.clone(),
            MatchSpotOrderV3Input {
                lsn: input.lsn,
                trade_id_start: input.trade_id_start,
                maker_fee_bps: input.maker_fee_bps,
                taker_fee_bps: input.taker_fee_bps,
                timestamp: input.timestamp,
            },
        )?;
        let (taker_after, makers_after, trades, kind) = unpack_outcome(outcome);
        let mut makers_after_by_id = makers_after
            .into_iter()
            .map(|maker| (maker.order_base().order_id, maker))
            .collect::<HashMap<_, _>>();
        let updated_maker_orders_after = input
            .maker_orders
            .iter()
            .filter_map(|before| {
                makers_after_by_id
                    .remove(&before.order_base().order_id)
                    .filter(|after| after != before)
            })
            .collect();
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
            let (buyer_base, buyer_quote) = balances.indices(buyer_order_id)?;
            let (seller_base, seller_quote) = balances.indices(seller_order_id)?;
            let settled = SpotTradeDomainService::settlement(
                trade,
                balances.working[buyer_base].clone(),
                balances.working[buyer_quote].clone(),
                balances.working[seller_base].clone(),
                balances.working[seller_quote].clone(),
                input.timestamp,
            )?;
            balances.working[buyer_base] = settled.buyer_base_balance_after;
            balances.working[buyer_quote] = settled.buyer_quote_balance_after;
            balances.working[seller_base] = settled.seller_base_balance_after;
            balances.working[seller_quote] = settled.seller_quote_balance_after;
            settled_trades.push(settled.trade_after);
        }
        balances.working[held_index] = SpotOrderDomainService::reconcile_taker_hold(
            &taker_after,
            balances.working[held_index].clone(),
            frozen_before_command,
            input.timestamp,
        )?;
        let updated_balances_after = balances.into_updated_after();
        Ok(match kind {
            NormalOutcomeKind::Resting => PlaceNormalSpotOrderV3AfterChanges::Resting {
                created_taker_order: taker_after,
                updated_balances_after,
            },
            NormalOutcomeKind::PartiallyFilled => {
                PlaceNormalSpotOrderV3AfterChanges::PartiallyFilled {
                    created_taker_order: taker_after,
                    updated_maker_orders_after,
                    created_settled_trades: settled_trades,
                    updated_balances_after,
                }
            }
            NormalOutcomeKind::Filled => PlaceNormalSpotOrderV3AfterChanges::Filled {
                created_taker_order: taker_after,
                updated_maker_orders_after,
                created_settled_trades: settled_trades,
                updated_balances_after,
            },
            NormalOutcomeKind::Canceled => {
                PlaceNormalSpotOrderV3AfterChanges::CanceledAfterPartialFill {
                    created_taker_order: taker_after,
                    updated_maker_orders_after,
                    created_settled_trades: settled_trades,
                    updated_balances_after,
                }
            }
            NormalOutcomeKind::Rejected => {
                PlaceNormalSpotOrderV3AfterChanges::Rejected { created_taker_order: taker_after }
            }
        })
    }

    pub fn place_trigger_order_v2(
        input: PlaceTriggerSpotOrderV2Input,
    ) -> Result<PlaceTriggerSpotOrderV3AfterChanges, PlaceDomainServiceError> {
        if !input.order.order_type.is_trigger() {
            return Err(PlaceDomainServiceError::ExpectedTriggerOrder);
        }
        Ok(PlaceTriggerSpotOrderV3AfterChanges {
            created_trigger_order: build_order(
                &input.order,
                input.relation,
                input.taker_fee_bps,
                input.lsn,
                input.timestamp,
            )?,
        })
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
        taker: &PlaceSpotOrderV3OrderCmd,
        makers: &[SpotOrderV7],
        bindings: &HashMap<u64, SpotOrderV3Balances>,
    ) -> Result<Self, PlaceDomainServiceError> {
        validate_all_bound_snapshots(bindings)?;
        let mut orders = Vec::with_capacity(makers.len() + 1);
        orders.push((taker.order_id, taker.party_id, taker.symbol));
        orders.extend(makers.iter().map(|maker| {
            let base = maker.order_base();
            (base.order_id, base.owner.0, base.symbol)
        }));
        let mut originals = Vec::with_capacity(orders.len() * 2);
        let mut by_identity = HashMap::new();
        let mut indices_by_order_id = HashMap::with_capacity(orders.len());
        for (order_id, party_id, symbol) in orders {
            let binding =
                bindings.get(&order_id).ok_or(PlaceDomainServiceError::InvalidBalances)?;
            if binding.base.coin_idx() != u64::from(symbol.base.index())
                || binding.quote.coin_idx() != u64::from(symbol.quote.index())
                || binding.base.party_id() != party_id
                || binding.quote.party_id() != party_id
                || binding.base.account_id() != binding.quote.account_id()
            {
                return Err(PlaceDomainServiceError::InvalidBalances);
            }
            let base = insert_balance(&mut originals, &mut by_identity, &binding.base)?;
            let quote = insert_balance(&mut originals, &mut by_identity, &binding.quote)?;
            indices_by_order_id.insert(order_id, (base, quote));
        }
        Ok(Self { working: originals.clone(), originals, indices_by_order_id })
    }

    fn indices(&self, order_id: u64) -> Result<(usize, usize), PlaceDomainServiceError> {
        self.indices_by_order_id
            .get(&order_id)
            .copied()
            .ok_or(PlaceDomainServiceError::InvalidBalances)
    }

    fn held_index(&self, order_id: u64, side: OrderSide) -> Result<usize, PlaceDomainServiceError> {
        let (base, quote) = self.indices(order_id)?;
        Ok(match side {
            OrderSide::Buy => quote,
            OrderSide::Sell => base,
        })
    }

    fn into_updated_after(self) -> Vec<BalanceV2> {
        self.originals
            .into_iter()
            .zip(self.working)
            .filter_map(|(before, after)| (before != after).then_some(after))
            .collect()
    }
}

fn validate_all_bound_snapshots(
    bindings: &HashMap<u64, SpotOrderV3Balances>,
) -> Result<(), PlaceDomainServiceError> {
    let mut snapshots = HashMap::<BalanceIdentity, &BalanceV2>::new();
    for balance in bindings.values().flat_map(|binding| [&binding.base, &binding.quote]) {
        let identity = (balance.account_id(), balance.coin_idx());
        if snapshots.insert(identity, balance).is_some_and(|previous| previous != balance) {
            return Err(PlaceDomainServiceError::InvalidBalances);
        }
    }
    Ok(())
}

fn insert_balance(
    balances: &mut Vec<BalanceV2>,
    indices: &mut HashMap<BalanceIdentity, usize>,
    balance: &BalanceV2,
) -> Result<usize, PlaceDomainServiceError> {
    let identity = (balance.account_id(), balance.coin_idx());
    if let Some(index) = indices.get(&identity).copied() {
        if balances[index] != *balance {
            return Err(PlaceDomainServiceError::InvalidBalances);
        }
        return Ok(index);
    }
    let index = balances.len();
    balances.push(balance.clone());
    indices.insert(identity, index);
    Ok(index)
}

fn build_order(
    cmd: &PlaceSpotOrderV3OrderCmd,
    relation: OrderGroupRelation,
    fee_bps: u64,
    lsn: u64,
    timestamp: u64,
) -> Result<SpotOrderV7, EntityError> {
    Ok(SpotOrderV7::pending(
        OrderBase {
            owner: UserId(cmd.party_id),
            order_id: cmd.order_id,
            symbol: cmd.symbol,
            side: cmd.side,
            limit_px: cmd.price,
            size: cmd.size,
            orig_size: cmd.size,
            execution: PriceType::Limit,
            tif: OrderTif::Gtc,
            trigger: Default::default(),
            trigger_px: None,
            group_relation: OrderGroupRelation::Standalone,
            is_position_tpsl: false,
            reduce_only: false,
            status: SpotOrderStatus::Pending,
            buy_quote_fee_reserve_limit: Decimal::ZERO,
            buy_quote_fee_reserve_consumed: Decimal::ZERO,
            cloid: cmd.cloid.clone(),
            created_at: timestamp,
            updated_at: timestamp,
            lsn,
        },
        cmd.order_type,
        relation,
        fee_bps,
        timestamp,
        lsn,
    )?)
}

enum NormalOutcomeKind {
    Resting,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

fn unpack_outcome(
    value: MatchSpotOrderV3Outcome,
) -> (SpotOrderV7, Vec<SpotOrderV7>, Vec<SpotTradeV3>, NormalOutcomeKind) {
    match value {
        MatchSpotOrderV3Outcome::Resting { taker_order_after } => {
            (taker_order_after, vec![], vec![], NormalOutcomeKind::Resting)
        }
        MatchSpotOrderV3Outcome::PartiallyFilled {
            taker_order_after,
            maker_orders_after,
            created_trades,
        } => (
            taker_order_after,
            maker_orders_after,
            created_trades,
            NormalOutcomeKind::PartiallyFilled,
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
            NormalOutcomeKind::Filled,
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
            NormalOutcomeKind::Canceled,
        ),
        MatchSpotOrderV3Outcome::Rejected { rejected_taker_order_after, .. } => {
            (rejected_taker_order_after, vec![], vec![], NormalOutcomeKind::Rejected)
        }
    }
}
