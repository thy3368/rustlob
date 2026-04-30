use l1_core::{PendingRequest, VmRuntimeError};

use crate::core::{
    ExchangeCommand, ExchangeCommandEnvelope, ProductType, SpotCommand, SpotPlaceOrderCmd,
    SpotSide, TradingCommand,
};

use super::shared::parse_request_ids;

pub(super) fn build_spot_envelope(
    request: &PendingRequest,
) -> Result<ExchangeCommandEnvelope, VmRuntimeError> {
    let (command_id, trader_id) = parse_request_ids(request);

    Ok(ExchangeCommandEnvelope {
        command_id,
        trader_id,
        nonce: command_id,
        timestamp_ns: 1_000 + command_id,
        product_type: ProductType::Spot,
        command: ExchangeCommand::TradingCommand(TradingCommand::Spot(SpotCommand::PlaceOrder(
            SpotPlaceOrderCmd {
                trader_id,
                market: "BTC-USDT".into(),
                side: SpotSide::Buy,
                price: 100_000,
                quantity: 1,
            },
        ))),
    })
}
