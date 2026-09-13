use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2Unchecked,
    ReplayableChanges,
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
use crate::{
    MatchSpotOrderV2Input, PlaceSpotOrderV2Input, SpotOrderExecution, SpotOrderTimeInForce,
    SpotTrade,
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlaceSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub tif: String,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2TakerTemplateContext<'a> {
    pub order_id: String,
    pub symbol: String,
    pub settlement_balances: &'a [Balance],
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2AfterChanges {
    pub taker_order_initial: SpotOrderV2,
    pub taker_order_after: SpotOrderV2,
    pub maker_orders_after: Vec<SpotOrderV2>,
    pub balances_after: Vec<Balance>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2Changes {
    pub updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceSpotOrderV2Error {
    #[error("price must be a positive integer string")]
    InvalidPrice,
    #[error("size must be a positive integer string")]
    InvalidSize,
    #[error("time in force must be gtc, ioc, or alo")]
    InvalidTimeInForce,
    #[error("order template does not match command-derived order")]
    OrderTemplateMismatch,
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

impl ReplayableChanges for PlaceSpotOrderV2Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let event_capacity = self
            .created_trades
            .len()
            .saturating_add(self.updated_maker_orders.len())
            .saturating_add(1)
            .saturating_add(self.created_vouchers.len())
            .saturating_add(self.created_balance_ledger_entries.len())
            .saturating_add(self.created_balance_ledger_entries.len());
        let mut events = Vec::with_capacity(event_capacity);
        for trade in &self.created_trades {
            events.push(trade.track_create_event()?);
        }
        for maker in &self.updated_maker_orders {
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        if self.updated_taker_order.before == self.updated_taker_order.after {
            events.push(self.updated_taker_order.after.track_create_event()?);
        } else {
            events.push(
                self.updated_taker_order
                    .after
                    .track_update_event_from(&self.updated_taker_order.before)?,
            );
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

struct PlaceAfterContext<'a> {
    cmd: &'a PlaceSpotOrderV2Cmd,
    order_id: &'a str,
    symbol: &'a str,
    maker_orders: &'a [SpotOrderV2],
    settlement_balances: &'a [Balance],
    base_asset_id: &'a str,
    quote_asset_id: &'a str,
    fee_account_id: &'a str,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
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
}

struct ActiveOrderAfter {
    taker_order_after: SpotOrderV2,
    maker_orders_after: Vec<SpotOrderV2>,
    balances_after: Vec<Balance>,
    created_trades: Vec<SpotTrade>,
    created_vouchers: Vec<SettlementTransferVoucher>,
    created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

fn compute_place_after(
    context: PlaceAfterContext<'_>,
) -> Result<PlaceSpotOrderV2AfterChanges, PlaceSpotOrderV2Error> {
    let maker_orders_after = context.maker_orders.to_vec();
    let mut balance_book = BalanceMap::new(context.settlement_balances);
    let place_input = place_input_from_context(
        context.cmd,
        &PlaceSpotOrderV2TakerTemplateContext {
            order_id: context.order_id.to_string(),
            symbol: context.symbol.to_string(),
            settlement_balances: context.settlement_balances,
            base_asset_id: context.base_asset_id.to_string(),
            quote_asset_id: context.quote_asset_id.to_string(),
            maker_fee_bps: context.maker_fee_bps,
            taker_fee_bps: context.taker_fee_bps,
        },
    )?;
    let place_outcome = SpotOrderV2::place(place_input)?;
    let taker_initial = place_outcome.order.clone();
    let taker_after = place_outcome.order;
    let mut created_balance_ledger_entries = Vec::with_capacity(1);
    let freeze_ledger_entry =
        apply_behavior_ledger_entry(place_outcome.freeze_ledger_entry, &mut balance_book)?;
    created_balance_ledger_entries.push(freeze_ledger_entry);
    let after = compute_active_order_after(ActiveOrderAfterContext {
        taker_after,
        maker_orders_after,
        balance_book,
        created_balance_ledger_entries,
        base_asset_id: context.base_asset_id,
        quote_asset_id: context.quote_asset_id,
        fee_account_id: context.fee_account_id,
        maker_fee_bps: context.maker_fee_bps,
        taker_fee_bps: context.taker_fee_bps,
    })?;

    Ok(PlaceSpotOrderV2AfterChanges {
        taker_order_initial: taker_initial,
        taker_order_after: after.taker_order_after,
        maker_orders_after: after.maker_orders_after,
        balances_after: after.balances_after,
        created_trades: after.created_trades,
        created_vouchers: after.created_vouchers,
        created_balance_ledger_entries: after.created_balance_ledger_entries,
    })
}

fn compute_active_order_after(
    context: ActiveOrderAfterContext<'_>,
) -> Result<ActiveOrderAfter, PlaceSpotOrderV2Error> {
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
            taker_after.reject_as_bad_alo()?;
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
        },
    )?;
    let mut total_taker_fill = 0_u64;
    for (index, trade) in match_outcome.trades.into_iter().enumerate() {
        let trade_notional =
            trade.notional_quote().ok_or(PlaceSpotOrderV2Error::ArithmeticOverflow)?;

        total_taker_fill = total_taker_fill
            .checked_add(trade.qty)
            .ok_or(PlaceSpotOrderV2Error::ArithmeticOverflow)?;

        let taker_principal_consume =
            principal_consume_amount_for_taker(&taker_after, trade.qty, trade_notional);
        consume_reservation(
            &mut taker_after.reservation,
            taker_principal_consume,
            ReservationCloseReason::Filled,
        )?;
        let Some(maker_order_after) = maker_orders_after.get_mut(index) else {
            return Err(PlaceSpotOrderV2Error::BalanceNotFound);
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
            .ok_or(PlaceSpotOrderV2Error::ArithmeticOverflow)?;

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
    taker_after.finish_after_match(total_taker_fill)?;
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

fn parse_positive_u64(
    raw: &str,
    error: PlaceSpotOrderV2Error,
) -> Result<u64, PlaceSpotOrderV2Error> {
    let value = raw.parse::<u64>().map_err(|_| error.clone())?;
    if value == 0 {
        return Err(error);
    }
    Ok(value)
}

fn parse_tif(raw: &str) -> Result<SpotOrderTimeInForce, PlaceSpotOrderV2Error> {
    match raw {
        "gtc" | "Gtc" => Ok(SpotOrderTimeInForce::Gtc),
        "ioc" | "Ioc" => Ok(SpotOrderTimeInForce::Ioc),
        "alo" | "Alo" => Ok(SpotOrderTimeInForce::Alo),
        _ => Err(PlaceSpotOrderV2Error::InvalidTimeInForce),
    }
}

fn place_input_from_context(
    cmd: &PlaceSpotOrderV2Cmd,
    context: &PlaceSpotOrderV2TakerTemplateContext<'_>,
) -> Result<PlaceSpotOrderV2Input, PlaceSpotOrderV2Error> {
    let side = if cmd.is_buy { SpotOrderSide::Buy } else { SpotOrderSide::Sell };
    let price = parse_positive_u64(&cmd.price, PlaceSpotOrderV2Error::InvalidPrice)?;
    let qty = parse_positive_u64(&cmd.size, PlaceSpotOrderV2Error::InvalidSize)?;
    let base_balance_entity_id = balance_entity_id_for_account_asset(
        context.settlement_balances,
        &cmd.party_id,
        &context.base_asset_id,
    )?;
    let quote_balance_entity_id = balance_entity_id_for_account_asset(
        context.settlement_balances,
        &cmd.party_id,
        &context.quote_asset_id,
    )?;

    Ok(PlaceSpotOrderV2Input {
        order_id: context.order_id.clone(),
        asset: cmd.asset,
        account_id: cmd.party_id.clone(),
        symbol: context.symbol.clone(),
        side,
        execution: SpotOrderExecution::Limit { price },
        time_in_force: parse_tif(&cmd.tif)?,
        qty,
        base_asset_id: context.base_asset_id.clone(),
        quote_asset_id: context.quote_asset_id.clone(),
        base_balance_entity_id,
        quote_balance_entity_id,
        maker_fee_bps: context.maker_fee_bps,
        taker_fee_bps: context.taker_fee_bps,
        client_order_id: cmd.cloid.clone(),
    })
}

pub(super) fn balance_entity_id_for_account_asset(
    balances: &[Balance],
    account_id: &str,
    asset_id: &str,
) -> Result<String, PlaceSpotOrderV2Error> {
    balances
        .iter()
        .find(|balance| balance.account_id == account_id && balance.asset_id == asset_id)
        .map(Entity::entity_id)
        .ok_or(PlaceSpotOrderV2Error::BalanceNotFound)
}

pub(super) fn balance_entity_id_for_reservation(
    balances: &[Balance],
    reservation: &Reservation,
) -> Result<String, PlaceSpotOrderV2Error> {
    balance_entity_id_for_account_asset(
        balances,
        &reservation.owner_account_id,
        &reservation.asset_id,
    )
}

fn validate_reservation_for_order(
    order: &SpotOrderV2,
    reservation: &Reservation,
    expected_kind: ReservationKind,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), PlaceSpotOrderV2Error> {
    if reservation.market_kind != ReservationMarketKind::Spot {
        return Err(PlaceSpotOrderV2Error::ReservationKindMismatch);
    }
    if reservation.caused_by_order_id != order.order_id() {
        return Err(PlaceSpotOrderV2Error::ReservationOrderMismatch);
    }
    if reservation.reservation_kind != expected_kind {
        return Err(PlaceSpotOrderV2Error::ReservationKindMismatch);
    }
    let expected_asset = match expected_kind {
        ReservationKind::SpotBuyQuote
        | ReservationKind::SpotBuyFeeQuote
        | ReservationKind::SpotSellFeeQuote => quote_asset_id,
        ReservationKind::SpotSellBase => base_asset_id,
        _ => return Err(PlaceSpotOrderV2Error::ReservationKindMismatch),
    };
    if reservation.asset_id != expected_asset {
        return Err(PlaceSpotOrderV2Error::ReservationAssetMismatch);
    }
    Ok(())
}

pub(super) fn validate_all_reservations_for_order(
    order: &SpotOrderV2,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), PlaceSpotOrderV2Error> {
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
) -> Result<(), PlaceSpotOrderV2Error> {
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
) -> Result<(), PlaceSpotOrderV2Error> {
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

pub(super) fn release_remaining_for_cancel(
    order: &mut SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), PlaceSpotOrderV2Error> {
    if let Some(requirement) = order.fee_hold_requirement(maker_fee_bps, taker_fee_bps) {
        release_order_reservation(
            order,
            OrderReservationSlot::Fee,
            requirement.amount,
            ReservationCloseReason::Canceled,
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
) -> Result<(), PlaceSpotOrderV2Error> {
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
) -> Result<(), PlaceSpotOrderV2Error> {
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
        ledger_entries.len().checked_add(1).ok_or(PlaceSpotOrderV2Error::ArithmeticOverflow)?;
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
) -> Result<(), PlaceSpotOrderV2Error> {
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

pub(super) fn apply_behavior_ledger_entry(
    mut entry: BalanceLedgerEntryV2,
    balance_book: &mut BalanceMap,
) -> Result<BalanceLedgerEntryV2, PlaceSpotOrderV2Error> {
    let balance = balance_book.get_by_entity_id_mut(&entry.balance_entity_id)?;
    entry.apply_to(balance)?;
    Ok(entry)
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
) -> Result<(), PlaceSpotOrderV2Error> {
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

    let quote_notional = trade.notional_quote().ok_or(PlaceSpotOrderV2Error::ArithmeticOverflow)?;
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

fn map_reservation_error_to_family(error: crate::ReservationError) -> PlaceSpotOrderV2Error {
    match error {
        crate::ReservationError::ArithmeticOverflow => PlaceSpotOrderV2Error::ArithmeticOverflow,
        crate::ReservationError::AmountExceedsRemaining => {
            PlaceSpotOrderV2Error::InsufficientFrozenBalance
        }
        crate::ReservationError::AlreadyClosed
        | crate::ReservationError::InvalidAmount
        | crate::ReservationError::InvalidOriginalAmount
        | crate::ReservationError::MissingCloseReason => {
            PlaceSpotOrderV2Error::ReservationKindMismatch
        }
    }
}

fn zip_pairs<T>(
    before: Vec<T>,
    after: Vec<T>,
) -> Result<Vec<UpdatedEntityPair<T>>, PlaceSpotOrderV2Error> {
    if before.len() != after.len() {
        return Err(PlaceSpotOrderV2Error::ReservationCountMismatch);
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
) -> Result<Vec<UpdatedEntityPair<Balance>>, PlaceSpotOrderV2Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
        let before_balance =
            before_map.get(&balance_id).cloned().ok_or(PlaceSpotOrderV2Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(PlaceSpotOrderV2Error::BalanceNotFound);
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
pub struct PlaceSpotOrderV2State {
    pub order_id: String,
    pub symbol: String,
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceSpotOrderV2UseCase;

impl MiStateMachineV2Unchecked for PlaceSpotOrderV2UseCase {
    type Command = PlaceSpotOrderV2Cmd;
    type GivenState = PlaceSpotOrderV2State;
    type Error = PlaceSpotOrderV2Error;
    type AfterChanges = PlaceSpotOrderV2AfterChanges;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        parse_positive_u64(&cmd.price, PlaceSpotOrderV2Error::InvalidPrice)?;
        parse_positive_u64(&cmd.size, PlaceSpotOrderV2Error::InvalidSize)?;
        parse_tif(&cmd.tif)?;
        Ok(())
    }

    fn validate_against_given_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.fee_account_id.is_empty() {
            return Err(PlaceSpotOrderV2Error::InvalidFeeAccountId);
        }
        let taker_order = build_place_spot_order_v2_taker_template(
            cmd,
            PlaceSpotOrderV2TakerTemplateContext {
                order_id: state.order_id.clone(),
                symbol: state.symbol.clone(),
                settlement_balances: &state.settlement_balances,
                base_asset_id: state.base_asset_id.clone(),
                quote_asset_id: state.quote_asset_id.clone(),
                maker_fee_bps: state.maker_fee_bps,
                taker_fee_bps: state.taker_fee_bps,
            },
        )?;
        for maker in &state.maker_orders {
            validate_all_reservations_for_order(
                maker,
                &state.base_asset_id,
                &state.quote_asset_id,
            )?;
        }
        taker_order.ensure_matchable()?;
        Ok(())
    }

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        compute_place_after(PlaceAfterContext {
            cmd,
            order_id: &state.order_id,
            symbol: &state.symbol,
            maker_orders: &state.maker_orders,
            settlement_balances: &state.settlement_balances,
            base_asset_id: &state.base_asset_id,
            quote_asset_id: &state.quote_asset_id,
            fee_account_id: &state.fee_account_id,
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
        })
    }
}

impl MiStateMachineOwnedV2BeforeAfter for PlaceSpotOrderV2UseCase {
    type BeforeAfterChanges = PlaceSpotOrderV2Changes;

    fn merge_before_and_after(
        state: PlaceSpotOrderV2State,
        after: Self::AfterChanges,
    ) -> Result<Self::BeforeAfterChanges, Self::Error> {
        Ok(PlaceSpotOrderV2Changes {
            updated_taker_order: UpdatedEntityPair {
                before: after.taker_order_initial,
                after: after.taker_order_after,
            },
            updated_maker_orders: zip_pairs(state.maker_orders, after.maker_orders_after)?,
            updated_balances: merge_balance_pairs(state.settlement_balances, after.balances_after)?,
            created_trades: after.created_trades,
            created_vouchers: after.created_vouchers,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
    }
}

pub fn build_place_spot_order_v2_taker_template(
    cmd: &PlaceSpotOrderV2Cmd,
    context: PlaceSpotOrderV2TakerTemplateContext<'_>,
) -> Result<SpotOrderV2, PlaceSpotOrderV2Error> {
    let input = place_input_from_context(cmd, &context)?;
    Ok(SpotOrderV2::place(input)?.order)
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
    ) -> Result<&mut Balance, PlaceSpotOrderV2Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(PlaceSpotOrderV2Error::BalanceNotFound)
    }

    pub(super) fn get_by_entity_id_mut(
        &mut self,
        entity_id: &str,
    ) -> Result<&mut Balance, PlaceSpotOrderV2Error> {
        self.balances.get_mut(entity_id).ok_or(PlaceSpotOrderV2Error::BalanceNotFound)
    }

    pub(super) fn entity_id_for_account_asset(
        &self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<String, PlaceSpotOrderV2Error> {
        self.balances
            .get(&concat3(account_id, ":", asset_id))
            .map(Entity::entity_id)
            .ok_or(PlaceSpotOrderV2Error::BalanceNotFound)
    }

    pub(super) fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2};

    use super::*;
    use crate::{SpotOrderExecution, SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce};

    fn test_principal_reservation(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        qty: u64,
        order_price: u64,
    ) -> Reservation {
        match SpotOrderV2::principal_reservation(
            order_id,
            account_id,
            side,
            qty,
            order_price,
            "BTC",
            "USDT",
        ) {
            Ok(reservation) => reservation,
            Err(error) => panic!("invalid test spot order reservation: {error}"),
        }
    }

    fn buy_order(tif: SpotOrderTimeInForce) -> SpotOrderV2 {
        SpotOrderV2::place(PlaceSpotOrderV2Input {
            order_id: "taker-buy".to_string(),
            asset: 10_001,
            account_id: "buyer".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: SpotOrderSide::Buy,
            execution: SpotOrderExecution::Limit { price: 100 },
            time_in_force: tif,
            qty: 2,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            base_balance_entity_id: "buyer:BTC".to_string(),
            quote_balance_entity_id: "buyer:USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
            client_order_id: None,
        })
        .unwrap()
        .order
    }

    fn place_cmd(tif: &str) -> PlaceSpotOrderV2Cmd {
        PlaceSpotOrderV2Cmd {
            party_id: "buyer".to_string(),
            asset: 10_001,
            is_buy: true,
            price: "100".to_string(),
            size: "2".to_string(),
            tif: tif.to_string(),
            cloid: None,
        }
    }

    fn sell_order(order_id: &str, account_id: &str, price: u64, qty: u64) -> SpotOrderV2 {
        SpotOrderV2::new(
            order_id.to_string(),
            10_001,
            Some(price),
            account_id.to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            0,
            SpotOrderStatus::Open,
            None,
            test_principal_reservation(order_id, account_id, SpotOrderSide::Sell, qty, price),
            None,
            1,
        )
    }

    fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
        Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 1)
    }

    #[test]
    fn place_gtc_without_cross_keeps_state_and_outputs_no_side_effects() {
        let use_case = PlaceSpotOrderV2UseCase;
        let taker = buy_order(SpotOrderTimeInForce::Gtc);
        let makers = vec![sell_order("maker-1", "seller", 110, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = PlaceSpotOrderV2State {
            order_id: "taker-buy".to_string(),
            symbol: "BTCUSDT".to_string(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let after = use_case.compute_after_changes(&place_cmd("gtc"), &state).unwrap();

        assert_eq!(after.taker_order_after, taker);
        assert_eq!(after.maker_orders_after, makers);
        assert!(after.created_trades.is_empty());
        assert!(after.created_vouchers.is_empty());
        assert_eq!(after.created_balance_ledger_entries.len(), 1);
        assert_eq!(
            after.created_balance_ledger_entries[0].operation,
            BalanceLedgerOperation::Freeze
        );
    }

    #[test]
    fn place_ioc_partial_fill_releases_remainder() {
        let use_case = PlaceSpotOrderV2UseCase;
        let makers = vec![sell_order("maker-1", "seller", 100, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = PlaceSpotOrderV2State {
            order_id: "taker-buy".to_string(),
            symbol: "BTCUSDT".to_string(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let changes = use_case.compute_before_after_changes(&place_cmd("ioc"), state).unwrap();

        assert_eq!(changes.created_trades.len(), 1);
        assert_eq!(changes.created_trades[0].taker_fee, 1);
        assert_eq!(changes.created_trades[0].maker_fee, 1);
        assert_eq!(changes.updated_taker_order.after.status(), SpotOrderStatus::Canceled);
        assert_eq!(
            changes.updated_taker_order.after.status_reason(),
            Some(SpotOrderStatusReason::IocCancelRejected)
        );
        assert!(!changes.created_balance_ledger_entries.is_empty());
        assert!(!changes.to_replayable_events().unwrap().is_empty());
    }

    #[test]
    fn merge_before_after_uses_generated_taker_as_before_truth() {
        let use_case = PlaceSpotOrderV2UseCase;
        let taker = buy_order(SpotOrderTimeInForce::Ioc);
        let makers = vec![sell_order("maker-1", "seller", 100, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = PlaceSpotOrderV2State {
            order_id: "taker-buy".to_string(),
            symbol: "BTCUSDT".to_string(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let after = use_case.compute_after_changes(&place_cmd("ioc"), &state).unwrap();

        let changes = PlaceSpotOrderV2UseCase::merge_before_and_after(state, after).unwrap();

        assert_eq!(changes.updated_taker_order.before, taker);
        assert_eq!(changes.updated_maker_orders[0].before, makers[0]);
        assert_eq!(changes.updated_balances.len(), balances.len());
        let before_by_key = balances
            .iter()
            .map(|balance| ((balance.account_id.clone(), balance.asset_id.clone()), balance))
            .collect::<HashMap<_, _>>();
        for pair in &changes.updated_balances {
            let key = (pair.before.account_id.clone(), pair.before.asset_id.clone());
            assert_eq!(Some(&pair.before), before_by_key.get(&key).copied());
        }
    }

    #[test]
    fn place_alo_cross_rejects_and_releases() {
        let use_case = PlaceSpotOrderV2UseCase;
        let makers = vec![sell_order("maker-1", "seller", 99, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = PlaceSpotOrderV2State {
            order_id: "taker-buy".to_string(),
            symbol: "BTCUSDT".to_string(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let after = use_case.compute_after_changes(&place_cmd("alo"), &state).unwrap();

        assert_eq!(after.taker_order_after.status(), SpotOrderStatus::Rejected);
        assert_eq!(
            after.taker_order_after.status_reason(),
            Some(SpotOrderStatusReason::BadAloPxRejected)
        );
        assert!(after.created_trades.is_empty());
        assert!(
            after
                .created_balance_ledger_entries
                .iter()
                .any(|entry| entry.operation == BalanceLedgerOperation::Unfreeze)
        );
    }
}
