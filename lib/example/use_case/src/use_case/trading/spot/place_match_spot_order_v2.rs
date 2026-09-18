use common_entity::{
    Entity, EntityError, EntityReplayableEvent, IssuedByParty, MiStateMachineV2,
    MiStateMachineV2Unchecked, ReplayableChanges, StateMachineOwnedV2Diff,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{Balance, SpotOrderV2};
use crate::{
    MatchSpotOrderV2AfterChanges, MatchSpotOrderV2Changes, MatchSpotOrderV2Cmd,
    MatchSpotOrderV2Error, MatchSpotOrderV2State, MatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2AfterChanges, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2Error,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, PlaceOnlySpotOrderV2UseCase,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceMatchSpotOrderV2Cmd {
    pub place_order: PlaceOnlySpotOrderV2OrderCmd,
}

impl IssuedByParty for PlaceMatchSpotOrderV2Cmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.place_order.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceMatchSpotOrderV2State {
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub fee_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceMatchSpotOrderV2AfterChanges {
    pub created_taker_order: SpotOrderV2,
    pub match_after: MatchSpotOrderV2AfterChanges,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceMatchSpotOrderV2Changes {
    pub created_taker_order: SpotOrderV2,
    pub match_changes: MatchSpotOrderV2Changes,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceMatchSpotOrderV2Error {
    #[error(transparent)]
    PlaceOnly(#[from] PlaceOnlySpotOrderV2Error),
    #[error(transparent)]
    Match(#[from] MatchSpotOrderV2Error),
    #[error("place-match only supports a single active limit order")]
    InvalidPlaceBranch,
    #[error("trigger pending spot order cannot be matched immediately")]
    UnsupportedTriggerOrder,
    #[error("fee account id must not be empty")]
    InvalidFeeAccountId,
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceMatchSpotOrderV2UseCase;

impl ReplayableChanges for PlaceMatchSpotOrderV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::new();
        events.push(self.created_taker_order.track_create_event()?);
        events.extend(self.match_changes.to_replayable_events()?);
        Ok(events)
    }
}

impl MiStateMachineV2Unchecked for PlaceMatchSpotOrderV2UseCase {
    type Command = PlaceMatchSpotOrderV2Cmd;
    type StateGiven = PlaceMatchSpotOrderV2State;
    type Error = PlaceMatchSpotOrderV2Error;
    type StateChanged = PlaceMatchSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        PlaceOnlySpotOrderV2UseCase
            .check_command(&PlaceOnlySpotOrderV2Cmd::Single(cmd.place_order.clone()))?;
        if matches!(cmd.place_order.order_type, PlaceOnlySpotOrderV2OrderType::Trigger { .. }) {
            return Err(PlaceMatchSpotOrderV2Error::UnsupportedTriggerOrder);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        _cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.fee_account_id.is_empty() {
            return Err(PlaceMatchSpotOrderV2Error::InvalidFeeAccountId);
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let place_cmd = PlaceOnlySpotOrderV2Cmd::Single(cmd.place_order.clone());
        let place_after = PlaceOnlySpotOrderV2UseCase.compute_state_changed(&place_cmd, &())?;
        let PlaceOnlySpotOrderV2AfterChanges::Single { created_order } = place_after else {
            return Err(PlaceMatchSpotOrderV2Error::InvalidPlaceBranch);
        };

        let match_cmd = MatchSpotOrderV2Cmd {
            party_id: cmd.place_order.party_id.clone(),
            asset: cmd.place_order.asset,
            order_id: cmd.place_order.order_id.clone(),
        };
        let match_state = MatchSpotOrderV2State {
            taker_order: created_order.clone(),
            maker_orders: state.maker_orders.clone(),
            settlement_balances: state.settlement_balances.clone(),
            base_asset_id: cmd.place_order.base_asset_id.clone(),
            quote_asset_id: cmd.place_order.quote_asset_id.clone(),
            fee_account_id: state.fee_account_id.clone(),
            maker_fee_bps: cmd.place_order.maker_fee_bps,
            taker_fee_bps: cmd.place_order.taker_fee_bps,
        };
        let match_after =
            MatchSpotOrderV2UseCase.compute_state_changed(&match_cmd, &match_state)?;

        Ok(PlaceMatchSpotOrderV2AfterChanges { created_taker_order: created_order, match_after })
    }
}

impl StateMachineOwnedV2Diff for PlaceMatchSpotOrderV2UseCase {
    type StateDiff = PlaceMatchSpotOrderV2Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        let match_state = MatchSpotOrderV2State {
            taker_order: after.created_taker_order.clone(),
            maker_orders: state.maker_orders,
            settlement_balances: state.settlement_balances,
            base_asset_id: after.created_taker_order.reservation.asset_id.clone(),
            quote_asset_id: after.created_taker_order.fee_reservation.asset_id.clone(),
            fee_account_id: state.fee_account_id,
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        };
        let match_changes =
            MatchSpotOrderV2UseCase::do_compute_state_diff(match_state, after.match_after)?;
        Ok(PlaceMatchSpotOrderV2Changes {
            created_taker_order: after.created_taker_order,
            match_changes,
        })
    }
}

#[cfg(test)]
mod tests {
    use common_entity::StateMachineOwnedV2Diff;

    use super::*;
    use crate::entity::{BalanceLedgerOperation, Reservation, SpotOrderSide, SpotOrderTif};
    use crate::{PlaceOnlySpotOrderV2OrderType, SpotOrderStatus, SpotOrderType};

    fn place_cmd(order_type: PlaceOnlySpotOrderV2OrderType) -> PlaceMatchSpotOrderV2Cmd {
        PlaceMatchSpotOrderV2Cmd {
            place_order: PlaceOnlySpotOrderV2OrderCmd {
                party_id: "buyer".to_string(),
                asset: 10_001,
                order_id: "taker-buy".to_string(),
                symbol: "BTCUSDT".to_string(),
                is_buy: true,
                price: "100".to_string(),
                size: "1".to_string(),
                order_type,
                reduce_only: false,
                cloid: None,
                base_asset_id: "BTC".to_string(),
                quote_asset_id: "USDT".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            },
        }
    }

    fn test_principal_reservation(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        qty: u64,
        order_price: u64,
    ) -> Result<Reservation, Box<dyn std::error::Error>> {
        Ok(SpotOrderV2::principal_reservation(
            order_id,
            account_id,
            side,
            qty,
            order_price,
            "BTC",
            "USDT",
        )?)
    }

    fn sell_order(
        order_id: &str,
        account_id: &str,
        price: u64,
        qty: u64,
    ) -> Result<SpotOrderV2, Box<dyn std::error::Error>> {
        Ok(SpotOrderV2::new(
            order_id.to_string(),
            10_001,
            Some(price),
            account_id.to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            price,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            qty,
            0,
            SpotOrderStatus::Open,
            None,
            test_principal_reservation(order_id, account_id, SpotOrderSide::Sell, qty, price)?,
            None,
            1,
        ))
    }

    fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
        Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 1)
    }

    #[test]
    fn single_ioc_buy_creates_taker_then_freezes_and_matches()
    -> Result<(), Box<dyn std::error::Error>> {
        let cmd = place_cmd(PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".to_string() });
        let state = PlaceMatchSpotOrderV2State {
            maker_orders: vec![sell_order("maker-1", "seller", 100, 1)?],
            settlement_balances: vec![
                balance("buyer", "USDT", 101, 0),
                balance("buyer", "BTC", 0, 0),
                balance("seller", "BTC", 0, 1),
                balance("seller", "USDT", 0, 1),
                balance("fee", "USDT", 0, 0),
            ],
            fee_account_id: "fee".to_string(),
        };

        let changes = PlaceMatchSpotOrderV2UseCase.compute_state_diff(&cmd, state)?;

        assert_eq!(changes.created_taker_order.order_id(), cmd.place_order.order_id);
        assert_eq!(
            changes
                .match_changes
                .created_balance_ledger_entries
                .first()
                .map(|entry| entry.operation),
            Some(BalanceLedgerOperation::Freeze)
        );
        assert_eq!(changes.match_changes.created_trades.len(), 1);
        let events = changes.to_replayable_events()?;
        assert!(events[0].is_created());
        assert!(events.iter().skip(1).any(EntityReplayableEvent::is_created));
        Ok(())
    }

    #[test]
    fn rejects_trigger_order_before_matching() {
        let cmd = place_cmd(PlaceOnlySpotOrderV2OrderType::Trigger {
            is_market: false,
            trigger_price: "90".to_string(),
            trigger_role: "sl".to_string(),
        });

        assert_eq!(
            PlaceMatchSpotOrderV2UseCase.check_command(&cmd),
            Err(PlaceMatchSpotOrderV2Error::UnsupportedTriggerOrder)
        );
    }
}
