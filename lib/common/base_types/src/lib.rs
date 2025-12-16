//! 基础类型定义
//!
//! 提供交易系统的核心基础类型，供所有模块共享使用
//! 遵循 Clean Architecture 原则，将共享的基础类型提取到独立模块

pub mod order_types;
pub mod position_types;
pub mod trading_types;

// Re-export all types
pub use order_types::{OrderId, Side};
pub use position_types::{PositionId, PositionInfo, PositionSide, Price, Quantity, Symbol};
pub use trading_types::{AccountId, AssetId, Timestamp, TradingPair, UserId};
