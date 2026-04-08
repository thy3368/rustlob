//! Command Handler 模块
//!
//! 设计原则：
//! - 每个 Handler 负责一类命令的处理
//! - Handler 依赖共享的 HandlerContext
//! - 使用 trait 定义统一接口
//! - 支持独立测试和组合

pub mod order_handler;

pub mod place_order_handler;

pub mod spot_trade_v4;

pub mod spot_trade_v2;


// 撮合相关模块


pub mod v3;
