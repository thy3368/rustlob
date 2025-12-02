/// 订单仓储模块
///
/// 遵循Clean Architecture的仓储模式，将数据访问逻辑与业务逻辑分离
///
/// ## 模块结构
///
/// - `traits`: 仓储接口定义（OrderRepository, RepositoryAccessor）
/// - `errors`: 错误类型定义（RepositoryError）
/// - `in_memory`: 内存仓储实现（InMemoryOrderRepository）
///
/// ## 使用示例
///
/// ```rust
/// use lob::lob::domain::::{OrderRepository, InMemoryOrderRepository};
///
/// let mut repo = InMemoryOrderRepository::new(100_000, 1000);
/// ```
// 子模块
pub mod traits;

// 重新导出公共接口
pub use crate::lob::adaptor::outbound::in_memory::InMemoryOrderRepository;
pub use traits::{OrderRepository, RepositoryAccessor, RepositoryError};
