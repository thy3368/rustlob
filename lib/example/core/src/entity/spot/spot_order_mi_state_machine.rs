use common_entity::{Entity, EntityReplayableEvent, MiStateMachine, ReplayableChanges};
use thiserror::Error;

use super::spot_order::{SpotOrder, SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce};
use super::spot_trade::SpotTrade;

/// `SpotOrder` 的显式状态机命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderMiCommand {
    /// 下单并对 makers 进行即时撮合。
    Place(PlaceSpotOrderCmd),
    /// 对当前开放订单执行业务撤销。
    Cancel(CancelSpotOrderCmd),
}

/// `Place` 命令携带的 makers。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderCmd {
    /// 按撮合优先级排序的 makers。
    pub makers: Option<Vec<SpotOrder>>,
}

/// `Place` 命令的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderChanges {
    /// 订单 before/after。
    pub updated_order: SpotOrderUpdated,
    /// 本次生成的成交事实。
    pub created_trades: Option<Vec<SpotTrade>>,

    //todo
}

/// `Cancel` 命令不携带额外参数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderCmd;

/// `Cancel` 命令的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderChanges {
    /// 订单 before/after。
    pub updated_order: SpotOrderUpdated,
}

/// `SpotOrder` 状态机的一次业务变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderMiChanges {
    /// `Place` 的 changes。
    Place(PlaceSpotOrderChanges),
    /// `Cancel` 的 changes。
    Cancel(CancelSpotOrderChanges),
}

/// `SpotOrder` before/after 对。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderUpdated {
    /// 状态变化前的订单。
    pub before: SpotOrder,
    /// 状态变化后的订单。
    pub after: SpotOrder,
}

impl ReplayableChanges for PlaceSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let mut events = Vec::with_capacity(1 + self.created_trades.as_ref().map_or(0, Vec::len));
        events.push(self.updated_order.after.track_update_event_from(&self.updated_order.before)?);
        if let Some(created_trades) = &self.created_trades {
            for trade in created_trades {
                events.push(trade.track_create_event()?);
            }
        }
        Ok(events)
    }
}

impl ReplayableChanges for CancelSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        Ok(vec![self.updated_order.after.track_update_event_from(&self.updated_order.before)?])
    }
}

impl ReplayableChanges for SpotOrderMiChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Place(changes) => changes.to_replayable_events(),
            Self::Cancel(changes) => changes.to_replayable_events(),
        }
    }
}

/// `SpotOrder` 显式状态机的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotOrderMiStateMachineError {
    #[error("spot order execution state is inconsistent")]
    InconsistentExecutionState,
    #[error("command `{command}` is not allowed when order status is `{status}`")]
    CommandNotAllowedInCurrentState { command: &'static str, status: &'static str },
    #[error("invalid spot order state machine command: {reason}")]
    InvalidCommandFields { reason: &'static str },
    #[error("filled quantity overflow or exceeds order qty")]
    FilledQtyOverflowOrExceedsQty,
    #[error("spot order time-in-force semantics mismatch: {reason}")]
    TimeInForceSemanticsMismatch { reason: &'static str },
    #[error("spot order version overflow")]
    VersionOverflow,
}

impl MiStateMachine for SpotOrder {
    type Command = SpotOrderMiCommand;
    type State = SpotOrderStatus;
    type Error = SpotOrderMiStateMachineError;
    type Changes = SpotOrderMiChanges;

    fn state(&self) -> &Self::State {
        &self.status
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if !self.has_consistent_execution_state() {
            return Err(SpotOrderMiStateMachineError::InconsistentExecutionState);
        }

        match cmd {
            SpotOrderMiCommand::Place(place) => self.pre_check_place_command(place),
            SpotOrderMiCommand::Cancel(_) => Ok(()),
        }
    }

    fn validate_state_transition(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd {
            SpotOrderMiCommand::Place(_) => {
                if matches!(self.state(), SpotOrderStatus::Open) {
                    Ok(())
                } else {
                    Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                        command: "place",
                        status: self.state().as_str(),
                    })
                }
            }
            SpotOrderMiCommand::Cancel(_) => {
                if self.can_be_cancelled() {
                    Ok(())
                } else {
                    Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                        command: "cancel",
                        status: self.state().as_str(),
                    })
                }
            }
        }
    }

    fn compute_changes(&self, cmd: &Self::Command) -> Result<Self::Changes, Self::Error> {
        self.pre_check_command(cmd)?;
        self.validate_state_transition(cmd)?;

        match cmd {
            SpotOrderMiCommand::Place(place) => self.compute_place_changes(place),
            SpotOrderMiCommand::Cancel(cancel) => self.compute_cancel_changes(cancel),
        }
    }
}

