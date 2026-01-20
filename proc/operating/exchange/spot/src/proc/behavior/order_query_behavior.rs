//! OrderQueryService CQRS 重构版本
//!
//! 使用统一的 Query<T> → QueryResult<T> 模式
//!
//! ## 重构前后对比
//!
//! ### 重构前（传统方式）
//! ```rust,ignore
//! fn get_order_by_id(&self, order_id: OrderId) -> Option<OrderView>;
//! fn get_orders_by_trader(&self, trader_id: TraderId, active_only: bool) -> Vec<OrderView>;
//! fn get_order_statistics(&self) -> OrderStatistics;
//! ```
//!
//! ### 重构后（CQRS 模式）
//! ```rust,ignore
//! async fn query(&self, query: Query<GetOrderById>) -> Result<QueryResult<GetOrderByIdData>, CqrsError>;
//! async fn query(&self, query: Query<GetOrdersByTrader>) -> Result<QueryResult<GetOrdersByTraderData>, CqrsError>;
//! async fn query(&self, query: Query<GetOrderStatistics>) -> Result<QueryResult<OrderStatisticsData>, CqrsError>;
//! ```
//!
//! ## 优势
//!
//! 1. **统一接口**：所有查询使用相同的模式
//! 2. **可扩展**：添加新参数不破坏接口
//! 3. **可序列化**：支持查询日志和缓存
//! 4. **元数据支持**：内置追踪、缓存、性能监控

use base_types::exchange::spot::spot_types::TraderId;
use base_types::{OrderId, Price, Quantity, Side};
use cqrs::{CqrsError, Query, QueryResult};
// use crate::lob::{
//     domain::entity::spot_types::{OrderId, Price, Quantity, Side},
//     TraderId
// };
// ==================== 临时类型定义（原来在 mgn.rs 中）====================

/// 订单查询请求（临时定义，实际应从 mgn 模块导入）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderQuery {
    pub order_id: Option<OrderId>,
    pub trader_id: Option<TraderId>,
    pub side: Option<Side>,
    pub price_range: Option<(Price, Price)>,
    pub active_only: bool,
    pub offset: usize,
    pub limit: usize
}

/// 订单查询结果（临时定义）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderQueryResult {
    pub orders: Vec<OrderView>,
    pub total_count: usize,
    pub has_more: bool
}

/// 订单视图（临时定义）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderView {
    pub order_id: OrderId,
    pub trader: TraderId,
    pub side: Side,
    pub price: Price,
    pub total_quantity: Quantity,
    pub unfilled_quantity: Quantity,
    pub filled_quantity: Quantity,
    pub fill_rate: u32,
    pub status: OrderStatusEnum,
    pub created_at: u64,
    pub updated_at: u64
}

/// 订单状态（临时定义）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderStatusEnum {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled
}

/// 订单统计（临时定义）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderStatistics {
    pub total_orders: usize,
    pub active_orders: usize,
    pub filled_orders: usize,
    pub cancelled_orders: usize
}

// ==================== 查询负载定义 ====================

/// 根据订单ID查询
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetOrderById {
    pub order_id: OrderId
}

/// 查询交易员的订单
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetOrdersByTrader {
    pub trader_id: TraderId,
    pub active_only: bool,
    /// 可选：分页限制
    pub limit: Option<usize>,
    /// 可选：分页偏移
    pub offset: Option<usize>
}

impl GetOrdersByTrader {
    /// 创建查询所有订单的请求
    pub fn all(trader_id: TraderId) -> Self {
        Self {
            trader_id,
            active_only: false,
            limit: None,
            offset: None
        }
    }

    /// 创建查询活跃订单的请求
    pub fn active_only(trader_id: TraderId) -> Self {
        Self {
            trader_id,
            active_only: true,
            limit: None,
            offset: None
        }
    }

    /// 设置分页
    pub fn with_pagination(mut self, limit: usize, offset: usize) -> Self {
        self.limit = Some(limit);
        self.offset = Some(offset);
        self
    }
}

/// 获取订单统计
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetOrderStatistics {
    /// 可选：按交易员过滤
    pub trader_id: Option<TraderId>,
    /// 可选：按方向过滤
    pub side: Option<Side>
}

impl Default for GetOrderStatistics {
    fn default() -> Self {
        Self {
            trader_id: None,
            side: None
        }
    }
}

impl GetOrderStatistics {
    /// 全局统计
    pub fn global() -> Self { Self::default() }

    /// 按交易员统计
    pub fn by_trader(trader_id: TraderId) -> Self {
        Self {
            trader_id: Some(trader_id),
            side: None
        }
    }

    /// 按方向统计
    pub fn by_side(side: Side) -> Self {
        Self {
            trader_id: None,
            side: Some(side)
        }
    }
}

/// 获取最优买价
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetBestBid {}

/// 获取最优卖价
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetBestAsk {}

/// 获取价差
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSpread {}

/// 获取中间价
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMidPrice {}

