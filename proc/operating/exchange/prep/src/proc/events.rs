//! 简化版事件系统 - 暂时移除serde依赖

pub trait DomainEvent: Send + Sync + std::fmt::Debug {
    fn event_type(&self) -> &'static str;
    fn timestamp(&self) -> u64;
    fn aggregate_id(&self) -> String;
}

// 暂时为空，待后续实现完整事件系统
