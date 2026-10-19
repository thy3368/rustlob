// /// 运行时生成实体 ID 的独立能力。
// ///
// /// 该 trait 不要求实现者同时实现 [`Entity`]，用于让实体创建流程依赖
// /// ID 生成能力，而不把 ID 生成职责绑定到实体生命周期契约。
pub trait EntityIdGenerator {
    fn next_id(&self) -> u64;
}
