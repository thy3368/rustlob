use base_types::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};

use super::trading_command::{
    AmendOrderCmd, CancelOrderCmd, OrderSide, PlaceOrderCmd, TradingCommand,
    TradingCommandEnvelope,
};

type ExecuteTradingBatchError = String;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BatchExecutionResult {
    pub total_commands: usize,
    pub place_order_commands: usize,
    pub cancel_order_commands: usize,
    pub amend_order_commands: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExecuteTradingBatchState {
    pub batch_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteTradingBatchLog {
    BatchExecuted { batch_size: usize },
}

#[derive(Debug, Default)]
pub struct ExecuteTradingBatchHandler;

impl ExecuteTradingBatchHandler {
    pub fn new() -> Self {
        Self
    }
}

impl CmdHandlerForUpdate<
    Vec<TradingCommandEnvelope>,
    ExecuteTradingBatchState,
    BatchExecutionResult,
    ExecuteTradingBatchLog,
    ExecuteTradingBatchError,
> for ExecuteTradingBatchHandler
{
    fn pre_check_command(
        &self,
        _cmd: &Vec<TradingCommandEnvelope>,
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        cmd: &Vec<TradingCommandEnvelope>,
    ) -> Result<ExecuteTradingBatchState, ExecuteTradingBatchError> {
        Ok(ExecuteTradingBatchState {
            batch_size: cmd.len(),
        })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &Vec<TradingCommandEnvelope>,
        _state_set: &ExecuteTradingBatchState,
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Vec<TradingCommandEnvelope>,
        state_set: ExecuteTradingBatchState,
    ) -> Result<ChangeSet<BatchExecutionResult, ExecuteTradingBatchLog>, ExecuteTradingBatchError>
    {
        let mut writes = BatchExecutionResult {
            total_commands: state_set.batch_size,
            ..BatchExecutionResult::default()
        };

        for envelope in cmd {
            match &envelope.command {
                TradingCommand::PlaceOrder(_) => writes.place_order_commands += 1,
                TradingCommand::CancelOrder(_) => writes.cancel_order_commands += 1,
                TradingCommand::AmendOrder(_) => writes.amend_order_commands += 1,
            }
        }

        Ok(ChangeSet {
            writes,
            changelogs: vec![ExecuteTradingBatchLog::BatchExecuted {
                batch_size: state_set.batch_size,
            }],
        })
    }

    fn persist_changelogs(
        &self,
        _changelogs: &[ExecuteTradingBatchLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn replay_changelogs_to_state(
        &self,
        _changelogs: &[ExecuteTradingBatchLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn publish_changelog(
        &self,
        _changelogs: &[ExecuteTradingBatchLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place_order_envelope(command_id: u64, trader_id: u64) -> TradingCommandEnvelope {
        TradingCommandEnvelope {
            command_id,
            trader_id,
            nonce: command_id,
            timestamp_ns: 1_000 + command_id,
            command: TradingCommand::PlaceOrder(PlaceOrderCmd {
                trader_id,
                market: "BTC-PERP".into(),
                side: OrderSide::Buy,
                price: 100_000,
                quantity: 2,
            }),
        }
    }

    #[test]
    fn cmd_handle_counts_each_command_kind() {
        let handler = ExecuteTradingBatchHandler::new();

        let result = handler.cmd_handle(
            vec![
                place_order_envelope(1, 1),
                TradingCommandEnvelope {
                    command_id: 2,
                    trader_id: 1,
                    nonce: 2,
                    timestamp_ns: 1_002,
                    command: TradingCommand::CancelOrder(CancelOrderCmd {
                        trader_id: 1,
                        order_id: 42,
                    }),
                },
                TradingCommandEnvelope {
                    command_id: 3,
                    trader_id: 1,
                    nonce: 3,
                    timestamp_ns: 1_003,
                    command: TradingCommand::AmendOrder(AmendOrderCmd {
                        trader_id: 1,
                        order_id: 42,
                        new_price: Some(101_000),
                        new_quantity: Some(3),
                    }),
                },
                TradingCommandEnvelope {
                    command_id: 4,
                    trader_id: 2,
                    nonce: 4,
                    timestamp_ns: 1_004,
                    command: TradingCommand::PlaceOrder(PlaceOrderCmd {
                        trader_id: 2,
                        market: "ETH-PERP".into(),
                        side: OrderSide::Sell,
                        price: 3_000,
                        quantity: 5,
                    }),
                },
            ],
            |writes, _| writes.clone(),
        );

        assert!(result.is_ok());
        let writes = result.unwrap();
        assert_eq!(writes.total_commands, 4);
        assert_eq!(writes.place_order_commands, 2);
        assert_eq!(writes.cancel_order_commands, 1);
        assert_eq!(writes.amend_order_commands, 1);
    }

    #[test]
    fn cmd_handler_for_update_emits_batch_log() {
        let handler = ExecuteTradingBatchHandler::new();

        let result = handler.cmd_handle(vec![place_order_envelope(1, 1)], |writes, changelogs| {
            (writes.clone(), changelogs.to_vec())
        });

        assert!(result.is_ok());
        let (writes, changelogs) = result.unwrap();
        assert_eq!(writes.total_commands, 1);
        assert_eq!(changelogs.len(), 1);
        assert_eq!(
            changelogs[0],
            ExecuteTradingBatchLog::BatchExecuted { batch_size: 1 }
        );
    }
}
