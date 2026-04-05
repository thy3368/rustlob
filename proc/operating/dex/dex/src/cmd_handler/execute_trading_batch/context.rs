use base_types::handler::handler_update::ChangeSet;

use crate::cmd_handler::execute_trading_batch::SpotOrderBook;
use crate::cmd_handler::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};

pub struct ExecuteTradingBatchContext<'a> {
    pub writes: &'a mut ExecutedBatchBlock,
    pub changelogs: &'a mut Vec<TradeExecutionLog>,
    pub spot_order_book: &'a mut SpotOrderBook,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SpotCommandState;

pub type SpotCommandChangeSet = ChangeSet<ExecutedBatchBlock, TradeExecutionLog>;
