//! EventActor: 单线程接收队列事件并调用 handler
//!
//! 模型：
//! - Actor 在独立线程中运行
//! - 接收 Channel / Kafka 等队列的事件
//! - 调用 handler 处理事件

use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

// =============================================================================
// EVENT
// =============================================================================

pub trait Event: Send {
    fn event_type(&self) -> &str;
}

// =============================================================================
// ACTOR STATE
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorState {
    Starting,
    Running,
    Stopping,
    Stopped,
}

// =============================================================================
// ACTOR
// =============================================================================

pub struct Actor<E: Event> {
    name: String,
    state: Arc<AtomicU8>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E: Event> Actor<E> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: Arc::new(AtomicU8::new(ActorState::Starting as u8)),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stop(&self) {
        self.state.store(ActorState::Stopping as u8, Ordering::SeqCst);
    }

    pub fn state(&self) -> ActorState {
        match self.state.load(Ordering::SeqCst) {
            0 => ActorState::Starting,
            1 => ActorState::Running,
            2 => ActorState::Stopping,
            _ => ActorState::Stopped,
        }
    }
}

// =============================================================================
// ERROR
// =============================================================================

#[derive(Debug)]
pub struct ActorError(pub String);

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use super::*;

    struct TestEvent {
        data: String,
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            "test"
        }
    }

    #[test]
    fn test_actor_state() {
        assert_eq!(ActorState::Starting as u8, 0);
        assert_eq!(ActorState::Running as u8, 1);
        assert_eq!(ActorState::Stopping as u8, 2);
        assert_eq!(ActorState::Stopped as u8, 3);
    }

    #[test]
    fn test_actor_creation() {
        let actor = Actor::<TestEvent>::new("test".to_string());

        assert_eq!(actor.name(), "test");
        assert_eq!(actor.state(), ActorState::Starting);
    }

    #[test]
    fn test_actor_thread_example() {
        // 1. 创建 Channel
        let (tx, rx) = mpsc::channel::<TestEvent>();

        // 2. 创建 Actor
        let _actor = Actor::<TestEvent>::new("event_actor".to_string());

        // 3. 在线程中运行
        let handle = thread::spawn(move || {
            // 模拟事件处理
            let mut events_received = 0;

            while let Ok(event) = rx.recv_timeout(Duration::from_millis(100)) {
                events_received += 1;
                // 处理事件...
                assert_eq!(event.data, "hello");
            }

            events_received
        });

        // 4. 发送事件
        tx.send(TestEvent { data: "hello".to_string() }).unwrap();
        drop(tx); // 关闭 sender

        // 5. 等待线程结束
        let count = handle.join().unwrap();
        assert_eq!(count, 1);
    }
}
