use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicU64, Ordering},
        RwLock
    }
};

use crate::k_line::k_line_types::{TimeWindow, OHLC, KLineUpdateEvent};
use diff::{Entity, ChangeLogEntry};

pub struct KLineAggregator {
    // 当前活跃窗口 [1s, 1m, 15m, 1h]
    current_windows: [RwLock<Option<OHLC>>; 4],

    // 历史K线存储（环形缓冲区）
    history_1s: RwLock<VecDeque<OHLC>>,  // 存储最近3600秒
    history_1m: RwLock<VecDeque<OHLC>>,  // 存储最近1440分钟
    history_15m: RwLock<VecDeque<OHLC>>, // 存储最近672个15分钟
    history_1h: RwLock<VecDeque<OHLC>>,  // 存储最近720小时

    // 滑动窗口统计
    sliding_1s: RwLock<VecDeque<OHLC>>,  // 最近60秒
    sliding_1m: RwLock<VecDeque<OHLC>>,  // 最近60分钟
    sliding_15m: RwLock<VecDeque<OHLC>>, // 最近96个15分钟
    sliding_1h: RwLock<VecDeque<OHLC>>,  // 最近168小时

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
}

impl KLineAggregator {
    pub fn new() -> Self {
        Self {
            current_windows: [
                RwLock::new(None), // 1s
                RwLock::new(None), // 1m
                RwLock::new(None), // 15m
                RwLock::new(None)  // 1h
            ],

            history_1s: RwLock::new(VecDeque::with_capacity(3600)),
            history_1m: RwLock::new(VecDeque::with_capacity(1440)),
            history_15m: RwLock::new(VecDeque::with_capacity(672)),
            history_1h: RwLock::new(VecDeque::with_capacity(720)),

            sliding_1s: RwLock::new(VecDeque::with_capacity(60)),
            sliding_1m: RwLock::new(VecDeque::with_capacity(60)),
            sliding_15m: RwLock::new(VecDeque::with_capacity(96)),
            sliding_1h: RwLock::new(VecDeque::with_capacity(168)),

            total_trades: AtomicU64::new(0),
            total_volume: AtomicU64::new(0),

            window_sizes: [1, 60, 900, 3600],
            history_capacities: [3600, 1440, 672, 720],
            sliding_capacities: [60, 60, 96, 168],

            event_handlers: RwLock::new(Vec::new()),
        }
    }

    // 订阅K线更新事件
    pub fn subscribe<F>(&self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + Send + Sync + 'static,
    {
        let mut handlers = self.event_handlers.write().unwrap();
        handlers.push(Box::new(handler));
    }

    // 发送K线更新事件
    fn send_event(&self, window: TimeWindow, ohlc: OHLC, is_new_window: bool) {
        let event = KLineUpdateEvent {
            window,
            ohlc,
            is_new_window,
        };

        let handlers = self.event_handlers.read().unwrap();
        for handler in handlers.iter() {
            handler(event.clone());
        }
    }

    // O(1) 复杂度处理单笔成交
    pub fn process_trade(&self, timestamp: u64, price: f64, volume: f64) -> Result<(), String> {
        // 原子计数器
        self.total_trades.fetch_add(1, Ordering::Relaxed);
        self.total_volume.fetch_add(volume as u64, Ordering::Relaxed);

        // 并行更新所有时间窗口
        self.update_window(0, timestamp, price, volume)?; // 1s
        self.update_window(1, timestamp, price, volume)?; // 1m
        self.update_window(2, timestamp, price, volume)?; // 15m
        self.update_window(3, timestamp, price, volume)?; // 1h

        Ok(())
    }

    // 批量处理成交（优化版）
    pub fn process_trades_batch(&self, trades: &[(u64, f64, f64)]) -> Result<(), String> {
        let len = trades.len();

        // 批量预处理
        for chunk in trades.chunks(1000) {
            for &(timestamp, price, volume) in chunk {
                self.process_trade(timestamp, price, volume)?;
            }
        }

        Ok(())
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
            _ => return Err("Invalid window index".to_string()),
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
                    self.save_to_history(window_idx, old);
                }

                // 创建新的当前窗口
                let new_ohlc = OHLC::new(window_start, price, volume);
                *current_lock = Some(new_ohlc);

                // 更新滑动窗口
                self.update_sliding_window(window_idx, new_ohlc);

                // 触发窗口切换事件！
                self.send_event(window, new_ohlc, true);
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

        let mut lock = sliding_window.write().unwrap();
        let capacity = self.sliding_capacities[window_idx];

        // 检查是否已存在相同时间戳的K线，避免重复添加
        if let Some(last) = lock.back() {
            if last.timestamp == ohlc.timestamp {
                // 如果已存在，更新最后一个K线（因为可能有价格/成交量变化）
                lock.pop_back();
                lock.push_back(ohlc);
                return;
            }
        }

        // 检查容量并删除最旧的K线
        if lock.len() >= capacity {
            lock.pop_front();
        }

        // 添加新的K线
        lock.push_back(ohlc);
    }

    fn save_to_history(&self, window_idx: usize, ohlc: OHLC) {
        let history = match window_idx {
            0 => &self.history_1s,
            1 => &self.history_1m,
            2 => &self.history_15m,
            3 => &self.history_1h,
            _ => return
        };

        let mut lock = history.write().unwrap();
        let capacity = self.history_capacities[window_idx];

        if lock.len() >= capacity {
            lock.pop_front();
        }
        lock.push_back(ohlc);
    }

    // 获取当前K线
    pub fn get_current_ohlc(&self, window: TimeWindow) -> Option<OHLC> {
        let idx = window.index();
        self.current_windows[idx].read().unwrap().clone()
    }

    // 获取历史K线
    pub fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC> {
        let history = match window {
            TimeWindow::Second => &self.history_1s,
            TimeWindow::Minute => &self.history_1m,
            TimeWindow::FifteenMin => &self.history_15m,
            TimeWindow::Hour => &self.history_1h
        };

        let lock = history.read().unwrap();
        let start = if lock.len() > limit { lock.len() - limit } else { 0 };

        lock.range(start..).cloned().collect()
    }

    // 获取滑动窗口统计
    pub fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64) {
        let sliding = match window {
            TimeWindow::Second => &self.sliding_1s,
            TimeWindow::Minute => &self.sliding_1m,
            TimeWindow::FifteenMin => &self.sliding_15m,
            TimeWindow::Hour => &self.sliding_1h
        };

        let lock = sliding.read().unwrap();
        let len = lock.len().min(period);

        if len == 0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }

        let mut high = f64::MIN;
        let mut low = f64::MAX;
        let mut total_volume = 0.0;

        for ohlc in lock.iter().rev().take(len) {
            high = high.max(ohlc.high);
            low = low.min(ohlc.low);
            total_volume += ohlc.volume;
        }

        let first = lock.back().unwrap();
        let last = lock.front().unwrap();

        (first.open, high, low, last.close, total_volume)
    }

    pub fn get_total_stats(&self) -> (u64, u64) {
        (self.total_trades.load(Ordering::Relaxed), self.total_volume.load(Ordering::Relaxed))
    }
}
