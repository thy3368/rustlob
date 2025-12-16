use std::collections::HashMap;
use base_types::{OrderId, Price, Quantity, Side, Symbol};
use crate::core::symbol_lob_repo::{Order, RepoError, SymbolLob};

/// 价格点结构
///
/// 存储某个价格级别的所有订单的链表头尾指针
#[derive(Debug, Default, Clone)]
struct PricePoint {
    /// 该价格级别的第一个订单在 orders 中的索引
    first_order_idx: Option<usize>,
    /// 该价格级别的最后一个订单在 orders 中的索引
    last_order_idx: Option<usize>,
}

impl PricePoint {
    /// 将订单索引添加到链表尾部
    fn push_back(&mut self, idx: usize) {
        if self.first_order_idx.is_none() {
            self.first_order_idx = Some(idx);
        }
        self.last_order_idx = Some(idx);
    }
}

/// 订单包装器
///
/// 包装 Order trait 对象，并添加链表指针
struct OrderNode<O: Order> {
    order: O,
    /// 指向同价格级别下一个订单的索引
    next_idx: Option<usize>,
}

impl<O: Order> OrderNode<O> {
    fn new(order: O) -> Self {
        Self {
            order,
            next_idx: None,
        }
    }
}

/// 本地 LOB 实现
///
/// 使用内存存储订单，支持 O(1) 的价格查找和 O(k) 的订单匹配
pub struct LocalLob<O: Order> {
    symbol: Symbol,
    /// 买单价格点（索引即价格）
    bids: Vec<PricePoint>,
    /// 卖单价格点（索引即价格）
    asks: Vec<PricePoint>,
    /// 订单存储池
    orders: Vec<Option<OrderNode<O>>>,
    /// 订单ID到存储池索引的映射
    order_index: HashMap<OrderId, usize>,
    /// 最佳买价（最高买入价）
    bid_max: Option<Price>,
    /// 最佳卖价（最低卖出价）
    ask_min: Option<Price>,
    /// 最后一笔成交价
    last_trade_price: Option<Price>,
    /// 下一个可用的订单槽位索引
    next_slot: usize,
}

impl<O: Order> LocalLob<O> {
    /// 创建新的本地 LOB
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    /// - `max_price`: 支持的最大价格（默认 100,000）
    /// - `max_orders`: 最大订单数量（默认 10,000）
    pub fn new(symbol: Symbol) -> Self {
        Self::with_capacity(symbol, 100_000, 10_000)
    }

