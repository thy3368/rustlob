use std::collections::{HashMap, HashSet};

use common_entity::{
    Entity, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff, UpdatedEntityPair,
    UseCase,
};
use example_core_entity::{SpotOrderHoldAsset, SpotTradeFeeRole};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use spot_entity::spot_order_v2::{SpotOrderV2, SpotOrderV2BehaviorError, SpotOrderV2MatchError};
use thiserror::Error;

use crate::common_entity::account::balance_ledger_v3::{
    BalanceLedgerV3, BalanceLedgerV3Error, LedgerOperation,
};
use crate::common_entity::{BalanceV2, OrderSide, OrderTif, SpotOrderStatus, spot as spot_entity};
use crate::support::{concat2, concat3, concat4};
use crate::{MatchSpotOrderV2Input, SpotTrade};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchSpotOrderV3Cmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: u64,
}

/// 撮合后对 taker 的五种业务结论。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV3AfterChanges {
    /// 没有成交，订单继续留在订单簿；本次没有其它业务副作用。
    Resting,
    /// 发生成交但仍有剩余数量，订单继续可撮合。
    PartiallyFilled {
        taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        balances_after: Vec<BalanceV2>,
        created_trades: Vec<SpotTrade>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
    /// 成交数量达到订单总数量，订单终止为 Filled。
    Filled {
        filled_taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        balances_after: Vec<BalanceV2>,
        created_trades: Vec<SpotTrade>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
    /// IOC 部分成交后，剩余数量被取消并释放冻结。
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        balances_after: Vec<BalanceV2>,
        created_trades: Vec<SpotTrade>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
    /// ALO crossing maker 或 IOC 无流动性导致拒单。
    Rejected {
        rejected_taker_order_after: SpotOrderV2,
        balances_after: Vec<BalanceV2>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
}

impl MatchSpotOrderV3AfterChanges {
    pub fn created_trades(&self) -> &[SpotTrade] {
        match self {
            Self::Resting | Self::Rejected { .. } => &[],
            Self::PartiallyFilled { created_trades, .. }
            | Self::Filled { created_trades, .. }
            | Self::CanceledAfterPartialFill { created_trades, .. } => created_trades,
        }
    }
}

/// 撮合后的 before/after 业务变化。
///
/// 更新实体只通过 `UpdatedEntityPair` 表达 authoritative before/after；成交和 ledger
/// 则直接表达本次撮合新产生的事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV3Changes {
    /// 没有状态变化，也没有新增业务事实。
    Resting,
    PartiallyFilled {
        updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        updated_balances: Vec<UpdatedEntityPair<BalanceV2>>,
        created_trades: Vec<SpotTrade>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
    Filled {
        filled_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        updated_balances: Vec<UpdatedEntityPair<BalanceV2>>,
        created_trades: Vec<SpotTrade>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
    CanceledAfterPartialFill {
        canceled_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        updated_balances: Vec<UpdatedEntityPair<BalanceV2>>,
        created_trades: Vec<SpotTrade>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
    Rejected {
        rejected_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_balances: Vec<UpdatedEntityPair<BalanceV2>>,
        created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderV3Error {
    #[error("party id must not be empty")]
    InvalidPartyId,
    #[error("order oid must be greater than zero")]
    InvalidOrderId,
    #[error("taker order id does not match on_command")]
    TakerOrderIdMismatch,
    #[error("taker account id does not match on_command")]
    TakerAccountIdMismatch,
    #[error("taker asset does not match on_command")]
    TakerAssetMismatch,
    #[error("fee account id must not be empty")]
    InvalidFeeAccountId,
    #[error("trade count does not match maker order count")]
    TradeOrderCountMismatch,
    #[error("balance not found")]
    BalanceNotFound,
    #[error("available balance is insufficient")]
    InsufficientAvailableBalance,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("arithmetic overflow while computing spot order v3 family")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderMatch(#[from] SpotOrderV2MatchError),
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    BalanceLedger(#[from] BalanceLedgerV3Error),
}

impl MatchSpotOrderV3Changes {
    pub(crate) fn to_replayable_events_without_taker_order_or_balances(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Resting => Ok(Vec::new()),
            Self::PartiallyFilled {
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
                ..
            }
            | Self::Filled {
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
                ..
            }
            | Self::CanceledAfterPartialFill {
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
                ..
            } => replay_events_without_balances(
                None,
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
            ),
            Self::Rejected { created_balance_ledger_entries, .. } => {
                replay_events_without_balances(None, &[], &[], created_balance_ledger_entries)
            }
        }
    }

    pub(crate) fn to_replayable_events_without_balances(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Resting => Ok(Vec::new()),
            Self::PartiallyFilled {
                updated_taker_order,
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
                ..
            }
            | Self::Filled {
                filled_taker_order: updated_taker_order,
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
                ..
            }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order,
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
                ..
            } => replay_events_without_balances(
                Some(updated_taker_order),
                updated_maker_orders,
                created_trades,
                created_balance_ledger_entries,
            ),
            Self::Rejected {
                rejected_taker_order: updated_taker_order,
                created_balance_ledger_entries,
                ..
            } => replay_events_without_balances(
                Some(updated_taker_order),
                &[],
                &[],
                created_balance_ledger_entries,
            ),
        }
    }
    pub(crate) fn to_replayable_events_without_taker_order(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Resting => Ok(Vec::new()),
            Self::PartiallyFilled {
                updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            }
            | Self::Filled {
                filled_taker_order: updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            } => replay_events_without_taker_order(
                updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            ),
            Self::Rejected {
                rejected_taker_order: updated_taker_order,
                updated_balances,
                created_balance_ledger_entries,
            } => replay_events_without_taker_order(
                updated_taker_order,
                &[],
                updated_balances,
                &[],
                created_balance_ledger_entries,
            ),
        }
    }
}

impl ReplayableChanges for MatchSpotOrderV3Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Resting => Ok(Vec::with_capacity(0)),
            Self::PartiallyFilled {
                updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            }
            | Self::Filled {
                filled_taker_order: updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            } => replay_events(
                Some(updated_taker_order),
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_balance_ledger_entries,
            ),
            Self::Rejected {
                rejected_taker_order: updated_taker_order,
                updated_balances,
                created_balance_ledger_entries,
            } => replay_events(
                Some(updated_taker_order),
                &[],
                updated_balances,
                &[],
                created_balance_ledger_entries,
            ),
        }
    }
}

fn replay_events_without_taker_order(
    _updated_taker_order: &UpdatedEntityPair<SpotOrderV2>,
    updated_maker_orders: &[UpdatedEntityPair<SpotOrderV2>],
    updated_balances: &[UpdatedEntityPair<BalanceV2>],
    created_trades: &[SpotTrade],
    created_balance_ledger_entries: &[BalanceLedgerV3],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let events = replay_events(
        None,
        updated_maker_orders,
        updated_balances,
        created_trades,
        created_balance_ledger_entries,
    )?;
    Ok(events)
}

fn replay_events(
    updated_taker_order: Option<&UpdatedEntityPair<SpotOrderV2>>,
    updated_maker_orders: &[UpdatedEntityPair<SpotOrderV2>],
    updated_balances: &[UpdatedEntityPair<BalanceV2>],
    created_trades: &[SpotTrade],
    created_balance_ledger_entries: &[BalanceLedgerV3],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let event_capacity = 1_usize
        .saturating_add(created_trades.len())
        .saturating_add(updated_maker_orders.len())
        .saturating_add(created_balance_ledger_entries.len().saturating_mul(2));
    let mut events = Vec::with_capacity(event_capacity);
    if let Some(updated_taker_order) = updated_taker_order {
        events
            .push(updated_taker_order.after.track_update_event_from(&updated_taker_order.before)?);
    }
    for trade in created_trades {
        events.push(trade.track_create_event()?);
    }
    for maker in updated_maker_orders {
        events.push(maker.after.track_update_event_from(&maker.before)?);
    }
    events.extend(balance_replay_events_from_ledger_entries(
        updated_balances,
        created_balance_ledger_entries,
    )?);
    for entry in created_balance_ledger_entries {
        events.push(entry.track_create_event()?);
    }
    Ok(events)
}

pub(crate) fn replay_events_without_balances(
    updated_taker_order: Option<&UpdatedEntityPair<SpotOrderV2>>,
    updated_maker_orders: &[UpdatedEntityPair<SpotOrderV2>],
    created_trades: &[SpotTrade],
    created_balance_ledger_entries: &[BalanceLedgerV3],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut events = Vec::with_capacity(
        updated_taker_order.is_some() as usize
            + updated_maker_orders.len()
            + created_trades.len()
            + created_balance_ledger_entries.len(),
    );
    if let Some(taker) = updated_taker_order {
        events.push(taker.after.track_update_event_from(&taker.before)?);
    }
    for trade in created_trades {
        events.push(trade.track_create_event()?);
    }
    for maker in updated_maker_orders {
        events.push(maker.after.track_update_event_from(&maker.before)?);
    }
    for entry in created_balance_ledger_entries {
        events.push(entry.track_create_event()?);
    }
    Ok(events)
}

struct MatchedTradeEffects {
    created_trades: Vec<SpotTrade>,
    created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    total_taker_fill: u64,
}

struct MatchedTradeSettlementContext<'a> {
    base_asset_id: &'a str,
    quote_asset_id: &'a str,
    fee_account_id: &'a str,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
    balance_book: &'a mut BalanceMap,
    ledger_entries: &'a mut Vec<BalanceLedgerV3>,
    timestamp: u64,
}

