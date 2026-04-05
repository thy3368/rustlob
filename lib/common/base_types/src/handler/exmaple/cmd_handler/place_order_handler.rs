//! PlaceOrder CommandHandler 示例实现。

use crate::handler::exmaple::cmd_handler::example_types::{
    AccountBalance, BalanceChange, HandlerError, Order, OrderBookSnapshot, OrderStatus,
};
use crate::handler::handler_update::{
    ApplyCommandChanges, ChangeSet, CmdHandlerForUpdate,
};

pub struct PlaceOrderCmd {
    pub cmd_id: String,
    pub user_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: Option<i64>,
    pub quantity: i64,
    pub timestamp_ms: i64,
}

pub enum OrderSide {
    Buy,
    Sell,
}

pub enum OrderType {
    Limit,
    Market,
}

pub struct PlaceOrderState {
    pub account: AccountBalance,
    pub orderbook: OrderBookSnapshot,
    pub open_orders: Vec<Order>,
}

pub struct PlaceOrderResult {
    pub order_id: String,
    pub status: OrderStatus,
    pub balance_change: Option<BalanceChange>,
}

pub struct PlaceOrderOutput {
    pub result: PlaceOrderResult,
    pub events: Vec<PlaceOrderEvent>,
}

pub enum PlaceOrderEvent {
    Accepted(PlaceOrderAcceptedEvent),
}

pub struct PlaceOrderAcceptedEvent {
    pub order_id: String,
}

pub enum PlaceOrderLog {
    OrderCreated(OrderCreated),
    BalanceFrozen(BalanceFrozen),
}

pub struct OrderCreated {
    pub order_id: String,
}

pub struct BalanceFrozen {
    pub user_id: String,
    pub amount: i64,
}

pub struct PlaceOrderHandler;

impl PlaceOrderHandler {
    pub fn new() -> Self {
        Self
    }
}

impl
    ApplyCommandChanges<
        PlaceOrderCmd,
        PlaceOrderState,
        PlaceOrderOutput,
        PlaceOrderLog,
        HandlerError,
    > for PlaceOrderHandler
{
    fn apply_command_and_collect_changes(
        &self,
        cmd: &PlaceOrderCmd,
        _state_set: PlaceOrderState,
    ) -> Result<ChangeSet<PlaceOrderOutput, PlaceOrderLog>, HandlerError> {
        let order_id = format!("order_{}", cmd.cmd_id);
        let result = PlaceOrderResult {
            order_id: order_id.clone(),
            status: OrderStatus::Open,
            balance_change: Some(BalanceChange {
                user_id: cmd.user_id.clone(),
                asset: "USDT".into(),
                change: -(cmd.price.unwrap_or(0) * cmd.quantity),
            }),
        };
        let output = PlaceOrderOutput {
            result,
            events: vec![PlaceOrderEvent::Accepted(PlaceOrderAcceptedEvent {
                order_id: order_id.clone(),
            })],
        };

        Ok(ChangeSet {
            writes: output,
            changelogs: vec![PlaceOrderLog::OrderCreated(OrderCreated { order_id })],
        })
    }
}

impl
    CmdHandlerForUpdate<
        PlaceOrderCmd,
        PlaceOrderState,
        PlaceOrderOutput,
        PlaceOrderLog,
        HandlerError,
    > for PlaceOrderHandler
{
    fn pre_check_command(&self, _cmd: &PlaceOrderCmd) -> Result<(), HandlerError> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        _cmd: &PlaceOrderCmd,
    ) -> Result<PlaceOrderState, HandlerError> {
        Ok(PlaceOrderState {
            account: AccountBalance::default(),
            orderbook: OrderBookSnapshot::default(),
            open_orders: vec![],
        })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &PlaceOrderCmd,
        _state_set: &PlaceOrderState,
    ) -> Result<(), HandlerError> {
        Ok(())
    }

    fn persist_changelogs(&self, _changelogs: &[PlaceOrderLog]) -> Result<(), HandlerError> {
        Ok(())
    }

    fn replay_changelogs_to_state(
        &self,
        _changelogs: &[PlaceOrderLog],
    ) -> Result<(), HandlerError> {
        Ok(())
    }

    fn publish_changelog(&self, _changelogs: &[PlaceOrderLog]) -> Result<(), HandlerError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_handler() {
        let handler = PlaceOrderHandler::new();
        let cmd = PlaceOrderCmd {
            cmd_id: "1".into(),
            user_id: "u1".into(),
            symbol: "BTCUSDT".into(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some(50000),
            quantity: 10,
            timestamp_ms: 1234567890,
        };

        let result = handler.cmd_handle(cmd, |writes, _| PlaceOrderOutput {
            result: PlaceOrderResult {
                order_id: writes.result.order_id.clone(),
                status: OrderStatus::Open,
                balance_change: None,
            },
            events: writes
                .events
                .iter()
                .map(|event| match event {
                    PlaceOrderEvent::Accepted(event) => {
                        PlaceOrderEvent::Accepted(PlaceOrderAcceptedEvent {
                            order_id: event.order_id.clone(),
                        })
                    }
                })
                .collect(),
        });
        assert!(result.is_ok());
    }
}
