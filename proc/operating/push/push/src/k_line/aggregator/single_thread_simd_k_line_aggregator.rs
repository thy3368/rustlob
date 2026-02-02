use std::simd::{f64x8, num::SimdFloat};
use std::collections::VecDeque;
use std::cell::RefCell;

use crate::k_line::k_line_types::{
    KLineAgg, KLineUpdateEvent, TimeWindow, OHLC, TradeDataSoA
};

// 单线程 SIMD 优化的 K 线聚合器
// 不考虑数据竞争，专为单线程场景优化性能
//
// 优化空间分析：
// 1. 去掉了所有线程安全机制（RwLock、Atomic），避免了锁开销
// 2. 使用 VecDeque 代替 LockFreeRingBuffer，简化数据结构
// 3. 直接字段访问，避免了原子操作的开销
// 4. 使用 RefCell 实现内部可变性，以满足 trait 不可变引用的要求
// 5. 预期性能提升：20-30%（与多线程版本相比）
//
pub struct SingleThreadSimdKLineAggregator {
    // 当前活跃窗口 [1s, 1m, 15m, 1h]
    current_windows: RefCell<[Option<OHLC>; 4]>,

    // 历史K线存储（直接使用VecDeque）
    history_1s: RefCell<VecDeque<OHLC>>,  // 存储最近3600秒
    history_1m: RefCell<VecDeque<OHLC>>,  // 存储最近1440分钟
    history_15m: RefCell<VecDeque<OHLC>>, // 存储最近672个15分钟
    history_1h: RefCell<VecDeque<OHLC>>,  // 存储最近720小时

    // 滑动窗口统计（直接使用VecDeque）
    sliding_1s: RefCell<VecDeque<OHLC>>,  // 最近60秒
    sliding_1m: RefCell<VecDeque<OHLC>>,  // 最近60分钟
    sliding_15m: RefCell<VecDeque<OHLC>>, // 最近96个15分钟
    sliding_1h: RefCell<VecDeque<OHLC>>,  // 最近168小时

    // 普通计数器（不需要原子操作）
    total_trades: RefCell<u64>,
    total_volume: RefCell<u64>,

    // 预计算的窗口大小
    window_sizes: [u64; 4],
    // 预计算的历史容量
    history_capacities: [usize; 4],
    // 预计算的滑动窗口容量
    sliding_capacities: [usize; 4],

    // 事件处理器列表
    event_handlers: RefCell<Vec<Box<dyn Fn(KLineUpdateEvent)>>>,

    // 批处理缓冲区
    batch_buffer: RefCell<Vec<(u64, f64, f64)>>,
    batch_size: usize
}

impl SingleThreadSimdKLineAggregator {
    pub fn new_with_batch_size(batch_size: usize) -> Self {
        Self {
            current_windows: RefCell::new([None, None, None, None]),
            history_1s: RefCell::new(VecDeque::with_capacity(3600)),
            history_1m: RefCell::new(VecDeque::with_capacity(1440)),
            history_15m: RefCell::new(VecDeque::with_capacity(672)),
            history_1h: RefCell::new(VecDeque::with_capacity(720)),
            sliding_1s: RefCell::new(VecDeque::with_capacity(60)),
            sliding_1m: RefCell::new(VecDeque::with_capacity(60)),
            sliding_15m: RefCell::new(VecDeque::with_capacity(96)),
            sliding_1h: RefCell::new(VecDeque::with_capacity(168)),
            total_trades: RefCell::new(0),
            total_volume: RefCell::new(0),
            window_sizes: [1, 60, 900, 3600],
            history_capacities: [3600, 1440, 672, 720],
            sliding_capacities: [60, 60, 96, 168],
            event_handlers: RefCell::new(Vec::new()),
            batch_buffer: RefCell::new(Vec::with_capacity(batch_size)),
            batch_size
        }
    }

    // 发送K线更新事件
    fn send_event(&self, window: TimeWindow, ohlc: OHLC, is_new_window: bool) {
        let event = KLineUpdateEvent {
            window,
            ohlc,
            is_new_window
        };

        let handlers = self.event_handlers.borrow();
        for handler in handlers.iter() {
            handler(event.clone());
        }
    }

