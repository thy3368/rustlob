use std::sync::Arc;

use base_types::handler::event_actor::EventRecvActor;

use crate::proc::v2::processor::kafka::base::KafkaProcessorConfig;
use crate::proc::v2::processor::kafka::matching_event_actor::KafkaMatchingEventActor;
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_order_place_event_handler::NewOrderPlaceEventHandler;

/// 在独立线程中启动 KafkaMatchingEventActor 的示例。
///
/// 调用方负责准备 `order_repo` 和 `matching_handler` 依赖。
/// actor 线程启动后会持续阻塞消费 Kafka 消息。
pub fn start_matching_event_actor_in_thread(
    event_handler: Arc<NewOrderPlaceEventHandler>,
    config: KafkaProcessorConfig,
    topic: String,
) -> Result<std::thread::JoinHandle<()>, String> {
    let actor = KafkaMatchingEventActor::new(event_handler, config, topic)?;

    std::thread::Builder::new()
        .name("matching-event-actor".to_string())
        .spawn(move || {
            let mut actor = actor;
            if let Err(error) = actor.run() {
                tracing::error!(error = ?error, "matching event actor exited");
            }
        })
        .map_err(|e| format!("Failed to spawn matching-event-actor thread: {}", e))
}

/*
Usage example:

use std::sync::Arc;

use crate::proc::v2::processor::kafka::base::KafkaProcessorConfig;
use crate::proc::v2::processor::kafka::matching_event_actor_example::start_matching_event_actor_in_thread;
use crate::proc::v2::trade_event_handlers::new_order_place_event_handler::NewOrderPlaceEventHandler;

let order_repo = Arc::new(order_repo);
let matching_handler = Arc::new(matching_handler);
let event_handler = Arc::new(NewOrderPlaceEventHandler::new(order_repo, matching_handler));
let config = KafkaProcessorConfig::new("localhost:9092", "matching-event-actor-group");

let join_handle = start_matching_event_actor_in_thread(
    event_handler,
    config,
    "spot-order-events".to_string(),
)?;

// 如需等待线程结束：
join_handle.join().expect("matching-event-actor thread panicked");
*/
