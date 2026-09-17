use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, StateMachineOwnedV2Diff, MiStateMachineV2Unchecked,
    ReplayableChanges,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error, BalanceLedgerOperation,
};
use crate::entity::account::balance_ledger_reason::BalanceLedgerReason;
use crate::entity::spot::spot_order_v2::{
    SpotOrderV2, SpotOrderV2BehaviorError, SpotOrderV2MatchError,
};
use crate::entity::{
    Balance, CancelSpotOrderV2Input, Reservation, ReservationCloseReason, ReservationKind,
    ReservationMarketKind, SpotOrderSide,
};
use crate::support::{concat3, concat4};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CancelSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub lookup: CancelSpotOrderV2Lookup,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CancelSpotOrderV2Lookup {
    #[default]
    Missing,
    Oid(u64),
    Cloid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2State {
    pub order: SpotOrderV2,
    pub balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2AfterChanges {
    pub order_after: SpotOrderV2,
    pub balances_after: Vec<Balance>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV2Error {
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
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("arithmetic overflow while computing spot order v2 cancel")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderMatch(#[from] SpotOrderV2MatchError),
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderV2UseCase;

impl ReplayableChanges for CancelSpotOrderV2Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let event_capacity = 1usize
            .saturating_add(self.created_balance_ledger_entries.len())
            .saturating_add(self.created_balance_ledger_entries.len());
        let mut events = Vec::with_capacity(event_capacity);
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

impl MiStateMachineV2Unchecked for CancelSpotOrderV2UseCase {
    type Command = CancelSpotOrderV2Cmd;
    type StateGiven = CancelSpotOrderV2State;
    type Error = CancelSpotOrderV2Error;
    type StateChanged = CancelSpotOrderV2AfterChanges;

    fn validate_state_given(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let mut order_after = given_state.order.clone();
        order_after.cancel(CancelSpotOrderV2Input {
            balance_entity_id: balance_entity_id_for_reservation(
                &given_state.balances,
                &given_state.order.reservation,
            )?,
        })?;
        validate_all_reservations_for_order(
            &given_state.order,
            &given_state.base_asset_id,
            &given_state.quote_asset_id,
        )?;
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut order_after = given_state.order.clone();
        let mut balance_book = BalanceMap::new(&given_state.balances);
        let mut created_balance_ledger_entries = Vec::with_capacity(0);

        let cancel_outcome = order_after.cancel(CancelSpotOrderV2Input {
            balance_entity_id: balance_entity_id_for_reservation(
                &given_state.balances,
                &given_state.order.reservation,
            )?,
        })?;
        if let Some(unfreeze_ledger_entry) = cancel_outcome.unfreeze_ledger_entry {
            let unfreeze_ledger_entry =
                apply_behavior_ledger_entry(unfreeze_ledger_entry, &mut balance_book)?;
            created_balance_ledger_entries.push(unfreeze_ledger_entry);
        }

        release_remaining_for_cancel(
            &mut order_after,
            &mut balance_book,
            &mut created_balance_ledger_entries,
            given_state.maker_fee_bps,
            given_state.taker_fee_bps,
        )?;

        Ok(CancelSpotOrderV2AfterChanges {
            order_after,
            balances_after: balance_book.into_balances(),
            created_balance_ledger_entries,
        })
    }
}

impl StateMachineOwnedV2Diff for CancelSpotOrderV2UseCase {
    type StateDiff = CancelSpotOrderV2Changes;

    fn do_compute_state_diff(
        given_state: CancelSpotOrderV2State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(CancelSpotOrderV2Changes {
            updated_order: UpdatedEntityPair {
                before: given_state.order,
                after: after.order_after,
            },
            updated_balances: merge_balance_pairs(given_state.balances, after.balances_after)?,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
    }
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
    ) -> Result<&mut Balance, CancelSpotOrderV2Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(CancelSpotOrderV2Error::BalanceNotFound)
    }

    fn get_by_entity_id_mut(
        &mut self,
        entity_id: &str,
    ) -> Result<&mut Balance, CancelSpotOrderV2Error> {
        self.balances.get_mut(entity_id).ok_or(CancelSpotOrderV2Error::BalanceNotFound)
    }

    fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
}

fn balance_entity_id_for_account_asset(
    balances: &[Balance],
    account_id: &str,
    asset_id: &str,
) -> Result<String, CancelSpotOrderV2Error> {
    balances
        .iter()
        .find(|balance| balance.account_id == account_id && balance.asset_id == asset_id)
        .map(Entity::entity_id)
        .ok_or(CancelSpotOrderV2Error::BalanceNotFound)
}

fn balance_entity_id_for_reservation(
    balances: &[Balance],
    reservation: &Reservation,
) -> Result<String, CancelSpotOrderV2Error> {
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
) -> Result<(), CancelSpotOrderV2Error> {
    if reservation.market_kind != ReservationMarketKind::Spot {
        return Err(CancelSpotOrderV2Error::ReservationKindMismatch);
    }
    if reservation.caused_by_order_id != order.order_id() {
        return Err(CancelSpotOrderV2Error::ReservationOrderMismatch);
    }
    if reservation.reservation_kind != expected_kind {
        return Err(CancelSpotOrderV2Error::ReservationKindMismatch);
    }
    let expected_asset = match expected_kind {
        ReservationKind::SpotBuyQuote
        | ReservationKind::SpotBuyFeeQuote
        | ReservationKind::SpotSellFeeQuote => quote_asset_id,
        ReservationKind::SpotSellBase => base_asset_id,
        _ => return Err(CancelSpotOrderV2Error::ReservationKindMismatch),
    };
    if reservation.asset_id != expected_asset {
        return Err(CancelSpotOrderV2Error::ReservationAssetMismatch);
    }
    Ok(())
}

fn validate_all_reservations_for_order(
    order: &SpotOrderV2,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), CancelSpotOrderV2Error> {
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

enum OrderReservationSlot {
    Fee,
}

fn release_remaining_for_cancel(
    order: &mut SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), CancelSpotOrderV2Error> {
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

fn release_order_reservation(
    order: &mut SpotOrderV2,
    slot: OrderReservationSlot,
    max_amount: u64,
    close_reason: ReservationCloseReason,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), CancelSpotOrderV2Error> {
    let (asset_id, release_amount) = {
        let reservation = match slot {
            OrderReservationSlot::Fee => &mut order.fee_reservation,
        };
        let release_amount = reservation.remaining_amount.min(max_amount);
        if release_amount == 0 {
            return Ok(());
        }
        let asset_id = reservation.asset_id.clone();
        reservation
            .release(release_amount, Some(close_reason))
            .map_err(map_reservation_error_to_cancel)?;
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
) -> Result<(), CancelSpotOrderV2Error> {
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
        ledger_entries.len().checked_add(1).ok_or(CancelSpotOrderV2Error::ArithmeticOverflow)?;
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

fn apply_behavior_ledger_entry(
    mut entry: BalanceLedgerEntryV2,
    balance_book: &mut BalanceMap,
) -> Result<BalanceLedgerEntryV2, CancelSpotOrderV2Error> {
    let balance = balance_book.get_by_entity_id_mut(&entry.balance_entity_id)?;
    entry.apply_to(balance)?;
    Ok(entry)
}

fn merge_balance_pairs(
    before: Vec<Balance>,
    after: Vec<Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, CancelSpotOrderV2Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
        let before_balance =
            before_map.get(&balance_id).cloned().ok_or(CancelSpotOrderV2Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(CancelSpotOrderV2Error::BalanceNotFound);
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

fn map_reservation_error_to_cancel(error: crate::ReservationError) -> CancelSpotOrderV2Error {
    match error {
        crate::ReservationError::ArithmeticOverflow => CancelSpotOrderV2Error::ArithmeticOverflow,
        crate::ReservationError::AmountExceedsRemaining => {
            CancelSpotOrderV2Error::InsufficientFrozenBalance
        }
        crate::ReservationError::AlreadyClosed
        | crate::ReservationError::InvalidAmount
        | crate::ReservationError::InvalidOriginalAmount
        | crate::ReservationError::MissingCloseReason => CancelSpotOrderV2Error::ArithmeticOverflow,
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{StateMachineOwnedV2Diff, ReplayableChanges};

    use super::*;
    use crate::{
        Balance, SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
        SpotOrderTimeInForce,
    };

    #[test]
    fn cancel_open_order_releases_principal_and_fee() {
        let order = buy_order();
        let balances = vec![Balance::new("buyer".to_string(), "USDT".to_string(), 1000, 201, 1)];
        let state = CancelSpotOrderV2State {
            order: order.clone(),
            balances,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        let changes = CancelSpotOrderV2UseCase
            .compute_state_diff(&CancelSpotOrderV2Cmd::default(), state)
            .expect("open order cancellation should compute changes");

        assert_eq!(changes.updated_order.before.status(), SpotOrderStatus::Open);
        assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Canceled);
        assert_eq!(
            changes.updated_order.after.status_reason(),
            Some(SpotOrderStatusReason::CanceledByUser)
        );
        assert_eq!(changes.updated_order.after.reservation.remaining_amount, 0);
        assert_eq!(changes.updated_order.after.fee_reservation.remaining_amount, 0);
        assert_eq!(changes.updated_balances.len(), 1);
        assert_eq!(changes.updated_balances[0].before.frozen, 201);
        assert_eq!(changes.updated_balances[0].after.frozen, 0);
        assert_eq!(changes.created_balance_ledger_entries.len(), 2);

        let events = changes.to_replayable_events().expect("cancel changes should project events");
        assert_eq!(events.len(), 5);
        assert!(events.iter().all(|event| event.new_version == event.old_version + 1));
        assert!(events[0].is_updated());
        assert!(events[1].is_updated());
        assert!(events[2].is_updated());
        assert!(events[3].is_created());
        assert!(events[4].is_created());
    }

    fn buy_order() -> SpotOrderV2 {
        let principal_reservation = SpotOrderV2::principal_reservation(
            "order-1",
            "buyer",
            SpotOrderSide::Buy,
            2,
            100,
            "BTC",
            "USDT",
        )
        .expect("principal reservation should be valid");
        let fee_reservation = SpotOrderV2::fee_reservation(
            "order-1",
            "buyer",
            SpotOrderSide::Buy,
            2,
            100,
            "USDT",
            5,
            10,
        )
        .expect("fee reservation should be valid");
        SpotOrderV2::new_with_fee_reservation(
            "order-1".to_string(),
            10000,
            Some(77738308),
            "buyer".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            SpotOrderTimeInForce::Gtc,
            2,
            0,
            SpotOrderStatus::Open,
            None,
            principal_reservation,
            fee_reservation,
            None,
            1,
        )
    }
}
