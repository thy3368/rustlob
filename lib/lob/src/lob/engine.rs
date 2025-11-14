/// 订单簿引擎 - Facade模式
///
/// 提供统一的订单簿操作接口，内部使用Repository和MatchingService
/// 遵循Clean Architecture的Facade模式，简化客户端使用

use super::handler::OrderCommandHandler;
use super::matching_service::{MatchingService, MarketDataService};
use super::repository::{InMemoryOrderRepository, OrderRepository};
use super::types::{OrderEntry, OrderId, Price, Quantity, Side, Trade, TraderId};

/// 最大价格级别（以分为单位）
const MAX_PRICE: usize = 10_000_000; // 最高价格 $100,000

/// 订单簿引擎
///
/// 整合仓储和匹配服务，提供简单的订单操作接口
pub struct OrderBook {
    /// 订单仓储
    repository: InMemoryOrderRepository,
    /// 匹配服务
    matching_service: MatchingService,
    /// 市场数据服务
    market_data_service: MarketDataService,
    /// 交易历史
    trades: Vec<Trade>,
}

impl OrderBook {
    /// 创建新的订单簿
    pub fn new() -> Self {
        Self::with_capacity(MAX_PRICE, 1_000_000)
    }

    /// 创建指定容量的新订单簿
    pub fn with_capacity(max_price: usize, max_orders: usize) -> Self {
        Self {
            repository: InMemoryOrderRepository::new(max_price, max_orders),
            matching_service: MatchingService::new(),
            market_data_service: MarketDataService::new(),
            trades: Vec::new(),
        }
    }

    /// 获取下一个订单ID
    #[inline]
    pub fn next_order_id(&self) -> OrderId {
        self.repository.next_order_id()
    }

    /// 设置下一个订单ID（用于状态恢复）
    #[inline]
    pub fn set_next_order_id(&mut self, id: OrderId) {
        self.repository.set_next_order_id(id);
    }

    /// 获取最佳买价
    #[inline]
    pub fn best_bid(&self) -> Option<Price> {
        self.market_data_service.find_best_bid(&self.repository)
    }

    /// 获取最佳卖价
    #[inline]
    pub fn best_ask(&self) -> Option<Price> {
        self.market_data_service.find_best_ask(&self.repository)
    }

    /// 获取买卖价差（卖价 - 买价）
    #[inline]
    pub fn spread(&self) -> Option<Price> {
        self.market_data_service.calculate_spread(&self.repository)
    }

    /// 获取中间价
    #[inline]
    pub fn mid_price(&self) -> Option<Price> {
        self.market_data_service.calculate_mid_price(&self.repository)
    }

