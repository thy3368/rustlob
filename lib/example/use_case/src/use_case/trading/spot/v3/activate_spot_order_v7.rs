use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff, UpdatedEntityPair,
    UseCase,
};
use example_core_entity::spot::spot_order_v7::SpotOrderV7;
use example_core_entity::{BalanceV2, SpotOrderStatus};
use serde::{Deserialize, Serialize};

use super::spot_order_v7_domain_service::SpotOrderV7DomainService;
pub(crate) use crate::spot_order_v7_domain_service::ActivateSpotOrderV7Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivateSpotOrderV7Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV7State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV7AfterChanges {
    pub order_after: SpotOrderV7,
    pub balance_after: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV7Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV7>,
    pub updated_balance: UpdatedEntityPair<BalanceV2>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActivateSpotOrderV7UseCase;

impl UseCase for ActivateSpotOrderV7UseCase {
    type Command = LogEntry<ActivateSpotOrderV7Cmd>;
    type StateGiven = ActivateSpotOrderV7State;
    type Error = ActivateSpotOrderV7Error;
    type StateChanged = ActivateSpotOrderV7AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.payload().order_id == 0 {
            return Err(ActivateSpotOrderV7Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.order.order_base().order_id != cmd.payload().order_id {
            return Err(ActivateSpotOrderV7Error::OrderMismatch);
        }
        if state.order.status() != SpotOrderStatus::Pending {
            return Err(ActivateSpotOrderV7Error::OrderNotPending);
        }
        if state.balance.party_id() != state.order.order_base().owner.0 {
            return Err(ActivateSpotOrderV7Error::OwnerMismatch);
        }
        let requirement = state.order.freeze_requirement().map_err(|error| {
            ActivateSpotOrderV7Error::Entity(EntityError::Custom(error.to_string()))
        })?;
        if state.balance.coin_idx() != u64::from(requirement.currency().index()) {
            return Err(ActivateSpotOrderV7Error::CoinMismatch);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let (order_after, balance_after) = SpotOrderV7DomainService::freeze(
            state.order.clone(),
            state.balance.clone(),
            cmd.logical_ts(),
        )?;
        Ok(ActivateSpotOrderV7AfterChanges { order_after, balance_after })
    }
}

impl StateDiff for ActivateSpotOrderV7UseCase {
    type StateDiff = ActivateSpotOrderV7Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(ActivateSpotOrderV7Changes {
            updated_order: UpdatedEntityPair::new(state.order, after.order_after),
            updated_balance: UpdatedEntityPair::new(state.balance, after.balance_after),
        })
    }
}

impl ReplayableChanges for ActivateSpotOrderV7Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.updated_order.to_replayable_event()?,
            self.updated_balance.to_replayable_event()?,
        ])
    }
}

#[cfg(test)]
mod tests {
    use common_entity::LogEntrySequencer;
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::entity::common::{OrderBase, UserId};
    use example_core_entity::OrderSide;
    use rust_decimal::Decimal;

    use super::*;

    const ORDER_ID: u64 = 7;
    const OWNER_ID: u64 = 11;

    fn command() -> LogEntry<ActivateSpotOrderV7Cmd> {
        LogEntrySequencer::from_metadata(
            1,
            500,
            1,
            1,
            ActivateSpotOrderV7Cmd { order_id: ORDER_ID },
        )
    }

    fn state(status: SpotOrderStatus, owner_id: u64, coin: Coin) -> ActivateSpotOrderV7State {
        let order = SpotOrderV7::from_record(OrderBase {
            owner: UserId(OWNER_ID),
            order_id: ORDER_ID,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            limit_px: Decimal::from(10),
            size: Decimal::from(3),
            orig_size: Decimal::from(3),
            status,
            ..OrderBase::default()
        })
        .unwrap();
        let balance = BalanceV2::new(owner_id, 99, u64::from(coin.index()), 100, 0, 1, 100);
        ActivateSpotOrderV7State { order, balance }
    }

    #[test]
    fn rejects_non_pending_owner_mismatch_and_coin_mismatch() {
        let cases = [
            (
                state(SpotOrderStatus::Open, OWNER_ID, Coin::USDT),
                ActivateSpotOrderV7Error::OrderNotPending,
            ),
            (
                state(SpotOrderStatus::Pending, OWNER_ID + 1, Coin::USDT),
                ActivateSpotOrderV7Error::OwnerMismatch,
            ),
            (
                state(SpotOrderStatus::Pending, OWNER_ID, Coin::BTC),
                ActivateSpotOrderV7Error::CoinMismatch,
            ),
        ];

        for (state, expected) in cases {
            assert_eq!(
                ActivateSpotOrderV7UseCase.validate_state_given(&command(), &state),
                Err(expected)
            );
        }
    }
}
