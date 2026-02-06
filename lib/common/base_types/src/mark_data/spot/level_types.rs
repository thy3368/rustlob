use crate::base_types::TraderId;
use crate::{Decimal, OrderId, OrderSide, Price, Quantity};

/// 市场数据等级定义（Level 1-3）
///
/// 遵循金融行业标准的市场数据分级：
/// - Level 1: 最佳买卖价（BBO）和最新成交
/// - Level 2: 市场深度（多档价格，不含订单详情）
/// - Level 3: 完整订单簿（包含所有订单详情）

// ============================================================================
// 类型别名
// ============================================================================

/// 交易对ID类型别名
///
/// 使用 u32 类型表示交易对ID，支持最多 4,294,967,295 个交易对
pub type SymbolId = u32;

/// 序列号类型别名
///
/// 用于标识事件的顺序，确保数据一致性
pub type SequenceNumber = u64;

/// Level 1 市场数据 - 顶层报价（Top of Book）
///
/// 包含最基础的市场信息，适合零售交易者和显示报价
///
/// 缓存行对齐确保高性能访问
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub struct Level1 {
    /// 交易对ID
    pub symbol_id: u32,
    /// 快照时间戳（纳秒）
    pub timestamp: u64,
    /// 序列号（用于检测丢包和排序）
    pub sequence: u64,
    /// 最佳买价（Bid）
    pub best_bid: Option<Price>,
    /// 最佳买价总数量
    pub best_bid_quantity: Quantity,
    /// 最佳卖价（Ask/Offer）
    pub best_ask: Option<Price>,
    /// 最佳卖价总数量
    pub best_ask_quantity: Quantity,
    /// 最新成交价
    pub last_trade_price: Option<Price>,
    /// 最新成交数量
    pub last_trade_quantity: Quantity,
    /// 买卖价差（Spread）
    pub spread: Option<Price>,
    /// 中间价（Mid Price）
    pub mid_price: Option<Price>,
}

impl Default for Level1 {
    fn default() -> Self {
        Self {
            symbol_id: 0,
            timestamp: 0,
            sequence: 0,
            best_bid: None,
            best_bid_quantity: Quantity::default(),
            best_ask: None,
            best_ask_quantity: Quantity::default(),
            last_trade_price: None,
            last_trade_quantity: Quantity::default(),
            spread: None,
            mid_price: None,
        }
    }
}

impl Level1 {
    /// 创建新的 Level 1 数据快照
    #[inline]
    pub fn new(
        symbol_id: u32,
        timestamp: u64,
        sequence: u64,
        best_bid: Option<Price>,
        best_bid_quantity: Quantity,
        best_ask: Option<Price>,
        best_ask_quantity: Quantity,
    ) -> Self {
        let spread = match (best_ask, best_bid) {
            (Some(ask), Some(bid)) if ask > bid => Some(ask - bid),
            _ => None,
        };

        let mid_price = match (best_ask, best_bid) {
            (Some(ask), Some(bid)) => Some((ask + bid) / Decimal::from_raw(200000000)),
            _ => None,
        };

        Self {
            symbol_id,
            timestamp,
            sequence,
            best_bid,
            best_bid_quantity,
            best_ask,
            best_ask_quantity,
            last_trade_price: None,
            last_trade_quantity: Quantity::default(),
            spread,
            mid_price,
        }
    }

    /// 更新最新成交信息
    #[inline]
    pub fn update_last_trade(&mut self, price: Price, quantity: Quantity) {
        self.last_trade_price = Some(price);
        self.last_trade_quantity = quantity;
    }

    /// 检查是否有有效的买卖价
    #[inline]
    pub fn has_valid_market(&self) -> bool {
        self.best_bid.is_some() && self.best_ask.is_some()
    }
}

/// Level 2 价格档位
///
/// 表示单个价格档位的聚合信息
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PriceLevel {
    /// 价格
    pub price: Price,
    /// 该价格的总数量
    pub quantity: Quantity,
    /// 该价格的订单数量
    pub order_count: u32,
}

impl PriceLevel {
    /// 创建新的价格档位
    #[inline]
    pub fn new(price: Price, quantity: Quantity, order_count: u32) -> Self {
        Self { price, quantity, order_count }
    }
}

