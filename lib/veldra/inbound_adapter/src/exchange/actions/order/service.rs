use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::actions::order::error::OrderContractError;
use crate::exchange::actions::order::reply::{
    FilledOrderStatusWire, OrderResponseDataWire, OrderResponseEnvelopeWire, OrderResponseWire,
    OrderStatusWire, RestingOrderStatusWire,
};
use crate::exchange::actions::order::wire::OrderRequestWire;

const STUB_FILLED_PREFIX: &str = "stub-filled";
const STUB_ERROR_PREFIX: &str = "stub-error";
pub const STUB_ERROR_MESSAGE: &str = "Order must have minimum value of $10.";
const STUB_OID_BASE: u64 = 77738308;

pub fn execute(
    request: OrderRequestWire,
    _deps: &ExchangeActionDeps,
) -> Result<OrderResponseWire, OrderContractError> {
    // 这里只是 adapter contract stub。后续接 use case 时替换数据来源，不改外部 JSON 形状。
    let statuses = request
        .action
        .orders
        .iter()
        .enumerate()
        .map(|(index, order)| {
            let oid = STUB_OID_BASE + index as u64;
            match order.c.as_deref() {
                Some(cloid) if cloid.starts_with(STUB_FILLED_PREFIX) => OrderStatusWire::Filled {
                    filled: FilledOrderStatusWire {
                        total_sz: order.s.clone(),
                        avg_px: order.p.clone(),
                        oid,
                    },
                },
                Some(cloid) if cloid.starts_with(STUB_ERROR_PREFIX) => {
                    OrderStatusWire::Error { error: STUB_ERROR_MESSAGE.to_string() }
                }
                _ => OrderStatusWire::Resting { resting: RestingOrderStatusWire { oid } },
            }
        })
        .collect();

    Ok(OrderResponseWire {
        status: "ok",
        response: OrderResponseEnvelopeWire {
            type_: "order",
            data: OrderResponseDataWire { statuses },
        },
    })
}
