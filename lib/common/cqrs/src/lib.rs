//! CQRS (Command Query Responsibility Segregation) Framework
//!
//! 提供统一的 Command/Query/Result 模式，支持：
//! - 命令和查询的元数据追踪
//! - 序列化支持（可选 serde feature）
//! - 异步处理（可选 async feature）
//! - 类型安全的处理器接口

mod cqrs;

// 重新导出所有公共类型
pub use cqrs::*;
