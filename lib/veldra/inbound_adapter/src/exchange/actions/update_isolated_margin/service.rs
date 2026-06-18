use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::update_isolated_margin::reply::UpdateIsolatedMarginResponseWire;
use crate::exchange::common::wire::DefaultExchangeResponseEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

pub fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<UpdateIsolatedMarginResponseWire, ExchangeHttpError> {
    Ok(UpdateIsolatedMarginResponseWire {
        status: "ok",
        response: DefaultExchangeResponseEnvelopeWire { type_: "default" },
    })
}
