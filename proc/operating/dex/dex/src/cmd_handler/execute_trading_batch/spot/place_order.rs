use base_types::handler::handler_update::ApplyCommandChanges;

use crate::cmd_handler::execute_trading_batch::context::{SpotCommandChangeSet, SpotCommandState};
use crate::cmd_handler::execute_trading_batch::ExecuteTradingBatchError;
use crate::cmd_handler::execute_trading_batch_handler::{ExecutedBatchBlock, ExecutedOrder};
use crate::cmd_handler::{ExchangeCommandEnvelope, SpotPlaceOrderCmd};

use super::shared::build_spot_order;

pub(super) struct PlaceOrderApplier;

impl ApplyCommandChanges<
    SpotPlaceOrderCmd,
    SpotCommandState,
    ExecutedBatchBlock,
    crate::cmd_handler::TradeExecutionLog,
    ExecuteTradingBatchError,
> for PlaceOrderApplier
{
    fn apply_command_and_collect_changes(
        &self,
        command: &SpotPlaceOrderCmd,
        _state_set: SpotCommandState,
    ) -> Result<SpotCommandChangeSet, ExecuteTradingBatchError> {
        let envelope = ExchangeCommandEnvelope {
            command_id: 0,
            trader_id: command.trader_id,
            nonce: 0,
            timestamp_ns: 0,
            command: crate::cmd_handler::ExchangeCommand::TradingCommand(
                crate::cmd_handler::TradingCommand::Spot(crate::cmd_handler::SpotCommand::PlaceOrder(
                    command.clone(),
                )),
            ),
        };

        let resting_order = crate::cmd_handler::execute_trading_batch::RestingSpotOrder {
            order_id: envelope.command_id,
            trader_id: envelope.trader_id,
            market: command.market.clone(),
            side: command.side.clone(),
            price: command.price,
            original_quantity: command.quantity,
            remaining_quantity: command.quantity,
        };

        let spot_order = build_spot_order(&envelope, command, &resting_order)?;

        Ok(SpotCommandChangeSet {
            writes: ExecutedBatchBlock {
                summary: crate::cmd_handler::BatchExecutionSummary {
                    accepted_commands: 1,
                    orders_created: 1,
                    ..Default::default()
                },
                orders: vec![ExecutedOrder::SpotOrder(spot_order)],
                ..Default::default()
            },
            changelogs: Vec::new(),
        })
    }
}