fn settle_matched_trades(
    taker_order: &mut SpotOrderV2,
    maker_orders: &mut [SpotOrderV2],
    trades: impl IntoIterator<Item = SpotTrade>,
    context: MatchedTradeSettlementContext<'_>,
) -> Result<MatchedTradeEffects, MatchSpotOrderV3Error> {
    let MatchedTradeSettlementContext {
        base_asset_id,
        quote_asset_id,
        fee_account_id,
        maker_fee_bps,
        taker_fee_bps,
        balance_book,
        ledger_entries,
        timestamp,
    } = context;
    let mut created_trades = Vec::new();
    let mut total_taker_fill = 0_u64;

    for (index, mut trade) in trades.into_iter().enumerate() {
        let _trade_notional =
            trade.notional_quote().ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;
        total_taker_fill = total_taker_fill
            .checked_add(trade.size)
            .ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;

        let Some(maker_order) = maker_orders.get_mut(index) else {
            return Err(MatchSpotOrderV3Error::TradeOrderCountMismatch);
        };
        let taker_fee = taker_order.fee_consume_requirement_for_trade(
            trade.size,
            trade.price,
            SpotTradeFeeRole::Taker,
            maker_fee_bps,
            taker_fee_bps,
        )?;
        let maker_fee = maker_order.fee_consume_requirement_for_trade(
            trade.size,
            trade.price,
            SpotTradeFeeRole::Maker,
            maker_fee_bps,
            taker_fee_bps,
        )?;
        debug_assert_eq!(taker_fee.asset, SpotOrderHoldAsset::Quote);
        debug_assert_eq!(maker_fee.asset, SpotOrderHoldAsset::Quote);
        trade.taker_fee = taker_fee.amount;
        trade.maker_fee = maker_fee.amount;

        let settlement_id = concat2("spot-settlement:", trade.trade_id.as_str());
        apply_trade_balance_effects(
            &trade,
            TradeBalanceEffectsContext {
                settlement_id: &settlement_id,
                base_asset_id,
                quote_asset_id,
                fee_account_id,
                balance_book,
                ledger_entries,
                timestamp,
            },
        )?;
        created_trades.push(trade);
    }

    Ok(MatchedTradeEffects {
        created_trades,
        created_balance_ledger_entries: std::mem::take(ledger_entries),
        total_taker_fill,
    })
}

