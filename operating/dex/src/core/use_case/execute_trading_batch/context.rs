use base_types::handler::handler_update::ChangeSet;

use crate::core::use_case::execute_trading_batch::SpotOrderBook;
use crate::core::use_case::execute_trading_batch::spot::handler::SpotBatchHandler;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};
use crate::core::ExchangeCommandEnvelope;

pub type SpotCommandState<'a> = (&'a SpotBatchHandler, &'a ExchangeCommandEnvelope, &'a mut SpotOrderBook);

pub type SpotCommandChangeSet = ChangeSet<ExecutedBatchBlock, TradeExecutionLog>;