/// Level 2 市场数据 - 市场深度（Market Depth）
///
/// 显示多档价格的聚合订单数量，不包含单个订单详情
/// 适合专业交易者分析市场流动性
///
/// 使用固定大小数组避免堆分配，提升性能
#[derive(Debug, Clone)]
pub struct Level2<const MAX_LEVELS: usize = 10> {
    /// 买方价格档位（从高到低排序）
    pub bids: [Option<PriceLevel>; MAX_LEVELS],
    /// 买方有效档位数
    pub bid_count: usize,
    /// 卖方价格档位（从低到高排序）
    pub asks: [Option<PriceLevel>; MAX_LEVELS],
    /// 卖方有效档位数
    pub ask_count: usize,
    /// 包含 Level 1 数据
    pub level1: Level1,
}

impl<const MAX_LEVELS: usize> Default for Level2<MAX_LEVELS> {
    fn default() -> Self {
        Self {
            bids: [None; MAX_LEVELS],
            bid_count: 0,
            asks: [None; MAX_LEVELS],
            ask_count: 0,
            level1: Level1::default(),
        }
    }
}

impl<const MAX_LEVELS: usize> Level2<MAX_LEVELS> {
    /// 创建新的 Level 2 数据
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加买方价格档位
    #[inline]
    pub fn add_bid(&mut self, level: PriceLevel) -> Result<(), &'static str> {
        if self.bid_count >= MAX_LEVELS {
            return Err("Bid levels full");
        }
        self.bids[self.bid_count] = Some(level);
        self.bid_count += 1;
        Ok(())
    }

    /// 添加卖方价格档位
    #[inline]
    pub fn add_ask(&mut self, level: PriceLevel) -> Result<(), &'static str> {
        if self.ask_count >= MAX_LEVELS {
            return Err("Ask levels full");
        }
        self.asks[self.ask_count] = Some(level);
        self.ask_count += 1;
        Ok(())
    }

    /// 计算买方总深度（所有档位总数量）
    #[inline]
    pub fn total_bid_quantity(&self) -> Quantity {
        self.bids[..self.bid_count].iter().filter_map(|&level| level.map(|l| l.quantity)).sum()
    }

    /// 计算卖方总深度（所有档位总数量）
    #[inline]
    pub fn total_ask_quantity(&self) -> Quantity {
        self.asks[..self.ask_count].iter().filter_map(|&level| level.map(|l| l.quantity)).sum()
    }

    /// 清空所有档位
    #[inline]
    pub fn clear(&mut self) {
        self.bid_count = 0;
        self.ask_count = 0;
        self.level1 = Level1::default();
    }
}

/// Level 3 订单条目
///
/// 包含单个订单的完整信息
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Level3Order {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易员ID
    pub trader_id: TraderId,
    /// 价格
    pub price: Price,
    /// 数量
    pub quantity: Quantity,
    /// 未成交数量
    pub unfilled_quantity: Quantity,
}

impl Level3Order {
    /// 创建新的 Level 3 订单
    #[inline]
    pub fn new(
        order_id: OrderId,
        trader_id: TraderId,
        price: Price,
        quantity: Quantity,
        unfilled_quantity: Quantity,
    ) -> Self {
        Self { order_id, trader_id, price, quantity, unfilled_quantity }
    }

    /// 检查订单是否仍然活跃
    #[inline]
    pub fn is_active(&self) -> bool {
        self.unfilled_quantity > Quantity::default()
    }
}

/// Level 3 市场数据 - 完整订单簿（Full Order Book）
///
/// 包含所有订单的完整详细信息，适合高频交易和做市商
/// 提供最细粒度的市场视图
///
/// 使用 Vec 存储可变数量的订单（可根据需要优化为固定大小数组+对象池）
#[derive(Debug, Clone)]
pub struct Level3 {
    /// 所有买单（按价格-时间优先排序）
    pub bids: Vec<Level3Order>,
    /// 所有卖单（按价格-时间优先排序）
    pub asks: Vec<Level3Order>,
    /// 包含 Level 2 数据（可聚合生成）
    pub level2: Level2<10>,
}

impl Default for Level3 {
    fn default() -> Self {
        Self { bids: Vec::new(), asks: Vec::new(), level2: Level2::default() }
    }
}

