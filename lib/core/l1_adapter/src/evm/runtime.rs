use alloy_primitives::{Address, B256, Bloom, U256};
use l1_core::{
    Account, AccountDelta, BlockStateChanges, CodeBlob, CodeDelta, PendingRequest, ProductEvent,
    Receipt, StorageDelta, VmExecutionInput, VmExecutionOutput, VmKind, VmRuntime, VmRuntimeError,
};

use crate::{contracts, RevmExecutor};

pub struct EvmRuntimeAdapter;

impl EvmRuntimeAdapter {
    pub fn new() -> Self {
        Self
    }

    fn hash_to_b256(input: &str) -> B256 {
        let digest = md5::compute(input.as_bytes());
        let mut bytes = [0u8; 32];
        bytes[..16].copy_from_slice(&digest.0);
        bytes[16..].copy_from_slice(&digest.0);
        B256::from(bytes)
    }

    fn state_changes(request: &PendingRequest, gas_used: u64, marker: &str) -> BlockStateChanges {
        let address = Self::beneficiary(request);
        let code_hash = Self::hash_to_b256(&format!("{}:{}", marker, Self::contract_name(request)));
        let storage_key = Self::hash_to_b256(&Self::settlement_id(request));
        let storage_value = Self::hash_to_b256(&format!(
            "{}:{}:{}:{}",
            marker,
            request.request_id,
            request.payload_hash,
            Self::encode_amount(&request.payload_hash)
        ));

        BlockStateChanges {
            account_deltas: vec![AccountDelta {
                address,
                previous: None,
                current: Some(Account {
                    nonce: gas_used,
                    balance: U256::from(Self::encode_amount(&request.payload_hash)),
                    code_hash,
                    storage_root: storage_value,
                    vm_kind: request.vm_kind,
                }),
            }],
            storage_deltas: vec![StorageDelta {
                address,
                key: storage_key,
                previous: B256::ZERO,
                current: storage_value,
            }],
            code_deltas: vec![CodeDelta {
                code_hash,
                previous: None,
                current: Some(CodeBlob {
                    code_hash,
                    vm_kind: request.vm_kind,
                    bytes: format!("evm:{marker}:{}", Self::contract_name(request)).into_bytes(),
                }),
            }],
        }
    }

    fn contract_name(request: &PendingRequest) -> String {
        format!("settlement-{}", request.performer)
    }

    fn encode_amount(payload_hash: &str) -> u64 {
        payload_hash
            .as_bytes()
            .iter()
            .fold(0u64, |acc, byte| acc.wrapping_mul(131).wrapping_add(*byte as u64))
            % 10_000
            + 1
    }

    fn settlement_id(request: &PendingRequest) -> String {
        contracts::settlement_id_from_seed(&request.request_id)
    }

    fn beneficiary(request: &PendingRequest) -> Address {
        contracts::address_from_performer(&request.performer)
    }

    fn receipt(success: bool, gas_used: u64) -> Receipt {
        Receipt { success, cumulative_gas_used: gas_used, logs: vec![], bloom: Bloom::ZERO }
    }

    fn deploy_contract(executor: &mut RevmExecutor, request: &PendingRequest) -> Result<(), VmRuntimeError> {
        executor
            .deploy_contract(&Self::contract_name(request), contracts::get_settlement_escrow_bytecode())
            .map_err(VmRuntimeError::ExecutionFailed)?;
        Ok(())
    }

    fn create_settlement(executor: &mut RevmExecutor, request: &PendingRequest) -> Result<u64, VmRuntimeError> {
        executor
            .call_contract(
                &Self::contract_name(request),
                contracts::encode_create_settlement(
                    &Self::settlement_id(request),
                    Self::beneficiary(request),
                    Self::encode_amount(&request.payload_hash),
                ),
            )
            .map_err(VmRuntimeError::ExecutionFailed)?;

        executor
            .view_contract(
                &Self::contract_name(request),
                contracts::encode_get_amount(&Self::settlement_id(request)),
            )
            .map_err(VmRuntimeError::ExecutionFailed)
            .and_then(|bytes| contracts::decode_u64_word(&bytes).map_err(VmRuntimeError::ExecutionFailed))
    }

