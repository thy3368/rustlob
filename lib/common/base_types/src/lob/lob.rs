use diff::Entity;
use crate::{OrderId, Price, Quantity, OrderSide, TradingPair};

/// 仓储接口定义

/// 订单抽象 trait
///
/// 定义订单的核心行为，遵循依赖倒置原则
/// 业务层依赖此抽象接口，而非具体的 InternalOrder 实现
///
/// Order trait 继承 Entity trait，支持完整的事件溯源和审计能力
pub trait LobOrder: Entity + Send + Sync {
    /// 获取订单ID
    fn order_id(&self) -> OrderId;

    /// 获取价格
    fn price(&self) -> Price;

    /// 获取数量（订单总数量）
    fn quantity(&self) -> Quantity;

    /// 获取已成交数量
    ///
    /// # 返回
    /// 订单的已成交数量
    ///
    /// # 说明
    /// - 对于未成交订单，返回 0
    /// - 对于部分成交订单，返回已成交的数量
    /// - 对于完全成交订单，返回值等于 `quantity()`
    fn filled_quantity(&self) -> Quantity;

    /// 获取方向
    fn side(&self) -> OrderSide;

    /// 获取交易对
    fn symbol(&self) -> TradingPair;
}
