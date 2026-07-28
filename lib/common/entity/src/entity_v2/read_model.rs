/// 领域读模型标识。
///
/// 表示该类型属于 query/read-side 语义：
/// 只承载已装配事实、快照或派生查询状态，不作为命令写侧 aggregate 使用。
///
/// 读模型不要求具备独立持久化身份、版本递增、diff 或 replay 语义。
/// 它可以由 DB、API、缓存、事件投影或多个领域事实临时装配出来；
/// 是否缓存或序列化属于 adapter / infra 选择，不是该类型的核心领域身份。
pub trait DomainReadModel {}

/// 领域读快照标识。
///
/// 表示该读模型是某个业务时点上已装配事实与派生状态的截面。
/// 快照可以被重新计算或重新装载，不要求作为权威写侧状态持久化。
pub trait DomainReadSnapshot: DomainReadModel {}
