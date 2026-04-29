use base_types::handler::handler_update::ChangeSet;

use crate::core::use_case::execute_trading_batch::SpotOrderBook;
use crate::core::use_case::execute_trading_batch_handler::{
    ExecutedBatchBlock, ExecuteTradingBatchHandler, TradeExecutionLog,
};
use crate::core::ExchangeCommandEnvelope;

pub struct ExecuteTradingBatchContext<'a> {
    pub writes: &'a mut ExecutedBatchBlock,
    pub changelogs: &'a mut Vec<TradeExecutionLog>,
    pub spot_order_book: &'a mut SpotOrderBook,
}

pub struct SpotCommandState<'a> {
    pub handler: &'a ExecuteTradingBatchHandler,
    pub envelope: &'a ExchangeCommandEnvelope,
    pub spot_order_book: &'a mut SpotOrderBook,
}

pub type SpotCommandChangeSet = ChangeSet<ExecutedBatchBlock, TradeExecutionLog>;
