use crate::DomainReadModel;

/// 从已加载领域读模型计算查询业务结果的 core use case。
pub trait QueryUseCase<Q, R>: Send + Sync
where
    R: DomainReadModel,
{
    type Output;
    type Error;

    /// 校验不依赖读模型的查询输入。
    fn validate_query(&self, query: &Q) -> Result<(), Self::Error>;

    /// 基于已加载的领域事实计算查询业务结果。
    fn compute_query(&self, query: &Q, read_model: R) -> Result<Self::Output, Self::Error>;
}
