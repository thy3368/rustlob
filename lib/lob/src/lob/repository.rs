/// 订单仓储接口和实现
///
/// 遵循Clean Architecture的仓储模式，将数据访问逻辑与业务逻辑分离

use super::types::{OrderEntry, OrderId, Price, PricePoint, Side};
use super::arena::OrderArena;
use std::collections::HashMap;

/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
pub trait OrderRepository {
    /// 添加订单到仓储
    fn add_order(&mut self, order_id: OrderId, entry: OrderEntry, side: Side, price: Price) -> Result<(), RepositoryError>;

    /// 根据订单ID查找订单
    fn find_order(&self, order_id: OrderId) -> Option<&OrderEntry>;

    /// 根据订单ID查找订单（可变引用）
    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut OrderEntry>;

    /// 取消订单
    fn cancel_order(&mut self, order_id: OrderId) -> bool;

    /// 获取指定价格级别的第一个订单索引
    fn get_first_order_at_price(&self, price: Price, side: Side) -> Option<usize>;

    /// 获取订单的下一个订单索引
    fn get_next_order(&self, idx: usize) -> Option<usize>;

    /// 更新价格点的首个订单索引
    fn update_price_point(&mut self, price: Price, side: Side, first_idx: Option<usize>, last_idx: Option<usize>);

    /// 检查价格级别是否为空
    fn is_price_empty(&self, price: Price, side: Side) -> bool;

    /// 获取活跃订单数量
    fn active_order_count(&self) -> usize;

    /// 分配订单ID
    fn allocate_order_id(&mut self) -> OrderId;

    /// 获取下一个订单ID（不分配）
    fn next_order_id(&self) -> OrderId;

    /// 设置下一个订单ID（用于状态恢复）
    fn set_next_order_id(&mut self, id: OrderId);
}

/// 仓储错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    /// 容量已满
    CapacityExceeded,
    /// 订单已存在
    OrderAlreadyExists,
    /// 订单未找到
    OrderNotFound,
    /// 价格超出范围
    PriceOutOfRange,
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::CapacityExceeded => write!(f, "订单容量已满"),
            RepositoryError::OrderAlreadyExists => write!(f, "订单已存在"),
            RepositoryError::OrderNotFound => write!(f, "订单未找到"),
            RepositoryError::PriceOutOfRange => write!(f, "价格超出范围"),
        }
    }
}

impl std::error::Error for RepositoryError {}

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
}

impl OrderRepository for InMemoryOrderRepository {
    fn add_order(&mut self, order_id: OrderId, entry: OrderEntry, side: Side, price: Price) -> Result<(), RepositoryError> {
        // 检查订单是否已存在
        if self.order_index.contains_key(&order_id) {
            return Err(RepositoryError::OrderAlreadyExists);
        }

        // 分配内存
        let idx = self.arena
            .allocate(entry)
            .ok_or(RepositoryError::CapacityExceeded)?;

        // 记录索引
        self.order_index.insert(order_id, idx);

        // 获取价格点的 last_idx（避免借用冲突）
        let last_idx = {
            let price_point = self.get_price_point(price, side)
                .ok_or(RepositoryError::PriceOutOfRange)?;
            price_point.last_order_idx
        };

        // 链接到现有订单
        if let Some(last_idx) = last_idx {
            if let Some(last_entry) = self.arena.get_mut(last_idx) {
                last_entry.next_idx = Some(idx);
            }
        }

        // 更新价格点
        let price_point = self.get_price_point_mut(price, side)
            .ok_or(RepositoryError::PriceOutOfRange)?;
        price_point.push_back(idx);

        Ok(())
    }

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

    fn update_price_point(&mut self, price: Price, side: Side, first_idx: Option<usize>, last_idx: Option<usize>) {
        if let Some(price_point) = self.get_price_point_mut(price, side) {
            price_point.first_order_idx = first_idx;
            price_point.last_order_idx = last_idx;
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
}

/// 仓储访问器trait
///
/// 用于从仓储中获取订单条目的只读访问
pub trait RepositoryAccessor {
    /// 获取订单条目的不可变引用
    fn get_entry(&self, idx: usize) -> Option<&OrderEntry>;

    /// 获取订单条目的可变引用
    fn get_entry_mut(&mut self, idx: usize) -> Option<&mut OrderEntry>;
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
    use crate::lob::types::{TraderId, OrderEntry};

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
        assert_eq!(found.unwrap().quantity, 100);
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
}