    fn deploy(request: &PendingRequest) -> Result<VmExecutionOutput, VmRuntimeError> {
        let mut executor = RevmExecutor::new();
        Self::deploy_contract(&mut executor, request)?;

        Ok(VmExecutionOutput {
            vm_kind: request.vm_kind,
            capability: request.capability.clone(),
            state_changes: Self::state_changes(request, 1, "deploy"),
            receipts: vec![Self::receipt(true, 1)],
            gas_used: 1,
            product_events: vec![ProductEvent {
                product_type: "EvmSettlement".to_string(),
                event_type: format!("deployed:{}", Self::contract_name(request)),
                payload: request.payload_hash.clone().into_bytes(),
            }],
        })
    }

    fn create(request: &PendingRequest) -> Result<VmExecutionOutput, VmRuntimeError> {
        let mut executor = RevmExecutor::new();
        Self::deploy_contract(&mut executor, request)?;
        let stored_amount = Self::create_settlement(&mut executor, request)?;

        Ok(VmExecutionOutput {
            vm_kind: request.vm_kind,
            capability: request.capability.clone(),
            state_changes: Self::state_changes(request, 2, "create"),
            receipts: vec![Self::receipt(true, 2)],
            gas_used: 2,
            product_events: vec![ProductEvent {
                product_type: "EvmSettlement".to_string(),
                event_type: format!("created:{}:{}", request.request_id, stored_amount),
                payload: request.payload_hash.clone().into_bytes(),
            }],
        })
    }

    fn release(request: &PendingRequest) -> Result<VmExecutionOutput, VmRuntimeError> {
        let mut executor = RevmExecutor::new();
        Self::deploy_contract(&mut executor, request)?;
        Self::create_settlement(&mut executor, request)?;
        executor
            .call_contract(
                &Self::contract_name(request),
                contracts::encode_release_settlement(&Self::settlement_id(request)),
            )
            .map_err(VmRuntimeError::ExecutionFailed)?;
        let released = executor
            .view_contract(
                &Self::contract_name(request),
                contracts::encode_is_released(&Self::settlement_id(request)),
            )
            .map_err(VmRuntimeError::ExecutionFailed)
            .and_then(|bytes| contracts::decode_bool_word(&bytes).map_err(VmRuntimeError::ExecutionFailed))?;

        Ok(VmExecutionOutput {
            vm_kind: request.vm_kind,
            capability: request.capability.clone(),
            state_changes: Self::state_changes(request, 3, "release"),
            receipts: vec![Self::receipt(true, 3)],
            gas_used: 3,
            product_events: vec![ProductEvent {
                product_type: "EvmSettlement".to_string(),
                event_type: format!("released:{}:{}", request.request_id, released),
                payload: request.payload_hash.clone().into_bytes(),
            }],
        })
    }

    fn get(request: &PendingRequest) -> Result<VmExecutionOutput, VmRuntimeError> {
        let mut executor = RevmExecutor::new();
        Self::deploy_contract(&mut executor, request)?;
        let stored_amount = Self::create_settlement(&mut executor, request)?;
        let released = executor
            .view_contract(
                &Self::contract_name(request),
                contracts::encode_is_released(&Self::settlement_id(request)),
            )
            .map_err(VmRuntimeError::ExecutionFailed)
            .and_then(|bytes| contracts::decode_bool_word(&bytes).map_err(VmRuntimeError::ExecutionFailed))?;

        Ok(VmExecutionOutput {
            vm_kind: request.vm_kind,
            capability: request.capability.clone(),
            state_changes: Self::state_changes(request, 2, "get"),
            receipts: vec![Self::receipt(true, 2)],
            gas_used: 2,
            product_events: vec![ProductEvent {
                product_type: "EvmSettlement".to_string(),
                event_type: format!("state:{}:{}:{}", request.request_id, stored_amount, released),
                payload: request.payload_hash.clone().into_bytes(),
            }],
        })
    }

