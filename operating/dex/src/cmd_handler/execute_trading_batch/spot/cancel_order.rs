use base_types::handler::handler_update::ApplyCommandChanges;

use crate::cmd_handler::execute_trading_batch::context::{SpotCommandChangeSet, SpotCommandState};
use crate::cmd_handler::execute_trading_batch::ExecuteTradingBatchError;
use crate::cmd_handler::execute_trading_batch_handler::ExecutedBatchBlock;
use crate::cmd_handler::SpotCancelOrderCmd;

pub(super) struct CancelOrderApplier;

impl<'a> ApplyCommandChanges<
    SpotCancelOrderCmd,
    SpotCommandState<'a>,
    ExecutedBatchBlock,
    crate::cmd_handler::TradeExecutionLog,
    ExecuteTradingBatchError,
> for CancelOrderApplier
{
    fn apply_command_and_collect_changes(
        &self,
        _cmd: &SpotCancelOrderCmd,
        _state_set: SpotCommandState<'a>,
    ) -> Result<SpotCommandChangeSet, ExecuteTradingBatchError> {
        Ok(SpotCommandChangeSet {
            writes: ExecutedBatchBlock {
                summary: crate::cmd_handler::BatchExecutionSummary {
                    accepted_commands: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            changelogs: Vec::new(),
        })
    }
}
