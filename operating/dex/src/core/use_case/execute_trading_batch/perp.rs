use crate::core::{
    ExchangeCommandEnvelope, PerpCommand,
};
use crate::core::use_case::execute_trading_batch::{context::ExecuteTradingBatchContext, ExecuteTradingBatchError};
use crate::core::use_case::execute_trading_batch_handler::ExecuteTradingBatchHandler;

pub fn handle_perp_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &PerpCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}
