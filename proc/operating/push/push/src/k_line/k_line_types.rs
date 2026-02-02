use std::sync::atomic::{AtomicUsize, Ordering};

use diff::{Entity, FromCreatedEvent};
use entity_derive::Entity;

// 缓存行对齐类型，用于高性能无锁编程
#[repr(align(64))]
pub struct CacheAligned<T> {
    pub value: T
}

impl<T> CacheAligned<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        CacheAligned {
            value
        }
    }
}

// K 线数据结构
#[derive(Debug, Clone, Copy, Entity, Default)]
#[entity(id = "timestamp", type_name = "OHLC")]
pub struct OHLC {
    pub open: f64,      // 开盘价
    pub high: f64,      // 最高价
    pub low: f64,       // 最低价
    pub close: f64,     // 收盘价
    pub volume: f64,    // 成交量
    pub timestamp: u64, // 窗口开始时间（秒）
    pub count: u32      // 成交笔数
}

impl OHLC {
    pub fn new(timestamp: u64, price: f64, volume: f64) -> Self {
        Self {
            open: price,
            high: price,
            low: price,
            close: price,
            volume,
            timestamp,
            count: 1
        }
    }

    pub fn update(&mut self, price: f64, volume: f64) {
        self.high = self.high.max(price);
        self.low = self.low.min(price);
        self.close = price;
        self.volume += volume;
        self.count += 1;
    }

    fn merge(&mut self, other: &OHLC) {
        self.high = self.high.max(other.high);
        self.low = self.low.min(other.low);
        self.close = other.close;
        self.volume += other.volume;
        self.count += other.count;
    }
}

// 时间窗口定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeWindow {
    Second,     // 1秒
    Minute,     // 1分钟
    FifteenMin, // 15分钟
    Hour        // 1小时
}

impl TimeWindow {
    pub fn duration_seconds(&self) -> u64 {
        match self {
            TimeWindow::Second => 1,
            TimeWindow::Minute => 60,
            TimeWindow::FifteenMin => 900,
            TimeWindow::Hour => 3600
        }
    }

    pub fn index(&self) -> usize {
        match self {
            TimeWindow::Second => 0,
            TimeWindow::Minute => 1,
            TimeWindow::FifteenMin => 2,
            TimeWindow::Hour => 3
        }
    }
}

// 高性能无锁环形缓冲区
pub struct LockFreeRingBuffer<T: Copy> {
    buffer: Vec<T>,
    capacity: usize,
    head: AtomicUsize, // 写位置
    tail: AtomicUsize, // 读位置
    mask: usize
}

impl<T: Copy + Default> LockFreeRingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.next_power_of_two();
        let mut buffer = Vec::with_capacity(cap);
        unsafe {
            buffer.set_len(cap);
        }

        Self {
            buffer,
            capacity: cap,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            mask: cap - 1
        }
    }

    #[inline(always)]
    pub fn push(&self, value: T) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        if head.wrapping_sub(tail) >= self.capacity {
            return false; // 缓冲区满
        }

        unsafe {
            let ptr = self.buffer.as_ptr().add(head & self.mask) as *mut T;
            std::ptr::write(ptr, value);
        }

        self.head.store(head.wrapping_add(1), Ordering::Release);
        true
    }

    #[inline(always)]
    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if tail >= head {
            return None; // 缓冲区空
        }

        let value = unsafe {
            let ptr = self.buffer.as_ptr().add(tail & self.mask);
            std::ptr::read(ptr)
        };

        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Some(value)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<T> {
        let len = self.len();
        if index >= len {
            return None;
        }

        let tail = self.tail.load(Ordering::Acquire);
        let actual_index = (tail + index) & self.mask;
        unsafe { Some(std::ptr::read(self.buffer.as_ptr().add(actual_index))) }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        let len = self.len();
        (0..len).filter_map(move |i| self.get(i))
    }

    #[inline(always)]
    pub fn back(&self) -> Option<T> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            self.get(len - 1)
        }
    }

    #[inline(always)]
    pub fn front(&self) -> Option<T> {
        self.get(0)
    }
}


// K线更新事件（用于内部通知）
#[derive(Debug, Clone)]
pub struct KLineUpdateEvent {
    pub window: TimeWindow,
    pub ohlc: OHLC,
    pub is_new_window: bool
}

pub trait KLineAgg {
    fn new() -> Self;
    // 订阅K线更新事件
    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(KLineUpdateEvent) + Send + Sync + 'static;
    // O(1) 复杂度处理单笔成交
    fn process_trade(&self, timestamp: u64, price: f64, volume: f64) -> Result<(), String>;
    // 批量处理成交（优化版）
    fn process_trades_batch(&self, trades: &[(u64, f64, f64)]) -> Result<(), String>;
    // 获取当前K线
    fn get_current_ohlc(&self, window: TimeWindow) -> Option<OHLC>;
    // 获取历史K线
    fn get_history_ohlc(&self, window: TimeWindow, limit: usize) -> Vec<OHLC>;
    // 获取滑动窗口统计
    fn get_sliding_stats(&self, window: TimeWindow, period: usize) -> (f64, f64, f64, f64, f64);
    fn get_total_stats(&self) -> (u64, u64);
}
