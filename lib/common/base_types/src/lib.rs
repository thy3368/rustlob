//! 基础类型定义
//!
//! 提供交易系统的核心基础类型，供所有模块共享使用
//! 遵循 Clean Architecture 原则，将共享的基础类型提取到独立模块

#![allow(
    deprecated,
    clippy::assign_op_pattern,
    clippy::derivable_impls,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::doc_lazy_continuation,
    clippy::empty_line_after_doc_comments,
    clippy::empty_line_after_outer_attr,
    clippy::let_and_return,
    clippy::clone_on_copy,
    clippy::manual_checked_ops,
    clippy::module_inception,
    clippy::new_without_default,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::disallowed_types,
    clippy::should_implement_trait,
    clippy::todo,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    clippy::unnecessary_find_map,
    clippy::unnecessary_lazy_evaluations,
    clippy::unwrap_used,
    reason = "base_types 存在较多历史建模与文档风格债务，先保留具体 lint 豁免以替代 allow(warnings)。"
)]

extern crate core;

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

pub mod operator;

pub mod sys_error;

// Re-export all types
pub use base_types::{
    AccountId, AssetId, OrderId, OrderSide, PositionId, Price, Quantity, Timestamp, TradeId,
    TradingPair, UserId,
};
pub use exchange::prep::perp_types::{PositionSide, PrepPosition, PrepTrade};
pub use exchange::prep::prep_order::{FutureOrderStatus, TimeInForce};
pub use instrument::instrument_types::InstrumentType;
pub use rust_decimal::Decimal;
