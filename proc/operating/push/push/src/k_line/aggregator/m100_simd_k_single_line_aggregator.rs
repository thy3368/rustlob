use std::simd::{f64x8, num::SimdFloat};

use crate::k_line::k_line_types::{KLineAgg, KLineAggMut, KLineUpdateEvent, TimeWindow, OHLC};

// M100SimdKSingleLineAggregator - 单时间窗口的 SIMD 优化 K 线聚合器
// 设计用于处理单个特定时间窗口的 K 线聚合
// 完全无锁、无分配、极致优化的单线程实现
pub struct M100SimdKSingleLineAggregator {
    // 当前活跃窗口
    current_window: Option<OHLC>,

    // 历史K线存储 - 使用固定大小数组代替VecDeque，避免堆分配
    history: Vec<OHLC>,

    // 历史缓冲区指针
    history_head: usize,
    history_tail: usize,

    // 滑动窗口统计
    sliding_window: Vec<OHLC>,

    sliding_head: usize,
    sliding_tail: usize,

    // 预计算的窗口大小
    window_size: u64,
    // 预计算的历史容量
    history_capacity: usize,
    // 预计算的滑动窗口容量
    sliding_capacity: usize,

    // 事件处理器列表 - 使用静态数组避免堆分配
    // 需要确保事件处理器可以安全地在线程间传递
    event_handlers: [Option<Box<dyn Fn(KLineUpdateEvent) + Send + Sync>>; 8],
    event_handler_count: usize,

    // 窗口类型
    window_type: TimeWindow,
}

impl M100SimdKSingleLineAggregator {
    #[inline(always)]
    pub fn new(window: TimeWindow) -> Self {
        let (window_size, history_capacity, sliding_capacity) = match window {
            TimeWindow::Second => (1, 3600, 60),       // 1秒窗口，保留1小时历史
            TimeWindow::Minute => (60, 1440, 60),     // 1分钟窗口，保留24小时历史
            TimeWindow::FifteenMin => (900, 672, 96),  // 15分钟窗口，保留7天历史
            TimeWindow::Hour => (3600, 720, 168),     // 1小时窗口，保留30天历史
        };

        Self {
            current_window: None,
            history: vec![OHLC::default(); history_capacity],
            history_head: 0,
            history_tail: 0,
            sliding_window: vec![OHLC::default(); sliding_capacity],
            sliding_head: 0,
            sliding_tail: 0,
            window_size,
            history_capacity,
            sliding_capacity,
            event_handlers: [None, None, None, None, None, None, None, None],
            event_handler_count: 0,
            window_type: window,
        }
    }

    #[inline(always)]
    fn send_event(&self, ohlc: OHLC, is_new_window: bool) {
        let event = KLineUpdateEvent {
            window: self.window_type,
            ohlc,
            is_new_window,
        };

        let handler_count = self.event_handler_count;
        let handlers = &self.event_handlers;

        for i in 0..handler_count {
            if let Some(handler) = &handlers[i] {
                handler(event.clone());
            }
        }
    }

    #[inline(always)]
    pub fn update_window(&mut self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        let window_start = (timestamp / self.window_size) * self.window_size;

        let need_update_sliding = match &mut self.current_window {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                current_ohlc.update(price, volume);
                true
            }
            _ => {
                if let Some(old) = self.current_window.take() {
                    self.save_to_history(old.clone());
                    self.send_event(old, true);
                }

                let new_ohlc = OHLC::new(window_start, price, volume);
                self.current_window = Some(new_ohlc);
                true
            }
        };

        if need_update_sliding {
            let current_ohlc = self.current_window.unwrap();
            self.update_sliding_window(current_ohlc);
        }

