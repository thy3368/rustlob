//! EventActor 职责定义。
//!
//! EventActor 位于 transport / queue 的消费侧，职责是：
//! 1. 启动单线程消费循环
//! 2. 从 channel / Kafka / NATS 等队列拉取事件
//! 3. 调用具体实现提供的事件处理逻辑
//!
//! 职责边界：
//! - EventActor 只负责消费循环
//! - 具体 actor 自己决定如何处理事件
//! - 队列 / broker client 细节由具体 EventActor 实现持有

pub trait EventActor<Ev, Err>: Send {
    /// 从底层队列拉取一个事件。
    ///
    /// - 返回 `Ok(Some(event))` 表示成功取到事件
    /// - 返回 `Ok(None)` 表示消费循环结束
    fn recv_event(&mut self) -> Result<Option<Ev>, Err>;

    /// 处理单个事件。
    fn handle_event(&self, event: Ev) -> Result<(), Err>;

    /// 启动单线程消费循环。
    fn run(&mut self) -> Result<(), Err> {
        while let Some(event) = self.recv_event()? {
            self.handle_event(event)?;
        }

        Ok(())
    }
}

