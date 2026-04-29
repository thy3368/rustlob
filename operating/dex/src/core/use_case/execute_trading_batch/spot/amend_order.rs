use base_types::handler::handler_update::ApplyCommandChanges;

use crate::core::use_case::execute_trading_batch::context::{SpotCommandChangeSet, SpotCommandState};
use crate::core::use_case::execute_trading_batch::ExecuteTradingBatchError;
use crate::core::use_case::execute_trading_batch_handler::ExecutedBatchBlock;
use crate::core::SpotAmendOrderCmd;

pub(in crate::core) struct AmendOrderApplier;

impl<'a> ApplyCommandChanges<
    SpotAmendOrderCmd,
    SpotCommandState<'a>,
    ExecutedBatchBlock,
    crate::core::TradeExecutionLog,
    ExecuteTradingBatchError,
> for AmendOrderApplier
{
    fn apply_command_and_collect_changes(
        &self,
        _cmd: &SpotAmendOrderCmd,
        _state_set: SpotCommandState<'a>,
    ) -> Result<SpotCommandChangeSet, ExecuteTradingBatchError> {
        Ok(SpotCommandChangeSet {
            writes: ExecutedBatchBlock {
                summary: crate::core::BatchExecutionSummary {
                    accepted_commands: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            changelogs: Vec::new(),
        })
    }
}