impl Level3 {
    /// 创建新的 Level 3 数据
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 预分配容量（避免运行时扩容）
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bids: Vec::with_capacity(capacity),
            asks: Vec::with_capacity(capacity),
            level2: Level2::default(),
        }
    }

    /// 添加买单
    #[inline]
    pub fn add_bid(&mut self, order: Level3Order) {
        self.bids.push(order);
    }

    /// 添加卖单
    #[inline]
    pub fn add_ask(&mut self, order: Level3Order) {
        self.asks.push(order);
    }

    /// 获取指定订单ID的订单
    #[inline]
    pub fn find_order(&self, order_id: OrderId) -> Option<&Level3Order> {
        self.bids.iter().chain(self.asks.iter()).find(|order| order.order_id == order_id)
    }

    /// 移除指定订单
    #[inline]
    pub fn remove_order(&mut self, order_id: OrderId) -> bool {
        if let Some(pos) = self.bids.iter().position(|o| o.order_id == order_id) {
            self.bids.remove(pos);
            return true;
        }
        if let Some(pos) = self.asks.iter().position(|o| o.order_id == order_id) {
            self.asks.remove(pos);
            return true;
        }
        false
    }

    /// 清空所有订单
    #[inline]
    pub fn clear(&mut self) {
        self.bids.clear();
        self.asks.clear();
        self.level2.clear();
    }

    /// 统计活跃订单数
    #[inline]
    pub fn active_order_count(&self) -> usize {
        self.bids.iter().filter(|o| o.is_active()).count()
            + self.asks.iter().filter(|o| o.is_active()).count()
    }
}

// ============================================================================
// 增量数据事件定义
// ============================================================================

/// 订单簿变更类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderChangeType {
    /// 新增订单
    Add,
    /// 修改订单（数量变化）
    Modify,
    /// 删除订单
    Delete,
}

/// 订单簿增量变更事件
#[derive(Debug, Clone, Copy)]
pub struct OrderDelta {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 事件时间戳（纳秒）
    pub timestamp: u64,
    /// 序列号
    pub sequence: u64,
    /// 变更类型
    pub change_type: OrderChangeType,
    /// 订单ID
    pub order_id: OrderId,
    /// 买卖方向
    pub side: OrderSide,
    /// 价格
    pub price: Price,
    /// 数量（新数量或变化量）
    pub quantity: Quantity,
    /// 交易者ID（可选）
    pub trader_id: Option<TraderId>,
}

/// 成交事件
#[derive(Debug, Clone, Copy)]
pub struct TradeEvent {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 成交时间戳（纳秒）
    pub timestamp: u64,
    /// 序列号
    pub sequence: u64,
    /// 成交ID
    pub trade_id: u64,
    /// 买方订单ID
    pub buyer_order_id: OrderId,
    /// 卖方订单ID
    pub seller_order_id: OrderId,
    /// 成交价格
    pub price: Price,
    /// 成交数量
    pub quantity: Quantity,
    /// 主动方（买方或卖方）
    pub aggressor_side: OrderSide,
}

/// 最优买卖价变更事件
#[derive(Debug, Clone, Copy)]
pub struct BboChangeEvent {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 事件时间戳（纳秒）
    pub timestamp: u64,
    /// 序列号
    pub sequence: u64,
    /// 最优买价
    pub best_bid: Option<Price>,
    /// 最优买价数量
    pub best_bid_quantity: Quantity,
    /// 最优卖价
    pub best_ask: Option<Price>,
    /// 最优卖价数量
    pub best_ask_quantity: Quantity,
}

/// 市场数据增量事件（统一枚举）
#[derive(Debug, Clone, Copy)]
pub enum MarketDataDelta {
    /// 订单簿变更
    OrderChange(OrderDelta),
    /// 成交事件
    Trade(TradeEvent),
    /// 最优买卖价变更
    BboChange(BboChangeEvent),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level1_creation() {
        let level1 = Level1::new(
            1,                            // symbol_id
            1000,                         // timestamp
            1,                            // sequence
            Some(Price::from_raw(50000)), // best_bid
            Quantity::from_raw(100),      // best_bid_quantity
            Some(Price::from_raw(50100)), // best_ask
            Quantity::from_raw(200),      // best_ask_quantity
        );

        assert_eq!(level1.symbol_id, 1);
        assert_eq!(level1.best_bid, Some(Price::from_raw(50000)));
        assert_eq!(level1.best_ask, Some(Price::from_raw(50100)));
        assert_eq!(level1.spread, Some(Price::from_raw(100)));
        assert_eq!(level1.mid_price, Some(Price::from_raw(50050)));
        assert!(level1.has_valid_market());
    }
}
