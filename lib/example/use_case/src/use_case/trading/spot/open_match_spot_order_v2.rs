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
    SettlementTransferVoucher, SpotOrderSide, spot as spot_entity,
};
use crate::support::{concat2, concat3, concat4};
use crate::{MatchSpotOrderV2Input, SpotTrade};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderV2AfterChanges {
    pub taker_order_after: SpotOrderV2,
    pub maker_orders_after: Vec<SpotOrderV2>,
    pub balances_after: Vec<Balance>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderV2Changes {
    pub updated_taker_order: Option<UpdatedEntityPair<SpotOrderV2>>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderV2Error {
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
    #[error("arithmetic overflow while computing spot order v2 family")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderMatch(#[from] SpotOrderV2MatchError),
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
}

impl ReplayableChanges for MatchSpotOrderV2Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let event_capacity = usize::from(self.updated_taker_order.is_some())
            .saturating_add(self.created_trades.len())
            .saturating_add(self.updated_maker_orders.len())
            .saturating_add(self.created_vouchers.len())
            .saturating_add(self.created_balance_ledger_entries.len())
            .saturating_add(self.created_balance_ledger_entries.len());
        let mut events = Vec::with_capacity(event_capacity);
        if let Some(updated_taker_order) = &self.updated_taker_order {
            events.push(
                updated_taker_order.after.track_update_event_from(&updated_taker_order.before)?,
            );
        }
        for trade in &self.created_trades {
            events.push(trade.track_create_event()?);
        }
        for maker in &self.updated_maker_orders {
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        for voucher in &self.created_vouchers {
            events.push(voucher.track_create_event()?);
        }
        events.extend(balance_replay_events_from_ledger_entries(
            &self.updated_balances,
            &self.created_balance_ledger_entries,
        )?);
        for entry in &self.created_balance_ledger_entries {
            events.push(entry.track_create_event()?);
        }
        Ok(events)
    }
}

struct ActiveOrderAfterContext<'a> {
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

fn compute_active_order_after(
    context: ActiveOrderAfterContext<'_>,
) -> Result<ActiveOrderAfter, MatchSpotOrderV2Error> {
    let ActiveOrderAfterContext {
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
    } = context;
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
            trade.notional_quote().ok_or(MatchSpotOrderV2Error::ArithmeticOverflow)?;

        total_taker_fill = total_taker_fill
            .checked_add(trade.qty)
            .ok_or(MatchSpotOrderV2Error::ArithmeticOverflow)?;

        let taker_principal_consume =
            principal_consume_amount_for_taker(&taker_after, trade.qty, trade_notional);
        consume_reservation(
            &mut taker_after.reservation,
            taker_principal_consume,
            ReservationCloseReason::Filled,
        )?;
        let Some(maker_order_after) = maker_orders_after.get_mut(index) else {
            return Err(MatchSpotOrderV2Error::BalanceNotFound);
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
            .ok_or(MatchSpotOrderV2Error::ArithmeticOverflow)?;

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
) -> Result<(), MatchSpotOrderV2Error> {
    if reservation.market_kind != ReservationMarketKind::Spot {
        return Err(MatchSpotOrderV2Error::ReservationKindMismatch);
    }
    if reservation.caused_by_order_id != order.order_id() {
        return Err(MatchSpotOrderV2Error::ReservationOrderMismatch);
    }
    if reservation.reservation_kind != expected_kind {
        return Err(MatchSpotOrderV2Error::ReservationKindMismatch);
    }
    let expected_asset = match expected_kind {
        ReservationKind::SpotBuyQuote
        | ReservationKind::SpotBuyFeeQuote
        | ReservationKind::SpotSellFeeQuote => quote_asset_id,
        ReservationKind::SpotSellBase => base_asset_id,
        _ => return Err(MatchSpotOrderV2Error::ReservationKindMismatch),
    };
    if reservation.asset_id != expected_asset {
        return Err(MatchSpotOrderV2Error::ReservationAssetMismatch);
    }
    Ok(())
}

pub(super) fn validate_all_reservations_for_order(
    order: &SpotOrderV2,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), MatchSpotOrderV2Error> {
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
) -> Result<(), MatchSpotOrderV2Error> {
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
) -> Result<(), MatchSpotOrderV2Error> {
    let requirements = order_after.terminal_release_requirements(maker_fee_bps, taker_fee_bps);

    if let Some(requirement) = requirements.principal {
        release_order_reservation(
            order_after,
            OrderReservationSlot::Principal,
            requirement.amount,
            reservation_close_reason_for_order_release(requirement.reason),
            balance_book,
            ledger_entries,
        )?;
    }

    if let Some(requirement) = requirements.fee {
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
) -> Result<(), MatchSpotOrderV2Error> {
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
) -> Result<(), MatchSpotOrderV2Error> {
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
        ledger_entries.len().checked_add(1).ok_or(MatchSpotOrderV2Error::ArithmeticOverflow)?;
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
) -> Result<(), MatchSpotOrderV2Error> {
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
) -> Result<(), MatchSpotOrderV2Error> {
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

    let quote_notional = trade.notional_quote().ok_or(MatchSpotOrderV2Error::ArithmeticOverflow)?;
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

fn map_reservation_error_to_family(error: crate::ReservationError) -> MatchSpotOrderV2Error {
    match error {
        crate::ReservationError::ArithmeticOverflow => MatchSpotOrderV2Error::ArithmeticOverflow,
        crate::ReservationError::AmountExceedsRemaining => {
            MatchSpotOrderV2Error::InsufficientFrozenBalance
        }
        crate::ReservationError::AlreadyClosed
        | crate::ReservationError::InvalidAmount
        | crate::ReservationError::InvalidOriginalAmount
        | crate::ReservationError::MissingCloseReason => {
            MatchSpotOrderV2Error::ReservationKindMismatch
        }
    }
}

fn zip_pairs<T>(
    before: Vec<T>,
    after: Vec<T>,
) -> Result<Vec<UpdatedEntityPair<T>>, MatchSpotOrderV2Error> {
    if before.len() != after.len() {
        return Err(MatchSpotOrderV2Error::ReservationCountMismatch);
    }
    Ok(before
        .into_iter()
        .zip(after)
        .map(|(before, after)| UpdatedEntityPair { before, after })
        .collect())
}

pub(super) fn merge_balance_pairs(
    before: Vec<Balance>,
    after: Vec<Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, MatchSpotOrderV2Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
        let before_balance =
            before_map.get(&balance_id).cloned().ok_or(MatchSpotOrderV2Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(MatchSpotOrderV2Error::BalanceNotFound);
        }
        pairs.push(UpdatedEntityPair { before: before_balance, after: balance });
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
pub struct MatchSpotOrderV2State {
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
pub struct OpenMatchSpotOrderV2UseCase;

impl StateMachineV2Unchecked for OpenMatchSpotOrderV2UseCase {
    type Command = MatchSpotOrderV2Cmd;
    type StateGiven = MatchSpotOrderV2State;
    type Error = MatchSpotOrderV2Error;
    type StateChanged = MatchSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(MatchSpotOrderV2Error::InvalidPartyId);
        }
        if cmd.order_id.is_empty() {
            return Err(MatchSpotOrderV2Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.taker_order.order_id() != cmd.order_id {
            return Err(MatchSpotOrderV2Error::TakerOrderIdMismatch);
        }
        if state.taker_order.account_id() != cmd.party_id {
            return Err(MatchSpotOrderV2Error::TakerAccountIdMismatch);
        }
        if state.taker_order.asset() != cmd.asset {
            return Err(MatchSpotOrderV2Error::TakerAssetMismatch);
        }
        if state.fee_account_id.is_empty() {
            return Err(MatchSpotOrderV2Error::InvalidFeeAccountId);
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
        state.taker_order.ensure_matchable()?;
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        _cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut balance_book = BalanceMap::new(&state.settlement_balances);
        let mut created_balance_ledger_entries = Vec::new();
        let freeze_ledger_entry = apply_freeze_for_open_taker_reservation(
            &state.taker_order,
            concat2("balance-ledger:freeze:", state.taker_order.order_id()),
            &state.taker_order.reservation.asset_id,
            state.taker_order.reservation.original_amount,
            &mut balance_book,
        )?;
        created_balance_ledger_entries.push(freeze_ledger_entry);
        if state.taker_order.fee_reservation.original_amount > 0 {
            let fee_freeze_ledger_entry = apply_freeze_for_open_taker_reservation(
                &state.taker_order,
                concat3("balance-ledger:freeze:", state.taker_order.order_id(), ":fee"),
                &state.taker_order.fee_reservation.asset_id,
                state.taker_order.fee_reservation.original_amount,
                &mut balance_book,
            )?;
            created_balance_ledger_entries.push(fee_freeze_ledger_entry);
        }

        let after = compute_active_order_after(ActiveOrderAfterContext {
            taker_after: state.taker_order.clone(),
            maker_orders_after: state.maker_orders.clone(),
            balance_book,
            created_balance_ledger_entries,
            base_asset_id: &state.base_asset_id,
            quote_asset_id: &state.quote_asset_id,
            fee_account_id: &state.fee_account_id,
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
            executed_at_ms: context.execution_time_ns / 1_000_000,
            timestamp: context.execution_time_ns,
        })?;

        Ok(MatchSpotOrderV2AfterChanges {
            taker_order_after: after.taker_order_after,
            maker_orders_after: after.maker_orders_after,
            balances_after: after.balances_after,
            created_trades: after.created_trades,
            created_vouchers: after.created_vouchers,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
    }
}

impl StateMachineOwnedV2Diff for OpenMatchSpotOrderV2UseCase {
    type StateDiff = MatchSpotOrderV2Changes;

    fn do_compute_state_diff(
        state: MatchSpotOrderV2State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        let updated_taker_order =
            (state.taker_order != after.taker_order_after).then(|| UpdatedEntityPair {
                before: state.taker_order.clone(),
                after: after.taker_order_after.clone(),
            });
        Ok(MatchSpotOrderV2Changes {
            updated_taker_order,
            updated_maker_orders: zip_pairs(state.maker_orders, after.maker_orders_after)?,
            updated_balances: merge_balance_pairs(state.settlement_balances, after.balances_after)?,
            created_trades: after.created_trades,
            created_vouchers: after.created_vouchers,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
    }
}

impl MatchSpotOrderV2Changes {
    pub fn taker_order_after(&self) -> Option<&SpotOrderV2> {
        self.updated_taker_order.as_ref().map(|pair| &pair.after)
    }
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
    ) -> Result<&mut Balance, MatchSpotOrderV2Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(MatchSpotOrderV2Error::BalanceNotFound)
    }

    pub(super) fn entity_id_for_account_asset(
        &self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<String, MatchSpotOrderV2Error> {
        self.balances
            .get(&concat3(account_id, ":", asset_id))
            .map(Entity::entity_id)
            .ok_or(MatchSpotOrderV2Error::BalanceNotFound)
    }

    pub(super) fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
}

fn apply_freeze_for_open_taker_reservation(
    order: &SpotOrderV2,
    entry_id: String,
    asset_id: &str,
    amount: u64,
    balance_book: &mut BalanceMap,
) -> Result<BalanceLedgerEntryV2, MatchSpotOrderV2Error> {
    let mut entry = BalanceLedgerEntryV2::freeze(
        entry_id,
        order.account_id().to_string(),
        asset_id.to_string(),
        balance_book.entity_id_for_account_asset(order.account_id(), asset_id)?,
        amount,
        BalanceLedgerReason::FreezeForOrder { order_id: order.order_id().to_string() },
    )?;
    let balance = balance_book.get_mut(order.account_id(), asset_id)?;
    entry.apply_to(balance)?;
    Ok(entry)
}
