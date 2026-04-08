//! EventHandler 职责定义。
//!
//! EventHandler 不直接承担状态更新职责。
//! 它的职责是：
//! 1. 接收上游事件
//! 2. 将事件映射成下游 command
//! 3. 调用对应的 CommandHandler
//! 4. 返回下游处理结果
//!
//! 典型链路：
//! - 收单事件 -> 撮合命令
//! - 成交事件 -> 结算命令
//!
//! 事件来源通常是 CommandHandler 的 `publish_changelog` 阶段。
//! 这些 changelog / domain event 可以发布到不同通道，例如：
//! - 进程内内存队列
//! - channel / ring buffer
//! - Kafka / NATS / Redis Stream 等消息系统
//!
//! EventHandler 的位置通常是在这些通道的消费侧：
//! - 消费 changelog / event
//! - 映射下游 command
//! - 驱动下一个 CommandHandler

pub trait EventHandler<C, R, E>: Send + Sync {
    fn event_handle(&self, event: C) -> Result<R, E>;
}

pub trait EventHandler2<C, E>: Send + Sync {
    fn event_handle(&self, event: C) -> Result<(), E>;
}

