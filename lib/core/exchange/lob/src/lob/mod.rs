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
//! - **repo**: 仓储接口和实现（数据访问层）
//! - **matching_service**: 匹配服务（领域服务层） - **核心服务**
//! - **market_data_service**: 市场数据查询服务（领域服务层）
//! - **arena**: 内存池分配器（基础设施层）
//!
//! ## 使用方式
//!
//! 直接使用 MatchingService 获得最佳性能：
//!
//! ```no_run
//! use lob::lob::{
//!     Cmd, SpotCmdAny, TimeInForce,
//!     TraderId, Side, Symbol
//! };
//!
//! // 创建限价单命令
//! let trader = TraderId::from_str("TRADER1");
//! let symbol = Symbol::from_str("BTCUSDT");
//! let nonce = 1001;
//!
//! let limit_order = Cmd::new(
//!     nonce,
//!     SpotCmdAny::LimitOrder {
//!         trader,
//!         trading_pair:symbol,
//!         side: Side::Buy,
//!         price: 10000,
//!         quantity: 100,
//!         time_in_force: TimeInForce::GoodTillCancel,
//!         client_order_id: None,
//!     }
//! );
//!
//! // 使用 MatchingService 执行订单
//! // let result = matching_service.execute(limit_order);
//! ```
//!
//! # 示例
//!
//! ```no_run
//! use lob::lob::{
//!     Cmd, SpotCmdAny, TimeInForce,
//!     TraderId, Side, Symbol
//! };
//!
//! // 创建交易员
//! let seller = TraderId::from_str("SELLER1");
//! let buyer = TraderId::from_str("BUYER1");
//! let symbol = Symbol::from_str("BTCUSDT");
//!
//! // 卖单：卖出 100 @ 10000
//! let sell_order = Cmd::new(
//!     1001,
//!     SpotCmdAny::LimitOrder {
//!         trader: seller,
//!         trading_pair:symbol,
//!         side: Side::Sell,
//!         price: 10000,
//!         quantity: 100,
//!         time_in_force: TimeInForce::GoodTillCancel,
//!         client_order_id: None,
//!     }
//! );
//!
//! // 买单：买入 50 @ 10000（会与卖单撮合）
//! let buy_order = Cmd::new(
//!     1002,
//!     SpotCmdAny::LimitOrder {
//!         trader: buyer,
//!         trading_pair:symbol,
//!         side: Side::Buy,
//!         price: 10000,
//!         quantity: 50,
//!         time_in_force: TimeInForce::GoodTillCancel,
//!         client_order_id: None,
//!     }
//! );
//!
//! // 执行订单
//! // let sell_result = matching_service.execute(sell_order);
//! // let buy_result = matching_service.execute(buy_order);
//! ```

mod adaptor;
pub mod domain;
// OrderQueryService CQRS 重构版本
// 数据类型定义
// pub mod level_types; // Level 1-3 市场数据类型

// 重新导出常用类型
// pub use types::{OrderEntry, OrderId, Price, Quantity, Side, Trade, TraderId};

// 导出 Level 1-3 市场数据类型
// pub use level_types::{Level1, Level2, Level3, Level3Order, PriceLevel};

// 导出服务和仓储（供高级用户使用）
// 幂等性包装
// 导出基础类型
// 算法交易命令
// 条件订单命令
// 做市商命令
// 核心现货命令


// Re-export base types for convenience
pub use base_types::{OrderId, Price, Quantity, Side};