fn compute_match_after(
    state: &MatchSpotOrderV3State,
    logical_ts: u64,
    version: u64,
) -> Result<MatchSpotOrderV3AfterChanges, MatchSpotOrderV3Error> {
    let mut taker_after = state.taker_order.clone();
    let mut maker_orders_after = state.maker_orders.clone();
    let mut balance_book = BalanceMap::new(&state.settlement_balances);
    let mut created_balance_ledger_entries = Vec::new();
    let executed_at_ms = logical_ts / 1_000_000;
    let timestamp = logical_ts;

    match taker_after.time_in_force() {
        OrderTif::Gtc | OrderTif::Ioc => {
            let taker_before_match = taker_after.clone();
            let match_outcome = taker_after.match_with_makers(
                &mut maker_orders_after,
                MatchSpotOrderV2Input {
                    match_id: concat2("spot-match:", taker_after.order_id().to_string().as_str()),
                    maker_fee_bps: state.maker_fee_bps,
                    taker_fee_bps: state.taker_fee_bps,
                    executed_at_ms,
                    timestamp,
                    version: Some(version),
                },
            )?;
            if matches!(taker_after.time_in_force(), OrderTif::Gtc)
                && match_outcome.trades.is_empty()
            {
                return Ok(MatchSpotOrderV3AfterChanges::Resting);
            }

            finish_matched_after(
                state,
                taker_before_match,
                taker_after,
                maker_orders_after,
                balance_book,
                created_balance_ledger_entries,
                match_outcome.trades,
                timestamp,
                version,
            )
        }
        OrderTif::Alo => {
            let Some(best_maker) = maker_orders_after.first() else {
                return Ok(MatchSpotOrderV3AfterChanges::Resting);
            };
            if !taker_after.crosses_maker(best_maker)? {
                return Ok(MatchSpotOrderV3AfterChanges::Resting);
            }

            taker_after.reject_as_bad_alo_at_version(timestamp, version)?;
            release_remaining_for_terminal(
                &mut taker_after,
                &mut balance_book,
                &mut created_balance_ledger_entries,
                state.maker_fee_bps,
                state.taker_fee_bps,
                &state.base_asset_id,
                &state.quote_asset_id,
                timestamp,
            )?;
            Ok(MatchSpotOrderV3AfterChanges::Rejected {
                rejected_taker_order_after: taker_after,
                balances_after: balance_book.into_balances(),
                created_balance_ledger_entries,
            })
        }
    }
}

