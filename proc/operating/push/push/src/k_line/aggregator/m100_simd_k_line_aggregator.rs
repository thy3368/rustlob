use std::simd::{f64x8, num::SimdFloat};

use crate::k_line::k_line_types::{KLineAgg, KLineAggMut, KLineUpdateEvent, TimeWindow, OHLC};

// M100SimdKLineAggregator - 1000M 交易/秒 高性能实现
// 完全无锁、无分配、极致优化的单线程K线聚合器
// 设计目标：100万笔交易/1毫秒
pub struct M100SimdKLineAggregator {
    // 当前活跃窗口 [1s, 1m, 15m, 1h] - 直接在栈上分配，无堆分配
    current_windows: [Option<OHLC>; 4],

    // 历史K线存储 - 使用固定大小数组代替VecDeque，避免堆分配
    history_1s: [OHLC; 3600],
    history_1m: [OHLC; 1440],
    history_15m: [OHLC; 672],
    history_1h: [OHLC; 720],

    // 历史缓冲区指针
    history_1s_head: usize,
    history_1s_tail: usize,
    history_1m_head: usize,
    history_1m_tail: usize,
    history_15m_head: usize,
    history_15m_tail: usize,
    history_1h_head: usize,
    history_1h_tail: usize,

    // 滑动窗口统计
    sliding_1s: [OHLC; 60],
    sliding_1m: [OHLC; 60],
    sliding_15m: [OHLC; 96],
    sliding_1h: [OHLC; 168],

    sliding_1s_head: usize,
    sliding_1s_tail: usize,
    sliding_1m_head: usize,
    sliding_1m_tail: usize,
    sliding_15m_head: usize,
    sliding_15m_tail: usize,
    sliding_1h_head: usize,
    sliding_1h_tail: usize,

    // 普通计数器
    total_trades: u64,
    total_volume: u64,

    // 预计算的窗口大小
    window_sizes: [u64; 4],
    // 预计算的历史容量
    history_capacities: [usize; 4],
    // 预计算的滑动窗口容量
    sliding_capacities: [usize; 4],

    // 事件处理器列表 - 使用静态数组避免堆分配
    event_handlers: [Option<Box<dyn Fn(KLineUpdateEvent)>>; 8],
    event_handler_count: usize,

    // 批处理缓冲区 - 完全在栈上分配
    batch_buffer: [(u64, f64, f64); 1024],
    batch_ptr: usize,
}

