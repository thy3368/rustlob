use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::approve_agent::reply::ApproveAgentResponseWire;
use crate::exchange::common::wire::DefaultExchangeResponseEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

pub fn execute(_deps: &ExchangeActionDeps) -> Result<ApproveAgentResponseWire, ExchangeHttpError> {
    Ok(ApproveAgentResponseWire {
        status: "ok",
        response: DefaultExchangeResponseEnvelopeWire { type_: "default" },
    })
}
