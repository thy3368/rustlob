use alloy_primitives::hex;

/// Counter 合约的字节码
///
/// Solidity源代码：
/// ```solidity
/// pragma solidity ^0.8.20;
/// contract Counter {
///     uint256 public count;
///     function increment() public { count += 1; }
/// }
/// ```
///
/// 包含函数：
/// - count(): 函数选择器 0x06661abd (自动生成的公共变量getter)
/// - increment(): 函数选择器 0xd09de08a
///
/// 使用 Solidity 0.8.20 编译生成的标准字节码
/// ✅ 完整的、由 solc 0.8.30 编译的 Counter 合约字节码（已验证）
///
/// Solidity 源码：
/// ```solidity
/// pragma solidity ^0.8.20;
/// contract Counter {
///     uint256 public count;
///     function increment() public { count += 1; }
/// }
/// ```
///
/// 编译命令：solc --bin --optimize --optimize-runs 200 Counter.sol
pub const COUNTER_BYTECODE: &str = "6080604052348015600e575f5ffd5b5060c580601a5f395ff3fe6080604052348015600e575f5ffd5b50600436106030575f3560e01c806306661abd146034578063d09de08a14604d575b5f5ffd5b603b5f5481565b60405190815260200160405180910390f35b60536055565b005b60015f5f82825460649190606b565b9091555050565b80820180821115608957634e487b7160e01b5f52601160045260245ffd5b9291505056fea2646970667358221220a48aa1ce6ad99f3a568df931a13cb1db6bbd0417ec8cf682182c794454b27c5664736f6c634300081e0033";

/// 将十六进制字符串转换为字节数组
pub fn get_counter_bytecode() -> Vec<u8> {
    hex::decode(COUNTER_BYTECODE).expect("Invalid hex string")
}

/// 函数选择器定义
pub mod selectors {
    use alloy_primitives::FixedBytes;

    /// increment() 函数选择器
    pub const INCREMENT: FixedBytes<4> = FixedBytes([0xd0, 0x9d, 0xe0, 0x8a]);

    /// get() 函数选择器
    pub const GET: FixedBytes<4> = FixedBytes([0x6d, 0x4c, 0xe6, 0x3c]);

    /// reset() 函数选择器
    pub const RESET: FixedBytes<4> = FixedBytes([0xd8, 0x26, 0xf8, 0x8f]);
}

/// 编码函数调用数据
pub fn encode_increment() -> Vec<u8> {
    selectors::INCREMENT.to_vec()
}

pub fn encode_get() -> Vec<u8> {
    selectors::GET.to_vec()
}

pub fn encode_reset() -> Vec<u8> {
    selectors::RESET.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha3::{Digest, Keccak256};

    fn calc_selector(sig: &str) -> [u8; 4] {
        let mut hasher = Keccak256::new();
        hasher.update(sig.as_bytes());
        let result = hasher.finalize();
        [result[0], result[1], result[2], result[3]]
    }

    #[test]
    fn test_bytecode_decode() {
        let bytecode = get_counter_bytecode();
        assert!(!bytecode.is_empty(), "Bytecode should not be empty");
    }

    #[test]
    fn test_function_selectors() {
        // 打印实际的函数选择器
        println!("count(): {:02x?}", calc_selector("count()"));
        println!("get(): {:02x?}", calc_selector("get()"));
        println!("increment(): {:02x?}", calc_selector("increment()"));
        println!("reset(): {:02x?}", calc_selector("reset()"));

        let increment_data = encode_increment();
        assert_eq!(increment_data.len(), 4);
        assert_eq!(increment_data, vec![0xd0, 0x9d, 0xe0, 0x8a]);
    }
}
