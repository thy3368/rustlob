use crate::core::{ExchangeCommandEnvelope, TreasuryCommand};
use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch::treasury_handler::TreasuryBatchHandler;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};

pub fn handle_treasury_command(
    handler: &TreasuryBatchHandler,
    envelope: &ExchangeCommandEnvelope,
    command: &TreasuryCommand,
    writes: &mut ExecutedBatchBlock,
    changelogs: &mut Vec<TradeExecutionLog>,
) -> Result<(), ExecuteTradingBatchError> {
    handler.handle_command(envelope, command, writes, changelogs)
}