fn finish_matched_after(
    state: &MatchSpotOrderV3State,
    taker_before_match: SpotOrderV2,
    mut taker_after: SpotOrderV2,
    mut maker_orders_after: Vec<SpotOrderV2>,
    mut balance_book: BalanceMap,
    mut created_balance_ledger_entries: Vec<BalanceLedgerV3>,
    trades: Vec<SpotTrade>,
    timestamp: u64,
    version: u64,
) -> Result<MatchSpotOrderV3AfterChanges, MatchSpotOrderV3Error> {
    let MatchedTradeEffects {
        created_trades,
        created_balance_ledger_entries: settled_balance_ledger_entries,
        total_taker_fill,
    } = settle_matched_trades(
        &mut taker_after,
        &mut maker_orders_after,
        trades,
        MatchedTradeSettlementContext {
            base_asset_id: &state.base_asset_id,
            quote_asset_id: &state.quote_asset_id,
            fee_account_id: &state.fee_account_id,
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
            balance_book: &mut balance_book,
            ledger_entries: &mut created_balance_ledger_entries,
            timestamp,
        },
    )?;
    created_balance_ledger_entries = settled_balance_ledger_entries;

    taker_after = taker_before_match;
    taker_after.finish_after_match_at_version(total_taker_fill, timestamp, version)?;
    release_remaining_for_terminal(
        &mut taker_after,
        &mut balance_book,
        &mut created_balance_ledger_entries,
        state.maker_fee_bps,
        state.taker_fee_bps,
        &state.base_asset_id,
        &state.quote_asset_id,
        timestamp,
    )?;

    let balances_after = balance_book.into_balances();
    match taker_after.status() {
        SpotOrderStatus::PartiallyFilled => Ok(MatchSpotOrderV3AfterChanges::PartiallyFilled {
            taker_order_after: taker_after,
            maker_orders_after,
            balances_after,
            created_trades,
            created_balance_ledger_entries,
        }),
        SpotOrderStatus::Filled => Ok(MatchSpotOrderV3AfterChanges::Filled {
            filled_taker_order_after: taker_after,
            maker_orders_after,
            balances_after,
            created_trades,
            created_balance_ledger_entries,
        }),
        SpotOrderStatus::Canceled => Ok(MatchSpotOrderV3AfterChanges::CanceledAfterPartialFill {
            canceled_taker_order_after: taker_after,
            maker_orders_after,
            balances_after,
            created_trades,
            created_balance_ledger_entries,
        }),
        SpotOrderStatus::Rejected => Ok(MatchSpotOrderV3AfterChanges::Rejected {
            rejected_taker_order_after: taker_after,
            balances_after,
            created_balance_ledger_entries,
        }),
        SpotOrderStatus::Open | SpotOrderStatus::Pending => {
            Err(MatchSpotOrderV3Error::OrderMatch(SpotOrderV2MatchError::NoTradesMatched))
        }
    }
}

fn release_remaining_for_terminal(
    order_after: &mut SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerV3>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
    base_asset_id: &str,
    quote_asset_id: &str,
    timestamp: u64,
) -> Result<(), MatchSpotOrderV3Error> {
    let (principal_requirement, fee_requirement) =
        order_after.terminal_release_requirements(maker_fee_bps, taker_fee_bps);
    for requirement in [principal_requirement, fee_requirement].into_iter().flatten() {
        let asset_id = match requirement.asset {
            SpotOrderHoldAsset::Base => base_asset_id,
            SpotOrderHoldAsset::Quote => quote_asset_id,
        };
        release_to_balance(
            order_after,
            asset_id,
            requirement.amount,
            balance_book,
            ledger_entries,
            timestamp,
        )?;
    }
    Ok(())
}