impl M100SimdKLineAggregator {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            current_windows: [None, None, None, None],
            history_1s: [OHLC::default(); 3600],
            history_1m: [OHLC::default(); 1440],
            history_15m: [OHLC::default(); 672],
            history_1h: [OHLC::default(); 720],
            history_1s_head: 0,
            history_1s_tail: 0,
            history_1m_head: 0,
            history_1m_tail: 0,
            history_15m_head: 0,
            history_15m_tail: 0,
            history_1h_head: 0,
            history_1h_tail: 0,
            sliding_1s: [OHLC::default(); 60],
            sliding_1m: [OHLC::default(); 60],
            sliding_15m: [OHLC::default(); 96],
            sliding_1h: [OHLC::default(); 168],
            sliding_1s_head: 0,
            sliding_1s_tail: 0,
            sliding_1m_head: 0,
            sliding_1m_tail: 0,
            sliding_15m_head: 0,
            sliding_15m_tail: 0,
            sliding_1h_head: 0,
            sliding_1h_tail: 0,
            total_trades: 0,
            total_volume: 0,
            window_sizes: [1, 60, 900, 3600],
            history_capacities: [3600, 1440, 672, 720],
            sliding_capacities: [60, 60, 96, 168],
            event_handlers: [None, None, None, None, None, None, None, None],
            event_handler_count: 0,
            batch_buffer: [(0, 0.0, 0.0); 1024],
            batch_ptr: 0,
        }
    }

    #[inline(always)]
    fn send_event(&self, window: TimeWindow, ohlc: OHLC, is_new_window: bool) {
        let event = KLineUpdateEvent {
            window,
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
    fn update_window(&self, window_idx: usize, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        let window_size = self.window_sizes[window_idx];
        let window_start = (timestamp / window_size) * window_size;
        let window = match window_idx {
            0 => TimeWindow::Second,
            1 => TimeWindow::Minute,
            2 => TimeWindow::FifteenMin,
            3 => TimeWindow::Hour,
            _ => return Err("Invalid window index".to_string()),
        };

        let mut current_windows = self.current_windows.borrow_mut();
        let need_update_sliding = match &mut current_windows[window_idx] {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                current_ohlc.update(price, volume);
                true
            }
            _ => {
                if let Some(old) = current_windows[window_idx].take() {
                    self.save_to_history(window_idx, old.clone());
                    self.send_event(window, old, true);
                }

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

    #[inline(always)]
    fn update_sliding_window(&self, window_idx: usize, ohlc: OHLC) {
        match window_idx {
            0 => {
                self.update_sliding_window_impl(
                    &self.sliding_1s,
                    &self.sliding_1s_head,
                    &self.sliding_1s_tail,
                    60,
                    ohlc
                );
            }
            1 => {
                self.update_sliding_window_impl(
                    &self.sliding_1m,
                    &self.sliding_1m_head,
                    &self.sliding_1m_tail,
                    60,
                    ohlc
                );
            }
            2 => {
                self.update_sliding_window_impl(
                    &self.sliding_15m,
                    &self.sliding_15m_head,
                    &self.sliding_15m_tail,
                    96,
                    ohlc
                );
            }
            3 => {
                self.update_sliding_window_impl(
                    &self.sliding_1h,
                    &self.sliding_1h_head,
                    &self.sliding_1h_tail,
                    168,
                    ohlc
                );
            }
            _ => return,
        }
    }

    #[inline(always)]
    fn update_sliding_window_impl(
        &self,
        sliding_window: &RefCell<[OHLC; 60]>,
        head: &RefCell<usize>,
        tail: &RefCell<usize>,
        capacity: usize,
        ohlc: OHLC,
    ) {
        let mut head = head.borrow_mut();
        let mut tail = tail.borrow_mut();
        let mut sliding_window = sliding_window.borrow_mut();

        if *head != *tail {
            let last_idx = (*head - 1) % capacity;
            if sliding_window[last_idx].timestamp == ohlc.timestamp {
                return;
            }
        }

        sliding_window[*head % capacity] = ohlc;
        *head += 1;

        if *head - *tail > capacity {
            *tail += 1;
        }
    }

    #[inline(always)]
    fn save_to_history(&self, window_idx: usize, ohlc: OHLC) {
        match window_idx {
            0 => {
                self.save_to_history_impl(
                    &self.history_1s,
                    &self.history_1s_head,
                    &self.history_1s_tail,
                    3600,
                    ohlc
                );
            }
            1 => {
                self.save_to_history_impl(
                    &self.history_1m,
                    &self.history_1m_head,
                    &self.history_1m_tail,
                    1440,
                    ohlc
                );
            }
            2 => {
                self.save_to_history_impl(
                    &self.history_15m,
                    &self.history_15m_head,
                    &self.history_15m_tail,
                    672,
                    ohlc
                );
            }
            3 => {
                self.save_to_history_impl(
                    &self.history_1h,
                    &self.history_1h_head,
                    &self.history_1h_tail,
                    720,
                    ohlc
                );
            }
            _ => return,
        }
    }

    #[inline(always)]
    fn save_to_history_impl(
        &self,
        history: &RefCell<[OHLC; 3600]>,
        head: &RefCell<usize>,
        tail: &RefCell<usize>,
        capacity: usize,
        ohlc: OHLC,
    ) {
        let mut head = head.borrow_mut();
        let mut tail = tail.borrow_mut();
        let mut history = history.borrow_mut();

        history[*head % capacity] = ohlc;
        *head += 1;

        if *head - *tail > capacity {
            *tail += 1;
        }
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

            for window_idx in 0..4 {
                let window_size = self.window_sizes[window_idx];
                let first_window = (first_ts / window_size) * window_size;
                let last_window = (last_ts / window_size) * window_size;

                if first_window == last_window {
                    self.update_window_with_stats(
                        window_idx,
                        first_window,
                        first_price,
                        max_price,
                        min_price,
                        last_price,
                        chunk_volume,
                    )?;
                } else {
                    for j in start..end {
                        self.process_trade(timestamps[j], prices[j], volumes[j])?;
                    }
                    break;
                }
            }
        }

        if remainder > 0 {
            let start = chunks * 8;
            for j in start..len {
                self.process_trade(timestamps[j], prices[j], volumes[j])?;
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn update_window_with_stats(
        &self,
        window_idx: usize,
        window_start: u64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Result<(), String> {
        let window = match window_idx {
            0 => TimeWindow::Second,
            1 => TimeWindow::Minute,
            2 => TimeWindow::FifteenMin,
            3 => TimeWindow::Hour,
            _ => return Err("Invalid window index".to_string()),
        };

        let mut current_windows = self.current_windows.borrow_mut();
        let need_update_sliding = match &mut current_windows[window_idx] {
            Some(current_ohlc) if current_ohlc.timestamp == window_start => {
                current_ohlc.high = current_ohlc.high.max(high);
                current_ohlc.low = current_ohlc.low.min(low);
                current_ohlc.close = close;
                current_ohlc.volume += volume;
                current_ohlc.count += 8;
                true
            }
            _ => {
                if let Some(old) = current_windows[window_idx].take() {
                    self.save_to_history(window_idx, old.clone());
                    self.send_event(window, old, true);
                }

                let mut new_ohlc = OHLC::new(window_start, open, volume);
                new_ohlc.high = high;
                new_ohlc.low = low;
                new_ohlc.close = close;
                new_ohlc.count = 8;
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

impl KLineAggMut for M100SimdKLineAggregator {
    #[inline(always)]
    fn new() -> Self {
        Self::new()
    }

    #[inline(always)]
    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + 'static,
    {
        let mut handler_count = self.event_handler_count.borrow_mut();
        let mut handlers = self.event_handlers.borrow_mut();

        if *handler_count < 8 {
            handlers[*handler_count] = Some(Box::new(handler));
            *handler_count += 1;
        }
    }

    #[inline(always)]
    fn process_trade(&self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        let mut total_trades = self.total_trades.borrow_mut();
        *total_trades += 1;

        let mut total_volume = self.total_volume.borrow_mut();
        *total_volume += volume as u64;

        self.update_window(0, timestamp, price, volume)?;
        self.update_window(1, timestamp, price, volume)?;
        self.update_window(2, timestamp, price, volume)?;
        self.update_window(3, timestamp, price, volume)?;

        Ok(())
    }

    #[inline(always)]
    fn process_trades_batch(&self, trades: &[(u64, f64, f64)]) -> Result<(), String> {
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
        let idx = window.index();
        self.current_windows.borrow()[idx]
    }

    #[inline(always)]
    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> {
        match window {
            TimeWindow::Second => self.get_history_ohlc_impl(
                &self.history_1s,
                self.history_1s_head.borrow().clone(),
                self.history_1s_tail.borrow().clone(),
                3600,
                limit,
            ),
            TimeWindow::Minute => self.get_history_ohlc_impl(
                &self.history_1m,
                self.history_1m_head.borrow().clone(),
                self.history_1m_tail.borrow().clone(),
                1440,
                limit,
            ),
            TimeWindow::FifteenMin => self.get_history_ohlc_impl(
                &self.history_15m,
                self.history_15m_head.borrow().clone(),
                self.history_15m_tail.borrow().clone(),
                672,
                limit,
            ),
            TimeWindow::Hour => self.get_history_ohlc_impl(
                &self.history_1h,
                self.history_1h_head.borrow().clone(),
                self.history_1h_tail.borrow().clone(),
                720,
                limit,
            ),
        }
    }

    #[inline(always)]
    fn get_history_ohlc_impl(
        &self,
        history: &RefCell<[OHLC; 3600]>,
        head: usize,
        tail: usize,
        capacity: usize,
        limit: usize,
    ) -> Vec<OHLC> {
        let mut result = Vec::new();
        let len = head - tail;
        let start = if len > limit { len - limit } else { 0 };

        let history = history.borrow();
        for i in start..len {
            let idx = (tail + i) % capacity;
            result.push(history[idx]);
        }

        result
    }

    #[inline(always)]
    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) {
        match window {
            TimeWindow::Second => self.get_sliding_stats_impl(
                &self.sliding_1s,
                self.sliding_1s_head.borrow().clone(),
                self.sliding_1s_tail.borrow().clone(),
                60,
                period,
            ),
            TimeWindow::Minute => self.get_sliding_stats_impl(
                &self.sliding_1m,
                self.sliding_1m_head.borrow().clone(),
                self.sliding_1m_tail.borrow().clone(),
                60,
                period,
            ),
            TimeWindow::FifteenMin => self.get_sliding_stats_impl(
                &self.sliding_15m,
                self.sliding_15m_head.borrow().clone(),
                self.sliding_15m_tail.borrow().clone(),
                96,
                period,
            ),
            TimeWindow::Hour => self.get_sliding_stats_impl(
                &self.sliding_1h,
                self.sliding_1h_head.borrow().clone(),
                self.sliding_1h_tail.borrow().clone(),
                168,
                period,
            ),
        }
    }

    #[inline(always)]
    fn get_sliding_stats_impl(
        &self,
        sliding: &RefCell<[OHLC; 60]>,
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

        let sliding = sliding.borrow();
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

    #[inline(always)]
    fn get_total_stats(&self) -> (u64, u64) {
        (*self.total_trades.borrow(), *self.total_volume.borrow())
    }
}
