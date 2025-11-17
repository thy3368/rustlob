use alloy_primitives::{Address, Bytes, U256};
use revm::{
    db::InMemoryDB,
    primitives::{
        AccountInfo, ExecutionResult, Output, TransactTo,
    },
    DatabaseCommit,
    Evm,
};
use std::collections::HashMap;

/// REVM 执行器 - 用于部署和执行智能合约
///
/// 这个执行器封装了 REVM 的核心功能，提供了简化的接口来：
/// - 部署智能合约
/// - 调用合约函数
/// - 查询合约状态
pub struct RevmExecutor {
    /// 内存数据库，存储账户和合约状态
    db: InMemoryDB,
    /// 部署的合约地址映射
    contracts: HashMap<String, Address>,
    /// 当前账户地址
    caller: Address,
}

impl RevmExecutor {
    /// 创建新的执行器实例
    pub fn new() -> Self {
        let mut db = InMemoryDB::default();

        // 初始化调用者账户，赋予充足的余额
        let caller = Address::from([0x01; 20]);
        let account_info = AccountInfo {
            balance: U256::from(1_000_000_000_000_000_000u128), // 1 ETH
            nonce: 0,
            code_hash: Default::default(),
            code: None,
        };
        db.insert_account_info(caller, account_info);

        Self {
            db,
            contracts: HashMap::new(),
            caller,
        }
    }

    /// 部署合约
    ///
    /// # 参数
    /// - `name`: 合约名称（用于后续引用）
    /// - `bytecode`: 合约字节码
    ///
    /// # 返回
    /// - `Ok(Address)`: 部署成功，返回合约地址
    /// - `Err(String)`: 部署失败，返回错误信息
    pub fn deploy_contract(
        &mut self,
        name: &str,
        bytecode: Vec<u8>,
    ) -> Result<Address, String> {
        // 创建 EVM 实例
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

        // 执行部署交易
        let result_and_state = evm
            .transact()
            .map_err(|e| format!("Transaction failed: {:?}", e))?;

        // 显式 drop EVM 以释放对 db 的借用
        drop(evm);

        // 手动提交状态变更
        self.db.commit(result_and_state.state);
        let result = result_and_state.result;

        // 检查执行结果
        match result {
            ExecutionResult::Success {
                output: Output::Create(_, Some(address)),
                ..
            } => {
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

    /// 调用合约函数
    ///
    /// # 参数
    /// - `contract_name`: 合约名称
    /// - `calldata`: 函数调用数据（函数选择器 + 参数）
    ///
    /// # 返回
    /// - `Ok(Bytes)`: 调用成功，返回执行结果
    /// - `Err(String)`: 调用失败，返回错误信息
    pub fn call_contract(
        &mut self,
        contract_name: &str,
        calldata: Vec<u8>,
    ) -> Result<Bytes, String> {
        // 获取合约地址
        let contract_address = self
            .contracts
            .get(contract_name)
            .ok_or_else(|| format!("Contract '{}' not found", contract_name))?;

        // 创建 EVM 实例
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

        // 执行调用
        let result_and_state = evm
            .transact()
            .map_err(|e| format!("Transaction failed: {:?}", e))?;

        // 显式 drop EVM 以释放对 db 的借用
        drop(evm);

        // 手动提交状态变更
        self.db.commit(result_and_state.state);
        let result = result_and_state.result;

        // 检查执行结果
        match result {
            ExecutionResult::Success {
                output: Output::Call(output),
                gas_used,
                ..
            } => {
                println!("✅ 合约调用成功，Gas 使用: {}", gas_used);
                Ok(output)
            }
            ExecutionResult::Revert { output, gas_used } => {
                Err(format!(
                    "Contract call reverted (gas used: {}): {:?}",
                    gas_used, output
                ))
            }
            ExecutionResult::Halt { reason, gas_used } => {
                Err(format!(
                    "Contract call halted (gas used: {}): {:?}",
                    gas_used, reason
                ))
            }
            _ => Err("Unexpected execution result".to_string()),
        }
    }

    /// 读取合约状态（view 调用，不修改状态）
    ///
    /// # 参数
    /// - `contract_name`: 合约名称
    /// - `calldata`: 函数调用数据
    ///
    /// # 返回
    /// - `Ok(Bytes)`: 调用成功，返回查询结果
    /// - `Err(String)`: 调用失败，返回错误信息
    pub fn view_contract(
        &self,
        contract_name: &str,
        calldata: Vec<u8>,
    ) -> Result<Bytes, String> {
        // 获取合约地址
        let contract_address = self
            .contracts
            .get(contract_name)
            .ok_or_else(|| format!("Contract '{}' not found", contract_name))?;

        // 创建临时数据库副本用于只读调用
        let mut db_clone = self.db.clone();

        // 创建 EVM 实例
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

        // 执行查询
        let result = evm
            .transact()
            .map_err(|e| format!("View call failed: {:?}", e))?;

        let result = result.result;

        // 检查执行结果
        match result {
            ExecutionResult::Success {
                output: Output::Call(output),
                ..
            } => Ok(output),
            ExecutionResult::Revert { output, .. } => {
                Err(format!("View call reverted: {:?}", output))
            }
            ExecutionResult::Halt { reason, .. } => {
                Err(format!("View call halted: {:?}", reason))
            }
            _ => Err("Unexpected execution result".to_string()),
        }
    }

    /// 获取合约地址
    #[allow(dead_code)]
    pub fn get_contract_address(&self, name: &str) -> Option<Address> {
        self.contracts.get(name).copied()
    }

    /// 获取调用者地址
    pub fn get_caller(&self) -> Address {
        self.caller
    }

    /// 调试：检查账户信息
    #[allow(dead_code)]
    pub fn debug_account(&self, address: Address) {
        if let Some(account) = self.db.accounts.get(&address) {
            println!("账户 {:?}:", address);
            println!("  余额: {}", account.info.balance);
            println!("  nonce: {}", account.info.nonce);
            println!("  代码哈希: {:?}", account.info.code_hash);
            if let Some(ref code) = account.info.code {
                println!("  代码长度: {} bytes", code.bytecode().len());
            } else {
                println!("  代码: None");
            }
        } else {
            println!("账户 {:?} 不存在", address);
        }
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

    #[test]
    fn test_simple_contract_deployment() {
        let mut executor = RevmExecutor::new();

        // 简单的合约字节码（返回常量）
        let bytecode = vec![
            0x60, 0x80, 0x60, 0x40, 0x52, // PUSH1 0x80 PUSH1 0x40 MSTORE
            0x34, 0x80, 0x15, 0x60, 0x0f, // CALLVALUE DUP1 ISZERO PUSH1 0x0f
        ];

        let result = executor.deploy_contract("test", bytecode);
        // 注意：这个简单的字节码可能不会成功部署，这只是测试框架
        println!("Deployment result: {:?}", result);
    }
}
