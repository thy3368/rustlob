use common_entity::{
    Entity, EntityError, EntityReplayableEvent, ExecutionContext, ReplayableChanges,
    StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use thiserror::Error;

use crate::entity::{Balance, SpotOrderV2};
use crate::{
    MatchSpotOrderV2AfterChanges, MatchSpotOrderV2Changes, MatchSpotOrderV2Cmd,
    MatchSpotOrderV2Error, MatchSpotOrderV2State, OpenMatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2AfterChanges, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2Error,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, PlaceOnlySpotOrderV2UseCase,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceMatchSpotOrderV2State {
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub fee_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceMatchSpotOrderV2AfterChanges {
    SinglePlacedOnly {
        created_order: SpotOrderV2,
    },
    SinglePlacedAndMatched {
        created_taker_order: SpotOrderV2,
        match_after: MatchSpotOrderV2AfterChanges,
    },
    NormalTpslPlacedAndMatched {
        created_parent_order: SpotOrderV2,
        created_child_orders: Vec<SpotOrderV2>,
        match_after: MatchSpotOrderV2AfterChanges,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceMatchSpotOrderV2Changes {
    SinglePlacedOnly {
        created_order: SpotOrderV2,
    },
    SinglePlacedAndMatched {
        created_taker_order: SpotOrderV2,
        match_changes: MatchSpotOrderV2Changes,
    },
    NormalTpslPlacedAndMatched {
        created_parent_order: SpotOrderV2,
        created_child_orders: Vec<SpotOrderV2>,
        match_changes: MatchSpotOrderV2Changes,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceMatchSpotOrderV2Error {
    #[error(transparent)]
    PlaceOnly(#[from] PlaceOnlySpotOrderV2Error),
    #[error(transparent)]
    Match(#[from] MatchSpotOrderV2Error),
    #[error("place-match only supports a single active limit order")]
    InvalidPlaceBranch,
    #[error("fee account id must not be empty")]
    InvalidFeeAccountId,
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceMatchSpotOrderV2UseCase;

impl ReplayableChanges for PlaceMatchSpotOrderV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        match self {
            Self::SinglePlacedOnly { created_order } => {
                Ok(vec![created_order.track_create_event()?])
            }
            Self::SinglePlacedAndMatched { created_taker_order, match_changes } => {
                let mut events = vec![created_taker_order.track_create_event()?];
                events.extend(match_changes.to_replayable_events()?);
                Ok(events)
            }
            Self::NormalTpslPlacedAndMatched {
                created_parent_order,
                created_child_orders,
                match_changes,
            } => {
                let mut events = vec![created_parent_order.track_create_event()?];
                for child in created_child_orders {
                    events.push(child.track_create_event()?);
                }
                events.extend(match_changes.to_replayable_events()?);
                Ok(events)
            }
        }
    }
}

impl StateMachineV2Unchecked for PlaceMatchSpotOrderV2UseCase {
    type Command = PlaceOnlySpotOrderV2Cmd;
    type StateGiven = PlaceMatchSpotOrderV2State;
    type Error = PlaceMatchSpotOrderV2Error;
    type StateChanged = PlaceMatchSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        PlaceOnlySpotOrderV2UseCase.check_command(cmd)?;
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        PlaceOnlySpotOrderV2UseCase.validate_state_given(cmd, &())?;
        if state.fee_account_id.is_empty() {
            return Err(PlaceMatchSpotOrderV2Error::InvalidFeeAccountId);
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let place_after =
            PlaceOnlySpotOrderV2UseCase.compute_state_changed_with_context(cmd, &(), context)?;
        match (cmd, place_after) {
            (
                PlaceOnlySpotOrderV2Cmd::Single(order_cmd),
                PlaceOnlySpotOrderV2AfterChanges::Single { created_order },
            ) => {
                if !matches!(order_cmd.order_type, PlaceOnlySpotOrderV2OrderType::Limit { .. }) {
                    return Ok(PlaceMatchSpotOrderV2AfterChanges::SinglePlacedOnly {
                        created_order,
                    });
                }
                let match_after = compute_match_after_for_created_taker(
                    order_cmd,
                    created_order.clone(),
                    state,
                    context,
                )?;
                Ok(PlaceMatchSpotOrderV2AfterChanges::SinglePlacedAndMatched {
                    created_taker_order: created_order,
                    match_after,
                })
            }
            (
                PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, .. },
                PlaceOnlySpotOrderV2AfterChanges::NormalTpsl {
                    created_parent_order,
                    created_child_orders,
                },
            ) => {
                let match_after = compute_match_after_for_created_taker(
                    parent,
                    created_parent_order.clone(),
                    state,
                    context,
                )?;
                Ok(PlaceMatchSpotOrderV2AfterChanges::NormalTpslPlacedAndMatched {
                    created_parent_order,
                    created_child_orders,
                    match_after,
                })
            }
            _ => Err(PlaceMatchSpotOrderV2Error::InvalidPlaceBranch),
        }
    }
}

impl StateMachineOwnedV2Diff for PlaceMatchSpotOrderV2UseCase {
    type StateDiff = PlaceMatchSpotOrderV2Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        match after {
            PlaceMatchSpotOrderV2AfterChanges::SinglePlacedOnly { created_order } => {
                Ok(PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order })
            }
            PlaceMatchSpotOrderV2AfterChanges::SinglePlacedAndMatched {
                created_taker_order,
                match_after,
            } => Ok(PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
                created_taker_order: created_taker_order.clone(),
                match_changes: compute_match_changes(state, created_taker_order, match_after)?,
            }),
            PlaceMatchSpotOrderV2AfterChanges::NormalTpslPlacedAndMatched {
                created_parent_order,
                created_child_orders,
                match_after,
            } => Ok(PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
                created_parent_order: created_parent_order.clone(),
                created_child_orders,
                match_changes: compute_match_changes(state, created_parent_order, match_after)?,
            }),
        }
    }
}

fn compute_match_after_for_created_taker(
    order_cmd: &PlaceOnlySpotOrderV2OrderCmd,
    created_taker_order: SpotOrderV2,
    state: &PlaceMatchSpotOrderV2State,
    context: &ExecutionContext,
) -> Result<MatchSpotOrderV2AfterChanges, PlaceMatchSpotOrderV2Error> {
    let match_cmd = MatchSpotOrderV2Cmd {
        party_id: order_cmd.party_id.clone(),
        asset: order_cmd.asset,
        order_id: order_cmd.order_id.clone(),
    };
    let match_state = MatchSpotOrderV2State {
        taker_order: created_taker_order,
        maker_orders: state.maker_orders.clone(),
        settlement_balances: state.settlement_balances.clone(),
        base_asset_id: order_cmd.base_asset_id.clone(),
        quote_asset_id: order_cmd.quote_asset_id.clone(),
        fee_account_id: state.fee_account_id.clone(),
        maker_fee_bps: order_cmd.maker_fee_bps,
        taker_fee_bps: order_cmd.taker_fee_bps,
    };
    Ok(OpenMatchSpotOrderV2UseCase.compute_state_changed_with_context(
        &match_cmd,
        &match_state,
        context,
    )?)
}

fn compute_match_changes(
    state: PlaceMatchSpotOrderV2State,
    created_taker_order: SpotOrderV2,
    match_after: MatchSpotOrderV2AfterChanges,
) -> Result<MatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Error> {
    let match_state = MatchSpotOrderV2State {
        taker_order: created_taker_order.clone(),
        maker_orders: state.maker_orders,
        settlement_balances: state.settlement_balances,
        base_asset_id: created_taker_order.reservation.asset_id.clone(),
        quote_asset_id: created_taker_order.fee_reservation.asset_id,
        fee_account_id: state.fee_account_id,
        maker_fee_bps: 0,
        taker_fee_bps: 0,
    };
    Ok(OpenMatchSpotOrderV2UseCase::do_compute_state_diff(match_state, match_after)?)
}

#[cfg(test)]
mod tests {
    use common_entity::{ExecutionContext, StateMachineOwnedV2Diff};

    use super::*;
    use crate::entity::{BalanceLedgerOperation, Reservation, SpotOrderSide, SpotOrderTif};
    use crate::{PlaceOnlySpotOrderV2OrderType, SpotOrderStatus, SpotOrderType};

    fn place_cmd(order_type: PlaceOnlySpotOrderV2OrderType) -> PlaceOnlySpotOrderV2Cmd {
        PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
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
        })
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

        let context = ExecutionContext { execution_time_ms: 1_717_171_717_000 };
        let changes =
            PlaceMatchSpotOrderV2UseCase.compute_state_diff_with_context(&cmd, state, &context)?;
        let PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
            created_taker_order,
            match_changes,
        } = &changes
        else {
            panic!("limit order should continue to matching");
        };

        assert_eq!(created_taker_order.order_id(), "taker-buy");
        assert_eq!(
            match_changes.created_balance_ledger_entries.first().map(|entry| entry.operation),
            Some(BalanceLedgerOperation::Freeze)
        );
        assert_eq!(match_changes.created_trades.len(), 1);
        assert_eq!(match_changes.created_trades[0].executed_at_ms, context.execution_time_ms);
        let events = changes.to_replayable_events()?;
        assert!(events[0].is_created());
        assert!(events.iter().skip(1).any(EntityReplayableEvent::is_created));
        Ok(())
    }

    #[test]
    fn normal_tpsl_parent_is_placed_then_matched_and_children_are_created()
    -> Result<(), Box<dyn std::error::Error>> {
        let PlaceOnlySpotOrderV2Cmd::Single(parent) =
            place_cmd(PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".to_string() })
        else {
            unreachable!();
        };
        let child = PlaceOnlySpotOrderV2OrderCmd {
            party_id: parent.party_id.clone(),
            asset: parent.asset,
            order_id: "tpsl-child".to_string(),
            symbol: parent.symbol.clone(),
            is_buy: false,
            price: "90".to_string(),
            size: "1".to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Trigger {
                is_market: false,
                trigger_price: "90".to_string(),
                trigger_role: "sl".to_string(),
            },
            reduce_only: true,
            cloid: None,
            base_asset_id: parent.base_asset_id.clone(),
            quote_asset_id: parent.quote_asset_id.clone(),
            maker_fee_bps: parent.maker_fee_bps,
            taker_fee_bps: parent.taker_fee_bps,
        };
        let cmd = PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children: vec![child] };
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
        let PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
            created_parent_order,
            created_child_orders,
            match_changes,
        } = &changes
        else {
            panic!("normal tpsl parent should continue to matching");
        };

        assert_eq!(created_parent_order.order_id(), "taker-buy");
        assert_eq!(
            created_child_orders.iter().map(SpotOrderV2::order_id).collect::<Vec<_>>(),
            vec!["tpsl-child"]
        );
        assert_eq!(match_changes.created_trades.len(), 1);
        let events = changes.to_replayable_events()?;
        assert!(events[0].is_created());
        assert!(events[1].is_created());
        assert!(events.iter().skip(2).any(EntityReplayableEvent::is_created));
        assert!(events.iter().skip(2).any(EntityReplayableEvent::is_updated));
        Ok(())
    }

    #[test]
    fn trigger_order_is_placed_without_matching() -> Result<(), Box<dyn std::error::Error>> {
        let cmd = place_cmd(PlaceOnlySpotOrderV2OrderType::Trigger {
            is_market: false,
            trigger_price: "90".to_string(),
            trigger_role: "sl".to_string(),
        });
        let state = PlaceMatchSpotOrderV2State {
            maker_orders: vec![sell_order("maker-1", "seller", 100, 1)?],
            settlement_balances: vec![],
            fee_account_id: "fee".to_string(),
        };

        let changes = PlaceMatchSpotOrderV2UseCase.compute_state_diff(&cmd, state)?;
        let PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order } = &changes else {
            panic!("trigger order should not continue to matching");
        };

        assert_eq!(created_order.order_id(), "taker-buy");
        let events = changes.to_replayable_events()?;
        assert_eq!(events.len(), 1);
        assert!(events[0].is_created());
        Ok(())
    }
}
