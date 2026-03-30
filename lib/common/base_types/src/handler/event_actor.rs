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
}
