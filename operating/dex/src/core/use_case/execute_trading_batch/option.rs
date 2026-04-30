use crate::core::{ExchangeCommandEnvelope, OptionCommand};
use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch::option_handler::OptionBatchHandler;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};

pub fn handle_option_command(
    handler: &OptionBatchHandler,
    envelope: &ExchangeCommandEnvelope,
    command: &OptionCommand,
    writes: &mut ExecutedBatchBlock,
    changelogs: &mut Vec<TradeExecutionLog>,
) -> Result<(), ExecuteTradingBatchError> {
    handler.handle_command(envelope, command, writes, changelogs)
}
