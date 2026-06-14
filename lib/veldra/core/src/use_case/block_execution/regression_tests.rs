use std::sync::Arc;

use cmd_handler::command_use_case_def2::CommandUseCase3;

use super::test_support::{UnsupportedSpotActionPlugin, sample_command, sample_state};
use super::*;
use crate::entity::{ProductPlugin, ProductPluginRegistry};

#[test]
fn role_is_block_builder() {
    assert_eq!(CommandUseCase3::role(&BuildBlockFromPendingRequestsUseCase), "BlockBuilder");
}

#[test]
fn pre_check_rejects_zero_block_height() {
    let cmd = BuildBlockFromPendingRequestsCommand { block_height: 0 };
    let result = CommandUseCase3::pre_check_command(&BuildBlockFromPendingRequestsUseCase, &cmd);
    assert_eq!(result, Err(BuildBlockError::BlockHeightMustBePositive));
}

#[test]
fn validate_rejects_empty_batch() {
    let mut state = sample_state();
    state.pending_requests.clear();
    let result = CommandUseCase3::validate_against_state(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        &state,
    );
    assert_eq!(result, Err(BuildBlockError::EmptyPendingRequests));
}

#[test]
fn validate_rejects_non_continuous_height() {
    let cmd = BuildBlockFromPendingRequestsCommand { block_height: 3 };
    let result = CommandUseCase3::validate_against_state(
        &BuildBlockFromPendingRequestsUseCase,
        &cmd,
        &sample_state(),
    );
    assert_eq!(
        result,
        Err(BuildBlockError::NonContinuousBlockHeight { parent_height: 1, actual: 3 })
    );
}

#[test]
fn validate_rejects_missing_plugin() {
    let mut state = sample_state();
    state.product_plugins = ProductPluginRegistry::default();
    let result = CommandUseCase3::validate_against_state(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        &state,
    );
    assert_eq!(
        result,
        Err(BuildBlockError::MissingProductPlugin { product_id: "spot".to_string() })
    );
}

#[test]
fn validate_rejects_unsupported_action() {
    let mut state = sample_state();
    state.product_plugins = ProductPluginRegistry::new(vec![
        Arc::new(UnsupportedSpotActionPlugin) as Arc<dyn ProductPlugin>
    ]);
    let result = CommandUseCase3::validate_against_state(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        &state,
    );
    assert_eq!(
        result,
        Err(BuildBlockError::UnsupportedAction {
            product_id: "spot".to_string(),
            action: "place_order".to_string(),
        })
    );
}

#[test]
fn validate_rejects_missing_context() {
    let mut state = sample_state();
    state.product_contexts.clear();
    let result = CommandUseCase3::validate_against_state(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        &state,
    );
    assert_eq!(
        result,
        Err(BuildBlockError::MissingProductContext { product_id: "spot".to_string() })
    );
}

#[test]
fn same_input_produces_stable_output_and_events() -> Result<(), BuildBlockError> {
    let left = CommandUseCase3::compute_output_and_events(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        sample_state(),
    )?;
    let right = CommandUseCase3::compute_output_and_events(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        sample_state(),
    )?;
    assert_eq!(left.output, right.output);
    assert_eq!(left.events, right.events);
    Ok(())
}

#[test]
fn block_commitments_change_when_request_or_event_changes() -> Result<(), BuildBlockError> {
    let baseline = CommandUseCase3::compute_output_and_events(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        sample_state(),
    )?;

    let mut modified_state = sample_state();
    modified_state.pending_requests[0].request_id = "req-2".to_string();
    let modified_request = CommandUseCase3::compute_output_and_events(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        modified_state,
    )?;

    let mut modified_state = sample_state();
    modified_state.pending_requests[0].payload = "{\"price\":101}".to_string();
    let modified_event = CommandUseCase3::compute_output_and_events(
        &BuildBlockFromPendingRequestsUseCase,
        &sample_command(),
        modified_state,
    )?;

    assert_ne!(
        baseline.output.new_block.request_ids_root,
        modified_request.output.new_block.request_ids_root
    );
    assert_ne!(baseline.output.new_block.events_root, modified_event.output.new_block.events_root);
    assert_ne!(baseline.output.new_block.block_hash, modified_request.output.new_block.block_hash);
    Ok(())
}
