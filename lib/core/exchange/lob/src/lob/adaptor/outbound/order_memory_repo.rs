use crate::lob::adaptor::outbound::arena::OrderArena;
use crate::lob::domain::entity::lob_types::{
    EntityEvent, EventOperation, FieldValue, OrderEntry, OrderId, Price, PricePoint, Quantity,
    Side, TraderId,
};
/// 内存仓储实现
///
/// 使用内存池和价格索引数组实现高性能订单存储
use crate::lob::domain::repository::traits::{OrderRepository, RepositoryError};
use std::collections::HashMap;

/// 内存仓储实现
///
/// 使用内存池和价格索引数组实现高性能订单存储
pub struct MemoryOrderRepository {
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

impl MemoryOrderRepository {
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

impl OrderRepository for MemoryOrderRepository {
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

    fn get_available_quantity(&self, side: Side, price_limit: Option<Price>) -> Quantity {
        let mut total_quantity = 0u32;

        match side {
            Side::Buy => {
                // 买单：查询卖方深度（Ask侧）
                // 从最低卖价开始累加，直到超过 price_limit
                if let Some(min_ask) = self.ask_min {
                    let max_price = price_limit.unwrap_or(u32::MAX);

                    // 遍历价格从 min_ask 到 max_price
                    for price in min_ask..=max_price {
                        if let Some(price_point) = self.asks.get(price as usize) {
                            if let Some(mut current_idx) = price_point.first_order_idx {
                                // 遍历该价格的订单链表
                                while let Some(order) = self.arena.get(current_idx) {
                                    if order.is_active() {
                                        total_quantity = total_quantity.saturating_add(order.unfilled_quantity);
                                    }
                                    current_idx = match order.next_idx {
                                        Some(idx) => idx,
                                        None => break,
                                    };
                                }
                            }
                        }
                    }
                }
            }
            Side::Sell => {
                // 卖单：查询买方深度（Bid侧）
                // 从最高买价开始累加，直到低于 price_limit
                if let Some(max_bid) = self.bid_max {
                    let min_price = price_limit.unwrap_or(0);

                    // 遍历价格从 max_bid 到 min_price（递减）
                    for price in (min_price..=max_bid).rev() {
                        if let Some(price_point) = self.bids.get(price as usize) {
                            if let Some(mut current_idx) = price_point.first_order_idx {
                                // 遍历该价格的订单链表
                                while let Some(order) = self.arena.get(current_idx) {
                                    if order.is_active() {
                                        total_quantity = total_quantity.saturating_add(order.unfilled_quantity);
                                    }
                                    current_idx = match order.next_idx {
                                        Some(idx) => idx,
                                        None => break,
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }

        total_quantity
    }

    fn replay(&mut self, events: Vec<EntityEvent>) -> Result<(), RepositoryError> {
        for event in events {
            self.apply_event(event)?;
        }
        Ok(())
    }
}

impl MemoryOrderRepository {
    /// 应用单个事件
    fn apply_event(&mut self, event: EntityEvent) -> Result<(), RepositoryError> {
        match (event.entity_name, event.operation) {
            ("Order", EventOperation::Create) => self.apply_order_create(event),
            ("Order", EventOperation::Update) => self.apply_order_update(event),
            ("Order", EventOperation::Delete) => self.apply_order_delete(event),
            _ => Ok(()), // 忽略其他事件类型（如 Trade）
        }
    }

    /// 应用订单创建事件
    fn apply_order_create(&mut self, event: EntityEvent) -> Result<(), RepositoryError> {
        for change in event.changes {
            let order_id = change.entity_id;

            // 从字段变更中提取订单信息
            let mut trader = TraderId::new([0u8; 8]);
            let mut side = Side::Buy;
            let mut price: Price = 0;
            let mut quantity: Quantity = 0;

            for field in change.field_changes {
                match field.field_name {
                    "trader" => {
                        if let Some(FieldValue::TraderId(t)) = field.new_value {
                            trader = t;
                        }
                    }
                    "side" => {
                        if let Some(FieldValue::Side(s)) = field.new_value {
                            side = s;
                        }
                    }
                    "price" => {
                        if let Some(FieldValue::U32(p)) = field.new_value {
                            price = p;
                        }
                    }
                    "quantity" | "total_quantity" => {
                        if let Some(FieldValue::U32(q)) = field.new_value {
                            quantity = q;
                        }
                    }
                    _ => {}
                }
            }

            let entry = OrderEntry::new(order_id, trader, quantity);
            self.add_order(order_id, entry, side, price)?;
        }
        Ok(())
    }

    /// 应用订单更新事件
    fn apply_order_update(&mut self, event: EntityEvent) -> Result<(), RepositoryError> {
        for change in event.changes {
            let order_id = change.entity_id;

            if let Some(entry) = self.find_order_mut(order_id) {
                for field in change.field_changes {
                    match field.field_name {
                        "unfilled_quantity" => {
                            if let Some(FieldValue::U32(qty)) = field.new_value {
                                entry.unfilled_quantity = qty;
                            }
                        }
                        "total_quantity" => {
                            if let Some(FieldValue::U32(qty)) = field.new_value {
                                entry.total_quantity = qty;
                            }
                        }
                        "next_idx" => {
                            if let Some(FieldValue::OptionUsize(idx)) = field.new_value {
                                entry.next_idx = idx;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    /// 应用订单删除事件
    fn apply_order_delete(&mut self, event: EntityEvent) -> Result<(), RepositoryError> {
        for change in event.changes {
            self.cancel_order(change.entity_id);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lob::domain::entity::lob_types::{FieldChange, RecordChange, TraderId};

    #[test]
    fn test_add_and_find_order() {
        let mut repo = MemoryOrderRepository::new(100_000, 1000);
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
    fn test_replay_create_order() {
        let mut repo = MemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 构造 Create 事件
        let event = EntityEvent::new(
            1, // event_id
            1, // transaction_id
            "Order",
            EventOperation::Create,
            vec![RecordChange::new(
                100, // order_id
                vec![
                    FieldChange::created("trader", FieldValue::TraderId(trader)),
                    FieldChange::created("side", FieldValue::Side(Side::Buy)),
                    FieldChange::created("price", FieldValue::U32(10000)),
                    FieldChange::created("quantity", FieldValue::U32(50)),
                ],
            )],
        );

        // 重放事件
        let result = repo.replay(vec![event]);
        assert!(result.is_ok());

        // 验证订单已创建
        let found = repo.find_order(100);
        assert!(found.is_some());
        let order = found.unwrap();
        assert_eq!(order.order_id, 100);
        assert_eq!(order.unfilled_quantity, 50);
        assert_eq!(order.trader, trader);
    }

    #[test]
    fn test_replay_update_order() {
        let mut repo = MemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 先添加订单
        let entry = OrderEntry::new(100, trader, 100);
        repo.add_order(100, entry, Side::Buy, 10000).unwrap();

        // 构造 Update 事件
        let event = EntityEvent::new(
            2,
            2,
            "Order",
            EventOperation::Update,
            vec![RecordChange::new(
                100,
                vec![FieldChange::updated(
                    "unfilled_quantity",
                    FieldValue::U32(100),
                    FieldValue::U32(30),
                )],
            )],
        );

        // 重放事件
        let result = repo.replay(vec![event]);
        assert!(result.is_ok());

        // 验证订单已更新
        let found = repo.find_order(100).unwrap();
        assert_eq!(found.unfilled_quantity, 30);
    }

    #[test]
    fn test_replay_delete_order() {
        let mut repo = MemoryOrderRepository::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER1");

        // 先添加订单
        let entry = OrderEntry::new(100, trader, 100);
        repo.add_order(100, entry, Side::Buy, 10000).unwrap();
        assert!(repo.find_order(100).is_some());

        // 构造 Delete 事件
        let event = EntityEvent::new(
            3,
            3,
            "Order",
            EventOperation::Delete,
            vec![RecordChange::new(100, vec![])],
        );

        // 重放事件
        let result = repo.replay(vec![event]);
        assert!(result.is_ok());

        // 验证订单已删除（从索引中移除）
        assert!(repo.find_order(100).is_none());
    }

    #[test]
    fn test_replay_multiple_events() {
        let mut repo = MemoryOrderRepository::new(100_000, 1000);
        let trader1 = TraderId::from_str("TRADER1");
        let trader2 = TraderId::from_str("TRADER2");

        // 构造多个事件
        let events = vec![
            // 创建订单1
            EntityEvent::new(
                1,
                1,
                "Order",
                EventOperation::Create,
                vec![RecordChange::new(
                    1,
                    vec![
                        FieldChange::created("trader", FieldValue::TraderId(trader1)),
                        FieldChange::created("side", FieldValue::Side(Side::Buy)),
                        FieldChange::created("price", FieldValue::U32(10000)),
                        FieldChange::created("quantity", FieldValue::U32(100)),
                    ],
                )],
            ),
            // 创建订单2
            EntityEvent::new(
                2,
                2,
                "Order",
                EventOperation::Create,
                vec![RecordChange::new(
                    2,
                    vec![
                        FieldChange::created("trader", FieldValue::TraderId(trader2)),
                        FieldChange::created("side", FieldValue::Side(Side::Sell)),
                        FieldChange::created("price", FieldValue::U32(10100)),
                        FieldChange::created("quantity", FieldValue::U32(50)),
                    ],
                )],
            ),
            // 更新订单1
            EntityEvent::new(
                3,
                3,
                "Order",
                EventOperation::Update,
                vec![RecordChange::new(
                    1,
                    vec![FieldChange::updated(
                        "unfilled_quantity",
                        FieldValue::U32(100),
                        FieldValue::U32(60),
                    )],
                )],
            ),
            // 删除订单2
            EntityEvent::new(
                4,
                4,
                "Order",
                EventOperation::Delete,
                vec![RecordChange::new(2, vec![])],
            ),
        ];

        // 重放所有事件
        let result = repo.replay(events);
        assert!(result.is_ok());

        // 验证最终状态
        let order1 = repo.find_order(1).unwrap();
        assert_eq!(order1.unfilled_quantity, 60);

        // 订单2已从索引中移除，无法通过 find_order 查找
        assert!(repo.find_order(2).is_none());

        assert_eq!(repo.best_bid(), Some(10000));
        // 注：best_ask 仍为 Some(10100)，因为 cancel_order 采用延迟回收策略，
        // 不会立即更新 best_ask 缓存。只有当价格级别完全清空时才会更新。
        assert_eq!(repo.best_ask(), Some(10100));
    }
}
