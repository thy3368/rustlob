//! 基础类型定义
//!
//! 提供交易系统的核心基础类型，供所有模块共享使用
//! 遵循 Clean Architecture 原则，将共享的基础类型提取到独立模块

pub mod account;
pub mod base_types;
pub mod exchange;
pub mod fee;
pub mod mark_data;

pub mod instrument;
pub mod lob;

pub mod cqrs;

pub mod actor_x;
pub mod handler;
pub mod spot_topic;

// Re-export all types
pub use base_types::{
    AccountId, AssetId, OrderId, OrderSide, PositionId, Price, Quantity, Timestamp, TradeId,
    TradingPair, UserId,
};
pub use decimal::Decimal;
pub use exchange::prep::perp_types::{PositionSide, PrepPosition, PrepTrade};
pub use instrument::instrument_types::InstrumentType;
