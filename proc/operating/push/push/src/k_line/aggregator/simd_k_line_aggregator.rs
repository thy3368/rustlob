use std::{
    simd::{f64x8, num::SimdFloat},
    sync::{
        atomic::{AtomicU64, Ordering},
        RwLock
    }
};

use crate::k_line::k_line_types::{KLineAgg, KLineUpdateEvent, TimeWindow, OHLC, LockFreeRingBuffer};

// SIMD 优化的 K 线聚合器
pub struct SimdKLineAggregator {
    // 当前活跃窗口 [1s, 1m, 15m, 1h]
    current_windows: [RwLock<Option<OHLC>>; 4],

    // 历史K线存储（无锁环形缓冲区）
    history_1s: LockFreeRingBuffer<OHLC>,  // 存储最近3600秒
    history_1m: LockFreeRingBuffer<OHLC>,  // 存储最近1440分钟
    history_15m: LockFreeRingBuffer<OHLC>, // 存储最近672个15分钟
    history_1h: LockFreeRingBuffer<OHLC>,  // 存储最近720小时

    // 滑动窗口统计（无锁环形缓冲区）
    sliding_1s: LockFreeRingBuffer<OHLC>,  // 最近60秒
    sliding_1m: LockFreeRingBuffer<OHLC>,  // 最近60分钟
    sliding_15m: LockFreeRingBuffer<OHLC>, // 最近96个15分钟
    sliding_1h: LockFreeRingBuffer<OHLC>,  // 最近168小时

    // 原子计数器
    total_trades: AtomicU64,
    total_volume: AtomicU64,

    // 预计算的窗口大小
    window_sizes: [u64; 4],
    // 预计算的历史容量
    history_capacities: [usize; 4],
    // 预计算的滑动窗口容量
    sliding_capacities: [usize; 4],

    // 事件处理器列表
    event_handlers: RwLock<Vec<Box<dyn Fn(KLineUpdateEvent) + Send + Sync>>>,

    // 批处理缓冲区
    batch_buffer: Vec<(u64, f64, f64)>,
    batch_size: usize
}

