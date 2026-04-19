use crate::cmd_handler::{
    execute_trading_batch::{context::ExecuteTradingBatchContext, ExecuteTradingBatchError},
    execute_trading_batch_handler::ExecuteTradingBatchHandler,
    ExchangeCommandEnvelope, OptionCommand,
};

pub fn handle_option_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &OptionCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}
