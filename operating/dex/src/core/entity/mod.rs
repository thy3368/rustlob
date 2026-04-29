pub mod product;

/// 实体设计目标： 扩展性 支持后续更多的交易产品上线，如事件预测等； 高性能 即支持evm，又要支持rust 虚拟机的高性能
pub use product::ProductType;

