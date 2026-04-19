use std::cell::UnsafeCell;
use std::sync::Arc;

use once_cell::sync::Lazy;
use rayon::ThreadPool;
use rayon::prelude::*;

use crate::k_line::aggregator::m100_simd_k_single_line_aggregator::M100SimdKSingleLineAggregator;
use crate::k_line::k_line_types::{KLineAgg, KLineAggMut, KLineUpdateEvent, OHLC, TimeWindow};

// 创建固定大小的线程池（4个线程，对应4个时间窗口）
static AGGREGATOR_THREAD_POOL: Lazy<ThreadPool> = Lazy::new(|| {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|i| format!("kline-agg-{}", i))
        .build()
        .expect("Failed to create KLine aggregator thread pool")
});

// 为 M100SimdKSingleLineAggregator 实现无锁内部可变性
struct UnsafeCellWrapper<T> {
    inner: UnsafeCell<T>,
}

impl<T> UnsafeCellWrapper<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        UnsafeCellWrapper { inner: UnsafeCell::new(value) }
    }

    #[inline(always)]
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.inner.get()
    }
}

// 不安全的 Send 和 Sync 实现，因为我们将确保每个 UnsafeCellWrapper 在固定线程上使用
unsafe impl<T: Send> Send for UnsafeCellWrapper<T> {}
unsafe impl<T: Sync> Sync for UnsafeCellWrapper<T> {}

// M100PSimdKLineAggregator - 使用 Rayon 的并行无锁版本
// 为每个时间窗口分配单独的线程，使用 UnsafeCell 提供无锁内部可变性
// 设计目标：通过线程固定和无锁编程进一步提高性能
pub struct M100PSimdKLineAggregator {
    // 每个窗口有独立的聚合器，完全避免线程间冲突
    // 使用 UnsafeCell 提供无锁内部可变性
    window_aggregators: [UnsafeCellWrapper<M100SimdKSingleLineAggregator>; 4],

    // 全局计数器（普通变量，无并发写入）
    total_trades: u64,
    total_volume: u64,

    // 事件处理器列表 - 使用Arc确保线程安全
    event_handlers: Arc<[Option<Arc<dyn Fn(KLineUpdateEvent) + Send + Sync>>; 8]>,
    event_handler_count: usize,
}

impl M100PSimdKLineAggregator {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            window_aggregators: [
                UnsafeCellWrapper::new(M100SimdKSingleLineAggregator::new(TimeWindow::Second)),
                UnsafeCellWrapper::new(M100SimdKSingleLineAggregator::new(TimeWindow::Minute)),
                UnsafeCellWrapper::new(M100SimdKSingleLineAggregator::new(TimeWindow::FifteenMin)),
                UnsafeCellWrapper::new(M100SimdKSingleLineAggregator::new(TimeWindow::Hour)),
            ],
            total_trades: 0,
            total_volume: 0,
            event_handlers: Arc::new([None, None, None, None, None, None, None, None]),
            event_handler_count: 0,
        }
    }

    #[inline(always)]
    fn send_event(&self, window: TimeWindow, ohlc: OHLC, is_new_window: bool) {
        let event = KLineUpdateEvent { window, ohlc, is_new_window };

        let handler_count = self.event_handler_count;
        let handlers = &self.event_handlers;

        for i in 0..handler_count {
            if let Some(handler) = &handlers[i] {
                handler(event.clone());
            }
        }
    }
}

impl KLineAggMut for M100PSimdKLineAggregator {
    #[inline(always)]
    fn new() -> Self {
        Self::new()
    }

