use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};
use crate::core::{ExchangeCommandEnvelope, OptionCommand};

#[derive(Debug, Default)]
pub struct OptionBatchHandler;

impl OptionBatchHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_command(
        &self,
        _envelope: &ExchangeCommandEnvelope,
        _command: &OptionCommand,
        writes: &mut ExecutedBatchBlock,
        _changelogs: &mut Vec<TradeExecutionLog>,
    ) -> Result<(), ExecuteTradingBatchError> {
        writes.summary.accepted_commands += 1;
        Ok(())
    }
}
