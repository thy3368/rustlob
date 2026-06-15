use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};
use crate::core::{ExchangeCommandEnvelope, TreasuryCommand};

#[derive(Debug, Default)]
pub struct TreasuryBatchHandler;

impl TreasuryBatchHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_command(
        &self,
        _envelope: &ExchangeCommandEnvelope,
        _command: &TreasuryCommand,
        writes: &mut ExecutedBatchBlock,
        _changelogs: &mut Vec<TradeExecutionLog>,
    ) -> Result<(), ExecuteTradingBatchError> {
        writes.summary.accepted_commands += 1;
        Ok(())
    }
}
