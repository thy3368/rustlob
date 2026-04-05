use crate::cmd_handler::{
    execute_trading_batch::context::ExecuteTradingBatchContext,
    execute_trading_batch_handler::{ExecuteTradingBatchError, ExecuteTradingBatchHandler},
    ExchangeCommandEnvelope, SpotCommand,
};

pub fn handle_spot_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &SpotCommand,
    _ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    todo!()
}
