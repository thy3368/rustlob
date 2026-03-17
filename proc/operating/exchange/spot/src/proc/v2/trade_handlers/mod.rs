//! Command Handler 模块
//!
//! 设计原则：
//! - 每个 Handler 负责一类命令的处理
//! - Handler 依赖共享的 HandlerContext
//! - 使用 trait 定义统一接口
//! - 支持独立测试和组合

use base_types::cqrs::cqrs_types::CmdResp;

pub mod account_handler;
pub mod oco_handler;
pub mod order_handler;
pub mod spot_trade_v3;
pub mod spot_trade_v2;

/// Command Handler trait
#[async_trait::async_trait]
pub trait CommandHandler<Cmd, Res, Err>: Send + Sync {
    async fn handle(&self, cmd: Cmd) -> Result<CmdResp<Res>, Err>;
}
