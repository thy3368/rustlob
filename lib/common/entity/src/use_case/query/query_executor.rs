use super::QueryUseCase;
use crate::DomainReadModel;

/// 根据查询加载结构一致领域事实的 outbound port。
///
/// source 不判断这些事实是否被某个具体查询用例接受。同一个 source 加载的读模型
/// 可以交给多个实现 [`QueryUseCase`] 的具体查询用例解释。
pub trait ReadModelSource<Q>: Send + Sync {
    type ReadModel: DomainReadModel;
    type Error: std::error::Error;

    fn load_read_model(&self, query: &Q) -> Result<Self::ReadModel, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryExecutionError<BE, OE> {
    Business(BE),
    LoadReadModel(OE),
}

/// 查询运行时编排器，只固定 `validate -> load -> compute` 的执行顺序。
#[derive(Debug, Clone, Copy, Default)]
pub struct QueryExecutor;

impl QueryExecutor {
    pub fn execute<Q, S, C>(
        &self,
        query: &Q,
        source: &S,
        use_case: &C,
    ) -> Result<C::Output, QueryExecutionError<C::Error, S::Error>>
    where
        S: ReadModelSource<Q>,
        C: QueryUseCase<Q, S::ReadModel>,
    {
        use_case.validate_query(query).map_err(QueryExecutionError::Business)?;
        let read_model =
            source.load_read_model(query).map_err(QueryExecutionError::LoadReadModel)?;
        use_case.compute_query(query, read_model).map_err(QueryExecutionError::Business)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TestReadModel(u64);

    impl DomainReadModel for TestReadModel {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("test error")
        }
    }

    impl std::error::Error for TestError {}

    struct TestSource {
        result: Result<TestReadModel, TestError>,
        calls: AtomicUsize,
    }

    impl ReadModelSource<u64> for TestSource {
        type ReadModel = TestReadModel;
        type Error = TestError;

        fn load_read_model(&self, _query: &u64) -> Result<Self::ReadModel, Self::Error> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            self.result
        }
    }

    struct AddQueryValue<'a> {
        compute_calls: &'a AtomicUsize,
    }

    impl QueryUseCase<u64, TestReadModel> for AddQueryValue<'_> {
        type Output = u64;
        type Error = TestError;

        fn validate_query(&self, query: &u64) -> Result<(), TestError> {
            (*query != 0).then_some(()).ok_or(TestError)
        }

        fn compute_query(&self, query: &u64, read_model: TestReadModel) -> Result<u64, TestError> {
            self.compute_calls.fetch_add(1, Ordering::Relaxed);
            query.checked_add(read_model.0).ok_or(TestError)
        }
    }

    struct TracedSource<'a>(&'a Mutex<Vec<&'static str>>);

    impl ReadModelSource<u64> for TracedSource<'_> {
        type ReadModel = TestReadModel;
        type Error = TestError;

        fn load_read_model(&self, _query: &u64) -> Result<Self::ReadModel, Self::Error> {
            self.0.lock().map_err(|_| TestError)?.push("load");
            Ok(TestReadModel(40))
        }
    }

    struct TracedUseCase<'a>(&'a Mutex<Vec<&'static str>>);

    impl QueryUseCase<u64, TestReadModel> for TracedUseCase<'_> {
        type Output = u64;
        type Error = TestError;

        fn validate_query(&self, _query: &u64) -> Result<(), Self::Error> {
            self.0.lock().map_err(|_| TestError)?.push("validate");
            Ok(())
        }

        fn compute_query(
            &self,
            query: &u64,
            read_model: TestReadModel,
        ) -> Result<Self::Output, Self::Error> {
            self.0.lock().map_err(|_| TestError)?.push("compute");
            Ok(query + read_model.0)
        }
    }

    #[test]
    fn invalid_query_is_mapped_without_loading_or_computing() {
        let compute_calls = AtomicUsize::new(0);
        let source = TestSource { result: Ok(TestReadModel(40)), calls: AtomicUsize::new(0) };

        let result =
            QueryExecutor.execute(&0, &source, &AddQueryValue { compute_calls: &compute_calls });

        assert_eq!(result, Err(QueryExecutionError::Business(TestError)));
        assert_eq!(source.calls.load(Ordering::Relaxed), 0);
        assert_eq!(compute_calls.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn source_error_is_mapped_without_calling_use_case() {
        let compute_calls = AtomicUsize::new(0);
        let result = QueryExecutor.execute(
            &2,
            &TestSource { result: Err(TestError), calls: AtomicUsize::new(0) },
            &AddQueryValue { compute_calls: &compute_calls },
        );

        assert_eq!(result, Err(QueryExecutionError::LoadReadModel(TestError)));
        assert_eq!(compute_calls.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn successful_execution_returns_core_output() {
        let calls = Mutex::new(Vec::new());

        assert_eq!(
            QueryExecutor.execute(&2, &TracedSource(&calls), &TracedUseCase(&calls)),
            Ok(42)
        );
        assert_eq!(
            calls.into_inner().expect("调用记录锁不应中毒"),
            ["validate", "load", "compute"]
        );
    }
}
