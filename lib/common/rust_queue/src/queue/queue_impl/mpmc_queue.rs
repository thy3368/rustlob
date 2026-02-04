use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Instant
};

use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    sync::{
        broadcast,
        broadcast::{error::SendError, Receiver}
    },
    time::Duration
};

use crate::queue::queue::{DefaultQueueConfig, FromBytes, Queue, SendOptions, SubscribeOptions, ToBytes};

/// 高性能异步广播队列，用于分发 K 线更新事件
/// 基于 Tokio 的 broadcast channel 实现，支持多生产者多消费者模式
/// 支持多个 topic 和消费组功能
/// 支持每个 topic 有不同类型的 Event
pub struct MPMCQueue {
    /// 每个 topic 对应一个 broadcast channel，存储字节形式的事件
    topic_channels: Arc<RwLock<HashMap<String, broadcast::Sender<bytes::Bytes>>>>,
    /// 配置信息
    config: DefaultQueueConfig
}

impl MPMCQueue {
    /// 创建新的广播队列
    /// 容量设置为 1024，足以应对高频 K 线更新场景
    pub fn new_with_config(config: DefaultQueueConfig) -> Self {
        let topic_channels = Arc::new(RwLock::new(HashMap::new()));
        let queue = MPMCQueue {
            topic_channels,
            config
        };


        queue
    }

    /// 检查是否需要背压控制
    fn should_apply_backpressure(&self, options: &Option<SendOptions>) -> bool {
        match options {
            Some(opts) => opts.enable_backpressure,
            None => self.config.enable_backpressure
        }
    }
}


impl Queue for MPMCQueue {
    /// 为指定 topic 创建或获取 channel
    fn get_or_create_channel(&self, topic: &str) -> broadcast::Sender<bytes::Bytes> {
        let mut channels = self.topic_channels.write().unwrap();

        if let Some(existing) = channels.get(topic) {
            return existing.clone();
        }

        let (tx, _) = broadcast::channel(1024);
        channels.insert(topic.to_string(), tx.clone());

        tx
    }
    type Config = DefaultQueueConfig;

    /// 创建新的广播队列
    /// 使用默认配置
    fn new() -> Self { Self::new_with_config(DefaultQueueConfig::default()) }

    /// 创建带有自定义配置的广播队列
    fn new_with_config(config: impl Into<Self::Config>) -> Self { Self::new_with_config(config.into()) }

    /// 发送事件到指定 topic
    /// 支持序列化的事件类型
    fn send<T: Serialize + ToBytes + Send + Sync + 'static + Clone>(
        &self, topic: &str, event: T, options: Option<SendOptions>
    ) -> Result<usize, SendError<T>> {
        let channel = self.get_or_create_channel(topic);

        // 背压控制
        if self.should_apply_backpressure(&options) {
            let current_subscribers = channel.receiver_count();
            let channel_capacity = 1024; // 目前 hardcode，后续可配置
            if current_subscribers == 0 && channel_capacity > 0 {
                tracing::warn!("No subscribers for topic {}, discarding event to prevent buffer overflow", topic);
                return Ok(0);
            }
        }

        // 序列化事件
        let bytes = event.to_bytes().map_err(|_| broadcast::error::SendError(event.clone()))?;

        // 发送字节数据
        channel.send(bytes).map_err(|_| broadcast::error::SendError(event))
    }

    /// 订阅指定 topic 的事件
    /// 支持反序列化的事件类型
    fn subscribe<T: DeserializeOwned + Send + Sync + 'static + Clone>(
        &self, topic: &str, _options: Option<SubscribeOptions>
    ) -> broadcast::Receiver<T> {
        let channel = self.get_or_create_channel(topic);
        let mut rx = channel.subscribe();

        // 创建新的 receiver，内部进行反序列化
        let (tx, rx_out) = broadcast::channel(1024);

        tokio::spawn(async move {
            while let Ok(bytes) = rx.recv().await {
                match T::from_bytes(&bytes) {
                    Ok(event) => {
                        let _ = tx.send(event);
                    }
                    Err(e) => {
                        tracing::error!("Failed to deserialize event: {}", e);
                    }
                }
            }
        });

        rx_out
    }

    /// 获取指定 topic 的当前订阅者数量
    fn subscriber_count(&self, topic: &str) -> usize {
        if let Ok(channels) = self.topic_channels.read() {
            if let Some(channel) = channels.get(topic) {
                return channel.receiver_count();
            }
        }
        0
    }

    /// 获取所有支持的 topic 列表
    fn topics(&self) -> Vec<String> {
        if let Ok(channels) = self.topic_channels.read() {
            channels.keys().cloned().collect()
        } else {
            vec![]
        }
    }
}

