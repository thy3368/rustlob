use crate::lob::adaptor::outbound::arena::OrderArena;
use crate::lob::domain::entity::lob_types::{
    EntityEvent, EventOperation, FieldValue, OrderEntry, OrderId, Price, PricePoint, Quantity, Side,
};
/// 内存仓储实现
///
/// 使用内存池和价格索引数组实现高性能订单存储
use crate::lob::domain::repository::traits::{OrderRepository, RepositoryError};
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

    // === 内部方法（不对外暴露）===

    /// 获取指定价格级别的第一个订单索引
    #[inline]
    fn get_first_order_at_price(&self, price: Price, side: Side) -> Option<usize> {
        self.get_price_point(price, side)
            .and_then(|pp| pp.first_order_idx)
    }

    /// 将订单链接到价格级别的链表尾部
    fn link_order_to_price_level(&mut self, idx: usize, price: Price, side: Side) {
        // 获取当前链表尾部
        let last_idx = self
            .get_price_point(price, side)
            .and_then(|pp| pp.last_order_idx);

        // 链接到现有尾部订单
        if let Some(last_idx) = last_idx {
            if let Some(last_entry) = self.arena.get_mut(last_idx) {
                last_entry.next_idx = Some(idx);
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

impl OrderRepository for InMemoryOrderRepository {
    fn add_order(
        &mut self,
        order_id: OrderId,
        entry: OrderEntry,
        side: Side,
        price: Price,
    ) -> Result<(), RepositoryError> {
        // === 1. 前置验证（不分配资源）===
        if self.order_index.contains_key(&order_id) {
            return Err(RepositoryError::OrderAlreadyExists);
        }
        if self.get_price_point(price, side).is_none() {
            return Err(RepositoryError::PriceOutOfRange);
        }

        // === 2. 分配 arena 槽位 ===
        let idx = self
            .arena
            .allocate(entry)
            .ok_or(RepositoryError::CapacityExceeded)?;

        // === 3. 更新索引和链表 ===
        self.order_index.insert(order_id, idx);
        self.link_order_to_price_level(idx, price, side);

        // === 4. 更新最佳价格缓存 ===
        self.update_best_price(price, side);

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

    fn active_order_count(&self) -> usize {
        self.order_index.len()
    }

    fn allocate_order_id(&mut self) -> OrderId {
        let id = self.next_order_id;
        self.next_order_id += 1;
        id
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
}
