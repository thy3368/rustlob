pub(crate) mod error;
pub(crate) mod reply;
mod service;
#[cfg(test)]
mod tests;
mod validate;
mod wire;

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::error::ExchangeHttpError;

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |request, deps| Box::pin(execute(request, deps))).await
}

fn parse(body: &[u8]) -> Result<wire::OrderRequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &wire::OrderRequestWire) -> Result<(), ExchangeHttpError> {
    validate::validate(request)
}

async fn execute(
    request: wire::OrderRequestWire,
    deps: &ExchangeActionDeps,
) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    service::execute(request, deps).map_err(ExchangeHttpError::OrderContract)
}
