use crate::cmd_handler::{
    execute_trading_batch::context::ExecuteTradingBatchContext,
    execute_trading_batch_handler::{ExecuteTradingBatchError, ExecuteTradingBatchHandler},
    ExchangeCommandEnvelope, OptionCommand,
};

pub fn handle_option_command(
    _handler: &ExecuteTradingBatchHandler,
    _envelope: &ExchangeCommandEnvelope,
    _command: &OptionCommand,
    _ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    todo!()
}
