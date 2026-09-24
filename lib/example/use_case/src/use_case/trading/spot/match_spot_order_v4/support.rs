use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{Entity, EntityReplayableEvent};
use spot_entity::spot_order_v2::{SpotOrderV2, SpotOrderV2BehaviorError, SpotOrderV2MatchError};
use thiserror::Error;

use crate::SpotTrade;
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

pub type SettleMatchedSpotTradesV4Error = MatchSpotOrderV4Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderV4Error {
    #[error("party id must not be empty")]
    InvalidPartyId,
    #[error("order oid must be greater than zero")]
    InvalidOrderId,
    #[error("settlement id prefix must not be empty")]
    InvalidSettlementIdPrefix,
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
    #[error("arithmetic overflow while computing spot order v4 family")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderMatch(#[from] SpotOrderV2MatchError),
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
}

pub(super) fn validate_all_reservations_for_order(
    order: &SpotOrderV2,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), MatchSpotOrderV4Error> {
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
) -> Result<(), MatchSpotOrderV4Error> {
    if reservation.market_kind != ReservationMarketKind::Spot {
        return Err(MatchSpotOrderV4Error::ReservationKindMismatch);
    }
    if reservation.caused_by_order_id != order.order_id() {
        return Err(MatchSpotOrderV4Error::ReservationOrderMismatch);
    }
    if reservation.reservation_kind != expected_kind {
        return Err(MatchSpotOrderV4Error::ReservationKindMismatch);
    }
    let is_zero_fee_reservation = matches!(
        expected_kind,
        ReservationKind::SpotBuyFeeQuote | ReservationKind::SpotSellFeeQuote
    ) && reservation.original_amount == 0
        && reservation.remaining_amount == 0;
    if !reservation.is_active() && !is_zero_fee_reservation {
        return Err(MatchSpotOrderV4Error::ReservationKindMismatch);
    }
    let expected_asset = match expected_kind {
        ReservationKind::SpotBuyQuote
        | ReservationKind::SpotBuyFeeQuote
        | ReservationKind::SpotSellFeeQuote => quote_asset_id,
        ReservationKind::SpotSellBase => base_asset_id,
        _ => return Err(MatchSpotOrderV4Error::ReservationKindMismatch),
    };
    if reservation.asset_id != expected_asset {
        return Err(MatchSpotOrderV4Error::ReservationAssetMismatch);
    }
    Ok(())
}

pub(super) fn zip_pairs<T: PartialEq>(
    before: Vec<T>,
    after: Vec<T>,
) -> Result<Vec<UpdatedEntityPair<T>>, MatchSpotOrderV4Error> {
    if before.len() != after.len() {
        return Err(MatchSpotOrderV4Error::ReservationCountMismatch);
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
) -> Result<Vec<UpdatedEntityPair<Balance>>, MatchSpotOrderV4Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
        let before_balance =
            before_map.get(&balance_id).cloned().ok_or(MatchSpotOrderV4Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(MatchSpotOrderV4Error::BalanceNotFound);
        }
        if before_balance != balance {
            pairs.push(UpdatedEntityPair { before: before_balance, after: balance });
        }
    }
    Ok(pairs)
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
    ) -> Result<&mut Balance, MatchSpotOrderV4Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(MatchSpotOrderV4Error::BalanceNotFound)
    }

    pub(super) fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
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

pub(super) struct MatchedTradeSettlementContext<'a> {
    pub(super) settlement_id_prefix: &'a str,
    pub(super) base_asset_id: &'a str,
    pub(super) quote_asset_id: &'a str,
    pub(super) fee_account_id: &'a str,
    pub(super) balance_book: &'a mut BalanceMap,
    pub(super) ledger_entries: &'a mut Vec<BalanceLedgerEntryV2>,
}

pub(super) struct MatchedTradeSettlementEffects {
    pub(super) created_vouchers: Vec<SettlementTransferVoucher>,
}

