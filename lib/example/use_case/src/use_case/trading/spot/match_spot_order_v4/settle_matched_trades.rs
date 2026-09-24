use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff,
    StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use spot_entity::spot_order_v2::SpotOrderV2;

use super::support::{
    BalanceMap, MatchSpotOrderV4Error, MatchedTradeSettlementContext,
    balance_replay_events_from_ledger_entries, merge_balance_pairs, release_remaining_for_terminal,
    settle_matched_trades, validate_all_reservations_for_order, zip_pairs,
};
use crate::SpotTrade;
use crate::entity::account::balance_ledger_entry_v2::BalanceLedgerEntryV2;
use crate::entity::{Balance, SettlementTransferVoucher, spot as spot_entity};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettleMatchedSpotTradesV4Cmd {
    pub settlement_id_prefix: String,
}

impl Default for SettleMatchedSpotTradesV4Cmd {
    fn default() -> Self {
        Self { settlement_id_prefix: "spot-settlement".to_owned() }
    }
}

/// V4 settlement use case 的 authoritative settlement-after truth。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettleMatchedSpotTradesV4AfterChanges {
    Unchanged,
    Settled {
        taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        balances_after: Vec<Balance>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
}

/// V4 settlement use case 的 replayable changes。
///
/// 撮合产生的 `SpotTrade` 不在这里重复创建；本 use case 只表达成交后的 reservation、
/// voucher、balance ledger 与 balance 更新事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettleMatchedSpotTradesV4Changes {
    Unchanged,
    Settled {
        updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        updated_balances: Vec<UpdatedEntityPair<Balance>>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
}

