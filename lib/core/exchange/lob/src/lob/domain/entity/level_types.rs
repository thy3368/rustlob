use crate::lob::domain::entity::lob_types::{OrderId, Price, Quantity, TraderId};

/// 市场数据等级定义（Level 1-3）
///
/// 遵循金融行业标准的市场数据分级：
/// - Level 1: 最佳买卖价（BBO）和最新成交
/// - Level 2: 市场深度（多档价格，不含订单详情）
/// - Level 3: 完整订单簿（包含所有订单详情）

// use super::types::{OrderId, Price, Quantity, TraderId};

/// Level 1 市场数据 - 顶层报价（Top of Book）
///
/// 包含最基础的市场信息，适合零售交易者和显示报价
///
/// 缓存行对齐确保高性能访问
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub struct Level1 {
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
            best_bid: None,
            best_bid_quantity: 0,
            best_ask: None,
            best_ask_quantity: 0,
            last_trade_price: None,
            last_trade_quantity: 0,
            spread: None,
            mid_price: None,
        }
    }
}

impl Level1 {
    /// 创建新的 Level 1 数据
    #[inline]
    pub fn new(
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
            (Some(ask), Some(bid)) => Some((ask + bid) / 2),
            _ => None,
        };

        Self {
            best_bid,
            best_bid_quantity,
            best_ask,
            best_ask_quantity,
            last_trade_price: None,
            last_trade_quantity: 0,
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
        Self {
            price,
            quantity,
            order_count,
        }
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
        self.bids[..self.bid_count]
            .iter()
            .filter_map(|&level| level.map(|l| l.quantity))
            .sum()
    }

    /// 计算卖方总深度（所有档位总数量）
    #[inline]
    pub fn total_ask_quantity(&self) -> Quantity {
        self.asks[..self.ask_count]
            .iter()
            .filter_map(|&level| level.map(|l| l.quantity))
            .sum()
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
        Self {
            order_id,
            trader_id,
            price,
            quantity,
            unfilled_quantity,
        }
    }

    /// 检查订单是否仍然活跃
    #[inline]
    pub fn is_active(&self) -> bool {
        self.unfilled_quantity > 0
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
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            level2: Level2::default(),
        }
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
        self.bids
            .iter()
            .chain(self.asks.iter())
            .find(|order| order.order_id == order_id)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level1_creation() {
        let l1 = Level1::new(Some(9900), 100, Some(10100), 200);

        assert_eq!(l1.best_bid, Some(9900));
        assert_eq!(l1.best_bid_quantity, 100);
        assert_eq!(l1.best_ask, Some(10100));
        assert_eq!(l1.best_ask_quantity, 200);
        assert_eq!(l1.spread, Some(200));
        assert_eq!(l1.mid_price, Some(10000));
        assert!(l1.has_valid_market());
    }

    #[test]
    fn test_level1_update_trade() {
        let mut l1 = Level1::new(Some(9900), 100, Some(10100), 200);
        l1.update_last_trade(10000, 50);

        assert_eq!(l1.last_trade_price, Some(10000));
        assert_eq!(l1.last_trade_quantity, 50);
    }

    #[test]
    fn test_level2_add_levels() {
        let mut l2 = Level2::<5>::new();

        l2.add_bid(PriceLevel::new(9900, 100, 3)).unwrap();
        l2.add_bid(PriceLevel::new(9800, 200, 5)).unwrap();
        l2.add_ask(PriceLevel::new(10100, 150, 4)).unwrap();

        assert_eq!(l2.bid_count, 2);
        assert_eq!(l2.ask_count, 1);
        assert_eq!(l2.total_bid_quantity(), 300);
        assert_eq!(l2.total_ask_quantity(), 150);
    }

    #[test]
    fn test_level2_capacity_limit() {
        let mut l2 = Level2::<2>::new();

        assert!(l2.add_bid(PriceLevel::new(9900, 100, 1)).is_ok());
        assert!(l2.add_bid(PriceLevel::new(9800, 100, 1)).is_ok());
        assert!(l2.add_bid(PriceLevel::new(9700, 100, 1)).is_err());
    }

    #[test]
    fn test_level3_operations() {
        let mut l3 = Level3::with_capacity(10);
        let trader = TraderId::from_str("TRADER1");

        let order1 = Level3Order::new(1, trader, 9900, 100, 100);
        let order2 = Level3Order::new(2, trader, 10100, 200, 150);

        l3.add_bid(order1);
        l3.add_ask(order2);

        assert_eq!(l3.bids.len(), 1);
        assert_eq!(l3.asks.len(), 1);
        assert_eq!(l3.active_order_count(), 2);

        assert!(l3.find_order(1).is_some());
        assert!(l3.find_order(999).is_none());

        assert!(l3.remove_order(1));
        assert_eq!(l3.bids.len(), 0);
    }

    #[test]
    fn test_level3_clear() {
        let mut l3 = Level3::new();
        let trader = TraderId::from_str("TRADER");

        l3.add_bid(Level3Order::new(1, trader, 9900, 100, 100));
        l3.add_ask(Level3Order::new(2, trader, 10100, 100, 100));

        l3.clear();

        assert_eq!(l3.bids.len(), 0);
        assert_eq!(l3.asks.len(), 0);
    }
}