pub(super) fn settle_matched_trades(
    taker_order: &mut SpotOrderV2,
    maker_orders: &mut [SpotOrderV2],
    trades: &[SpotTrade],
    context: MatchedTradeSettlementContext<'_>,
) -> Result<MatchedTradeSettlementEffects, MatchSpotOrderV4Error> {
    let MatchedTradeSettlementContext {
        settlement_id_prefix,
        base_asset_id,
        quote_asset_id,
        fee_account_id,
        balance_book,
        ledger_entries,
    } = context;
    let mut created_vouchers = Vec::with_capacity(trades.len());

    for (index, trade) in trades.iter().enumerate() {
        let trade_notional =
            trade.notional_quote().ok_or(MatchSpotOrderV4Error::ArithmeticOverflow)?;
        let taker_principal_consume =
            principal_consume_amount_for_order(taker_order, trade.qty, trade_notional);
        consume_reservation(
            &mut taker_order.reservation,
            taker_principal_consume,
            ReservationCloseReason::Filled,
        )?;
        let Some(maker_order) = maker_orders.get_mut(index) else {
            return Err(MatchSpotOrderV4Error::ReservationCountMismatch);
        };
        let maker_principal_consume =
            principal_consume_amount_for_order(maker_order, trade.qty, trade_notional);
        consume_reservation(
            &mut maker_order.reservation,
            maker_principal_consume,
            ReservationCloseReason::Filled,
        )?;
        consume_reservation(
            &mut taker_order.fee_reservation,
            trade.taker_fee,
            ReservationCloseReason::Filled,
        )?;
        consume_reservation(
            &mut maker_order.fee_reservation,
            trade.maker_fee,
            ReservationCloseReason::Filled,
        )?;

        let settlement_id = settlement_id_for_trade(settlement_id_prefix, &trade.trade_id);
        let voucher = trade
            .derive_spot_settlement_transfer_voucher_with_fees(
                concat2("spot-voucher:", trade.trade_id.as_str()),
                settlement_id.clone(),
                base_asset_id,
                quote_asset_id,
                fee_account_id.to_string(),
            )
            .ok_or(MatchSpotOrderV4Error::ArithmeticOverflow)?;

        apply_trade_balance_effects(
            trade,
            TradeBalanceEffectsContext {
                settlement_id: &settlement_id,
                base_asset_id,
                quote_asset_id,
                fee_account_id,
                balance_book,
                ledger_entries,
            },
        )?;
        created_vouchers.push(voucher);
    }

    Ok(MatchedTradeSettlementEffects { created_vouchers })
}

fn settlement_id_for_trade(settlement_id_prefix: &str, trade_id: &str) -> String {
    concat3(settlement_id_prefix, ":", trade_id)
}

fn principal_consume_amount_for_order(
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
) -> Result<(), MatchSpotOrderV4Error> {
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

pub(super) fn release_remaining_for_terminal(
    order_after: &mut SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), MatchSpotOrderV4Error> {
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
) -> Result<(), MatchSpotOrderV4Error> {
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
) -> Result<(), MatchSpotOrderV4Error> {
    let reason = match order.side() {
        SpotOrderSide::Buy => {
            BalanceLedgerReason::CancelSpotOrderReleaseQuote { order_id: order.order_id() }
        }
        SpotOrderSide::Sell => {
            BalanceLedgerReason::CancelSpotOrderReleaseBase { order_id: order.order_id() }
        }
    };
    let balance = balance_book.get_mut(order.account_id(), asset_id)?;
    let next_release_index =
        ledger_entries.len().checked_add(1).ok_or(MatchSpotOrderV4Error::ArithmeticOverflow)?;
    let entry = apply_balance_ledger_entry(
        BalanceLedgerOperation::Unfreeze,
        concat4(
            "balance-ledger:",
            order.order_id().to_string().as_str(),
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
) -> Result<(), MatchSpotOrderV4Error> {
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
) -> Result<(), MatchSpotOrderV4Error> {
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

    let quote_notional = trade.notional_quote().ok_or(MatchSpotOrderV4Error::ArithmeticOverflow)?;
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

fn map_reservation_error_to_family(error: crate::ReservationError) -> MatchSpotOrderV4Error {
    match error {
        crate::ReservationError::ArithmeticOverflow => MatchSpotOrderV4Error::ArithmeticOverflow,
        crate::ReservationError::AmountExceedsRemaining => {
            MatchSpotOrderV4Error::InsufficientFrozenBalance
        }
        crate::ReservationError::AlreadyClosed
        | crate::ReservationError::InvalidAmount
        | crate::ReservationError::InvalidOriginalAmount
        | crate::ReservationError::MissingCloseReason => {
            MatchSpotOrderV4Error::ReservationKindMismatch
        }
    }
}
