use std::collections::HashMap;

use base_types::lob::lob::LobOrder;
use base_types::{OrderId, OrderSide, Price, Quantity, TradingPair};

use crate::core::symbol_lob_repo::{RepoError, SymbolLob};

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

//todo 用type 代码范型
/// 订单包装器
///
/// 包装 Order trait 对象，并添加链表指针
struct OrderNode<O: LobOrder> {
    order: O,
    /// 指向同价格级别下一个订单的索引
    next_idx: Option<usize>,
}

impl<O: LobOrder> OrderNode<O> {
    fn new(order: O) -> Self {
        Self { order, next_idx: None }
    }
}

/// 本地 LOB 实现 (HashMap 版本)
///
/// 使用 HashMap 存储订单，支持 O(1) 期望的价格查找和 O(k) 的订单匹配
/// 使用 Tick Size 将价格规整到固定精度，避免浮点数精度问题
///
/// # 优势
/// - 不受价格范围限制（相比 Vec 方案）
/// - 内存使用高效（只存储有订单的价格级别）
/// - 支持任意精度的交易对（通过 tick_size 配置）
///
/// # 适用场景
/// - 低价币（SHIB, PEPE 等需要高精度）
/// - 价格波动范围大的币种
/// - 内存受限的环境
pub struct LocalLobHashMap<O: LobOrder> {
    symbol: TradingPair,
    /// 最小价格变动单位（tick size）
    tick_size: Price,
    /// 买单价格点（key = tick 数量）
    bids: HashMap<i64, PricePoint>,
    /// 卖单价格点（key = tick 数量）
    asks: HashMap<i64, PricePoint>,
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

impl<O: LobOrder> LocalLobHashMap<O> {
    /// 创建新的本地 LOB（使用默认 tick size）
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    ///
    /// # 默认配置
    /// - tick_size: 0.01 USDT (适合 BTC/ETH 等主流币)
    /// - max_orders: 10,000 个订单
    pub fn new(symbol: TradingPair) -> Self {
        Self::new_with_tick(symbol, Price::from_f64(0.01))
    }

    /// 创建指定 tick size 的本地 LOB
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    /// - `tick_size`: 最小价格变动单位
    ///
    /// # 示例
    /// ```ignore
    /// // BTC/ETH 等高价币：tick_size = 0.01
    /// let btc_lob = LocalLobHashMap::new_with_tick(Symbol::new("BTCUSDT"), Price::from_f64(0.01));
    ///
    /// // DOGE 等中价币：tick_size = 0.0001
    /// let doge_lob = LocalLobHashMap::new_with_tick(Symbol::new("DOGEUSDT"), Price::from_f64(0.0001));
    ///
    /// // SHIB/PEPE 等低价币：tick_size = 0.00000001
    /// let shib_lob = LocalLobHashMap::new_with_tick(Symbol::new("SHIBUSDT"), Price::from_f64(0.00000001));
    /// ```
    pub fn new_with_tick(symbol: TradingPair, tick_size: Price) -> Self {
        Self::with_capacity(symbol, tick_size, 10_000)
    }

    /// 创建指定容量的本地 LOB
    pub fn with_capacity(symbol: TradingPair, tick_size: Price, max_orders: usize) -> Self {
        Self {
            symbol,
            tick_size,
            bids: HashMap::new(),
            asks: HashMap::new(),
            orders: Vec::with_capacity(max_orders),
            order_index: HashMap::with_capacity(max_orders),
            bid_max: None,
            ask_min: None,
            last_trade_price: None,
            next_slot: 0,
        }
    }

    pub fn symbol(&self) -> &TradingPair {
        &self.symbol
    }

    /// 将价格转换为 tick 数量
    #[inline]
    fn price_to_tick(&self, price: Price) -> Option<i64> {
        if self.tick_size.raw() == 0 {
            return None;
        }
        Some(price.raw() / self.tick_size.raw())
    }

    /// 将 tick 数量转换为价格
    #[inline]
    fn tick_to_price(&self, tick: i64) -> Price {
        Price::from_raw(tick * self.tick_size.raw())
    }

    /// 获取价格点的可变引用
    fn get_price_point_mut(&mut self, price: Price, side: OrderSide) -> Option<&mut PricePoint> {
        let tick = self.price_to_tick(price)?;
        match side {
            OrderSide::Buy => self.bids.get_mut(&tick),
            OrderSide::Sell => self.asks.get_mut(&tick),
        }
    }

    /// 获取价格点的不可变引用
    fn get_price_point(&self, price: Price, side: OrderSide) -> Option<&PricePoint> {
        let tick = self.price_to_tick(price)?;
        match side {
            OrderSide::Buy => self.bids.get(&tick),
            OrderSide::Sell => self.asks.get(&tick),
        }
    }

    /// 获取指定价格级别的第一个订单索引
    #[inline]
    fn get_first_order_at_price(&self, price: Price, side: OrderSide) -> Option<usize> {
        self.get_price_point(price, side).and_then(|pp| pp.first_order_idx)
    }

