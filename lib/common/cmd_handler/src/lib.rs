//! 命令处理器模块
//!
//! 提供高性能命令处理框架，支持纳秒级延迟度量

pub mod handler_cmd;
pub mod handler_query;
pub mod trace_log;
pub mod use_case_def2;
#[cfg(test)]
mod use_case_examples;
#[cfg(test)]
mod use_case_proptest_examples;

pub use diff::{EntityReplayableEvent, ReplayFieldChange};
pub use handler_cmd::{
    CmdHandlerForUpdate3, CmdHandlerInternal, HandlerLatencyMetrics, ReplayableEventSet,
};
pub use trace_log::{
    FullTraceLogFormatter, MinimalTraceLogFormatter, TraceLogFormatter, build_dual_trace_subscriber,
};
