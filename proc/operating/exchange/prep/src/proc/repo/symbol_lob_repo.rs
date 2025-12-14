use account::{OrderId, Price, Quantity, Side, Symbol};

use crate::proc::prep_types::InternalOrder;

/// 仓储接口定义
// use crate::lob::domain::entity::lob_types::{EntityEvent, OrderEntry, OrderId, Price, Quantity, Side};

/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
/// 仅暴露业务层需要的操作，内部实现细节（如链表遍历、价格点管理）由具体实现封装
pub trait SymbolLobRepo {
    /// 匹配订单，返回匹配到的订单引用列表
    ///
    /// # 参数
    /// - `side`: 订单方向（买/卖）
    /// - `price`: 价格
    /// - `quantity`: 需要匹配的数量
    ///
    /// # 返回
    /// - `Some(Vec<&InternalOrder>)`: 匹配到的订单列表（总数量 >= quantity）
    /// - `None`: 无法匹配
    fn match_orders(&self, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&InternalOrder>>;


    /// 添加订单到仓储
    fn add_order(&mut self, order_id: OrderId, entry: InternalOrder, side: Side, price: Price)
        -> Result<(), RepoError>;

    /// 取消订单
    fn remove_order(&mut self, order_id: OrderId) -> bool;

    // === 核心读操作 ===

    /// 根据订单ID查找订单
    fn find_order(&self, order_id: OrderId) -> Option<&InternalOrder>;

    /// 根据订单ID查找订单（可变引用）
    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut InternalOrder>;


    // === 市场数据查询 ===

    /// 获取最佳买价（O(1) 缓存访问）
    fn best_bid(&self) -> Option<Price>;

    /// 获取最佳卖价（O(1) 缓存访问）
    fn best_ask(&self) -> Option<Price>;


    // // === 事件溯源 ===
    //
    // /// 重放事件列表，将事件应用到仓储状态
    // ///
    // /// # 参数
    // /// - `events`: 事件列表（按event_id顺序）
    // ///
    // /// # 返回
    // /// - `Ok(())`: 成功应用所有事件
    // /// - `Err(RepositoryError)`: 应用事件失败
    // fn replay(&mut self, events: Vec<EntityEvent>) -> Result<(), RepoError>;
}

/// 仓储错误类型

/// 仓储错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    /// 容量已满
    CapacityExceeded,
    /// 订单已存在
    OrderAlreadyExists,
    /// 订单未找到
    OrderNotFound,
    /// 价格超出范围
    PriceOutOfRange
}

impl std::fmt::Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::CapacityExceeded => write!(f, "订单容量已满"),
            RepoError::OrderAlreadyExists => write!(f, "订单已存在"),
            RepoError::OrderNotFound => write!(f, "订单未找到"),
            RepoError::PriceOutOfRange => write!(f, "价格超出范围")
        }
    }
}

impl std::error::Error for RepoError {}


/// 多 LOB 仓储接口
///
/// 定义多个交易对的 LOB 管理和订单匹配操作
/// 遵循 Clean Architecture 的依赖倒置原则，业务层依赖此抽象接口
pub trait MultiSymbolLobRepo: Send + Sync {
    /// 匹配订单
    ///
    /// 根据交易对 symbol 查找对应的 LOB 并进行订单匹配
    ///
    /// # 参数
    /// - `symbol`: 交易对符号（如 BTCUSDT）
    /// - `side`: 订单方向（买/卖）
    /// - `price`: 价格
    /// - `quantity`: 需要匹配的数量
    ///
    /// # 返回
    /// - `Some(Vec<&InternalOrder>)`: 匹配到的订单列表
    /// - `None`: 找不到对应的 LOB 或无法匹配足够数量的订单
    ///
    /// # 性能要求
    /// - 查找 LOB: O(1) 时间复杂度
    /// - 匹配订单: O(k) 时间复杂度，其中 k 是匹配的订单数量
    fn match_orders(&self, symbol: Symbol, side: Side, price: Price, quantity: Quantity)
                    -> Option<Vec<&InternalOrder>>;

    /// 获取指定交易对的最佳买价
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    ///
    /// # 返回
    /// - `Some(Price)`: 最佳买价
    /// - `None`: 找不到对应的 LOB 或买盘为空
    fn best_bid(&self, symbol: Symbol) -> Option<Price>;

    /// 获取指定交易对的最佳卖价
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    ///
    /// # 返回
    /// - `Some(Price)`: 最佳卖价
    /// - `None`: 找不到对应的 LOB 或卖盘为空
    fn best_ask(&self, symbol: Symbol) -> Option<Price>;

    /// 检查指定交易对的 LOB 是否存在
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    ///
    /// # 返回
    /// - `true`: LOB 存在
    /// - `false`: LOB 不存在
    fn contains_symbol(&self, symbol: &Symbol) -> bool;
}