    /// 将订单链接到价格级别的链表尾部
    fn link_order_to_price_level(&mut self, idx: usize, price: Price, side: OrderSide) {
        let tick = match self.price_to_tick(price) {
            Some(t) => t,
            None => return,
        };

        let price_map = match side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        // 获取或创建价格点
        let price_point = price_map.entry(tick).or_insert_with(PricePoint::default);

        // 获取当前链表尾部
        let last_idx = price_point.last_order_idx;

        // 链接到现有尾部订单
        if let Some(last_idx) = last_idx {
            if let Some(Some(last_node)) = self.orders.get_mut(last_idx) {
                last_node.next_idx = Some(idx);
            }
        }

        // 更新价格点的首尾指针
        price_point.push_back(idx);
    }

    /// 更新最佳买卖价缓存
    #[inline]
    fn update_best_price(&mut self, price: Price, side: OrderSide) {
        match side {
            OrderSide::Buy => {
                if self.bid_max.map_or(true, |max| price > max) {
                    self.bid_max = Some(price);
                }
            }
            OrderSide::Sell => {
                if self.ask_min.map_or(true, |min| price < min) {
                    self.ask_min = Some(price);
                }
            }
        }
    }
}

impl<O: LobOrder> SymbolLob for LocalLobHashMap<O> {
    type Order = O;
    /// 匹配订单
    ///
    /// 根据 side, price, quantity 匹配所有符合条件的订单
    /// 返回匹配上的订单数组和剩余未匹配数量
    ///
    /// # 算法
    /// - 买单：从最低卖价开始匹配（价格优先，时间优先）
    /// - 卖单：从最高买价开始匹配（价格优先，时间优先）
    fn match_orders(
        &self,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> (Option<Vec<&Self::Order>>, Quantity) {
        // 预分配容量，减少内存重分配开销
        let mut matched_orders = Vec::with_capacity(16);
        let mut remaining = quantity;
        let opposite_side = side.opposite();

        match side {
            OrderSide::Buy => {
                // 买单：从最低卖价开始匹配
                if let Some(ask_min) = self.ask_min {
                    if price < ask_min {
                        return (None, quantity); // 买价太低，无法匹配
                    }

                    // 获取价格范围的 tick
                    let price_tick = match self.price_to_tick(price) {
                        Some(t) => t,
                        None => return (None, quantity),
                    };
                    let ask_min_tick = match self.price_to_tick(ask_min) {
                        Some(t) => t,
                        None => return (None, quantity),
                    };

                    // 遍历所有存在的卖单价格点
                    let mut ticks: Vec<i64> = self
                        .asks
                        .keys()
                        .filter(|&&t| t >= ask_min_tick && t <= price_tick)
                        .copied()
                        .collect();
                    ticks.sort_unstable(); // 从低到高排序

                    for tick in ticks {
                        if remaining.is_zero() {
                            break;
                        }

                        let current_price = self.tick_to_price(tick);
                        if let Some(first_idx) =
                            self.get_first_order_at_price(current_price, opposite_side)
                        {
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
                                        remaining =
                                            Quantity::from_raw(remaining.raw() - fill_qty.raw());
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
            OrderSide::Sell => {
                // 卖单：从最高买价开始匹配
                if let Some(bid_max) = self.bid_max {
                    if price > bid_max {
                        return (None, quantity); // 卖价太高，无法匹配
                    }

                    // 获取价格范围的 tick
                    let price_tick = match self.price_to_tick(price) {
                        Some(t) => t,
                        None => return (None, quantity),
                    };
                    let bid_max_tick = match self.price_to_tick(bid_max) {
                        Some(t) => t,
                        None => return (None, quantity),
                    };

                    // 遍历所有存在的买单价格点
                    let mut ticks: Vec<i64> = self
                        .bids
                        .keys()
                        .filter(|&&t| t >= price_tick && t <= bid_max_tick)
                        .copied()
                        .collect();
                    ticks.sort_unstable();
                    ticks.reverse(); // 从高到低排序

                    for tick in ticks {
                        if remaining.is_zero() {
                            break;
                        }

                        let current_price = self.tick_to_price(tick);
                        if let Some(first_idx) =
                            self.get_first_order_at_price(current_price, opposite_side)
                        {
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
                                        remaining =
                                            Quantity::from_raw(remaining.raw() - fill_qty.raw());
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
            (None, quantity)
        } else {
            (Some(matched_orders), remaining)
        }
    }

    fn add_order(&mut self, order: Self::Order) -> Result<(), RepoError> {
        let order_id = order.order_id();
        let price = order.price();
        let side = order.side();

        // === 1. 前置验证（不分配资源）===
        if self.order_index.contains_key(&order_id) {
            return Err(RepoError::OrderAlreadyExists);
        }
        if self.price_to_tick(price).is_none() {
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

    fn find_order(&self, order_id: OrderId) -> Option<&Self::Order> {
        self.order_index
            .get(&order_id)
            .and_then(|&idx| self.orders.get(idx))
            .and_then(|opt_node| opt_node.as_ref())
            .map(|node| &node.order)
    }

    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut Self::Order> {
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