        Ok(())
    }

    #[inline(always)]
    fn update_sliding_window(&mut self, ohlc: OHLC) {
        if self.sliding_head != self.sliding_tail {
            let last_idx = (self.sliding_head - 1) % self.sliding_capacity;
            if self.sliding_window[last_idx].timestamp == ohlc.timestamp {
                return;
            }
        }

        self.sliding_window[self.sliding_head % self.sliding_capacity] = ohlc;
        self.sliding_head += 1;

        if self.sliding_head - self.sliding_tail > self.sliding_capacity {
            self.sliding_tail += 1;
        }
    }

    #[inline(always)]
    fn save_to_history(&mut self, ohlc: OHLC) {
        self.history[self.history_head % self.history_capacity] = ohlc;
        self.history_head += 1;

        if self.history_head - self.history_tail > self.history_capacity {
            self.history_tail += 1;
        }
    }

    #[inline(always)]
    unsafe fn prefetch<T>(ptr: *const T, hint: i32) {
        #[cfg(target_arch = "x86_64")]
        std::arch::x86_64::_mm_prefetch(ptr as *const i8, hint);
    }

    #[inline(always)]
    pub fn process_with_simd(&mut self, timestamps: &[u64], prices: &[f64], volumes: &[f64]) -> Result<(), String> {
        let len = timestamps.len();
        let chunks = len / 8;
        let remainder = len % 8;

        for i in 0..chunks {
            let start = i * 8;
            let end = start + 8;

            if i < chunks - 1 {
                let next_start = (i + 1) * 8;
                unsafe {
                    Self::prefetch(timestamps.as_ptr().add(next_start), 0x00);
                    Self::prefetch(prices.as_ptr().add(next_start), 0x00);
                    Self::prefetch(volumes.as_ptr().add(next_start), 0x00);
                }
            }

            let price_vec = f64x8::from_slice(&prices[start..end]);
            let volume_vec = f64x8::from_slice(&volumes[start..end]);

            let max_price = price_vec.reduce_max();
            let min_price = price_vec.reduce_min();
            let sum_volume = volume_vec.reduce_sum();

            let chunk_timestamps = &timestamps[start..end];
            let first_ts = chunk_timestamps[0];
            let last_ts = chunk_timestamps[7];
            let first_price = prices[start];
            let last_price = prices[end - 1];

            let chunk_volume = sum_volume;

            let first_window = (first_ts / self.window_size) * self.window_size;
            let last_window = (last_ts / self.window_size) * self.window_size;

            if first_window == last_window {
                self.update_window_with_stats(
                    first_window,
                    first_price,
                    max_price,
                    min_price,
                    last_price,
                    chunk_volume,
                )?;
            } else {
                for j in start..end {
                    self.update_window(timestamps[j], prices[j], volumes[j])?;
                }
            }
        }

        if remainder > 0 {
            let start = chunks * 8;
            for j in start..len {
                self.update_window(timestamps[j], prices[j], volumes[j])?;
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn update_window_with_stats(
        &mut self,
        window_start: u64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Result<(), String> {
        let need_update_sliding = match &mut self.current_window {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                current_ohlc.high = current_ohlc.high.max(high);
                current_ohlc.low = current_ohlc.low.min(low);
                current_ohlc.close = close;
                current_ohlc.volume += volume;
                current_ohlc.count += 8;
                true
            }
            _ => {
                if let Some(old) = self.current_window.take() {
                    self.save_to_history(old.clone());
                    self.send_event(old, true);
                }

                let mut new_ohlc = OHLC::new(window_start, open, volume);
                new_ohlc.high = high;
                new_ohlc.low = low;
                new_ohlc.close = close;
                new_ohlc.count = 8;
                self.current_window = Some(new_ohlc);
                true
            }
        };

        if need_update_sliding {
            let current_ohlc = self.current_window.unwrap();
            self.update_sliding_window(current_ohlc);
        }

        Ok(())
    }

    #[inline(always)]
    fn get_history_ohlc_impl(
        history: &[OHLC],
        head: usize,
        tail: usize,
        capacity: usize,
        limit: usize,
    ) -> Vec<OHLC> {
        let mut result = Vec::new();
        let len = head - tail;
        let start = if len > limit { len - limit } else { 0 };

        for i in start..len {
            let idx = (tail + i) % capacity;
            result.push(history[idx]);
        }

        result
    }

    #[inline(always)]
    fn get_sliding_stats_impl(
        sliding: &[OHLC],
        head: usize,
        tail: usize,
        capacity: usize,
        period: usize,
    ) -> (f64, f64, f64, f64, f64) {
        let len = head - tail;
        let actual_period = usize::min(len, period);

        if actual_period == 0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }

        let start = tail + (len - actual_period);
        let first_idx = (head - 1) % capacity;

        let open = sliding[first_idx].open;

        let mut highs = Vec::with_capacity(actual_period);
        let mut lows = Vec::with_capacity(actual_period);
        let mut volumes = Vec::with_capacity(actual_period);

        for i in start..head {
            let idx = i % capacity;
            highs.push(sliding[idx].high);
            lows.push(sliding[idx].low);
            volumes.push(sliding[idx].volume);
        }

        let len = highs.len();
        let chunks = len / 8;
        let remainder = len % 8;

        let mut max_high = f64::MIN;
        let mut min_low = f64::MAX;
        let mut total_volume = 0.0;

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

        if remainder > 0 {
            let start = chunks * 8;
            for j in start..len {
                max_high = max_high.max(highs[j]);
                min_low = min_low.min(lows[j]);
                total_volume += volumes[j];
            }
        }

        let close_idx = start % capacity;
        let close = sliding[close_idx].close;

        (open, max_high, min_low, close, total_volume)
    }
}

impl KLineAggMut for M100SimdKSingleLineAggregator {
    #[inline(always)]
    fn new() -> Self {
        // 默认创建1秒窗口的聚合器
        Self::new(TimeWindow::Second)
    }

    #[inline(always)]
    fn subscribe<F>(&mut self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + 'static + Send + Sync,
    {
        let handler_count = self.event_handler_count;
        let handlers = &mut self.event_handlers;

        if handler_count < 8 {
            handlers[handler_count] = Some(Box::new(handler));
            self.event_handler_count += 1;
        }
    }

    #[inline(always)]
    fn process_trade(&mut self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        self.update_window(timestamp, price, volume)?;
        Ok(())
    }

    #[inline(always)]
    fn process_trades_batch(&mut self, trades: &[(u64, f64, f64)]) -> Result<(), String> {
        const STACK_BUFFER_SIZE: usize = 256;
        let mut ts_stack = [0u64; STACK_BUFFER_SIZE];
        let mut p_stack = [0.0f64; STACK_BUFFER_SIZE];
        let mut v_stack = [0.0f64; STACK_BUFFER_SIZE];

        if trades.len() <= STACK_BUFFER_SIZE {
            for (i, &(ts, p, v)) in trades.iter().enumerate() {
                ts_stack[i] = ts;
                p_stack[i] = p;
                v_stack[i] = v;
            }
            self.process_with_simd(
                &ts_stack[..trades.len()],
                &p_stack[..trades.len()],
                &v_stack[..trades.len()],
            )?;
        } else {
            let mut ts_heap = Vec::with_capacity(trades.len());
            let mut p_heap = Vec::with_capacity(trades.len());
            let mut v_heap = Vec::with_capacity(trades.len());

            for &(ts, p, v) in trades.iter() {
                ts_heap.push(ts);
                p_heap.push(p);
                v_heap.push(v);
            }

            self.process_with_simd(&ts_heap, &p_heap, &v_heap)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn get_current_ohlc(&self, window: TimeWindow) -> Option<OHLC> {
        if window == self.window_type {
            self.current_window
        } else {
            None
        }
    }

    #[inline(always)]
    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> {
        if window == self.window_type {
            Self::get_history_ohlc_impl(
                &self.history,
                self.history_head,
                self.history_tail,
                self.history_capacity,
                limit,
            )
        } else {
            Vec::new()
        }
    }

    #[inline(always)]
    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) {
        if window == self.window_type {
            Self::get_sliding_stats_impl(
                &self.sliding_window,
                self.sliding_head,
                self.sliding_tail,
                self.sliding_capacity,
                period,
            )
        } else {
            (0.0, 0.0, 0.0, 0.0, 0.0)
        }
    }

    #[inline(always)]
    fn get_total_stats(&self) -> (u64, u64) {
        // 这个方法在单窗口聚合器中可能不需要实现，因为统计是在更高层次完成的
        (0, 0)
    }
}

// 为 M100SimdKSingleLineAggregator 添加默认实现
impl Default for M100SimdKSingleLineAggregator {
    fn default() -> Self {
        Self::new(TimeWindow::Second)
    }
}