impl SpotOrder {
    fn compute_place_changes(
        &self,
        cmd: &PlaceSpotOrderCmd,
    ) -> Result<SpotOrderMiChanges, SpotOrderMiStateMachineError> {
        let before = self.clone();
        let mut after = self.clone();
        let mut created_trades = Vec::new();

        let Some(makers) = &cmd.makers else {
            after = self.finalize_place_without_makers()?;
            return Ok(SpotOrderMiChanges::Place(PlaceSpotOrderChanges {
                updated_order: SpotOrderUpdated { before, after },
                created_trades: None,
            }));
        };

        if makers.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "makers must not be empty when present",
            });
        }

        let mut remaining_qty = after
            .remaining_qty()
            .map_err(|_| SpotOrderMiStateMachineError::FilledQtyOverflowOrExceedsQty)?;

        for (index, maker) in makers.iter().enumerate() {
            if remaining_qty == 0 {
                break;
            }

            maker.ensure_matchable().map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "maker is not matchable",
                }
            })?;
            maker.ensure_compatible_maker_for(self).map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "maker is not compatible with taker",
                }
            })?;

            if !self.crosses_order(maker).map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "failed to evaluate crossing",
                }
            })? {
                break;
            }

            let maker_price = maker.limit_price().ok_or(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "maker must be a limit order",
            })?;
            let maker_remaining = maker.remaining_qty().map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "maker execution state is inconsistent",
                }
            })?;
            let trade_qty = remaining_qty.min(maker_remaining);
            if trade_qty == 0 {
                continue;
            }

            created_trades.push(SpotTrade::new(
                format!("{}-{}", self.order_id, index + 1),
                self.order_id.clone(),
                self.asset,
                self.symbol.clone(),
                self.order_id.clone(),
                maker.order_id.clone(),
                self.account_id.clone(),
                maker.account_id.clone(),
                self.side,
                maker_price,
                trade_qty,
            ));

            after.filled_qty = after
                .filled_qty
                .checked_add(trade_qty)
                .ok_or(SpotOrderMiStateMachineError::FilledQtyOverflowOrExceedsQty)?;
            remaining_qty = remaining_qty
                .checked_sub(trade_qty)
                .ok_or(SpotOrderMiStateMachineError::FilledQtyOverflowOrExceedsQty)?;
        }

        after.status = if after.filled_qty == after.qty {
            SpotOrderStatus::Filled
        } else if after.filled_qty > 0 && after.time_in_force == SpotOrderTimeInForce::Ioc {
            after.status_reason = Some(SpotOrderStatusReason::IocCancelRejected);
            SpotOrderStatus::Canceled
        } else if after.filled_qty > 0 {
            SpotOrderStatus::PartiallyFilled
        } else {
            SpotOrderStatus::Open
        };

        if after.filled_qty == 0 && after.time_in_force == SpotOrderTimeInForce::Ioc {
            after.status = SpotOrderStatus::Rejected;
            after.status_reason = Some(after.no_liquidity_status_reason());
        }

        after.version = after
            .version
            .checked_add(1)
            .ok_or(SpotOrderMiStateMachineError::VersionOverflow)?;

        Ok(SpotOrderMiChanges::Place(PlaceSpotOrderChanges {
            updated_order: SpotOrderUpdated { before, after },
            created_trades: if created_trades.is_empty() { None } else { Some(created_trades) },
        }))
    }

    fn finalize_place_without_makers(&self) -> Result<SpotOrder, SpotOrderMiStateMachineError> {
        let mut after = self.clone();
        after.version =
            after.version.checked_add(1).ok_or(SpotOrderMiStateMachineError::VersionOverflow)?;
        after.status = match after.time_in_force {
            SpotOrderTimeInForce::Ioc => {
                after.status_reason = Some(after.no_liquidity_status_reason());
                SpotOrderStatus::Rejected
            }
            _ => SpotOrderStatus::Open,
        };
        Ok(after)
    }

    fn pre_check_place_command(
        &self,
        cmd: &PlaceSpotOrderCmd,
    ) -> Result<(), SpotOrderMiStateMachineError> {
        if let Some(makers) = &cmd.makers {
            if makers.is_empty() {
                return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "makers must not be empty when present",
                });
            }
            for maker in makers {
                if !maker.has_consistent_execution_state() {
                    return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                        reason: "maker execution state is inconsistent",
                    });
                }
            }
        }

        Ok(())
    }

    fn compute_cancel_changes(
        &self,
        _cmd: &CancelSpotOrderCmd,
    ) -> Result<SpotOrderMiChanges, SpotOrderMiStateMachineError> {
        let before = self.clone();
        let mut after = self.clone();
        after.status = SpotOrderStatus::Canceled;
        after.status_reason = Some(SpotOrderStatusReason::CanceledByUser);
        after.version =
            after.version.checked_add(1).ok_or(SpotOrderMiStateMachineError::VersionOverflow)?;
        Ok(SpotOrderMiChanges::Cancel(CancelSpotOrderChanges {
            updated_order: SpotOrderUpdated { before, after },
        }))
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{MiStateMachine, ReplayableChanges};

    use super::*;
    use crate::{SpotOrderExecution, SpotOrderSide};

    fn sample_order(time_in_force: SpotOrderTimeInForce) -> SpotOrder {
        SpotOrder::new(
            "order-1".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            time_in_force,
            10,
            0,
            1_000,
            Some("cloid-1".to_string()),
        )
    }

    fn maker(price: u64, qty: u64) -> SpotOrder {
        SpotOrder::new(
            "maker-1".to_string(),
            10_001,
            Some(99),
            "maker-account".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            0,
            0,
            None,
        )
    }

    fn place(makers: Option<Vec<SpotOrder>>) -> SpotOrderMiCommand {
        SpotOrderMiCommand::Place(PlaceSpotOrderCmd { makers })
    }

    fn cancel() -> SpotOrderMiCommand {
        SpotOrderMiCommand::Cancel(CancelSpotOrderCmd)
    }

    #[test]
    fn open_place_zero_makers_gtc_stays_open() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);

        let result =
            <SpotOrder as MiStateMachine>::compute_changes(&order, &place(None)).unwrap();
        let events = result.to_replayable_events().unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0]
                .field_changes
                .iter()
                .find(|c| c.field_name_as_str().ok() == Some("version"))
                .unwrap()
                .new_value_bytes(),
            b"2"
        );
    }

    #[test]
    fn open_place_with_makers_emits_trade_and_update() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let result =
            <SpotOrder as MiStateMachine>::compute_changes(
                &order,
                &place(Some(vec![maker(100, 4)])),
            )
                .unwrap();
        let events = result.to_replayable_events().unwrap();

        assert_eq!(events.len(), 2);
        assert!(events[0].is_updated());
        assert!(events[1].is_created());
    }

    #[test]
    fn open_cancel_updates_order_only() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &cancel(),
        )
        .unwrap();
        let events = result.to_replayable_events().unwrap();

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert_eq!(
            events[0]
                .field_changes
                .iter()
                .find(|c| c.field_name_as_str().ok() == Some("status_reason"))
                .unwrap()
                .new_value_bytes(),
            b"canceled"
        );
    }
}
