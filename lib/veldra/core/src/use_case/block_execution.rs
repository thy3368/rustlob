use std::collections::BTreeMap;

use cmd_handler::command_use_case_def2::{CommandUseCase3, IssuedByParty, UseCaseOutput};
use thiserror::Error;

use crate::entity::{
    NewBlock, PendingRequest, ProductContext, ProductPluginError, ProductPluginRegistry,
    RequestExecutionResult, build_new_block,
};

/// 构建新区块的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildBlockFromPendingRequestsCommand {
    /// 要构建的目标区块高度。
    pub block_height: u64,
}

impl IssuedByParty for BuildBlockFromPendingRequestsCommand {}

/// 构建新区块前已加载的业务状态。
#[derive(Debug, Clone)]
pub struct BuildBlockFromPendingRequestsState {
    /// 父区块高度。
    pub parent_height: u64,
    /// 父区块哈希。
    pub parent_block_hash: String,
    /// 待执行请求批次。
    pub pending_requests: Vec<PendingRequest>,
    /// 已注册产品插件。
    pub product_plugins: ProductPluginRegistry,
    /// 产品上下文快照。
    pub product_contexts: BTreeMap<String, ProductContext>,
}

/// 区块构建的强类型业务产出。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildBlockFromPendingRequestsOutput {
    /// 新区块承诺。
    pub new_block: NewBlock,
    /// 每个请求的业务执行结果。
    pub request_results: Vec<RequestExecutionResult>,
}

