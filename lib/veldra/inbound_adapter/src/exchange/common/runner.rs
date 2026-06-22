use std::future::Future;
use std::pin::Pin;

use actix_web::HttpResponse;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::common::parse::parse_json_request;
use crate::exchange::error::ExchangeHttpError;

pub(crate) type ExchangeActionFuture<'a, Reply> =
    Pin<Box<dyn Future<Output = Result<Reply, ExchangeHttpError>> + 'a>>;

pub(crate) trait ExchangeActionHandler {
    type Request;
    type Reply;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError>;

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply>;
}

pub(crate) async fn run_exchange_action<T>(body: &[u8]) -> Result<T::Reply, ExchangeHttpError>
where
    T: ExchangeActionHandler,
    T::Request: DeserializeOwned,
{
    let request = parse_json_request::<T::Request, ExchangeHttpError>(body)?;
    T::validate(&request)?;
    T::execute(request).await
}

pub(crate) async fn run_exchange_action_http<T>(
    body: &[u8],
) -> Result<HttpResponse, ExchangeHttpError>
where
    T: ExchangeActionHandler,
    T::Request: DeserializeOwned,
    T::Reply: Serialize,
{
    let reply = run_exchange_action::<T>(body).await?;
    Ok(HttpResponse::Ok().json(reply))
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};

    use actix_web::body::to_bytes;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::exchange::common::runner::{
        ExchangeActionFuture, ExchangeActionHandler, run_exchange_action, run_exchange_action_http,
    };
    use crate::exchange::error::ExchangeHttpError;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct ProbeRequest {
        value: bool,
    }

    struct ParseErrorAction;

    impl ExchangeActionHandler for ParseErrorAction {
        type Request = ProbeRequest;
        type Reply = ();

        fn validate(_request: &Self::Request) -> Result<(), ExchangeHttpError> {
            Ok(())
        }

        fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
            Box::pin(async { Ok(()) })
        }
    }

    struct ExecuteErrorAction;

    impl ExchangeActionHandler for ExecuteErrorAction {
        type Request = ProbeRequest;
        type Reply = ();

        fn validate(_request: &Self::Request) -> Result<(), ExchangeHttpError> {
            Ok(())
        }

        fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
            Box::pin(async {
                Err(ExchangeHttpError::InvalidJsonShape("missing field".to_string()))
            })
        }
    }

    #[derive(Deserialize)]
    struct EmptyRequest {}

    struct RequestlessAction;

    #[derive(Serialize)]
    struct RequestlessReply {
        status: &'static str,
    }

    impl ExchangeActionHandler for RequestlessAction {
        type Request = EmptyRequest;
        type Reply = RequestlessReply;

        fn validate(_request: &Self::Request) -> Result<(), ExchangeHttpError> {
            Ok(())
        }

        fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
            Box::pin(async { Ok(RequestlessReply { status: "ok" }) })
        }
    }

    #[actix_web::test]
    async fn parse_error_returns_immediately() {
        let result = run_exchange_action::<ParseErrorAction>(br#"{"value":}"#).await;

        assert!(matches!(result, Err(ExchangeHttpError::MalformedJson)));
    }

    #[actix_web::test]
    async fn validate_error_skips_execute() {
        struct ValidateErrorSkipExecuteAction;
        static EXECUTED: AtomicBool = AtomicBool::new(false);

        impl ExchangeActionHandler for ValidateErrorSkipExecuteAction {
            type Request = ProbeRequest;
            type Reply = ();

            fn validate(_request: &Self::Request) -> Result<(), ExchangeHttpError> {
                Err(ExchangeHttpError::UnsupportedActionType("order".to_string()))
            }

            fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
                Box::pin(async {
                    EXECUTED.store(true, Ordering::SeqCst);
                    Ok(())
                })
            }
        }

        EXECUTED.store(false, Ordering::SeqCst);
        let result =
            run_exchange_action::<ValidateErrorSkipExecuteAction>(br#"{"value":true}"#).await;

        assert!(matches!(result, Err(ExchangeHttpError::UnsupportedActionType(_))));
        assert!(!EXECUTED.load(Ordering::SeqCst));
    }

    #[actix_web::test]
    async fn execute_error_is_returned_as_is() {
        let result: Result<(), ExchangeHttpError> =
            run_exchange_action::<ExecuteErrorAction>(br#"{"value":true}"#).await;

        assert!(
            matches!(result, Err(ExchangeHttpError::InvalidJsonShape(message)) if message == "missing field")
        );
    }

    #[actix_web::test]
    async fn requestless_action_runs_through_shared_runner() {
        let result = run_exchange_action::<RequestlessAction>(br#"{}"#).await;

        assert_eq!(result.expect("requestless action should run").status, "ok");
    }

    #[actix_web::test]
    async fn requestless_action_runs_through_shared_http_runner() {
        let response = run_exchange_action_http::<RequestlessAction>(br#"{}"#)
            .await
            .expect("shared http runner should succeed");

        assert_eq!(response.status(), actix_web::http::StatusCode::OK);
        let body = to_bytes(response.into_body()).await.expect("response body should be readable");
        let actual: serde_json::Value =
            serde_json::from_slice(&body).expect("response body should be valid json");
        assert_eq!(actual, json!({ "status": "ok" }));
    }
}
