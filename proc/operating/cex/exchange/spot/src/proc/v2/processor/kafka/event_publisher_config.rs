use std::sync::Arc;

use crate::proc::v2::processor::kafka::event_publisher::{
    EventPublisher, PublisherConfig, PublisherConfigExt, PublisherFactory,
};

pub fn create_dev_publisher() -> Result<Arc<dyn EventPublisher>, String> {
    PublisherConfig::low_latency("localhost:9092")
        .order_topic("dev-order-logs")
        .balance_topic("dev-balance-logs")
        .trade_topic("dev-trade-logs")
        .batch_size(10)
        .send_timeout(1000)
        .into_publisher()
}

pub fn create_prod_publisher(kafka_brokers: &str) -> Result<Arc<dyn EventPublisher>, String> {
    PublisherConfig::low_latency(kafka_brokers)
        .order_topic("order-logs")
        .balance_topic("balance-logs")
        .trade_topic("trade-logs")
        .batch_size(1000)
        .into_publisher()
}

pub fn create_distributed_publisher(
    kafka_brokers: &str,
) -> Result<Arc<dyn EventPublisher>, String> {
    PublisherConfig::persistent(kafka_brokers)
        .order_topic("prod-order-logs")
        .balance_topic("prod-balance-logs")
        .trade_topic("prod-trade-logs")
        .batch_size(500)
        .into_publisher()
}

pub fn create_from_env() -> Result<Arc<dyn EventPublisher>, String> {
    let kafka_brokers =
        std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());

    let config = PublisherConfig::persistent(&kafka_brokers)
        .order_topic(std::env::var("ORDER_LOG_TOPIC").unwrap_or_else(|_| "order-logs".to_string()))
        .balance_topic(
            std::env::var("BALANCE_LOG_TOPIC").unwrap_or_else(|_| "balance-logs".to_string()),
        )
        .trade_topic(std::env::var("TRADE_LOG_TOPIC").unwrap_or_else(|_| "trade-logs".to_string()))
        .batch_size(
            std::env::var("BATCH_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
        )
        .send_timeout(
            std::env::var("SEND_TIMEOUT_MS")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .unwrap_or(5000),
        );

    PublisherFactory::create(config)
}

pub fn create_test_publisher() -> Result<Arc<dyn EventPublisher>, String> {
    PublisherConfig::persistent("localhost:9092")
        .order_topic("test-order-logs")
        .balance_topic("test-balance-logs")
        .trade_topic("test-trade-logs")
        .batch_size(10)
        .send_timeout(500)
        .into_publisher()
}

#[cfg(test)]
mod tests {
    use diff::{ChangeLog, ChangeType, FieldChange};

    use super::*;

    fn create_test_log() -> ChangeLog {
        ChangeLog::new(
            "test-123".to_string(),
            "Order".to_string(),
            ChangeType::Created { fields: Vec::new() },
            1000,
            1,
        )
    }

    #[test]
    fn test_dev_publisher() {
        let publisher = create_dev_publisher().unwrap();
        assert!(publisher.publish_order_log(&create_test_log()).is_ok());
    }

    #[test]
    fn test_test_publisher() {
        let publisher = create_test_publisher().unwrap();
        assert!(publisher.publish_order_logs(&[create_test_log(), create_test_log()]).is_ok());
    }
}
