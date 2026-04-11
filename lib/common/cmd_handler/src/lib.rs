//! 命令处理器模块
//!
//! 提供高性能命令处理框架，支持纳秒级延迟度量

pub mod handler;

// Re-export types from db_repo for convenience
// pub use db_repo::{CmdRepo2, EventPublisher2};
pub use handler::{
    CmdHandlerForUpdate3, CmdHandlerInternal, DomainEventSet, HandlerLatencyMetrics,
};
