use alloy_primitives::U256;

use super::contracts;
use super::executor::RevmExecutor;

/// REVM Counter 合约演示示例
///
/// 这个示例展示了如何：
/// 1. 部署 Counter 智能合约
/// 2. 调用 increment() 函数增加计数器
/// 3. 调用 get() 函数查询当前值
/// 4. 调用 reset() 函数重置计数器
pub fn run_counter_example() -> Result<(), String> {
    println!("{}", "=".repeat(60));
    println!("🚀 REVM Counter 合约演示");
    println!("{}", "=".repeat(60));
    println!();

    // 1. 创建执行器
    println!("📦 步骤 1: 初始化 REVM 执行器");
    let mut executor = RevmExecutor::new();
    println!("   调用者地址: {:?}", executor.get_caller());
    println!();

    // 2. 部署 Counter 合约
    println!("📝 步骤 2: 部署 Counter 合约");
    let bytecode = contracts::get_counter_bytecode();
    println!("   字节码长度: {} bytes", bytecode.len());

    let contract_address = executor.deploy_contract("Counter", bytecode)?;
    println!("   合约地址: {:?}", contract_address);

    // 调试：检查合约是否真的被部署
    executor.debug_account(contract_address);
    println!();

    // 3. 查询初始计数值
    println!("🔍 步骤 3: 查询初始计数值");
    println!("   使用 count() 而不是 get() 来读取公共变量");

    // 使用 count() 函数选择器（公共变量的自动getter）
    let count_selector = vec![0x06, 0x66, 0x1a, 0xbd];
    let result = executor.view_contract("Counter", count_selector.clone())?;

    let count = decode_uint256(&result);
    println!("   当前计数: {}", count);
    println!();

    // 4. 增加计数器（第一次）
    println!("➕ 步骤 4: 调用 increment() - 第 1 次");
    let increment_calldata = contracts::encode_increment();
    executor.call_contract("Counter", increment_calldata.clone())?;

    let result = executor.view_contract("Counter", count_selector.clone())?;
    let count = decode_uint256(&result);
    println!("   当前计数: {}", count);
    println!();

    // 5. 增加计数器（第二次）
    println!("➕ 步骤 5: 调用 increment() - 第 2 次");
    executor.call_contract("Counter", increment_calldata.clone())?;

    let result = executor.view_contract("Counter", count_selector.clone())?;
    let count = decode_uint256(&result);
    println!("   当前计数: {}", count);
    println!();

    // 6. 增加计数器（第三次）
    println!("➕ 步骤 6: 调用 increment() - 第 3 次");
    executor.call_contract("Counter", increment_calldata)?;

    let result = executor.view_contract("Counter", count_selector.clone())?;
    let count = decode_uint256(&result);
    println!("   当前计数: {}", count);
    println!();

    println!("{}", "=".repeat(60));
    println!("✨ 演示完成！Counter 合约在 REVM 上运行成功");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 解码 uint256 返回值
///
/// 将 32 字节的返回数据转换为 U256 类型
fn decode_uint256(data: &[u8]) -> U256 {
    if data.len() >= 32 { U256::from_be_slice(&data[0..32]) } else { U256::ZERO }
}

/// 打印执行统计信息
#[allow(dead_code)]
pub fn print_stats() {
    println!();
    println!("📊 REVM 统计信息:");
    println!("   - 执行环境: 内存数据库");
    println!("   - Gas 限制: 10,000,000");
    println!("   - 初始余额: 1 ETH");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_example() {
        let result = run_counter_example();
        assert!(result.is_ok(), "Counter example should run successfully");
    }

    #[test]
    fn test_decode_uint256() {
        let data = vec![0u8; 32];
        let value = decode_uint256(&data);
        assert_eq!(value, U256::ZERO);

        let mut data = vec![0u8; 32];
        data[31] = 42; // 设置最低字节为 42
        let value = decode_uint256(&data);
        assert_eq!(value, U256::from(42));
    }
}
