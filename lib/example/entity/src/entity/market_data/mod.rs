use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 现货 K 线聚合支持的时间周期。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KlineInterval {
    /// 1 分钟。
    OneMinute,
    /// 5 分钟。
    FiveMinutes,
    /// 15 分钟。
    FifteenMinutes,
    /// 1 小时。
    OneHour,
    /// 4 小时。
    FourHours,
    /// 1 天。
    OneDay,
}

impl KlineInterval {
    /// 兼容常见的短周期命名。
    pub const M1: Self = Self::OneMinute;
    /// 兼容常见的短周期命名。
    pub const M5: Self = Self::FiveMinutes;
    /// 兼容常见的短周期命名。
    pub const M15: Self = Self::FifteenMinutes;
    /// 兼容常见的短周期命名。
    pub const H1: Self = Self::OneHour;
    /// 兼容常见的短周期命名。
    pub const H4: Self = Self::FourHours;
    /// 兼容常见的短周期命名。
    pub const D1: Self = Self::OneDay;

    /// 返回周期长度，单位为毫秒。
    pub const fn duration_ms(self) -> u64 {
        match self {
            Self::OneMinute => 60_000,
            Self::FiveMinutes => 5 * 60_000,
            Self::FifteenMinutes => 15 * 60_000,
            Self::OneHour => 60 * 60_000,
            Self::FourHours => 4 * 60 * 60_000,
            Self::OneDay => 24 * 60 * 60_000,
        }
    }

    /// `duration_ms` 的语义别名。
    pub const fn interval_ms(self) -> u64 {
        self.duration_ms()
    }

    /// 返回适合行情协议的周期名称。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneMinute => "1m",
            Self::FiveMinutes => "5m",
            Self::FifteenMinutes => "15m",
            Self::OneHour => "1h",
            Self::FourHours => "4h",
            Self::OneDay => "1d",
        }
    }
}

/// 从成交事实中抽取的 K 线聚合输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotKlineTradeInput {
    /// 交易对。
    pub symbol: String,
    /// 成交价格，保持系统内部定点整数。
    pub price: u64,
    /// 成交数量，保持系统内部定点整数。
    pub qty: u64,
    /// 成交业务时间，单位为 Unix epoch 毫秒。
    pub executed_at_ms: u64,
}

/// 一个交易对在一个固定时间窗口内的 K 线快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KlineCandle {
    /// 交易对。
    pub symbol: String,
    /// K 线周期。
    pub interval: KlineInterval,
    /// 窗口起点，单位为 Unix epoch 毫秒。
    pub window_start_ms: u64,
    /// 首笔成交价。
    pub open: u64,
    /// 窗口内最高成交价。
    pub high: u64,
    /// 窗口内最低成交价。
    pub low: u64,
    /// 最新成交价。
    pub close: u64,
    /// 窗口内成交数量之和。
    pub volume: u64,
    /// 窗口内成交笔数。
    pub num_trades: u64,
}

/// 一笔成交对 K 线读模型产生的更新。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KlineUpdate {
    /// 当前窗口仍在接收成交。
    Updated(KlineCandle),
    /// 当前窗口已经被新窗口替代，可以持久化或广播。
    Closed(KlineCandle),
}

/// 由现货成交事实驱动的内存 K 线聚合器。
///
/// 第一版假定成交按 `executed_at_ms` 基本有序到达，不处理迟到成交、
/// 历史窗口回补或重复事件去重。
#[derive(Debug, Clone)]
pub struct SpotKlineAggregator {
    intervals: Vec<KlineInterval>,
    candles: HashMap<(String, KlineInterval), KlineCandle>,
}

impl Default for SpotKlineAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// K 线聚合过程中发生的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum KlineAggregationError {
    /// 交易对不能为空。
    #[error("kline symbol must not be empty")]
    EmptySymbol,
    /// 成交价格必须大于零。
    #[error("kline trade price must be greater than zero")]
    ZeroPrice,
    /// 成交数量必须大于零。
    #[error("kline trade quantity must be greater than zero")]
    ZeroQuantity,
    /// 成交时间必须大于零。
    #[error("kline executed_at_ms must be greater than zero")]
    ZeroExecutedAt,
    /// K 线成交量或成交笔数累加发生溢出。
    #[error("kline aggregate arithmetic overflow")]
    ArithmeticOverflow,
}

