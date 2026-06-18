use std::future::Future;
use std::pin::Pin;

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::error::ExchangeHttpError;

type ActionFuture<'a, Reply> = Pin<Box<dyn Future<Output = Result<Reply, ExchangeHttpError>> + 'a>>;

pub async fn run_action<Request, Reply, ParseFn, ValidateFn, ExecuteFn>(
    body: &[u8],
    deps: &ExchangeActionDeps,
    parse: ParseFn,
    validate: ValidateFn,
    execute: ExecuteFn,
) -> Result<Reply, ExchangeHttpError>
where
    ParseFn: FnOnce(&[u8]) -> Result<Request, ExchangeHttpError>,
    ValidateFn: FnOnce(&Request) -> Result<(), ExchangeHttpError>,
    ExecuteFn: for<'a> FnOnce(Request, &'a ExchangeActionDeps) -> ActionFuture<'a, Reply>,
{
    let request = parse(body)?;
    validate(&request)?;
    execute(request, deps).await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use crate::exchange::actions::ExchangeActionDeps;
    use crate::exchange::common::runner::run_action;
    use crate::exchange::error::ExchangeHttpError;

    #[actix_web::test]
    async fn parse_error_returns_immediately() {
        let deps = ExchangeActionDeps::default();
        let result = run_action(
            br#"{"invalid":true}"#,
            &deps,
            |_| Err(ExchangeHttpError::MalformedJson),
            |_| Ok(()),
            |_: (), _| Box::pin(async { Ok(()) }),
        )
        .await;

        assert!(matches!(result, Err(ExchangeHttpError::MalformedJson)));
    }

    #[actix_web::test]
    async fn validate_error_skips_execute() {
        let deps = ExchangeActionDeps::default();
        let executed = Arc::new(AtomicBool::new(false));
        let executed_in_execute = Arc::clone(&executed);
        let result = run_action(
            br#"{"ok":true}"#,
            &deps,
            |_| Ok("request"),
            |_| Err(ExchangeHttpError::UnsupportedActionType("order".to_string())),
            move |_, _| {
                Box::pin(async move {
                    executed_in_execute.store(true, Ordering::SeqCst);
                    Ok(())
                })
            },
        )
        .await;

        assert!(matches!(result, Err(ExchangeHttpError::UnsupportedActionType(_))));
        assert!(!executed.load(Ordering::SeqCst));
    }

    #[actix_web::test]
    async fn execute_error_is_returned_as_is() {
        let deps = ExchangeActionDeps::default();
        let result: Result<(), ExchangeHttpError> = run_action(
            br#"{"ok":true}"#,
            &deps,
            |_| Ok("request"),
            |_| Ok(()),
            |_, _| {
                Box::pin(async {
                    Err(ExchangeHttpError::InvalidJsonShape("missing field".to_string()))
                })
            },
        )
        .await;

        assert!(
            matches!(result, Err(ExchangeHttpError::InvalidJsonShape(message)) if message == "missing field")
        );
    }
}
