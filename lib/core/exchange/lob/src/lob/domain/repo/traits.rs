/// 仓储接口定义
use crate::lob::domain::entity::lob_types::{EntityEvent, OrderEntry, OrderId, Price, Quantity, Side};


/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
/// 仅暴露业务层需要的操作，内部实现细节（如链表遍历、价格点管理）由具体实现封装
pub trait OrderRepo {
    // === 核心写操作 ===

    /// 添加订单到仓储
    fn add_order(
        &mut self,
        order_id: OrderId,
        entry: OrderEntry,
        side: Side,
        price: Price,
    ) -> Result<(), RepositoryError>;

    /// 取消订单
    fn cancel_order(&mut self, order_id: OrderId) -> bool;

    // === 核心读操作 ===

    /// 根据订单ID查找订单
    fn find_order(&self, order_id: OrderId) -> Option<&OrderEntry>;

    /// 根据订单ID查找订单（可变引用）
    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut OrderEntry>;

    /// 匹配订单，返回匹配到的订单引用列表
    ///
    /// # 参数
    /// - `side`: 订单方向（买/卖）
    /// - `price`: 价格
    /// - `quantity`: 需要匹配的数量
    ///
    /// # 返回
    /// - `Some(Vec<&OrderEntry>)`: 匹配到的订单列表（总数量 >= quantity）
    /// - `None`: 无法匹配
    fn match_orders(&self, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&OrderEntry>>;

    // === 市场数据查询 ===

    /// 获取最佳买价（O(1) 缓存访问）
    fn best_bid(&self) -> Option<Price>;

    /// 获取最佳卖价（O(1) 缓存访问）
    fn best_ask(&self) -> Option<Price>;

    /// 获取指定方向和价格限制下的可用数量
    ///
    /// # 参数
    /// - `side`: 订单方向（Buy=查询卖方深度, Sell=查询买方深度）
    /// - `price_limit`: 价格限制（买单最高价/卖单最低价），None表示无限制
    ///
    /// # 返回
    /// 可用的总数量
    fn get_available_quantity(&self, side: Side, price_limit: Option<Price>) -> Quantity;

    /// 获取最佳卖价（别名方法，用于市价单逻辑）
    fn get_best_ask(&self) -> Option<Price> {
        self.best_ask()
    }

    /// 获取活跃订单数量
    fn active_order_count(&self) -> usize;

    // === 订单ID管理 ===

    /// 分配订单ID
    fn allocate_order_id(&mut self) -> OrderId;

    // === 事件溯源 ===

    /// 重放事件列表，将事件应用到仓储状态
    ///
    /// # 参数
    /// - `events`: 事件列表（按event_id顺序）
    ///
    /// # 返回
    /// - `Ok(())`: 成功应用所有事件
    /// - `Err(RepositoryError)`: 应用事件失败
    fn replay(&mut self, events: Vec<EntityEvent>) -> Result<(), RepositoryError>;
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
