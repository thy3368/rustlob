use base_types::handler::handler_update::ApplyCommandChanges;

use crate::core::use_case::execute_trading_batch::context::SpotCommandState;
use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch::RestingSpotOrder;
use crate::core::use_case::execute_trading_batch::spot::handler::SpotBatchHandler;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, ExecutedOrder};
use crate::core::SpotPlaceOrderCmd;

use super::shared::{build_spot_order, match_spot_order};

pub(in crate::core) struct PlaceOrderApplier;

impl<'a> ApplyCommandChanges<
    SpotPlaceOrderCmd,
    SpotCommandState<'a>,
    ExecutedBatchBlock,
    crate::core::TradeExecutionLog,
    ExecuteTradingBatchError,
> for PlaceOrderApplier
{
    fn apply_command_and_collect_changes(
        &self,
        command: &SpotPlaceOrderCmd,
        state: SpotCommandState<'a>,
    ) -> Result<crate::core::use_case::execute_trading_batch::context::SpotCommandChangeSet, ExecuteTradingBatchError> {
        let mut writes = ExecutedBatchBlock {
            summary: crate::core::BatchExecutionSummary {
                accepted_commands: 1,
                orders_created: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut changelogs = Vec::new();

        let (handler, envelope, spot_order_book) = state;
        let mut resting_order = RestingSpotOrder {
            order_id: envelope.command_id,
            trader_id: envelope.trader_id,
            market: command.market.clone(),
            side: command.side.clone(),
            price: command.price,
            original_quantity: command.quantity,
            remaining_quantity: command.quantity,
        };

        match_spot_order(
            handler,
            spot_order_book,
            &mut resting_order,
            &mut writes,
            &mut changelogs,
        )?;

        if resting_order.remaining_quantity > 0 {
            spot_order_book
                .entry(resting_order.market.clone())
                .or_default()
                .push(resting_order.clone());
        }

        let spot_order = build_spot_order(envelope, command, &resting_order)?;
        writes.orders.push(ExecutedOrder::SpotOrder(spot_order));
        writes.summary.balance_updates = writes.balance_deltas.len();

        Ok(crate::core::use_case::execute_trading_batch::context::SpotCommandChangeSet { writes, changelogs })
    }
}
