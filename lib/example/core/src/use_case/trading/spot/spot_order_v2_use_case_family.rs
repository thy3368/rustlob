use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2Unchecked,
    ReplayableChanges,
};
use thiserror::Error;

use crate::SpotTrade;
use crate::entity::account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error,
};
use crate::entity::account::balance_ledger_reason::BalanceLedgerReason;
use crate::entity::account::settlement_transfer_voucher::SettlementTransferPurpose;
use crate::entity::spot::spot_order::{
    SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce,
};
use crate::entity::spot::spot_order_v2::{SpotOrderV2, SpotOrderV2MatchError, SpotTradeFeeRole};
use crate::entity::{
    Balance, Reservation, ReservationCloseReason, ReservationConsumed, ReservationKind,
    ReservationMarketKind, ReservationReleased, SettlementTransferVoucher,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlaceSpotOrderV2Cmd;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CancelSpotOrderV2Cmd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV2Command {
    Place(PlaceSpotOrderV2Cmd),
    Cancel(CancelSpotOrderV2Cmd),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderV2GivenState<'a> {
    Place {
        taker_order: &'a SpotOrderV2,
        maker_orders: &'a [SpotOrderV2],
        taker_principal_reservation: &'a Reservation,
        taker_fee_reservation: &'a Reservation,
        maker_principal_reservations: &'a [Reservation],
        maker_fee_reservations: &'a [Reservation],
        settlement_balances: &'a [Balance],
        base_asset_id: &'a str,
        quote_asset_id: &'a str,
        fee_account_id: &'a str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    },
    Cancel {
        order: &'a SpotOrderV2,
        principal_reservation: &'a Reservation,
        fee_reservation: &'a Reservation,
        balances: &'a [Balance],
        base_asset_id: &'a str,
        quote_asset_id: &'a str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2AfterChanges {
    pub taker_order_after: SpotOrderV2,
    pub maker_orders_after: Vec<SpotOrderV2>,
    pub principal_reservations_after: Vec<Reservation>,
    pub fee_reservations_after: Vec<Reservation>,
    pub balances_after: Vec<Balance>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    pub created_reservation_consumed: Vec<ReservationConsumed>,
    pub created_reservation_released: Vec<ReservationReleased>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2AfterChanges {
    pub order_after: SpotOrderV2,
    pub principal_reservation_after: Reservation,
    pub fee_reservation_after: Reservation,
    pub balances_after: Vec<Balance>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    pub created_reservation_released: Vec<ReservationReleased>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV2AfterChanges {
    Place(PlaceSpotOrderV2AfterChanges),
    Cancel(CancelSpotOrderV2AfterChanges),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2Changes {
    pub updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
    pub updated_principal_reservations: Vec<UpdatedEntityPair<Reservation>>,
    pub updated_fee_reservations: Vec<UpdatedEntityPair<Reservation>>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    pub created_reservation_consumed: Vec<ReservationConsumed>,
    pub created_reservation_released: Vec<ReservationReleased>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_principal_reservation: UpdatedEntityPair<Reservation>,
    pub updated_fee_reservation: UpdatedEntityPair<Reservation>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
    pub created_reservation_released: Vec<ReservationReleased>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV2CaseChanges {
    Place(PlaceSpotOrderV2Changes),
    Cancel(CancelSpotOrderV2Changes),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotOrderV2UseCaseFamilyError {
    #[error("given_state branch does not match command branch")]
    BranchMismatch,
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
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
}

#[derive(Debug, Clone, Default)]
pub struct SpotOrderV2UseCaseFamily;

impl ReplayableChanges for PlaceSpotOrderV2Changes {
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
        events.extend(reservation_replay_events_from_actions(
            &self.updated_principal_reservations,
            &self.created_reservation_consumed,
            &self.created_reservation_released,
        )?);
        events.extend(reservation_replay_events_from_actions(
            &self.updated_fee_reservations,
            &self.created_reservation_consumed,
            &self.created_reservation_released,
        )?);
        for consumed in &self.created_reservation_consumed {
            events.push(consumed.track_create_event()?);
        }
        for released in &self.created_reservation_released {
            events.push(released.track_create_event()?);
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

impl ReplayableChanges for CancelSpotOrderV2Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let mut events = Vec::new();
        events.push(self.updated_order.after.track_update_event_from(&self.updated_order.before)?);
        events.extend(reservation_replay_events_from_actions(
            std::slice::from_ref(&self.updated_principal_reservation),
            &[],
            &self.created_reservation_released,
        )?);
        events.extend(reservation_replay_events_from_actions(
            std::slice::from_ref(&self.updated_fee_reservation),
            &[],
            &self.created_reservation_released,
        )?);
        for released in &self.created_reservation_released {
            events.push(released.track_create_event()?);
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

impl ReplayableChanges for SpotOrderV2CaseChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Place(changes) => changes.to_replayable_events(),
            Self::Cancel(changes) => changes.to_replayable_events(),
        }
    }
}

impl MiStateMachineV2Unchecked for SpotOrderV2UseCaseFamily {
    type Command = SpotOrderV2Command;
    type GivenState<'a>
        = SpotOrderV2GivenState<'a>
    where
        Self: 'a;
    type Error = SpotOrderV2UseCaseFamilyError;
    type AfterChanges = SpotOrderV2AfterChanges;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd {
            SpotOrderV2Command::Place(_) | SpotOrderV2Command::Cancel(_) => Ok(()),
        }
    }

    fn validate_against_given_state<'a>(
        &self,
        cmd: &Self::Command,
        given_state: &SpotOrderV2GivenState<'a>,
    ) -> Result<(), Self::Error> {
        match (cmd, given_state) {
            (
                SpotOrderV2Command::Place(_),
                SpotOrderV2GivenState::Place {
                    taker_order,
                    maker_orders,
                    taker_principal_reservation,
                    taker_fee_reservation,
                    maker_principal_reservations,
                    maker_fee_reservations,
                    base_asset_id,
                    quote_asset_id,
                    fee_account_id,
                    ..
                },
            ) => {
                if fee_account_id.is_empty() {
                    return Err(SpotOrderV2UseCaseFamilyError::InvalidFeeAccountId);
                }
                validate_reservation_for_order(
                    taker_order,
                    taker_principal_reservation,
                    expected_principal_kind_for(taker_order.side),
                    base_asset_id,
                    quote_asset_id,
                )?;
                validate_reservation_for_order(
                    taker_order,
                    taker_fee_reservation,
                    expected_fee_kind_for(taker_order.side),
                    quote_asset_id,
                    quote_asset_id,
                )?;
                if maker_principal_reservations.len() != maker_orders.len()
                    || maker_fee_reservations.len() != maker_orders.len()
                {
                    return Err(SpotOrderV2UseCaseFamilyError::ReservationCountMismatch);
                }
                for (maker, reservation) in
                    maker_orders.iter().zip(maker_principal_reservations.iter())
                {
                    validate_reservation_for_order(
                        maker,
                        reservation,
                        expected_principal_kind_for(maker.side),
                        base_asset_id,
                        quote_asset_id,
                    )?;
                }
                for (maker, reservation) in maker_orders.iter().zip(maker_fee_reservations.iter()) {
                    validate_reservation_for_order(
                        maker,
                        reservation,
                        expected_fee_kind_for(maker.side),
                        quote_asset_id,
                        quote_asset_id,
                    )?;
                }
                taker_order.ensure_matchable()?;
                Ok(())
            }
            (
                SpotOrderV2Command::Cancel(_),
                SpotOrderV2GivenState::Cancel {
                    order,
                    principal_reservation,
                    fee_reservation,
                    base_asset_id,
                    quote_asset_id,
                    ..
                },
            ) => {
                if !matches!(order.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
                {
                    return Err(SpotOrderV2UseCaseFamilyError::BranchMismatch);
                }
                validate_reservation_for_order(
                    order,
                    principal_reservation,
                    expected_principal_kind_for(order.side),
                    base_asset_id,
                    quote_asset_id,
                )?;
                validate_reservation_for_order(
                    order,
                    fee_reservation,
                    expected_fee_kind_for(order.side),
                    quote_asset_id,
                    quote_asset_id,
                )?;
                Ok(())
            }
            _ => Err(SpotOrderV2UseCaseFamilyError::BranchMismatch),
        }
    }

    fn compute_after_changes_unchecked<'a>(
        &self,
        cmd: &Self::Command,
        given_state: &SpotOrderV2GivenState<'a>,
    ) -> Result<Self::AfterChanges, Self::Error> {
        match (cmd, given_state) {
            (
                SpotOrderV2Command::Place(_),
                SpotOrderV2GivenState::Place {
                    taker_order,
                    maker_orders,
                    taker_principal_reservation,
                    taker_fee_reservation,
                    maker_principal_reservations,
                    maker_fee_reservations,
                    settlement_balances,
                    base_asset_id,
                    quote_asset_id,
                    fee_account_id,
                    maker_fee_bps,
                    taker_fee_bps,
                },
            ) => self.compute_place_after(
                taker_order,
                maker_orders,
                taker_principal_reservation,
                taker_fee_reservation,
                maker_principal_reservations,
                maker_fee_reservations,
                settlement_balances,
                base_asset_id,
                quote_asset_id,
                fee_account_id,
                *maker_fee_bps,
                *taker_fee_bps,
            ),
            (
                SpotOrderV2Command::Cancel(_),
                SpotOrderV2GivenState::Cancel {
                    order,
                    principal_reservation,
                    fee_reservation,
                    balances,
                    base_asset_id,
                    quote_asset_id,
                    maker_fee_bps,
                    taker_fee_bps,
                },
            ) => self.compute_cancel_after(
                order,
                principal_reservation,
                fee_reservation,
                balances,
                base_asset_id,
                quote_asset_id,
                *maker_fee_bps,
                *taker_fee_bps,
            ),
            _ => Err(SpotOrderV2UseCaseFamilyError::BranchMismatch),
        }
    }
}

impl MiStateMachineOwnedV2BeforeAfter for SpotOrderV2UseCaseFamily {
    type BeforeAfterChanges = SpotOrderV2CaseChanges;

    fn merge_before_and_after(
        given_state: &SpotOrderV2GivenState<'_>,
        after: Self::AfterChanges,
    ) -> Result<Self::BeforeAfterChanges, Self::Error> {
        match (given_state, after) {
            (
                SpotOrderV2GivenState::Place {
                    taker_order,
                    maker_orders,
                    taker_principal_reservation,
                    taker_fee_reservation,
                    maker_principal_reservations,
                    maker_fee_reservations,
                    settlement_balances,
                    ..
                },
                SpotOrderV2AfterChanges::Place(after),
            ) => Ok(SpotOrderV2CaseChanges::Place(PlaceSpotOrderV2Changes {
                updated_taker_order: UpdatedEntityPair {
                    before: (*taker_order).clone(),
                    after: after.taker_order_after,
                },
                updated_maker_orders: zip_pairs(maker_orders.to_vec(), after.maker_orders_after)?,
                updated_principal_reservations: zip_pairs(
                    std::iter::once((*taker_principal_reservation).clone())
                        .chain(maker_principal_reservations.iter().cloned())
                        .collect(),
                    after.principal_reservations_after,
                )?,
                updated_fee_reservations: zip_pairs(
                    std::iter::once((*taker_fee_reservation).clone())
                        .chain(maker_fee_reservations.iter().cloned())
                        .collect(),
                    after.fee_reservations_after,
                )?,
                updated_balances: merge_balance_pairs(
                    settlement_balances.to_vec(),
                    after.balances_after,
                )?,
                created_trades: after.created_trades,
                created_vouchers: after.created_vouchers,
                created_balance_ledger_entries: after.created_balance_ledger_entries,
                created_reservation_consumed: after.created_reservation_consumed,
                created_reservation_released: after.created_reservation_released,
            })),
            (
                SpotOrderV2GivenState::Cancel {
                    order,
                    principal_reservation,
                    fee_reservation,
                    balances,
                    ..
                },
                SpotOrderV2AfterChanges::Cancel(after),
            ) => Ok(SpotOrderV2CaseChanges::Cancel(CancelSpotOrderV2Changes {
                updated_order: UpdatedEntityPair {
                    before: (*order).clone(),
                    after: after.order_after,
                },
                updated_principal_reservation: UpdatedEntityPair {
                    before: (*principal_reservation).clone(),
                    after: after.principal_reservation_after,
                },
                updated_fee_reservation: UpdatedEntityPair {
                    before: (*fee_reservation).clone(),
                    after: after.fee_reservation_after,
                },
                updated_balances: merge_balance_pairs(balances.to_vec(), after.balances_after)?,
                created_balance_ledger_entries: after.created_balance_ledger_entries,
                created_reservation_released: after.created_reservation_released,
            })),
            _ => Err(SpotOrderV2UseCaseFamilyError::BranchMismatch),
        }
    }
}

impl SpotOrderV2UseCaseFamily {
    #[allow(clippy::too_many_arguments)]
    fn compute_place_after(
        &self,
        taker_order: &SpotOrderV2,
        maker_orders: &[SpotOrderV2],
        taker_principal_reservation: &Reservation,
        taker_fee_reservation: &Reservation,
        maker_principal_reservations: &[Reservation],
        maker_fee_reservations: &[Reservation],
        settlement_balances: &[Balance],
        base_asset_id: &str,
        quote_asset_id: &str,
        fee_account_id: &str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Result<SpotOrderV2AfterChanges, SpotOrderV2UseCaseFamilyError> {
        let mut taker_after = taker_order.clone();
        let mut maker_orders_after = maker_orders.to_vec();
        let mut principal_reservations_after = std::iter::once(taker_principal_reservation.clone())
            .chain(maker_principal_reservations.iter().cloned())
            .collect::<Vec<_>>();
        let mut fee_reservations_after = std::iter::once(taker_fee_reservation.clone())
            .chain(maker_fee_reservations.iter().cloned())
            .collect::<Vec<_>>();
        let mut balance_book = BalanceBook::new(settlement_balances);
        let mut created_trades = Vec::new();
        let mut created_vouchers = Vec::new();
        let mut created_balance_ledger_entries = Vec::new();
        let mut created_reservation_consumed = Vec::new();
        let mut created_reservation_released = Vec::new();

        let best_maker = maker_orders.first();
        let should_match = taker_order.should_enter_matching(best_maker)?;
        if !should_match {
            return Ok(SpotOrderV2AfterChanges::Place(PlaceSpotOrderV2AfterChanges {
                taker_order_after: taker_after,
                maker_orders_after,
                principal_reservations_after,
                fee_reservations_after,
                balances_after: balance_book.into_balances(),
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
                created_reservation_consumed,
                created_reservation_released,
            }));
        }

        if taker_order.would_be_rejected_as_alo(best_maker)? {
            taker_after.reject_as_bad_alo()?;
            release_remaining_for_terminal(
                &taker_after,
                &mut principal_reservations_after[0],
                &mut fee_reservations_after[0],
                &mut balance_book,
                &mut created_balance_ledger_entries,
                &mut created_reservation_released,
                maker_fee_bps,
                taker_fee_bps,
            )?;
            return Ok(SpotOrderV2AfterChanges::Place(PlaceSpotOrderV2AfterChanges {
                taker_order_after: taker_after,
                maker_orders_after,
                principal_reservations_after,
                fee_reservations_after,
                balances_after: balance_book.into_balances(),
                created_trades,
                created_vouchers,
                created_balance_ledger_entries,
                created_reservation_consumed,
                created_reservation_released,
            }));
        }

        let mut total_taker_fill = 0_u64;
        for (index, maker_after) in maker_orders_after.iter_mut().enumerate() {
            maker_after.ensure_matchable()?;
            maker_after.ensure_compatible_maker_for(&taker_after)?;
            if !taker_after.crosses_order(maker_after)? {
                break;
            }
            let maker_price =
                maker_after.limit_price().ok_or(SpotOrderV2MatchError::MakerMustBeLimit)?;
            let trade_qty = taker_after
                .remaining_qty()
                .and_then(|qty| maker_after.remaining_qty().map(|maker_qty| qty.min(maker_qty)))
                .ok_or(SpotOrderV2UseCaseFamilyError::ArithmeticOverflow)?;
            if trade_qty == 0 {
                break;
            }

            let trade = SpotTrade::new(
                format!("spot-trade:{}:{}", taker_after.order_id, created_trades.len() + 1),
                taker_after.order_id.clone(),
                taker_after.asset,
                taker_after.symbol.clone(),
                taker_after.order_id.clone(),
                maker_after.order_id.clone(),
                taker_after.account_id.clone(),
                maker_after.account_id.clone(),
                taker_after.side,
                maker_price,
                trade_qty,
            );
            let trade_notional =
                trade.notional_quote().ok_or(SpotOrderV2UseCaseFamilyError::ArithmeticOverflow)?;

            maker_after.apply_fill(trade_qty)?;
            total_taker_fill = total_taker_fill
                .checked_add(trade_qty)
                .ok_or(SpotOrderV2UseCaseFamilyError::ArithmeticOverflow)?;

            consume_reservation(
                &mut principal_reservations_after[0],
                principal_consume_amount_for_taker(&taker_after, trade_qty, trade_notional),
                ReservationCloseReason::Filled,
                trade.trade_id.clone(),
                &mut created_reservation_consumed,
            )?;
            consume_reservation(
                &mut principal_reservations_after[index + 1],
                principal_consume_amount_for_maker(maker_after, trade_qty, trade_notional),
                ReservationCloseReason::Filled,
                trade.trade_id.clone(),
                &mut created_reservation_consumed,
            )?;

            let taker_fee = taker_after.fee_consume_requirement_for_trade(
                trade_qty,
                maker_price,
                SpotTradeFeeRole::Taker,
                maker_fee_bps,
                taker_fee_bps,
            )?;
            let maker_fee = maker_after.fee_consume_requirement_for_trade(
                trade_qty,
                maker_price,
                SpotTradeFeeRole::Maker,
                maker_fee_bps,
                taker_fee_bps,
            )?;
            consume_reservation(
                &mut fee_reservations_after[0],
                taker_fee.amount,
                ReservationCloseReason::Filled,
                trade.trade_id.clone(),
                &mut created_reservation_consumed,
            )?;
            consume_reservation(
                &mut fee_reservations_after[index + 1],
                maker_fee.amount,
                ReservationCloseReason::Filled,
                trade.trade_id.clone(),
                &mut created_reservation_consumed,
            )?;

            let settlement_id = format!("spot-settlement:{}", trade.trade_id);
            let voucher = SettlementTransferVoucher::build_spot_voucher_with_fees(
                format!("spot-voucher:{}", trade.trade_id),
                settlement_id.clone(),
                &trade,
                base_asset_id,
                quote_asset_id,
                fee_account_id.to_string(),
                buyer_fee_for_trade(&trade, taker_fee.amount, maker_fee.amount),
                seller_fee_for_trade(&trade, taker_fee.amount, maker_fee.amount),
            )
            .ok_or(SpotOrderV2UseCaseFamilyError::ArithmeticOverflow)?;

            apply_trade_balance_effects(
                &trade,
                &settlement_id,
                base_asset_id,
                quote_asset_id,
                fee_account_id,
                taker_fee.amount,
                maker_fee.amount,
                &mut balance_book,
                &mut created_balance_ledger_entries,
            )?;

            created_trades.push(trade);
            created_vouchers.push(voucher);
        }

        let finalization = taker_after.finalize_after_match(total_taker_fill)?;
        taker_after.apply_finalization(finalization)?;
        if taker_after.time_in_force == SpotOrderTimeInForce::Ioc
            && taker_after.remaining_qty() != Some(0)
        {
            release_remaining_for_terminal(
                &taker_after,
                &mut principal_reservations_after[0],
                &mut fee_reservations_after[0],
                &mut balance_book,
                &mut created_balance_ledger_entries,
                &mut created_reservation_released,
                maker_fee_bps,
                taker_fee_bps,
            )?;
        }

        Ok(SpotOrderV2AfterChanges::Place(PlaceSpotOrderV2AfterChanges {
            taker_order_after: taker_after,
            maker_orders_after,
            principal_reservations_after,
            fee_reservations_after,
            balances_after: balance_book.into_balances(),
            created_trades,
            created_vouchers,
            created_balance_ledger_entries,
            created_reservation_consumed,
            created_reservation_released,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_cancel_after(
        &self,
        order: &SpotOrderV2,
        principal_reservation: &Reservation,
        fee_reservation: &Reservation,
        balances: &[Balance],
        _base_asset_id: &str,
        _quote_asset_id: &str,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> Result<SpotOrderV2AfterChanges, SpotOrderV2UseCaseFamilyError> {
        let mut order_after = order.clone();
        order_after.status = SpotOrderStatus::Canceled;
        order_after.status_reason = Some(SpotOrderStatusReason::CanceledByUser);
        order_after.version = order_after
            .version
            .checked_add(1)
            .ok_or(SpotOrderV2UseCaseFamilyError::ArithmeticOverflow)?;

        let mut principal_reservation_after = principal_reservation.clone();
        let mut fee_reservation_after = fee_reservation.clone();
        let mut balance_book = BalanceBook::new(balances);
        let mut created_balance_ledger_entries = Vec::new();
        let mut created_reservation_released = Vec::new();

        release_remaining_for_cancel(
            order,
            &mut principal_reservation_after,
            &mut fee_reservation_after,
            &mut balance_book,
            &mut created_balance_ledger_entries,
            &mut created_reservation_released,
            maker_fee_bps,
            taker_fee_bps,
        )?;

        Ok(SpotOrderV2AfterChanges::Cancel(CancelSpotOrderV2AfterChanges {
            order_after,
            principal_reservation_after,
            fee_reservation_after,
            balances_after: balance_book.into_balances(),
            created_balance_ledger_entries,
            created_reservation_released,
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

fn validate_reservation_for_order(
    order: &SpotOrderV2,
    reservation: &Reservation,
    expected_kind: ReservationKind,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), SpotOrderV2UseCaseFamilyError> {
    if reservation.market_kind != ReservationMarketKind::Spot {
        return Err(SpotOrderV2UseCaseFamilyError::ReservationKindMismatch);
    }
    if reservation.caused_by_order_id != order.order_id {
        return Err(SpotOrderV2UseCaseFamilyError::ReservationOrderMismatch);
    }
    if reservation.reservation_kind != expected_kind {
        return Err(SpotOrderV2UseCaseFamilyError::ReservationKindMismatch);
    }
    let expected_asset = match expected_kind {
        ReservationKind::SpotBuyQuote
        | ReservationKind::SpotBuyFeeQuote
        | ReservationKind::SpotSellFeeQuote => quote_asset_id,
        ReservationKind::SpotSellBase => base_asset_id,
        _ => return Err(SpotOrderV2UseCaseFamilyError::ReservationKindMismatch),
    };
    if reservation.asset_id != expected_asset {
        return Err(SpotOrderV2UseCaseFamilyError::ReservationAssetMismatch);
    }
    Ok(())
}

fn principal_consume_amount_for_taker(
    order: &SpotOrderV2,
    trade_qty: u64,
    trade_notional: u64,
) -> u64 {
    match order.side {
        SpotOrderSide::Buy => trade_notional,
        SpotOrderSide::Sell => trade_qty,
    }
}

fn principal_consume_amount_for_maker(
    order: &SpotOrderV2,
    trade_qty: u64,
    trade_notional: u64,
) -> u64 {
    match order.side {
        SpotOrderSide::Buy => trade_notional,
        SpotOrderSide::Sell => trade_qty,
    }
}

fn buyer_fee_for_trade(trade: &SpotTrade, taker_fee: u64, maker_fee: u64) -> u64 {
    match trade.taker_side {
        SpotOrderSide::Buy => taker_fee,
        SpotOrderSide::Sell => maker_fee,
    }
}

fn seller_fee_for_trade(trade: &SpotTrade, taker_fee: u64, maker_fee: u64) -> u64 {
    match trade.taker_side {
        SpotOrderSide::Buy => maker_fee,
        SpotOrderSide::Sell => taker_fee,
    }
}

fn consume_reservation(
    reservation: &mut Reservation,
    amount: u64,
    terminal_reason: ReservationCloseReason,
    caused_by_ref_id: String,
    out: &mut Vec<ReservationConsumed>,
) -> Result<(), SpotOrderV2UseCaseFamilyError> {
    if amount == 0 {
        return Ok(());
    }
    let close_reason =
        if reservation.remaining_amount == amount { Some(terminal_reason) } else { None };
    let after =
        reservation.consume(amount, close_reason).map_err(map_reservation_error_to_family)?;
    *reservation = after.clone();
    out.push(ReservationConsumed::new(
        format!("reservation-consumed:{}:{}", reservation.reservation_id, out.len() + 1),
        &after,
        amount,
        caused_by_ref_id,
    ));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn release_remaining_for_terminal(
    order_after: &SpotOrderV2,
    principal_reservation: &mut Reservation,
    fee_reservation: &mut Reservation,
    balance_book: &mut BalanceBook,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    released_events: &mut Vec<ReservationReleased>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), SpotOrderV2UseCaseFamilyError> {
    if let Some(requirement) = order_after.terminal_release_requirement() {
        let release_amount = principal_reservation.remaining_amount.min(requirement.amount);
        if release_amount > 0 {
            let close_reason = match requirement.reason {
                crate::SpotOrderReleaseReason::Canceled => ReservationCloseReason::Canceled,
                crate::SpotOrderReleaseReason::IocUnfilled => {
                    ReservationCloseReason::IocRemainderCanceled
                }
                crate::SpotOrderReleaseReason::Rejected => ReservationCloseReason::Rejected,
                crate::SpotOrderReleaseReason::FilledCleanup => ReservationCloseReason::Filled,
            };
            let after = principal_reservation
                .release(release_amount, Some(close_reason))
                .map_err(map_reservation_error_to_family)?;
            *principal_reservation = after.clone();
            release_to_balance(
                order_after,
                &after.asset_id,
                release_amount,
                balance_book,
                ledger_entries,
            )?;
            released_events.push(ReservationReleased::new(
                format!(
                    "reservation-released:{}:{}",
                    after.reservation_id,
                    released_events.len() + 1
                ),
                &after,
                release_amount,
                order_after.order_id.clone(),
                close_reason,
            ));
        }
    }

    if let Some(requirement) =
        order_after.fee_terminal_release_requirement(maker_fee_bps, taker_fee_bps)
    {
        let release_amount = fee_reservation.remaining_amount.min(requirement.amount);
        if release_amount > 0 {
            let close_reason = match requirement.reason {
                crate::SpotOrderReleaseReason::Canceled => ReservationCloseReason::Canceled,
                crate::SpotOrderReleaseReason::IocUnfilled => {
                    ReservationCloseReason::IocRemainderCanceled
                }
                crate::SpotOrderReleaseReason::Rejected => ReservationCloseReason::Rejected,
                crate::SpotOrderReleaseReason::FilledCleanup => ReservationCloseReason::Filled,
            };
            let after = fee_reservation
                .release(release_amount, Some(close_reason))
                .map_err(map_reservation_error_to_family)?;
            *fee_reservation = after.clone();
            release_to_balance(
                order_after,
                &after.asset_id,
                release_amount,
                balance_book,
                ledger_entries,
            )?;
            released_events.push(ReservationReleased::new(
                format!(
                    "reservation-released:{}:{}",
                    after.reservation_id,
                    released_events.len() + 1
                ),
                &after,
                release_amount,
                order_after.order_id.clone(),
                close_reason,
            ));
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn release_remaining_for_cancel(
    order: &SpotOrderV2,
    principal_reservation: &mut Reservation,
    fee_reservation: &mut Reservation,
    balance_book: &mut BalanceBook,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    released_events: &mut Vec<ReservationReleased>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), SpotOrderV2UseCaseFamilyError> {
    if let Some(requirement) = order.cancel_release_requirement() {
        let release_amount = principal_reservation.remaining_amount.min(requirement.amount);
        if release_amount > 0 {
            let after = principal_reservation
                .release(release_amount, Some(ReservationCloseReason::Canceled))
                .map_err(map_reservation_error_to_family)?;
            *principal_reservation = after.clone();
            release_to_balance(
                order,
                &after.asset_id,
                release_amount,
                balance_book,
                ledger_entries,
            )?;
            released_events.push(ReservationReleased::new(
                format!(
                    "reservation-released:{}:{}",
                    after.reservation_id,
                    released_events.len() + 1
                ),
                &after,
                release_amount,
                order.order_id.clone(),
                ReservationCloseReason::Canceled,
            ));
        }
    }
    if let Some(requirement) = order.fee_cancel_release_requirement(maker_fee_bps, taker_fee_bps) {
        let release_amount = fee_reservation.remaining_amount.min(requirement.amount);
        if release_amount > 0 {
            let after = fee_reservation
                .release(release_amount, Some(ReservationCloseReason::Canceled))
                .map_err(map_reservation_error_to_family)?;
            *fee_reservation = after.clone();
            release_to_balance(
                order,
                &after.asset_id,
                release_amount,
                balance_book,
                ledger_entries,
            )?;
            released_events.push(ReservationReleased::new(
                format!(
                    "reservation-released:{}:{}",
                    after.reservation_id,
                    released_events.len() + 1
                ),
                &after,
                release_amount,
                order.order_id.clone(),
                ReservationCloseReason::Canceled,
            ));
        }
    }
    Ok(())
}

fn release_to_balance(
    order: &SpotOrderV2,
    asset_id: &str,
    amount: u64,
    balance_book: &mut BalanceBook,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), SpotOrderV2UseCaseFamilyError> {
    let reason = match order.side {
        SpotOrderSide::Buy => {
            BalanceLedgerReason::CancelSpotOrderReleaseQuote { order_id: order.order_id.clone() }
        }
        SpotOrderSide::Sell => {
            BalanceLedgerReason::CancelSpotOrderReleaseBase { order_id: order.order_id.clone() }
        }
    };
    let balance = balance_book.get_mut(&order.account_id, asset_id)?;
    let entry = BalanceLedgerEntryV2::unfreeze(
        format!("balance-ledger:{}:release:{}", order.order_id, ledger_entries.len() + 1),
        balance,
        amount,
        reason,
    )?;
    ledger_entries.push(entry);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn apply_trade_balance_effects(
    trade: &SpotTrade,
    settlement_id: &str,
    base_asset_id: &str,
    quote_asset_id: &str,
    fee_account_id: &str,
    taker_fee: u64,
    maker_fee: u64,
    balance_book: &mut BalanceBook,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), SpotOrderV2UseCaseFamilyError> {
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
        trade.notional_quote().ok_or(SpotOrderV2UseCaseFamilyError::ArithmeticOverflow)?;
    let buyer_account_id = trade.buyer_account_id().to_string();
    let seller_account_id = trade.seller_account_id().to_string();

    {
        let balance = balance_book.get_mut(&buyer_account_id, base_asset_id)?;
        ledger_entries.push(BalanceLedgerEntryV2::credit_available(
            format!("balance-ledger:{}:buyer-base", settlement_id),
            balance,
            trade.qty,
            buyer_receive_base_reason,
        )?);
    }
    {
        let balance = balance_book.get_mut(&buyer_account_id, quote_asset_id)?;
        ledger_entries.push(BalanceLedgerEntryV2::debit_frozen(
            format!("balance-ledger:{}:buyer-quote", settlement_id),
            balance,
            quote_notional,
            buyer_release_quote_reason,
        )?);
    }
    {
        let balance = balance_book.get_mut(&seller_account_id, quote_asset_id)?;
        ledger_entries.push(BalanceLedgerEntryV2::credit_available(
            format!("balance-ledger:{}:seller-quote", settlement_id),
            balance,
            quote_notional,
            seller_receive_quote_reason,
        )?);
    }
    {
        let balance = balance_book.get_mut(&seller_account_id, base_asset_id)?;
        ledger_entries.push(BalanceLedgerEntryV2::debit_frozen(
            format!("balance-ledger:{}:seller-base", settlement_id),
            balance,
            trade.qty,
            seller_release_base_reason,
        )?);
    }
    if let Some((buyer_fee_account_id, buyer_fee_amount)) =
        fee_payer_for_buyer(trade, taker_fee, maker_fee)
    {
        let balance = balance_book.get_mut(&buyer_fee_account_id, quote_asset_id)?;
        ledger_entries.push(BalanceLedgerEntryV2::debit_frozen(
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
        ledger_entries.push(BalanceLedgerEntryV2::credit_available(
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
    if let Some((seller_fee_account_id, seller_fee_amount)) =
        fee_payer_for_seller(trade, taker_fee, maker_fee)
    {
        let balance = balance_book.get_mut(&seller_fee_account_id, quote_asset_id)?;
        ledger_entries.push(BalanceLedgerEntryV2::debit_frozen(
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
        ledger_entries.push(BalanceLedgerEntryV2::credit_available(
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

fn fee_payer_for_buyer(trade: &SpotTrade, taker_fee: u64, maker_fee: u64) -> Option<(String, u64)> {
    let amount = buyer_fee_for_trade(trade, taker_fee, maker_fee);
    if amount == 0 { None } else { Some((trade.buyer_account_id().to_string(), amount)) }
}

fn fee_payer_for_seller(
    trade: &SpotTrade,
    taker_fee: u64,
    maker_fee: u64,
) -> Option<(String, u64)> {
    let amount = seller_fee_for_trade(trade, taker_fee, maker_fee);
    if amount == 0 { None } else { Some((trade.seller_account_id().to_string(), amount)) }
}

fn map_reservation_error_to_family(
    error: crate::ReservationError,
) -> SpotOrderV2UseCaseFamilyError {
    match error {
        crate::ReservationError::ArithmeticOverflow => {
            SpotOrderV2UseCaseFamilyError::ArithmeticOverflow
        }
        crate::ReservationError::AmountExceedsRemaining => {
            SpotOrderV2UseCaseFamilyError::InsufficientFrozenBalance
        }
        crate::ReservationError::AlreadyClosed
        | crate::ReservationError::InvalidAmount
        | crate::ReservationError::InvalidOriginalAmount
        | crate::ReservationError::MissingCloseReason => {
            SpotOrderV2UseCaseFamilyError::BranchMismatch
        }
    }
}

fn zip_pairs<T>(
    before: Vec<T>,
    after: Vec<T>,
) -> Result<Vec<UpdatedEntityPair<T>>, SpotOrderV2UseCaseFamilyError> {
    if before.len() != after.len() {
        return Err(SpotOrderV2UseCaseFamilyError::ReservationCountMismatch);
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
) -> Result<Vec<UpdatedEntityPair<Balance>>, SpotOrderV2UseCaseFamilyError> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
        let before_balance = before_map
            .get(&balance_id)
            .cloned()
            .ok_or(SpotOrderV2UseCaseFamilyError::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(SpotOrderV2UseCaseFamilyError::BalanceNotFound);
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
        if before.available != entry.before_available || before.frozen != entry.before_frozen {
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
            entry.after_available,
            entry.after_frozen,
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

fn reservation_replay_events_from_actions(
    updated_reservations: &[UpdatedEntityPair<Reservation>],
    consumed_events: &[ReservationConsumed],
    released_events: &[ReservationReleased],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut current_reservations =
        HashMap::<String, Reservation>::with_capacity(updated_reservations.len());
    let mut expected_after_reservations =
        HashMap::<String, Reservation>::with_capacity(updated_reservations.len());
    for reservation in updated_reservations {
        let reservation_id = reservation.before.entity_id();
        current_reservations.insert(reservation_id.clone(), reservation.before.clone());
        expected_after_reservations.insert(reservation_id, reservation.after.clone());
    }

    let mut events = Vec::new();
    for consumed in consumed_events {
        let Some(before) = current_reservations.get(&consumed.reservation_id).cloned() else {
            continue;
        };
        let close_reason = if consumed.remaining_amount_after == 0 {
            Some(ReservationCloseReason::Filled)
        } else {
            None
        };
        let after = before
            .consume(consumed.amount, close_reason)
            .map_err(|error| common_entity::EntityError::Custom(error.to_string()))?;
        if after.remaining_amount != consumed.remaining_amount_after {
            return Err(common_entity::EntityError::Custom(
                "reservation consume replay chain mismatch".to_string(),
            ));
        }
        events.push(after.track_update_event_from(&before)?);
        current_reservations.insert(consumed.reservation_id.clone(), after);
    }

    for released in released_events {
        let Some(before) = current_reservations.get(&released.reservation_id).cloned() else {
            continue;
        };
        let after = before
            .release(released.amount, Some(released.close_reason))
            .map_err(|error| common_entity::EntityError::Custom(error.to_string()))?;
        if after.remaining_amount != released.remaining_amount_after {
            return Err(common_entity::EntityError::Custom(
                "reservation release replay chain mismatch".to_string(),
            ));
        }
        events.push(after.track_update_event_from(&before)?);
        current_reservations.insert(released.reservation_id.clone(), after);
    }

    for (reservation_id, expected_after) in expected_after_reservations {
        if current_reservations.get(&reservation_id) != Some(&expected_after) {
            return Err(common_entity::EntityError::Custom(
                "reservation replay chain does not reach case-level after state".to_string(),
            ));
        }
    }
    Ok(events)
}

struct BalanceBook {
    balances: HashMap<String, Balance>,
}

impl BalanceBook {
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
    ) -> Result<&mut Balance, SpotOrderV2UseCaseFamilyError> {
        self.balances
            .get_mut(&format!("{account_id}:{asset_id}"))
            .ok_or(SpotOrderV2UseCaseFamilyError::BalanceNotFound)
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
    use crate::{SpotOrderExecution, SpotOrderTimeInForce};

    fn buy_order(tif: SpotOrderTimeInForce) -> SpotOrderV2 {
        SpotOrderV2::new(
            "taker-buy".to_string(),
            10_001,
            Some(1),
            "buyer".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            tif,
            2,
            0,
            SpotOrderStatus::Open,
            None,
            0,
            200,
            None,
            1,
        )
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
            None,
            1,
        )
    }

    fn reservation(
        order_id: &str,
        account_id: &str,
        kind: ReservationKind,
        asset_id: &str,
        amount: u64,
    ) -> Reservation {
        Reservation::new(
            format!("reservation:{order_id}:{asset_id}:{kind:?}"),
            account_id.to_string(),
            order_id.to_string(),
            ReservationMarketKind::Spot,
            kind,
            asset_id.to_string(),
            amount,
        )
        .unwrap()
    }

    fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
        Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 1)
    }

    #[test]
    fn place_gtc_without_cross_keeps_state_and_outputs_no_side_effects() {
        let family = SpotOrderV2UseCaseFamily;
        let taker = buy_order(SpotOrderTimeInForce::Gtc);
        let makers = vec![sell_order("maker-1", "seller", 110, 1)];
        let principal = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyQuote,
            "USDT",
            200,
        );
        let fee = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyFeeQuote,
            "USDT",
            1,
        );
        let maker_principal =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellBase, "BTC", 1)];
        let maker_fee =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellFeeQuote, "USDT", 1)];
        let balances = vec![
            balance("buyer", "USDT", 1000, 201),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenState::Place {
            taker_order: &taker,
            maker_orders: &makers,
            taker_principal_reservation: &principal,
            taker_fee_reservation: &fee,
            maker_principal_reservations: &maker_principal,
            maker_fee_reservations: &maker_fee,
            settlement_balances: &balances,
            base_asset_id: "BTC",
            quote_asset_id: "USDT",
            fee_account_id: "fee",
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2AfterChanges::Place(after) = family
            .compute_after_changes(&SpotOrderV2Command::Place(Default::default()), &state)
            .unwrap()
        else {
            panic!("expected place after changes");
        };

        assert_eq!(after.taker_order_after, taker);
        assert_eq!(after.maker_orders_after, makers);
        assert!(after.created_trades.is_empty());
        assert!(after.created_vouchers.is_empty());
        assert!(after.created_balance_ledger_entries.is_empty());
        assert!(after.created_reservation_consumed.is_empty());
        assert!(after.created_reservation_released.is_empty());
    }

    #[test]
    fn place_ioc_partial_fill_releases_remainder() {
        let family = SpotOrderV2UseCaseFamily;
        let taker = buy_order(SpotOrderTimeInForce::Ioc);
        let makers = vec![sell_order("maker-1", "seller", 100, 1)];
        let principal = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyQuote,
            "USDT",
            200,
        );
        let fee = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyFeeQuote,
            "USDT",
            1,
        );
        let maker_principal =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellBase, "BTC", 1)];
        let maker_fee =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellFeeQuote, "USDT", 1)];
        let balances = vec![
            balance("buyer", "USDT", 1000, 201),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenState::Place {
            taker_order: &taker,
            maker_orders: &makers,
            taker_principal_reservation: &principal,
            taker_fee_reservation: &fee,
            maker_principal_reservations: &maker_principal,
            maker_fee_reservations: &maker_fee,
            settlement_balances: &balances,
            base_asset_id: "BTC",
            quote_asset_id: "USDT",
            fee_account_id: "fee",
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2CaseChanges::Place(changes) = family
            .compute_before_after_changes(&SpotOrderV2Command::Place(Default::default()), &state)
            .unwrap()
        else {
            panic!("expected place case changes");
        };

        assert_eq!(changes.created_trades.len(), 1);
        assert_eq!(changes.updated_taker_order.after.status, SpotOrderStatus::Canceled);
        assert_eq!(
            changes.updated_taker_order.after.status_reason,
            Some(SpotOrderStatusReason::IocCancelRejected)
        );
        assert!(!changes.created_reservation_released.is_empty());
        assert!(!changes.created_balance_ledger_entries.is_empty());
        assert!(!changes.to_replayable_events().unwrap().is_empty());
    }

    #[test]
    fn merge_before_after_extracts_before_truth_from_given_state() {
        let family = SpotOrderV2UseCaseFamily;
        let taker = buy_order(SpotOrderTimeInForce::Ioc);
        let makers = vec![sell_order("maker-1", "seller", 100, 1)];
        let principal = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyQuote,
            "USDT",
            200,
        );
        let fee = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyFeeQuote,
            "USDT",
            1,
        );
        let maker_principal =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellBase, "BTC", 1)];
        let maker_fee =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellFeeQuote, "USDT", 1)];
        let balances = vec![
            balance("buyer", "USDT", 1000, 201),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenState::Place {
            taker_order: &taker,
            maker_orders: &makers,
            taker_principal_reservation: &principal,
            taker_fee_reservation: &fee,
            maker_principal_reservations: &maker_principal,
            maker_fee_reservations: &maker_fee,
            settlement_balances: &balances,
            base_asset_id: "BTC",
            quote_asset_id: "USDT",
            fee_account_id: "fee",
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let after = family
            .compute_after_changes(&SpotOrderV2Command::Place(Default::default()), &state)
            .unwrap();

        let SpotOrderV2CaseChanges::Place(changes) =
            SpotOrderV2UseCaseFamily::merge_before_and_after(&state, after).unwrap()
        else {
            panic!("expected place case changes");
        };

        assert_eq!(changes.updated_taker_order.before, taker);
        assert_eq!(changes.updated_maker_orders[0].before, makers[0]);
        assert_eq!(changes.updated_principal_reservations[0].before, principal);
        assert_eq!(changes.updated_fee_reservations[0].before, fee);
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
        let family = SpotOrderV2UseCaseFamily;
        let taker = buy_order(SpotOrderTimeInForce::Alo);
        let makers = vec![sell_order("maker-1", "seller", 99, 1)];
        let principal = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyQuote,
            "USDT",
            200,
        );
        let fee = reservation(
            &taker.order_id,
            &taker.account_id,
            ReservationKind::SpotBuyFeeQuote,
            "USDT",
            1,
        );
        let maker_principal =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellBase, "BTC", 1)];
        let maker_fee =
            vec![reservation("maker-1", "seller", ReservationKind::SpotSellFeeQuote, "USDT", 1)];
        let balances = vec![
            balance("buyer", "USDT", 1000, 201),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let state = SpotOrderV2GivenState::Place {
            taker_order: &taker,
            maker_orders: &makers,
            taker_principal_reservation: &principal,
            taker_fee_reservation: &fee,
            maker_principal_reservations: &maker_principal,
            maker_fee_reservations: &maker_fee,
            settlement_balances: &balances,
            base_asset_id: "BTC",
            quote_asset_id: "USDT",
            fee_account_id: "fee",
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2AfterChanges::Place(after) = family
            .compute_after_changes(&SpotOrderV2Command::Place(Default::default()), &state)
            .unwrap()
        else {
            panic!("expected place after");
        };

        assert_eq!(after.taker_order_after.status, SpotOrderStatus::Rejected);
        assert_eq!(
            after.taker_order_after.status_reason,
            Some(SpotOrderStatusReason::BadAloPxRejected)
        );
        assert!(after.created_trades.is_empty());
        assert!(after.created_reservation_consumed.is_empty());
        assert!(!after.created_reservation_released.is_empty());
    }

    #[test]
    fn cancel_open_order_releases_principal_and_fee() {
        let family = SpotOrderV2UseCaseFamily;
        let order = buy_order(SpotOrderTimeInForce::Gtc);
        let principal = reservation(
            &order.order_id,
            &order.account_id,
            ReservationKind::SpotBuyQuote,
            "USDT",
            200,
        );
        let fee = reservation(
            &order.order_id,
            &order.account_id,
            ReservationKind::SpotBuyFeeQuote,
            "USDT",
            1,
        );
        let balances = vec![balance("buyer", "USDT", 1000, 201)];
        let state = SpotOrderV2GivenState::Cancel {
            order: &order,
            principal_reservation: &principal,
            fee_reservation: &fee,
            balances: &balances,
            base_asset_id: "BTC",
            quote_asset_id: "USDT",
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let SpotOrderV2CaseChanges::Cancel(changes) = family
            .compute_before_after_changes(&SpotOrderV2Command::Cancel(CancelSpotOrderV2Cmd), &state)
            .unwrap()
        else {
            panic!("expected cancel changes");
        };

        assert_eq!(changes.updated_order.after.status, SpotOrderStatus::Canceled);
        assert_eq!(
            changes.updated_order.after.status_reason,
            Some(SpotOrderStatusReason::CanceledByUser)
        );
        assert_eq!(changes.created_reservation_released.len(), 2);
        assert!(!changes.created_balance_ledger_entries.is_empty());
        assert!(!changes.to_replayable_events().unwrap().is_empty());
    }

    #[test]
    fn validate_rejects_branch_mismatch() {
        let family = SpotOrderV2UseCaseFamily;
        let order = buy_order(SpotOrderTimeInForce::Gtc);
        let principal = reservation(
            &order.order_id,
            &order.account_id,
            ReservationKind::SpotBuyQuote,
            "USDT",
            200,
        );
        let fee = reservation(
            &order.order_id,
            &order.account_id,
            ReservationKind::SpotBuyFeeQuote,
            "USDT",
            1,
        );
        let balances = vec![balance("buyer", "USDT", 1000, 201)];
        let state = SpotOrderV2GivenState::Cancel {
            order: &order,
            principal_reservation: &principal,
            fee_reservation: &fee,
            balances: &balances,
            base_asset_id: "BTC",
            quote_asset_id: "USDT",
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        assert_eq!(
            family.compute_after_changes(&SpotOrderV2Command::Place(Default::default()), &state),
            Err(SpotOrderV2UseCaseFamilyError::BranchMismatch)
        );
    }
}
