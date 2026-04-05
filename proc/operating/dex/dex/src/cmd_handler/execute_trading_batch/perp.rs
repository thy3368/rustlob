use crate::cmd_handler::{
    execute_trading_batch::{context::ExecuteTradingBatchContext, ExecuteTradingBatchError},
    execute_trading_batch_handler::ExecuteTradingBatchHandler,
    ExchangeCommandEnvelope, PerpCommand,
};

pub fn handle_perp_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &PerpCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}