impl SpotKlineAggregator {
    /// 创建包含六个标准现货周期的聚合器。
    pub fn new() -> Self {
        Self::with_intervals([
            KlineInterval::OneMinute,
            KlineInterval::FiveMinutes,
            KlineInterval::FifteenMinutes,
            KlineInterval::OneHour,
            KlineInterval::FourHours,
            KlineInterval::OneDay,
        ])
    }

    /// 创建只维护指定周期的聚合器。
    pub fn with_intervals<I>(intervals: I) -> Self
    where
        I: IntoIterator<Item = KlineInterval>,
    {
        let mut unique_intervals = Vec::new();
        for interval in intervals {
            if !unique_intervals.contains(&interval) {
                unique_intervals.push(interval);
            }
        }
        Self { intervals: unique_intervals, candles: HashMap::new() }
    }

    /// 返回当前聚合器维护的周期。
    pub fn intervals(&self) -> &[KlineInterval] {
        &self.intervals
    }

    /// 将一笔成交应用到所有已配置周期，并按周期顺序返回更新。
    pub fn update(
        &mut self,
        symbol: impl Into<String>,
        price: u64,
        qty: u64,
        executed_at_ms: u64,
    ) -> Result<Vec<KlineUpdate>, KlineAggregationError> {
        let trade = SpotKlineTradeInput { symbol: symbol.into(), price, qty, executed_at_ms };
        self.update_trade(&trade)
    }

    /// 将一笔已解析的成交输入应用到所有已配置周期。
    pub fn update_trade(
        &mut self,
        trade: &SpotKlineTradeInput,
    ) -> Result<Vec<KlineUpdate>, KlineAggregationError> {
        validate_trade_input(trade)?;

        let mut updates = Vec::with_capacity(self.intervals.len().saturating_mul(2));
        for &interval in &self.intervals {
            let interval_ms = interval.duration_ms();
            let window_start_ms = (trade.executed_at_ms / interval_ms) * interval_ms;
            let key = (trade.symbol.clone(), interval);

            match self.candles.get_mut(&key) {
                Some(candle) if candle.window_start_ms == window_start_ms => {
                    candle.high = candle.high.max(trade.price);
                    candle.low = candle.low.min(trade.price);
                    candle.close = trade.price;
                    candle.volume = candle
                        .volume
                        .checked_add(trade.qty)
                        .ok_or(KlineAggregationError::ArithmeticOverflow)?;
                    candle.num_trades = candle
                        .num_trades
                        .checked_add(1)
                        .ok_or(KlineAggregationError::ArithmeticOverflow)?;
                    updates.push(KlineUpdate::Updated(candle.clone()));
                }
                _ => {
                    if let Some(previous) = self.candles.remove(&key) {
                        updates.push(KlineUpdate::Closed(previous));
                    }

                    let candle = KlineCandle {
                        symbol: trade.symbol.clone(),
                        interval,
                        window_start_ms,
                        open: trade.price,
                        high: trade.price,
                        low: trade.price,
                        close: trade.price,
                        volume: trade.qty,
                        num_trades: 1,
                    };
                    self.candles.insert(key, candle.clone());
                    updates.push(KlineUpdate::Updated(candle));
                }
            }
        }
        Ok(updates)
    }

    /// `update_trade` 的语义别名，便于事件投影入口表达“摄取成交”。
    pub fn ingest_trade(
        &mut self,
        trade: &SpotKlineTradeInput,
    ) -> Result<Vec<KlineUpdate>, KlineAggregationError> {
        self.update_trade(trade)
    }

    /// 返回交易对和周期当前所在窗口的快照。
    pub fn current_candle(&self, symbol: &str, interval: KlineInterval) -> Option<&KlineCandle> {
        self.candles.get(&(symbol.to_string(), interval))
    }

    /// 返回当前维护的窗口数量。
    pub fn len(&self) -> usize {
        self.candles.len()
    }

    /// 判断当前是否没有任何窗口。
    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }
}