/// 获取市场深度
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMarketDepth {
    /// 深度级别数量
    pub levels: usize
}

impl GetMarketDepth {
    pub fn new(levels: usize) -> Self {
        Self {
            levels
        }
    }

    /// Level 1 数据（BBO）
    pub fn level1() -> Self {
        Self {
            levels: 1
        }
    }

    /// Level 2 数据（常见深度）
    pub fn level2() -> Self {
        Self {
            levels: 10
        }
    }
}

/// 复杂订单查询（原有的 OrderQuery）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryOrders {
    pub filter: OrderQuery
}

// ==================== 查询结果定义 ====================

/// 单个订单查询结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetOrderByIdData {
    pub order: Option<OrderView>
}

/// 订单列表查询结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetOrdersByTraderData {
    pub orders: Vec<OrderView>,
    pub total_count: usize,
    pub has_more: bool
}

/// 订单统计结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderStatisticsData {
    pub total_orders: usize,
    pub active_orders: usize,
    pub filled_orders: usize,
    pub cancelled_orders: usize,
    pub total_volume: Quantity,
    pub by_side: Option<SideStatistics>
}

/// 方向统计
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SideStatistics {
    pub buy_count: usize,
    pub sell_count: usize,
    pub buy_volume: Quantity,
    pub sell_volume: Quantity
}

/// 最优买价结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BestBidData {
    pub price: Option<Price>,
    pub quantity: Option<Quantity>
}

/// 最优卖价结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BestAskData {
    pub price: Option<Price>,
    pub quantity: Option<Quantity>
}

/// 价差结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpreadData {
    pub spread: Option<Price>,
    pub best_bid: Option<Price>,
    pub best_ask: Option<Price>
}

/// 中间价结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MidPriceData {
    pub mid_price: Option<Price>,
    pub best_bid: Option<Price>,
    pub best_ask: Option<Price>
}

/// 市场深度结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarketDepthData {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub timestamp: u64
}

/// 价格级别
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLevel {
    pub price: Price,
    pub quantity: Quantity,
    pub order_count: usize
}

/// 复杂查询结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryOrdersData {
    pub result: OrderQueryResult
}

// ==================== CQRS 风格的服务接口 ====================

/// OrderQueryService V2 - CQRS 版本
///
/// 使用统一的 Query<T> → QueryResult<T> 模式
#[async_trait::async_trait]
pub trait OrderQueryServiceV2: Send + Sync {
    /// 根据订单ID查询
    async fn get_order_by_id(&self, query: Query<GetOrderById>) -> Result<QueryResult<GetOrderByIdData>, CqrsError>;

    /// 查询交易员的订单
    async fn get_orders_by_trader(
        &self, query: Query<GetOrdersByTrader>
    ) -> Result<QueryResult<GetOrdersByTraderData>, CqrsError>;

    /// 获取订单统计
    async fn get_order_statistics(
        &self, query: Query<GetOrderStatistics>
    ) -> Result<QueryResult<OrderStatisticsData>, CqrsError>;

    /// 获取最优买价
    async fn get_best_bid(&self, query: Query<GetBestBid>) -> Result<QueryResult<BestBidData>, CqrsError>;

    /// 获取最优卖价
    async fn get_best_ask(&self, query: Query<GetBestAsk>) -> Result<QueryResult<BestAskData>, CqrsError>;

    /// 获取价差
    async fn get_spread(&self, query: Query<GetSpread>) -> Result<QueryResult<SpreadData>, CqrsError>;

    /// 获取中间价
    async fn get_mid_price(&self, query: Query<GetMidPrice>) -> Result<QueryResult<MidPriceData>, CqrsError>;

    /// 获取市场深度
    async fn get_market_depth(&self, query: Query<GetMarketDepth>) -> Result<QueryResult<MarketDepthData>, CqrsError>;

    /// 复杂订单查询
    async fn query_orders(&self, query: Query<QueryOrders>) -> Result<QueryResult<QueryOrdersData>, CqrsError>;
}

// ==================== 实现示例 ====================

/// 示例实现
pub struct OrderQueryServiceImpl {
    // repo: Arc<dyn OrderRepository>,
}

impl OrderQueryServiceImpl {
    pub fn new() -> Self { Self {} }
}

#[async_trait::async_trait]
impl OrderQueryServiceV2 for OrderQueryServiceImpl {
    async fn get_order_by_id(&self, query: Query<GetOrderById>) -> Result<QueryResult<GetOrderByIdData>, CqrsError> {
        // 从 repo 查询
        // let order = self.repo.find_by_id(query.payload.order_id).await?;

        // 示例数据
        let order = None; // 实际应从 repo 查询

        let data = GetOrderByIdData {
            order
        };

        Ok(QueryResult::success(data))
    }

