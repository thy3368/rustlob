/// 领域读模型标识。
///
/// 表示该类型属于 query/projection/read-side 语义：
/// 只承载已装配事实、快照或派生查询状态，不作为命令写侧 aggregate 使用。
pub trait DomainReadModel {}

/// 领域快照标识。
///
/// 表示该读模型是某个业务时点上已装配事实与派生状态的截面。
pub trait DomainReadSnapshot: DomainReadModel {}
