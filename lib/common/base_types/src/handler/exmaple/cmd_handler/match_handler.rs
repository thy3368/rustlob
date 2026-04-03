//! Match CommandHandler 示例实现。

use crate::handler::exmaple::cmd_handler::example_types::{
    AccountBalance, HandlerError, Order, OrderBookSnapshot, OrderStatus, Trade,
};
use crate::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};

pub struct MatchCmd {
    pub match_id: String,
    pub taker_order_id: String,
}

pub struct MatchState {
    pub taker_order: Order,
    pub maker_orders: Vec<Order>,
    pub taker_balance: AccountBalance,
    pub maker_balances: Vec<AccountBalance>,
    pub orderbook: OrderBookSnapshot,
}

pub struct MatchResult {
    pub trades: Vec<Trade>,
    pub order_updates: Vec<Order>,
}

pub struct MatchOutput {
    pub result: MatchResult,
    pub events: Vec<MatchEvent>,
}

pub enum MatchEvent {
    TradeCreated(TradeCreatedEvent),
}

pub struct TradeCreatedEvent {
    pub trade_id: String,
}

pub enum MatchLog {
    OrderUpdated(OrderUpdated),
    TradeCreated(TradeCreated),
    BalanceChanged(BalanceChanged),
}

pub struct OrderUpdated {
    pub order_id: String,
    pub status: OrderStatus,
}

pub struct TradeCreated {
    pub trade_id: String,
}

pub struct BalanceChanged {
    pub user_id: String,
    pub change: i64,
}

pub struct MatchHandler;

impl MatchHandler {
    pub fn new() -> Self {
        Self
    }
}

impl CmdHandlerForUpdate<MatchCmd, MatchState, MatchOutput, MatchLog, HandlerError>
    for MatchHandler
{
    fn pre_check_command(&self, _cmd: &MatchCmd) -> Result<(), HandlerError> {
        Ok(())
    }

    fn load_state_set_for_update(&self, _cmd: &MatchCmd) -> Result<MatchState, HandlerError> {
        Ok(MatchState {
            taker_order: Order::default(),
            maker_orders: vec![],
            taker_balance: AccountBalance::default(),
            maker_balances: vec![],
            orderbook: OrderBookSnapshot::default(),
        })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &MatchCmd,
        _state_set: &MatchState,
    ) -> Result<(), HandlerError> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        cmd: &MatchCmd,
        _state_set: MatchState,
    ) -> Result<ChangeSet<MatchOutput, MatchLog>, HandlerError> {
        let trade_id = format!("trade_{}", cmd.match_id);
        let result = MatchResult {
            trades: vec![Trade {
                trade_id: trade_id.clone(),
                taker_order_id: cmd.taker_order_id.clone(),
                maker_order_id: "maker_001".into(),
                price: 50000,
                quantity: 10,
            }],
            order_updates: vec![],
        };
        let output = MatchOutput {
            result,
            events: vec![MatchEvent::TradeCreated(TradeCreatedEvent {
                trade_id: trade_id.clone(),
            })],
        };

        Ok(ChangeSet {
            writes: output,
            changelogs: vec![MatchLog::TradeCreated(TradeCreated { trade_id })],
        })
    }

    fn persist_changelogs(&self, _changelogs: &[MatchLog]) -> Result<(), HandlerError> {
        Ok(())
    }

    fn replay_changelogs_to_state(&self, _changelogs: &[MatchLog]) -> Result<(), HandlerError> {
        Ok(())
    }

    fn publish_changelog(&self, _changelogs: &[MatchLog]) -> Result<(), HandlerError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_handler() {
        let handler = MatchHandler::new();
        let cmd = MatchCmd { match_id: "m1".into(), taker_order_id: "order_1".into() };

        let result = handler.cmd_handle(cmd, |writes, _| MatchOutput {
            result: MatchResult {
                trades: writes
                    .result
                    .trades
                    .iter()
                    .map(|trade| Trade {
                        trade_id: trade.trade_id.clone(),
                        taker_order_id: trade.taker_order_id.clone(),
                        maker_order_id: trade.maker_order_id.clone(),
                        price: trade.price,
                        quantity: trade.quantity,
                    })
                    .collect(),
                order_updates: vec![],
            },
            events: writes
                .events
                .iter()
                .map(|event| match event {
                    MatchEvent::TradeCreated(event) => {
                        MatchEvent::TradeCreated(TradeCreatedEvent {
                            trade_id: event.trade_id.clone(),
                        })
                    }
                })
                .collect(),
        });
        assert!(result.is_ok());
    }
}
