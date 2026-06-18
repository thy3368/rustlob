use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::twap_order::reply::{
    TwapOrderResponseDataWire, TwapOrderResponseEnvelopeWire, TwapOrderResponseWire,
    TwapOrderStatusWire, TwapRunningStatusWire,
};
use crate::exchange::error::ExchangeHttpError;

const STUB_TWAP_ID: u64 = 77738308;

pub fn execute(_deps: &ExchangeActionDeps) -> Result<TwapOrderResponseWire, ExchangeHttpError> {
    Ok(TwapOrderResponseWire {
        status: "ok",
        response: TwapOrderResponseEnvelopeWire {
            type_: "twapOrder",
            data: TwapOrderResponseDataWire {
                status: TwapOrderStatusWire::Running {
                    running: TwapRunningStatusWire { twap_id: STUB_TWAP_ID },
                },
            },
        },
    })
}
