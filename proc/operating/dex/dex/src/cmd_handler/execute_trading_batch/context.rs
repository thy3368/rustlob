use crate::cmd_handler::execute_trading_batch_handler::{
    ExecutedBatchBlock, SpotOrderBook, TradeExecutionLog,
};

pub struct ExecuteTradingBatchContext<'a> {
    pub writes: &'a mut ExecutedBatchBlock,
    pub changelogs: &'a mut Vec<TradeExecutionLog>,
    pub spot_order_book: &'a mut SpotOrderBook,
}
