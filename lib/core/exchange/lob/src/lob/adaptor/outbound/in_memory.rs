use crate::lob::adaptor::outbound::arena::OrderArena;
use crate::lob::domain::entity::lob_types::{
    EntityEvent, EventOperation, FieldValue, OrderEntry, OrderId, Price, PricePoint, Quantity, Side,
};
/// 内存仓储实现
///
/// 使用内存池和价格索引数组实现高性能订单存储
use crate::lob::domain::repository::traits::{
    OrderRepository, RepositoryAccessor, RepositoryError,
};
use std::collections::HashMap;

/// 内存仓储实现
///
/// 使用内存池和价格索引数组实现高性能订单存储
pub struct InMemoryOrderRepository {
    /// 买单价格点（出价）
    bids: Vec<PricePoint>,
    /// 卖单价格点（要价）
    asks: Vec<PricePoint>,
    /// 订单条目的内存池
    arena: OrderArena,
    /// 订单ID到内存池索引的映射
    order_index: HashMap<OrderId, usize>,
    /// 下一个订单ID
    next_order_id: OrderId,

    /// 最佳买价（最高买入价）
    bid_max: Option<Price>,
    /// 最佳卖价（最低卖出价）
    ask_min: Option<Price>,
}

impl InMemoryOrderRepository {
    /// 创建新的内存仓储
    pub fn new(max_price: usize, max_orders: usize) -> Self {
        Self {
            bids: vec![PricePoint::default(); max_price],
            asks: vec![PricePoint::default(); max_price],
            arena: OrderArena::new(max_orders),
            order_index: HashMap::with_capacity(max_orders),
            next_order_id: 1,
            bid_max: None,
            ask_min: None,
        }
    }

    /// 获取价格点的可变引用
    fn get_price_point_mut(&mut self, price: Price, side: Side) -> Option<&mut PricePoint> {
        let price_idx = price as usize;
        match side {
            Side::Buy => self.bids.get_mut(price_idx),
            Side::Sell => self.asks.get_mut(price_idx),
        }
    }

    /// 获取价格点的不可变引用
    fn get_price_point(&self, price: Price, side: Side) -> Option<&PricePoint> {
        let price_idx = price as usize;
        match side {
            Side::Buy => self.bids.get(price_idx),
            Side::Sell => self.asks.get(price_idx),
        }
    }

    /// 重新计算最佳买价（从高到低扫描）
    fn recalculate_bid_max(&mut self) {
        self.bid_max = None;
        for price in (0..self.bids.len()).rev() {
            if !self.bids[price].is_empty() {
                self.bid_max = Some(price as Price);
                return;
            }
        }
    }

    /// 重新计算最佳卖价（从低到高扫描）
    fn recalculate_ask_min(&mut self) {
        self.ask_min = None;
        for price in 0..self.asks.len() {
            if !self.asks[price].is_empty() {
                self.ask_min = Some(price as Price);
                return;
            }
        }
    }
}

impl OrderRepository for InMemoryOrderRepository {
    //good
    fn add_order(
        &mut self,
        order_id: OrderId,
        entry: OrderEntry,
        side: Side,
        price: Price,
    ) -> Result<(), RepositoryError> {
        // 检查订单是否已存在
        if self.order_index.contains_key(&order_id) {
            return Err(RepositoryError::OrderAlreadyExists);
        }

        // 分配内存
        let idx = self
            .arena
            .allocate(entry)
            .ok_or(RepositoryError::CapacityExceeded)?;

        // 记录索引
        self.order_index.insert(order_id, idx);

        // 获取价格点的 last_idx（避免借用冲突）
        let last_idx = {
            let price_point = self
                .get_price_point(price, side)
                .ok_or(RepositoryError::PriceOutOfRange)?;
            price_point.last_order_idx
        };

        // 链接到现有订单 完成插入操作
        if let Some(last_idx) = last_idx {
            if let Some(last_entry) = self.arena.get_mut(last_idx) {
                last_entry.next_idx = Some(idx);
            }
        }

        // 更新价格点
        let price_point = self
            .get_price_point_mut(price, side)
            .ok_or(RepositoryError::PriceOutOfRange)?;
        price_point.push_back(idx);

        // 更新最佳买卖价缓存
        match side {
            Side::Buy => {
                // 更新最高买价
                if self.bid_max.is_none() || price > self.bid_max.unwrap() {
                    self.bid_max = Some(price);
                }
            }
            Side::Sell => {
                // 更新最低卖价
                if self.ask_min.is_none() || price < self.ask_min.unwrap() {
                    self.ask_min = Some(price);
                }
            }
        }

        Ok(())
    }