/// 区块构建业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BuildBlockError {
    #[error("block height must be greater than zero")]
    BlockHeightMustBePositive,
    #[error("pending requests batch is empty")]
    EmptyPendingRequests,
    #[error("block height {actual} is not continuous after parent height {parent_height}")]
    NonContinuousBlockHeight { parent_height: u64, actual: u64 },
    #[error("missing product plugin for '{product_id}'")]
    MissingProductPlugin { product_id: String },
    #[error("plugin for '{product_id}' does not support action '{action}'")]
    UnsupportedAction { product_id: String, action: String },
    #[error("missing product context for '{product_id}'")]
    MissingProductContext { product_id: String },
    #[error("product context '{actual}' does not match request product '{expected}'")]
    ProductContextMismatch { expected: String, actual: String },
    #[error("product plugin execution failed: {0}")]
    ProductPlugin(#[from] ProductPluginError),
    #[error("failed to apply request result back into product context: {product_id}")]
    ApplyResultFailed { product_id: String },
}

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
            if context.product_id() != request.product_id {
                return Err(BuildBlockError::ProductContextMismatch {
                    expected: request.product_id.clone(),
                    actual: context.product_id().to_string(),
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
            let request_events = result.to_events(next_sequence);
            next_sequence += request_events.len() as u64;
            product_contexts
                .get_mut(&request.product_id)
                .ok_or_else(|| BuildBlockError::ApplyResultFailed {
                    product_id: request.product_id.clone(),
                })?
                .apply_result(&result)?;
            request_results.push(result);
            events.extend(request_events);
        }

        let new_block = build_new_block(
            cmd.block_height,
            parent_block_hash,
            &pending_requests,
            &request_results,
            &events,
            &product_contexts,
        );
        Ok(UseCaseOutput {
            output: BuildBlockFromPendingRequestsOutput { new_block, request_results },
            events,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use cmd_handler::command_use_case_def2::CommandUseCase3;

    use super::*;
    use crate::entity::{
        ProductPlugin, ProductPluginRegistry, SpotBalanceSnapshot, SpotMarketRules, SpotOrderSide,
        SpotPlaceOrderPayload, SpotProductContext, SpotProductPlugin,
    };

    fn sample_command() -> BuildBlockFromPendingRequestsCommand {
        BuildBlockFromPendingRequestsCommand { block_height: 2 }
    }

    fn sample_pending_request() -> PendingRequest {
        PendingRequest {
            request_id: "req-1".to_string(),
            product_id: "spot".to_string(),
            action: "place_order".to_string(),
            payload: serde_json::to_string(&SpotPlaceOrderPayload {
                account_id: "acct-1".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: SpotOrderSide::Buy,
                price: 100,
                qty: 3,
                client_order_id: Some("cl-1".to_string()),
            })
            .unwrap(),
        }
    }

    fn sample_spot_context() -> SpotProductContext {
        let mut balances = BTreeMap::new();
        balances.insert(
            "USDT".to_string(),
            SpotBalanceSnapshot {
                asset: "USDT".to_string(),
                available: 10_000,
                reserved: 0,
                version: 1,
            },
        );
        balances.insert(
            "BTC".to_string(),
            SpotBalanceSnapshot { asset: "BTC".to_string(), available: 5, reserved: 0, version: 1 },
        );
        SpotProductContext {
            account_id: "acct-1".to_string(),
            balances,
            market_rules: SpotMarketRules {
                symbol: "BTCUSDT".to_string(),
                base_asset: "BTC".to_string(),
                quote_asset: "USDT".to_string(),
                min_price: 1,
                min_qty: 1,
            },
            next_order_sequence: 9,
            trading_enabled: true,
        }
    }

    fn sample_state() -> BuildBlockFromPendingRequestsState {
        let mut product_contexts = BTreeMap::new();
        product_contexts.insert("spot".to_string(), ProductContext::Spot(sample_spot_context()));
        BuildBlockFromPendingRequestsState {
            parent_height: 1,
            parent_block_hash: "parent-1".to_string(),
            pending_requests: vec![sample_pending_request()],
            product_plugins: ProductPluginRegistry::new(vec![
                Arc::new(SpotProductPlugin) as Arc<dyn ProductPlugin>
            ]),
            product_contexts,
        }
    }

    #[derive(Debug)]
    struct UnsupportedSpotActionPlugin;

    impl ProductPlugin for UnsupportedSpotActionPlugin {
        fn product_id(&self) -> &'static str {
            "spot"
        }

        fn supports_action(&self, _action: &str) -> bool {
            false
        }

        fn execute(
            &self,
            _request: &PendingRequest,
            _context: &ProductContext,
        ) -> Result<RequestExecutionResult, ProductPluginError> {
            unreachable!()
        }
    }

    #[test]
    fn role_is_block_builder() {
        assert_eq!(CommandUseCase3::role(&BuildBlockFromPendingRequestsUseCase), "BlockBuilder");
    }

    #[test]
    fn pre_check_rejects_zero_block_height() {
        let cmd = BuildBlockFromPendingRequestsCommand { block_height: 0 };

        let result =
            CommandUseCase3::pre_check_command(&BuildBlockFromPendingRequestsUseCase, &cmd);

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
        state.product_plugins =
            ProductPluginRegistry::new(vec![
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
    fn happy_path_builds_new_block_and_events() -> Result<(), BuildBlockError> {
        let result = CommandUseCase3::compute_output_and_events(
            &BuildBlockFromPendingRequestsUseCase,
            &sample_command(),
            sample_state(),
        )?;

        assert_eq!(result.output.request_results.len(), 1);
        assert_eq!(result.events.len(), 2);
        assert_eq!(result.output.new_block.block_height, 2);
        assert_eq!(result.output.new_block.parent_block_hash, "parent-1");
        assert_eq!(result.output.request_results[0].request_id, "req-1");
        assert!(!result.output.new_block.request_ids_root.is_empty());
        assert!(!result.output.new_block.events_root.is_empty());
        assert!(!result.output.new_block.post_state_root.is_empty());
        assert!(!result.output.new_block.block_hash.is_empty());
        Ok(())
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
        modified_state.pending_requests[0].payload =
            serde_json::to_string(&SpotPlaceOrderPayload {
                account_id: "acct-1".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: SpotOrderSide::Buy,
                price: 101,
                qty: 3,
                client_order_id: Some("cl-1".to_string()),
            })
            .unwrap();
        let modified_event = CommandUseCase3::compute_output_and_events(
            &BuildBlockFromPendingRequestsUseCase,
            &sample_command(),
            modified_state,
        )?;

        assert_ne!(
            baseline.output.new_block.request_ids_root,
            modified_request.output.new_block.request_ids_root
        );
        assert_ne!(
            baseline.output.new_block.events_root,
            modified_event.output.new_block.events_root
        );
        assert_ne!(
            baseline.output.new_block.post_state_root,
            modified_event.output.new_block.post_state_root
        );
        assert_ne!(
            baseline.output.new_block.block_hash,
            modified_request.output.new_block.block_hash
        );
        assert_ne!(
            baseline.output.new_block.block_hash,
            modified_event.output.new_block.block_hash
        );
        Ok(())
    }

    #[test]
    fn plugin_rejects_unsupported_action_independently() {
        let plugin = SpotProductPlugin;
        let request =
            PendingRequest { action: "cancel_order".to_string(), ..sample_pending_request() };

        let result = plugin.execute(&request, &ProductContext::Spot(sample_spot_context()));

        assert!(matches!(
            result,
            Err(ProductPluginError::UnsupportedAction { product_id, action })
            if product_id == "spot" && action == "cancel_order"
        ));
    }
}
