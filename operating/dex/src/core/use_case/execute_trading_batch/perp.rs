use crate::core::{ExchangeCommandEnvelope, PerpCommand};
use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch::perp_handler::PerpBatchHandler;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};

pub fn handle_perp_command(
    handler: &PerpBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &PerpCommand,
    writes: &mut ExecutedBatchBlock,
    _changelogs: &mut Vec<TradeExecutionLog>,
) -> Result<(), ExecuteTradingBatchError> {
    handler.handle_command(_envelope, _command, writes, _changelogs)
}
