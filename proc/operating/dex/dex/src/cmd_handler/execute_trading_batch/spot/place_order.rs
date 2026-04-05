use base_types::handler::handler_update::ApplyCommandChanges;

use crate::cmd_handler::execute_trading_batch::context::{SpotCommandChangeSet, SpotCommandState};
use crate::cmd_handler::execute_trading_batch::ExecuteTradingBatchError;
use crate::cmd_handler::execute_trading_batch::RestingSpotOrder;
use crate::cmd_handler::execute_trading_batch_handler::{ExecutedBatchBlock, ExecutedOrder};
use crate::cmd_handler::SpotPlaceOrderCmd;

use super::shared::{build_spot_order, match_spot_order};

pub(super) struct PlaceOrderApplier;

impl<'a> ApplyCommandChanges<
    SpotPlaceOrderCmd,
    SpotCommandState<'a>,
    ExecutedBatchBlock,
    crate::cmd_handler::TradeExecutionLog,
    ExecuteTradingBatchError,
> for PlaceOrderApplier
{
    fn apply_command_and_collect_changes(
        &self,
        command: &SpotPlaceOrderCmd,
        mut state: SpotCommandState<'a>,
    ) -> Result<SpotCommandChangeSet, ExecuteTradingBatchError> {
        let mut writes = ExecutedBatchBlock {
            summary: crate::cmd_handler::BatchExecutionSummary {
                accepted_commands: 1,
                orders_created: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut changelogs = Vec::new();

        let mut resting_order = RestingSpotOrder {
            order_id: state.envelope.command_id,
            trader_id: state.envelope.trader_id,
            market: command.market.clone(),
            side: command.side.clone(),
            price: command.price,
            original_quantity: command.quantity,
            remaining_quantity: command.quantity,
        };

        match_spot_order(
            state.handler,
            state.spot_order_book,
            &mut resting_order,
            &mut writes,
            &mut changelogs,
        )?;

        if resting_order.remaining_quantity > 0 {
            state
                .spot_order_book
                .entry(resting_order.market.clone())
                .or_default()
                .push(resting_order.clone());
        }

        let spot_order = build_spot_order(state.envelope, command, &resting_order)?;
        writes.orders.push(ExecutedOrder::SpotOrder(spot_order));
        writes.summary.balance_updates = writes.balance_deltas.len();

        Ok(SpotCommandChangeSet { writes, changelogs })
    }
}