fn validate_trade_input(trade: &SpotKlineTradeInput) -> Result<(), KlineAggregationError> {
    if trade.symbol.is_empty() {
        return Err(KlineAggregationError::EmptySymbol);
    }
    if trade.price == 0 {
        return Err(KlineAggregationError::ZeroPrice);
    }
    if trade.qty == 0 {
        return Err(KlineAggregationError::ZeroQuantity);
    }
    if trade.executed_at_ms == 0 {
        return Err(KlineAggregationError::ZeroExecutedAt);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_standard_intervals_create_a_candle_from_the_first_trade() {
        let mut aggregator = SpotKlineAggregator::new();
        let updates = aggregator.update("BTCUSDT", 100, 2, 1_717_171_717_000).unwrap();

        assert_eq!(updates.len(), 6);
        assert!(updates.iter().all(|update| matches!(update, KlineUpdate::Updated(_))));
        assert_eq!(aggregator.len(), 6);
    }

    #[test]
    fn same_window_updates_ohlcv_and_trade_count() {
        let mut aggregator = SpotKlineAggregator::with_intervals([KlineInterval::OneMinute]);
        aggregator.update("BTCUSDT", 100, 2, 60_000).unwrap();
        let updates = aggregator.update("BTCUSDT", 110, 3, 61_000).unwrap();

        assert_eq!(
            updates,
            vec![KlineUpdate::Updated(KlineCandle {
                symbol: "BTCUSDT".to_string(),
                interval: KlineInterval::OneMinute,
                window_start_ms: 60_000,
                open: 100,
                high: 110,
                low: 100,
                close: 110,
                volume: 5,
                num_trades: 2,
            })]
        );
    }

    #[test]
    fn moving_to_a_new_window_closes_old_before_updating_new() {
        let mut aggregator = SpotKlineAggregator::with_intervals([KlineInterval::OneMinute]);
        aggregator.update("BTCUSDT", 100, 2, 60_000).unwrap();
        let updates = aggregator.update("BTCUSDT", 120, 1, 120_000).unwrap();

        assert!(matches!(updates[0], KlineUpdate::Closed(_)));
        assert!(matches!(updates[1], KlineUpdate::Updated(_)));
        assert_eq!(
            aggregator.current_candle("BTCUSDT", KlineInterval::OneMinute).unwrap().open,
            120
        );
    }

    #[test]
    fn symbols_have_independent_windows() {
        let mut aggregator = SpotKlineAggregator::with_intervals([KlineInterval::OneMinute]);
        aggregator.update("BTCUSDT", 100, 2, 60_000).unwrap();
        aggregator.update("ETHUSDT", 200, 3, 60_000).unwrap();

        assert_eq!(
            aggregator.current_candle("BTCUSDT", KlineInterval::OneMinute).unwrap().volume,
            2
        );
        assert_eq!(
            aggregator.current_candle("ETHUSDT", KlineInterval::OneMinute).unwrap().volume,
            3
        );
    }

    #[test]
    fn integer_values_are_not_converted_to_floating_point() {
        let mut aggregator = SpotKlineAggregator::with_intervals([KlineInterval::OneMinute]);
        let updates = aggregator.update("BTCUSDT", u64::MAX - 1, 2, 60_000).unwrap();
        let KlineUpdate::Updated(candle) = &updates[0] else {
            panic!("first trade must update the active window");
        };

        assert_eq!(candle.open, u64::MAX - 1);
        assert_eq!(candle.volume, 2);
    }

    #[test]
    fn invalid_trade_inputs_return_explicit_errors() {
        let mut aggregator = SpotKlineAggregator::new();

        assert_eq!(aggregator.update("", 100, 1, 1), Err(KlineAggregationError::EmptySymbol));
        assert_eq!(aggregator.update("BTCUSDT", 0, 1, 1), Err(KlineAggregationError::ZeroPrice));
        assert_eq!(
            aggregator.update("BTCUSDT", 100, 0, 1),
            Err(KlineAggregationError::ZeroQuantity)
        );
        assert_eq!(
            aggregator.update("BTCUSDT", 100, 1, 0),
            Err(KlineAggregationError::ZeroExecutedAt)
        );
    }
}