    // 核心更新逻辑
    fn update_window(&self, window_idx: usize, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        let window_size = self.window_sizes[window_idx];
        let window_start = (timestamp / window_size) * window_size;
        let window = match window_idx {
            0 => TimeWindow::Second,
            1 => TimeWindow::Minute,
            2 => TimeWindow::FifteenMin,
            3 => TimeWindow::Hour,
            _ => return Err("Invalid window index".to_string())
        };

        let mut current_windows = self.current_windows.borrow_mut();
        let need_update_sliding = match &mut current_windows[window_idx] {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                // 更新当前窗口
                current_ohlc.update(price, volume);
                true
            }
            _ => {
                // 保存旧窗口到历史
                if let Some(old) = current_windows[window_idx].take() {
                    self.save_to_history(window_idx, old.clone());
                    self.send_event(window, old, true);
                }

                // 创建新的当前窗口
                let new_ohlc = OHLC::new(window_start, price, volume);
                current_windows[window_idx] = Some(new_ohlc);
                true
            }
        };

        if need_update_sliding {
            let current_ohlc = current_windows[window_idx].unwrap();
            self.update_sliding_window(window_idx, current_ohlc);
        }

        Ok(())
    }

    fn update_sliding_window(&self, window_idx: usize, ohlc: OHLC) {
        let sliding_window = match window_idx {
            0 => &self.sliding_1s,
            1 => &self.sliding_1m,
            2 => &self.sliding_15m,
            3 => &self.sliding_1h,
            _ => return
        };

        let mut sliding_window = sliding_window.borrow_mut();
        let capacity = self.sliding_capacities[window_idx];

        // 检查是否已存在相同时间戳的K线，避免重复添加
        if let Some(last) = sliding_window.back() {
            if last.timestamp == ohlc.timestamp {
                return; // 暂时简单处理，不添加重复时间戳的K线
            }
        }

        // 检查容量并删除最旧的K线
        if sliding_window.len() >= capacity {
            sliding_window.pop_front();
        }

        // 添加新的K线
        sliding_window.push_back(ohlc);
    }

    fn save_to_history(&self, window_idx: usize, ohlc: OHLC) {
        let history = match window_idx {
            0 => &self.history_1s,
            1 => &self.history_1m,
            2 => &self.history_15m,
            3 => &self.history_1h,
            _ => return
        };

        let mut history = history.borrow_mut();
        let capacity = self.history_capacities[window_idx];

        if history.len() >= capacity {
            history.pop_front();
        }
        history.push_back(ohlc);
    }

    // SIMD 优化的批量处理内部方法
    fn process_batch_internal(&self, trades: &[(u64, f64, f64)]) -> Result<(), String> {
        // 对于小于8个交易的批次，直接处理
        if trades.len() < 8 {
            for &(timestamp, price, volume) in trades {
                self.process_trade(timestamp, price, volume)?;
            }
            return Ok(());
        }

        // 直接在栈上预分配缓冲区，避免堆分配
        const STACK_BUFFER_SIZE: usize = 256;
        let mut ts_stack = [0u64; STACK_BUFFER_SIZE];
        let mut p_stack = [0.0f64; STACK_BUFFER_SIZE];
        let mut v_stack = [0.0f64; STACK_BUFFER_SIZE];

        if trades.len() <= STACK_BUFFER_SIZE {
            // 小批量直接使用栈缓冲区
            for (i, &(ts, p, v)) in trades.iter().enumerate() {
                ts_stack[i] = ts;
                p_stack[i] = p;
                v_stack[i] = v;
            }
            self.process_with_simd(
                &ts_stack[..trades.len()],
                &p_stack[..trades.len()],
                &v_stack[..trades.len()]
            )?;
        } else {
            // 大容量使用 SoA 布局
            let data = TradeDataSoA::from_aos(trades);
            self.process_with_simd(&data.timestamps, &data.prices, &data.volumes)?;
        }

        Ok(())
    }

    #[inline(always)]
    unsafe fn prefetch<T>(ptr: *const T, hint: i32) {
        #[cfg(target_arch = "x86_64")]
        std::arch::x86_64::_mm_prefetch(ptr as *const i8, hint);
    }

    #[inline(always)]
    fn process_with_simd(&self, timestamps: &[u64], prices: &[f64], volumes: &[f64]) -> Result<(), String> {
        let len = timestamps.len();
        let chunks = len / 8;
        let remainder = len % 8;

        // 处理完整的 8 元素块
        for i in 0..chunks {
            let start = i * 8;
            let end = start + 8;

            // 预取下一个块的数据（距离设为T0 - 时间局部性）
            if i < chunks - 1 {
                let next_start = (i + 1) * 8;
                unsafe {
                    // _MM_HINT_T0 = 0x00 - 最高优先级预取
                    Self::prefetch(timestamps.as_ptr().add(next_start), 0x00);
                    Self::prefetch(prices.as_ptr().add(next_start), 0x00);
                    Self::prefetch(volumes.as_ptr().add(next_start), 0x00);
                }
            }

            // 加载到 SIMD 向量
            let price_vec = f64x8::from_slice(&prices[start..end]);
            let volume_vec = f64x8::from_slice(&volumes[start..end]);

            // SIMD 计算统计信息
            let max_price = price_vec.reduce_max();
            let min_price = price_vec.reduce_min();
            let sum_volume = volume_vec.reduce_sum();

            // 获取该块的时间戳范围
            let chunk_timestamps = &timestamps[start..end];
            let first_ts = chunk_timestamps[0];
            let last_ts = chunk_timestamps[7];
            let first_price = prices[start];
            let last_price = prices[end - 1];

            // 计算该块的成交量总和（使用 SIMD 结果）
            let chunk_volume = sum_volume;

            // 对于每个时间窗口，检查是否需要更新
            for window_idx in 0..4 {
                let window_size = self.window_sizes[window_idx];
                let first_window = (first_ts / window_size) * window_size;
                let last_window = (last_ts / window_size) * window_size;

                if first_window == last_window {
                    // 整个块在同一窗口中，进行优化更新
                    self.update_window_with_stats(
                        window_idx,
                        first_window,
                        first_price,
                        max_price,
                        min_price,
                        last_price,
                        chunk_volume
                    )?;
                } else {
                    // 块跨越窗口边界，逐个处理
                    for j in start..end {
                        self.process_trade(timestamps[j], prices[j], volumes[j])?;
                    }
                    break; // 已处理完整个块，跳出循环
                }
            }
        }

        // 处理剩余部分
        if remainder > 0 {
            let start = chunks * 8;
            for j in start..len {
                self.process_trade(timestamps[j], prices[j], volumes[j])?;
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn calculate_sliding_stats_with_simd(&self, highs: &[f64], lows: &[f64], volumes: &[f64]) -> (f64, f64, f64) {
        let len = highs.len();
        let chunks = len / 8;
        let remainder = len % 8;

        let mut max_high = f64::MIN;
        let mut min_low = f64::MAX;
        let mut total_volume = 0.0;

        // 处理完整的 8 元素块
        for i in 0..chunks {
            let start = i * 8;
            let end = start + 8;

            let high_vec = f64x8::from_slice(&highs[start..end]);
            let low_vec = f64x8::from_slice(&lows[start..end]);
            let volume_vec = f64x8::from_slice(&volumes[start..end]);

            max_high = max_high.max(high_vec.reduce_max());
            min_low = min_low.min(low_vec.reduce_min());
            total_volume += volume_vec.reduce_sum();
        }

        // 处理剩余部分
        if remainder > 0 {
            let start = chunks * 8;
            for j in start..len {
                max_high = max_high.max(highs[j]);
                min_low = min_low.min(lows[j]);
                total_volume += volumes[j];
            }
        }

        (max_high, min_low, total_volume)
    }


    #[inline(always)]
    fn update_window_with_stats(
        &self, window_idx: usize, window_start: u64, open: f64, high: f64, low: f64, close: f64, volume: f64
    ) -> Result<(), String> {
        let window = match window_idx {
            0 => TimeWindow::Second,
            1 => TimeWindow::Minute,
            2 => TimeWindow::FifteenMin,
            3 => TimeWindow::Hour,
            _ => return Err("Invalid window index".to_string())
        };

        let mut current_windows = self.current_windows.borrow_mut();
        let need_update_sliding = match &mut current_windows[window_idx] {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                // 更新当前窗口的统计信息（使用 SIMD 计算的结果）
                current_ohlc.high = current_ohlc.high.max(high);
                current_ohlc.low = current_ohlc.low.min(low);
                current_ohlc.close = close;
                current_ohlc.volume += volume;
                current_ohlc.count += 8; // 处理了8个交易
                true
            }
            _ => {
                // 保存旧窗口到历史
                if let Some(old) = current_windows[window_idx].take() {
                    self.save_to_history(window_idx, old.clone());
                    self.send_event(window, old, true);
                }

                // 创建新的当前窗口（使用 SIMD 计算的结果）
                let mut new_ohlc = OHLC::new(window_start, open, volume);
                new_ohlc.high = high;
                new_ohlc.low = low;
                new_ohlc.close = close;
                new_ohlc.count = 8; // 处理了8个交易
                current_windows[window_idx] = Some(new_ohlc);
                true
            }
        };

        if need_update_sliding {
            let current_ohlc = current_windows[window_idx].unwrap();
            self.update_sliding_window(window_idx, current_ohlc);
        }

        Ok(())
    }
}

impl KLineAgg for SingleThreadSimdKLineAggregator {
    fn new() -> Self { Self::new_with_batch_size(1000) }

    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + 'static
    {
        let mut handlers = self.event_handlers.borrow_mut();
        handlers.push(Box::new(handler));
    }

    fn process_trade(&self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        let mut total_trades = self.total_trades.borrow_mut();
        *total_trades += 1;

        let mut total_volume = self.total_volume.borrow_mut();
        *total_volume += volume as u64;

        self.update_window(0, timestamp, price, volume)?; // 1s
        self.update_window(1, timestamp, price, volume)?; // 1m
        self.update_window(2, timestamp, price, volume)?; // 15m
        self.update_window(3, timestamp, price, volume)?; // 1h

        Ok(())
    }

    fn process_trades_batch(&self, trades: &[(u64, f64, f64)]) -> Result<(), String> {
        self.process_batch_internal(trades)
    }

    fn get_current_ohlc(&self, window: TimeWindow) -> Option<OHLC> {
        let idx = window.index();
        self.current_windows.borrow()[idx].clone()
    }

    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> {
        let history = match window {
            TimeWindow::Second => &self.history_1s,
            TimeWindow::Minute => &self.history_1m,
            TimeWindow::FifteenMin => &self.history_15m,
            TimeWindow::Hour => &self.history_1h
        };

        let history = history.borrow();
        let len = history.len();
        let start = if len > limit { len - limit } else { 0 };

        history.iter().skip(start).cloned().collect()
    }

    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) {
        let sliding = match window {
            TimeWindow::Second => &self.sliding_1s,
            TimeWindow::Minute => &self.sliding_1m,
            TimeWindow::FifteenMin => &self.sliding_15m,
            TimeWindow::Hour => &self.sliding_1h
        };

        let sliding = sliding.borrow();
        let len = sliding.len().min(period);

        if len == 0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }

        // 收集所需的 OHLC 数据
        let mut highs = Vec::with_capacity(len);
        let mut lows = Vec::with_capacity(len);
        let mut volumes = Vec::with_capacity(len);

        // 第一个元素是最近的K线（用于开盘价）
        let first = sliding.back().unwrap();
        let open = first.open;

        highs.push(first.high);
        lows.push(first.low);
        volumes.push(first.volume);

        // 收集剩余元素
        for i in (sliding.len() - len)..(sliding.len() - 1) {
            if let Some(ohlc) = sliding.get(i) {
                highs.push(ohlc.high);
                lows.push(ohlc.low);
                volumes.push(ohlc.volume);
            }
        }

        // 使用 SIMD 优化计算统计信息
        let (high, low, total_volume) = self.calculate_sliding_stats_with_simd(&highs, &lows, &volumes);

        // 最后一个元素是最远的K线（用于收盘价）
        let close = sliding.get(sliding.len() - len).unwrap().close;

        (open, high, low, close, total_volume)
    }

    fn get_total_stats(&self) -> (u64, u64) {
        (*self.total_trades.borrow(), *self.total_volume.borrow())
    }
}