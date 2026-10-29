use common_entity::{LogEntry, StateMachineV2};
use example_core_entity::spot::spot_order_v3::MatchResult;
use example_core_entity::SpotOrderV3;
use serde::{Deserialize, Serialize};
use spot_entity::spot_order_v2::{SpotOrderV2BehaviorError, SpotOrderV2MatchError};
use thiserror::Error;

use crate::common_entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error;
use crate::common_entity::spot as spot_entity;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchSpotOrderV3Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderV3Error {
    #[error("party id must not be empty")]
    InvalidPartyId,
    #[error("order oid must be greater than zero")]
    InvalidOrderId,
    #[error("taker order id does not match on_command")]
    TakerOrderIdMismatch,
    #[error("taker account id does not match on_command")]
    TakerAccountIdMismatch,
    #[error("taker asset does not match on_command")]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderV3State {
    pub taker_order: SpotOrderV3,
    pub maker_orders: Vec<SpotOrderV3>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderV3UseCase;

impl StateMachineV2 for MatchSpotOrderV3UseCase {
    type Command = LogEntry<MatchSpotOrderV3Cmd>;
    type StateGiven = MatchSpotOrderV3State;
    type Error = MatchSpotOrderV3Error;
    type StateChanged = MatchResult;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        state.taker_order.matchOrder(state.maker_orders);

        Ok(Self::StateChanged {})
    }
}