    /// 提交新的限价订单
    ///
    /// 返回 (订单ID, 成交列表)
    pub fn limit_order(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> (OrderId, Vec<Trade>) {
        // 分配订单ID
        let order_id = self.repository.allocate_order_id();

        // 执行匹配
        let (trades, remaining) = self.matching_service.match_limit_order(
            &mut self.repository,
            trader,
            side,
            price,
            quantity,
        );

        // 如果有剩余，添加到订单簿
        if remaining > 0 {
            let entry = OrderEntry::new(order_id, trader, remaining);
            if let Err(e) = self.repository.add_order(order_id, entry, side, price) {
                eprintln!("添加订单失败: {}", e);
            }
        }

        // 存储交易记录
        self.trades.extend(&trades);

        (order_id, trades)
    }

    /// 提交市价订单
    ///
    /// 市价单以当前市场最优价格立即成交，无价格限制
    /// 返回 (成交列表)
    pub fn market_order(
        &mut self,
        trader: TraderId,
        side: Side,
        quantity: Quantity,
    ) -> Vec<Trade> {
        // 执行市价单匹配
        let (trades, _remaining) = self.matching_service.handle_market_order(
            &mut self.repository,
            trader,
            side,
            quantity,
        );

        // 存储交易记录
        self.trades.extend(&trades);

        trades
    }

    /// 提交冰山订单
    ///
    /// 冰山单只显示部分数量，隐藏总量以避免影响市场
    /// 返回 (订单ID, 成交列表, 剩余总量, 当前显示量)
    pub fn iceberg_order(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        total_quantity: Quantity,
        display_quantity: Quantity,
    ) -> (OrderId, Vec<Trade>, Quantity, Quantity) {
        // 分配订单ID
        let order_id = self.repository.allocate_order_id();

        // 执行冰山单匹配
        let (trades, remaining_total, current_display) = self.matching_service.handle_iceberg_order(
            &mut self.repository,
            trader,
            side,
            price,
            total_quantity,
            display_quantity,
        );

        // 如果有剩余显示数量，添加到订单簿
        if current_display > 0 {
            let entry = OrderEntry::new(order_id, trader, current_display);
            if let Err(e) = self.repository.add_order(order_id, entry, side, price) {
                eprintln!("添加冰山订单失败: {}", e);
            }
        }

        // 存储交易记录
        self.trades.extend(&trades);

        (order_id, trades, remaining_total, current_display)
    }

    /// 取消订单
    pub fn cancel_order(&mut self, order_id: OrderId) -> bool {
        self.repository.cancel_order(order_id)
    }

    /// 获取交易历史
    pub fn trades(&self) -> &[Trade] {
        &self.trades
    }

    /// 清空交易历史
    pub fn clear_trades(&mut self) {
        self.trades.clear();
    }

    /// 获取订单簿状态快照
    pub fn snapshot(&self) -> OrderBookSnapshot {
        OrderBookSnapshot {
            next_order_id: self.repository.next_order_id(),
            bid_max: self.best_bid(),
            ask_min: self.best_ask(),
            active_orders: self.repository.active_order_count(),
            total_trades: self.trades.len(),
        }
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

/// 订单簿状态快照
#[derive(Debug, Clone, Copy)]
pub struct OrderBookSnapshot {
    pub next_order_id: OrderId,       // 下一个订单ID
    pub bid_max: Option<Price>,       // 最佳买价
    pub ask_min: Option<Price>,       // 最佳卖价
    pub active_orders: usize,         // 活跃订单数
    pub total_trades: usize,          // 总交易数
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_buy_order() {
        let mut book = OrderBook::new();
        let trader = TraderId::from_str("TRADER1");

        let (order_id, trades) = book.limit_order(trader, Side::Buy, 10000, 100);

        assert_eq!(order_id, 1);
        assert_eq!(trades.len(), 0); // No matches
        assert_eq!(book.best_bid(), Some(10000));
        assert_eq!(book.best_ask(), None);
    }

    #[test]
    fn test_simple_match() {
        let mut book = OrderBook::new();
        let buyer = TraderId::from_str("BUYER");
        let seller = TraderId::from_str("SELLER");

        // Place sell order
        book.limit_order(seller, Side::Sell, 10000, 100);

        // Place matching buy order
        let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 100);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 100);
        assert_eq!(trades[0].price, 10000);
    }

    #[test]
    fn test_partial_fill() {
        let mut book = OrderBook::new();
        let buyer = TraderId::from_str("BUYER");
        let seller = TraderId::from_str("SELLER");

        // Place large sell order
        book.limit_order(seller, Side::Sell, 10000, 200);

        // Place smaller buy order
        let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 50);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 50);
        assert_eq!(book.best_ask(), Some(10000)); // Still has remaining
    }

    #[test]
    fn test_price_improvement() {
        let mut book = OrderBook::new();
        let buyer = TraderId::from_str("BUYER");
        let seller = TraderId::from_str("SELLER");

        // Place sell order at 10000
        book.limit_order(seller, Side::Sell, 10000, 100);

        // Place buy order at higher price (11000)
        let (_order_id, trades) = book.limit_order(buyer, Side::Buy, 11000, 100);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].price, 10000); // Matched at seller's price
    }

    #[test]
    fn test_cancel_order() {
        let mut book = OrderBook::new();
        let trader = TraderId::from_str("TRADER1");

        let (order_id, _) = book.limit_order(trader, Side::Buy, 10000, 100);
        assert!(book.cancel_order(order_id));
        assert!(!book.cancel_order(order_id)); // Already cancelled
    }

    #[test]
    fn test_spread() {
        let mut book = OrderBook::new();

        book.limit_order(TraderId::from_str("B"), Side::Buy, 9900, 100);
        book.limit_order(TraderId::from_str("S"), Side::Sell, 10100, 100);

        assert_eq!(book.best_bid(), Some(9900));
        assert_eq!(book.best_ask(), Some(10100));
        assert_eq!(book.spread(), Some(200));
        assert_eq!(book.mid_price(), Some(10000));
    }

    #[test]
    fn test_market_order_buy() {
        let mut book = OrderBook::new();
        let buyer = TraderId::from_str("BUYER");
        let seller = TraderId::from_str("SELLER");

        // 添加多层卖单
        book.limit_order(seller, Side::Sell, 10000, 50);
        book.limit_order(seller, Side::Sell, 10100, 50);
        book.limit_order(seller, Side::Sell, 10200, 50);

        // 市价买单应该从最低价开始成交
        let trades = book.market_order(buyer, Side::Buy, 100);

        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, 10000);
        assert_eq!(trades[0].quantity, 50);
        assert_eq!(trades[1].price, 10100);
        assert_eq!(trades[1].quantity, 50);
    }

    #[test]
    fn test_market_order_sell() {
        let mut book = OrderBook::new();
        let buyer = TraderId::from_str("BUYER");
        let seller = TraderId::from_str("SELLER");

        // 添加多层买单
        book.limit_order(buyer, Side::Buy, 10200, 50);
        book.limit_order(buyer, Side::Buy, 10100, 50);
        book.limit_order(buyer, Side::Buy, 10000, 50);

        // 市价卖单应该从最高价开始成交
        let trades = book.market_order(seller, Side::Sell, 100);

        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, 10200);
        assert_eq!(trades[0].quantity, 50);
        assert_eq!(trades[1].price, 10100);
        assert_eq!(trades[1].quantity, 50);
    }

    #[test]
    fn test_iceberg_order_partial_display() {
        let mut book = OrderBook::new();
        let buyer = TraderId::from_str("BUYER");
        let seller = TraderId::from_str("SELLER");

        // 添加对手方订单
        book.limit_order(buyer, Side::Buy, 10000, 50);

        // 冰山卖单：总量1000，显示量100
        let (_order_id, trades, remaining_total, current_display) =
            book.iceberg_order(seller, Side::Sell, 10000, 1000, 100);

        // 应该成交50，剩余总量950，当前显示量50
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 50);
        assert_eq!(remaining_total, 950);
        assert_eq!(current_display, 50);
    }

    #[test]
    fn test_iceberg_order_full_display() {
        let mut book = OrderBook::new();
        let buyer = TraderId::from_str("BUYER");
        let seller = TraderId::from_str("SELLER");

        // 添加对手方订单
        book.limit_order(buyer, Side::Buy, 10000, 150);

        // 冰山卖单：总量1000，显示量100
        let (_order_id, trades, remaining_total, current_display) =
            book.iceberg_order(seller, Side::Sell, 10000, 1000, 100);

        // 第一批100全部成交，补充新的100，再成交50
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].quantity, 100);
        assert_eq!(trades[1].quantity, 50);
        assert_eq!(remaining_total, 850);
        assert_eq!(current_display, 50);
    }

    #[test]
    fn test_iceberg_order_no_match() {
        let mut book = OrderBook::new();
        let seller = TraderId::from_str("SELLER");

        // 没有对手方订单
        let (_order_id, trades, remaining_total, current_display) =
            book.iceberg_order(seller, Side::Sell, 10000, 1000, 100);

        // 无成交，显示量100进入订单簿
        assert_eq!(trades.len(), 0);
        assert_eq!(remaining_total, 1000);
        assert_eq!(current_display, 100);
        assert_eq!(book.best_ask(), Some(10000));
    }
}