impl ReplayableChanges for SettleMatchedSpotTradesV4Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Unchanged => Ok(Vec::new()),
            Self::Settled {
                updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_vouchers,
                created_balance_ledger_entries,
            } => {
                let mut events = Vec::with_capacity(
                    1_usize
                        .saturating_add(updated_maker_orders.len())
                        .saturating_add(created_vouchers.len())
                        .saturating_add(created_balance_ledger_entries.len().saturating_mul(2)),
                );
                events.push(
                    updated_taker_order
                        .after
                        .track_update_event_from(&updated_taker_order.before)?,
                );
                for maker in updated_maker_orders {
                    events.push(maker.after.track_update_event_from(&maker.before)?);
                }
                for voucher in created_vouchers {
                    events.push(voucher.track_create_event()?);
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleMatchedSpotTradesV4State {
    /// MatchV4 产出的 taker 订单 after；settlement 会在此基础上消费/释放 reservation。
    pub taker_order: SpotOrderV2,
    /// MatchV4 产出的 maker 订单 after 列表，顺序必须与成交 maker 优先级一致。
    pub maker_orders: Vec<SpotOrderV2>,
    pub created_trades: Vec<SpotTrade>,
    pub settlement_balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SettleMatchedSpotTradesV4UseCase;

impl StateMachineV2Unchecked for SettleMatchedSpotTradesV4UseCase {
    type Command = SettleMatchedSpotTradesV4Cmd;
    type StateGiven = SettleMatchedSpotTradesV4State;
    type Error = MatchSpotOrderV4Error;
    type StateChanged = SettleMatchedSpotTradesV4AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.settlement_id_prefix.is_empty() {
            return Err(MatchSpotOrderV4Error::InvalidSettlementIdPrefix);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        _cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.fee_account_id.is_empty() {
            return Err(MatchSpotOrderV4Error::InvalidFeeAccountId);
        }
        if state.created_trades.len() > state.maker_orders.len() {
            return Err(MatchSpotOrderV4Error::ReservationCountMismatch);
        }
        validate_all_reservations_for_order(
            &state.taker_order,
            &state.base_asset_id,
            &state.quote_asset_id,
        )?;
        for maker in &state.maker_orders {
            validate_all_reservations_for_order(
                maker,
                &state.base_asset_id,
                &state.quote_asset_id,
            )?;
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        compute_settlement_after(cmd, state, context)
    }
}

impl StateMachineOwnedV2Diff for SettleMatchedSpotTradesV4UseCase {
    type StateDiff = SettleMatchedSpotTradesV4Changes;

    fn do_compute_state_diff(
        state: SettleMatchedSpotTradesV4State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        match after {
            SettleMatchedSpotTradesV4AfterChanges::Unchanged => {
                Ok(SettleMatchedSpotTradesV4Changes::Unchanged)
            }
            SettleMatchedSpotTradesV4AfterChanges::Settled {
                taker_order_after,
                maker_orders_after,
                balances_after,
                created_vouchers,
                created_balance_ledger_entries,
            } => {
                let updated_taker_order =
                    UpdatedEntityPair { before: state.taker_order, after: taker_order_after };
                let updated_maker_orders = zip_pairs(state.maker_orders, maker_orders_after)?;
                let updated_balances =
                    merge_balance_pairs(state.settlement_balances, balances_after)?;
                Ok(SettleMatchedSpotTradesV4Changes::Settled {
                    updated_taker_order,
                    updated_maker_orders,
                    updated_balances,
                    created_vouchers,
                    created_balance_ledger_entries,
                })
            }
        }
    }
}

fn compute_settlement_after(
    cmd: &SettleMatchedSpotTradesV4Cmd,
    state: &SettleMatchedSpotTradesV4State,
    context: &ExecutionContext,
) -> Result<SettleMatchedSpotTradesV4AfterChanges, MatchSpotOrderV4Error> {
    let mut taker_after = state.taker_order.clone();
    let mut maker_orders_after = state.maker_orders.clone();
    let mut balance_book = BalanceMap::new(&state.settlement_balances);
    let mut created_balance_ledger_entries = Vec::new();

    let settlement_effects = settle_matched_trades(
        &mut taker_after,
        &mut maker_orders_after,
        &state.created_trades,
        MatchedTradeSettlementContext {
            settlement_id_prefix: &cmd.settlement_id_prefix,
            base_asset_id: &state.base_asset_id,
            quote_asset_id: &state.quote_asset_id,
            fee_account_id: &state.fee_account_id,
            balance_book: &mut balance_book,
            ledger_entries: &mut created_balance_ledger_entries,
        },
    )?;

    release_remaining_for_terminal(
        &mut taker_after,
        &mut balance_book,
        &mut created_balance_ledger_entries,
        state.maker_fee_bps,
        state.taker_fee_bps,
    )?;

    bump_order_version_if_changed(&state.taker_order, &mut taker_after, context.execution_time_ns)?;
    bump_changed_maker_versions(
        &state.maker_orders,
        &mut maker_orders_after,
        context.execution_time_ns,
    )?;

    if taker_after == state.taker_order
        && maker_orders_after == state.maker_orders
        && created_balance_ledger_entries.is_empty()
        && settlement_effects.created_vouchers.is_empty()
    {
        return Ok(SettleMatchedSpotTradesV4AfterChanges::Unchanged);
    }

    Ok(SettleMatchedSpotTradesV4AfterChanges::Settled {
        taker_order_after: taker_after,
        maker_orders_after,
        balances_after: balance_book.into_balances(),
        created_vouchers: settlement_effects.created_vouchers,
        created_balance_ledger_entries,
    })
}

fn bump_changed_maker_versions(
    before: &[SpotOrderV2],
    after: &mut [SpotOrderV2],
    timestamp: u64,
) -> Result<(), MatchSpotOrderV4Error> {
    if before.len() != after.len() {
        return Err(MatchSpotOrderV4Error::ReservationCountMismatch);
    }
    for (before, after) in before.iter().zip(after.iter_mut()) {
        bump_order_version_if_changed(before, after, timestamp)?;
    }
    Ok(())
}

fn bump_order_version_if_changed(
    before: &SpotOrderV2,
    after: &mut SpotOrderV2,
    timestamp: u64,
) -> Result<(), MatchSpotOrderV4Error> {
    if before == after {
        return Ok(());
    }
    after.version =
        before.version.checked_add(1).ok_or(MatchSpotOrderV4Error::ArithmeticOverflow)?;
    after.updated_at = timestamp;
    Ok(())
}

impl SettleMatchedSpotTradesV4Changes {
    pub fn taker_order_after(&self) -> Option<&SpotOrderV2> {
        match self {
            Self::Unchanged => None,
            Self::Settled { updated_taker_order, .. } => Some(&updated_taker_order.after),
        }
    }
}
