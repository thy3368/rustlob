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
//! - **matching_service**: 匹配服务（领域服务层）
//! - **engine**: 订单簿Facade（应用层）
//! - **arena**: 内存池分配器（基础设施层）
//!
//! # 示例
//!
//! ```
//! use lob::lob::{OrderBook, TraderId, Side};
//!
//! let mut book = OrderBook::new();
//!
//! // 放置卖单
//! let seller = TraderId::from_str("SELLER1");
//! book.limit_order(seller, Side::Sell, 10000, 100);
//!
//! // 放置匹配的买单
//! let buyer = TraderId::from_str("BUYER1");
//! let (order_id, trades) = book.limit_order(buyer, Side::Buy, 10000, 50);
//!
//! assert_eq!(trades.len(), 1);
//! assert_eq!(trades[0].quantity, 50);
//! ```

pub mod arena;             // 内存池分配器
pub mod engine;            // 订单簿Facade
pub mod matching_service;  // 匹配服务
pub mod repository;        // 仓储接口和实现
pub mod types;             // 数据类型定义

// 重新导出常用类型
pub use engine::{OrderBook, OrderBookSnapshot};
pub use types::{OrderEntry, OrderId, Price, Quantity, Side, Trade, TraderId};

// 导出服务和仓储（供高级用户使用）
pub use matching_service::{MatchingService, MarketDataService};
pub use repository::{OrderRepository, InMemoryOrderRepository, RepositoryError};
