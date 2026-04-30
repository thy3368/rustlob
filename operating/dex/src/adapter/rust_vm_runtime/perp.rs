use l1_core::{PendingRequest, VmRuntimeError};

use crate::core::{
    ExchangeCommand, ExchangeCommandEnvelope, PerpCommand, PerpPlaceOrderCmd, PerpSide,
    ProductType, TradingCommand,
};

use super::shared::parse_request_ids;

pub(super) fn build_perp_envelope(
    request: &PendingRequest,
) -> Result<ExchangeCommandEnvelope, VmRuntimeError> {
    let (command_id, trader_id) = parse_request_ids(request);

    Ok(ExchangeCommandEnvelope {
        command_id,
        trader_id,
        nonce: command_id,
        timestamp_ns: 1_000 + command_id,
        product_type: ProductType::Perp,
        command: ExchangeCommand::TradingCommand(TradingCommand::Perp(PerpCommand::PlaceOrder(
            PerpPlaceOrderCmd {
                trader_id,
                market: "BTC-PERP".into(),
                side: PerpSide::Buy,
                price: 100_000,
                quantity: 2,
                leverage: 2,
                reduce_only: false,
            },
        ))),
    })
}
