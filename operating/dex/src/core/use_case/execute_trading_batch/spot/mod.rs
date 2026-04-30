mod amend_order;
mod cancel_order;
pub mod handler;
mod place_order;
mod shared;

use base_types::handler::handler_update::ApplyCommandChanges;

use crate::core::use_case::execute_trading_batch::context::SpotCommandChangeSet;
use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch::SpotOrderBook;
use crate::core::use_case::execute_trading_batch::spot::handler::SpotBatchHandler;
use crate::core::use_case::execute_trading_batch_handler::{ExecutedBatchBlock, TradeExecutionLog};
use crate::core::{ExchangeCommandEnvelope, SpotCommand};

use self::amend_order::AmendOrderApplier;
use self::cancel_order::CancelOrderApplier;
use self::place_order::PlaceOrderApplier;

pub(crate) fn handle_spot_command(
    handler: &SpotBatchHandler,
    envelope: &ExchangeCommandEnvelope,
    command: &SpotCommand,
    writes: &mut ExecutedBatchBlock,
    changelogs: &mut Vec<TradeExecutionLog>,
    spot_order_book: &mut SpotOrderBook,
) -> Result<(), ExecuteTradingBatchError> {
    match command {
        SpotCommand::PlaceOrder(command) => {
            let changes = PlaceOrderApplier.apply_command_and_collect_changes(
                command,
                (handler, envelope, spot_order_book),
            )?;
            apply_changeset(changes, writes, changelogs)
        }
        SpotCommand::CancelOrder(command) => {
            let changes = CancelOrderApplier.apply_command_and_collect_changes(
                command,
                (handler, envelope, spot_order_book),
            )?;
            apply_changeset(changes, writes, changelogs)
        }
        SpotCommand::AmendOrder(command) => {
            let changes = AmendOrderApplier.apply_command_and_collect_changes(
                command,
                (handler, envelope, spot_order_book),
            )?;
            apply_changeset(changes, writes, changelogs)
        }
    }
}

fn apply_changeset(
    changes: SpotCommandChangeSet,
    writes: &mut ExecutedBatchBlock,
    changelogs: &mut Vec<TradeExecutionLog>,
) -> Result<(), ExecuteTradingBatchError> {
    writes.summary.accepted_commands += changes.writes.summary.accepted_commands;
    writes.summary.orders_created += changes.writes.summary.orders_created;
    writes.summary.trades_executed += changes.writes.summary.trades_executed;
    writes.summary.balance_updates += changes.writes.summary.balance_updates;
    writes.orders.extend(changes.writes.orders);
    writes.trades.extend(changes.writes.trades);
    writes.balance_deltas.extend(changes.writes.balance_deltas);
    changelogs.extend(changes.changelogs);
    Ok(())
}