    fn match_orders(
        &self,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> Option<Vec<&OrderEntry>> {
        // 根据 side,price,quantity 匹配所有的Order
        // quantity总和要大于等于quantity, 返回匹配上的订单数组

        let mut matched_orders = Vec::new();
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
                    for current_price in ask_min..=price {
                        if remaining == 0 {
                            break;
                        }

                        if let Some(first_idx) =
                            self.get_first_order_at_price(current_price, opposite_side)
                        {
                            let mut current_idx = Some(first_idx);

                            while remaining > 0 && current_idx.is_some() {
                                let idx = current_idx.unwrap();

                                if let Some(entry) = self.arena.get(idx) {
                                    if entry.is_active() && entry.unfilled_quantity > 0 {
                                        let fill_qty = remaining.min(entry.unfilled_quantity);
                                        remaining -= fill_qty;
                                        matched_orders.push(entry);
                                    }
                                    current_idx = entry.next_idx;
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
                    for current_price in (price..=bid_max).rev() {
                        if remaining == 0 {
                            break;
                        }

                        if let Some(first_idx) =
                            self.get_first_order_at_price(current_price, opposite_side)
                        {
                            let mut current_idx = Some(first_idx);

                            while remaining > 0 && current_idx.is_some() {
                                let idx = current_idx.unwrap();

                                if let Some(entry) = self.arena.get(idx) {
                                    if entry.is_active() && entry.unfilled_quantity > 0 {
                                        let fill_qty = remaining.min(entry.unfilled_quantity);
                                        remaining -= fill_qty;
                                        matched_orders.push(entry);
                                    }
                                    current_idx = entry.next_idx;
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

    //good
    fn find_order(&self, order_id: OrderId) -> Option<&OrderEntry> {
        self.order_index
            .get(&order_id)
            .and_then(|&idx| self.arena.get(idx))
    }

    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut OrderEntry> {
        self.order_index
            .get(&order_id)
            .and_then(|&idx| self.arena.get_mut(idx))
    }

    fn cancel_order(&mut self, order_id: OrderId) -> bool {
        if let Some(&idx) = self.order_index.get(&order_id) {
            if let Some(entry) = self.arena.get_mut(idx) {
                entry.cancel();
                self.order_index.remove(&order_id);
                // 注意：不立即free，因为订单可能在链表中间
                // 会在update_price_point时批量回收链表前面的inactive订单
                return true;
            }
        }
        false
    }

    fn get_first_order_at_price(&self, price: Price, side: Side) -> Option<usize> {
        self.get_price_point(price, side)
            .and_then(|pp| pp.first_order_idx)
    }

    fn get_next_order(&self, idx: usize) -> Option<usize> {
        self.arena.get(idx).and_then(|entry| entry.next_idx)
    }

    fn update_price_point(
        &mut self,
        price: Price,
        side: Side,
        first_idx: Option<usize>,
        last_idx: Option<usize>,
    ) {
        // 在更新价格点之前，先批量回收链表前面的inactive订单
        // 这些订单通常是完全成交的订单，按FIFO顺序排在链表前面
        if let Some(old_first_idx) = self
            .get_price_point(price, side)
            .and_then(|pp| pp.first_order_idx)
        {
            let mut current_idx = Some(old_first_idx);

            // 遍历链表前面的inactive订单并回收
            while let Some(idx) = current_idx {
                if let Some(entry) = self.arena.get(idx) {
                    if !entry.is_active() {
                        // 记录next_idx（因为下面要free当前节点）
                        let next = entry.next_idx;
                        // 回收这个inactive订单的arena槽位
                        self.arena.free(idx);
                        current_idx = next;
                    } else {
                        // 遇到第一个活跃订单，停止回收
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if let Some(price_point) = self.get_price_point_mut(price, side) {
            price_point.first_order_idx = first_idx;
            price_point.last_order_idx = last_idx;
        }

        // 如果价格级别变空，可能需要重新计算最佳价格
        if first_idx.is_none() && last_idx.is_none() {
            match side {
                Side::Buy => {
                    // 如果清空的是最佳买价，需要重新查找
                    if Some(price) == self.bid_max {
                        self.recalculate_bid_max();
                    }
                }
                Side::Sell => {
                    // 如果清空的是最佳卖价，需要重新查找
                    if Some(price) == self.ask_min {
                        self.recalculate_ask_min();
                    }
                }
            }
        }
    }

    fn is_price_empty(&self, price: Price, side: Side) -> bool {
        self.get_price_point(price, side)
            .map_or(true, |pp| pp.is_empty())
    }

    fn active_order_count(&self) -> usize {
        self.order_index.len()
    }

    fn allocate_order_id(&mut self) -> OrderId {
        let id = self.next_order_id;
        self.next_order_id += 1;
        id
    }

    fn next_order_id(&self) -> OrderId {
        self.next_order_id
    }

    fn set_next_order_id(&mut self, id: OrderId) {
        self.next_order_id = id;
    }

    fn best_bid(&self) -> Option<Price> {
        self.bid_max
    }

    fn best_ask(&self) -> Option<Price> {
        self.ask_min
    }

    fn replay(&mut self, events: Vec<EntityEvent>) -> Result<(), RepositoryError> {
        for event in events {
            match (event.entity_name, event.operation) {
                ("Order", EventOperation::Create) => {
                    let order_entry: OrderEntry = trans(event);
                    // self.add_order(order_entry);
                    // 处理订单创建事件
                }
                ("Order", EventOperation::Update) => {
                    // 处理订单更新事件
                    for change in event.changes {
                        let order_id = change.entity_id;

                        for field in change.field_changes {
                            if field.field_name == "unfilled_quantity" {
                                if let Some(FieldValue::U32(new_qty)) = field.new_value {
                                    // 更新订单数量
                                    if let Some(entry) = self.find_order_mut(order_id) {
                                        entry.unfilled_quantity = new_qty;
                                    }
                                }
                            }
                        }
                    }
                }
                ("Order", EventOperation::Delete) => {
                    // 处理订单删除事件
                    for change in event.changes {
                        let order_id = change.entity_id;
                        // cancel_order内部已经调用了arena.free()
                        self.cancel_order(order_id);
                    }
                }
                _ => {
                    // 忽略其他事件类型（如Trade）
                }
            }
        }
        Ok(())
    }
}

fn trans(entity_event: EntityEvent) -> OrderEntry {
    //将EntityEvent 转成OrderEntry
    todo!()
}

impl RepositoryAccessor for InMemoryOrderRepository {
    fn get_entry(&self, idx: usize) -> Option<&OrderEntry> {
        self.arena.get(idx)
    }

    fn get_entry_mut(&mut self, idx: usize) -> Option<&mut OrderEntry> {
        self.arena.get_mut(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lob::domain::entity::lob_types::TraderId;

    #[test]
    fn test_add_and_find_order() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");
        let order_id = 1;
        let entry = OrderEntry::new(order_id, trader, 100);

        // 添加订单
        let result = repo.add_order(order_id, entry, Side::Buy, 10000);
        assert!(result.is_ok());

        // 查找订单
        let found = repo.find_order(order_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().unfilled_quantity, 100);
    }

    #[test]
    fn test_cancel_order() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");
        let order_id = 1;
        let entry = OrderEntry::new(order_id, trader, 100);

        repo.add_order(order_id, entry, Side::Buy, 10000).unwrap();

        // 取消订单
        assert!(repo.cancel_order(order_id));

        // 再次取消应该失败
        assert!(!repo.cancel_order(order_id));
    }

    #[test]
    fn test_order_id_allocation() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);

        assert_eq!(repo.next_order_id(), 1);

        let id1 = repo.allocate_order_id();
        assert_eq!(id1, 1);
        assert_eq!(repo.next_order_id(), 2);

        let id2 = repo.allocate_order_id();
        assert_eq!(id2, 2);
        assert_eq!(repo.next_order_id(), 3);
    }

    #[test]
    fn test_price_point_empty() {
        let repo = InMemoryOrderRepository::new(100_000, 1000);

        // 空仓储中所有价格点都应该为空
        assert!(repo.is_price_empty(10000, Side::Buy));
        assert!(repo.is_price_empty(10000, Side::Sell));
    }

    #[test]
    fn test_duplicate_order() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");
        let order_id = 1;
        let entry1 = OrderEntry::new(order_id, trader, 100);
        let entry2 = OrderEntry::new(order_id, trader, 200);

        // 第一次添加成功
        assert!(repo.add_order(order_id, entry1, Side::Buy, 10000).is_ok());

        // 第二次添加相同ID应该失败
        let result = repo.add_order(order_id, entry2, Side::Buy, 10000);
        assert_eq!(result, Err(RepositoryError::OrderAlreadyExists));
    }

    #[test]
    fn test_best_bid_ask_initial_state() {
        let repo = InMemoryOrderRepository::new(100_000, 1000);

        // 初始状态应该都是 None
        assert_eq!(repo.best_bid(), None);
        assert_eq!(repo.best_ask(), None);
    }

    #[test]
    fn test_best_bid_updates_on_add() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 添加第一个买单
        let order1 = OrderEntry::new(1, trader, 100);
        repo.add_order(1, order1, Side::Buy, 9900).unwrap();
        assert_eq!(repo.best_bid(), Some(9900));

        // 添加更高价买单（应该更新）
        let order2 = OrderEntry::new(2, trader, 100);
        repo.add_order(2, order2, Side::Buy, 10000).unwrap();
        assert_eq!(repo.best_bid(), Some(10000));

        // 添加更低价买单（不应该更新）
        let order3 = OrderEntry::new(3, trader, 100);
        repo.add_order(3, order3, Side::Buy, 9800).unwrap();
        assert_eq!(repo.best_bid(), Some(10000));
    }

    #[test]
    fn test_best_ask_updates_on_add() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 添加第一个卖单
        let order1 = OrderEntry::new(1, trader, 100);
        repo.add_order(1, order1, Side::Sell, 10100).unwrap();
        assert_eq!(repo.best_ask(), Some(10100));

        // 添加更低价卖单（应该更新）
        let order2 = OrderEntry::new(2, trader, 100);
        repo.add_order(2, order2, Side::Sell, 10000).unwrap();
        assert_eq!(repo.best_ask(), Some(10000));

        // 添加更高价卖单（不应该更新）
        let order3 = OrderEntry::new(3, trader, 100);
        repo.add_order(3, order3, Side::Sell, 10200).unwrap();
        assert_eq!(repo.best_ask(), Some(10000));
    }

    #[test]
    fn test_best_bid_recalculates_on_clear() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 添加多个买单
        repo.add_order(1, OrderEntry::new(1, trader, 100), Side::Buy, 9800)
            .unwrap();
        repo.add_order(2, OrderEntry::new(2, trader, 100), Side::Buy, 9900)
            .unwrap();
        repo.add_order(3, OrderEntry::new(3, trader, 100), Side::Buy, 10000)
            .unwrap();
        assert_eq!(repo.best_bid(), Some(10000));

        // 清空最佳买价级别（模拟订单完全成交）
        repo.update_price_point(10000, Side::Buy, None, None);

        // 应该自动重新计算为次优买价
        assert_eq!(repo.best_bid(), Some(9900));

        // 再次清空
        repo.update_price_point(9900, Side::Buy, None, None);
        assert_eq!(repo.best_bid(), Some(9800));

        // 清空最后一个
        repo.update_price_point(9800, Side::Buy, None, None);
        assert_eq!(repo.best_bid(), None);
    }

    #[test]
    fn test_best_ask_recalculates_on_clear() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 添加多个卖单
        repo.add_order(1, OrderEntry::new(1, trader, 100), Side::Sell, 10000)
            .unwrap();
        repo.add_order(2, OrderEntry::new(2, trader, 100), Side::Sell, 10100)
            .unwrap();
        repo.add_order(3, OrderEntry::new(3, trader, 100), Side::Sell, 10200)
            .unwrap();
        assert_eq!(repo.best_ask(), Some(10000));

        // 清空最佳卖价级别
        repo.update_price_point(10000, Side::Sell, None, None);

        // 应该自动重新计算为次优卖价
        assert_eq!(repo.best_ask(), Some(10100));

        // 再次清空
        repo.update_price_point(10100, Side::Sell, None, None);
        assert_eq!(repo.best_ask(), Some(10200));

        // 清空最后一个
        repo.update_price_point(10200, Side::Sell, None, None);
        assert_eq!(repo.best_ask(), None);
    }

    #[test]
    fn test_best_bid_ask_independent() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 添加买单不应影响卖单
        repo.add_order(1, OrderEntry::new(1, trader, 100), Side::Buy, 9900)
            .unwrap();
        assert_eq!(repo.best_bid(), Some(9900));
        assert_eq!(repo.best_ask(), None);

        // 添加卖单不应影响买单
        repo.add_order(2, OrderEntry::new(2, trader, 100), Side::Sell, 10100)
            .unwrap();
        assert_eq!(repo.best_bid(), Some(9900));
        assert_eq!(repo.best_ask(), Some(10100));

        // 清空买单不应影响卖单
        repo.update_price_point(9900, Side::Buy, None, None);
        assert_eq!(repo.best_bid(), None);
        assert_eq!(repo.best_ask(), Some(10100));
    }

    #[test]
    fn test_invariant_bid_less_than_ask() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 添加正常的买卖价差
        repo.add_order(1, OrderEntry::new(1, trader, 100), Side::Buy, 9900)
            .unwrap();
        repo.add_order(2, OrderEntry::new(2, trader, 100), Side::Sell, 10100)
            .unwrap();

        // 验证不变式：bid_max <= ask_min
        assert!(repo.best_bid().unwrap() <= repo.best_ask().unwrap());

        // 添加更接近的价格
        repo.add_order(3, OrderEntry::new(3, trader, 100), Side::Buy, 9999)
            .unwrap();
        repo.add_order(4, OrderEntry::new(4, trader, 100), Side::Sell, 10001)
            .unwrap();

        // 仍然满足不变式
        assert!(repo.best_bid().unwrap() <= repo.best_ask().unwrap());
    }

    #[test]
    fn test_invariant_after_clear() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 建立多层订单簿
        repo.add_order(1, OrderEntry::new(1, trader, 100), Side::Buy, 9900)
            .unwrap();
        repo.add_order(2, OrderEntry::new(2, trader, 100), Side::Buy, 9950)
            .unwrap();
        repo.add_order(3, OrderEntry::new(3, trader, 100), Side::Sell, 10050)
            .unwrap();
        repo.add_order(4, OrderEntry::new(4, trader, 100), Side::Sell, 10100)
            .unwrap();

        assert_eq!(repo.best_bid(), Some(9950));
        assert_eq!(repo.best_ask(), Some(10050));

        // 清空最佳买价
        repo.update_price_point(9950, Side::Buy, None, None);

        // 不变式仍然成立
        assert_eq!(repo.best_bid(), Some(9900));
        assert_eq!(repo.best_ask(), Some(10050));
        assert!(repo.best_bid().unwrap() < repo.best_ask().unwrap());
    }

    #[test]
    fn test_invariant_valid_spread() {
        let trader = TraderId::from_str("TRADER1");

        // 测试有效价差的各种情况
        let test_cases = vec![
            (9000, 11000), // 大价差
            (9900, 10100), // 正常价差
            (9999, 10000), // 最小价差 (差1)
        ];

        for (bid_price, ask_price) in test_cases {
            let mut test_repo = InMemoryOrderRepository::new(100_000, 1000);
            test_repo
                .add_order(1, OrderEntry::new(1, trader, 100), Side::Buy, bid_price)
                .unwrap();
            test_repo
                .add_order(2, OrderEntry::new(2, trader, 100), Side::Sell, ask_price)
                .unwrap();

            // 所有情况都应该满足 bid <= ask
            assert!(
                test_repo.best_bid().unwrap() <= test_repo.best_ask().unwrap(),
                "bid={} should be <= ask={}",
                test_repo.best_bid().unwrap(),
                test_repo.best_ask().unwrap()
            );
        }
    }

    #[test]
    fn test_match_Orders() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("SELLER1");

        // 添加多个卖单
        let sell1 = OrderEntry::new(1, trader, 50);
        repo.add_order(1, sell1, Side::Sell, 10000).unwrap();

        let sell2 = OrderEntry::new(2, trader, 60);
        repo.add_order(2, sell2, Side::Sell, 10000).unwrap();

        let sell3 = OrderEntry::new(3, trader, 40);
        repo.add_order(3, sell3, Side::Sell, 10100).unwrap();

        // 测试买单匹配：需要100数量
        let matched = repo.match_orders(Side::Buy, 10100, 100);

        assert!(matched.is_some());
        let orders = matched.unwrap();

        // 应该匹配2个订单（50 + 50 = 100）
        assert_eq!(orders.len(), 2);

        // 验证第一个订单
        assert_eq!(orders[0].order_id, 1);
        assert_eq!(orders[0].unfilled_quantity, 50);

        // 验证第二个订单
        assert_eq!(orders[1].order_id, 2);
        assert_eq!(orders[1].unfilled_quantity, 60);

        // 验证总数量 >= 100
        let total: u32 = orders.iter().map(|o| o.unfilled_quantity).sum();
        assert!(total >= 100);
    }
}
