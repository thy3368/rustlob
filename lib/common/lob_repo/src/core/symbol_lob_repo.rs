use base_types::lob::lob::LobOrder;
use base_types::{OrderId, OrderSide, Price, Quantity, TradingPair};
use diff::Entity;

pub(crate) use crate::core::repo_snapshot_support::RepoError;

// /// 仓储接口定义
//
// /// 订单抽象 trait
// ///
// /// 定义订单的核心行为，遵循依赖倒置原则
// /// 业务层依赖此抽象接口，而非具体的 InternalOrder 实现
// ///
// /// Order trait 继承 Entity trait，支持完整的事件溯源和审计能力
// pub trait Order: Entity + Send + Sync {
//     /// 获取订单ID
//     fn order_id(&self) -> OrderId;
//
//     /// 获取价格
//     fn price(&self) -> Price;
//
//     /// 获取数量（订单总数量）
//     fn quantity(&self) -> Quantity;
//
//     /// 获取已成交数量
//     ///
//     /// # 返回
//     /// 订单的已成交数量
//     ///
//     /// # 说明
//     /// - 对于未成交订单，返回 0
//     /// - 对于部分成交订单，返回已成交的数量
//     /// - 对于完全成交订单，返回值等于 `quantity()`
//     fn filled_quantity(&self) -> Quantity;
//
//     /// 获取方向
//     fn side(&self) -> Side;
//
//     /// 获取交易对
//     fn symbol(&self) -> TradingPair;
// }

/// LOB 快照数据
///
/// 用于保存 LOB 某个时间点的完整状态，支持事件溯源和状态重建
#[derive(Debug, Clone)]
pub struct LobSnapshot {
    /// 交易对符号
    pub symbol: TradingPair,
    /// 快照时间戳（纳秒）
    pub timestamp: u64,
    /// 快照序列号
    pub sequence: u64,
    /// 序列化的 LOB 状态数据
    pub data: Vec<u8>,
    /// 最佳买价（快照时）
    pub best_bid: Option<Price>,
    /// 最佳卖价（快照时）
    pub best_ask: Option<Price>,
    /// 最后成交价（快照时）
    pub last_price: Option<Price>,
}

impl LobSnapshot {
    /// 创建 LOB 快照
    pub fn new(
        symbol: TradingPair,
        timestamp: u64,
        sequence: u64,
        data: Vec<u8>,
        best_bid: Option<Price>,
        best_ask: Option<Price>,
        last_price: Option<Price>,
    ) -> Self {
        Self { symbol, timestamp, sequence, data, best_bid, best_ask, last_price }
    }
}

/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
/// 仅暴露业务层需要的操作，内部实现细节（如链表遍历、价格点管理）由具体实现封装
pub trait SymbolLob {
    /// 订单类型关联类型
    type Order: LobOrder;

    /// 匹配订单，返回匹配到的订单引用列表
    ///
    /// # 参数
    /// - `side`: 订单方向（买/卖）
    /// - `price`: 价格
    /// - `quantity`: 需要匹配的数量
    ///
    /// # 返回
    /// - `Some(Vec<&Self::Order>)`: 匹配到的订单列表（总数量 >= quantity）
    /// - `None`: 无法匹配
    fn match_orders(
        &self,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> Option<Vec<&Self::Order>>;

    /// 添加订单到仓储
    ///
    /// # 参数
    /// - `order`: 实现了 Order trait 的订单对象
    ///
    /// # 返回
    /// - `Ok(())`: 成功添加订单
    /// - `Err(RepoError::OrderAlreadyExists)`: 订单ID已存在
    /// - `Err(RepoError::PriceOutOfRange)`: 价格超出仓储支持范围
    /// - `Err(RepoError::CapacityExceeded)`: 订单容量已满
    ///
    /// # 实现说明
    /// 实现层应从订单对象中提取所需信息：
    /// - `order.order_id()` - 订单ID
    /// - `order.side()` - 订单方向
    /// - `order.price()` - 价格
    fn add_order(&mut self, order: Self::Order) -> Result<(), RepoError>;

    /// 取消订单
    ///
    /// # 参数
    /// - `order_id`: 要取消的订单ID
    ///
    /// # 返回
    /// - `true`: 成功取消订单
    /// - `false`: 订单不存在
    fn remove_order(&mut self, order_id: OrderId) -> bool;

    // === 核心读操作 ===

    /// 根据订单ID查找订单
    fn find_order(&self, order_id: OrderId) -> Option<&Self::Order>;

    /// 根据订单ID查找订单（可变引用）
    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut Self::Order>;

    // === 市场数据查询 ===

    /// 获取最佳买价（O(1) 缓存访问）
    fn best_bid(&self) -> Option<Price>;

    /// 获取最佳卖价（O(1) 缓存访问）
    fn best_ask(&self) -> Option<Price>;

    /// 获取最后一笔成交价（O(1) 缓存访问）
    ///
    /// # 返回
    /// - `Some(Price)`: 最后一笔成交价
    /// - `None`: 尚未发生任何成交
    ///
    /// # 说明
    /// 此价格在发生成交时更新，通常在撮合引擎执行成交后调用 `update_last_price`
    /// 更新
    fn last_price(&self) -> Option<Price>;

    /// 更新最后一笔成交价
    ///
    /// # 参数
    /// - `price`: 成交价格
    ///
    /// # 说明
    /// 此方法通常由撮合引擎在成交发生后调用，用于更新市场数据
    fn update_last_price(&mut self, price: Price);
}

impl std::fmt::Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::CapacityExceeded => write!(f, "订单容量已满"),
            RepoError::OrderAlreadyExists => write!(f, "订单已存在"),
            RepoError::OrderNotFound => write!(f, "订单未找到"),
            RepoError::PriceOutOfRange => write!(f, "价格超出范围"),
            RepoError::SnapshotNotSupported => write!(f, "不支持快照功能"),
            RepoError::DeserializationFailed(msg) => write!(f, "反序列化失败: {}", msg),
            RepoError::SymbolMismatch { expected, actual } => {
                write!(f, "交易对不匹配: 期望 {}, 实际 {}", expected, actual)
            }
            RepoError::SerializationFailed(msg) => write!(f, "序列化失败: {}", msg),
        }
    }
}