fn release_to_balance(
    order: &SpotOrderV2,
    asset_id: &str,
    amount: u64,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerV3>,
    timestamp: u64,
) -> Result<(), MatchSpotOrderV3Error> {
    let balance = balance_book.get_mut(order.account_id(), asset_id)?;
    let next_release_index =
        ledger_entries.len().checked_add(1).ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;
    let entry = apply_balance_ledger_entry(
        LedgerOperation::Unfreeze,
        common_entity::stable_string_entity_id(&concat4(
            "balance-ledger:",
            order.order_id().to_string().as_str(),
            ":release:",
            next_release_index.to_string().as_str(),
        )),
        balance,
        amount,
        timestamp,
    )?;
    ledger_entries.push(entry);
    Ok(())
}

fn apply_balance_ledger_entry(
    operation: LedgerOperation,
    entry_id: u64,
    balance: &mut BalanceV2,
    amount: u64,
    timestamp: u64,
) -> Result<BalanceLedgerV3, BalanceLedgerV3Error> {
    let mut entry =
        BalanceLedgerV3::new(entry_id, balance, operation, Decimal::from(amount), timestamp)?;
    entry.apply_to(balance, timestamp)?;
    Ok(entry)
}

struct BalanceLedgerDraft {
    operation: LedgerOperation,
    entry_id: u64,
    account_id: String,
    asset_id: String,
    amount: u64,
    timestamp: u64,
}

fn push_applied_balance_ledger_entry(
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerV3>,
    draft: BalanceLedgerDraft,
) -> Result<(), MatchSpotOrderV3Error> {
    let balance = balance_book.get_mut(&draft.account_id, &draft.asset_id)?;
    let entry = apply_balance_ledger_entry(
        draft.operation,
        draft.entry_id,
        balance,
        draft.amount,
        draft.timestamp,
    )?;
    ledger_entries.push(entry);
    Ok(())
}

struct TradeBalanceEffectsContext<'a> {
    settlement_id: &'a str,
    base_asset_id: &'a str,
    quote_asset_id: &'a str,
    fee_account_id: &'a str,
    balance_book: &'a mut BalanceMap,
    ledger_entries: &'a mut Vec<BalanceLedgerV3>,
    timestamp: u64,
}

fn apply_trade_balance_effects(
    trade: &SpotTrade,
    context: TradeBalanceEffectsContext<'_>,
) -> Result<(), MatchSpotOrderV3Error> {
    let quote_notional = trade.notional_quote().ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;
    let buyer_account_id = trade.buyer_account_id().to_string();
    let seller_account_id = trade.seller_account_id().to_string();

    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: LedgerOperation::CreditAvailable,
            entry_id: common_entity::stable_string_entity_id(&concat3(
                "balance-ledger:",
                context.settlement_id,
                ":buyer-base",
            )),
            account_id: buyer_account_id.clone(),
            asset_id: context.base_asset_id.to_string(),
            amount: trade.size,
            timestamp: context.timestamp,
        },
    )?;
    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: LedgerOperation::DebitFrozen,
            entry_id: common_entity::stable_string_entity_id(&concat3(
                "balance-ledger:",
                context.settlement_id,
                ":buyer-quote",
            )),
            account_id: buyer_account_id,
            asset_id: context.quote_asset_id.to_string(),
            amount: quote_notional,
            timestamp: context.timestamp,
        },
    )?;
    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: LedgerOperation::CreditAvailable,
            entry_id: common_entity::stable_string_entity_id(&concat3(
                "balance-ledger:",
                context.settlement_id,
                ":seller-quote",
            )),
            account_id: seller_account_id.clone(),
            asset_id: context.quote_asset_id.to_string(),
            amount: quote_notional,
            timestamp: context.timestamp,
        },
    )?;
    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: LedgerOperation::DebitFrozen,
            entry_id: common_entity::stable_string_entity_id(&concat3(
                "balance-ledger:",
                context.settlement_id,
                ":seller-base",
            )),
            account_id: seller_account_id,
            asset_id: context.base_asset_id.to_string(),
            amount: trade.size,
            timestamp: context.timestamp,
        },
    )?;
    if let Some((buyer_fee_account_id, buyer_fee_amount)) = fee_payer_for_buyer(trade) {
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: LedgerOperation::DebitFrozen,
                entry_id: common_entity::stable_string_entity_id(&concat3(
                    "balance-ledger:",
                    context.settlement_id,
                    ":buyer-fee",
                )),
                account_id: buyer_fee_account_id,
                asset_id: context.quote_asset_id.to_string(),
                amount: buyer_fee_amount,
                timestamp: context.timestamp,
            },
        )?;
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: LedgerOperation::CreditAvailable,
                entry_id: common_entity::stable_string_entity_id(&concat3(
                    "balance-ledger:",
                    context.settlement_id,
                    ":buyer-fee-recv",
                )),
                account_id: context.fee_account_id.to_string(),
                asset_id: context.quote_asset_id.to_string(),
                amount: buyer_fee_amount,
                timestamp: context.timestamp,
            },
        )?;
    }
    if let Some((seller_fee_account_id, seller_fee_amount)) = fee_payer_for_seller(trade) {
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: LedgerOperation::DebitFrozen,
                entry_id: common_entity::stable_string_entity_id(&concat3(
                    "balance-ledger:",
                    context.settlement_id,
                    ":seller-fee",
                )),
                account_id: seller_fee_account_id,
                asset_id: context.quote_asset_id.to_string(),
                amount: seller_fee_amount,
                timestamp: context.timestamp,
            },
        )?;
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: LedgerOperation::CreditAvailable,
                entry_id: common_entity::stable_string_entity_id(&concat3(
                    "balance-ledger:",
                    context.settlement_id,
                    ":seller-fee-recv",
                )),
                account_id: context.fee_account_id.to_string(),
                asset_id: context.quote_asset_id.to_string(),
                amount: seller_fee_amount,
                timestamp: context.timestamp,
            },
        )?;
    }
    Ok(())
}

