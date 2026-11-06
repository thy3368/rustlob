use common_entity::{LogEntry, UseCase};
use example_core_entity::spot::spot_order_v7::SpotOrderV7;
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, SpotAmountConversionError, SpotOrderStatus,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UnFreezeSpotOrderV2Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2AfterChanges {
    pub balance_after: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FreezeSpotOrderV2Error {
    #[error("order id must be greater than zero")]
    InvalidOrderId,
    #[error("command order id does not match the loaded order")]
    OrderIdMismatch,
    #[error("only pending orders can be frozen")]
    OrderNotPending,
    #[error("balance party does not match order owner")]
    PartyMismatch,
    #[error("balance asset does not match the order freeze asset")]
    AssetMismatch,
    #[error(transparent)]
    FreezeRequirement(#[from] SpotAmountConversionError),
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Freeze4SpotOrderV2UseCase;

impl UseCase for Freeze4SpotOrderV2UseCase {
    type Command = LogEntry<UnFreezeSpotOrderV2Cmd>;
    type StateGiven = UnFreezeSpotOrderV2State;
    type Error = FreezeSpotOrderV2Error;
    type StateChanged = UnFreezeSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.order_id == 0 {
            return Err(FreezeSpotOrderV2Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let order = given_state.order.order_base();
        if cmd.order_id != order.order_id {
            return Err(FreezeSpotOrderV2Error::OrderIdMismatch);
        }
        if given_state.order.status() != SpotOrderStatus::Pending {
            return Err(FreezeSpotOrderV2Error::OrderNotPending);
        }
        if given_state.balance.party_id() != order.owner.0 {
            return Err(FreezeSpotOrderV2Error::PartyMismatch);
        }
        let requirement = given_state.order.freeze_requirement()?;
        if given_state.balance.coin_idx() != u64::from(requirement.currency().index()) {
            return Err(FreezeSpotOrderV2Error::AssetMismatch);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let requirement = given_state.order.freeze_requirement()?;
        let mut balance_after = given_state.balance.clone();
        common_entity::Fsm::apply(
            &mut balance_after,
            BalanceV2Event::Freeze { amount: requirement.amount(), timestamp: cmd.logical_ts() },
        )?;
        Ok(UnFreezeSpotOrderV2AfterChanges { balance_after })
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, UseCase};
    use example_core_entity::OrderSide;
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::entity::common::{OrderBase, UserId};
    use rstest::rstest;
    use rust_decimal::Decimal;

    use super::*;

    const ORDER_ID: u64 = 7;
    const OWNER_ID: u64 = 11;
    const LOGICAL_TS: u64 = 500;

    fn command(order_id: u64) -> LogEntry<UnFreezeSpotOrderV2Cmd> {
        LogEntrySequencer::from_metadata(1, LOGICAL_TS, 1, 1, UnFreezeSpotOrderV2Cmd { order_id })
    }

    fn order(side: OrderSide, status: SpotOrderStatus) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(OWNER_ID),
            order_id: ORDER_ID,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(10),
            size: Decimal::from(3),
            orig_size: Decimal::from(3),
            status,
            buy_quote_fee_reserve_limit: Decimal::from(2),
            ..OrderBase::default()
        })
        .unwrap()
    }

    fn balance(party_id: u64, coin: Coin, available: i64) -> BalanceV2 {
        BalanceV2::new(
            party_id,
            party_id,
            u64::from(coin.index()),
            Decimal::from(available),
            Decimal::from(5),
            4,
            100,
        )
    }

    fn state(side: OrderSide, status: SpotOrderStatus) -> UnFreezeSpotOrderV2State {
        let coin = if side == OrderSide::Buy { Coin::USDT } else { Coin::BTC };
        UnFreezeSpotOrderV2State {
            order: order(side, status),
            balance: balance(OWNER_ID, coin, 100),
        }
    }

    #[test]
    fn rejects_zero_order_id() {
        assert_eq!(
            Freeze4SpotOrderV2UseCase.check_command(&command(0)),
            Err(FreezeSpotOrderV2Error::InvalidOrderId)
        );
    }

    #[test]
    fn rejects_loaded_order_with_different_id() {
        let given = state(OrderSide::Buy, SpotOrderStatus::Pending);

        assert_eq!(
            Freeze4SpotOrderV2UseCase.validate_state_given(&command(ORDER_ID + 1), &given),
            Err(FreezeSpotOrderV2Error::OrderIdMismatch)
        );
    }

    #[rstest]
    #[case(SpotOrderStatus::Open)]
    #[case(SpotOrderStatus::PartiallyFilled)]
    #[case(SpotOrderStatus::Filled)]
    #[case(SpotOrderStatus::Canceled)]
    #[case(SpotOrderStatus::Rejected)]
    fn rejects_every_non_pending_status(#[case] status: SpotOrderStatus) {
        let given = state(OrderSide::Buy, status);

        assert_eq!(
            Freeze4SpotOrderV2UseCase.validate_state_given(&command(ORDER_ID), &given),
            Err(FreezeSpotOrderV2Error::OrderNotPending)
        );
    }

    #[test]
    fn rejects_balance_for_different_party() {
        let mut given = state(OrderSide::Buy, SpotOrderStatus::Pending);
        given.balance = balance(OWNER_ID + 1, Coin::USDT, 100);

        assert_eq!(
            Freeze4SpotOrderV2UseCase.validate_state_given(&command(ORDER_ID), &given),
            Err(FreezeSpotOrderV2Error::PartyMismatch)
        );
    }

    #[test]
    fn rejects_balance_for_different_asset() {
        let mut given = state(OrderSide::Buy, SpotOrderStatus::Pending);
        given.balance = balance(OWNER_ID, Coin::BTC, 100);

        assert_eq!(
            Freeze4SpotOrderV2UseCase.validate_state_given(&command(ORDER_ID), &given),
            Err(FreezeSpotOrderV2Error::AssetMismatch)
        );
    }

    #[test]
    fn buy_freezes_quote_notional_and_fee_reserve() {
        let given = state(OrderSide::Buy, SpotOrderStatus::Pending);

        let changed =
            Freeze4SpotOrderV2UseCase.compute_state_changed(&command(ORDER_ID), &given).unwrap();

        assert_eq!(changed.balance_after.available(), Decimal::from(68));
        assert_eq!(changed.balance_after.frozen(), Decimal::from(37));
        assert_eq!(changed.balance_after.version(), 5);
        assert_eq!(changed.balance_after.updated_at(), LOGICAL_TS);
    }

    #[test]
    fn sell_freezes_original_base_quantity() {
        let given = state(OrderSide::Sell, SpotOrderStatus::Pending);

        let changed =
            Freeze4SpotOrderV2UseCase.compute_state_changed(&command(ORDER_ID), &given).unwrap();

        assert_eq!(changed.balance_after.available(), Decimal::from(97));
        assert_eq!(changed.balance_after.frozen(), Decimal::from(8));
        assert_eq!(changed.balance_after.version(), 5);
        assert_eq!(changed.balance_after.updated_at(), LOGICAL_TS);
    }

    #[test]
    fn propagates_insufficient_available_balance() {
        let mut given = state(OrderSide::Buy, SpotOrderStatus::Pending);
        given.balance = balance(OWNER_ID, Coin::USDT, 31);

        assert_eq!(
            Freeze4SpotOrderV2UseCase.compute_state_changed(&command(ORDER_ID), &given),
            Err(FreezeSpotOrderV2Error::Balance(BalanceV2Error::InsufficientAvailableBalance))
        );
    }
}
