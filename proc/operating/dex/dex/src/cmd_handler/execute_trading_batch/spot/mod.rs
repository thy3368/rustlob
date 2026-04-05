mod amend_order;
mod cancel_order;
mod place_order;
mod shared;

use base_types::handler::handler_update::ApplyCommandChanges;

use crate::cmd_handler::execute_trading_batch::context::{
    ExecuteTradingBatchContext, SpotCommandChangeSet, SpotCommandState,
};
use crate::cmd_handler::execute_trading_batch::ExecuteTradingBatchError;
use crate::cmd_handler::execute_trading_batch_handler::ExecuteTradingBatchHandler;
use crate::cmd_handler::{ExchangeCommandEnvelope, SpotCommand};

use self::amend_order::AmendOrderApplier;
use self::cancel_order::CancelOrderApplier;
use self::place_order::PlaceOrderApplier;

pub(crate) fn handle_spot_command(
    handler: &ExecuteTradingBatchHandler,
    envelope: &ExchangeCommandEnvelope,
    command: &SpotCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    match command {
        SpotCommand::PlaceOrder(command) => {
            let changes = PlaceOrderApplier.apply_command_and_collect_changes(command, SpotCommandState)?;
            apply_changeset(changes, ctx)
        }
        SpotCommand::CancelOrder(command) => {
            let changes = CancelOrderApplier.apply_command_and_collect_changes(command, SpotCommandState)?;
            apply_changeset(changes, ctx)
        }
        SpotCommand::AmendOrder(command) => {
            let changes = AmendOrderApplier.apply_command_and_collect_changes(command, SpotCommandState)?;
            apply_changeset(changes, ctx)
        }
    }
}

fn apply_changeset(
    changes: SpotCommandChangeSet,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += changes.writes.summary.accepted_commands;
    ctx.writes.summary.orders_created += changes.writes.summary.orders_created;
    ctx.writes.summary.trades_executed += changes.writes.summary.trades_executed;
    ctx.writes.summary.balance_updates += changes.writes.summary.balance_updates;
    ctx.writes.orders.extend(changes.writes.orders);
    ctx.writes.trades.extend(changes.writes.trades);
    ctx.writes.balance_deltas.extend(changes.writes.balance_deltas);
    ctx.changelogs.extend(changes.changelogs);
    Ok(())
}
