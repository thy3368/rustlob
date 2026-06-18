use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::user_set_abstraction::reply::UserSetAbstractionResponseWire;
use crate::exchange::common::wire::DefaultExchangeResponseEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

pub fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<UserSetAbstractionResponseWire, ExchangeHttpError> {
    Ok(UserSetAbstractionResponseWire {
        status: "ok",
        response: DefaultExchangeResponseEnvelopeWire { type_: "default" },
    })
}
