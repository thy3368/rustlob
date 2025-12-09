/// 订单仓储模块
///
/// 遵循Clean Architecture的仓储模式，将数据访问逻辑与业务逻辑分离
///
/// ## 模块结构
///
/// - `traits`: 仓储接口定义（OrderRepository）
/// - `errors`: 错误类型定义（RepositoryError）
/// - `in_memory`: 内存仓储实现（InMemoryOrderRepository）
///
/// ## 使用示例
///
/// ```no_run
/// use lob::lob::{MemoryOrderRepo, OrderRepo};
///
/// let mut repo = MemoryOrderRepo::new(100_000, 1000);
/// let order_id = repo.allocate_order_id();
/// ```
// 子模块
pub mod traits;

// 重新导出公共接口
pub use crate::lob::adaptor::outbound::order_memory_repo::MemoryOrderRepo;
pub use traits::{OrderRepo, RepositoryError};
