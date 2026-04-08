use base_types::lob::lob::LobOrder;
use base_types::{OrderId, OrderSide, Price, Quantity, TradingPair};

pub use crate::core::repo_snapshot_support::LobError;

/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
/// 仅暴露业务层需要的操作，内部实现细节（如链表遍历、价格点管理）由具体实现封装
pub trait SymbolLob2 {
    /// 匹配订单，返回匹配到的订单引用列表和剩余未匹配数量
    ///
    /// # 参数
    /// - `side`: 订单方向（买/卖）
    /// - `price`: 价格
    /// - `quantity`: 需要匹配的数量
    ///
    /// # 返回
    /// - `(Some(Vec<&Self::Order>), remaining)`: 匹配到的订单列表和剩余未匹配数量
    ///   - `remaining`: 0 表示全部匹配（全成交）
    ///   - `remaining` > 0 表示部分匹配（部分成交）
    /// - `(None, quantity)`: 无法匹配，返回原始数量
    fn match_orders<O: LobOrder>(
        &self,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> (Option<Vec<&O>>, Quantity);

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
    fn add_order<O: LobOrder>(&mut self, order: O) -> Result<(), LobError>;

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
    fn find_order<O: LobOrder>(&self, order_id: OrderId) -> Option<&O>;

    /// 根据订单ID查找订单（可变引用）
    fn find_order_mut<O: LobOrder>(&mut self, order_id: OrderId) -> Option<&mut O>;

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

/// 多 LOB 仓储接口
///
/// 定义多个交易对的 LOB 管理和订单匹配操作
/// 遵循 Clean Architecture 的依赖倒置原则，业务层依赖此抽象接口
pub trait MultiSymbolLobRepo2: Send + Sync {
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
    /// - `(Some(Vec<&Self::Order>), remaining)`: 匹配到的订单列表和剩余未匹配数量
    ///   - `remaining`: 0 表示全部匹配（全成交）
    ///   - `remaining` > 0 表示部分匹配（部分成交）
    /// - `(None, quantity)`: 找不到对应的 LOB 或无法匹配，返回原始数量
    ///
    /// # 性能要求
    /// - 查找 LOB: O(1) 时间复杂度
    /// - 匹配订单: O(k) 时间复杂度，其中 k 是匹配的订单数量
    fn match_orders<O: LobOrder>(
        &self,
        symbol: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> (Option<Vec<&O>>, Quantity);

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

    fn add_order<O: LobOrder>(&self, symbol: TradingPair, order: O) -> Result<(), LobError>;

    /// 取消订单
    ///
    /// # 参数
    /// - `order_id`: 要取消的订单ID
    ///
    /// # 返回
    /// - `true`: 成功取消订单
    /// - `false`: 订单不存在
    fn remove_order(&self, symbol: TradingPair, order_id: OrderId) -> bool;

    fn find_order<O: LobOrder>(&self, p0: TradingPair, p1: OrderId) -> Option<&O>;

    fn find_order_mut<O: LobOrder>(&self, p0: TradingPair, order_id: OrderId) -> Option<&mut O>;

    /// 获取指定交易对的最后一笔成交价
    fn last_price(&self, symbol: TradingPair) -> Option<Price>;

    /// 更新指定交易对的最后一笔成交价
    fn update_last_price(&self, symbol: TradingPair, price: Price);
}
