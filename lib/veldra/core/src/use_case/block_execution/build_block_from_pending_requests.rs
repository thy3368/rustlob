use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase3, UseCaseOutput};

use super::{
    BuildBlockError, BuildBlockFromPendingRequestsCommand, BuildBlockFromPendingRequestsOutput,
    BuildBlockFromPendingRequestsState,
};
use crate::entity::build_new_block;

/// 顺序执行待处理请求并构建新区块承诺。
#[derive(Debug, Clone, Copy, Default)]
pub struct BuildBlockFromPendingRequestsUseCase;

impl CommandUseCase3 for BuildBlockFromPendingRequestsUseCase {
    type Command = BuildBlockFromPendingRequestsCommand;
    type GivenState = BuildBlockFromPendingRequestsState;
    type Error = BuildBlockError;
    type Output = BuildBlockFromPendingRequestsOutput;

    fn role(&self) -> &'static str {
        "BlockBuilder"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.block_height == 0 {
            return Err(BuildBlockError::BlockHeightMustBePositive);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.pending_requests.is_empty() {
            return Err(BuildBlockError::EmptyPendingRequests);
        }

        let expected_block_height = state.parent_height + 1;
        if cmd.block_height != expected_block_height {
            return Err(BuildBlockError::NonContinuousBlockHeight {
                parent_height: state.parent_height,
                actual: cmd.block_height,
            });
        }

        for request in &state.pending_requests {
            let plugin = state.product_plugins.plugin(&request.product_id).ok_or_else(|| {
                BuildBlockError::MissingProductPlugin { product_id: request.product_id.clone() }
            })?;
            if !plugin.supports_action(&request.action) {
                return Err(BuildBlockError::UnsupportedAction {
                    product_id: request.product_id.clone(),
                    action: request.action.clone(),
                });
            }

            let context = state.product_contexts.get(&request.product_id).ok_or_else(|| {
                BuildBlockError::MissingProductContext { product_id: request.product_id.clone() }
            })?;
            if context.product_id != request.product_id {
                return Err(BuildBlockError::ProductContextMismatch {
                    expected: request.product_id.clone(),
                    actual: context.product_id.clone(),
                });
            }
        }

        Ok(())
    }

    fn compute_output_and_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {
        let BuildBlockFromPendingRequestsState {
            parent_block_hash,
            pending_requests,
            product_plugins,
            mut product_contexts,
            ..
        } = state;
        let mut request_results = Vec::with_capacity(pending_requests.len());
        let mut events = Vec::new();
        let mut next_sequence = 0_u64;

        for request in &pending_requests {
            let plugin = product_plugins.plugin(&request.product_id).ok_or_else(|| {
                BuildBlockError::MissingProductPlugin { product_id: request.product_id.clone() }
            })?;
            let context = product_contexts.get(&request.product_id).cloned().ok_or_else(|| {
                BuildBlockError::MissingProductContext { product_id: request.product_id.clone() }
            })?;
            let result = plugin.execute(request, &context)?;

            let rebased_events = rebase_events(&result.events, next_sequence);
            next_sequence += rebased_events.len() as u64;
            events.extend(rebased_events);

            let mut rebased_result = result.clone();
            rebased_result.events =
                rebase_events(&result.events, next_sequence - rebased_result.events.len() as u64);
            product_contexts
                .get_mut(&request.product_id)
                .ok_or_else(|| BuildBlockError::ApplyResultFailed {
                    product_id: request.product_id.clone(),
                })?
                .apply_result(&rebased_result)?;
            request_results.push(rebased_result);
        }

        let new_block = build_new_block(
            cmd.block_height,
            parent_block_hash,
            &pending_requests,
            &events,
            &product_contexts,
        );
        Ok(UseCaseOutput {
            output: BuildBlockFromPendingRequestsOutput { new_block, request_results },
            events,
        })
    }
}

fn rebase_events(
    events: &[EntityReplayableEvent],
    base_sequence: u64,
) -> Vec<EntityReplayableEvent> {
    events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            let mut cloned = event.clone();
            cloned.timestamp = 0;
            cloned.sequence = base_sequence + index as u64;
            cloned
        })
        .collect()
}