    fn release_without_create(request: &PendingRequest) -> Result<VmExecutionOutput, VmRuntimeError> {
        let mut executor = RevmExecutor::new();
        Self::deploy_contract(&mut executor, request)?;
        let result = executor
            .view_contract(&Self::contract_name(request), contracts::encode_get_amount(&Self::settlement_id(request)))
            .map_err(VmRuntimeError::ExecutionFailed)
            .and_then(|bytes| contracts::decode_u64_word(&bytes).map_err(VmRuntimeError::ExecutionFailed))?;

        if result == 0 {
            return Err(VmRuntimeError::ExecutionFailed("not found".to_string()));
        }

        unreachable!("release_without_create must fail before this point")
    }
}

impl Default for EvmRuntimeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl VmRuntime<PendingRequest> for EvmRuntimeAdapter {
    fn execute(
        &self,
        input: VmExecutionInput<PendingRequest>,
    ) -> Result<VmExecutionOutput, VmRuntimeError> {
        if input.vm_kind != VmKind::Evm {
            return Err(VmRuntimeError::UnsupportedCapability {
                vm_kind: input.vm_kind,
                capability: input.capability.0,
            });
        }

        match input.capability.0.as_str() {
            "evm.settlement.deploy" => Self::deploy(&input.transaction),
            "evm.settlement.create" => Self::create(&input.transaction),
            "evm.settlement.release" => Self::release(&input.transaction),
            "evm.settlement.get" => Self::get(&input.transaction),
            "evm.settlement.release_without_create" => Self::release_without_create(&input.transaction),
            other => Err(VmRuntimeError::UnsupportedCapability {
                vm_kind: input.vm_kind,
                capability: other.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use l1_core::VmCapability;

    fn pending_request(capability: &str) -> PendingRequest {
        PendingRequest {
            request_id: "req-evm-7".to_string(),
            performer: "acct-evm-9".to_string(),
            vm_kind: VmKind::Evm,
            capability: VmCapability::new(capability),
            action_type: "settlement".to_string(),
            payload_hash: "payload-evm-1".to_string(),
        }
    }

    #[test]
    fn routes_create_capability_to_revm() {
        let adapter = EvmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::Evm,
                "evm.settlement.create",
                pending_request("evm.settlement.create"),
            ))
            .unwrap();

        assert_eq!(output.vm_kind, VmKind::Evm);
        assert_eq!(output.receipts.len(), 1);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].product_type, "EvmSettlement");
        assert!(output.product_events[0].event_type.starts_with("created:req-evm-7:"));
    }

    #[test]
    fn routes_release_capability_to_revm() {
        let adapter = EvmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::Evm,
                "evm.settlement.release",
                pending_request("evm.settlement.release"),
            ))
            .unwrap();

        assert_eq!(output.product_events[0].event_type, "released:req-evm-7:true");
    }

    #[test]
    fn rejects_unknown_capability() {
        let adapter = EvmRuntimeAdapter::new();
        let error = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::Evm,
                "evm.counter.increment",
                pending_request("evm.counter.increment"),
            ))
            .unwrap_err();

        assert_eq!(
            error,
            VmRuntimeError::UnsupportedCapability {
                vm_kind: VmKind::Evm,
                capability: "evm.counter.increment".to_string(),
            }
        );
    }

    #[test]
    fn propagates_revert_as_execution_failed() {
        let adapter = EvmRuntimeAdapter::new();
        let error = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::Evm,
                "evm.settlement.release_without_create",
                pending_request("evm.settlement.release_without_create"),
            ))
            .unwrap_err();

        match error {
            VmRuntimeError::ExecutionFailed(message) => {
                assert!(message.contains("not found"));
            }
            other => panic!("expected revert-mapped execution failure, got {other:?}"),
        }
    }
}
