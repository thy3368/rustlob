use crate::cmd_handler::{
    execute_trading_batch::context::ExecuteTradingBatchContext,
    execute_trading_batch_handler::{ExecuteTradingBatchError, ExecuteTradingBatchHandler},
    ExchangeCommandEnvelope, PerpCommand,
};

pub fn handle_perp_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &PerpCommand,
    _ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    todo!()
}
