use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff,
    StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use spot_entity::spot_order_v2::{
    SpotOrderV2, SpotOrderV2BehaviorError, SpotOrderV2MatchError, SpotOrderV2MatchingDecision,
    spot_order_v2_matching_decision,
};
use thiserror::Error;

use crate::entity::account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error, BalanceLedgerOperation,
};
use crate::entity::account::balance_ledger_reason::BalanceLedgerReason;
use crate::entity::account::settlement_transfer_voucher::SettlementTransferPurpose;
use crate::entity::{
    Balance, Reservation, ReservationCloseReason, ReservationKind, ReservationMarketKind,
    SettlementTransferVoucher, SpotOrderSide, SpotOrderStatus, spot as spot_entity,
};
use crate::support::{concat2, concat3, concat4};
use crate::{MatchSpotOrderV2Input, SpotTrade};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchSpotOrderV3Cmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: String,
}

/// 撮合后对 taker 的五种业务结论。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV3AfterChanges {
    /// 没有成交，订单继续留在订单簿；本次没有其它业务副作用。
    Resting ,
    /// 发生成交但仍有剩余数量，订单继续可撮合。
    PartiallyFilled {
        taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        balances_after: Vec<Balance>,
        created_trades: Vec<SpotTrade>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
    /// 成交数量达到订单总数量，订单终止为 Filled。
    Filled {
        filled_taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        balances_after: Vec<Balance>,
        created_trades: Vec<SpotTrade>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
    /// IOC 部分成交后，剩余数量被取消并释放冻结。
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        balances_after: Vec<Balance>,
        created_trades: Vec<SpotTrade>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
    /// ALO crossing maker 或 IOC 无流动性导致拒单。
    Rejected {
        rejected_taker_order_after: SpotOrderV2,
        balances_after: Vec<Balance>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
}

/// 撮合后的 before/after 业务变化。
///
/// 更新实体只通过 `UpdatedEntityPair` 表达 authoritative before/after；成交、voucher
/// 和 ledger 则直接表达本次撮合新产生的事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV3Changes {
    /// 没有状态变化，也没有新增业务事实。
    Resting,
    PartiallyFilled {
        updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        updated_balances: Vec<UpdatedEntityPair<Balance>>,
        created_trades: Vec<SpotTrade>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
    Filled {
        filled_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        updated_balances: Vec<UpdatedEntityPair<Balance>>,
        created_trades: Vec<SpotTrade>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
    CanceledAfterPartialFill {
        canceled_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        updated_balances: Vec<UpdatedEntityPair<Balance>>,
        created_trades: Vec<SpotTrade>,
        created_vouchers: Vec<SettlementTransferVoucher>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
    Rejected {
        rejected_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_balances: Vec<UpdatedEntityPair<Balance>>,
        created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderV3Error {
    #[error("party id must not be empty")]
    InvalidPartyId,
    #[error("order id must not be empty")]
    InvalidOrderId,
    #[error("taker order id does not match command")]
    TakerOrderIdMismatch,
    #[error("taker account id does not match command")]
    TakerAccountIdMismatch,
    #[error("taker asset does not match command")]
    TakerAssetMismatch,
    #[error("fee account id must not be empty")]
    InvalidFeeAccountId,
    #[error("reservation caused_by_order_id does not match order")]
    ReservationOrderMismatch,
    #[error("reservation kind does not match order role")]
    ReservationKindMismatch,
    #[error("reservation asset does not match order hold asset")]
    ReservationAssetMismatch,
    #[error("reservation count does not match maker order count")]
    ReservationCountMismatch,
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
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
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
                created_vouchers,
                created_balance_ledger_entries,
            }
            | Self::Filled {
                filled_taker_order: updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            } => replay_events(
                updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            ),
            Self::Rejected {
                rejected_taker_order: updated_taker_order,
                updated_balances,
                created_balance_ledger_entries,
            } => replay_events(
                updated_taker_order,
                &[],
                updated_balances,
                &[],
                &[],
                created_balance_ledger_entries,
            ),
        }
    }
}

fn replay_events(
    updated_taker_order: &UpdatedEntityPair<SpotOrderV2>,
    updated_maker_orders: &[UpdatedEntityPair<SpotOrderV2>],
    updated_balances: &[UpdatedEntityPair<Balance>],
    created_trades: &[SpotTrade],
    created_vouchers: &[SettlementTransferVoucher],
    created_balance_ledger_entries: &[BalanceLedgerEntryV2],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let event_capacity = 1_usize
        .saturating_add(created_trades.len())
        .saturating_add(updated_maker_orders.len())
        .saturating_add(created_vouchers.len())
        .saturating_add(created_balance_ledger_entries.len().saturating_mul(2));
    let mut events = Vec::with_capacity(event_capacity);
    events.push(updated_taker_order.after.track_update_event_from(&updated_taker_order.before)?);
    for trade in created_trades {
        events.push(trade.track_create_event()?);
    }
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

struct ActiveOrderAfterInput<'a> {
    taker_after: SpotOrderV2,
    maker_orders_after: Vec<SpotOrderV2>,
    balance_book: BalanceMap,
    created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    base_asset_id: &'a str,
    quote_asset_id: &'a str,
    fee_account_id: &'a str,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
    executed_at_ms: u64,
    timestamp: u64,
}

struct ActiveOrderAfter {
    taker_order_after: SpotOrderV2,
    maker_orders_after: Vec<SpotOrderV2>,
    balances_after: Vec<Balance>,
    created_trades: Vec<SpotTrade>,
    created_vouchers: Vec<SettlementTransferVoucher>,
    created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

fn classify_after(after: ActiveOrderAfter) -> MatchSpotOrderV3AfterChanges {
    let ActiveOrderAfter {
        taker_order_after,
        maker_orders_after,
        balances_after,
        created_trades,
        created_vouchers,
        created_balance_ledger_entries,
    } = after;

    match taker_order_after.status() {
        SpotOrderStatus::Open => MatchSpotOrderV3AfterChanges::Resting,
        SpotOrderStatus::PartiallyFilled => MatchSpotOrderV3AfterChanges::PartiallyFilled {
            taker_order_after,
            maker_orders_after,
            balances_after,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        },
        SpotOrderStatus::Filled => MatchSpotOrderV3AfterChanges::Filled {
            filled_taker_order_after: taker_order_after,
            maker_orders_after,
            balances_after,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        },
        SpotOrderStatus::Canceled => MatchSpotOrderV3AfterChanges::CanceledAfterPartialFill {
            canceled_taker_order_after: taker_order_after,
            maker_orders_after,
            balances_after,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        },
        SpotOrderStatus::Rejected => MatchSpotOrderV3AfterChanges::Rejected {
            rejected_taker_order_after: taker_order_after,
            balances_after,
            created_balance_ledger_entries,
        },
        SpotOrderStatus::Pending => MatchSpotOrderV3AfterChanges::Rejected {
            rejected_taker_order_after: taker_order_after,
            balances_after,
            created_balance_ledger_entries,
        },
    }
}

fn compute_active_order_after(
    input: ActiveOrderAfterInput<'_>,
) -> Result<ActiveOrderAfter, MatchSpotOrderV3Error> {
    let ActiveOrderAfterInput {
        mut taker_after,
        mut maker_orders_after,
        mut balance_book,
        mut created_balance_ledger_entries,
        base_asset_id,
        quote_asset_id,
        fee_account_id,
        maker_fee_bps,
        taker_fee_bps,
        executed_at_ms,
        timestamp,
    } = input;
    let mut created_trades = Vec::with_capacity(0);
    let mut created_vouchers = Vec::with_capacity(0);

    match spot_order_v2_matching_decision(&taker_after, maker_orders_after.first())? {
        SpotOrderV2MatchingDecision::Rest => {
            return Ok(ActiveOrderAfter {
                taker_order_after: taker_after,
                maker_orders_after,
                balances_after: balance_book.into_balances(),
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            });
        }
        SpotOrderV2MatchingDecision::RejectAlo => {
            taker_after.reject_as_bad_alo(timestamp)?;
            release_remaining_for_terminal(
                &mut taker_after,
                &mut balance_book,
                &mut created_balance_ledger_entries,
                maker_fee_bps,
                taker_fee_bps,
            )?;
            return Ok(ActiveOrderAfter {
                taker_order_after: taker_after,
                maker_orders_after,
                balances_after: balance_book.into_balances(),
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            });
        }
        SpotOrderV2MatchingDecision::Match => {}
    }

    let taker_before_match = taker_after.clone();
    let match_outcome = taker_after.match_with_makers(
        &mut maker_orders_after,
        MatchSpotOrderV2Input {
            match_id: concat2("spot-match:", taker_after.order_id()),
            maker_fee_bps,
            taker_fee_bps,
            executed_at_ms,
            timestamp,
        },
    )?;
    let mut total_taker_fill = 0_u64;
    for (index, trade) in match_outcome.trades.into_iter().enumerate() {
        let trade_notional =
            trade.notional_quote().ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;

        total_taker_fill = total_taker_fill
            .checked_add(trade.qty)
            .ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;

        let taker_principal_consume =
            principal_consume_amount_for_taker(&taker_after, trade.qty, trade_notional);
        consume_reservation(
            &mut taker_after.reservation,
            taker_principal_consume,
            ReservationCloseReason::Filled,
        )?;
        let Some(maker_order_after) = maker_orders_after.get_mut(index) else {
            return Err(MatchSpotOrderV3Error::BalanceNotFound);
        };
        let maker_principal_consume =
            principal_consume_amount_for_maker(maker_order_after, trade.qty, trade_notional);
        consume_reservation(
            &mut maker_order_after.reservation,
            maker_principal_consume,
            ReservationCloseReason::Filled,
        )?;

        consume_reservation(
            &mut taker_after.fee_reservation,
            trade.taker_fee,
            ReservationCloseReason::Filled,
        )?;
        consume_reservation(
            &mut maker_order_after.fee_reservation,
            trade.maker_fee,
            ReservationCloseReason::Filled,
        )?;

        let settlement_id = concat2("spot-settlement:", trade.trade_id.as_str());
        let voucher = trade
            .derive_spot_settlement_transfer_voucher_with_fees(
                concat2("spot-voucher:", trade.trade_id.as_str()),
                settlement_id.clone(),
                base_asset_id,
                quote_asset_id,
                fee_account_id.to_string(),
            )
            .ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;

        apply_trade_balance_effects(
            &trade,
            TradeBalanceEffectsContext {
                settlement_id: &settlement_id,
                base_asset_id,
                quote_asset_id,
                fee_account_id,
                balance_book: &mut balance_book,
                ledger_entries: &mut created_balance_ledger_entries,
            },
        )?;

        created_trades.push(trade);
        created_vouchers.push(voucher);
    }

    let taker_reservation_after_match = taker_after.reservation.clone();
    let taker_fee_reservation_after_match = taker_after.fee_reservation.clone();
    taker_after = taker_before_match;
    taker_after.reservation = taker_reservation_after_match;
    taker_after.fee_reservation = taker_fee_reservation_after_match;
    taker_after.finish_after_match(total_taker_fill, timestamp)?;
    release_remaining_for_terminal(
        &mut taker_after,
        &mut balance_book,
        &mut created_balance_ledger_entries,
        maker_fee_bps,
        taker_fee_bps,
    )?;

    Ok(ActiveOrderAfter {
        taker_order_after: taker_after,
        maker_orders_after,
        balances_after: balance_book.into_balances(),
        created_trades,
        created_vouchers,
        created_balance_ledger_entries,
    })
}

fn expected_principal_kind_for(side: SpotOrderSide) -> ReservationKind {
    match side {
        SpotOrderSide::Buy => ReservationKind::SpotBuyQuote,
        SpotOrderSide::Sell => ReservationKind::SpotSellBase,
    }
}

fn expected_fee_kind_for(side: SpotOrderSide) -> ReservationKind {
    match side {
        SpotOrderSide::Buy => ReservationKind::SpotBuyFeeQuote,
        SpotOrderSide::Sell => ReservationKind::SpotSellFeeQuote,
    }
}

fn validate_reservation_for_order(
    order: &SpotOrderV2,
    reservation: &Reservation,
    expected_kind: ReservationKind,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), MatchSpotOrderV3Error> {
    if reservation.market_kind != ReservationMarketKind::Spot {
        return Err(MatchSpotOrderV3Error::ReservationKindMismatch);
    }
    if reservation.caused_by_order_id != order.order_id() {
        return Err(MatchSpotOrderV3Error::ReservationOrderMismatch);
    }
    if reservation.reservation_kind != expected_kind {
        return Err(MatchSpotOrderV3Error::ReservationKindMismatch);
    }
    let is_zero_fee_reservation = matches!(
        expected_kind,
        ReservationKind::SpotBuyFeeQuote | ReservationKind::SpotSellFeeQuote
    ) && reservation.original_amount == 0
        && reservation.remaining_amount == 0;
    if !reservation.is_active() && !is_zero_fee_reservation {
        return Err(MatchSpotOrderV3Error::ReservationKindMismatch);
    }
    let expected_asset = match expected_kind {
        ReservationKind::SpotBuyQuote
        | ReservationKind::SpotBuyFeeQuote
        | ReservationKind::SpotSellFeeQuote => quote_asset_id,
        ReservationKind::SpotSellBase => base_asset_id,
        _ => return Err(MatchSpotOrderV3Error::ReservationKindMismatch),
    };
    if reservation.asset_id != expected_asset {
        return Err(MatchSpotOrderV3Error::ReservationAssetMismatch);
    }
    Ok(())
}

pub(super) fn validate_all_reservations_for_order(
    order: &SpotOrderV2,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), MatchSpotOrderV3Error> {
    validate_reservation_for_order(
        order,
        &order.reservation,
        expected_principal_kind_for(order.side()),
        base_asset_id,
        quote_asset_id,
    )?;
    validate_reservation_for_order(
        order,
        &order.fee_reservation,
        expected_fee_kind_for(order.side()),
        base_asset_id,
        quote_asset_id,
    )
}

fn principal_consume_amount_for_taker(
    order: &SpotOrderV2,
    trade_qty: u64,
    trade_notional: u64,
) -> u64 {
    match order.side() {
        SpotOrderSide::Buy => trade_notional,
        SpotOrderSide::Sell => trade_qty,
    }
}

fn principal_consume_amount_for_maker(
    order: &SpotOrderV2,
    trade_qty: u64,
    trade_notional: u64,
) -> u64 {
    match order.side() {
        SpotOrderSide::Buy => trade_notional,
        SpotOrderSide::Sell => trade_qty,
    }
}

fn consume_reservation(
    reservation: &mut Reservation,
    amount: u64,
    terminal_reason: ReservationCloseReason,
) -> Result<(), MatchSpotOrderV3Error> {
    if amount == 0 {
        return Ok(());
    }
    let before = reservation.clone();
    let close_reason = if amount == before.remaining_amount { Some(terminal_reason) } else { None };
    reservation.consume(amount, close_reason).map_err(map_reservation_error_to_family)?;
    Ok(())
}

enum OrderReservationSlot {
    Principal,
    Fee,
}

fn release_remaining_for_terminal(
    order_after: &mut SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), MatchSpotOrderV3Error> {
    let (principal_requirement, fee_requirement) =
        order_after.terminal_release_requirements(maker_fee_bps, taker_fee_bps);

    if let Some(requirement) = principal_requirement {
        release_order_reservation(
            order_after,
            OrderReservationSlot::Principal,
            requirement.amount,
            reservation_close_reason_for_order_release(requirement.reason),
            balance_book,
            ledger_entries,
        )?;
    }

    if let Some(requirement) = fee_requirement {
        release_order_reservation(
            order_after,
            OrderReservationSlot::Fee,
            requirement.amount,
            reservation_close_reason_for_order_release(requirement.reason),
            balance_book,
            ledger_entries,
        )?;
    }
    Ok(())
}

fn reservation_close_reason_for_order_release(
    reason: crate::SpotOrderReleaseReason,
) -> ReservationCloseReason {
    match reason {
        crate::SpotOrderReleaseReason::Canceled => ReservationCloseReason::Canceled,
        crate::SpotOrderReleaseReason::IocUnfilled => ReservationCloseReason::IocRemainderCanceled,
        crate::SpotOrderReleaseReason::Rejected => ReservationCloseReason::Rejected,
        crate::SpotOrderReleaseReason::FilledCleanup => ReservationCloseReason::Filled,
    }
}

fn release_order_reservation(
    order: &mut SpotOrderV2,
    slot: OrderReservationSlot,
    max_amount: u64,
    close_reason: ReservationCloseReason,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), MatchSpotOrderV3Error> {
    let (asset_id, release_amount) = {
        let reservation = match slot {
            OrderReservationSlot::Principal => &mut order.reservation,
            OrderReservationSlot::Fee => &mut order.fee_reservation,
        };
        let release_amount = reservation.remaining_amount.min(max_amount);
        if release_amount == 0 {
            return Ok(());
        }
        let asset_id = reservation.asset_id.clone();
        reservation
            .release(release_amount, Some(close_reason))
            .map_err(map_reservation_error_to_family)?;
        (asset_id, release_amount)
    };
    release_to_balance(order, &asset_id, release_amount, balance_book, ledger_entries)
}

fn release_to_balance(
    order: &SpotOrderV2,
    asset_id: &str,
    amount: u64,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), MatchSpotOrderV3Error> {
    let reason = match order.side() {
        SpotOrderSide::Buy => BalanceLedgerReason::CancelSpotOrderReleaseQuote {
            order_id: order.order_id().to_string(),
        },
        SpotOrderSide::Sell => BalanceLedgerReason::CancelSpotOrderReleaseBase {
            order_id: order.order_id().to_string(),
        },
    };
    let balance = balance_book.get_mut(order.account_id(), asset_id)?;
    let next_release_index =
        ledger_entries.len().checked_add(1).ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;
    let entry = apply_balance_ledger_entry(
        BalanceLedgerOperation::Unfreeze,
        concat4(
            "balance-ledger:",
            order.order_id(),
            ":release:",
            next_release_index.to_string().as_str(),
        ),
        balance,
        amount,
        reason,
    )?;
    ledger_entries.push(entry);
    Ok(())
}

fn apply_balance_ledger_entry(
    operation: BalanceLedgerOperation,
    entry_id: String,
    balance: &mut Balance,
    amount: u64,
    reason: BalanceLedgerReason,
) -> Result<BalanceLedgerEntryV2, BalanceLedgerEntryV2Error> {
    let account_id = balance.account_id.clone();
    let asset_id = balance.asset_id.clone();
    let balance_entity_id = balance.entity_id();
    let mut entry = match operation {
        BalanceLedgerOperation::Unfreeze => BalanceLedgerEntryV2::unfreeze(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::CreditAvailable => BalanceLedgerEntryV2::credit_available(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::DebitFrozen => BalanceLedgerEntryV2::debit_frozen(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::Freeze | BalanceLedgerOperation::DebitAvailable => {
            return Err(BalanceLedgerEntryV2Error::InvalidAmount);
        }
    }?;
    entry.apply_to(balance)?;
    Ok(entry)
}

struct BalanceLedgerDraft {
    operation: BalanceLedgerOperation,
    entry_id: String,
    account_id: String,
    asset_id: String,
    amount: u64,
    reason: BalanceLedgerReason,
}

fn push_applied_balance_ledger_entry(
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    draft: BalanceLedgerDraft,
) -> Result<(), MatchSpotOrderV3Error> {
    let balance = balance_book.get_mut(&draft.account_id, &draft.asset_id)?;
    let entry = apply_balance_ledger_entry(
        draft.operation,
        draft.entry_id,
        balance,
        draft.amount,
        draft.reason,
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
    ledger_entries: &'a mut Vec<BalanceLedgerEntryV2>,
}

fn apply_trade_balance_effects(
    trade: &SpotTrade,
    context: TradeBalanceEffectsContext<'_>,
) -> Result<(), MatchSpotOrderV3Error> {
    let trade_ids = vec![trade.trade_id.clone()];
    let settlement_ids = vec![context.settlement_id.to_string()];

    let buyer_receive_base_reason = BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
        trade_ids: trade_ids.clone(),
        settlement_ids: settlement_ids.clone(),
    };
    let buyer_release_quote_reason = BalanceLedgerReason::SettleSpotTradeBuyerReleaseFrozenQuote {
        trade_ids: trade_ids.clone(),
        settlement_ids: settlement_ids.clone(),
    };
    let seller_receive_quote_reason = BalanceLedgerReason::SettleSpotTradeSellerReceiveQuote {
        trade_ids: trade_ids.clone(),
        settlement_ids: settlement_ids.clone(),
    };
    let seller_release_base_reason =
        BalanceLedgerReason::SettleSpotTradeSellerReleaseFrozenBase { trade_ids, settlement_ids };

    let quote_notional = trade.notional_quote().ok_or(MatchSpotOrderV3Error::ArithmeticOverflow)?;
    let buyer_account_id = trade.buyer_account_id().to_string();
    let seller_account_id = trade.seller_account_id().to_string();

    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: BalanceLedgerOperation::CreditAvailable,
            entry_id: concat3("balance-ledger:", context.settlement_id, ":buyer-base"),
            account_id: buyer_account_id.clone(),
            asset_id: context.base_asset_id.to_string(),
            amount: trade.qty,
            reason: buyer_receive_base_reason,
        },
    )?;
    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: BalanceLedgerOperation::DebitFrozen,
            entry_id: concat3("balance-ledger:", context.settlement_id, ":buyer-quote"),
            account_id: buyer_account_id,
            asset_id: context.quote_asset_id.to_string(),
            amount: quote_notional,
            reason: buyer_release_quote_reason,
        },
    )?;
    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: BalanceLedgerOperation::CreditAvailable,
            entry_id: concat3("balance-ledger:", context.settlement_id, ":seller-quote"),
            account_id: seller_account_id.clone(),
            asset_id: context.quote_asset_id.to_string(),
            amount: quote_notional,
            reason: seller_receive_quote_reason,
        },
    )?;
    push_applied_balance_ledger_entry(
        context.balance_book,
        context.ledger_entries,
        BalanceLedgerDraft {
            operation: BalanceLedgerOperation::DebitFrozen,
            entry_id: concat3("balance-ledger:", context.settlement_id, ":seller-base"),
            account_id: seller_account_id,
            asset_id: context.base_asset_id.to_string(),
            amount: trade.qty,
            reason: seller_release_base_reason,
        },
    )?;
    if let Some((buyer_fee_account_id, buyer_fee_amount)) = fee_payer_for_buyer(trade) {
        let reason = BalanceLedgerReason::SettleSpotTrade {
            trade_id: trade.trade_id.clone(),
            match_id: trade.match_id.clone(),
            settlement_batch_id: context.settlement_id.to_string(),
            purpose: SettlementTransferPurpose::TradingFee,
        };
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: BalanceLedgerOperation::DebitFrozen,
                entry_id: concat3("balance-ledger:", context.settlement_id, ":buyer-fee"),
                account_id: buyer_fee_account_id,
                asset_id: context.quote_asset_id.to_string(),
                amount: buyer_fee_amount,
                reason: reason.clone(),
            },
        )?;
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: BalanceLedgerOperation::CreditAvailable,
                entry_id: concat3("balance-ledger:", context.settlement_id, ":buyer-fee-recv"),
                account_id: context.fee_account_id.to_string(),
                asset_id: context.quote_asset_id.to_string(),
                amount: buyer_fee_amount,
                reason,
            },
        )?;
    }
    if let Some((seller_fee_account_id, seller_fee_amount)) = fee_payer_for_seller(trade) {
        let reason = BalanceLedgerReason::SettleSpotTrade {
            trade_id: trade.trade_id.clone(),
            match_id: trade.match_id.clone(),
            settlement_batch_id: context.settlement_id.to_string(),
            purpose: SettlementTransferPurpose::TradingFee,
        };
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: BalanceLedgerOperation::DebitFrozen,
                entry_id: concat3("balance-ledger:", context.settlement_id, ":seller-fee"),
                account_id: seller_fee_account_id,
                asset_id: context.quote_asset_id.to_string(),
                amount: seller_fee_amount,
                reason: reason.clone(),
            },
        )?;
        push_applied_balance_ledger_entry(
            context.balance_book,
            context.ledger_entries,
            BalanceLedgerDraft {
                operation: BalanceLedgerOperation::CreditAvailable,
                entry_id: concat3("balance-ledger:", context.settlement_id, ":seller-fee-recv"),
                account_id: context.fee_account_id.to_string(),
                asset_id: context.quote_asset_id.to_string(),
                amount: seller_fee_amount,
                reason,
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

fn map_reservation_error_to_family(error: crate::ReservationError) -> MatchSpotOrderV3Error {
    match error {
        crate::ReservationError::ArithmeticOverflow => MatchSpotOrderV3Error::ArithmeticOverflow,
        crate::ReservationError::AmountExceedsRemaining => {
            MatchSpotOrderV3Error::InsufficientFrozenBalance
        }
        crate::ReservationError::AlreadyClosed
        | crate::ReservationError::InvalidAmount
        | crate::ReservationError::InvalidOriginalAmount
        | crate::ReservationError::MissingCloseReason => {
            MatchSpotOrderV3Error::ReservationKindMismatch
        }
    }
}

fn zip_pairs<T: PartialEq>(
    before: Vec<T>,
    after: Vec<T>,
) -> Result<Vec<UpdatedEntityPair<T>>, MatchSpotOrderV3Error> {
    if before.len() != after.len() {
        return Err(MatchSpotOrderV3Error::ReservationCountMismatch);
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
    before: Vec<Balance>,
    after: Vec<Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, MatchSpotOrderV3Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
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
    updated_balances: &[UpdatedEntityPair<Balance>],
    ledger_entries: &[BalanceLedgerEntryV2],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut current_balances = HashMap::<String, Balance>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.entity_id();
        current_balances.insert(balance_id, balance.before.clone());
    }

    let mut events = Vec::with_capacity(ledger_entries.len());
    for entry in ledger_entries {
        let Some(before) = current_balances.get(&entry.balance_entity_id).cloned() else {
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
        if before.available != entry_before_available || before.frozen != entry_before_frozen {
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry breaks balance replay chain".to_string(),
            ));
        }
        let next_version = before
            .version
            .checked_add(1)
            .ok_or(common_entity::EntityError::VersionOverflow { version: before.version })?;
        let after = Balance::new(
            before.account_id.clone(),
            before.asset_id.clone(),
            entry_after_available,
            entry_after_frozen,
            next_version,
        );
        events.push(after.track_update_event_from(&before)?);
        current_balances.insert(entry.balance_entity_id.clone(), after);
    }

    for balance in updated_balances {
        let balance_id = balance.after.entity_id();
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
    pub settlement_balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderV3UseCase;

impl StateMachineV2Unchecked for MatchSpotOrderV3UseCase {
    type Command = MatchSpotOrderV3Cmd;
    type StateGiven = MatchSpotOrderV3State;
    type Error = MatchSpotOrderV3Error;
    type StateChanged = MatchSpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(MatchSpotOrderV3Error::InvalidPartyId);
        }
        if cmd.order_id.is_empty() {
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
        validate_all_reservations_for_order(
            &state.taker_order,
            &state.base_asset_id,
            &state.quote_asset_id,
        )?;
        for maker in &state.maker_orders {
            maker.ensure_matchable()?;
            validate_all_reservations_for_order(
                maker,
                &state.base_asset_id,
                &state.quote_asset_id,
            )?;
        }
        state.taker_order.ensure_matchable()?;
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        _cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let after = compute_active_order_after(ActiveOrderAfterInput {
            taker_after: state.taker_order.clone(),
            maker_orders_after: state.maker_orders.clone(),
            balance_book: BalanceMap::new(&state.settlement_balances),
            created_balance_ledger_entries: Vec::with_capacity(0),
            base_asset_id: &state.base_asset_id,
            quote_asset_id: &state.quote_asset_id,
            fee_account_id: &state.fee_account_id,
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
            executed_at_ms: context.execution_time_ns / 1_000_000,
            timestamp: context.execution_time_ns,
        })?;

        Ok(classify_after(after))
    }
}

impl StateMachineOwnedV2Diff for MatchSpotOrderV3UseCase {
    type StateDiff = MatchSpotOrderV3Changes;

    fn do_compute_state_diff(
        state: MatchSpotOrderV3State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        match after {
            MatchSpotOrderV3AfterChanges::Resting { .. } => Ok(MatchSpotOrderV3Changes::Resting),
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
                created_vouchers,
                created_balance_ledger_entries,
            } => matched_after_to_changes(
                state,
                MatchSpotOrderV3MatchedKind::PartiallyFilled,
                taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            ),
            MatchSpotOrderV3AfterChanges::Filled {
                filled_taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            } => matched_after_to_changes(
                state,
                MatchSpotOrderV3MatchedKind::Filled,
                filled_taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            ),
            MatchSpotOrderV3AfterChanges::CanceledAfterPartialFill {
                canceled_taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            } => matched_after_to_changes(
                state,
                MatchSpotOrderV3MatchedKind::CanceledAfterPartialFill,
                canceled_taker_order_after,
                maker_orders_after,
                balances_after,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            ),
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "the three matched outcomes share one complete settlement fact set"
)]
fn matched_after_to_changes(
    state: MatchSpotOrderV3State,
    kind: MatchSpotOrderV3MatchedKind,
    taker_order_after: SpotOrderV2,
    maker_orders_after: Vec<SpotOrderV2>,
    balances_after: Vec<Balance>,
    created_trades: Vec<SpotTrade>,
    created_vouchers: Vec<SettlementTransferVoucher>,
    created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
) -> Result<MatchSpotOrderV3Changes, MatchSpotOrderV3Error> {
    if created_trades.is_empty() {
        return Err(MatchSpotOrderV3Error::OrderMatch(SpotOrderV2MatchError::NoTradesMatched));
    }
    let updated_taker_order =
        UpdatedEntityPair { before: state.taker_order, after: taker_order_after };
    let updated_maker_orders = zip_pairs(state.maker_orders, maker_orders_after)?;
    let updated_balances = merge_balance_pairs(state.settlement_balances, balances_after)?;
    Ok(match kind {
        MatchSpotOrderV3MatchedKind::PartiallyFilled => MatchSpotOrderV3Changes::PartiallyFilled {
            updated_taker_order,
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        },
        MatchSpotOrderV3MatchedKind::Filled => MatchSpotOrderV3Changes::Filled {
            filled_taker_order: updated_taker_order,
            updated_maker_orders,
            updated_balances,
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        },
        MatchSpotOrderV3MatchedKind::CanceledAfterPartialFill => {
            MatchSpotOrderV3Changes::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order,
                updated_maker_orders,
                updated_balances,
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
            }
        }
    })
}

impl MatchSpotOrderV3Changes {
    pub fn taker_order_after(&self) -> Option<&SpotOrderV2> {
        match self {
            Self::Resting => None,
            Self::PartiallyFilled { updated_taker_order, .. }
            | Self::Filled { filled_taker_order: updated_taker_order, .. }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order, ..
            }
            | Self::Rejected { rejected_taker_order: updated_taker_order, .. } => {
                Some(&updated_taker_order.after)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchSpotOrderV3MatchedKind {
    PartiallyFilled,
    Filled,
    CanceledAfterPartialFill,
}

pub(super) struct BalanceMap {
    balances: HashMap<String, Balance>,
}

impl BalanceMap {
    pub(super) fn new(balances: &[Balance]) -> Self {
        Self {
            balances: balances
                .iter()
                .cloned()
                .map(|balance| (balance.entity_id(), balance))
                .collect(),
        }
    }

    pub(super) fn get_mut(
        &mut self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<&mut Balance, MatchSpotOrderV3Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(MatchSpotOrderV3Error::BalanceNotFound)
    }

    pub(super) fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
}
