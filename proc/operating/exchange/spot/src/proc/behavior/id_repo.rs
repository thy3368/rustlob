use once_cell::sync::Lazy;
use id_generator::generator::IdGenerator;


pub struct IdRepo {}

/// Order ID生成器全局实例
static ORDER_ID_GEN: Lazy<IdGenerator> = Lazy::new(|| {
    let node_id = std::env::var("NODE_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    IdGenerator::new(node_id)
});

/// Trade ID生成器全局实例
static TRADE_ID_GEN: Lazy<IdGenerator> = Lazy::new(|| {
    let node_id = std::env::var("NODE_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    IdGenerator::new(node_id)
});

/// Event ID生成器全局实例
static EVENT_ID_GEN: Lazy<IdGenerator> = Lazy::new(|| {
    let node_id = std::env::var("NODE_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    IdGenerator::new(node_id)
});

/// Trans ID生成器全局实例
static TRANS_ID_GEN: Lazy<IdGenerator> = Lazy::new(|| {
    let node_id = std::env::var("NODE_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    IdGenerator::new(node_id)
});

impl IdRepo {
    pub fn next_event_id(&mut self) -> i64 { EVENT_ID_GEN.next_id() }

    pub fn next_transaction_id(&mut self) -> i64 { TRANS_ID_GEN.next_id() }

    pub fn next_order_id(&mut self) -> i64 { ORDER_ID_GEN.next_id() }

    pub fn next_trade_id(&mut self) -> i64 { TRADE_ID_GEN.next_id() }

    // 兼容 u64 的方法（用于事件系统）
    pub fn next_event_id_u64(&mut self) -> u64 { self.next_event_id() as u64 }

    pub fn next_transaction_id_u64(&mut self) -> u64 { self.next_transaction_id() as u64 }
}