impl Default for MPMCQueue {
    fn default() -> Self { Self::new() }
}

//todo 新建一个简单的event 代替 KLineUpdateEvent
/*
#[tokio::test]
async fn broadcast_example() {
    let config = DefaultQueueConfig::new().with_send_timeout(5000).with_recv_timeout(3000);

    let queue = MPMCQueue::new_with_config(config);

    println!("初始订阅者数量: {}", queue.subscriber_count("test-topic"));

    // 创建 3 个订阅者
    let mut subscribers = vec![];
    for i in 0..3 {
        let mut rx = queue.subscribe::<KLineUpdateEvent>("test-topic", None);
        subscribers.push(tokio::spawn(async move {
            let mut received = 0;
            while let Ok(event) = rx.recv().await {
                println!(
                    "[Subscriber {}] Received: Window={:?}, OHLC={:?}, New={}",
                    i, event.window, event.ohlc, event.is_new_window
                );
                received += 1;
                if received >= 5 {
                    break;
                }
            }
        }));
    }

    println!("订阅后数量: {}", queue.subscriber_count("test-topic"));

    // 模拟发送 K 线更新事件
    let base_timestamp = Instant::now().elapsed().as_secs();
    for i in 0..5 {
        let event = KLineUpdateEvent {
            window: TimeWindow::Second,
            ohlc: OHLC::new(base_timestamp + i, 50000.0 + (i as f64) * 10.0, 1.0 + (i as f64) * 0.1),
            is_new_window: i % 2 == 0
        };

        match queue.send("test-topic", event, None) {
            Ok(count) => println!("[Broadcaster] Sent: Message {}, Received by {} subscribers", i, count),
            Err(e) => println!("[Broadcaster] Error: {}", e)
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // 等待所有订阅者完成
    for subscriber in subscribers {
        subscriber.await.unwrap();
    }

    println!("\n在广播模式中:");
    println!("每条消息被所有订阅者接收");
    println!("这是真正的'多消费者接收同一条消息'");
}

#[tokio::test]
async fn test_multiple_topics_with_different_events() {
    let queue = MPMCQueue::new();

    // 测试创建和订阅多个 topic，每个 topic 有不同类型的事件
    let mut rx1 = queue.subscribe::<KLineUpdateEvent>("topic1", None);
    let mut rx2 = queue.subscribe::<String>("topic2", None);

    // 发送不同类型的事件到不同 topic
    queue.send("topic1", create_test_event(1), None).unwrap();
    queue.send("topic2", "Hello, World!".to_string(), None).unwrap();

    // 接收验证
    let handle1 = tokio::spawn(async move {
        if let Ok(event) = rx1.recv().await {
            println!("Topic1 received: {:?}", event.window);
            assert_eq!(event.window, TimeWindow::Minute);
        }
    });

    let handle2 = tokio::spawn(async move {
        if let Ok(event) = rx2.recv().await {
            println!("Topic2 received: {}", event);
            assert_eq!(event, "Hello, World!");
        }
    });

    handle1.await.unwrap();
    handle2.await.unwrap();

    // 验证 topic 列表
    let topics = queue.topics();
    assert!(topics.contains(&"topic1".to_string()));
    assert!(topics.contains(&"topic2".to_string()));
    println!("All topics: {:?}", topics);
}


fn create_test_event(window: usize) -> KLineUpdateEvent {
    let time_window = match window {
        0 => TimeWindow::Second,
        1 => TimeWindow::Minute,
        2 => TimeWindow::FifteenMin,
        _ => TimeWindow::Hour
    };

    KLineUpdateEvent {
        window: time_window,
        ohlc: OHLC::new(1600000000, 100.0, 1.0),
        is_new_window: true
    }
}
*/
