use std::collections::BTreeMap;
use std::sync::Arc;

use super::test_support::{
    SequencedContextPlugin, compute_result, context, request, sample_command, sample_state,
};
use super::*;
use crate::entity::{ProductPlugin, ProductPluginRegistry};

#[test]
fn single_request_successfully_builds_block() -> Result<(), BuildBlockError> {
    let result = compute_result(&sample_command(), sample_state())?;

    assert_eq!(result.output.request_results.len(), 1);
    assert_eq!(result.events.len(), 2);
    assert_eq!(result.events[0].timestamp, 0);
    assert_eq!(result.events[0].sequence, 0);
    assert_eq!(result.events[1].timestamp, 0);
    assert_eq!(result.events[1].sequence, 1);
    assert_eq!(result.output.new_block.block_height, 2);
    assert_eq!(result.output.new_block.parent_block_hash, "parent-1");
    assert!(!result.output.new_block.request_ids_root.is_empty());
    assert!(!result.output.new_block.events_root.is_empty());
    assert!(!result.output.new_block.post_state_root.is_empty());
    assert!(!result.output.new_block.block_hash.is_empty());

    Ok(())
}

#[test]
fn sequential_requests_for_same_product_use_updated_context() -> Result<(), BuildBlockError> {
    let mut product_contexts = BTreeMap::new();
    product_contexts.insert("spot".to_string(), context("{\"step\":0}", "ctx-commitment-step-0"));
    let state = BuildBlockFromPendingRequestsState {
        parent_height: 1,
        parent_block_hash: "parent-1".to_string(),
        pending_requests: vec![request("req-1", "{\"step\":1}"), request("req-2", "{\"step\":2}")],
        product_plugins: ProductPluginRegistry::new(vec![
            Arc::new(SequencedContextPlugin) as Arc<dyn ProductPlugin>
        ]),
        product_contexts,
    };

    let result = compute_result(&sample_command(), state)?;

    assert_eq!(result.output.request_results.len(), 2);
    assert_eq!(result.output.request_results[0].request_id, "req-1");
    assert_eq!(result.output.request_results[1].request_id, "req-2");
    assert_eq!(result.output.request_results[0].next_product_context.snapshot, "{\"step\":1}");
    assert_eq!(result.output.request_results[1].next_product_context.snapshot, "{\"step\":2}");
    assert_eq!(
        result.output.request_results[1].next_product_context.commitment,
        "ctx-commitment-step-2"
    );
    assert!(!result.output.new_block.post_state_root.is_empty());

    Ok(())
}
