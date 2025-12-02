//! 高性能订单簿实现
//!
//! 本模块提供低时延限价订单簿，具有以下特性：
//! - 使用价格索引数组实现 O(1) 订单放置
//! - 价格-时间优先匹配
//! - 内存池分配提升缓存效率
//! - 交易执行追踪
//!
//! # 架构
//!
//! 订单簿基于 Quant Cup 参考实现，并针对生产使用进行了增强：
//! - 通过内存池分配器实现静态内存分配
//! - 缓存行对齐的数据结构
//! - 快速订单取消（单次内存写入）
//!
//! ## Clean Architecture 分层
//!
//! - **types**: 领域实体和值对象
//! - **repository**: 仓储接口和实现（数据访问层）
//! - **matching_service**: 匹配服务（领域服务层） - **核心服务**
//! - **market_data_service**: 市场数据查询服务（领域服务层）
//! - **arena**: 内存池分配器（基础设施层）
//!
//! ## 使用方式
//!
//! 直接使用 MatchingService 获得最佳性能：
//!
//! ```rust
//! use lob::lob::{
//!     MatchingService, InMemoryOrderRepository, OrderRepository,
//!     TraderId, Side, OrderEntry
//! };
//!
//! // 创建 repository
//! let repository = InMemoryOrderRepository::new(100_000, 1000);
//!
//! // 创建匹配服务
//! let mut matching_service = MatchingService::new(repository);
//!
//! // 执行订单匹配
//! let trader = TraderId::from_str("TRADER1");
//! let (trades, remaining) = matching_service.match_limit_order(
//!     trader, Side::Buy, 10000, 100
//! );
//!
//! // 如果有剩余，手动添加到订单簿
//! if remaining > 0 {
//!     let order_id = matching_service.repository_mut().allocate_order_id();
//!     let entry = OrderEntry::new(order_id, trader, remaining);
//!     matching_service.repository_mut()
//!         .add_order(order_id, entry, Side::Buy, 10000).unwrap();
//! }
//! ```
//!
//! # 示例
//!
//! ```
//! use lob::lob::{
//!     MatchingService, InMemoryOrderRepository, OrderRepository,
//!     TraderId, Side, OrderEntry
//! };
//!
//! let repository = InMemoryOrderRepository::new(100_000, 1000);
//! let mut matching_service = MatchingService::new(repository);
//!
//! // 放置卖单
//! let seller = TraderId::from_str("SELLER1");
//! let sell_order_id = matching_service.repository_mut().allocate_order_id();
//! let sell_entry = OrderEntry::new(sell_order_id, seller, 100);
//! matching_service.repository_mut()
//!     .add_order(sell_order_id, sell_entry, Side::Sell, 10000).unwrap();
//!
//! // 放置匹配的买单
//! let buyer = TraderId::from_str("BUYER1");
//! let (trades, _remaining) = matching_service.match_limit_order(
//!     buyer, Side::Buy, 10000, 50
//! );
//!
//! assert_eq!(trades.len(), 1);
//! assert_eq!(trades[0].quantity, 50);
//! ```

mod domain;
mod adaptor;
// OrderQueryService CQRS 重构版本
// 数据类型定义
// pub mod level_types; // Level 1-3 市场数据类型

// 重新导出常用类型
// pub use types::{OrderEntry, OrderId, Price, Quantity, Side, Trade, TraderId};

// 导出 Level 1-3 市场数据类型
// pub use level_types::{Level1, Level2, Level3, Level3Order, PriceLevel};

// 导出服务和仓储（供高级用户使用）
// 核心现货命令
pub use domain::service::handler::{
    SpotCommand, SpotCommandResult, SpotOrderHandler,
};
// 算法交易命令
pub use domain::service::handler::{
    AlgoCommand, AlgoCommandResult, AlgoOrderHandler, UrgencyLevel,
};
// 条件订单命令
pub use domain::service::handler::{
    ConditionalCommand, ConditionalCommandResult, ConditionalOrderHandler,
    PegType, AuctionType,
};
// 做市商命令
pub use domain::service::handler::{
    MarketMakerCommand, MarketMakerCommandResult, MarketMakerHandler,
};
pub use domain::service::market_data_service::MarketDataService;
pub use domain::service::matching_service::MatchingService;
pub use domain::repository::{InMemoryOrderRepository, OrderRepository, RepositoryError};

// 导出基础类型
pub use domain::entity::lob_types::{
    EntityEvent, EventOperation, FieldChange, FieldValue, OrderEntry, OrderId, Price,
    PricePoint, Quantity, RecordChange, Side, Trade, TraderId,
};
