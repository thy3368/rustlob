use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use base_types::exchange::prep::perp_types::PrepTrade;
use base_types::exchange::prep::prep_order::PrepOrder;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler_update::{
    ApplyCommandChanges, ChangeSet, CmdHandlerForUpdate,
};
use super::execute_trading_batch::context::ExecuteTradingBatchContext;
use super::execute_trading_batch::{ExecuteTradingBatchError, SpotOrderBook};
use super::trading_command::{ExchangeCommand, ExchangeCommandEnvelope, TradingCommand};


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BatchExecutionSummary {
    pub total_commands: usize,
    pub accepted_commands: usize,
    pub rejected_commands: usize,
    pub orders_created: usize,
    pub trades_executed: usize,
    pub balance_updates: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutedOrder {
    SpotOrder(SpotOrder),
    PrepOrder(PrepOrder),
}

#[derive(Debug, Clone)]
pub enum ExecutedTrade {
    SpotTrade(SpotTrade),
    PrepTrade(PrepTrade),
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceDelta {
    pub trader_id: u64,
    pub asset: String,
    pub delta: i64,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutedBatchBlock {
    pub summary: BatchExecutionSummary,
    pub orders: Vec<ExecutedOrder>,
    pub trades: Vec<ExecutedTrade>,
    pub balance_deltas: Vec<BalanceDelta>,
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
pub struct ExecuteTradingBatchHandler {
    spot_order_book: Mutex<SpotOrderBook>,
    next_order_id: AtomicU64,
}

impl ExecuteTradingBatchHandler {
    pub fn new() -> Self {
        Self {
            spot_order_book: Mutex::new(BTreeMap::new()),
            next_order_id: AtomicU64::new(1),
        }
    }

    pub(crate) fn next_order_id(&self) -> u64 {
        self.next_order_id.fetch_add(1, Ordering::Relaxed)
    }
    fn handle_envelope(
        &self,
        envelope: &ExchangeCommandEnvelope,
        ctx: &mut ExecuteTradingBatchContext<'_>,
    ) -> Result<(), ExecuteTradingBatchError> {
        match &envelope.command {
            ExchangeCommand::TradingCommand(command) => {
                self.handle_trading_command(envelope, command, ctx)
            }
            ExchangeCommand::TreasuryCommand(command) => {
                super::execute_trading_batch::treasury::handle_treasury_command(
                    self,
                    envelope,
                    command,
                    ctx,
                )
            }
        }
    }

    fn handle_trading_command(
        &self,
        envelope: &ExchangeCommandEnvelope,
        command: &TradingCommand,
        ctx: &mut ExecuteTradingBatchContext<'_>,
    ) -> Result<(), ExecuteTradingBatchError> {
        match command {
            TradingCommand::Spot(command) => {
                super::execute_trading_batch::spot::handle_spot_command(self, envelope, command, ctx)
            }
            TradingCommand::Perp(command) => {
                super::execute_trading_batch::perp::handle_perp_command(self, envelope, command, ctx)
            }
            TradingCommand::Option(command) => {
                super::execute_trading_batch::option::handle_option_command(
                    self,
                    envelope,
                    command,
                    ctx,
                )
            }
        }
    }
}


//todo CmdHandlerForUpdate里对 ExchangeCommandEnvelope里面的子command 非常多，怎么做个每个子command的处理 提取到一个单独文件中方便维护？
impl ApplyCommandChanges<
    Vec<ExchangeCommandEnvelope>,
    ExecuteTradingBatchState,
    ExecutedBatchBlock,
    TradeExecutionLog,
    ExecuteTradingBatchError,
> for ExecuteTradingBatchHandler
{
    fn apply_command_and_collect_changes(
        &self,
        cmd: &Vec<ExchangeCommandEnvelope>,
        state_set: ExecuteTradingBatchState,
    ) -> Result<ChangeSet<ExecutedBatchBlock, TradeExecutionLog>, ExecuteTradingBatchError> {
        let mut writes = ExecutedBatchBlock {
            summary: BatchExecutionSummary {
                total_commands: state_set.batch_size,
                ..BatchExecutionSummary::default()
            },
            ..ExecutedBatchBlock::default()
        };

        let mut changelogs = Vec::new();
        let mut spot_order_book = self.spot_order_book.lock().unwrap();

        for envelope in cmd {
            self.handle_envelope(
                envelope,
                &mut ExecuteTradingBatchContext {
                    writes: &mut writes,
                    changelogs: &mut changelogs,
                    spot_order_book: &mut spot_order_book,
                },
            )?;
        }

        writes.summary.balance_updates = writes.balance_deltas.len();

        changelogs.push(TradeExecutionLog::BatchExecuted {
            batch_size: writes.summary.total_commands,
        });

        Ok(ChangeSet { writes, changelogs })
    }
}

impl CmdHandlerForUpdate<
    Vec<ExchangeCommandEnvelope>,
    ExecuteTradingBatchState,
    ExecutedBatchBlock,
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
    fn cmd_handler_for_update_emits_batch_log() {
        let handler = ExecuteTradingBatchHandler::new();

        let result = handler.cmd_handle(vec![place_order_envelope(1, 1)], |writes, changelogs| {
            (writes.clone(), changelogs.to_vec())
        });

        assert!(result.is_ok());
        let (writes, changelogs) = result.unwrap();
        assert_eq!(writes.summary.total_commands, 1);
        assert_eq!(changelogs.len(), 1);
        assert_eq!(
            changelogs[0],
            TradeExecutionLog::BatchExecuted { batch_size: 1 }
        );
    }
}
