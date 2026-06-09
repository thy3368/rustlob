/// Query use case 视角下统一的 outbound port。
///
/// `load_read_model` 属于 adapter.outbound，
/// executor 只依赖这一个抽象。
pub trait QueryUseCaseOutbound: Send + Sync {
    type Query;
    type ReadModel;
    type Error: std::error::Error;

    fn load_read_model(&self, query: &Self::Query) -> Result<Self::ReadModel, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryUseCaseOutboundPhase {
    LoadReadModel,
}

impl std::fmt::Display for QueryUseCaseOutboundPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadReadModel => f.write_str("load_read_model"),
        }
    }
}

impl<T> QueryUseCaseOutbound for &T
where
    T: ?Sized + QueryUseCaseOutbound,
{
    type Query = T::Query;
    type ReadModel = T::ReadModel;
    type Error = T::Error;

    fn load_read_model(&self, query: &Self::Query) -> Result<Self::ReadModel, Self::Error> {
        (*self).load_read_model(query)
    }
}
