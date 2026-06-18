use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::update_leverage::reply::UpdateLeverageResponseWire;
use crate::exchange::common::wire::DefaultExchangeResponseEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

pub fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<UpdateLeverageResponseWire, ExchangeHttpError> {
    Ok(UpdateLeverageResponseWire {
        status: "ok",
        response: DefaultExchangeResponseEnvelopeWire { type_: "default" },
    })
}
