use crate::command_use_case_def2::IssuedByParty;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QueryMeta {
    /// Tracing correlation id for spans/logs across service hops.
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QueryEnvelope<Q> {
    pub meta: QueryMeta,
    pub query: Q,
}

/// 更贴近 Use Cases（用例）的查询型抽象：
/// 只定义业务输入、读模型校验与业务视图产出。
pub trait QueryUseCase: Send + Sync {
    /// 对应 cqrs 的 query
    type Query: IssuedByParty;

    /// 对应 clean 架构里查询侧输入到 use case 的业务读模型。
    type ReadModel;

    /// query use case 的业务结果，不是 transport response。
    type View;

    /// core.use_case 只表达业务错误。
    type Error: std::error::Error;

    /// 对应四色建模的 role。
    fn role(&self) -> &'static str {
        "UnknownActor用来做权限控制和追溯"
    }

    /// 对 query 自身做快速检查。
    fn pre_check_query(&self, _query: &Self::Query) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 基于 read model 的业务校验，可为空。
    fn validate_against_read_model(
        &self,
        _query: &Self::Query,
        _read_model: &Self::ReadModel,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 计算业务视图，核心方法，测试要覆盖 query/read-model 矩阵。
    fn compute_view(
        &self,
        query: &Self::Query,
        read_model: Self::ReadModel,
    ) -> Result<Self::View, Self::Error>;
}
