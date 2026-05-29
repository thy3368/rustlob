//! 命令处理器模块
//!
//! 提供高性能命令处理框架，支持纳秒级延迟度量

pub mod handler_cmd;
pub mod handler_query;
pub mod pipe_line_handler;
pub mod use_case_def;
pub mod use_case_def2;
#[cfg(test)]
mod use_case_review_examples;
mod use_case_design;
pub mod use_case_example;
mod use_case_impl;

pub use handler_cmd::{
    CmdHandlerForUpdate3, CmdHandlerInternal, HandlerLatencyMetrics, TraceableEventSet,
};
