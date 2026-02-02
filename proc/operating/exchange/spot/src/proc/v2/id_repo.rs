use id_generator::generator::IdGenerator;
use once_cell::sync::Lazy;

// todo service/adapter/repo的实例化都可以在这，他们都是单例的

static ORDER_ID_GEN: Lazy<IdGenerator> = Lazy::new(|| {
    let node_id = std::env::var("NODE_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    IdGenerator::new(node_id)
});

pub fn order_next_id() -> i64 { ORDER_ID_GEN.next_id() }
#[test]
fn abc() {
    let id = order_next_id();
    println!("✅ {} unique IDs", id);
}
