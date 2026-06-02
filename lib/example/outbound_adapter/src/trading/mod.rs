mod place_order_in_memory;
mod place_order_mysql;

pub use place_order_in_memory::InMemoryPlaceOrderOutbound;
pub use place_order_mysql::MySqlPlaceOrderOutbound;
