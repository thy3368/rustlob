//! 基础类型定义
//!
//! 提供交易系统的核心基础类型，供所有模块共享使用
//! 遵循 Clean Architecture 原则，将共享的基础类型提取到独立模块

pub mod base_types;
pub mod instrument_types;
pub mod perp_types;

// Re-export all types
pub use base_types::{
    AccountId, AssetId, OrderId, PositionId, Price, Quantity, Side, Timestamp, TradeId, TradingPair, UserId
};
pub use decimal::Decimal;
pub use instrument_types::InstrumentType;
pub use perp_types::{PositionSide, PrepPosition, PrepTrade};

