use crate::core::{ExchangeCommandEnvelope, PerpCommand};
use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};

#[derive(Debug, Default)]
pub struct PerpBatchHandler;

impl PerpBatchHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_command(
        &self,
        _envelope: &ExchangeCommandEnvelope,
        _command: &PerpCommand,
        writes: &mut ExecutedBatchBlock,
        _changelogs: &mut Vec<TradeExecutionLog>,
    ) -> Result<(), ExecuteTradingBatchError> {
        writes.summary.accepted_commands += 1;
        Ok(())
    }
}