impl SimdKLineAggregator {
    pub fn new_with_batch_size(batch_size: usize) -> Self {
        Self {
            current_windows: [RwLock::new(None), RwLock::new(None), RwLock::new(None), RwLock::new(None)],
            history_1s: LockFreeRingBuffer::new(3600),
            history_1m: LockFreeRingBuffer::new(1440),
            history_15m: LockFreeRingBuffer::new(672),
            history_1h: LockFreeRingBuffer::new(720),
            sliding_1s: LockFreeRingBuffer::new(60),
            sliding_1m: LockFreeRingBuffer::new(60),
            sliding_15m: LockFreeRingBuffer::new(96),
            sliding_1h: LockFreeRingBuffer::new(168),
            total_trades: AtomicU64::new(0),
            total_volume: AtomicU64::new(0),
            window_sizes: [1, 60, 900, 3600],
            history_capacities: [3600, 1440, 672, 720],
            sliding_capacities: [60, 60, 96, 168],
            event_handlers: RwLock::new(Vec::new()),
            batch_buffer: Vec::with_capacity(batch_size),
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

        let handlers = self.event_handlers.read().unwrap();
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

        let mut current_lock = self.current_windows[window_idx].write().unwrap();

        match &mut *current_lock {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                // 更新当前窗口
                current_ohlc.update(price, volume);

                // 更新滑动窗口
                self.update_sliding_window(window_idx, *current_ohlc);
            }
            _ => {
                // 保存旧窗口到历史
                if let Some(old) = current_lock.take() {
                    self.save_to_history(window_idx, old.clone());
                    self.send_event(window, old, true);
                }

                // 创建新的当前窗口
                let new_ohlc = OHLC::new(window_start, price, volume);
                *current_lock = Some(new_ohlc);

                // 更新滑动窗口
                self.update_sliding_window(window_idx, new_ohlc);
            }
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

        let capacity = self.sliding_capacities[window_idx];

        // 检查是否已存在相同时间戳的K线，避免重复添加
        if let Some(last) = sliding_window.back() {
            if last.timestamp == ohlc.timestamp {
                // 如果已存在，更新最后一个K线（因为可能有价格/成交量变化）
                return; // 暂时简单处理，不添加重复时间戳的K线
            }
        }

        // 检查容量并删除最旧的K线
        if sliding_window.len() >= capacity {
            sliding_window.pop();
        }

        // 添加新的K线
        sliding_window.push(ohlc);
    }

    fn save_to_history(&self, window_idx: usize, ohlc: OHLC) {
        let history = match window_idx {
            0 => &self.history_1s,
            1 => &self.history_1m,
            2 => &self.history_15m,
            3 => &self.history_1h,
            _ => return
        };

        let capacity = self.history_capacities[window_idx];

        if history.len() >= capacity {
            history.pop();
        }
        history.push(ohlc);
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

        // 预分配缓冲区用于 SIMD 处理
        let mut prices = Vec::with_capacity(trades.len());
        let mut volumes = Vec::with_capacity(trades.len());
        let mut timestamps = Vec::with_capacity(trades.len());

        for &(ts, p, v) in trades {
            timestamps.push(ts);
            prices.push(p);
            volumes.push(v);
        }

        // 使用 SIMD 优化滑动窗口统计和 OHLC 计算
        self.process_with_simd(&timestamps, &prices, &volumes)?;

        Ok(())
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

        let mut current_lock = self.current_windows[window_idx].write().unwrap();

        match &mut *current_lock {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                // 更新当前窗口的统计信息（使用 SIMD 计算的结果）
                current_ohlc.high = current_ohlc.high.max(high);
                current_ohlc.low = current_ohlc.low.min(low);
                current_ohlc.close = close;
                current_ohlc.volume += volume;
                current_ohlc.count += 8; // 处理了8个交易

                // 更新滑动窗口
                self.update_sliding_window(window_idx, *current_ohlc);
            }
            _ => {
                // 保存旧窗口到历史
                if let Some(old) = current_lock.take() {
                    self.save_to_history(window_idx, old.clone());
                    self.send_event(window, old, true);
                }

                // 创建新的当前窗口（使用 SIMD 计算的结果）
                let mut new_ohlc = OHLC::new(window_start, open, volume);
                new_ohlc.high = high;
                new_ohlc.low = low;
                new_ohlc.close = close;
                new_ohlc.count = 8; // 处理了8个交易
                *current_lock = Some(new_ohlc);

                // 更新滑动窗口
                self.update_sliding_window(window_idx, new_ohlc);
            }
        }

        Ok(())
    }
}

impl KLineAgg for SimdKLineAggregator {
    fn new() -> Self { Self::new_with_batch_size(1000) }

    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + Send + Sync + 'static
    {
        let mut handlers = self.event_handlers.write().unwrap();
        handlers.push(Box::new(handler));
    }


    fn process_trade(&self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        self.total_trades.fetch_add(1, Ordering::Relaxed);
        self.total_volume.fetch_add(volume as u64, Ordering::Relaxed);

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
        self.current_windows[idx].read().unwrap().clone()
    }

    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> {
        let history = match window {
            TimeWindow::Second => &self.history_1s,
            TimeWindow::Minute => &self.history_1m,
            TimeWindow::FifteenMin => &self.history_15m,
            TimeWindow::Hour => &self.history_1h
        };

        let len = history.len();
        let start = if len > limit { len - limit } else { 0 };

        history.iter().skip(start).collect()
    }

    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) {
        let sliding = match window {
            TimeWindow::Second => &self.sliding_1s,
            TimeWindow::Minute => &self.sliding_1m,
            TimeWindow::FifteenMin => &self.sliding_15m,
            TimeWindow::Hour => &self.sliding_1h
        };

        let len = sliding.len().min(period);

        if len == 0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }

        // 收集所需的 OHLC 数据
        let mut highs = Vec::with_capacity(len);
        let mut lows = Vec::with_capacity(len);
        let mut volumes = Vec::with_capacity(len);

        // 第一个元素是最近的K线（用于开盘价）
        let first = sliding.get(sliding.len() - 1).unwrap();
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
        (self.total_trades.load(Ordering::Relaxed), self.total_volume.load(Ordering::Relaxed))
    }
}
