use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::noop::reply::NoopResponseWire;
use crate::exchange::common::wire::DefaultExchangeResponseEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

pub fn execute(_deps: &ExchangeActionDeps) -> Result<NoopResponseWire, ExchangeHttpError> {
    Ok(NoopResponseWire {
        status: "ok",
        response: DefaultExchangeResponseEnvelopeWire { type_: "default" },
    })
}
