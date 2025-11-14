/// 仓储接口定义
use crate::lob::types::{OrderEntry, OrderId, Price, Side};


/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
pub trait OrderRepository {
    /// 添加订单到仓储
    fn add_order(
        &mut self,
        order_id: OrderId,
        entry: OrderEntry,
        side: Side,
        price: Price,
    ) -> Result<(), RepositoryError>;

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
    fn update_price_point(
        &mut self,
        price: Price,
        side: Side,
        first_idx: Option<usize>,
        last_idx: Option<usize>,
    );

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

    /// 获取最佳买价（O(1) 缓存访问）
    fn best_bid(&self) -> Option<Price>;

    /// 获取最佳卖价（O(1) 缓存访问）
    fn best_ask(&self) -> Option<Price>;
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

/// 仓储错误类型

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
