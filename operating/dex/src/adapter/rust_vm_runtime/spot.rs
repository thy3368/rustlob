use serde::Deserialize;

use l1_core::{PendingRequest, VmRuntimeError};

use crate::core::{
    ExchangeCommand, ExchangeCommandEnvelope, ProductType, SpotCommand, SpotPlaceOrderCmd,
    SpotSide, TradingCommand,
};

use super::shared::parse_request_ids;

#[derive(Debug, Deserialize)]
struct SpotPayload {
    #[serde(default)]
    market: Option<String>,
    #[serde(default)]
    side: Option<SpotSide>,
    #[serde(default)]
    price: Option<u64>,
    #[serde(default)]
    quantity: Option<u64>,
}

fn spot_payload(request: &PendingRequest) -> Result<Option<SpotPayload>, VmRuntimeError> {
    request
        .payload
        .as_deref()
        .map(|payload| {
            serde_json::from_str(payload).map_err(|err| VmRuntimeError::ExecutionFailed(err.to_string()))
        })
        .transpose()
}

pub(super) fn build_spot_envelope(
    request: &PendingRequest,
) -> Result<ExchangeCommandEnvelope, VmRuntimeError> {
    let (command_id, trader_id) = parse_request_ids(request);
    let payload = spot_payload(request)?;

    Ok(ExchangeCommandEnvelope {
        command_id,
        trader_id,
        nonce: command_id,
        timestamp_ns: 1_000 + command_id,
        product_type: ProductType::Spot,
        command: ExchangeCommand::TradingCommand(TradingCommand::Spot(SpotCommand::PlaceOrder(
            SpotPlaceOrderCmd {
                trader_id,
                market: payload
                    .as_ref()
                    .and_then(|payload| payload.market.clone())
                    .unwrap_or_else(|| "BTC-USDT".into()),
                side: payload
                    .as_ref()
                    .and_then(|payload| payload.side.clone())
                    .unwrap_or(SpotSide::Buy),
                price: payload
                    .as_ref()
                    .and_then(|payload| payload.price)
                    .unwrap_or(100_000),
                quantity: payload
                    .as_ref()
                    .and_then(|payload| payload.quantity)
                    .unwrap_or(1),
            },
        ))),
    })
}