    async fn get_orders_by_trader(
        &self, query: Query<GetOrdersByTrader>
    ) -> Result<QueryResult<GetOrdersByTraderData>, CqrsError> {
        // 从 repo 查询
        // let orders = self.repo
        //     .find_by_trader(query.payload.trader_id)
        //     .await?;

        // 示例数据
        let orders = vec![];
        let total_count = 0;
        let has_more = false;

        let data = GetOrdersByTraderData {
            orders,
            total_count,
            has_more
        };

        Ok(QueryResult::success(data))
    }

    async fn get_order_statistics(
        &self, _query: Query<GetOrderStatistics>
    ) -> Result<QueryResult<OrderStatisticsData>, CqrsError> {
        // 从 repo 统计
        let data = OrderStatisticsData {
            total_orders: 0,
            active_orders: 0,
            filled_orders: 0,
            cancelled_orders: 0,
            total_volume: Quantity::default(),
            by_side: None
        };

        Ok(QueryResult::success(data))
    }

    async fn get_best_bid(&self, _query: Query<GetBestBid>) -> Result<QueryResult<BestBidData>, CqrsError> {
        // 从 repo 查询
        let data = BestBidData {
            price: Some(Price::from_raw(9900)),
            quantity: Some(Quantity::from_raw(1000))
        };

        Ok(QueryResult::success(data))
    }

    async fn get_best_ask(&self, _query: Query<GetBestAsk>) -> Result<QueryResult<BestAskData>, CqrsError> {
        let data = BestAskData {
            price: Some(Price::from_raw(10100)),
            quantity: Some(Quantity::from_raw(800))
        };

        Ok(QueryResult::success(data))
    }

    async fn get_spread(&self, _query: Query<GetSpread>) -> Result<QueryResult<SpreadData>, CqrsError> {
        let data = SpreadData {
            spread: Some(Price::from_raw(200)),
            best_bid: Some(Price::from_raw(9900)),
            best_ask: Some(Price::from_raw(10100))
        };

        Ok(QueryResult::success(data))
    }

    async fn get_mid_price(&self, _query: Query<GetMidPrice>) -> Result<QueryResult<MidPriceData>, CqrsError> {
        let data = MidPriceData {
            mid_price: Some(Price::from_raw(10000)),
            best_bid: Some(Price::from_raw(9900)),
            best_ask: Some(Price::from_raw(10100))
        };

        Ok(QueryResult::success(data))
    }

    async fn get_market_depth(&self, query: Query<GetMarketDepth>) -> Result<QueryResult<MarketDepthData>, CqrsError> {
        let _levels = query.payload.levels;

        let data = MarketDepthData {
            bids: vec![],
            asks: vec![],
            timestamp: 1234567890
        };

        Ok(QueryResult::success(data))
    }

    async fn query_orders(&self, query: Query<QueryOrders>) -> Result<QueryResult<QueryOrdersData>, CqrsError> {
        // 使用原有的 OrderQuery 逻辑
        let result = OrderQueryResult {
            orders: vec![],
            total_count: 0,
            has_more: false
        };

        let data = QueryOrdersData {
            result
        };

        Ok(QueryResult::success(data))
    }
}

// ==================== 使用示例 ====================

#[cfg(test)]
mod tests {
    use cqrs::QueryMetadata;
    use super::*;

    #[tokio::test]
    async fn test_get_order_by_id() {
        let service = OrderQueryServiceImpl::new();

        // 创建查询
        let query = Query::new(GetOrderById {
            order_id: 12345
        });

        // 执行查询
        let result = service.get_order_by_id(query).await.unwrap();

        // 验证结果
        assert!(result.is_success());
        println!("Query result: {:?}", result.data);
    }

    #[tokio::test]
    async fn test_get_best_bid_ask() {
        let service = OrderQueryServiceImpl::new();

        // 获取最优买价
        let bid_query = Query::new(GetBestBid::default());
        let bid_result = service.get_best_bid(bid_query).await.unwrap();

        // 获取最优卖价
        let ask_query = Query::new(GetBestAsk::default());
        let ask_result = service.get_best_ask(ask_query).await.unwrap();

        assert!(bid_result.is_success());
        assert!(ask_result.is_success());

        println!("Best bid: {:?}", bid_result.data.price);
        println!("Best ask: {:?}", ask_result.data.price);
    }

    #[tokio::test]
    async fn test_get_market_depth() {
        let service = OrderQueryServiceImpl::new();

        // Level 2 数据（10 层深度）
        let query = Query::new(GetMarketDepth::level2());

        let result = service.get_market_depth(query).await.unwrap();

        assert!(result.is_success());
        println!("Market depth: {:?}", result.data);
    }

    #[tokio::test]
    async fn test_query_with_cache() {
        let service = OrderQueryServiceImpl::new();

        // 启用缓存的查询
        let metadata = QueryMetadata::new().with_cache(300); // 缓存 300 秒

        let query = Query::with_metadata(GetBestBid::default(), metadata);

        let result = service.get_best_bid(query).await.unwrap();

        assert!(result.is_success());
        // 第一次查询不会来自缓存
        assert!(!result.is_from_cache());
    }
}
