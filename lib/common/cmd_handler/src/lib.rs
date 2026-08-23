//! 命令处理器模块
//!
//! 提供高性能命令处理框架，支持纳秒级延迟度量

#![allow(
    clippy::disallowed_macros,
    clippy::todo,
    reason = "cmd_handler 仍保留旧版 handler_cmd 草稿实现，先显式记录 todo 债务。"
)]

pub mod command_use_case_def2;
pub mod handler_cmd;
pub mod handler_query;
pub mod query_use_case_def;
pub mod trace_log;
#[cfg(test)]
mod use_case_examples;
#[cfg(test)]
mod use_case_proptest_examples;

pub use common_entity::{EntityReplayableEvent, ReplayFieldChange};
pub use handler_cmd::{
    CmdHandlerForUpdate3, CmdHandlerInternal, HandlerLatencyMetrics, ReplayableEventSet,
};
pub use trace_log::{
    FullTraceLogFormatter, MinimalTraceLogFormatter, TraceLogFormatter, build_dual_trace_subscriber,
};