impl std::error::Error for RepoError {}

/// 多 LOB 仓储接口
///
/// 定义多个交易对的 LOB 管理和订单匹配操作
/// 遵循 Clean Architecture 的依赖倒置原则，业务层依赖此抽象接口
///
/// # 关联类型
/// - `Order`: 实现了 Order trait 的订单类型
pub trait MultiSymbolLobRepo: Send + Sync {
    /// 订单类型关联类型
    type Order: LobOrder;

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
    /// - `Some(Vec<&Self::Order>)`: 匹配到的订单列表
    /// - `None`: 找不到对应的 LOB 或无法匹配足够数量的订单
    ///
    /// # 性能要求
    /// - 查找 LOB: O(1) 时间复杂度
    /// - 匹配订单: O(k) 时间复杂度，其中 k 是匹配的订单数量
    fn match_orders(
        &self,
        symbol: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> Option<Vec<&Self::Order>>;

    /// 获取指定交易对的最佳买价
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    ///
    /// # 返回
    /// - `Some(Price)`: 最佳买价
    /// - `None`: 找不到对应的 LOB 或买盘为空
    fn best_bid(&self, symbol: TradingPair) -> Option<Price>;

    /// 获取指定交易对的最佳卖价
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    ///
    /// # 返回
    /// - `Some(Price)`: 最佳卖价
    /// - `None`: 找不到对应的 LOB 或卖盘为空
    fn best_ask(&self, symbol: TradingPair) -> Option<Price>;

    /// 检查指定交易对的 LOB 是否存在
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    ///
    /// # 返回
    /// - `true`: LOB 存在
    /// - `false`: LOB 不存在
    fn contains_symbol(&self, symbol: &TradingPair) -> bool;

    fn add_order(&self, symbol: TradingPair, order: Self::Order) -> Result<(), RepoError>;

    /// 取消订单
    ///
    /// # 参数
    /// - `order_id`: 要取消的订单ID
    ///
    /// # 返回
    /// - `true`: 成功取消订单
    /// - `false`: 订单不存在
    fn remove_order(&self, symbol: TradingPair, order_id: OrderId) -> bool;

    fn find_order(&self, p0: TradingPair, p1: OrderId) -> Option<&Self::Order>;

    fn find_order_mut(&self, p0: TradingPair, order_id: OrderId) -> Option<&mut Self::Order>;
}

/// 为 Arc<L> 实现 MultiSymbolLobRepo trait
impl<L> MultiSymbolLobRepo for std::sync::Arc<L>
where
    L: MultiSymbolLobRepo + ?Sized,
{
    type Order = L::Order;

    fn match_orders(
        &self,
        symbol: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> Option<Vec<&Self::Order>> {
        (**self).match_orders(symbol, side, price, quantity)
    }

    fn best_bid(&self, symbol: TradingPair) -> Option<Price> {
        (**self).best_bid(symbol)
    }

    fn best_ask(&self, symbol: TradingPair) -> Option<Price> {
        (**self).best_ask(symbol)
    }

    fn contains_symbol(&self, symbol: &TradingPair) -> bool {
        (**self).contains_symbol(symbol)
    }

    fn add_order(&self, symbol: TradingPair, order: Self::Order) -> Result<(), RepoError> {
        (**self).add_order(symbol, order)
    }

    fn remove_order(&self, symbol: TradingPair, order_id: OrderId) -> bool {
        (**self).remove_order(symbol, order_id)
    }

    fn find_order(&self, p0: TradingPair, p1: OrderId) -> Option<&Self::Order> {
        (**self).find_order(p0, p1)
    }

    fn find_order_mut(&self, p0: TradingPair, order_id: OrderId) -> Option<&mut Self::Order> {
        (**self).find_order_mut(p0, order_id)
    }
}
