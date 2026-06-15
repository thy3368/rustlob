use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};

use crate::core::use_case::execute_trading_batch::{ExecuteTradingBatchError, SpotOrderBook, spot};
use crate::core::use_case::execute_trading_batch_handler::{
    BalanceDelta, BatchExecutionSummary, ExecutedBatchBlock, ExecutedOrder, ExecutedTrade,
    TradeExecutionLog,
};
use crate::core::{ExchangeCommandEnvelope, SpotCommand};

#[derive(Debug, Default)]
pub struct SpotBatchHandler {
    pub(crate) spot_order_book: Mutex<SpotOrderBook>,
    next_order_id: AtomicU64,
}

impl SpotBatchHandler {
    pub fn new() -> Self {
        Self { spot_order_book: Mutex::new(BTreeMap::new()), next_order_id: AtomicU64::new(1) }
    }

    pub(crate) fn next_order_id(&self) -> u64 {
        self.next_order_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn handle_command(
        &self,
        envelope: &ExchangeCommandEnvelope,
        command: &SpotCommand,
        writes: &mut ExecutedBatchBlock,
        changelogs: &mut Vec<TradeExecutionLog>,
    ) -> Result<(), ExecuteTradingBatchError> {
        let mut spot_order_book = self.spot_order_book.lock().unwrap();
        spot::handle_spot_command(self, envelope, command, writes, changelogs, &mut spot_order_book)
    }
}
