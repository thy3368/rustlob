use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::handler::event_actor::{Actor, Event};

// =============================================================================
// EXAMPLE: OrderCreated -> MatchCmd
// =============================================================================

// 1. 定义事件
struct OrderCreatedEvent {
    order_id: String,
    user_id: String,
    symbol: String,
}

impl Event for OrderCreatedEvent {
    fn event_type(&self) -> &str {
        "OrderCreated"
    }
}

// 2. 定义 EventHandler (使用闭包)
fn create_match_handler() -> impl Fn(OrderCreatedEvent) {
    move |event: OrderCreatedEvent| {
        // 将事件映射为 MatchCmd
        let _match_cmd = MatchCmd {
            match_id: format!("match_{}", event.order_id),
            taker_order_id: event.order_id,
        };
        // 调用 MatchHandler 处理...
    }
}

struct MatchCmd {
    match_id: String,
    taker_order_id: String,
}

// =============================================================================
// EXAMPLE: 在线程中运行 Actor
// =============================================================================

pub fn example_actor_thread() {
    // 1. 创建 Channel
    let (tx, rx) = mpsc::channel();

    // 2. 创建 Actor
    let actor = Actor::<OrderCreatedEvent>::new("match_actor".to_string());

    // 3. 创建 handler
    let handler = create_match_handler();

    // 4. 在线程中运行
    let handle = thread::spawn(move || {
        while let Ok(event) = rx.recv_timeout(Duration::from_millis(100)) {
            handler(event);
        }
    });

    // 5. 发送事件
    tx.send(OrderCreatedEvent {
        order_id: "order_123".to_string(),
        user_id: "user_456".to_string(),
        symbol: "BTCUSDT".to_string(),
    })
    .unwrap();
    drop(tx);

    // 6. 等待线程结束
    let _ = handle.join();
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_actor_example() {
        example_actor_thread();
    }
}
