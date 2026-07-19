use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2Unchecked,
    ReplayableChanges,
};
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
use crate::{
    CancelSpotOrderV2Input, MatchSpotOrderV2Input, PlaceSpotOrderV2Input, SpotOrderExecution,
    SpotOrderTimeInForce, SpotTrade,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlaceSpotOrderV2CmdV3 {
    pub party_id: String,
    pub asset: u32,
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub tif: String,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CancelSpotOrderV2CmdV3 {
    pub party_id: String,
    pub asset: u32,
    pub lookup: CancelSpotOrderV2LookupV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CancelSpotOrderV2LookupV3 {
    #[default]
    Missing,
    Oid(u64),
    Cloid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV2CommandV3 {
    Place(PlaceSpotOrderV2CmdV3),
    Cancel(CancelSpotOrderV2CmdV3),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV2GivenStateV3 {
    Place {
        taker_order: SpotOrderV2,
        maker_orders: Vec<SpotOrderV2>,
        settlement_balances: Vec<Balance>,
        base_asset_id: String,
        quote_asset_id: String,
        fee_account_id: String,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    },
    Cancel {
        order: SpotOrderV2,
        balances: Vec<Balance>,
        base_asset_id: String,
        quote_asset_id: String,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2AfterChangesV3 {
    pub taker_order_after: SpotOrderV2,
    pub maker_orders_after: Vec<SpotOrderV2>,
    pub balances_after: Vec<Balance>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2AfterChangesV3 {
    pub order_after: SpotOrderV2,
    pub balances_after: Vec<Balance>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV2AfterChangesV3 {
    Place(PlaceSpotOrderV2AfterChangesV3),
    Cancel(CancelSpotOrderV2AfterChangesV3),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2ChangesV3 {
    pub updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2ChangesV3 {
    pub updated_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV2CaseChangesV3 {
    Place(PlaceSpotOrderV2ChangesV3),
    Cancel(CancelSpotOrderV2ChangesV3),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotOrderV2UseCaseFamilyV3Error {
    #[error("given_state branch does not match command branch")]
    BranchMismatch,
    #[error("price must be a positive integer string")]
    InvalidPrice,
    #[error("size must be a positive integer string")]
    InvalidSize,
    #[error("time in force must be gtc, ioc, or alo")]
    InvalidTimeInForce,
    #[error("order template does not match command-derived order")]
    OrderTemplateMismatch,
    #[error("match id must not be empty")]
    InvalidMatchId,
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

#[derive(Debug, Clone, Default)]
pub struct SpotOrderV2UseCaseFamilyV3;

impl ReplayableChanges for PlaceSpotOrderV2ChangesV3 {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let mut events = Vec::new();
        for trade in &self.created_trades {
            events.push(trade.track_create_event()?);
        }
        for maker in &self.updated_maker_orders {
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        events.push(
            self.updated_taker_order
                .after
                .track_update_event_from(&self.updated_taker_order.before)?,
        );
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

impl ReplayableChanges for CancelSpotOrderV2ChangesV3 {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let mut events = Vec::new();
        events.push(self.updated_order.after.track_update_event_from(&self.updated_order.before)?);
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

impl ReplayableChanges for SpotOrderV2CaseChangesV3 {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Place(changes) => changes.to_replayable_events(),
            Self::Cancel(changes) => changes.to_replayable_events(),
        }
    }
}

impl MiStateMachineV2Unchecked for SpotOrderV2UseCaseFamilyV3 {
    type Command = SpotOrderV2CommandV3;
    type GivenState = SpotOrderV2GivenStateV3;
    type Error = SpotOrderV2UseCaseFamilyV3Error;
    type AfterChanges = SpotOrderV2AfterChangesV3;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd {
            SpotOrderV2CommandV3::Place(cmd) => {
                parse_positive_u64(&cmd.price, SpotOrderV2UseCaseFamilyV3Error::InvalidPrice)?;
                parse_positive_u64(&cmd.size, SpotOrderV2UseCaseFamilyV3Error::InvalidSize)?;
                parse_tif(&cmd.tif)?;
                Ok(())
            }
            SpotOrderV2CommandV3::Cancel(_) => Ok(()),
        }
    }

    fn validate_against_given_state(
        &self,
        cmd: &Self::Command,
        given_state: &SpotOrderV2GivenStateV3,
    ) -> Result<(), Self::Error> {
        match (cmd, given_state) {
            (
                SpotOrderV2CommandV3::Place(cmd),
                SpotOrderV2GivenStateV3::Place {
                    taker_order,
                    maker_orders,
                    settlement_balances,
                    base_asset_id,
                    quote_asset_id,
                    fee_account_id,
                    maker_fee_bps,
                    taker_fee_bps,
                    ..
                },
            ) => {
                if fee_account_id.is_empty() {
                    return Err(SpotOrderV2UseCaseFamilyV3Error::InvalidFeeAccountId);
                }
                let input = place_input_from_cmd(
                    cmd,
                    taker_order,
                    settlement_balances,
                    base_asset_id,
                    quote_asset_id,
                    *maker_fee_bps,
                    *taker_fee_bps,
                )?;
                let placed = SpotOrderV2::place(input)?.order;
                if &placed != taker_order {
                    return Err(SpotOrderV2UseCaseFamilyV3Error::OrderTemplateMismatch);
                }
                validate_reservation_for_order(
                    taker_order,
                    &taker_order.reservation,
                    expected_principal_kind_for(taker_order.side()),
                    base_asset_id,
                    quote_asset_id,
                )?;
                validate_reservation_for_order(
                    taker_order,
                    &taker_order.fee_reservation,
                    expected_fee_kind_for(taker_order.side()),
                    quote_asset_id,
                    quote_asset_id,
                )?;
                for maker in maker_orders {
                    validate_reservation_for_order(
                        maker,
                        &maker.reservation,
                        expected_principal_kind_for(maker.side()),
                        base_asset_id,
                        quote_asset_id,
                    )?;
                    validate_reservation_for_order(
                        maker,
                        &maker.fee_reservation,
                        expected_fee_kind_for(maker.side()),
                        quote_asset_id,
                        quote_asset_id,
                    )?;
                }
                taker_order.ensure_matchable()?;
                Ok(())
            }
            (
                SpotOrderV2CommandV3::Cancel(_),
                SpotOrderV2GivenStateV3::Cancel {
                    order,
                    balances,
                    base_asset_id,
                    quote_asset_id,
                    ..
                },
            ) => {
                let mut order_after = order.clone();
                order_after.cancel(CancelSpotOrderV2Input {
                    balance_entity_id: balance_entity_id_for_reservation(
                        balances,
                        &order.reservation,
                    )?,
                })?;
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
                    quote_asset_id,
                    quote_asset_id,
                )?;
                Ok(())
            }
            _ => Err(SpotOrderV2UseCaseFamilyV3Error::BranchMismatch),
        }
    }

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        given_state: &SpotOrderV2GivenStateV3,
    ) -> Result<Self::AfterChanges, Self::Error> {
        match (cmd, given_state) {
            (
                SpotOrderV2CommandV3::Place(place_cmd),
                SpotOrderV2GivenStateV3::Place {
                    taker_order,
                    maker_orders,
                    settlement_balances,
                    base_asset_id,
                    quote_asset_id,
                    fee_account_id,
                    maker_fee_bps,
                    taker_fee_bps,
                },
            ) => self.compute_place_after(
                place_cmd,
                taker_order,
                maker_orders,
                settlement_balances,
                base_asset_id,
                quote_asset_id,
                fee_account_id,
                *maker_fee_bps,
                *taker_fee_bps,
            ),
            (
                SpotOrderV2CommandV3::Cancel(_),
                SpotOrderV2GivenStateV3::Cancel {
                    order,
                    balances,
                    base_asset_id,
                    quote_asset_id,
                    maker_fee_bps,
                    taker_fee_bps,
                },
            ) => self.compute_cancel_after(
                order,
                balances,
                base_asset_id,
                quote_asset_id,
                *maker_fee_bps,
                *taker_fee_bps,
            ),
            _ => Err(SpotOrderV2UseCaseFamilyV3Error::BranchMismatch),
        }
    }
}

impl MiStateMachineOwnedV2BeforeAfter for SpotOrderV2UseCaseFamilyV3 {
    type BeforeAfterChanges = SpotOrderV2CaseChangesV3;

    fn merge_before_and_after(
        given_state: SpotOrderV2GivenStateV3,
        after: Self::AfterChanges,
    ) -> Result<Self::BeforeAfterChanges, Self::Error> {
        match (given_state, after) {
            (
                SpotOrderV2GivenStateV3::Place {
                    taker_order,
                    maker_orders,
                    settlement_balances,
                    ..
                },
                SpotOrderV2AfterChangesV3::Place(after),
            ) => Ok(SpotOrderV2CaseChangesV3::Place(PlaceSpotOrderV2ChangesV3 {
                updated_taker_order: UpdatedEntityPair {
                    before: taker_order,
                    after: after.taker_order_after,
                },
                updated_maker_orders: zip_pairs(maker_orders, after.maker_orders_after)?,
                updated_balances: merge_balance_pairs(settlement_balances, after.balances_after)?,
                created_trades: after.created_trades,
                created_vouchers: after.created_vouchers,
                created_balance_ledger_entries: after.created_balance_ledger_entries,
            })),
            (
                SpotOrderV2GivenStateV3::Cancel { order, balances, .. },
                SpotOrderV2AfterChangesV3::Cancel(after),
            ) => Ok(SpotOrderV2CaseChangesV3::Cancel(CancelSpotOrderV2ChangesV3 {
                updated_order: UpdatedEntityPair { before: order, after: after.order_after },
                updated_balances: merge_balance_pairs(balances, after.balances_after)?,
                created_balance_ledger_entries: after.created_balance_ledger_entries,
            })),
            _ => Err(SpotOrderV2UseCaseFamilyV3Error::BranchMismatch),
        }
    }
}

impl SpotOrderV2UseCaseFamilyV3 {
    #[allow(clippy::too_many_arguments)]
    fn compute_place_after(
        &self,
        cmd: &PlaceSpotOrderV2CmdV3,
        taker_order: &SpotOrderV2,
        maker_orders: &[SpotOrderV2],
        settlement_balances: &[Balance],
        base_asset_id: &str,
        quote_asset_id: &str,
        fee_account_id: &str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Result<SpotOrderV2AfterChangesV3, SpotOrderV2UseCaseFamilyV3Error> {
        let mut maker_orders_after = maker_orders.to_vec();
        let mut balance_book = BalanceMap::new(settlement_balances);
        let place_input = place_input_from_cmd(
            cmd,
            taker_order,
            settlement_balances,
            base_asset_id,
            quote_asset_id,
            maker_fee_bps,
            taker_fee_bps,
        )?;
        let place_outcome = SpotOrderV2::place(place_input)?;
        if place_outcome.order != *taker_order {
            return Err(SpotOrderV2UseCaseFamilyV3Error::OrderTemplateMismatch);
        }
        let mut taker_after = place_outcome.order;
        let mut created_balance_ledger_entries = Vec::new();
        let freeze_ledger_entry =
            apply_behavior_ledger_entry(place_outcome.freeze_ledger_entry, &mut balance_book)?;
        created_balance_ledger_entries.push(freeze_ledger_entry);
        let mut created_trades = Vec::new();
        let mut created_vouchers = Vec::new();

        match spot_order_v2_matching_decision(taker_order, maker_orders.first())? {
            SpotOrderV2MatchingDecision::Rest => {
                return Ok(SpotOrderV2AfterChangesV3::Place(PlaceSpotOrderV2AfterChangesV3 {
                    taker_order_after: taker_after,
                    maker_orders_after,
                    balances_after: balance_book.into_balances(),
                    created_trades,
                    created_vouchers,
                    created_balance_ledger_entries,
                }));
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
                return Ok(SpotOrderV2AfterChangesV3::Place(PlaceSpotOrderV2AfterChangesV3 {
                    taker_order_after: taker_after,
                    maker_orders_after,
                    balances_after: balance_book.into_balances(),
                    created_trades,
                    created_vouchers,
                    created_balance_ledger_entries,
                }));
            }
            SpotOrderV2MatchingDecision::Match => {}
        }

        let taker_before_match = taker_after.clone();
        let match_outcome = taker_after.match_with_makers(
            &mut maker_orders_after,
            MatchSpotOrderV2Input {
                match_id: format!("spot-match:{}", taker_after.order_id()),
                maker_fee_bps,
                taker_fee_bps,
            },
        )?;
        let mut total_taker_fill = 0_u64;
        for (index, trade) in match_outcome.trades.into_iter().enumerate() {
            let trade_notional = trade
                .notional_quote()
                .ok_or(SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow)?;

            total_taker_fill = total_taker_fill
                .checked_add(trade.qty)
                .ok_or(SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow)?;

            let taker_principal_consume =
                principal_consume_amount_for_taker(&taker_after, trade.qty, trade_notional);
            consume_reservation(
                &mut taker_after.reservation,
                taker_principal_consume,
                ReservationCloseReason::Filled,
            )?;
            let maker_principal_consume = principal_consume_amount_for_maker(
                &maker_orders_after[index],
                trade.qty,
                trade_notional,
            );
            consume_reservation(
                &mut maker_orders_after[index].reservation,
                maker_principal_consume,
                ReservationCloseReason::Filled,
            )?;

            consume_reservation(
                &mut taker_after.fee_reservation,
                trade.taker_fee,
                ReservationCloseReason::Filled,
            )?;
            consume_reservation(
                &mut maker_orders_after[index].fee_reservation,
                trade.maker_fee,
                ReservationCloseReason::Filled,
            )?;

            let settlement_id = format!("spot-settlement:{}", trade.trade_id);
            let voucher = trade
                .derive_spot_settlement_transfer_voucher_with_fees(
                    format!("spot-voucher:{}", trade.trade_id),
                    settlement_id.clone(),
                    base_asset_id,
                    quote_asset_id,
                    fee_account_id.to_string(),
                )
                .ok_or(SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow)?;

            apply_trade_balance_effects(
                &trade,
                &settlement_id,
                base_asset_id,
                quote_asset_id,
                fee_account_id,
                &mut balance_book,
                &mut created_balance_ledger_entries,
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

        Ok(SpotOrderV2AfterChangesV3::Place(PlaceSpotOrderV2AfterChangesV3 {
            taker_order_after: taker_after,
            maker_orders_after,
            balances_after: balance_book.into_balances(),
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_cancel_after(
        &self,
        order: &SpotOrderV2,
        balances: &[Balance],
        _base_asset_id: &str,
        _quote_asset_id: &str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Result<SpotOrderV2AfterChangesV3, SpotOrderV2UseCaseFamilyV3Error> {
        let mut order_after = order.clone();
        let mut balance_book = BalanceMap::new(balances);
        let mut created_balance_ledger_entries = Vec::new();

        let cancel_outcome = order_after.cancel(CancelSpotOrderV2Input {
            balance_entity_id: balance_entity_id_for_reservation(balances, &order.reservation)?,
        })?;
        let unfreeze_ledger_entry =
            apply_behavior_ledger_entry(cancel_outcome.unfreeze_ledger_entry, &mut balance_book)?;
        created_balance_ledger_entries.push(unfreeze_ledger_entry);

        release_remaining_for_cancel(
            &mut order_after,
            &mut balance_book,
            &mut created_balance_ledger_entries,
            maker_fee_bps,
            taker_fee_bps,
        )?;

        Ok(SpotOrderV2AfterChangesV3::Cancel(CancelSpotOrderV2AfterChangesV3 {
            order_after,
            balances_after: balance_book.into_balances(),
            created_balance_ledger_entries,
        }))
    }
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
    error: SpotOrderV2UseCaseFamilyV3Error,
) -> Result<u64, SpotOrderV2UseCaseFamilyV3Error> {
    let value = raw.parse::<u64>().map_err(|_| error.clone())?;
    if value == 0 {
        return Err(error);
    }
    Ok(value)
}

fn parse_tif(raw: &str) -> Result<SpotOrderTimeInForce, SpotOrderV2UseCaseFamilyV3Error> {
    match raw {
        "gtc" | "Gtc" => Ok(SpotOrderTimeInForce::Gtc),
        "ioc" | "Ioc" => Ok(SpotOrderTimeInForce::Ioc),
        "alo" | "Alo" => Ok(SpotOrderTimeInForce::Alo),
        _ => Err(SpotOrderV2UseCaseFamilyV3Error::InvalidTimeInForce),
    }
}

fn place_input_from_cmd(
    cmd: &PlaceSpotOrderV2CmdV3,
    template: &SpotOrderV2,
    settlement_balances: &[Balance],
    base_asset_id: &str,
    quote_asset_id: &str,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<PlaceSpotOrderV2Input, SpotOrderV2UseCaseFamilyV3Error> {
    let side = if cmd.is_buy { SpotOrderSide::Buy } else { SpotOrderSide::Sell };
    let price = parse_positive_u64(&cmd.price, SpotOrderV2UseCaseFamilyV3Error::InvalidPrice)?;
    let qty = parse_positive_u64(&cmd.size, SpotOrderV2UseCaseFamilyV3Error::InvalidSize)?;
    let base_balance_entity_id =
        balance_entity_id_for_account_asset(settlement_balances, &cmd.party_id, base_asset_id)?;
    let quote_balance_entity_id =
        balance_entity_id_for_account_asset(settlement_balances, &cmd.party_id, quote_asset_id)?;

    Ok(PlaceSpotOrderV2Input {
        order_id: template.order_id().to_string(),
        asset: cmd.asset,
        account_id: cmd.party_id.clone(),
        symbol: template.symbol().to_string(),
        side,
        execution: SpotOrderExecution::Limit { price },
        time_in_force: parse_tif(&cmd.tif)?,
        qty,
        base_asset_id: base_asset_id.to_string(),
        quote_asset_id: quote_asset_id.to_string(),
        base_balance_entity_id,
        quote_balance_entity_id,
        maker_fee_bps,
        taker_fee_bps,
        client_order_id: cmd.cloid.clone(),
    })
}

fn balance_entity_id_for_account_asset(
    balances: &[Balance],
    account_id: &str,
    asset_id: &str,
) -> Result<String, SpotOrderV2UseCaseFamilyV3Error> {
    balances
        .iter()
        .find(|balance| balance.account_id == account_id && balance.asset_id == asset_id)
        .map(Entity::entity_id)
        .ok_or(SpotOrderV2UseCaseFamilyV3Error::BalanceNotFound)
}

fn balance_entity_id_for_reservation(
    balances: &[Balance],
    reservation: &Reservation,
) -> Result<String, SpotOrderV2UseCaseFamilyV3Error> {
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
) -> Result<(), SpotOrderV2UseCaseFamilyV3Error> {
    if reservation.market_kind != ReservationMarketKind::Spot {
        return Err(SpotOrderV2UseCaseFamilyV3Error::ReservationKindMismatch);
    }
    if reservation.caused_by_order_id != order.order_id() {
        return Err(SpotOrderV2UseCaseFamilyV3Error::ReservationOrderMismatch);
    }
    if reservation.reservation_kind != expected_kind {
        return Err(SpotOrderV2UseCaseFamilyV3Error::ReservationKindMismatch);
    }
    let expected_asset = match expected_kind {
        ReservationKind::SpotBuyQuote
        | ReservationKind::SpotBuyFeeQuote
        | ReservationKind::SpotSellFeeQuote => quote_asset_id,
        ReservationKind::SpotSellBase => base_asset_id,
        _ => return Err(SpotOrderV2UseCaseFamilyV3Error::ReservationKindMismatch),
    };
    if reservation.asset_id != expected_asset {
        return Err(SpotOrderV2UseCaseFamilyV3Error::ReservationAssetMismatch);
    }
    Ok(())
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
) -> Result<(), SpotOrderV2UseCaseFamilyV3Error> {
    if amount == 0 {
        return Ok(());
    }
    let before = reservation.clone();
    let close_reason = if amount == before.remaining_amount { Some(terminal_reason) } else { None };
    let after = before.consume(amount, close_reason).map_err(map_reservation_error_to_family)?;
    *reservation = after;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn release_remaining_for_terminal(
    order_after: &mut SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), SpotOrderV2UseCaseFamilyV3Error> {
    let requirements = order_after.terminal_release_requirements(maker_fee_bps, taker_fee_bps);

    if let Some(requirement) = requirements.principal {
        let release_amount = order_after.reservation.remaining_amount.min(requirement.amount);
        if release_amount > 0 {
            let close_reason = match requirement.reason {
                crate::SpotOrderReleaseReason::Canceled => ReservationCloseReason::Canceled,
                crate::SpotOrderReleaseReason::IocUnfilled => {
                    ReservationCloseReason::IocRemainderCanceled
                }
                crate::SpotOrderReleaseReason::Rejected => ReservationCloseReason::Rejected,
                crate::SpotOrderReleaseReason::FilledCleanup => ReservationCloseReason::Filled,
            };
            let after = order_after
                .reservation
                .release(release_amount, Some(close_reason))
                .map_err(map_reservation_error_to_family)?;
            let asset_id = after.asset_id.clone();
            order_after.reservation = after;
            release_to_balance(
                order_after,
                &asset_id,
                release_amount,
                balance_book,
                ledger_entries,
            )?;
        }
    }

    if let Some(requirement) = requirements.fee {
        let release_amount = order_after.fee_reservation.remaining_amount.min(requirement.amount);
        if release_amount > 0 {
            let close_reason = match requirement.reason {
                crate::SpotOrderReleaseReason::Canceled => ReservationCloseReason::Canceled,
                crate::SpotOrderReleaseReason::IocUnfilled => {
                    ReservationCloseReason::IocRemainderCanceled
                }
                crate::SpotOrderReleaseReason::Rejected => ReservationCloseReason::Rejected,
                crate::SpotOrderReleaseReason::FilledCleanup => ReservationCloseReason::Filled,
            };
            let after = order_after
                .fee_reservation
                .release(release_amount, Some(close_reason))
                .map_err(map_reservation_error_to_family)?;
            let asset_id = after.asset_id.clone();
            order_after.fee_reservation = after;
            release_to_balance(
                order_after,
                &asset_id,
                release_amount,
                balance_book,
                ledger_entries,
            )?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn release_remaining_for_cancel(
    order: &mut SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), SpotOrderV2UseCaseFamilyV3Error> {
    if let Some(requirement) = order.fee_hold_requirement(maker_fee_bps, taker_fee_bps) {
        let release_amount = order.fee_reservation.remaining_amount.min(requirement.amount);
        if release_amount > 0 {
            let after = order
                .fee_reservation
                .release(release_amount, Some(ReservationCloseReason::Canceled))
                .map_err(map_reservation_error_to_family)?;
            let asset_id = after.asset_id.clone();
            order.fee_reservation = after;
            release_to_balance(order, &asset_id, release_amount, balance_book, ledger_entries)?;
        }
    }
    Ok(())
}

fn release_to_balance(
    order: &SpotOrderV2,
    asset_id: &str,
    amount: u64,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), SpotOrderV2UseCaseFamilyV3Error> {
    let reason = match order.side() {
        SpotOrderSide::Buy => BalanceLedgerReason::CancelSpotOrderReleaseQuote {
            order_id: order.order_id().to_string(),
        },
        SpotOrderSide::Sell => BalanceLedgerReason::CancelSpotOrderReleaseBase {
            order_id: order.order_id().to_string(),
        },
    };
    let balance = balance_book.get_mut(&order.account_id(), asset_id)?;
    let entry = apply_balance_ledger_entry(
        BalanceLedgerOperation::Unfreeze,
        format!("balance-ledger:{}:release:{}", order.order_id(), ledger_entries.len() + 1),
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
            unreachable!()
        }
    }?;
    entry.apply_to(balance)?;
    Ok(entry)
}

fn apply_behavior_ledger_entry(
    mut entry: BalanceLedgerEntryV2,
    balance_book: &mut BalanceMap,
) -> Result<BalanceLedgerEntryV2, SpotOrderV2UseCaseFamilyV3Error> {
    let balance = balance_book.get_by_entity_id_mut(&entry.balance_entity_id)?;
    entry.apply_to(balance)?;
    Ok(entry)
}

#[allow(clippy::too_many_arguments)]
fn apply_trade_balance_effects(
    trade: &SpotTrade,
    settlement_id: &str,
    base_asset_id: &str,
    quote_asset_id: &str,
    fee_account_id: &str,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), SpotOrderV2UseCaseFamilyV3Error> {
    let trade_ids = vec![trade.trade_id.clone()];
    let settlement_ids = vec![settlement_id.to_string()];

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
    let seller_release_base_reason = BalanceLedgerReason::SettleSpotTradeSellerReleaseFrozenBase {
        trade_ids: trade_ids.clone(),
        settlement_ids: settlement_ids.clone(),
    };

    let quote_notional =
        trade.notional_quote().ok_or(SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow)?;
    let buyer_account_id = trade.buyer_account_id().to_string();
    let seller_account_id = trade.seller_account_id().to_string();

    {
        let balance = balance_book.get_mut(&buyer_account_id, base_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::CreditAvailable,
            format!("balance-ledger:{}:buyer-base", settlement_id),
            balance,
            trade.qty,
            buyer_receive_base_reason,
        )?);
    }
    {
        let balance = balance_book.get_mut(&buyer_account_id, quote_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::DebitFrozen,
            format!("balance-ledger:{}:buyer-quote", settlement_id),
            balance,
            quote_notional,
            buyer_release_quote_reason,
        )?);
    }
    {
        let balance = balance_book.get_mut(&seller_account_id, quote_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::CreditAvailable,
            format!("balance-ledger:{}:seller-quote", settlement_id),
            balance,
            quote_notional,
            seller_receive_quote_reason,
        )?);
    }
    {
        let balance = balance_book.get_mut(&seller_account_id, base_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::DebitFrozen,
            format!("balance-ledger:{}:seller-base", settlement_id),
            balance,
            trade.qty,
            seller_release_base_reason,
        )?);
    }
    if let Some((buyer_fee_account_id, buyer_fee_amount)) = fee_payer_for_buyer(trade) {
        let balance = balance_book.get_mut(&buyer_fee_account_id, quote_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::DebitFrozen,
            format!("balance-ledger:{}:buyer-fee", settlement_id),
            balance,
            buyer_fee_amount,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: trade.trade_id.clone(),
                match_id: trade.match_id.clone(),
                settlement_batch_id: settlement_id.to_string(),
                purpose: SettlementTransferPurpose::TradingFee,
            },
        )?);
        let fee_balance = balance_book.get_mut(fee_account_id, quote_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::CreditAvailable,
            format!("balance-ledger:{}:buyer-fee-recv", settlement_id),
            fee_balance,
            buyer_fee_amount,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: trade.trade_id.clone(),
                match_id: trade.match_id.clone(),
                settlement_batch_id: settlement_id.to_string(),
                purpose: SettlementTransferPurpose::TradingFee,
            },
        )?);
    }
    if let Some((seller_fee_account_id, seller_fee_amount)) = fee_payer_for_seller(trade) {
        let balance = balance_book.get_mut(&seller_fee_account_id, quote_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::DebitFrozen,
            format!("balance-ledger:{}:seller-fee", settlement_id),
            balance,
            seller_fee_amount,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: trade.trade_id.clone(),
                match_id: trade.match_id.clone(),
                settlement_batch_id: settlement_id.to_string(),
                purpose: SettlementTransferPurpose::TradingFee,
            },
        )?);
        let fee_balance = balance_book.get_mut(fee_account_id, quote_asset_id)?;
        ledger_entries.push(apply_balance_ledger_entry(
            BalanceLedgerOperation::CreditAvailable,
            format!("balance-ledger:{}:seller-fee-recv", settlement_id),
            fee_balance,
            seller_fee_amount,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: trade.trade_id.clone(),
                match_id: trade.match_id.clone(),
                settlement_batch_id: settlement_id.to_string(),
                purpose: SettlementTransferPurpose::TradingFee,
            },
        )?);
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

fn map_reservation_error_to_family(
    error: crate::ReservationError,
) -> SpotOrderV2UseCaseFamilyV3Error {
    match error {
        crate::ReservationError::ArithmeticOverflow => {
            SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow
        }
        crate::ReservationError::AmountExceedsRemaining => {
            SpotOrderV2UseCaseFamilyV3Error::InsufficientFrozenBalance
        }
        crate::ReservationError::AlreadyClosed
        | crate::ReservationError::InvalidAmount
        | crate::ReservationError::InvalidOriginalAmount
        | crate::ReservationError::MissingCloseReason => {
            SpotOrderV2UseCaseFamilyV3Error::BranchMismatch
        }
    }
}

fn zip_pairs<T>(
    before: Vec<T>,
    after: Vec<T>,
) -> Result<Vec<UpdatedEntityPair<T>>, SpotOrderV2UseCaseFamilyV3Error> {
    if before.len() != after.len() {
        return Err(SpotOrderV2UseCaseFamilyV3Error::ReservationCountMismatch);
    }
    Ok(before
        .into_iter()
        .zip(after)
        .map(|(before, after)| UpdatedEntityPair { before, after })
        .collect())
}

fn merge_balance_pairs(
    before: Vec<Balance>,
    after: Vec<Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, SpotOrderV2UseCaseFamilyV3Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
        let before_balance = before_map
            .get(&balance_id)
            .cloned()
            .ok_or(SpotOrderV2UseCaseFamilyV3Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(SpotOrderV2UseCaseFamilyV3Error::BalanceNotFound);
        }
        pairs.push(UpdatedEntityPair { before: before_balance, after: balance });
    }
    Ok(pairs)
}

fn balance_replay_events_from_ledger_entries(
    updated_balances: &[UpdatedEntityPair<Balance>],
    ledger_entries: &[BalanceLedgerEntryV2],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut current_balances = HashMap::<String, Balance>::with_capacity(updated_balances.len());
    let mut expected_after_balances =
        HashMap::<String, Balance>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.entity_id();
        current_balances.insert(balance_id.clone(), balance.before.clone());
        expected_after_balances.insert(balance_id, balance.after.clone());
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

    for (balance_id, expected_after) in expected_after_balances {
        if current_balances.get(&balance_id) != Some(&expected_after) {
            return Err(common_entity::EntityError::Custom(
                "balance replay chain does not reach case-level balance after state".to_string(),
            ));
        }
    }
    Ok(events)
}

struct BalanceMap {
    balances: HashMap<String, Balance>,
}

impl BalanceMap {
    fn new(balances: &[Balance]) -> Self {
        Self {
            balances: balances
                .iter()
                .cloned()
                .map(|balance| (balance.entity_id(), balance))
                .collect(),
        }
    }

    fn get_mut(
        &mut self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<&mut Balance, SpotOrderV2UseCaseFamilyV3Error> {
        self.balances
            .get_mut(&format!("{account_id}:{asset_id}"))
            .ok_or(SpotOrderV2UseCaseFamilyV3Error::BalanceNotFound)
    }

    fn get_by_entity_id_mut(
        &mut self,
        entity_id: &str,
    ) -> Result<&mut Balance, SpotOrderV2UseCaseFamilyV3Error> {
        self.balances.get_mut(entity_id).ok_or(SpotOrderV2UseCaseFamilyV3Error::BalanceNotFound)
    }

    fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by(|lhs, rhs| lhs.entity_id().cmp(&rhs.entity_id()));
        balances
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{MiStateMachineOwnedV2, MiStateMachineOwnedV2BeforeAfter};

    use super::*;
    use crate::entity::spot::spot_order_v2::test_principal_reservation;
    use crate::{SpotOrderExecution, SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce};

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

    fn place_cmd(tif: &str) -> SpotOrderV2CommandV3 {
        SpotOrderV2CommandV3::Place(PlaceSpotOrderV2CmdV3 {
            party_id: "buyer".to_string(),
            asset: 10_001,
            is_buy: true,
            price: "100".to_string(),
            size: "2".to_string(),
            tif: tif.to_string(),
            cloid: None,
        })
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
            qty,
            0,
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
        let family = SpotOrderV2UseCaseFamilyV3;
        let taker = buy_order(SpotOrderTimeInForce::Gtc);
        let makers = vec![sell_order("maker-1", "seller", 110, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenStateV3::Place {
            taker_order: taker.clone(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2AfterChangesV3::Place(after) =
            family.compute_after_changes(&place_cmd("gtc"), &state).unwrap()
        else {
            panic!("expected place after changes");
        };

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
        let family = SpotOrderV2UseCaseFamilyV3;
        let taker = buy_order(SpotOrderTimeInForce::Ioc);
        let makers = vec![sell_order("maker-1", "seller", 100, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenStateV3::Place {
            taker_order: taker.clone(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2CaseChangesV3::Place(changes) =
            family.compute_before_after_changes(&place_cmd("ioc"), state).unwrap()
        else {
            panic!("expected place case changes");
        };

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
    fn merge_before_after_extracts_before_truth_from_given_state() {
        let family = SpotOrderV2UseCaseFamilyV3;
        let taker = buy_order(SpotOrderTimeInForce::Ioc);
        let makers = vec![sell_order("maker-1", "seller", 100, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenStateV3::Place {
            taker_order: taker.clone(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let after = family.compute_after_changes(&place_cmd("ioc"), &state).unwrap();

        let SpotOrderV2CaseChangesV3::Place(changes) =
            SpotOrderV2UseCaseFamilyV3::merge_before_and_after(state, after).unwrap()
        else {
            panic!("expected place case changes");
        };

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
        let family = SpotOrderV2UseCaseFamilyV3;
        let taker = buy_order(SpotOrderTimeInForce::Alo);
        let makers = vec![sell_order("maker-1", "seller", 99, 1)];
        let balances = vec![
            balance("buyer", "USDT", 1200, 1),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenStateV3::Place {
            taker_order: taker.clone(),
            maker_orders: makers.clone(),
            settlement_balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            fee_account_id: "fee".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2AfterChangesV3::Place(after) =
            family.compute_after_changes(&place_cmd("alo"), &state).unwrap()
        else {
            panic!("expected place after");
        };

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

    #[test]
    fn cancel_open_order_releases_principal_and_fee() {
        let family = SpotOrderV2UseCaseFamilyV3;
        let order = buy_order(SpotOrderTimeInForce::Gtc);
        let balances = vec![balance("buyer", "USDT", 1000, 201)];
        let state = SpotOrderV2GivenStateV3::Cancel {
            order: order.clone(),
            balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2CaseChangesV3::Cancel(changes) = family
            .compute_before_after_changes(
                &SpotOrderV2CommandV3::Cancel(CancelSpotOrderV2CmdV3::default()),
                state,
            )
            .unwrap()
        else {
            panic!("expected cancel changes");
        };

        assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Canceled);
        assert_eq!(
            changes.updated_order.after.status_reason(),
            Some(SpotOrderStatusReason::CanceledByUser)
        );
        assert_eq!(changes.updated_order.after.reservation.remaining_amount, 0);
        assert_eq!(changes.updated_order.after.fee_reservation.remaining_amount, 0);
        assert!(!changes.created_balance_ledger_entries.is_empty());
        assert!(!changes.to_replayable_events().unwrap().is_empty());
    }

    #[test]
    fn validate_rejects_branch_mismatch() {
        let family = SpotOrderV2UseCaseFamilyV3;
        let order = buy_order(SpotOrderTimeInForce::Gtc);
        let balances = vec![balance("buyer", "USDT", 1000, 201)];
        let state = SpotOrderV2GivenStateV3::Cancel {
            order: order.clone(),
            balances: balances.clone(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        assert_eq!(
            family.compute_after_changes(&place_cmd("gtc"), &state),
            Err(SpotOrderV2UseCaseFamilyV3Error::BranchMismatch)
        );
    }
}
