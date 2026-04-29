use crate::core::{
    ExchangeCommandEnvelope, TreasuryCommand,
};
use crate::core::use_case::execute_trading_batch::{context::ExecuteTradingBatchContext, ExecuteTradingBatchError};
use crate::core::use_case::execute_trading_batch_handler::ExecuteTradingBatchHandler;

pub fn handle_treasury_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &TreasuryCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}
