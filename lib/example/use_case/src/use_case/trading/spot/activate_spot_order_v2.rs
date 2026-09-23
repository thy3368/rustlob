use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff,
    StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error,
};
use crate::entity::account::balance_ledger_reason::BalanceLedgerReason;
use crate::entity::{
    ActivatePendingSpotOrderV2Input, Balance, Reservation, SpotOrderV2, SpotOrderV2BehaviorError,
};
use crate::support::{concat2, concat3};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActivateSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV2State {
    pub pending_order: SpotOrderV2,
    pub balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV2AfterChanges {
    pub activated_order_after: SpotOrderV2,
    pub balances_after: Vec<Balance>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV2Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ActivateSpotOrderV2Error {
    #[error("party id must not be empty")]
    InvalidPartyId,
    #[error("order id must not be empty")]
    InvalidOrderId,
    #[error("pending order id does not match command")]
    OrderIdMismatch,
    #[error("pending order account id does not match command")]
    AccountIdMismatch,
    #[error("pending order asset does not match command")]
    AssetMismatch,
    #[error("order is not pending")]
    OrderNotPending,
    #[error("order execution state is inconsistent")]
    InconsistentExecutionState,
    #[error("base asset id must not be empty")]
    InvalidBaseAssetId,
    #[error("quote asset id must not be empty")]
    InvalidQuoteAssetId,
    #[error("balance not found")]
    BalanceNotFound,
    #[error("available balance is insufficient")]
    InsufficientAvailableBalance,
    #[error("arithmetic overflow while computing spot order activation")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    BalanceLedger(BalanceLedgerEntryV2Error),
}

impl From<BalanceLedgerEntryV2Error> for ActivateSpotOrderV2Error {
    fn from(error: BalanceLedgerEntryV2Error) -> Self {
        match error {
            BalanceLedgerEntryV2Error::InsufficientAvailableBalance => {
                Self::InsufficientAvailableBalance
            }
            BalanceLedgerEntryV2Error::ArithmeticOverflow => Self::ArithmeticOverflow,
            other => Self::BalanceLedger(other),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActivateSpotOrderV2UseCase;

impl ReplayableChanges for ActivateSpotOrderV2Changes {
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

impl StateMachineV2Unchecked for ActivateSpotOrderV2UseCase {
    type Command = ActivateSpotOrderV2Cmd;
    type StateGiven = ActivateSpotOrderV2State;
    type Error = ActivateSpotOrderV2Error;
    type StateChanged = ActivateSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(ActivateSpotOrderV2Error::InvalidPartyId);
        }
        if cmd.order_id.is_empty() {
            return Err(ActivateSpotOrderV2Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        validate_activation_state(cmd, state)
    }

    fn compute_state_changed_unchecked(
        &self,
        _cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut activated_order_after = state.pending_order.clone();
        activated_order_after.activate_pending(ActivatePendingSpotOrderV2Input {
            base_asset_id: state.base_asset_id.clone(),
            quote_asset_id: state.quote_asset_id.clone(),
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
            timestamp: context.execution_time_ns,
        })?;

        let mut balance_book = BalanceMap::new(&state.balances);
        let mut created_balance_ledger_entries = Vec::with_capacity(2);
        let principal_entry = apply_freeze_for_reservation(
            &activated_order_after,
            &activated_order_after.reservation,
            concat2("balance-ledger:freeze:", activated_order_after.order_id()),
            &mut balance_book,
        )?;
        created_balance_ledger_entries.push(principal_entry);

        if activated_order_after.fee_reservation.original_amount > 0 {
            let fee_entry = apply_freeze_for_reservation(
                &activated_order_after,
                &activated_order_after.fee_reservation,
                concat3("balance-ledger:freeze:", activated_order_after.order_id(), ":fee"),
                &mut balance_book,
            )?;
            created_balance_ledger_entries.push(fee_entry);
        }

        Ok(ActivateSpotOrderV2AfterChanges {
            activated_order_after,
            balances_after: balance_book.into_balances(),
            created_balance_ledger_entries,
        })
    }
}

impl StateMachineOwnedV2Diff for ActivateSpotOrderV2UseCase {
    type StateDiff = ActivateSpotOrderV2Changes;

    fn do_compute_state_diff(
        state: ActivateSpotOrderV2State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(ActivateSpotOrderV2Changes {
            updated_order: UpdatedEntityPair {
                before: state.pending_order,
                after: after.activated_order_after,
            },
            updated_balances: merge_balance_pairs(state.balances, after.balances_after)?,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
    }
}

fn validate_activation_state(
    cmd: &ActivateSpotOrderV2Cmd,
    state: &ActivateSpotOrderV2State,
) -> Result<(), ActivateSpotOrderV2Error> {
    if state.pending_order.order_id() != cmd.order_id {
        return Err(ActivateSpotOrderV2Error::OrderIdMismatch);
    }
    if !state.pending_order.belongs_to_account(&cmd.party_id) {
        return Err(ActivateSpotOrderV2Error::AccountIdMismatch);
    }
    if !state.pending_order.trades_asset(cmd.asset) {
        return Err(ActivateSpotOrderV2Error::AssetMismatch);
    }
    if !state.pending_order.is_pending() {
        return Err(ActivateSpotOrderV2Error::OrderNotPending);
    }
    if !state.pending_order.has_consistent_execution_state() {
        return Err(ActivateSpotOrderV2Error::InconsistentExecutionState);
    }
    if state.base_asset_id.is_empty() {
        return Err(ActivateSpotOrderV2Error::InvalidBaseAssetId);
    }
    if state.quote_asset_id.is_empty() {
        return Err(ActivateSpotOrderV2Error::InvalidQuoteAssetId);
    }

    let principal_reservation = SpotOrderV2::principal_reservation(
        state.pending_order.order_id(),
        state.pending_order.account_id(),
        state.pending_order.side(),
        state.pending_order.qty(),
        state.pending_order.order_price(),
        &state.base_asset_id,
        &state.quote_asset_id,
    )
    .map_err(SpotOrderV2BehaviorError::from)?;
    ensure_balance_exists(&state.balances, &principal_reservation)?;

    let fee_reservation = SpotOrderV2::fee_reservation(
        state.pending_order.order_id(),
        state.pending_order.account_id(),
        state.pending_order.side(),
        state.pending_order.qty(),
        state.pending_order.order_price(),
        &state.quote_asset_id,
        state.maker_fee_bps,
        state.taker_fee_bps,
    )
    .map_err(SpotOrderV2BehaviorError::from)?;
    if fee_reservation.original_amount > 0 {
        ensure_balance_exists(&state.balances, &fee_reservation)?;
    }

    Ok(())
}

fn ensure_balance_exists(
    balances: &[Balance],
    reservation: &Reservation,
) -> Result<(), ActivateSpotOrderV2Error> {
    balances
        .iter()
        .find(|balance| {
            balance.belongs_to_account(&reservation.owner_account_id)
                && balance.is_asset(&reservation.asset_id)
        })
        .map(|_| ())
        .ok_or(ActivateSpotOrderV2Error::BalanceNotFound)
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
    ) -> Result<&mut Balance, ActivateSpotOrderV2Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(ActivateSpotOrderV2Error::BalanceNotFound)
    }

    fn entity_id_for_account_asset(
        &self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<String, ActivateSpotOrderV2Error> {
        self.balances
            .get(&concat3(account_id, ":", asset_id))
            .map(Entity::entity_id)
            .ok_or(ActivateSpotOrderV2Error::BalanceNotFound)
    }

    fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
}

fn apply_freeze_for_reservation(
    order: &SpotOrderV2,
    reservation: &Reservation,
    entry_id: String,
    balance_book: &mut BalanceMap,
) -> Result<BalanceLedgerEntryV2, ActivateSpotOrderV2Error> {
    let mut entry = BalanceLedgerEntryV2::freeze(
        entry_id,
        order.account_id().to_string(),
        reservation.asset_id.clone(),
        balance_book.entity_id_for_account_asset(order.account_id(), &reservation.asset_id)?,
        reservation.original_amount,
        BalanceLedgerReason::FreezeForOrder { order_id: order.order_id().to_string() },
    )
    .map_err(ActivateSpotOrderV2Error::from)?;
    let balance = balance_book.get_mut(order.account_id(), &reservation.asset_id)?;
    entry.apply_to(balance).map_err(ActivateSpotOrderV2Error::from)?;
    Ok(entry)
}

fn merge_balance_pairs(
    before: Vec<Balance>,
    after: Vec<Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, ActivateSpotOrderV2Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.entity_id();
        let before_balance = before_map
            .get(&balance_id)
            .cloned()
            .ok_or(ActivateSpotOrderV2Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(ActivateSpotOrderV2Error::BalanceNotFound);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{
        SpotOrderSide, SpotOrderStatus, SpotOrderTif, SpotOrderTriggerRole, SpotOrderType,
    };

    fn context() -> ExecutionContext {
        ExecutionContext { execution_time_ns: 100 }
    }

    fn cmd(order_id: &str) -> ActivateSpotOrderV2Cmd {
        ActivateSpotOrderV2Cmd {
            party_id: "trader-1".to_string(),
            asset: 10_001,
            order_id: order_id.to_string(),
        }
    }

    fn pending_limit(order_id: &str, side: SpotOrderSide) -> SpotOrderV2 {
        SpotOrderV2::new_pending_limit(
            order_id.to_string(),
            10_001,
            Some(1),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            side,
            2,
            100,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            None,
            1,
            1,
        )
    }

    fn pending_trigger(order_id: &str) -> SpotOrderV2 {
        SpotOrderV2::new_pending_trigger(
            order_id.to_string(),
            10_001,
            Some(1),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            2,
            100,
            SpotOrderType::Trigger {
                is_market: false,
                trigger_price: 90,
                tpsl: SpotOrderTriggerRole::TakeProfit,
            },
            None,
            1,
            1,
        )
    }

    fn state(
        order: SpotOrderV2,
        maker_fee_bps: u64,
        taker_fee_bps: u64,
    ) -> ActivateSpotOrderV2State {
        ActivateSpotOrderV2State {
            pending_order: order,
            balances: vec![
                Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1),
                Balance::new("trader-1".to_string(), "BTC".to_string(), 10, 0, 1),
            ],
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps,
            taker_fee_bps,
        }
    }

    #[test]
    fn buy_activation_freezes_quote_principal_and_quote_fee() {
        let order = pending_limit("buy-1", SpotOrderSide::Buy);

        let changes = ActivateSpotOrderV2UseCase
            .compute_state_diff_with_context(&cmd("buy-1"), state(order, 5, 10), &context())
            .expect("buy activation should compute changes");

        assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Open);
        assert_eq!(changes.updated_order.after.reservation.asset_id, "USDT");
        assert_eq!(changes.updated_order.after.reservation.original_amount, 200);
        assert_eq!(changes.created_balance_ledger_entries.len(), 2);
        assert_eq!(changes.updated_balances[1].before.available, 1_000);
        assert_eq!(changes.updated_balances[1].after.available, 799);
        assert_eq!(changes.updated_balances[1].after.frozen, 201);

        let events = changes.to_replayable_events().expect("activation should project events");
        assert_eq!(events.len(), 5);
        assert!(events[0].is_updated());
        assert!(events[1].is_updated());
        assert!(events[2].is_updated());
        assert!(events[3].is_created());
        assert!(events[4].is_created());
    }

    #[test]
    fn sell_activation_freezes_base_principal() {
        let order = pending_limit("sell-1", SpotOrderSide::Sell);

        let changes = ActivateSpotOrderV2UseCase
            .compute_state_diff_with_context(&cmd("sell-1"), state(order, 5, 10), &context())
            .expect("sell activation should compute changes");

        assert_eq!(changes.updated_order.after.reservation.asset_id, "BTC");
        assert_eq!(changes.updated_order.after.reservation.original_amount, 2);
        assert_eq!(changes.updated_balances[0].before.available, 10);
        assert_eq!(changes.updated_balances[0].after.available, 8);
    }

    #[test]
    fn zero_fee_activation_skips_fee_ledger() {
        let order = pending_limit("zero-fee", SpotOrderSide::Buy);

        let changes = ActivateSpotOrderV2UseCase
            .compute_state_diff_with_context(&cmd("zero-fee"), state(order, 0, 0), &context())
            .expect("zero fee activation should compute changes");

        assert_eq!(changes.updated_order.after.fee_reservation.original_amount, 0);
        assert_eq!(changes.created_balance_ledger_entries.len(), 1);
        assert_eq!(changes.to_replayable_events().expect("events should project").len(), 3);
    }

    #[test]
    fn pending_trigger_activation_does_not_need_market_price() {
        let order = pending_trigger("trigger-1");

        let changes = ActivateSpotOrderV2UseCase
            .compute_state_diff_with_context(&cmd("trigger-1"), state(order, 5, 10), &context())
            .expect("trigger activation should compute without market price");

        assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Open);
        assert!(matches!(changes.updated_order.after.order_type, SpotOrderType::Trigger { .. }));
    }

    #[test]
    fn open_order_is_rejected_by_activation_state_validation() {
        let mut order = pending_limit("open-1", SpotOrderSide::Buy);
        order
            .activate_pending(ActivatePendingSpotOrderV2Input {
                base_asset_id: "BTC".to_string(),
                quote_asset_id: "USDT".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
                timestamp: 2,
            })
            .expect("fixture activation should work");

        let result =
            ActivateSpotOrderV2UseCase.validate_state_given(&cmd("open-1"), &state(order, 5, 10));

        assert_eq!(result, Err(ActivateSpotOrderV2Error::OrderNotPending));
    }
}
