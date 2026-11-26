// 此 crate 是进程内演示 composition root；选举等待、诊断输出和 snapshot 编解码不在交易热路径。
#![allow(
    clippy::arithmetic_side_effects,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::result_large_err
)]

mod cluster;
mod model;
mod network;
mod storage;

pub use cluster::{NodeSnapshot, SpotRaftCluster};
pub use model::{
    CommandStatus, SpotCommandResponse, SpotRaftRequest, SpotRaftResponse, cancel_request,
    invalid_request, place_and_cancel_request, place_request,
};
pub use storage::{MemLogStore, SpotStateMachine, TypeConfig};

pub type DemoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