fn fee_payer_for_buyer(trade: &SpotTrade) -> Option<(String, u64)> {
    let amount = trade.buyer_fee();
    if amount == 0 { None } else { Some((trade.buyer_account_id().to_string(), amount)) }
}

fn fee_payer_for_seller(trade: &SpotTrade) -> Option<(String, u64)> {
    let amount = trade.seller_fee();
    if amount == 0 { None } else { Some((trade.seller_account_id().to_string(), amount)) }
}

fn zip_pairs<T: PartialEq>(
    before: Vec<T>,
    after: Vec<T>,
) -> Result<Vec<UpdatedEntityPair<T>>, MatchSpotOrderV3Error> {
    if before.len() != after.len() {
        return Err(MatchSpotOrderV3Error::TradeOrderCountMismatch);
    }
    Ok(before
        .into_iter()
        .zip(after)
        .filter_map(|(before, after)| {
            (before != after).then_some(UpdatedEntityPair { before, after })
        })
        .collect())
}

pub(super) fn merge_balance_pairs(
    before: Vec<BalanceV2>,
    after: Vec<BalanceV2>,
) -> Result<Vec<UpdatedEntityPair<BalanceV2>>, MatchSpotOrderV3Error> {
    let before_map = before
        .into_iter()
        .map(|balance| (balance.business_entity_id(), balance))
        .collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.business_entity_id();
        let before_balance =
            before_map.get(&balance_id).cloned().ok_or(MatchSpotOrderV3Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(MatchSpotOrderV3Error::BalanceNotFound);
        }
        if before_balance != balance {
            pairs.push(UpdatedEntityPair { before: before_balance, after: balance });
        }
    }
    Ok(pairs)
}

