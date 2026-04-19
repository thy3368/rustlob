use crate::cmd_handler::{
    execute_trading_batch::{context::ExecuteTradingBatchContext, ExecuteTradingBatchError},
    execute_trading_batch_handler::ExecuteTradingBatchHandler,
    ExchangeCommandEnvelope, TreasuryCommand,
};

pub fn handle_treasury_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &TreasuryCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}
