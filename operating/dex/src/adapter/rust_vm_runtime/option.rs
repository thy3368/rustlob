use l1_core::{PendingRequest, VmRuntimeError};

use crate::core::{
    ExchangeCommand, ExchangeCommandEnvelope, OptionCommand, OptionKind, OptionPlaceOrderCmd,
    OptionSide, ProductType, TradingCommand,
};

use super::shared::parse_request_ids;

pub(super) fn build_option_envelope(
    request: &PendingRequest,
) -> Result<ExchangeCommandEnvelope, VmRuntimeError> {
    let (command_id, trader_id) = parse_request_ids(request);

    Ok(ExchangeCommandEnvelope {
        command_id,
        trader_id,
        nonce: command_id,
        timestamp_ns: 1_000 + command_id,
        product_type: ProductType::Option,
        command: ExchangeCommand::TradingCommand(TradingCommand::Option(
            OptionCommand::PlaceOrder(OptionPlaceOrderCmd {
                trader_id,
                underlying: "BTC".into(),
                expiry_ts: 1_700_000_000,
                strike_price: 100_000,
                kind: OptionKind::Call,
                side: OptionSide::Buy,
                premium: 1_000,
                quantity: 1,
            }),
        )),
    })
}