pub(super) fn balance_replay_events_from_ledger_entries(
    updated_balances: &[UpdatedEntityPair<BalanceV2>],
    ledger_entries: &[BalanceLedgerV3],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut current_balances = HashMap::<String, BalanceV2>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.business_entity_id();
        current_balances.insert(balance_id, balance.before.clone());
    }
    let mut events = Vec::with_capacity(ledger_entries.len());

    for entry in ledger_entries {
        let Some(before) = current_balances
            .get(&concat3(&entry.account_id.to_string(), ":", &entry.coin_idx.to_string()))
            .cloned()
        else {
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry does not belong to updated balances".to_string(),
            ));
        };
        let (
            Some(entry_before_available),
            Some(entry_before_frozen),
            Some(entry_after_available),
            Some(entry_after_frozen),
        ) = (
            entry.before_available,
            entry.before_frozen,
            entry.after_available,
            entry.after_frozen,
        )
        else {
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry has not been applied".to_string(),
            ));
        };
        if before.available() != entry_before_available || before.frozen() != entry_before_frozen {
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry breaks balance replay chain".to_string(),
            ));
        }
        let next_version = before
            .version()
            .checked_add(1)
            .ok_or(common_entity::EntityError::VersionOverflow { version: before.version() })?;
        let after = BalanceV2::new(
            before.party_id(),
            before.account_id(),
            before.coin_idx(),
            entry_after_available,
            entry_after_frozen,
            next_version,
            before.created_at(),
        );
        events.push(after.track_update_event_from(&before)?);
        current_balances.insert(
            concat3(&entry.account_id.to_string(), ":", &entry.coin_idx.to_string()).clone(),
            after,
        );
    }

    for balance in updated_balances {
        let balance_id = balance.after.business_entity_id();
        if current_balances.get(&balance_id) != Some(&balance.after) {
            return Err(common_entity::EntityError::Custom(
                "balance replay chain does not reach case-level balance after state".to_string(),
            ));
        }
    }
    Ok(events)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderV3State {
    pub taker_order: SpotOrderV2,
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<BalanceV2>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderV3UseCase;

impl UseCase for MatchSpotOrderV3UseCase {
    type Command = LogEntry<MatchSpotOrderV3Cmd>;
    type StateGiven = MatchSpotOrderV3State;
    type Error = MatchSpotOrderV3Error;
    type StateChanged = MatchSpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(MatchSpotOrderV3Error::InvalidPartyId);
        }
        if cmd.order_id == 0 {
            return Err(MatchSpotOrderV3Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.taker_order.order_id() != cmd.order_id {
            return Err(MatchSpotOrderV3Error::TakerOrderIdMismatch);
        }
        if state.taker_order.account_id() != cmd.party_id {
            return Err(MatchSpotOrderV3Error::TakerAccountIdMismatch);
        }
        if state.taker_order.asset() != cmd.asset {
            return Err(MatchSpotOrderV3Error::TakerAssetMismatch);
        }
        if state.fee_account_id.is_empty() {
            return Err(MatchSpotOrderV3Error::InvalidFeeAccountId);
        }

        if !state.taker_order.can_enter_matching() {
            return Err(MatchSpotOrderV3Error::OrderMatch(
                SpotOrderV2MatchError::OrderNotMatchable,
            ));
        }
        for maker in &state.maker_orders {
            maker.ensure_matchable()?;
        }
        state.taker_order.ensure_matchable()?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        compute_match_after(state, cmd.logical_ts(), cmd.lsn())
    }
}

impl StateDiff for MatchSpotOrderV3UseCase {
    type StateDiff = MatchSpotOrderV3Changes;

    fn do_compute_state_diff(
        state: MatchSpotOrderV3State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        match after {
            MatchSpotOrderV3AfterChanges::Resting => Ok(MatchSpotOrderV3Changes::Resting),
            MatchSpotOrderV3AfterChanges::Rejected {
                rejected_taker_order_after,
                balances_after,
                created_balance_ledger_entries,
            } => {
                let updated_taker_order = UpdatedEntityPair {
                    before: state.taker_order,
                    after: rejected_taker_order_after,
                };
                let updated_balances =
                    merge_balance_pairs(state.settlement_balances, balances_after)?;
                Ok(MatchSpotOrderV3Changes::Rejected {
                    rejected_taker_order: updated_taker_order,
                    updated_balances,
                    created_balance_ledger_entries,
                })
            }
            MatchSpotOrderV3AfterChanges::PartiallyFilled {
                taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_balance_ledger_entries,
            } => {
                if created_trades.is_empty() {
                    return Err(MatchSpotOrderV3Error::OrderMatch(
                        SpotOrderV2MatchError::NoTradesMatched,
                    ));
                }
                let (updated_taker_order, updated_maker_orders, updated_balances) =
                    build_matched_pairs(
                        state,
                        taker_order_after,
                        maker_orders_after,
                        balances_after,
                    )?;
                Ok(MatchSpotOrderV3Changes::PartiallyFilled {
                    updated_taker_order,
                    updated_maker_orders,
                    updated_balances,
                    created_trades,
                    created_balance_ledger_entries,
                })
            }
            MatchSpotOrderV3AfterChanges::Filled {
                filled_taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_balance_ledger_entries,
            } => {
                if created_trades.is_empty() {
                    return Err(MatchSpotOrderV3Error::OrderMatch(
                        SpotOrderV2MatchError::NoTradesMatched,
                    ));
                }
                let (filled_taker_order, updated_maker_orders, updated_balances) =
                    build_matched_pairs(
                        state,
                        filled_taker_order_after,
                        maker_orders_after,
                        balances_after,
                    )?;
                Ok(MatchSpotOrderV3Changes::Filled {
                    filled_taker_order,
                    updated_maker_orders,
                    updated_balances,
                    created_trades,
                    created_balance_ledger_entries,
                })
            }
            MatchSpotOrderV3AfterChanges::CanceledAfterPartialFill {
                canceled_taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_balance_ledger_entries,
            } => {
                if created_trades.is_empty() {
                    return Err(MatchSpotOrderV3Error::OrderMatch(
                        SpotOrderV2MatchError::NoTradesMatched,
                    ));
                }
                let (canceled_taker_order, updated_maker_orders, updated_balances) =
                    build_matched_pairs(
                        state,
                        canceled_taker_order_after,
                        maker_orders_after,
                        balances_after,
                    )?;
                Ok(MatchSpotOrderV3Changes::CanceledAfterPartialFill {
                    canceled_taker_order,
                    updated_maker_orders,
                    updated_balances,
                    created_trades,
                    created_balance_ledger_entries,
                })
            }
        }
    }
}

fn build_matched_pairs(
    state: MatchSpotOrderV3State,
    taker_order_after: SpotOrderV2,
    maker_orders_after: Vec<SpotOrderV2>,
    balances_after: Vec<BalanceV2>,
) -> Result<
    (
        UpdatedEntityPair<SpotOrderV2>,
        Vec<UpdatedEntityPair<SpotOrderV2>>,
        Vec<UpdatedEntityPair<BalanceV2>>,
    ),
    MatchSpotOrderV3Error,
> {
    let updated_taker_order =
        UpdatedEntityPair { before: state.taker_order, after: taker_order_after };
    let updated_maker_orders = zip_pairs(state.maker_orders, maker_orders_after)?;
    let updated_balances = merge_balance_pairs(state.settlement_balances, balances_after)?;
    Ok((updated_taker_order, updated_maker_orders, updated_balances))
}

impl MatchSpotOrderV3Changes {
    pub fn taker_order_pair(&self) -> Option<&UpdatedEntityPair<SpotOrderV2>> {
        match self {
            Self::Resting => None,
            Self::PartiallyFilled { updated_taker_order, .. }
            | Self::Filled { filled_taker_order: updated_taker_order, .. }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order, ..
            }
            | Self::Rejected { rejected_taker_order: updated_taker_order, .. } => {
                Some(updated_taker_order)
            }
        }
    }

    pub fn taker_order_after(&self) -> Option<&SpotOrderV2> {
        self.taker_order_pair().map(|pair| &pair.after)
    }

    pub fn updated_maker_orders(&self) -> &[UpdatedEntityPair<SpotOrderV2>] {
        match self {
            Self::Resting | Self::Rejected { .. } => &[],
            Self::PartiallyFilled { updated_maker_orders, .. }
            | Self::Filled { updated_maker_orders, .. }
            | Self::CanceledAfterPartialFill { updated_maker_orders, .. } => updated_maker_orders,
        }
    }

    pub fn updated_balances(&self) -> &[UpdatedEntityPair<BalanceV2>] {
        match self {
            Self::Resting => &[],
            Self::PartiallyFilled { updated_balances, .. }
            | Self::Filled { updated_balances, .. }
            | Self::CanceledAfterPartialFill { updated_balances, .. }
            | Self::Rejected { updated_balances, .. } => updated_balances,
        }
    }

    pub fn created_trades(&self) -> &[SpotTrade] {
        match self {
            Self::Resting | Self::Rejected { .. } => &[],
            Self::PartiallyFilled { created_trades, .. }
            | Self::Filled { created_trades, .. }
            | Self::CanceledAfterPartialFill { created_trades, .. } => created_trades,
        }
    }

    pub fn created_balance_ledger_entries(&self) -> &[BalanceLedgerV3] {
        match self {
            Self::Resting => &[],
            Self::PartiallyFilled { created_balance_ledger_entries, .. }
            | Self::Filled { created_balance_ledger_entries, .. }
            | Self::CanceledAfterPartialFill { created_balance_ledger_entries, .. }
            | Self::Rejected { created_balance_ledger_entries, .. } => {
                created_balance_ledger_entries
            }
        }
    }
}

pub(super) struct BalanceMap {
    balances: HashMap<String, BalanceV2>,
}

impl BalanceMap {
    pub(super) fn new(balances: &[BalanceV2]) -> Self {
        Self {
            balances: balances
                .iter()
                .cloned()
                .map(|balance| (balance.business_entity_id(), balance))
                .collect(),
        }
    }

    pub(super) fn get_mut(
        &mut self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<&mut BalanceV2, MatchSpotOrderV3Error> {
        self.balances
            .get_mut(&concat3(
                &common_entity::stable_string_entity_id(account_id).to_string(),
                ":",
                &common_entity::stable_string_entity_id(asset_id).to_string(),
            ))
            .ok_or(MatchSpotOrderV3Error::BalanceNotFound)
    }

    pub(super) fn into_balances(self) -> Vec<BalanceV2> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
}
