use base_types::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};

use super::trading_command::{
    ExchangeCommand, ExchangeCommandEnvelope, OptionCommand, PerpCommand, SpotCommand,
};

type ExecuteTradingBatchError = String;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TradeExecutionResult {
    pub market: String,
    pub maker_order_id: u64,
    pub taker_order_id: u64,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BatchExecutionResult {
    pub total_commands: usize,
    pub place_order_commands: usize,
    pub cancel_order_commands: usize,
    pub amend_order_commands: usize,
    pub trade_execution_commands: usize,
    pub trades_executed: Vec<TradeExecutionResult>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExecuteTradingBatchState {
    pub batch_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeExecutionLog {
    TradeExecuted {
        market: String,
        maker_order_id: u64,
        taker_order_id: u64,
        price: u64,
        quantity: u64,
    },
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
    Vec<ExchangeCommandEnvelope>,
    ExecuteTradingBatchState,
    BatchExecutionResult,
    TradeExecutionLog,
    ExecuteTradingBatchError,
> for ExecuteTradingBatchHandler
{
    fn pre_check_command(
        &self,
        _cmd: &Vec<ExchangeCommandEnvelope>,
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        cmd: &Vec<ExchangeCommandEnvelope>,
    ) -> Result<ExecuteTradingBatchState, ExecuteTradingBatchError> {
        Ok(ExecuteTradingBatchState {
            batch_size: cmd.len(),
        })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &Vec<ExchangeCommandEnvelope>,
        _state_set: &ExecuteTradingBatchState,
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Vec<ExchangeCommandEnvelope>,
        state_set: ExecuteTradingBatchState,
    ) -> Result<ChangeSet<BatchExecutionResult, TradeExecutionLog>, ExecuteTradingBatchError>
    {
        let mut writes = BatchExecutionResult {
            total_commands: state_set.batch_size,
            ..BatchExecutionResult::default()
        };

        let mut changelogs = Vec::new();

        for envelope in cmd {
            match &envelope.command {
                ExchangeCommand::TradingCommand(trading_command) => match trading_command {
                    super::trading_command::TradingCommand::Spot(SpotCommand::PlaceOrder(_)) => {
                        writes.place_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Spot(SpotCommand::CancelOrder(_)) => {
                        writes.cancel_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Spot(SpotCommand::AmendOrder(_)) => {
                        writes.amend_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Perp(PerpCommand::PlaceOrder(_)) => {
                        writes.place_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Perp(PerpCommand::CancelOrder(_)) => {
                        writes.cancel_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Perp(PerpCommand::AmendOrder(_)) => {
                        writes.amend_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Perp(PerpCommand::ExecuteTrade(cmd)) => {
                        writes.trade_execution_commands += 1;
                        writes.trades_executed.push(TradeExecutionResult {
                            market: cmd.market.clone(),
                            maker_order_id: cmd.maker_order_id,
                            taker_order_id: cmd.taker_order_id,
                            price: cmd.price,
                            quantity: cmd.quantity,
                        });
                        changelogs.push(TradeExecutionLog::TradeExecuted {
                            market: cmd.market.clone(),
                            maker_order_id: cmd.maker_order_id,
                            taker_order_id: cmd.taker_order_id,
                            price: cmd.price,
                            quantity: cmd.quantity,
                        });
                    }
                    super::trading_command::TradingCommand::Perp(
                        PerpCommand::SettleFunding(_) | PerpCommand::LiquidatePosition(_),
                    ) => {}
                    super::trading_command::TradingCommand::Option(OptionCommand::PlaceOrder(_)) => {
                        writes.place_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Option(OptionCommand::CancelOrder(_)) => {
                        writes.cancel_order_commands += 1
                    }
                    super::trading_command::TradingCommand::Option(OptionCommand::AmendOrder(_)) => {
                        writes.amend_order_commands += 1
                    }
                },
                ExchangeCommand::TreasuryCommand(_) => {}
            }
        }

        changelogs.push(TradeExecutionLog::BatchExecuted {
            batch_size: state_set.batch_size,
        });

        Ok(ChangeSet {
            writes,
            changelogs,
        })
    }

    fn persist_changelogs(
        &self,
        _changelogs: &[TradeExecutionLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn replay_changelogs_to_state(
        &self,
        _changelogs: &[TradeExecutionLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn publish_changelog(
        &self,
        _changelogs: &[TradeExecutionLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd_handler::{
        ExchangeCommand, PerpAmendOrderCmd, PerpCommand, PerpPlaceOrderCmd, PerpSide,
        SpotCancelOrderCmd, SpotCommand, TradingCommand,
    };

    fn place_order_envelope(command_id: u64, trader_id: u64) -> ExchangeCommandEnvelope {
        ExchangeCommandEnvelope {
            command_id,
            trader_id,
            nonce: command_id,
            timestamp_ns: 1_000 + command_id,
            command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                PerpCommand::PlaceOrder(PerpPlaceOrderCmd {
                    trader_id,
                    market: "BTC-PERP".into(),
                    side: PerpSide::Buy,
                    price: 100_000,
                    quantity: 2,
                    leverage: 1,
                    reduce_only: false,
                }),
            )),
        }
    }

    #[test]
    fn cmd_handle_counts_each_command_kind() {
        let handler = ExecuteTradingBatchHandler::new();

        let result = handler.cmd_handle(
            vec![
                place_order_envelope(1, 1),
                ExchangeCommandEnvelope {
                    command_id: 2,
                    trader_id: 1,
                    nonce: 2,
                    timestamp_ns: 1_002,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Spot(
                        SpotCommand::CancelOrder(SpotCancelOrderCmd {
                            trader_id: 1,
                            order_id: 42,
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 3,
                    trader_id: 1,
                    nonce: 3,
                    timestamp_ns: 1_003,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::AmendOrder(PerpAmendOrderCmd {
                            trader_id: 1,
                            order_id: 42,
                            new_price: Some(101_000),
                            new_quantity: Some(3),
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 4,
                    trader_id: 2,
                    nonce: 4,
                    timestamp_ns: 1_004,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::PlaceOrder(PerpPlaceOrderCmd {
                            trader_id: 2,
                            market: "ETH-PERP".into(),
                            side: PerpSide::Sell,
                            price: 3_000,
                            quantity: 5,
                            leverage: 1,
                            reduce_only: false,
                        }),
                    )),
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
            TradeExecutionLog::BatchExecuted { batch_size: 1 }
        );
    }
}
