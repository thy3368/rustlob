use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU64, Ordering};
use crossbeam_channel::{bounded, Receiver, Sender, unbounded};
use parking_lot::RwLock;
use crate::k_line::k_line_types::{CacheAligned, OHLC, TimeWindow, KLineAgg, KLineUpdateEvent, LockFreeRingBuffer};


// 高性能无锁聚合器
pub struct HighPerformanceKLineAggregator {
    // 原子化的当前窗口
    current_windows: [CacheAligned<Option<OHLC>>; 4],

    // 无锁历史缓冲区
    history_buffers: [LockFreeRingBuffer<OHLC>; 4],

    // 统计信息
    stats: CacheAligned<(AtomicU64, AtomicU64)>,  // (交易数, 交易量)

    // 时间窗口配置
    config: CacheAligned<AggregatorConfig>,
}

#[derive(Clone, Copy)]
struct AggregatorConfig {
    window_sizes: [u64; 4],
    history_capacities: [usize; 4],
}

impl HighPerformanceKLineAggregator {
    pub fn new() -> Self {
        let config = AggregatorConfig {
            window_sizes: [1, 60, 900, 3600],
            history_capacities: [3600, 1440, 672, 720],
        };

        let history_buffers = [
            LockFreeRingBuffer::new(config.history_capacities[0]),
            LockFreeRingBuffer::new(config.history_capacities[1]),
            LockFreeRingBuffer::new(config.history_capacities[2]),
            LockFreeRingBuffer::new(config.history_capacities[3]),
        ];

        Self {
            current_windows: [
                CacheAligned::new(None),
                CacheAligned::new(None),
                CacheAligned::new(None),
                CacheAligned::new(None),
            ],
            history_buffers,
            stats: CacheAligned::new((AtomicU64::new(0), AtomicU64::new(0))),
            config: CacheAligned::new(config),
        }
    }




}

//todo 完全重新实现
impl KLineAgg for HighPerformanceKLineAggregator {
    fn new() -> Self {
        todo!()
    }

    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + Send + Sync + 'static
    {
        todo!()
    }

    fn process_trade(&self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        todo!()
    }

    fn process_trades_batch(&self, trades: &[(u64, f64, f64)]) -> Result<(), String> {
        todo!()
    }

    fn get_current_ohlc(&self, window: TimeWindow) -> Option<OHLC> {
        todo!()
    }

    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> {
        todo!()
    }

    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) {
        todo!()
    }

    fn get_total_stats(&self) -> (u64, u64) {
        todo!()
    }
}