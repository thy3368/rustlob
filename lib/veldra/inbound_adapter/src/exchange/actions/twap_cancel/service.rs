use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::twap_cancel::reply::{
    TwapCancelResponseDataWire, TwapCancelResponseEnvelopeWire, TwapCancelResponseWire,
    TwapCancelStatusWire,
};
use crate::exchange::error::ExchangeHttpError;

pub fn execute(_deps: &ExchangeActionDeps) -> Result<TwapCancelResponseWire, ExchangeHttpError> {
    Ok(TwapCancelResponseWire {
        status: "ok",
        response: TwapCancelResponseEnvelopeWire {
            type_: "twapCancel",
            data: TwapCancelResponseDataWire { status: TwapCancelStatusWire::Success("success") },
        },
    })
}
