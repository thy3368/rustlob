use common_entity::{
    EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::spot::spot_order_v7::{
    CancelSpotOrderV7Output, SpotOrderV7, SpotOrderV7Event, SpotOrderV7Output,
};
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, SpotAmountConversionError, SpotOrderStatus,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CancelSpotOrderV3Cmd {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3AfterChanges {
    pub canceled_order: SpotOrderV7,
    pub balance: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV7>,
    pub updated_balance: UpdatedEntityPair<BalanceV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV3Error {
    #[error("numeric order id must be greater than zero")]
    InvalidOid,
    #[error("client order id must not be empty")]
    EmptyCloid,
    #[error("command client order id does not match loaded order")]
    CloidMismatch,
    #[error("command order id does not match loaded order")]
    OidMismatch,
    #[error("loaded order has no client order id")]
    MissingCloid,
    #[error("order is not cancelable")]
    OrderNotCancelable,
    #[error("balance party does not match order owner")]
    PartyMismatch,
    #[error("balance asset does not match the order release asset")]
    AssetMismatch,
    #[error(transparent)]
    ReleaseRequirement(#[from] SpotAmountConversionError),
    #[error(transparent)]
    BalanceV2(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderV3UseCase;

impl UseCase for CancelSpotOrderV3UseCase {
    type Command = LogEntry<CancelSpotOrderV3Cmd>;
    type StateGiven = CancelSpotOrderV3State;
    type Error = CancelSpotOrderV3Error;
    type StateChanged = CancelSpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match &cmd.payload().order_id {
            OrderId::Oid(oid) if *oid == 0 => return Err(CancelSpotOrderV3Error::InvalidOid),
            OrderId::Cloid(cloid) if cloid.trim().is_empty() => {
                return Err(CancelSpotOrderV3Error::EmptyCloid);
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        match &cmd.payload().order_id {
            OrderId::Oid(oid) => {
                if given_state.order.order_base().order_id != *oid {
                    return Err(CancelSpotOrderV3Error::OidMismatch);
                }
            }
            OrderId::Cloid(expected) => {
                let Some(cloid) = given_state.order.order_base().cloid.as_deref() else {
                    return Err(CancelSpotOrderV3Error::MissingCloid);
                };
                if cloid != expected {
                    return Err(CancelSpotOrderV3Error::CloidMismatch);
                }
            }
        }
        if !matches!(
            given_state.order.status(),
            SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) {
            return Err(CancelSpotOrderV3Error::OrderNotCancelable);
        }
        if given_state.balance.party_id() != given_state.order.order_base().owner.0 {
            return Err(CancelSpotOrderV3Error::PartyMismatch);
        }
        let mut canceled = given_state.order.clone();
        let (_, output) =
            canceled.transition(SpotOrderV7Event::Cancel { timestamp: cmd.logical_ts() })?;
        let SpotOrderV7Output::Canceled(CancelSpotOrderV7Output {
            release_requirement: requirement,
            ..
        }) = output
        else {
            return Err(
                EntityError::Custom("cancel event produced mismatched output".into()).into()
            );
        };
        if given_state.balance.coin_idx() != u64::from(requirement.currency().index()) {
            return Err(CancelSpotOrderV3Error::AssetMismatch);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut canceled_order = given_state.order.clone();
        let (_, output) =
            canceled_order.transition(SpotOrderV7Event::Cancel { timestamp: cmd.logical_ts() })?;
        let SpotOrderV7Output::Canceled(CancelSpotOrderV7Output {
            order_after,
            release_requirement: requirement,
        }) = output
        else {
            return Err(
                EntityError::Custom("cancel event produced mismatched output".into()).into()
            );
        };
        if given_state.balance.coin_idx() != u64::from(requirement.currency().index()) {
            return Err(CancelSpotOrderV3Error::AssetMismatch);
        }
        let mut balance = given_state.balance.clone();
        common_entity::Fsm::apply(
            &mut balance,
            BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: requirement.amount(),
                payment_amount: Decimal::ZERO,
                timestamp: cmd.logical_ts(),
            },
        )?;
        Ok(CancelSpotOrderV3AfterChanges { canceled_order: order_after, balance })
    }
}

impl StateDiff for CancelSpotOrderV3UseCase {
    type StateDiff = CancelSpotOrderV3Changes;

    fn do_compute_state_diff(
        given_state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(CancelSpotOrderV3Changes {
            updated_order: UpdatedEntityPair::new(given_state.order, after.canceled_order),
            updated_balance: UpdatedEntityPair::new(given_state.balance, after.balance),
        })
    }
}

impl ReplayableChanges for CancelSpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.updated_order.to_replayable_event()?,
            self.updated_balance.to_replayable_event()?,
        ])
    }
}
