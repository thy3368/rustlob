use l1_core::{PendingRequest, VmRuntimeError};

use crate::core::{DepositCmd, ExchangeCommand, ExchangeCommandEnvelope, ProductType, TreasuryCommand};

use super::shared::parse_request_ids;

pub(super) fn build_treasury_envelope(
    request: &PendingRequest,
) -> Result<ExchangeCommandEnvelope, VmRuntimeError> {
    let (command_id, trader_id) = parse_request_ids(request);

    Ok(ExchangeCommandEnvelope {
        command_id,
        trader_id,
        nonce: command_id,
        timestamp_ns: 1_000 + command_id,
        product_type: ProductType::Treasury,
        command: ExchangeCommand::TreasuryCommand(TreasuryCommand::Deposit(DepositCmd {
            trader_id,
            asset: "USDT".into(),
            amount: 1_000,
        })),
    })
}
