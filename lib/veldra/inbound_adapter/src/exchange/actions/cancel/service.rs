use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::cancel::reply::{
    CancelResponseDataWire, CancelResponseEnvelopeWire, CancelResponseWire, CancelStatusWire,
};
use crate::exchange::actions::cancel::wire::CancelRequestWire;
use crate::exchange::error::ExchangeHttpError;

pub fn execute(
    request: CancelRequestWire,
    _deps: &ExchangeActionDeps,
) -> Result<CancelResponseWire, ExchangeHttpError> {
    let statuses =
        request.action.cancels.iter().map(|_| CancelStatusWire::Success("success")).collect();
    Ok(CancelResponseWire {
        status: "ok",
        response: CancelResponseEnvelopeWire {
            type_: "cancel",
            data: CancelResponseDataWire { statuses },
        },
    })
}