    #[inline(always)]
    fn subscribe<F>(&mut self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + 'static + Send + Sync,
    {
        let handler_arc = Arc::new(handler);

        // 需要内部可变性来修改事件处理器列表
        let mut handlers = self.event_handlers.as_ref().clone();
        let handler_count = self.event_handler_count;

        if handler_count < 8 {
            handlers[handler_count] = Some(handler_arc);
            self.event_handler_count += 1;
            self.event_handlers = Arc::new(handlers);

            // 同时将事件处理器添加到每个窗口聚合器
            for (window_idx, aggregator) in self.window_aggregators.iter_mut().enumerate() {
                let window = match window_idx {
                    0 => TimeWindow::Second,
                    1 => TimeWindow::Minute,
                    2 => TimeWindow::FifteenMin,
                    3 => TimeWindow::Hour,
                    _ => unreachable!(),
                };

                let handler_clone = self.event_handlers[handler_count - 1].clone().unwrap();
                let aggregator = unsafe { aggregator.get_mut() };
                aggregator.subscribe(move |event| {
                    if event.window == window {
                        handler_clone(event);
                    }
                });
            }
        }
    }

    #[inline(always)]
    fn process_trade(&mut self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        // 单笔交易处理：完全避免并行，因为任务粒度太小
        // 线程调度开销会完全抵消并行的好处
        self.total_trades += 1;
        self.total_volume += volume as u64;

        // 单线程处理所有窗口
        for window_idx in 0..4 {
            let aggregator = unsafe { self.window_aggregators[window_idx].get_mut() };
            aggregator.process_trade(timestamp, price, volume).expect("Failed to process trade");
        }

        Ok(())
    }

    #[inline(always)]
    fn process_trades_batch(&mut self, trades: &[(u64, f64, f64)]) -> Result<(), String> {
        // 批量更新计数器
        let trade_count = trades.len() as u64;
        let volume_sum = trades.iter().map(|&(_, _, v)| v as u64).sum::<u64>();
        self.total_trades += trade_count;
        self.total_volume += volume_sum;

        // 增大任务粒度：将交易分成较大的块，每个块由一个线程处理
        // 对于大任务量，使用更粗粒度的并行划分
        let chunk_size = if trades.len() > 100_000 {
            // 对于100万笔交易，分成10个块，每个块10万笔
            trades.len() / 10
        } else if trades.len() > 10_000 {
            // 对于10万-100万笔交易，分成8个块
            trades.len() / 8
        } else {
            // 对于小任务量，不分块（单线程处理）
            trades.len()
        };

        if chunk_size == trades.len() {
            // 小任务量，单线程处理
            for window_idx in 0..4 {
                let aggregator = unsafe { self.window_aggregators[window_idx].get_mut() };
                aggregator.process_trades_batch(trades).expect("Failed to process trades batch");
            }
        } else {
            // 大任务量，并行处理
            AGGREGATOR_THREAD_POOL.install(|| {
                (0..4).into_par_iter().for_each(|window_idx| {
                    let aggregator = unsafe { self.window_aggregators[window_idx].get_mut() };
                    // 每个线程处理整个批次的交易，但可以考虑块级别的优化
                    aggregator
                        .process_trades_batch(trades)
                        .expect("Failed to process trades batch");
                });
            });
        }

        Ok(())
    }

    #[inline(always)]
    fn get_current_ohlc(&self, window: TimeWindow) -> Option<OHLC> {
        let idx = window.index();
        let aggregator = unsafe { self.window_aggregators[idx].get_mut() };
        aggregator.get_current_ohlc(window)
    }

    #[inline(always)]
    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> {
        let idx = window.index();
        let aggregator = unsafe { self.window_aggregators[idx].get_mut() };
        aggregator.get_history_ohlc(window, limit)
    }

    #[inline(always)]
    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) {
        let idx = window.index();
        let aggregator = unsafe { self.window_aggregators[idx].get_mut() };
        aggregator.get_sliding_stats(window, period)
    }

    #[inline(always)]
    fn get_total_stats(&self) -> (u64, u64) {
        (self.total_trades, self.total_volume)
    }
}

// 为 M100PSimdKLineAggregator 添加默认实现
impl Default for M100PSimdKLineAggregator {
    fn default() -> Self {
        Self::new()
    }
}
