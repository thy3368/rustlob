use crate::core::{
    ExchangeCommandEnvelope, OptionCommand,
};
use crate::core::use_case::execute_trading_batch::{context::ExecuteTradingBatchContext, ExecuteTradingBatchError};
use crate::core::use_case::execute_trading_batch_handler::ExecuteTradingBatchHandler;

pub fn handle_option_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &OptionCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}