    /// 创建指定容量的本地 LOB
    pub fn with_capacity(symbol: Symbol, max_price: usize, max_orders: usize) -> Self {
        Self {
            symbol,
            bids: vec![PricePoint::default(); max_price],
            asks: vec![PricePoint::default(); max_price],
            orders: Vec::with_capacity(max_orders),
            order_index: HashMap::with_capacity(max_orders),
            bid_max: None,
            ask_min: None,
            last_trade_price: None,
            next_slot: 0,
        }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    /// 获取价格点的可变引用
    fn get_price_point_mut(&mut self, price: Price, side: Side) -> Option<&mut PricePoint> {
        let price_idx = price.raw() as usize;
        match side {
            Side::Buy => self.bids.get_mut(price_idx),
            Side::Sell => self.asks.get_mut(price_idx),
        }
    }

    /// 获取价格点的不可变引用
    fn get_price_point(&self, price: Price, side: Side) -> Option<&PricePoint> {
        let price_idx = price.raw() as usize;
        match side {
            Side::Buy => self.bids.get(price_idx),
            Side::Sell => self.asks.get(price_idx),
        }
    }

    /// 获取指定价格级别的第一个订单索引
    #[inline]
    fn get_first_order_at_price(&self, price: Price, side: Side) -> Option<usize> {
        self.get_price_point(price, side).and_then(|pp| pp.first_order_idx)
    }

    /// 将订单链接到价格级别的链表尾部
    fn link_order_to_price_level(&mut self, idx: usize, price: Price, side: Side) {
        // 获取当前链表尾部
        let last_idx = self.get_price_point(price, side).and_then(|pp| pp.last_order_idx);

        // 链接到现有尾部订单
        if let Some(last_idx) = last_idx {
            if let Some(Some(last_node)) = self.orders.get_mut(last_idx) {
                last_node.next_idx = Some(idx);
            }
        }

        // 更新价格点的首尾指针
        if let Some(price_point) = self.get_price_point_mut(price, side) {
            price_point.push_back(idx);
        }
    }

    /// 更新最佳买卖价缓存
    #[inline]
    fn update_best_price(&mut self, price: Price, side: Side) {
        match side {
            Side::Buy => {
                if self.bid_max.map_or(true, |max| price > max) {
                    self.bid_max = Some(price);
                }
            }
            Side::Sell => {
                if self.ask_min.map_or(true, |min| price < min) {
                    self.ask_min = Some(price);
                }
            }
        }
    }
}

impl<O: Order> SymbolLob<O> for LocalLob<O> {
    /// 匹配订单
    ///
    /// 根据 side, price, quantity 匹配所有符合条件的订单
    /// 总数量要大于等于 quantity，返回匹配上的订单数组
    ///
    /// # 算法
    /// - 买单：从最低卖价开始匹配（价格优先，时间优先）
    /// - 卖单：从最高买价开始匹配（价格优先，时间优先）
    fn match_orders(&self, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&O>> {
        // 预分配容量，减少内存重分配开销
        let mut matched_orders = Vec::with_capacity(16);
        let mut remaining = quantity;
        let opposite_side = side.opposite();

        match side {
            Side::Buy => {
                // 买单：从最低卖价开始匹配
                if let Some(ask_min) = self.ask_min {
                    if price < ask_min {
                        return None; // 买价太低，无法匹配
                    }

                    // 遍历价格从低到高
                    let price_raw = price.raw() as i64;
                    let ask_min_raw = ask_min.raw() as i64;

                    for current_price_raw in ask_min_raw..=price_raw {
                        if remaining.is_zero() {
                            break;
                        }

                        let current_price = Price::from_raw(current_price_raw);
                        if let Some(first_idx) = self.get_first_order_at_price(current_price, opposite_side) {
                            let mut current_idx = Some(first_idx);

                            while !remaining.is_zero() && current_idx.is_some() {
                                let idx = current_idx.unwrap();

                                if let Some(Some(node)) = self.orders.get(idx) {
                                    let order_qty = node.order.quantity();
                                    if order_qty > Quantity::from_raw(0) {
                                        let fill_qty = if remaining < order_qty {
                                            remaining
                                        } else {
                                            order_qty
                                        };
                                        remaining = Quantity::from_raw(remaining.raw() - fill_qty.raw());
                                        matched_orders.push(&node.order);
                                    }
                                    current_idx = node.next_idx;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Side::Sell => {
                // 卖单：从最高买价开始匹配
                if let Some(bid_max) = self.bid_max {
                    if price > bid_max {
                        return None; // 卖价太高，无法匹配
                    }

                    // 遍历价格从高到低
                    let price_raw = price.raw() as i64;
                    let bid_max_raw = bid_max.raw() as i64;

                    for current_price_raw in (price_raw..=bid_max_raw).rev() {
                        if remaining.is_zero() {
                            break;
                        }

                        let current_price = Price::from_raw(current_price_raw);
                        if let Some(first_idx) = self.get_first_order_at_price(current_price, opposite_side) {
                            let mut current_idx = Some(first_idx);

                            while !remaining.is_zero() && current_idx.is_some() {
                                let idx = current_idx.unwrap();

                                if let Some(Some(node)) = self.orders.get(idx) {
                                    let order_qty = node.order.quantity();
                                    if order_qty > Quantity::from_raw(0) {
                                        let fill_qty = if remaining < order_qty {
                                            remaining
                                        } else {
                                            order_qty
                                        };
                                        remaining = Quantity::from_raw(remaining.raw() - fill_qty.raw());
                                        matched_orders.push(&node.order);
                                    }
                                    current_idx = node.next_idx;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if matched_orders.is_empty() {
            None
        } else {
            Some(matched_orders)
        }
    }

    fn add_order(&mut self, order: O) -> Result<(), RepoError> {
        let order_id = order.order_id();
        let price = order.price();
        let side = order.side();

        // === 1. 前置验证（不分配资源）===
        if self.order_index.contains_key(&order_id) {
            return Err(RepoError::OrderAlreadyExists);
        }
        if self.get_price_point(price, side).is_none() {
            return Err(RepoError::PriceOutOfRange);
        }

        // === 2. 分配槽位 ===
        let idx = self.next_slot;
        if idx >= self.orders.capacity() {
            return Err(RepoError::CapacityExceeded);
        }

        // === 3. 存储订单 ===
        let node = OrderNode::new(order);
        if idx == self.orders.len() {
            self.orders.push(Some(node));
        } else {
            self.orders[idx] = Some(node);
        }
        self.next_slot += 1;

        // === 4. 更新索引和链表 ===
        self.order_index.insert(order_id, idx);
        self.link_order_to_price_level(idx, price, side);

        // === 5. 更新最佳价格缓存 ===
        self.update_best_price(price, side);

        Ok(())
    }

    fn remove_order(&mut self, order_id: OrderId) -> bool {
        if let Some(&idx) = self.order_index.get(&order_id) {
            if self.orders.get(idx).and_then(|o| o.as_ref()).is_some() {
                self.orders[idx] = None;
                self.order_index.remove(&order_id);
                // 注意：不立即更新链表，订单标记为 None 即可
                return true;
            }
        }
        false
    }

    fn find_order(&self, order_id: OrderId) -> Option<&O> {
        self.order_index
            .get(&order_id)
            .and_then(|&idx| self.orders.get(idx))
            .and_then(|opt_node| opt_node.as_ref())
            .map(|node| &node.order)
    }

    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut O> {
        self.order_index
            .get(&order_id)
            .and_then(|&idx| self.orders.get_mut(idx))
            .and_then(|opt_node| opt_node.as_mut())
            .map(|node| &mut node.order)
    }

    fn best_bid(&self) -> Option<Price> {
        self.bid_max
    }

    fn best_ask(&self) -> Option<Price> {
        self.ask_min
    }

    fn last_price(&self) -> Option<Price> {
        self.last_trade_price
    }

    fn update_last_price(&mut self, price: Price) {
        self.last_trade_price = Some(price);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 创建模拟订单用于测试
    #[derive(Debug, Clone)]
    struct MockOrder {
        id: u64,
        symbol: Symbol,
        price: Price,
        quantity: Quantity,
        side: Side,
    }

    impl Order for MockOrder {
        fn order_id(&self) -> OrderId {
            self.id
        }

        fn price(&self) -> Price {
            self.price
        }

        fn quantity(&self) -> Quantity {
            self.quantity
        }

        fn side(&self) -> Side {
            self.side
        }

        fn symbol(&self) -> Symbol {
            self.symbol
        }
    }

    #[test]
    fn test_last_price_initially_none() {
        let symbol = Symbol::new("BTCUSDT");
        let lob: LocalLob<MockOrder> = LocalLob::new(symbol);

        // 初始状态，最后成交价应为 None
        assert_eq!(lob.last_price(), None);
    }

    #[test]
    fn test_update_last_price() {
        let symbol = Symbol::new("BTCUSDT");
        let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

        // 更新最后成交价
        let trade_price = Price::from_raw(50000);
        lob.update_last_price(trade_price);

        // 验证最后成交价已更新
        assert_eq!(lob.last_price(), Some(trade_price));
    }

    #[test]
    fn test_multiple_price_updates() {
        let symbol = Symbol::new("BTCUSDT");
        let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

        // 第一次成交
        let price1 = Price::from_raw(50000);
        lob.update_last_price(price1);
        assert_eq!(lob.last_price(), Some(price1));

        // 第二次成交
        let price2 = Price::from_raw(50100);
        lob.update_last_price(price2);
        assert_eq!(lob.last_price(), Some(price2));

        // 第三次成交
        let price3 = Price::from_raw(49900);
        lob.update_last_price(price3);
        assert_eq!(lob.last_price(), Some(price3));
    }

    #[test]
    fn test_add_and_find_order() {
        let symbol = Symbol::new("BTCUSDT");
        let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

        let order = MockOrder {
            id: 1,
            symbol,
            price: Price::from_raw(50000),  // 使用 raw 值在有效范围内
            quantity: Quantity::from_raw(100),
            side: Side::Buy,
        };

        // 添加订单
        let result = lob.add_order(order.clone());
        assert!(result.is_ok());

        // 查找订单
        let found = lob.find_order(1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().order_id(), 1);
    }

    #[test]
    fn test_best_bid_ask() {
        let symbol = Symbol::new("BTCUSDT");
        let mut lob: LocalLob<MockOrder> = LocalLob::new(symbol);

        // 初始状态
        assert_eq!(lob.best_bid(), None);
        assert_eq!(lob.best_ask(), None);

        // 添加买单
        let buy_order = MockOrder {
            id: 1,
            symbol,
            price: Price::from_raw(50000),  // 使用 raw 值在有效范围内
            quantity: Quantity::from_raw(100),
            side: Side::Buy,
        };
        lob.add_order(buy_order).unwrap();
        assert_eq!(lob.best_bid(), Some(Price::from_raw(50000)));

        // 添加卖单
        let sell_order = MockOrder {
            id: 2,
            symbol,
            price: Price::from_raw(50100),  // 使用 raw 值在有效范围内
            quantity: Quantity::from_raw(100),
            side: Side::Sell,
        };
        lob.add_order(sell_order).unwrap();
        assert_eq!(lob.best_ask(), Some(Price::from_raw(50100)));
    }
}

