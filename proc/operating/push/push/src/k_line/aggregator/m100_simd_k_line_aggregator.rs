use crate::k_line::k_line_types::{KLineAgg, KLineUpdateEvent, TimeWindow, OHLC};

// todo 实现 Aggregator 10亿/秒处理的 参考
// /Users/hongyaotang/src/rustlob/proc/operating/push/push/docs/
// SingleThreadSimdKLineAggregator_Optimization_Plan.md
pub struct M100SimdKLineAggregator {}

impl KLineAgg for M100SimdKLineAggregator {
    fn new() -> Self { todo!() }

    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + Send + Sync + 'static
    {
        todo!()
    }

    fn process_trade(&self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> { todo!() }

    fn process_trades_batch(&self, trades: &[(u64, f64, f64)]) -> Result<(), String> { todo!() }

    fn get_current_ohlc(&self, window: TimeWindow) -> Option<OHLC> { todo!() }

    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> { todo!() }

    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) { todo!() }

    fn get_total_stats(&self) -> (u64, u64) { todo!() }
}
