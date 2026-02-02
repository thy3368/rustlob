use diff::{Entity, FromCreatedEvent};
use entity_derive::Entity;

// K 线数据结构
#[derive(Debug, Clone, Copy, Entity)]
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

// K线更新事件（用于内部通知）
#[derive(Debug, Clone)]
pub struct KLineUpdateEvent {
    pub window: TimeWindow,
    pub ohlc: OHLC,
    pub is_new_window: bool,
}
