use alloy_primitives::{Address, Bytes, U256};
use revm::{
    db::InMemoryDB,
    primitives::{AccountInfo, ExecutionResult, Output, TransactTo},
    DatabaseCommit, Evm,
};
use std::collections::HashMap;

pub struct RevmExecutor {
    db: InMemoryDB,
    contracts: HashMap<String, Address>,
    caller: Address,
}

impl RevmExecutor {
    pub fn new() -> Self {
        let mut db = InMemoryDB::default();
        let caller = Address::from([0x01; 20]);
        let account_info = AccountInfo {
            balance: U256::from(1_000_000_000_000_000_000u128),
            nonce: 0,
            code_hash: Default::default(),
            code: None,
        };
        db.insert_account_info(caller, account_info);

        Self { db, contracts: HashMap::new(), caller }
    }

    pub fn deploy_contract(&mut self, name: &str, bytecode: Vec<u8>) -> Result<Address, String> {
        let mut evm = Evm::builder()
            .with_db(&mut self.db)
            .modify_tx_env(|tx| {
                tx.caller = self.caller;
                tx.transact_to = TransactTo::Create;
                tx.data = Bytes::from(bytecode);
                tx.value = U256::from(0);
                tx.gas_limit = 10_000_000;
            })
            .build();

        let result_and_state = evm.transact().map_err(|e| format!("Transaction failed: {:?}", e))?;
        drop(evm);
        self.db.commit(result_and_state.state);
        let result = result_and_state.result;

        match result {
            ExecutionResult::Success { output: Output::Create(_, Some(address)), .. } => {
                println!("✅ 合约 '{}' 部署成功: {:?}", name, address);
                self.contracts.insert(name.to_string(), address);
                Ok(address)
            }
            ExecutionResult::Success { .. } => {
                Err("Contract deployment succeeded but no address returned".to_string())
            }
            ExecutionResult::Revert { output, .. } => {
                Err(format!("Contract deployment reverted: {:?}", output))
            }
            ExecutionResult::Halt { reason, .. } => {
                Err(format!("Contract deployment halted: {:?}", reason))
            }
        }
    }

    pub fn call_contract(&mut self, contract_name: &str, calldata: Vec<u8>) -> Result<Bytes, String> {
        let contract_address = self
            .contracts
            .get(contract_name)
            .ok_or_else(|| format!("Contract '{}' not found", contract_name))?;

        let mut evm = Evm::builder()
            .with_db(&mut self.db)
            .modify_tx_env(|tx| {
                tx.caller = self.caller;
                tx.transact_to = TransactTo::Call(*contract_address);
                tx.data = Bytes::from(calldata.clone());
                tx.value = U256::from(0);
                tx.gas_limit = 10_000_000;
            })
            .build();

        let result_and_state = evm.transact().map_err(|e| format!("Transaction failed: {:?}", e))?;
        drop(evm);
        self.db.commit(result_and_state.state);
        let result = result_and_state.result;

        match result {
            ExecutionResult::Success { output: Output::Call(output), gas_used, .. } => {
                println!("✅ 合约调用成功，Gas 使用: {}", gas_used);
                Ok(output)
            }
            ExecutionResult::Revert { output, gas_used } => {
                Err(format!("Contract call reverted (gas used: {}): {:?}", gas_used, output))
            }
            ExecutionResult::Halt { reason, gas_used } => {
                Err(format!("Contract call halted (gas used: {}): {:?}", gas_used, reason))
            }
            _ => Err("Unexpected execution result".to_string()),
        }
    }

    pub fn view_contract(&self, contract_name: &str, calldata: Vec<u8>) -> Result<Bytes, String> {
        let contract_address = self
            .contracts
            .get(contract_name)
            .ok_or_else(|| format!("Contract '{}' not found", contract_name))?;

        let mut db_clone = self.db.clone();
        let mut evm = Evm::builder()
            .with_db(&mut db_clone)
            .modify_tx_env(|tx| {
                tx.caller = self.caller;
                tx.transact_to = TransactTo::Call(*contract_address);
                tx.data = Bytes::from(calldata);
                tx.value = U256::from(0);
                tx.gas_limit = 10_000_000;
            })
            .build();

        let result = evm.transact().map_err(|e| format!("View call failed: {:?}", e))?;
        let result = result.result;

        match result {
            ExecutionResult::Success { output: Output::Call(output), .. } => Ok(output),
            ExecutionResult::Revert { output, .. } => Err(format!("View call reverted: {:?}", output)),
            ExecutionResult::Halt { reason, .. } => Err(format!("View call halted: {:?}", reason)),
            _ => Err("Unexpected execution result".to_string()),
        }
    }

    pub fn get_caller(&self) -> Address {
        self.caller
    }
}

impl Default for RevmExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = RevmExecutor::new();
        assert!(executor.contracts.is_empty());
    }
}
