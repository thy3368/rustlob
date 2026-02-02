use id_generator::generator::IdGenerator;
use once_cell::sync::Lazy;


static ORDER_ID_GEN: Lazy<IdGenerator> = Lazy::new(|| {
    let node_id = std::env::var("NODE_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    IdGenerator::new(node_id)
});


// 公共访问方法
pub fn order_next_id() -> i64 { ORDER_ID_GEN.next_id() }


#[test]
fn abc() {
    let id = order_next_id();
    println!("✅ {} unique IDs", id);
}
