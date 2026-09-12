use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2Unchecked,
    ReplayableChanges,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::spot_order_v2_use_case_family_v3::{
    BalanceMap, SpotOrderV2UseCaseFamilyV3Error, apply_behavior_ledger_entry,
    balance_entity_id_for_reservation, balance_replay_events_from_ledger_entries,
    merge_balance_pairs, release_remaining_for_cancel, validate_all_reservations_for_order,
};
use crate::entity::account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error,
};
use crate::entity::spot::spot_order_v2::{
    SpotOrderV2, SpotOrderV2BehaviorError, SpotOrderV2MatchError,
};
use crate::entity::{Balance, CancelSpotOrderV2Input};

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

impl From<SpotOrderV2UseCaseFamilyV3Error> for CancelSpotOrderV2Error {
    fn from(error: SpotOrderV2UseCaseFamilyV3Error) -> Self {
        match error {
            SpotOrderV2UseCaseFamilyV3Error::ReservationOrderMismatch => {
                Self::ReservationOrderMismatch
            }
            SpotOrderV2UseCaseFamilyV3Error::ReservationKindMismatch => {
                Self::ReservationKindMismatch
            }
            SpotOrderV2UseCaseFamilyV3Error::ReservationAssetMismatch => {
                Self::ReservationAssetMismatch
            }
            SpotOrderV2UseCaseFamilyV3Error::ReservationCountMismatch => {
                Self::ReservationCountMismatch
            }
            SpotOrderV2UseCaseFamilyV3Error::BalanceNotFound => Self::BalanceNotFound,
            SpotOrderV2UseCaseFamilyV3Error::InsufficientFrozenBalance => {
                Self::InsufficientFrozenBalance
            }
            SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow => Self::ArithmeticOverflow,
            SpotOrderV2UseCaseFamilyV3Error::OrderMatch(error) => Self::OrderMatch(error),
            SpotOrderV2UseCaseFamilyV3Error::OrderBehavior(error) => Self::OrderBehavior(error),
            SpotOrderV2UseCaseFamilyV3Error::BalanceLedger(error) => Self::BalanceLedger(error),
            _ => Self::ArithmeticOverflow,
        }
    }
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
    type GivenState = CancelSpotOrderV2State;
    type Error = CancelSpotOrderV2Error;
    type AfterChanges = CancelSpotOrderV2AfterChanges;

    fn validate_against_given_state(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::GivenState,
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

    fn compute_after_changes_unchecked(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
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

impl MiStateMachineOwnedV2BeforeAfter for CancelSpotOrderV2UseCase {
    type BeforeAfterChanges = CancelSpotOrderV2Changes;

    fn merge_before_and_after(
        given_state: CancelSpotOrderV2State,
        after: Self::AfterChanges,
    ) -> Result<Self::BeforeAfterChanges, Self::Error> {
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

#[cfg(test)]
mod tests {
    use common_entity::{MiStateMachineOwnedV2BeforeAfter, ReplayableChanges};

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
            .compute_before_after_changes(&CancelSpotOrderV2Cmd::default(), state)
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
